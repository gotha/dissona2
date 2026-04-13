/**
 * Notification Prompt Component
 * 
 * Shows a friendly prompt to enable push notifications.
 * - Appears after first upload starts processing
 * - Can be dismissed for 7 days
 * - Explains the benefit clearly
 */

import { useState, useEffect } from 'react';
import { useAuthStore } from '../../stores/authStore';
import { subscribeToPush, isPushSupported } from '../../services/push';

interface NotificationPromptProps {
  onClose?: () => void;
}

export default function NotificationPrompt({ onClose }: NotificationPromptProps) {
  const { shouldShowNotificationPrompt, dismissNotificationPrompt } = useAuthStore();
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [permissionState, setPermissionState] = useState<NotificationPermission | null>(null);

  useEffect(() => {
    // Check current permission state
    if ('Notification' in window) {
      setPermissionState(Notification.permission);
    }
  }, []);

  // Don't show if:
  // - Push not supported
  // - Already granted or denied
  // - User dismissed recently
  if (!isPushSupported()) return null;
  if (permissionState === 'granted' || permissionState === 'denied') return null;
  if (!shouldShowNotificationPrompt()) return null;

  const handleEnable = async () => {
    setIsLoading(true);
    setError(null);

    try {
      const success = await subscribeToPush();
      if (success) {
        setPermissionState('granted');
        onClose?.();
      } else {
        setError('Failed to enable notifications');
      }
    } catch (err) {
      console.error('Push subscription error:', err);
      setError('Failed to enable notifications');
    } finally {
      setIsLoading(false);
    }
  };

  const handleDismiss = () => {
    dismissNotificationPrompt();
    onClose?.();
  };

  return (
    <div className="fixed bottom-4 left-4 right-4 md:left-auto md:right-4 md:w-96 bg-neutral-800 rounded-xl shadow-2xl border border-neutral-700 p-4 z-50 animate-slide-up">
      {/* Header */}
      <div className="flex items-start gap-3 mb-4">
        <div className="w-10 h-10 rounded-full bg-blue-500/20 flex items-center justify-center flex-shrink-0">
          <span className="text-xl">🔔</span>
        </div>
        <div>
          <h3 className="text-white font-medium">Get notified when your audio is ready</h3>
          <p className="text-neutral-400 text-sm mt-1">
            We'll send you a notification so you don't have to wait around.
          </p>
        </div>
      </div>

      {/* Example notification */}
      <div className="bg-neutral-900 rounded-lg p-3 mb-4">
        <div className="flex items-center gap-2 text-sm">
          <span className="text-neutral-500">Preview:</span>
        </div>
        <div className="flex items-center gap-3 mt-2">
          <div className="w-8 h-8 rounded bg-blue-500 flex items-center justify-center text-white text-xs font-bold">
            D
          </div>
          <div>
            <p className="text-white text-sm font-medium">Dissona</p>
            <p className="text-neutral-400 text-xs">"Deep Work" is ready to listen!</p>
          </div>
        </div>
      </div>

      {/* Error */}
      {error && (
        <div className="text-red-400 text-sm mb-3">{error}</div>
      )}

      {/* Actions */}
      <div className="flex gap-3">
        <button
          onClick={handleEnable}
          disabled={isLoading}
          className="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-medium transition-colors disabled:opacity-50"
        >
          {isLoading ? 'Enabling...' : 'Enable'}
        </button>
        <button
          onClick={handleDismiss}
          className="px-4 py-2 text-neutral-400 hover:text-white text-sm transition-colors"
        >
          Not now
        </button>
      </div>
    </div>
  );
}
