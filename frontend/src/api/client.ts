import axios from 'axios';
import { useAuthStore } from '../stores/authStore';

const apiClient = axios.create({
  baseURL: import.meta.env.VITE_API_URL || '',
  headers: {
    'Content-Type': 'application/json',
  },
});

// Request interceptor to add auth token
apiClient.interceptors.request.use(
  (config) => {
    const accessToken = useAuthStore.getState().accessToken;
    if (accessToken) {
      config.headers.Authorization = `Bearer ${accessToken}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor to handle token refresh
apiClient.interceptors.response.use(
  (response) => response,
  async (error) => {
    const originalRequest = error.config;

    // If 401 and not already retrying
    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;

      try {
        await useAuthStore.getState().refreshToken();
        const newToken = useAuthStore.getState().accessToken;
        originalRequest.headers.Authorization = `Bearer ${newToken}`;
        return apiClient(originalRequest);
      } catch {
        useAuthStore.getState().logout();
        window.location.href = '/login';
      }
    }

    return Promise.reject(error);
  }
);

export const api = {
  get: async <T>(url: string): Promise<T> => {
    const response = await apiClient.get<T>(url);
    return response.data;
  },

  post: async <T>(url: string, data?: unknown): Promise<T> => {
    const response = await apiClient.post<T>(url, data);
    return response.data;
  },

  put: async <T>(url: string, data?: unknown): Promise<T> => {
    const response = await apiClient.put<T>(url, data);
    return response.data;
  },

  delete: async <T>(url: string): Promise<T> => {
    const response = await apiClient.delete<T>(url);
    return response.data;
  },
};

export default apiClient;
