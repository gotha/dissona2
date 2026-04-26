import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export interface User {
  id: string;
  email: string;
  name: string;
  avatarUrl?: string;
  hasCompletedFirstUpload?: boolean;
}

interface AuthState {
  user: User | null;
  accessToken: string | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  error: string | null;
  notificationPromptDismissedAt: string | null;

  // Actions
  setAuth: (user: User, accessToken: string) => void;
  setUser: (user: User) => void;
  logout: () => Promise<void>;
  refreshToken: () => Promise<boolean>;
  clearError: () => void;
  setError: (error: string) => void;
  dismissNotificationPrompt: () => void;
  shouldShowNotificationPrompt: () => boolean;
}

const AUTH_API_BASE = import.meta.env.VITE_AUTH_URL || '';

export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      user: null,
      accessToken: null,
      isAuthenticated: false,
      isLoading: false,
      error: null,
      notificationPromptDismissedAt: null,

      setAuth: (user, accessToken) => {
        set({ user, accessToken, isAuthenticated: true, error: null });
      },

      setUser: (user) => {
        set({ user });
      },

      logout: async () => {
        set({ isLoading: true });
        try {
          await fetch(`${AUTH_API_BASE}/auth/logout`, {
            method: 'POST',
            credentials: 'include',
          });
        } catch (error) {
          console.error('Logout error:', error);
        }
        set({
          user: null,
          accessToken: null,
          isAuthenticated: false,
          isLoading: false,
          error: null,
        });
        // Clear any cached data
        localStorage.removeItem('progress-storage');
      },

      refreshToken: async () => {
        try {
          const response = await fetch(`${AUTH_API_BASE}/auth/refresh`, {
            method: 'POST',
            credentials: 'include',
          });

          if (!response.ok) {
            if (response.status === 401) {
              set({
                user: null,
                accessToken: null,
                isAuthenticated: false,
                error: 'Your session has expired. Please sign in again.',
              });
              return false;
            }
            throw new Error('Token refresh failed');
          }

          const data = await response.json();
          set({ accessToken: data.access_token, error: null });
          return true;
        } catch (error) {
          console.error('Token refresh error:', error);
          await get().logout();
          return false;
        }
      },

      clearError: () => set({ error: null }),

      setError: (error) => set({ error }),

      dismissNotificationPrompt: () => {
        set({ notificationPromptDismissedAt: new Date().toISOString() });
      },

      shouldShowNotificationPrompt: () => {
        const { notificationPromptDismissedAt, isAuthenticated } = get();
        if (!isAuthenticated) return false;
        if (!notificationPromptDismissedAt) return true;

        // Show again after 7 days
        const dismissedDate = new Date(notificationPromptDismissedAt);
        const sevenDaysAgo = new Date();
        sevenDaysAgo.setDate(sevenDaysAgo.getDate() - 7);
        return dismissedDate < sevenDaysAgo;
      },
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({
        user: state.user,
        accessToken: state.accessToken,
        isAuthenticated: state.isAuthenticated,
        notificationPromptDismissedAt: state.notificationPromptDismissedAt,
      }),
    }
  )
);
