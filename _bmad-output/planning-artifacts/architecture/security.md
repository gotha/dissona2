# API Security Specification

## Security Layers

```
Request → TLS → Rate Limit → Authentication → Authorization → Validation → Handler
```

---

## 1. Transport Security

| Aspect | Implementation |
|--------|----------------|
| TLS | Traefik + Let's Encrypt |
| Version | TLS 1.3 preferred |
| HTTP | Redirect to HTTPS |
| HSTS | Enabled (1 year) |

---

## 2. Authentication

### JWT Tokens

| Token | Lifetime | Storage | Purpose |
|-------|----------|---------|---------|
| Access | 1 hour | Memory | API requests |
| Refresh | 30 days | HttpOnly cookie | Get new access token |

### Access Token Payload

```json
{
  "sub": "user-uuid",
  "email": "user@example.com",
  "name": "John Doe",
  "iat": 1712678400,
  "exp": 1712682000,
  "jti": "token-uuid"
}
```

### Cookie Settings (Refresh Token)

```
HttpOnly; Secure; SameSite=Strict; Path=/auth
```

---

## 3. Authorization

| Check | Description |
|-------|-------------|
| Ownership | User owns the resource |
| Share access | Resource shared with user |
| Permission level | What shared user can do |

---

## 4. Rate Limiting

| Endpoint | Limit | Window | By |
|----------|-------|--------|-----|
| `/auth/*` | 20 | 1 min | IP |
| `/api/*` | 100 | 1 min | User |
| `POST /api/projects` | 10 | 1 hour | User |
| `POST /*/generate` | 5 | 1 hour | User |

### Response Headers

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1712678520
```

---

## 5. Input Validation

| Input | Validation |
|-------|------------|
| Request body | JSON schema validation |
| File uploads | Type (magic bytes), size (100MB max) |
| Path params | UUID format |
| Query params | Type and range validation |

---

## 6. Security Headers

| Header | Value |
|--------|-------|
| X-Frame-Options | DENY |
| X-Content-Type-Options | nosniff |
| X-XSS-Protection | 1; mode=block |
| Referrer-Policy | strict-origin-when-cross-origin |
| Strict-Transport-Security | max-age=31536000; includeSubDomains |
| Content-Security-Policy | default-src 'self'; ... |

---

## 7. CORS

| Setting | Value |
|---------|-------|
| Allowed Origins | https://app.disona.app, localhost:5173 (dev) |
| Allowed Methods | GET, POST, PUT, DELETE, OPTIONS |
| Credentials | true (for cookies) |
| Max Age | 86400 (24 hours) |

---

## 8. Attack Mitigations

| Attack | Mitigation |
|--------|------------|
| XSS | CSP, input sanitization, HttpOnly cookies |
| CSRF | SameSite cookies, verify origin |
| SQL Injection | Parameterized queries (sqlx) |
| Brute Force | Rate limiting |
| JWT Theft | Short expiry, HttpOnly refresh |
| Session Hijacking | Secure cookies, session binding |

---

## 9. Secrets Management

### Development
- `.env` file (gitignored)

### Production
- Environment variables
- Docker secrets
- File permissions (600)

| Secret | Required |
|--------|----------|
| JWT_SECRET | Yes |
| GOOGLE_CLIENT_SECRET | Yes |
| DATABASE_URL | Yes |
| S3_SECRET_KEY | Yes |
