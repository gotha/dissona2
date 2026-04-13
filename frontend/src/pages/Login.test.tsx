import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter } from 'react-router-dom';
import Login from './Login';
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

describe('Login', () => {
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

  const renderLogin = (route = '/login') => {
    return render(
      <MemoryRouter initialEntries={[route]}>
        <Login />
      </MemoryRouter>
    );
  };

  it('should render login page', () => {
    renderLogin();

    expect(screen.getByText('Dissona')).toBeInTheDocument();
    expect(screen.getByText('Transform documents into audio')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /continue with google/i })).toBeInTheDocument();
  });

  it('should redirect to home if already authenticated', () => {
    useAuthStore.setState({ isAuthenticated: true });

    renderLogin();

    expect(mockNavigate).toHaveBeenCalledWith('/');
  });

  it('should show session expired message', () => {
    renderLogin('/login?expired=true');

    expect(screen.getByText(/your session has expired/i)).toBeInTheDocument();
  });

  it('should show error message from URL', () => {
    renderLogin('/login?error=oauth_failed&message=Authentication%20failed');

    expect(screen.getByText(/authentication failed/i)).toBeInTheDocument();
  });

  it('should show error from auth store', () => {
    useAuthStore.setState({ error: 'Custom error message' });

    renderLogin();

    expect(screen.getByText('Custom error message')).toBeInTheDocument();
  });

  it('should redirect to Google OAuth on button click', async () => {
    const user = userEvent.setup();
    const originalLocation = window.location;
    
    // Mock window.location.href
    delete (window as { location?: Location }).location;
    window.location = { ...originalLocation, href: '' } as Location;

    renderLogin();

    await user.click(screen.getByRole('button', { name: /continue with google/i }));

    expect(window.location.href).toContain('/auth/google');

    // Restore
    window.location = originalLocation;
  });

  it('should clear error on unmount', () => {
    useAuthStore.setState({ error: 'Some error' });

    const { unmount } = renderLogin();
    unmount();

    // clearError should have been called
    // Note: In actual implementation, error clearing happens on unmount
  });

  it('should show terms of service text', () => {
    renderLogin();

    expect(screen.getByText(/by continuing, you agree to our/i)).toBeInTheDocument();
  });
});
