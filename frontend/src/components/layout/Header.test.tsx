import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter } from 'react-router-dom';
import Header from './Header';
import { useAuthStore } from '../../stores/authStore';

describe('Header', () => {
  const mockUser = {
    id: '1',
    email: 'test@example.com',
    name: 'Test User',
    avatarUrl: 'https://example.com/avatar.jpg',
  };

  beforeEach(() => {
    useAuthStore.setState({
      user: mockUser,
      accessToken: 'token',
      isAuthenticated: true,
      isLoading: false,
      error: null,
      notificationPromptDismissedAt: null,
    });
  });

  const renderHeader = () => {
    return render(
      <MemoryRouter>
        <Header />
      </MemoryRouter>
    );
  };

  it('should render logo', () => {
    renderHeader();
    expect(screen.getByText('Dissona')).toBeInTheDocument();
  });

  it('should render navigation links', () => {
    renderHeader();
    expect(screen.getByRole('link', { name: /library/i })).toBeInTheDocument();
    expect(screen.getByRole('link', { name: /settings/i })).toBeInTheDocument();
  });

  it('should display user avatar when available', () => {
    renderHeader();
    const avatar = screen.getByRole('img', { name: /test user/i });
    expect(avatar).toHaveAttribute('src', mockUser.avatarUrl);
  });

  it('should display initials when no avatar', () => {
    useAuthStore.setState({
      ...useAuthStore.getState(),
      user: { ...mockUser, avatarUrl: undefined },
    });

    renderHeader();
    expect(screen.getByText('TU')).toBeInTheDocument();
  });

  it('should toggle dropdown menu on avatar click', async () => {
    const user = userEvent.setup();
    renderHeader();

    // Initially dropdown is closed
    expect(screen.queryByText('Sign out')).not.toBeInTheDocument();

    // Find the avatar button (has the user's image)
    const avatarButton = screen.getByRole('img', { name: /test user/i }).closest('button')!;

    // Open dropdown
    await user.click(avatarButton);
    expect(screen.getByText('Sign out')).toBeInTheDocument();
    expect(screen.getByText(mockUser.name)).toBeInTheDocument();
    expect(screen.getByText(mockUser.email)).toBeInTheDocument();

    // Close dropdown by clicking avatar again
    await user.click(avatarButton);
    await waitFor(() => {
      expect(screen.queryByText('Sign out')).not.toBeInTheDocument();
    });
  });

  it('should show subscription link in dropdown', async () => {
    const user = userEvent.setup();
    renderHeader();

    const avatarButton = screen.getByRole('img', { name: /test user/i }).closest('button')!;
    await user.click(avatarButton);
    expect(screen.getByRole('link', { name: /subscription/i })).toBeInTheDocument();
  });

  it('should call logout on sign out click', async () => {
    const user = userEvent.setup();
    const logoutSpy = vi.spyOn(useAuthStore.getState(), 'logout');

    renderHeader();

    const avatarButton = screen.getByRole('img', { name: /test user/i }).closest('button')!;
    await user.click(avatarButton);
    await user.click(screen.getByText('Sign out'));

    expect(logoutSpy).toHaveBeenCalled();
  });

  it('should close dropdown when clicking outside', async () => {
    const user = userEvent.setup();
    renderHeader();

    const avatarButton = screen.getByRole('img', { name: /test user/i }).closest('button')!;

    // Open dropdown
    await user.click(avatarButton);
    expect(screen.getByText('Sign out')).toBeInTheDocument();

    // Click outside (on the logo)
    await user.click(screen.getByText('Dissona'));

    await waitFor(() => {
      expect(screen.queryByText('Sign out')).not.toBeInTheDocument();
    });
  });

  it('should highlight current route', () => {
    render(
      <MemoryRouter initialEntries={['/settings']}>
        <Header />
      </MemoryRouter>
    );

    const settingsLink = screen.getByRole('link', { name: /settings/i });
    expect(settingsLink).toHaveClass('bg-neutral-800');
  });

  it('should show loading state during logout', async () => {
    const user = userEvent.setup();

    renderHeader();
    const avatarButton = screen.getByRole('img', { name: /test user/i }).closest('button')!;

    // Open dropdown first
    await user.click(avatarButton);

    // Set loading state while dropdown is open
    useAuthStore.setState({ ...useAuthStore.getState(), isLoading: true });

    // Re-render to pick up state change
    renderHeader();
    const avatarButton2 = screen.getAllByRole('img', { name: /test user/i })[0].closest('button')!;
    await user.click(avatarButton2);

    expect(screen.getByText('Signing out...')).toBeInTheDocument();
  });

  it('should disable avatar button during logout', () => {
    useAuthStore.setState({ ...useAuthStore.getState(), isLoading: true });
    renderHeader();

    const avatarButton = screen.getByRole('img', { name: /test user/i }).closest('button')!;
    expect(avatarButton).toBeDisabled();
  });
});
