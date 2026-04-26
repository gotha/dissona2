/**
 * API Client with automatic token refresh
 * 
 * Use this for all authenticated API calls. It will:
 * 1. Add the Authorization header with access token
 * 2. Automatically refresh the token on 401 errors
 * 3. Retry the original request with the new token
 */

import { useAuthStore } from '../stores/authStore';

const API_BASE = import.meta.env.VITE_API_URL || '';

interface ApiOptions extends RequestInit {
  skipAuth?: boolean;
}

class ApiError extends Error {
  constructor(
    public status: number,
    public statusText: string,
    public data?: unknown
  ) {
    super(`API Error: ${status} ${statusText}`);
    this.name = 'ApiError';
  }
}

async function apiFetch<T = unknown>(
  endpoint: string,
  options: ApiOptions = {}
): Promise<T> {
  const { skipAuth = false, ...fetchOptions } = options;
  const { accessToken, refreshToken, logout } = useAuthStore.getState();

  const headers = new Headers(fetchOptions.headers);
  headers.set('Content-Type', 'application/json');

  if (!skipAuth && accessToken) {
    headers.set('Authorization', `Bearer ${accessToken}`);
  }

  const url = endpoint.startsWith('http') ? endpoint : `${API_BASE}${endpoint}`;

  let response = await fetch(url, {
    ...fetchOptions,
    headers,
    credentials: 'include',
  });

  // If 401 and we have a token, try to refresh
  if (response.status === 401 && !skipAuth && accessToken) {
    const refreshed = await refreshToken();

    if (refreshed) {
      // Retry with new token
      const newToken = useAuthStore.getState().accessToken;
      headers.set('Authorization', `Bearer ${newToken}`);

      response = await fetch(url, {
        ...fetchOptions,
        headers,
        credentials: 'include',
      });
    } else {
      // Refresh failed, user will be logged out
      throw new ApiError(401, 'Session expired');
    }
  }

  if (!response.ok) {
    let data: unknown;
    try {
      data = await response.json();
    } catch {
      data = null;
    }
    throw new ApiError(response.status, response.statusText, data);
  }

  // Handle empty responses
  const text = await response.text();
  if (!text) {
    return {} as T;
  }

  return JSON.parse(text) as T;
}

// Convenience methods
export const api = {
  get: <T = unknown>(endpoint: string, options?: ApiOptions) =>
    apiFetch<T>(endpoint, { ...options, method: 'GET' }),

  post: <T = unknown>(endpoint: string, data?: unknown, options?: ApiOptions) =>
    apiFetch<T>(endpoint, {
      ...options,
      method: 'POST',
      body: data ? JSON.stringify(data) : undefined,
    }),

  put: <T = unknown>(endpoint: string, data?: unknown, options?: ApiOptions) =>
    apiFetch<T>(endpoint, {
      ...options,
      method: 'PUT',
      body: data ? JSON.stringify(data) : undefined,
    }),

  patch: <T = unknown>(endpoint: string, data?: unknown, options?: ApiOptions) =>
    apiFetch<T>(endpoint, {
      ...options,
      method: 'PATCH',
      body: data ? JSON.stringify(data) : undefined,
    }),

  delete: <T = unknown>(endpoint: string, options?: ApiOptions) =>
    apiFetch<T>(endpoint, { ...options, method: 'DELETE' }),
};

export { ApiError };
export default api;
