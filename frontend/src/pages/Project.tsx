import { useParams, Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';

import { api } from '../api/client';

interface ProjectDetail {
  id: string;
  title: string;
  description: string | null;
  status: string;
  audiobook_status: string | null;
  podcast_status: string | null;
  created_at: string;
}

interface Chapter {
  id: string;
  title: string;
  chapter_number: number;
  word_count: number | null;
  status: string;
}

const STATUS_LABELS: Record<string, string> = {
  processing: 'Processing your document...',
  parsing: 'Parsing PDF...',
  detecting_chapters: 'Detecting chapters...',
  saving: 'Saving results...',
  ready: 'Ready',
  failed: 'Processing failed',
};

function ProcessingStatus({ status }: { status: string }) {
  if (status === 'ready' || status === 'failed') return null;

  return (
    <div className="card p-6 mb-8">
      <div className="flex items-center space-x-3">
        <div className="animate-spin h-5 w-5 border-2 border-indigo-600 border-t-transparent rounded-full" />
        <p className="text-gray-700 font-medium">
          {STATUS_LABELS[status] || 'Processing...'}
        </p>
      </div>
    </div>
  );
}

export default function Project() {
  const { id } = useParams<{ id: string }>();

  const { data: project, isLoading } = useQuery<ProjectDetail>({
    queryKey: ['project', id],
    queryFn: () => api.get(`/api/projects/${id}`),
    refetchInterval: (query) => {
      const status = query.state.data?.status;
      // Poll every 3s while processing
      return status === 'processing' ? 3000 : false;
    },
  });

  const { data: chapters } = useQuery<Chapter[]>({
    queryKey: ['project-chapters', id],
    queryFn: () => api.get(`/api/projects/${id}/chapters`),
    enabled: project?.status === 'ready',
  });

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-500">Loading...</div>
      </div>
    );
  }

  if (!project) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-500">Project not found</div>
      </div>
    );
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="mb-2">
        <Link to="/library" className="text-sm text-indigo-600 hover:text-indigo-800">← Library</Link>
      </div>
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">{project.title}</h1>
        {project.description && (
          <p className="mt-2 text-gray-500">{project.description}</p>
        )}
      </div>

      <ProcessingStatus status={project.status} />

      {project.status === 'failed' && (
        <div className="card p-6 mb-8 border-red-200 bg-red-50">
          <p className="text-red-700 font-medium">Processing failed</p>
          <p className="text-sm text-red-600 mt-1">Please try uploading again.</p>
        </div>
      )}

      {/* Chapters */}
      {chapters && chapters.length > 0 && (
        <div className="card p-6 mb-8">
          <h2 className="text-lg font-medium text-gray-900 mb-4">
            Chapters ({chapters.length})
          </h2>
          <div className="divide-y">
            {chapters.map((ch) => (
              <div key={ch.id} className="py-3 flex items-center justify-between">
                <div>
                  <p className="font-medium text-gray-900">{ch.title}</p>
                  {ch.word_count && (
                    <p className="text-sm text-gray-500">{ch.word_count.toLocaleString()} words</p>
                  )}
                </div>
                <span className={`px-2 py-1 text-xs rounded-full ${
                  ch.status === 'ready' ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
                }`}>
                  {ch.status}
                </span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Actions */}
      {project.status === 'ready' && (
        <div className="flex space-x-4">
          <button className="btn-primary">Generate Audiobook</button>
          <button className="btn-secondary">Generate Podcast</button>
        </div>
      )}
    </div>
  );
}
