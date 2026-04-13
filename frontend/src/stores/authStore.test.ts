import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { useAuthStore } from './authStore';

// Mock fetch
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('authStore', () => {
  beforeEach(() => {
    // Reset store state
    useAuthStore.setState({
      user: null,
      accessToken: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,
      notificationPromptDismissedAt: null,
    });
    mockFetch.mockReset();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  describe('setAuth', () => {
    it('should set user and token', () => {
      const user = { id: '1', email: 'test@example.com', name: 'Test User' };
      const token = 'test-token';

      useAuthStore.getState().setAuth(user, token);

      const state = useAuthStore.getState();
      expect(state.user).toEqual(user);
      expect(state.accessToken).toBe(token);
      expect(state.isAuthenticated).toBe(true);
      expect(state.error).toBeNull();
    });

    it('should clear error when setting auth', () => {
      useAuthStore.setState({ error: 'Previous error' });
      const user = { id: '1', email: 'test@example.com', name: 'Test User' };

      useAuthStore.getState().setAuth(user, 'token');

      expect(useAuthStore.getState().error).toBeNull();
    });
  });

  describe('setUser', () => {
    it('should update user without affecting token', () => {
      const initialUser = { id: '1', email: 'test@example.com', name: 'Test User' };
      useAuthStore.setState({ user: initialUser, accessToken: 'token', isAuthenticated: true });

      const updatedUser = { ...initialUser, name: 'Updated Name', avatarUrl: 'https://example.com/avatar.jpg' };
      useAuthStore.getState().setUser(updatedUser);

      const state = useAuthStore.getState();
      expect(state.user?.name).toBe('Updated Name');
      expect(state.user?.avatarUrl).toBe('https://example.com/avatar.jpg');
      expect(state.accessToken).toBe('token');
    });
  });

  describe('logout', () => {
    it('should clear all auth state', async () => {
      useAuthStore.setState({
        user: { id: '1', email: 'test@example.com', name: 'Test' },
        accessToken: 'token',
        isAuthenticated: true,
      });
      mockFetch.mockResolvedValueOnce({ ok: true });

      await useAuthStore.getState().logout();

      const state = useAuthStore.getState();
      expect(state.user).toBeNull();
      expect(state.accessToken).toBeNull();
      expect(state.isAuthenticated).toBe(false);
      expect(state.isLoading).toBe(false);
    });

    it('should call logout API', async () => {
      mockFetch.mockResolvedValueOnce({ ok: true });

      await useAuthStore.getState().logout();

      expect(mockFetch).toHaveBeenCalledWith(
        expect.stringContaining('/auth/logout'),
        expect.objectContaining({ method: 'POST', credentials: 'include' })
      );
    });

    it('should clear state even if API fails', async () => {
      useAuthStore.setState({ isAuthenticated: true, user: { id: '1', email: 'test@example.com', name: 'Test' } });
      mockFetch.mockRejectedValueOnce(new Error('Network error'));

      await useAuthStore.getState().logout();

      expect(useAuthStore.getState().isAuthenticated).toBe(false);
    });
  });

  describe('refreshToken', () => {
    it('should update access token on success', async () => {
      mockFetch.mockResolvedValueOnce({
        ok: true,
        json: () => Promise.resolve({ access_token: 'new-token' }),
      });

      const result = await useAuthStore.getState().refreshToken();

      expect(result).toBe(true);
      expect(useAuthStore.getState().accessToken).toBe('new-token');
    });

    it('should clear auth on 401', async () => {
      useAuthStore.setState({ isAuthenticated: true, user: { id: '1', email: 'test@example.com', name: 'Test' } });
      mockFetch.mockResolvedValueOnce({ ok: false, status: 401 });

      const result = await useAuthStore.getState().refreshToken();

      expect(result).toBe(false);
      expect(useAuthStore.getState().isAuthenticated).toBe(false);
      expect(useAuthStore.getState().error).toBe('Your session has expired. Please sign in again.');
    });
  });

  describe('error handling', () => {
    it('should set and clear errors', () => {
      useAuthStore.getState().setError('Test error');
      expect(useAuthStore.getState().error).toBe('Test error');

      useAuthStore.getState().clearError();
      expect(useAuthStore.getState().error).toBeNull();
    });
  });

  describe('notification prompt', () => {
    it('should track dismissal', () => {
      useAuthStore.setState({ isAuthenticated: true });
      expect(useAuthStore.getState().shouldShowNotificationPrompt()).toBe(true);

      useAuthStore.getState().dismissNotificationPrompt();

      expect(useAuthStore.getState().notificationPromptDismissedAt).not.toBeNull();
      expect(useAuthStore.getState().shouldShowNotificationPrompt()).toBe(false);
    });

    it('should not show prompt if not authenticated', () => {
      useAuthStore.setState({ isAuthenticated: false });
      expect(useAuthStore.getState().shouldShowNotificationPrompt()).toBe(false);
    });
  });
});
