/**
 * Empty Library State Component
 * 
 * Shown when user has no projects. Offers:
 * - Try a Sample button
 * - Upload first PDF button
 */

import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { api } from '../../lib/api';

interface EmptyLibraryProps {
  onUploadClick: () => void;
}

export default function EmptyLibrary({ onUploadClick }: EmptyLibraryProps) {
  const navigate = useNavigate();
  const [isLoadingSample, setIsLoadingSample] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleTrySample = async () => {
    setIsLoadingSample(true);
    setError(null);

    try {
      const result = await api.post<{ project: { id: string } }>('/api/samples/try');
      // Navigate to the sample project
      navigate(`/projects/${result.project.id}`);
    } catch (err) {
      console.error('Failed to create sample:', err);
      setError('Failed to load sample. Please try again.');
      setIsLoadingSample(false);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center py-16 px-4">
      {/* Icon */}
      <div className="w-24 h-24 rounded-full bg-neutral-800 flex items-center justify-center mb-6">
        <svg
          className="w-12 h-12 text-neutral-500"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={1.5}
            d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
          />
        </svg>
      </div>

      {/* Title */}
      <h2 className="text-2xl font-semibold text-white mb-2">Your library is empty</h2>
      <p className="text-neutral-400 text-center max-w-md mb-8">
        Upload a PDF document and we'll transform it into an intelligent audiobook
        you can listen to anywhere.
      </p>

      {/* Error message */}
      {error && (
        <div className="mb-4 p-3 bg-red-900/30 border border-red-700 rounded-lg text-red-200 text-sm">
          {error}
        </div>
      )}

      {/* Actions */}
      <div className="flex flex-col sm:flex-row gap-4 w-full max-w-md">
        {/* Try Sample */}
        <button
          onClick={handleTrySample}
          disabled={isLoadingSample}
          className="flex-1 flex items-center justify-center gap-2 px-6 py-3 bg-neutral-800 hover:bg-neutral-700 text-white rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isLoadingSample ? (
            <>
              <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white"></div>
              Loading...
            </>
          ) : (
            <>
              <span className="text-lg">✨</span>
              Try a Sample
            </>
          )}
        </button>

        {/* Upload */}
        <button
          onClick={onUploadClick}
          className="flex-1 flex items-center justify-center gap-2 px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors"
        >
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 4v16m8-8H4"
            />
          </svg>
          Upload a PDF
        </button>
      </div>

      {/* Hint */}
      <p className="text-neutral-500 text-sm mt-6">
        Try the sample to see how Disona works — no upload required!
      </p>
    </div>
  );
}
