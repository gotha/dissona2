import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import AuthCallback from './AuthCallback';
import { useAuthStore } from '../stores/authStore';

// Mock useNavigate
const mockNavigate = vi.fn();
vi.mock('react-router-dom', async () => {
  const actual = await vi.importActual('react-router-dom');
  return {
    ...actual,
    useNavigate: () => mockNavigate,
  };
});

describe('AuthCallback', () => {
  beforeEach(() => {
    useAuthStore.setState({
      user: null,
      accessToken: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,
      notificationPromptDismissedAt: null,
    });
    mockNavigate.mockReset();
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  const renderAuthCallback = (route: string) => {
    return render(
      <MemoryRouter initialEntries={[route]}>
        <AuthCallback />
      </MemoryRouter>
    );
  };

  // Create a valid JWT-like token (base64 encoded payload)
  const createToken = (payload: object) => {
    const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
    const body = btoa(JSON.stringify(payload));
    const signature = 'fake-signature';
    return `${header}.${body}.${signature}`;
  };

  it('should parse token and set auth state', async () => {
    const token = createToken({
      sub: 'user-123',
      email: 'test@example.com',
      name: 'Test User',
      avatar_url: 'https://example.com/avatar.jpg',
    });

    renderAuthCallback(`/auth/callback?access_token=${token}`);

    await waitFor(() => {
      const state = useAuthStore.getState();
      expect(state.isAuthenticated).toBe(true);
      expect(state.user?.id).toBe('user-123');
      expect(state.user?.email).toBe('test@example.com');
      expect(state.user?.name).toBe('Test User');
      expect(state.user?.avatarUrl).toBe('https://example.com/avatar.jpg');
    });
  });

  it('should show success state after parsing valid token', async () => {
    const token = createToken({ sub: '1', email: 'test@example.com', name: 'Test' });

    renderAuthCallback(`/auth/callback?access_token=${token}`);

    await waitFor(() => {
      expect(screen.getByText('Welcome back!')).toBeInTheDocument();
    });
  });

  it('should handle OAuth error in URL', async () => {
    renderAuthCallback('/auth/callback?error=access_denied&message=User%20denied%20access');

    await waitFor(() => {
      expect(mockNavigate).toHaveBeenCalled();
    });

    // Check that it navigated to login with error
    expect(mockNavigate.mock.calls[0][0]).toContain('/login');
  });

  it('should show error state for invalid token', async () => {
    renderAuthCallback('/auth/callback?access_token=invalid-token');

    await waitFor(() => {
      expect(screen.getByText(/something went wrong/i)).toBeInTheDocument();
    });
  });

  it('should set error in auth store on failure', async () => {
    renderAuthCallback('/auth/callback?access_token=invalid');

    await waitFor(() => {
      const state = useAuthStore.getState();
      expect(state.error).toBe('Failed to process authentication');
    });
  });
});
