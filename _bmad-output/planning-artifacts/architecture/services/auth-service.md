# Auth Service Specification

## Overview

| Attribute | Value |
|-----------|-------|
| **Language** | Rust |
| **Framework** | Actix-web |
| **Port** | 8081 |
| **Database** | PostgreSQL (read) + Redis (sessions) |

---

## Responsibilities

### Primary Functions

1. **OAuth Authentication** — Google sign-in flow
2. **JWT Management** — Issue and refresh tokens
3. **Session Management** — Track active sessions
4. **Rate Limiting** — Protect API from abuse
5. **Forward Auth** — Validate requests for Traefik

### Security Boundary

This service is separate from the API Service to:
- Isolate credentials and secrets
- Enable independent security audits
- Scale authentication separately

---

## API Endpoints

### OAuth Flow

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/auth/google` | Redirect to Google OAuth |
| `GET` | `/auth/google/callback` | Handle OAuth callback |
| `POST` | `/auth/refresh` | Refresh access token |
| `POST` | `/auth/logout` | Invalidate session |

### Forward Auth (Internal)

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/verify` | Validate request (called by Traefik) |

---

## OAuth Flow

```
1. User clicks "Sign in with Google"
         │
         ▼
2. Frontend redirects to: GET /auth/google
         │
         ▼
3. Auth Service redirects to Google OAuth:
   https://accounts.google.com/o/oauth2/v2/auth?
     client_id=...
     redirect_uri=.../auth/google/callback
     scope=email%20profile
     state=<csrf_token>
         │
         ▼
4. User authenticates with Google
         │
         ▼
5. Google redirects to: GET /auth/google/callback?code=...
         │
         ▼
6. Auth Service:
   - Exchanges code for tokens with Google
   - Fetches user profile (email, name)
   - Creates/updates user in database
   - Creates session in Redis
   - Issues JWT tokens (access + refresh)
         │
         ▼
7. Redirects to frontend with tokens:
   https://app.disona.com/auth/success?token=...
```

---

## JWT Structure

### Access Token (short-lived)

```json
{
  "sub": "user-uuid",
  "email": "user@example.com",
  "name": "John Doe",
  "tier": "pro",
  "quota_remaining_hours": 45.5,
  "iat": 1712678400,
  "exp": 1712682000
}
```

**Expiration:** 1 hour

### Refresh Token (long-lived)

```json
{
  "sub": "user-uuid",
  "session_id": "session-uuid",
  "iat": 1712678400,
  "exp": 1714492800
}
```

**Expiration:** 30 days

---

## Forward Auth

Traefik calls `/verify` before forwarding requests to API Service.

### Request (from Traefik)

```http
GET /verify HTTP/1.1
X-Forwarded-Uri: /api/books
X-Forwarded-Method: GET
Authorization: Bearer <jwt>
```

### Response (Success)

```http
HTTP/1.1 200 OK
X-User-Id: user-uuid
X-User-Email: user@example.com
X-User-Tier: pro
X-User-Quota-Remaining: 45.5
```

### Response (Failure)

```http
HTTP/1.1 401 Unauthorized
```

---

## Rate Limiting

### Limits by Tier

| Tier | Requests/minute | Uploads/day |
|------|-----------------|-------------|
| Free | 60 | 3 |
| Standard | 120 | 10 |
| Pro | 300 | Unlimited |

### Implementation

```rust
async fn check_rate_limit(user_id: &str, endpoint: &str) -> Result<(), RateLimitError> {
    let key = format!("ratelimit:{}:{}", user_id, endpoint);
    let count = redis.incr(&key).await?;
    
    if count == 1 {
        redis.expire(&key, 60).await?;
    }
    
    let limit = get_limit_for_user(user_id, endpoint).await?;
    
    if count > limit {
        return Err(RateLimitError::Exceeded);
    }
    
    Ok(())
}
```

### Rate Limit Response

```http
HTTP/1.1 429 Too Many Requests
X-RateLimit-Limit: 60
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1712678460
Retry-After: 45
```

---

## Session Management

### Redis Session Structure

```
Key: session:{session_id}
Value: {
  "user_id": "user-uuid",
  "created_at": "2026-04-09T12:00:00Z",
  "last_active": "2026-04-09T14:30:00Z",
  "ip": "192.168.1.1",
  "user_agent": "Mozilla/5.0..."
}
TTL: 30 days
```

### Logout

```rust
async fn logout(session_id: &str) {
    // Delete session from Redis
    redis.del(format!("session:{}", session_id)).await?;
    
    // Add refresh token to blacklist (until expiry)
    redis.setex(
        format!("blacklist:{}", refresh_token_jti),
        remaining_ttl,
        "1"
    ).await?;
}
```

---

## Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection | required |
| `REDIS_URL` | Redis connection | `redis://localhost:6379` |
| `GOOGLE_CLIENT_ID` | OAuth client ID | required |
| `GOOGLE_CLIENT_SECRET` | OAuth client secret | required |
| `JWT_SECRET` | JWT signing key | required |
| `JWT_ACCESS_TTL` | Access token TTL | `3600` (1 hour) |
| `JWT_REFRESH_TTL` | Refresh token TTL | `2592000` (30 days) |

---

## Security Considerations

| Concern | Mitigation |
|---------|------------|
| Token theft | Short-lived access tokens (1h) |
| Session hijacking | Bind sessions to IP + user agent |
| CSRF | State parameter in OAuth flow |
| Brute force | Rate limiting on all endpoints |
| Token reuse after logout | Refresh token blacklist |
