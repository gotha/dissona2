import { useState, useRef, useEffect } from 'react';
import { useParams, useNavigate, Link } from 'react-router-dom';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';

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
  if (status !== 'processing') return null;

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

function EditableTitle({ title, projectId }: { title: string; projectId: string }) {
  const [editing, setEditing] = useState(false);
  const [value, setValue] = useState(title);
  const inputRef = useRef<HTMLInputElement>(null);
  const queryClient = useQueryClient();

  useEffect(() => { setValue(title); }, [title]);
  useEffect(() => { if (editing) inputRef.current?.select(); }, [editing]);

  const mutation = useMutation({
    mutationFn: (newTitle: string) =>
      api.put(`/api/projects/${projectId}`, { title: newTitle }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['project', projectId] });
      queryClient.invalidateQueries({ queryKey: ['projects'] });
      setEditing(false);
    },
  });

  const handleSubmit = () => {
    const trimmed = value.trim();
    if (trimmed && trimmed !== title) {
      mutation.mutate(trimmed);
    } else {
      setValue(title);
      setEditing(false);
    }
  };

  if (editing) {
    return (
      <div className="flex items-center gap-2">
        <input
          ref={inputRef}
          type="text"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') handleSubmit();
            if (e.key === 'Escape') { setValue(title); setEditing(false); }
          }}
          className="text-2xl font-bold text-gray-900 border-b-2 border-indigo-500 outline-none bg-transparent flex-1 py-0"
        />
        <button
          onClick={handleSubmit}
          disabled={mutation.isPending}
          className="px-3 py-1 text-sm bg-indigo-600 text-white rounded hover:bg-indigo-700"
        >
          {mutation.isPending ? '...' : 'OK'}
        </button>
        <button
          onClick={() => { setValue(title); setEditing(false); }}
          className="px-3 py-1 text-sm text-gray-500 hover:text-gray-700"
        >
          Cancel
        </button>
      </div>
    );
  }

  return (
    <div className="group flex items-center gap-2">
      <h1 className="text-2xl font-bold text-gray-900">{title}</h1>
      <button
        onClick={() => setEditing(true)}
        className="opacity-0 group-hover:opacity-100 transition-opacity p-1 text-gray-400 hover:text-gray-600"
        title="Edit title"
      >
        <svg className="h-4 w-4" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" d="m16.862 4.487 1.687-1.688a1.875 1.875 0 1 1 2.652 2.652L10.582 16.07a4.5 4.5 0 0 1-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 0 1 1.13-1.897l8.932-8.931Zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0 1 15.75 21H5.25A2.25 2.25 0 0 1 3 18.75V8.25A2.25 2.25 0 0 1 5.25 6H10" />
        </svg>
      </button>
    </div>
  );
}

export default function Project() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const queryClient = useQueryClient();

  const deleteMutation = useMutation({
    mutationFn: () => api.delete(`/api/projects/${id}`),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['projects'] });
      navigate('/library');
    },
  });

  const handleDelete = () => {
    if (window.confirm('Delete this project? This cannot be undone.')) {
      deleteMutation.mutate();
    }
  };

  const { data: project, isLoading } = useQuery<ProjectDetail>({
    queryKey: ['project', id],
    queryFn: () => api.get(`/api/projects/${id}`),
    refetchInterval: (query) => {
      const status = query.state.data?.status;
      // Poll every 3s while processing, stop when draft/ready/failed
      return status === 'processing' ? 3000 : false;
    },
  });

  const { data: chapters } = useQuery<Chapter[]>({
    queryKey: ['project-chapters', id],
    queryFn: () => api.get(`/api/projects/${id}/chapters`),
    enabled: project?.status === 'draft' || project?.status === 'ready',
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
      <div className="mb-2 flex items-center justify-between">
        <Link to="/library" className="text-sm text-indigo-600 hover:text-indigo-800">← Library</Link>
        <button
          onClick={handleDelete}
          disabled={deleteMutation.isPending}
          className="text-sm text-red-500 hover:text-red-700"
        >
          {deleteMutation.isPending ? 'Deleting...' : 'Delete project'}
        </button>
      </div>
      <div className="mb-8">
        <EditableTitle title={project.title} projectId={project.id} />
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
                {ch.status === 'ready' && (
                  <span className="px-2 py-1 text-xs rounded-full bg-green-100 text-green-800">
                    Audio ready
                  </span>
                )}
                {ch.status === 'generating' && (
                  <span className="px-2 py-1 text-xs rounded-full bg-yellow-100 text-yellow-800">
                    Generating audio...
                  </span>
                )}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Actions */}
      {(project.status === 'draft' || project.status === 'ready') && (
        <div className="flex space-x-4">
          <button className="btn-primary">Generate Audiobook</button>
          <button className="btn-secondary">Generate Podcast</button>
        </div>
      )}
    </div>
  );
}
