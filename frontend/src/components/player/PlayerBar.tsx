import { usePlayerStore } from '../../stores/playerStore';

export default function PlayerBar() {
  const {
    isPlaying,
    currentTime,
    duration,
    play,
    pause,
    seek,
  } = usePlayerStore();

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const progress = duration > 0 ? (currentTime / duration) * 100 : 0;

  return (
    <div className="fixed bottom-0 left-0 right-0 bg-white border-t border-gray-200 shadow-lg">
      {/* Progress bar */}
      <div className="h-1 bg-gray-200">
        <div
          className="h-full bg-primary-600 transition-all"
          style={{ width: `${progress}%` }}
        />
      </div>

      <div className="flex items-center justify-between px-4 py-3">
        {/* Track info */}
        <div className="flex-1 min-w-0">
          <p className="font-medium truncate">Chapter Title</p>
          <p className="text-sm text-gray-500 truncate">Project Title</p>
        </div>

        {/* Controls */}
        <div className="flex items-center space-x-4">
          <button
            onClick={() => seek(Math.max(0, currentTime - 15))}
            className="p-2 text-gray-500 hover:text-gray-900"
          >
            <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12.066 11.2a1 1 0 000 1.6l5.334 4A1 1 0 0019 16V8a1 1 0 00-1.6-.8l-5.333 4zM4.066 11.2a1 1 0 000 1.6l5.334 4A1 1 0 0011 16V8a1 1 0 00-1.6-.8l-5.334 4z" />
            </svg>
          </button>

          <button
            onClick={isPlaying ? pause : play}
            className="p-3 bg-primary-600 text-white rounded-full hover:bg-primary-700"
          >
            {isPlaying ? (
              <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            ) : (
              <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            )}
          </button>

          <button
            onClick={() => seek(Math.min(duration, currentTime + 30))}
            className="p-2 text-gray-500 hover:text-gray-900"
          >
            <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11.933 12.8a1 1 0 000-1.6L6.6 7.2A1 1 0 005 8v8a1 1 0 001.6.8l5.333-4zM19.933 12.8a1 1 0 000-1.6l-5.333-4A1 1 0 0013 8v8a1 1 0 001.6.8l5.333-4z" />
            </svg>
          </button>
        </div>

        {/* Time */}
        <div className="flex-1 text-right">
          <span className="text-sm text-gray-500">
            {formatTime(currentTime)} / {formatTime(duration)}
          </span>
        </div>
      </div>
    </div>
  );
}
