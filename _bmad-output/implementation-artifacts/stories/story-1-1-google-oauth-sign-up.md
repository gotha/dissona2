# Story 1.1: Google OAuth Sign Up

**Epic:** E1 - Authentication & Onboarding  
**Priority:** P0 (Foundation)  
**Status:** Ready for Dev  
**Story Points:** 5  
**Created:** 2026-04-12

---

## User Story

As a **new user**,  
I want to **sign up using my Google account**,  
So that I can **start using Disona without creating a new password**.

---

## Requirements Traceability

| Requirement | Description |
|-------------|-------------|
| FR42 | Support Google OAuth 2.0 for authentication |
| FR43 | Create user account upon first OAuth login |
| NFR-S1 | JWT tokens with 1-hour access, 30-day refresh |
| NFR-S3 | HTTPS required for all auth flows |

---

## Acceptance Criteria

### AC1: Google OAuth Redirect

- **Given** I am on the login page (`/login`)
- **When** I tap "Continue with Google"
- **Then** I am redirected to Google's OAuth consent screen
- **And** the consent screen requests `email` and `profile` scopes

### AC2: Successful OAuth Callback

- **Given** I have granted Google OAuth consent
- **When** Google redirects back to `/auth/google/callback`
- **Then** my account is created (or retrieved if existing)
- **And** I receive a JWT access token (1-hour expiry)
- **And** I receive an HttpOnly refresh token cookie (30-day expiry)
- **And** I am redirected to the home page (`/`)

### AC3: User Record Creation

- **Given** I am a first-time user
- **When** OAuth callback completes
- **Then** a user record is created with:
  - `email` from Google profile
  - `name` from Google profile
  - `google_id` from Google profile
  - `avatar_url` from Google profile (optional)
- **And** `created_at` is set to current timestamp

### AC4: Returning User Update

- **Given** I have logged in before
- **When** I log in again via Google OAuth
- **Then** my existing user record is updated:
  - `name` refreshed from Google
  - `avatar_url` refreshed from Google
  - `updated_at` set to current timestamp
- **And** my `email` and `google_id` remain unchanged

### AC5: Frontend State Update

- **Given** OAuth callback redirects to `/auth/callback`
- **When** the page loads with `access_token` query param
- **Then** the auth store is updated with user info
- **And** user is redirected to `/` (library)
- **And** auth state persists across page refreshes

### AC6: Error Handling

- **Given** OAuth fails (user denies, network error)
- **When** callback receives an error
- **Then** user is redirected to `/login`
- **And** an error message is displayed

---

## Technical Design

### Architecture Overview

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Frontend  │───►│   Traefik   │───►│ Auth Service│───►│  PostgreSQL │
│   (React)   │    │   (Proxy)   │    │   (Rust)    │    │  (auth_db)  │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                                    │
       │                                    ▼
       │                            ┌─────────────┐
       │                            │   Google    │
       │                            │   OAuth     │
       │                            └─────────────┘
       │
       ▼
┌─────────────┐
│ Auth Store  │
│  (Zustand)  │
└─────────────┘
```

### Components to Modify

| Component | File | Changes |
|-----------|------|---------|
| Login Page | `frontend/src/pages/Login.tsx` | Review, add error state |
| Auth Callback | `frontend/src/pages/AuthCallback.tsx` | Add error handling |
| Auth Store | `frontend/src/stores/authStore.ts` | Add avatar_url field |
| Google Handler | `services/auth/src/handlers/google.rs` | Add error responses |
| Config | `services/auth/src/config.rs` | Verify OAuth settings |

---

## Implementation Tasks

### Backend Tasks

- [ ] **1.1.1** Verify Google OAuth client configuration in `.env`
- [ ] **1.1.2** Add CSRF token validation to callback
- [ ] **1.1.3** Add error response handling for OAuth failures
- [ ] **1.1.4** Add unit tests for `google_login` handler
- [ ] **1.1.5** Add unit tests for `google_callback` handler
- [ ] **1.1.6** Add integration test for full OAuth flow (mocked)

### Frontend Tasks

- [ ] **1.1.7** Add `avatar_url` to User interface in authStore
- [ ] **1.1.8** Add error handling to AuthCallback page
- [ ] **1.1.9** Add error display to Login page
- [ ] **1.1.10** Add loading state during OAuth redirect
- [ ] **1.1.11** Add unit tests for authStore
- [ ] **1.1.12** Add E2E test for OAuth flow (Playwright)

### Infrastructure Tasks

- [ ] **1.1.13** Verify Traefik routes for `/auth/*`
- [ ] **1.1.14** Add Google OAuth credentials to environment

---

## Existing Code Reference

### Backend: OAuth Flow (Already Implemented)

**File:** `services/auth/src/handlers/google.rs`

Key endpoints:
- `GET /auth/google` → Redirects to Google OAuth
- `GET /auth/google/callback` → Handles OAuth callback

**File:** `services/auth/src/oauth.rs`

Google user info struct and API call already implemented.

**File:** `services/auth/src/jwt.rs`

JWT creation for access/refresh tokens already implemented.

### Frontend: Auth Components (Already Implemented)

**File:** `frontend/src/pages/Login.tsx`

Basic login page with Google button - needs error state.

**File:** `frontend/src/pages/AuthCallback.tsx`

Callback handler - needs error handling improvements.

**File:** `frontend/src/stores/authStore.ts`

Zustand store with persist - needs avatar_url field.

### Database: User Table (Already Implemented)

**File:** `services/auth/migrations/001_create_users.sql`

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255),
    avatar_url VARCHAR(500),
    google_id VARCHAR(255) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
```

---

## API Contracts

### Endpoint: GET /auth/google

**Description:** Initiates Google OAuth flow

**Response:** HTTP 302 Redirect to Google OAuth

### Endpoint: GET /auth/google/callback

**Description:** Handles OAuth callback from Google

**Query Parameters:**
- `code` - Authorization code from Google
- `state` - CSRF state token

**Success Response:** HTTP 302 Redirect
```
Location: /auth/callback?access_token=<jwt>
Set-Cookie: refresh_token=<jwt>; HttpOnly; Secure; SameSite=Strict; Path=/auth; Max-Age=2592000
```

**Error Response:** HTTP 302 Redirect
```
Location: /login?error=oauth_failed&message=<description>
```

---

## Environment Variables Required

```bash
# Google OAuth
GOOGLE_CLIENT_ID=your-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=your-client-secret
GOOGLE_REDIRECT_URI=http://localhost:8080/auth/google/callback

# JWT
JWT_SECRET=your-secure-secret-at-least-32-characters
```

---

## Testing Strategy

### Unit Tests

| Test | File | Description |
|------|------|-------------|
| `test_google_login_redirect` | `handlers/google.rs` | Verify redirect URL generation |
| `test_google_callback_new_user` | `handlers/google.rs` | Verify user creation |
| `test_google_callback_existing_user` | `handlers/google.rs` | Verify user update |
| `test_jwt_creation` | `jwt.rs` | Verify token structure |
| `test_auth_store_setAuth` | `authStore.test.ts` | Verify state update |

### Integration Tests

| Test | Description |
|------|-------------|
| `test_oauth_flow_e2e` | Full OAuth with mocked Google API |
| `test_callback_invalid_code` | Error handling for invalid code |

### E2E Tests (Playwright)

| Test | Description |
|------|-------------|
| `test_login_button_visible` | Login page renders correctly |
| `test_google_redirect` | Click triggers OAuth redirect |

---

## Definition of Done

- [ ] All acceptance criteria verified
- [ ] Unit tests pass (>80% coverage for new code)
- [ ] Integration tests pass
- [ ] Code reviewed and approved
- [ ] No security vulnerabilities (CSRF, token leakage)
- [ ] Error messages are user-friendly
- [ ] Works on Chrome, Firefox, Safari (mobile + desktop)

---

## Notes & Decisions

### Decision: Access Token in URL

Access token is passed via URL query parameter to frontend callback. This is acceptable for:
- Single-page redirect (token not stored in history)
- HTTPS required in production
- Token is short-lived (1 hour)

**Alternative considered:** Pass token in response body - rejected because it requires additional frontend complexity.

### Decision: Refresh Token in Cookie

Refresh token uses HttpOnly cookie to prevent XSS attacks. Cookie is:
- `HttpOnly` - Not accessible via JavaScript
- `Secure` - HTTPS only in production
- `SameSite=Strict` - CSRF protection
- `Path=/auth` - Only sent to auth endpoints

---

## Dependencies

| Dependency | Status |
|------------|--------|
| PostgreSQL (auth_db) | ✅ Running |
| Traefik proxy | ✅ Configured |
| Google OAuth app | 🟡 Needs credentials |

---

## Out of Scope

- Apple Sign In (future story)
- Email/password authentication (not planned for MVP)
- Two-factor authentication (post-MVP)
- Session management UI (future story)

---

## Related Stories

| Story | Relationship |
|-------|--------------|
| 1.2 Sign In & Sign Out | Uses same OAuth flow |
| 1.3 Cross-Device Session | Uses JWT from this story |
| 8.1 View Subscription Status | Depends on user record |
