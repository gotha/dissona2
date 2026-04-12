import { Link, useLocation } from 'react-router-dom';
import { useAuthStore } from '../../stores/authStore';

export default function Header() {
  const location = useLocation();
  const { user, logout } = useAuthStore();

  const navItems = [
    { path: '/', label: 'Home' },
    { path: '/library', label: 'Library' },
    { path: '/settings', label: 'Settings' },
  ];

  return (
    <header className="bg-white border-b border-gray-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <Link to="/" className="flex items-center">
            <span className="text-2xl font-bold text-primary-600">Disona</span>
          </Link>

          {/* Navigation */}
          <nav className="hidden md:flex space-x-8">
            {navItems.map((item) => (
              <Link
                key={item.path}
                to={item.path}
                className={`px-3 py-2 text-sm font-medium rounded-md ${
                  location.pathname === item.path
                    ? 'text-primary-600 bg-primary-50'
                    : 'text-gray-500 hover:text-gray-900'
                }`}
              >
                {item.label}
              </Link>
            ))}
          </nav>

          {/* User menu */}
          <div className="flex items-center space-x-4">
            <span className="text-sm text-gray-500">{user?.name}</span>
            <button
              onClick={logout}
              className="text-sm text-gray-500 hover:text-gray-900"
            >
              Logout
            </button>
          </div>
        </div>
      </div>
    </header>
  );
}
