import { useParams } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';

import { api } from '../api/client';

export default function Project() {
  const { id } = useParams<{ id: string }>();

  const { data: project, isLoading } = useQuery({
    queryKey: ['project', id],
    queryFn: () => api.get(`/api/projects/${id}`),
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
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">{project.title}</h1>
        {project.description && (
          <p className="mt-2 text-gray-500">{project.description}</p>
        )}
      </div>

      {/* Status */}
      <div className="card p-6 mb-8">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Status</h2>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div>
            <p className="text-sm text-gray-500">Document Status</p>
            <p className="font-medium">{project.status}</p>
          </div>
          <div>
            <p className="text-sm text-gray-500">Audiobook</p>
            <p className="font-medium">{project.audiobook_status || 'Not started'}</p>
          </div>
          <div>
            <p className="text-sm text-gray-500">Podcast</p>
            <p className="font-medium">{project.podcast_status || 'Not started'}</p>
          </div>
        </div>
      </div>

      {/* Actions */}
      <div className="flex space-x-4">
        <button className="btn-primary">Generate Audiobook</button>
        <button className="btn-secondary">Generate Podcast</button>
        <button className="btn-secondary">Add Documents</button>
      </div>
    </div>
  );
}
