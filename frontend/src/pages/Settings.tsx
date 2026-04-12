import { useAuthStore } from '../stores/authStore';
import { usePlayerStore } from '../stores/playerStore';

export default function Settings() {
  const user = useAuthStore((state) => state.user);
  const { playbackRate, setPlaybackRate, listeningMode } = usePlayerStore();

  return (
    <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <h1 className="text-2xl font-bold text-gray-900 mb-8">Settings</h1>

      {/* Account */}
      <div className="card p-6 mb-6">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Account</h2>
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-gray-500">Email</label>
            <p className="font-medium">{user?.email}</p>
          </div>
          <div>
            <label className="block text-sm text-gray-500">Name</label>
            <p className="font-medium">{user?.name}</p>
          </div>
        </div>
      </div>

      {/* Playback */}
      <div className="card p-6 mb-6">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Playback</h2>
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-gray-500 mb-2">
              Playback Speed
            </label>
            <div className="flex space-x-2">
              {[0.75, 1, 1.25, 1.5, 2].map((rate) => (
                <button
                  key={rate}
                  onClick={() => setPlaybackRate(rate)}
                  className={`px-3 py-1 rounded ${
                    playbackRate === rate
                      ? 'bg-primary-600 text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  {rate}x
                </button>
              ))}
            </div>
          </div>

          <div>
            <label className="block text-sm text-gray-500 mb-2">
              Default Listening Mode
            </label>
            <p className="font-medium capitalize">{listeningMode}</p>
          </div>
        </div>
      </div>

      {/* Storage */}
      <div className="card p-6">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Storage</h2>
        <p className="text-gray-500 text-sm">
          Downloaded content will be stored here for offline use.
        </p>
        <button className="mt-4 btn-secondary">Clear Downloads</button>
      </div>
    </div>
  );
}
