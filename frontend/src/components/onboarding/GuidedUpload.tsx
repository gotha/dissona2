/**
 * Guided Upload Component
 * 
 * Step-by-step guidance for first-time uploaders.
 * Shows tips, encouraging messages, and celebration on completion.
 */

import { useState, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { useDropzone } from 'react-dropzone';
import { api } from '../../lib/api';
import { useAuthStore } from '../../stores/authStore';

interface GuidedUploadProps {
  onSkip?: () => void;
  onComplete?: (projectId: string) => void;
}

type UploadStep = 'tips' | 'selecting' | 'uploading' | 'processing' | 'complete';

const PROCESSING_MESSAGES = [
  'Analyzing your document...',
  'Detecting chapters...',
  'Extracting key points...',
  'Preparing your audiobook...',
  'Almost there...',
];

export default function GuidedUpload({ onSkip, onComplete }: GuidedUploadProps) {
  const navigate = useNavigate();
  const { user, setUser } = useAuthStore();
  const [step, setStep] = useState<UploadStep>('tips');
  const [uploadProgress, setUploadProgress] = useState(0);
  const [processingMessage, setProcessingMessage] = useState(PROCESSING_MESSAGES[0]);
  const [processingProgress, setProcessingProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);
  const [projectId, setProjectId] = useState<string | null>(null);
  const [projectTitle, setProjectTitle] = useState<string>('');
  const [chaptersCount, setChaptersCount] = useState(0);

  // Cycle through processing messages
  const cycleMessages = useCallback(() => {
    let index = 0;
    return setInterval(() => {
      index = (index + 1) % PROCESSING_MESSAGES.length;
      setProcessingMessage(PROCESSING_MESSAGES[index]);
    }, 3000);
  }, []);

  const onDrop = useCallback(async (acceptedFiles: File[]) => {
    if (acceptedFiles.length === 0) return;

    const file = acceptedFiles[0];
    setStep('uploading');
    setError(null);

    try {
      // Get presigned upload URL
      const { upload_url, project_id } = await api.post<{
        upload_url: string;
        project_id: string;
      }>('/api/uploads/presign', {
        filename: file.name,
        content_type: file.type,
        size: file.size,
      });

      setProjectId(project_id);

      // Upload file with progress
      await uploadWithProgress(upload_url, file, setUploadProgress);

      // Notify backend that upload is complete
      await api.post(`/api/projects/${project_id}/process`);

      // Start processing phase
      setStep('processing');
      const messageInterval = cycleMessages();

      // Poll for processing status
      await pollProcessingStatus(project_id, (progress, title, chapters) => {
        setProcessingProgress(progress);
        if (title) setProjectTitle(title);
        if (chapters) setChaptersCount(chapters);
      });

      clearInterval(messageInterval);

      // Mark first upload complete
      if (user && !user.hasCompletedFirstUpload) {
        setUser({ ...user, hasCompletedFirstUpload: true });
      }

      setStep('complete');
    } catch (err) {
      console.error('Upload error:', err);
      setError(err instanceof Error ? err.message : 'Upload failed');
      setStep('tips');
    }
  }, [cycleMessages, user, setUser]);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: { 'application/pdf': ['.pdf'] },
    maxFiles: 1,
    maxSize: 100 * 1024 * 1024, // 100MB
  });

  const handleContinue = () => {
    if (projectId) {
      onComplete?.(projectId);
      navigate(`/projects/${projectId}`);
    }
  };

  return (
    <div className="max-w-xl mx-auto p-6">
      {/* Step: Tips */}
      {step === 'tips' && (
        <div className="text-center">
          <h2 className="text-2xl font-semibold text-white mb-4">
            Upload your first PDF
          </h2>
          
          <div className="bg-neutral-800 rounded-lg p-6 mb-6 text-left">
            <h3 className="text-lg font-medium text-white mb-4">
              📄 Tips for best results:
            </h3>
            <ul className="space-y-3 text-neutral-300">
              <li className="flex items-start gap-2">
                <span className="text-green-500 mt-0.5">✓</span>
                <span>Text-based PDFs work best (not scanned images)</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-500 mt-0.5">✓</span>
                <span>Chapters are automatically detected from headings</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-500 mt-0.5">✓</span>
                <span>Processing typically takes 1-2 minutes</span>
              </li>
            </ul>
          </div>

          {error && (
            <div className="mb-4 p-3 bg-red-900/30 border border-red-700 rounded-lg text-red-200 text-sm">
              {error}
            </div>
          )}

          <div
            {...getRootProps()}
            className={`border-2 border-dashed rounded-lg p-8 cursor-pointer transition-colors ${
              isDragActive
                ? 'border-blue-500 bg-blue-500/10'
                : 'border-neutral-600 hover:border-neutral-500'
            }`}
          >
            <input {...getInputProps()} />
            <div className="text-center">
              <svg
                className="w-12 h-12 text-neutral-500 mx-auto mb-4"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                />
              </svg>
              <p className="text-white font-medium mb-1">
                {isDragActive ? 'Drop your PDF here' : 'Select a PDF file'}
              </p>
              <p className="text-neutral-400 text-sm">
                or drag and drop (max 100MB)
              </p>
            </div>
          </div>

          {onSkip && (
            <button
              onClick={onSkip}
              className="mt-4 text-neutral-400 hover:text-white text-sm"
            >
              Skip tutorial
            </button>
          )}
        </div>
      )}

      {/* Step: Uploading */}
      {step === 'uploading' && (
        <div className="text-center">
          <div className="w-16 h-16 mx-auto mb-6">
            <svg className="animate-pulse text-blue-500" viewBox="0 0 24 24" fill="currentColor">
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <polyline points="14 2 14 8 20 8" fill="none" stroke="currentColor" strokeWidth="2" />
            </svg>
          </div>
          <h2 className="text-xl font-semibold text-white mb-2">Uploading...</h2>
          <div className="w-full bg-neutral-700 rounded-full h-2 mb-2">
            <div
              className="bg-blue-500 h-2 rounded-full transition-all duration-300"
              style={{ width: `${uploadProgress}%` }}
            />
          </div>
          <p className="text-neutral-400">{uploadProgress}%</p>
        </div>
      )}

      {/* Step: Processing */}
      {step === 'processing' && (
        <div className="text-center">
          <div className="w-20 h-20 mx-auto mb-6 relative">
            <div className="animate-spin rounded-full h-20 w-20 border-4 border-neutral-700 border-t-blue-500" />
            <span className="absolute inset-0 flex items-center justify-center text-2xl">📚</span>
          </div>
          <h2 className="text-xl font-semibold text-white mb-2">Creating your audiobook...</h2>
          <p className="text-neutral-400 mb-4">{processingMessage}</p>
          {chaptersCount > 0 && (
            <p className="text-green-400 mb-4">Found {chaptersCount} chapters! 📚</p>
          )}
          <div className="w-full bg-neutral-700 rounded-full h-2 mb-2">
            <div
              className="bg-blue-500 h-2 rounded-full transition-all duration-500"
              style={{ width: `${processingProgress}%` }}
            />
          </div>
          <p className="text-neutral-500 text-sm mt-4">
            💡 Tip: Dissona creates summaries so you can get the key points in minutes!
          </p>
        </div>
      )}

      {/* Step: Complete */}
      {step === 'complete' && (
        <div className="text-center">
          <div className="text-6xl mb-6">🎉</div>
          <h2 className="text-2xl font-semibold text-white mb-2">Your audiobook is ready!</h2>
          <div className="bg-neutral-800 rounded-lg p-6 mb-6">
            <div className="text-4xl mb-2">📘</div>
            <h3 className="text-lg font-medium text-white">{projectTitle || 'Your Project'}</h3>
            <p className="text-neutral-400">{chaptersCount} chapters</p>
          </div>
          <div className="flex flex-col gap-3">
            <button
              onClick={handleContinue}
              className="w-full px-6 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors flex items-center justify-center gap-2"
            >
              <span>🎧</span> Generate Audio
            </button>
            <button
              onClick={handleContinue}
              className="w-full px-6 py-3 bg-neutral-700 hover:bg-neutral-600 text-white rounded-lg transition-colors flex items-center justify-center gap-2"
            >
              <span>📖</span> View Chapters
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

// Helper: Upload with progress tracking
async function uploadWithProgress(
  url: string,
  file: File,
  onProgress: (progress: number) => void
): Promise<void> {
  return new Promise((resolve, reject) => {
    const xhr = new XMLHttpRequest();
    xhr.upload.addEventListener('progress', (e) => {
      if (e.lengthComputable) {
        onProgress(Math.round((e.loaded / e.total) * 100));
      }
    });
    xhr.addEventListener('load', () => {
      if (xhr.status >= 200 && xhr.status < 300) {
        resolve();
      } else {
        reject(new Error(`Upload failed: ${xhr.status}`));
      }
    });
    xhr.addEventListener('error', () => reject(new Error('Upload failed')));
    xhr.open('PUT', url);
    xhr.setRequestHeader('Content-Type', file.type);
    xhr.send(file);
  });
}

// Helper: Poll processing status
async function pollProcessingStatus(
  projectId: string,
  onProgress: (progress: number, title?: string, chapters?: number) => void
): Promise<void> {
  const POLL_INTERVAL = 2000;
  const MAX_ATTEMPTS = 120; // 4 minutes max

  for (let i = 0; i < MAX_ATTEMPTS; i++) {
    const status = await api.get<{
      status: string;
      progress: number;
      title?: string;
      chapters_count?: number;
    }>(`/api/projects/${projectId}/status`);

    onProgress(status.progress, status.title, status.chapters_count);

    if (status.status === 'ready') {
      return;
    }

    if (status.status === 'failed') {
      throw new Error('Processing failed');
    }

    await new Promise((r) => setTimeout(r, POLL_INTERVAL));
  }

  throw new Error('Processing timeout');
}
