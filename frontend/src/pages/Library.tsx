import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { Link } from 'react-router-dom';

import { api } from '../api/client';

interface Project {
  id: string;
  title: string;
  description: string | null;
  status: string;
  audiobook_status: string | null;
  podcast_status: string | null;
  is_sample?: boolean;
  created_at: string;
}

function EmptyLibrary({ onTrySample, isLoading }: { onTrySample: () => void; isLoading: boolean }) {
  return (
    <div className="text-center py-16">
      <div className="mx-auto h-16 w-16 text-gray-300 mb-4">
        <svg fill="none" viewBox="0 0 24 24" strokeWidth={1} stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" d="M12 6.042A8.967 8.967 0 0 0 6 3.75c-1.052 0-2.062.18-3 .512v14.25A8.987 8.987 0 0 1 6 18c2.305 0 4.408.867 6 2.292m0-14.25a8.966 8.966 0 0 1 6-2.292c1.052 0 2.062.18 3 .512v14.25A8.987 8.987 0 0 0 18 18a8.967 8.967 0 0 0-6 2.292m0-14.25v14.25" />
        </svg>
      </div>
      <h2 className="text-xl font-semibold text-gray-900 mb-2">Your library is empty</h2>
      <p className="text-gray-500 mb-6 max-w-md mx-auto">
        Upload your first document to turn it into an audiobook or podcast,
        or try a sample to see how it works.
      </p>
      <div className="flex items-center justify-center gap-4">
        <Link to="/upload" className="btn-primary">
          Upload Document
        </Link>
        <button
          onClick={onTrySample}
          disabled={isLoading}
          className="px-4 py-2 text-sm font-medium text-indigo-600 bg-indigo-50 rounded-lg hover:bg-indigo-100 disabled:opacity-50 transition-colors"
        >
          {isLoading ? 'Creating...' : '✨ Try a Sample'}
        </button>
      </div>
    </div>
  );
}

function ProjectCard({ project }: { project: Project }) {
  return (
    <Link
      to={`/project/${project.id}`}
      className="card p-6 hover:shadow-md transition-shadow"
    >
      <div className="flex items-start justify-between">
        <h3 className="text-lg font-medium text-gray-900 truncate flex-1">
          {project.title}
        </h3>
        {project.is_sample && (
          <span className="ml-2 px-2 py-0.5 text-xs font-medium rounded-full bg-indigo-100 text-indigo-700 whitespace-nowrap">
            Sample
          </span>
        )}
      </div>
      {project.description && (
        <p className="mt-2 text-sm text-gray-500 line-clamp-2">
          {project.description}
        </p>
      )}
      <div className="mt-4 flex items-center space-x-2">
        <span
          className={`px-2 py-1 text-xs rounded-full ${
            project.status === 'ready'
              ? 'bg-green-100 text-green-800'
              : project.status === 'processing'
              ? 'bg-yellow-100 text-yellow-800'
              : 'bg-gray-100 text-gray-800'
          }`}
        >
          {project.status}
        </span>
      </div>
    </Link>
  );
}

export default function Library() {
  const queryClient = useQueryClient();

  const { data: projects, isLoading } = useQuery({
    queryKey: ['projects'],
    queryFn: () => api.get<Project[]>('/api/projects'),
    refetchInterval: (query) => {
      const hasProcessing = query.state.data?.some((p: Project) => p.status === 'processing');
      return hasProcessing ? 3000 : false;
    },
  });

  const trySampleMutation = useMutation({
    mutationFn: () => api.post('/api/samples/try'),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['projects'] });
    },
  });

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-500">Loading...</div>
      </div>
    );
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="flex items-center justify-between mb-8">
        <h1 className="text-2xl font-bold text-gray-900">Library</h1>
        <Link to="/upload" className="btn-primary">New Project</Link>
      </div>

      {projects?.length === 0 ? (
        <EmptyLibrary
          onTrySample={() => trySampleMutation.mutate()}
          isLoading={trySampleMutation.isPending}
        />
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {projects?.map((project) => (
            <ProjectCard key={project.id} project={project} />
          ))}
        </div>
      )}
    </div>
  );
}
