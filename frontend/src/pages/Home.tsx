import { Link } from 'react-router-dom';

export default function Home() {
  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
      <div className="text-center">
        <h1 className="text-4xl font-bold text-gray-900">
          Welcome to Dissona
        </h1>
        <p className="mt-4 text-xl text-gray-500">
          Turn your documents into intelligent audio
        </p>
        <div className="mt-8 flex justify-center space-x-4">
          <Link to="/library" className="btn-primary">
            Go to Library
          </Link>
          <button className="btn-secondary">
            Upload Document
          </button>
        </div>
      </div>

      {/* Quick start cards */}
      <div className="mt-16 grid grid-cols-1 md:grid-cols-3 gap-8">
        <div className="card p-6">
          <h3 className="text-lg font-medium text-gray-900">Upload a PDF</h3>
          <p className="mt-2 text-gray-500">
            Start by uploading a book, paper, or any document
          </p>
        </div>

        <div className="card p-6">
          <h3 className="text-lg font-medium text-gray-900">Generate Audio</h3>
          <p className="mt-2 text-gray-500">
            We'll analyze and convert it to audio summaries
          </p>
        </div>

        <div className="card p-6">
          <h3 className="text-lg font-medium text-gray-900">Listen Anywhere</h3>
          <p className="mt-2 text-gray-500">
            Listen on any device, online or offline
          </p>
        </div>
      </div>
    </div>
  );
}
