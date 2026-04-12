import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';

import { api } from '../api/client';

interface Project {
  id: string;
  title: string;
  description: string | null;
  status: string;
  audiobook_status: string | null;
  podcast_status: string | null;
  created_at: string;
}

export default function Library() {
  const { data: projects, isLoading } = useQuery({
    queryKey: ['projects'],
    queryFn: () => api.get<Project[]>('/api/projects'),
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
        <button className="btn-primary">New Project</button>
      </div>

      {projects?.length === 0 ? (
        <div className="text-center py-12">
          <p className="text-gray-500">No projects yet. Create your first one!</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {projects?.map((project) => (
            <Link
              key={project.id}
              to={`/project/${project.id}`}
              className="card p-6 hover:shadow-md transition-shadow"
            >
              <h3 className="text-lg font-medium text-gray-900 truncate">
                {project.title}
              </h3>
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
          ))}
        </div>
      )}
    </div>
  );
}
