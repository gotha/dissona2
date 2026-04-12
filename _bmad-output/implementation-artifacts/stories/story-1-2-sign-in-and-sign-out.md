# Story 1.2: Sign In & Sign Out

**Epic:** E1 - Authentication & Onboarding  
**Priority:** P0 (Foundation)  
**Status:** Ready for Dev  
**Story Points:** 3  
**Created:** 2026-04-12

---

## User Story

As a **returning user**,  
I want to **sign in and sign out of my account**,  
So that I can **access my content securely across sessions**.

---

## Requirements Traceability

| Requirement | Description |
|-------------|-------------|
| FR43 | Sign in via Google OAuth |
| FR46 | Session persists for 30 days of inactivity |
| NFR-S1 | JWT tokens with 1-hour access, 30-day refresh |
| NFR-S2 | Secure logout clears all tokens |

---

## Acceptance Criteria

### AC1: Returning User Sign In

- **Given** I have an existing account
- **When** I tap "Sign In" on the login page
- **Then** I am redirected to Google OAuth
- **And** after authenticating, I return to my library (`/`)
- **And** my session is restored with my projects visible

### AC2: Session Persistence

- **Given** I am logged in
- **When** I close the browser and reopen it within 30 days
- **Then** I remain logged in
- **And** I can access protected routes without re-authenticating

### AC3: Token Refresh

- **Given** my access token has expired (after 1 hour)
- **When** I make an API request
- **Then** the access token is automatically refreshed using the refresh token
- **And** the request completes successfully
- **And** I am not redirected to login

### AC4: Sign Out from Header

- **Given** I am logged in
- **When** I tap "Logout" in the header
- **Then** I am logged out
- **And** my refresh token cookie is cleared
- **And** I am redirected to `/login`
- **And** I cannot access protected routes

### AC5: Sign Out Clears Local State

- **Given** I tap "Logout"
- **When** logout completes
- **Then** auth store state is cleared
- **And** localStorage auth data is removed
- **And** any cached data is cleared

### AC6: Expired Session Redirect

- **Given** my refresh token has expired (after 30 days)
- **When** I try to access a protected route
- **Then** I am redirected to `/login`
- **And** I see a message: "Your session has expired. Please sign in again."

---

## Technical Design

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           SIGN IN FLOW                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  [User clicks Sign In] → [Google OAuth] → [Callback] → [Library]        │
│                                                                          │
│  Same as Story 1.1 - reuses Google OAuth flow                           │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                           TOKEN REFRESH FLOW                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  [API Request] → [401 Unauthorized] → [POST /auth/refresh]              │
│       │                                      │                           │
│       │                                      ▼                           │
│       │                              [New Access Token]                  │
│       │                                      │                           │
│       ▼                                      ▼                           │
│  [Retry Request] ◄───────────────────────────┘                          │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                           SIGN OUT FLOW                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  [User clicks Logout] → [POST /auth/logout] → [Clear Cookie]            │
│                               │                                          │
│                               ▼                                          │
│                      [Clear Auth Store]                                  │
│                               │                                          │
│                               ▼                                          │
│                      [Redirect to /login]                               │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### Components to Modify

| Component | File | Changes |
|-----------|------|---------|
| Header | `frontend/src/components/layout/Header.tsx` | Add avatar, improve logout UI |
| Auth Store | `frontend/src/stores/authStore.ts` | Add clearCache, improve refresh |
| API Client | `frontend/src/lib/api.ts` | Add auto-refresh interceptor |
| Auth Callback | `frontend/src/pages/AuthCallback.tsx` | Handle returning user |
| App | `frontend/src/App.tsx` | Add session expiry check |

---

## Implementation Tasks

### Backend Tasks

- [ ] **1.2.1** Add session expiry message to redirect URL
- [ ] **1.2.2** Add unit test for `logout` handler
- [ ] **1.2.3** Add unit test for `refresh_token` handler
- [ ] **1.2.4** Verify refresh token rotation works correctly

### Frontend Tasks

- [ ] **1.2.5** Create API client with auto-refresh interceptor
- [ ] **1.2.6** Add avatar display to Header component
- [ ] **1.2.7** Improve logout button styling (dropdown menu)
- [ ] **1.2.8** Add session expiry detection on app mount
- [ ] **1.2.9** Add "session expired" message to login page
- [ ] **1.2.10** Clear cached data on logout
- [ ] **1.2.11** Add unit tests for token refresh logic
- [ ] **1.2.12** Add E2E test for logout flow

---

## Existing Code Reference

### Backend: Already Implemented ✅

**File:** `services/auth/src/handlers/refresh.rs`

```rust
// Refresh token endpoint - rotates tokens
pub async fn refresh_token(...) -> Result<impl Responder, AuthError>

// Logout endpoint - clears cookie
pub async fn logout(req: HttpRequest) -> impl Responder
```

**Routes configured in:** `services/auth/src/handlers/mod.rs`
- `POST /auth/refresh` → `refresh::refresh_token`
- `POST /auth/logout` → `refresh::logout`

### Frontend: Partially Implemented

**File:** `frontend/src/stores/authStore.ts`

```typescript
// Logout function - calls API and clears state
logout: async () => {
  await fetch('/auth/logout', { method: 'POST', credentials: 'include' });
  set({ user: null, accessToken: null, isAuthenticated: false });
}

// Refresh function - needs interceptor integration
refreshToken: async () => {
  const response = await fetch('/auth/refresh', {...});
  set({ accessToken: data.access_token });
}
```

**File:** `frontend/src/components/layout/Header.tsx`

```typescript
// Basic logout button - needs avatar and dropdown
<button onClick={logout}>Logout</button>
```

---

## API Contracts

### Endpoint: POST /auth/refresh

**Description:** Refreshes access token using refresh cookie

**Cookies Required:**
- `refresh_token` (HttpOnly cookie)

**Success Response:** HTTP 200
```json
{
  "access_token": "<new_jwt>"
}
```
```
Set-Cookie: refresh_token=<new_jwt>; HttpOnly; Secure; SameSite=Strict
```

**Error Response:** HTTP 401
```json
{
  "error": "invalid_token",
  "message": "Refresh token expired"
}
```

### Endpoint: POST /auth/logout

**Description:** Logs out user by clearing refresh token

**Success Response:** HTTP 200
```json
{
  "message": "Logged out"
}
```
```
Set-Cookie: refresh_token=; Max-Age=0; Path=/auth
```

---

## Auto-Refresh Interceptor Design

Create `frontend/src/lib/api.ts`:

```typescript
import { useAuthStore } from '../stores/authStore';

const api = {
  async fetch(url: string, options: RequestInit = {}): Promise<Response> {
    const { accessToken, refreshToken, logout } = useAuthStore.getState();

    // Add auth header
    const headers = new Headers(options.headers);
    if (accessToken) {
      headers.set('Authorization', `Bearer ${accessToken}`);
    }

    let response = await fetch(url, { ...options, headers });

    // If 401, try refresh
    if (response.status === 401) {
      try {
        await refreshToken();
        // Retry with new token
        const newToken = useAuthStore.getState().accessToken;
        headers.set('Authorization', `Bearer ${newToken}`);
        response = await fetch(url, { ...options, headers });
      } catch {
        logout();
        window.location.href = '/login?expired=true';
      }
    }

    return response;
  }
};

export default api;
```

---

## Testing Strategy

### Unit Tests

| Test | File | Description |
|------|------|-------------|
| `test_logout_clears_cookie` | `refresh.rs` | Verify cookie removal |
| `test_refresh_returns_new_token` | `refresh.rs` | Verify token rotation |
| `test_refresh_invalid_token` | `refresh.rs` | Verify 401 on bad token |
| `test_auto_refresh` | `api.test.ts` | Verify interceptor retries |
| `test_logout_clears_state` | `authStore.test.ts` | Verify state cleared |

### E2E Tests (Playwright)

| Test | Description |
|------|-------------|
| `test_logout_redirects` | Click logout → lands on /login |
| `test_session_persists` | Login → close → reopen → still logged in |
| `test_expired_session` | Mock expired token → redirected to login |

---

## Definition of Done

- [ ] All acceptance criteria verified
- [ ] Unit tests pass (>80% coverage for new code)
- [ ] E2E tests pass
- [ ] Code reviewed and approved
- [ ] Token refresh works seamlessly (no user interruption)
- [ ] Logout clears all auth state
- [ ] Session expiry shows friendly message

---

## Dependencies

| Dependency | Status |
|------------|--------|
| Story 1.1 (OAuth) | 🟡 Ready for Dev |
| Auth service running | ✅ |
| Refresh endpoint | ✅ Implemented |
| Logout endpoint | ✅ Implemented |

---

## Out of Scope

- "Remember me" checkbox (always remembers for 30 days)
- Force logout from other devices
- Session list management
- Logout confirmation dialog

---

## Related Stories

| Story | Relationship |
|-------|--------------|
| 1.1 Google OAuth Sign Up | Uses same OAuth flow |
| 1.3 Cross-Device Session | Uses token refresh from this story |

---

## Notes & Decisions

### Decision: Auto-Refresh Interceptor

All API calls should use the `api.fetch()` wrapper to ensure automatic token refresh. This prevents 401 errors from disrupting user experience.

### Decision: Token Rotation on Refresh

Each refresh request returns a new refresh token (rotation). This limits the damage if a refresh token is compromised.

### Decision: No Logout Confirmation

Logout is immediate with no confirmation dialog. Users can easily sign back in with Google OAuth.
