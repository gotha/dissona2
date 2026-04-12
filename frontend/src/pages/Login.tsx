import { useNavigate, useSearchParams } from 'react-router-dom';
import { useAuthStore } from '../stores/authStore';
import { useEffect } from 'react';

const AUTH_URL = import.meta.env.VITE_AUTH_URL || 'http://localhost:8081';

export default function Login() {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const { isAuthenticated, error, clearError } = useAuthStore();

  // Check for error in URL (from OAuth redirect)
  const urlError = searchParams.get('error');
  const errorMessage = searchParams.get('message');
  const sessionExpired = searchParams.get('expired') === 'true';

  useEffect(() => {
    if (isAuthenticated) {
      navigate('/');
    }
  }, [isAuthenticated, navigate]);

  // Clear error when component unmounts
  useEffect(() => {
    return () => clearError();
  }, [clearError]);

  const handleGoogleLogin = () => {
    window.location.href = `${AUTH_URL}/auth/google`;
  };

  const displayError = error || (urlError && (errorMessage || 'Authentication failed. Please try again.'));

  return (
    <div className="min-h-screen flex items-center justify-center bg-neutral-950">
      <div className="max-w-md w-full space-y-8 p-8">
        <div className="text-center">
          <h1 className="text-4xl font-bold text-white mb-2">Disona</h1>
          <p className="text-neutral-400">Transform documents into audio</p>
        </div>

        {/* Session expired message */}
        {sessionExpired && (
          <div className="bg-amber-900/30 border border-amber-700 rounded-lg p-4 text-amber-200 text-sm">
            Your session has expired. Please sign in again.
          </div>
        )}

        {/* Error message */}
        {displayError && !sessionExpired && (
          <div className="bg-red-900/30 border border-red-700 rounded-lg p-4 text-red-200 text-sm">
            {displayError}
          </div>
        )}

        <div className="mt-8 space-y-4">
          <button
            onClick={handleGoogleLogin}
            className="w-full flex items-center justify-center gap-3 px-4 py-3 border border-neutral-700 rounded-lg text-white hover:bg-neutral-800 transition-colors"
          >
            <svg className="w-5 h-5" viewBox="0 0 24 24">
              <path
                fill="#4285F4"
                d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
              />
              <path
                fill="#34A853"
                d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
              />
              <path
                fill="#FBBC05"
                d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
              />
              <path
                fill="#EA4335"
                d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
              />
            </svg>
            Continue with Google
          </button>
        </div>

        <p className="text-center text-sm text-neutral-500">
          By continuing, you agree to our Terms of Service and Privacy Policy
        </p>
      </div>
    </div>
  );
}
