import { useEffect } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { useAuthStore } from '../stores/authStore';

export default function AuthCallback() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const setAuth = useAuthStore((state) => state.setAuth);

  useEffect(() => {
    const accessToken = searchParams.get('access_token');

    if (accessToken) {
      // Decode JWT to get user info (simplified - in production use a proper JWT library)
      try {
        const payload = JSON.parse(atob(accessToken.split('.')[1]));
        setAuth(
          {
            id: payload.sub,
            email: payload.email,
            name: payload.name,
          },
          accessToken
        );
        navigate('/', { replace: true });
      } catch {
        console.error('Failed to parse access token');
        navigate('/login', { replace: true });
      }
    } else {
      navigate('/login', { replace: true });
    }
  }, [searchParams, setAuth, navigate]);

  return (
    <div className="min-h-screen flex items-center justify-center">
      <div className="text-gray-500">Signing in...</div>
    </div>
  );
}
