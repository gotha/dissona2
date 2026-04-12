import { useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { useAuthStore } from '../stores/authStore';

export default function AuthCallback() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const { setAuth, setError } = useAuthStore();
  const [status, setStatus] = useState<'loading' | 'success' | 'error'>('loading');

  useEffect(() => {
    const accessToken = searchParams.get('access_token');
    const error = searchParams.get('error');
    const errorMessage = searchParams.get('message');

    // Handle OAuth error
    if (error) {
      setError(errorMessage || 'Authentication failed');
      navigate('/login?error=' + error, { replace: true });
      return;
    }

    if (accessToken) {
      // Decode JWT to get user info
      try {
        const payload = JSON.parse(atob(accessToken.split('.')[1]));
        setAuth(
          {
            id: payload.sub,
            email: payload.email,
            name: payload.name,
            avatarUrl: payload.avatar_url,
          },
          accessToken
        );
        setStatus('success');
        // Short delay to show success state
        setTimeout(() => {
          navigate('/', { replace: true });
        }, 500);
      } catch (err) {
        console.error('Failed to parse access token:', err);
        setStatus('error');
        setError('Failed to process authentication');
        setTimeout(() => {
          navigate('/login?error=invalid_token', { replace: true });
        }, 1500);
      }
    } else {
      setStatus('error');
      setTimeout(() => {
        navigate('/login', { replace: true });
      }, 1000);
    }
  }, [searchParams, setAuth, setError, navigate]);

  return (
    <div className="min-h-screen flex items-center justify-center bg-neutral-950">
      <div className="text-center">
        {status === 'loading' && (
          <>
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-white mx-auto mb-4"></div>
            <p className="text-neutral-400">Signing you in...</p>
          </>
        )}
        {status === 'success' && (
          <>
            <div className="text-green-500 text-4xl mb-4">✓</div>
            <p className="text-white">Welcome back!</p>
          </>
        )}
        {status === 'error' && (
          <>
            <div className="text-red-500 text-4xl mb-4">✗</div>
            <p className="text-neutral-400">Something went wrong. Redirecting...</p>
          </>
        )}
      </div>
    </div>
  );
}
