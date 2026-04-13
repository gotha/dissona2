import { useState, useRef, useEffect } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { useAuthStore } from '../../stores/authStore';

export default function Header() {
  const location = useLocation();
  const { user, logout, isLoading } = useAuthStore();
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const menuRef = useRef<HTMLDivElement>(null);

  // Close menu when clicking outside
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsMenuOpen(false);
      }
    }
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const navItems = [
    { path: '/', label: 'Library' },
    { path: '/settings', label: 'Settings' },
  ];

  const handleLogout = async () => {
    setIsMenuOpen(false);
    await logout();
  };

  // Get initials for avatar fallback
  const initials = user?.name
    ?.split(' ')
    .map((n) => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2) || '?';

  return (
    <header className="bg-neutral-900 border-b border-neutral-800">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          {/* Logo */}
          <Link to="/" className="flex items-center">
            <span className="text-2xl font-bold text-white">Dissona</span>
          </Link>

          {/* Navigation */}
          <nav className="hidden md:flex space-x-1">
            {navItems.map((item) => (
              <Link
                key={item.path}
                to={item.path}
                className={`px-4 py-2 text-sm font-medium rounded-lg transition-colors ${
                  location.pathname === item.path
                    ? 'text-white bg-neutral-800'
                    : 'text-neutral-400 hover:text-white hover:bg-neutral-800/50'
                }`}
              >
                {item.label}
              </Link>
            ))}
          </nav>

          {/* User menu */}
          <div className="relative" ref={menuRef}>
            <button
              onClick={() => setIsMenuOpen(!isMenuOpen)}
              className="flex items-center gap-3 p-1.5 rounded-lg hover:bg-neutral-800 transition-colors"
              disabled={isLoading}
            >
              {/* Avatar */}
              {user?.avatarUrl ? (
                <img
                  src={user.avatarUrl}
                  alt={user.name || 'User'}
                  className="w-8 h-8 rounded-full object-cover"
                />
              ) : (
                <div className="w-8 h-8 rounded-full bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center text-white text-sm font-medium">
                  {initials}
                </div>
              )}
              {/* Dropdown arrow */}
              <svg
                className={`w-4 h-4 text-neutral-400 transition-transform ${isMenuOpen ? 'rotate-180' : ''}`}
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
              </svg>
            </button>

            {/* Dropdown menu */}
            {isMenuOpen && (
              <div className="absolute right-0 mt-2 w-56 bg-neutral-800 rounded-lg shadow-lg border border-neutral-700 py-1 z-50">
                {/* User info */}
                <div className="px-4 py-3 border-b border-neutral-700">
                  <p className="text-sm font-medium text-white truncate">{user?.name}</p>
                  <p className="text-xs text-neutral-400 truncate">{user?.email}</p>
                </div>

                {/* Menu items */}
                <div className="py-1">
                  <Link
                    to="/settings"
                    onClick={() => setIsMenuOpen(false)}
                    className="block px-4 py-2 text-sm text-neutral-300 hover:bg-neutral-700"
                  >
                    Settings
                  </Link>
                  <Link
                    to="/settings/subscription"
                    onClick={() => setIsMenuOpen(false)}
                    className="block px-4 py-2 text-sm text-neutral-300 hover:bg-neutral-700"
                  >
                    Subscription
                  </Link>
                </div>

                {/* Logout */}
                <div className="border-t border-neutral-700 py-1">
                  <button
                    onClick={handleLogout}
                    disabled={isLoading}
                    className="w-full text-left px-4 py-2 text-sm text-red-400 hover:bg-neutral-700 disabled:opacity-50"
                  >
                    {isLoading ? 'Signing out...' : 'Sign out'}
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </header>
  );
}
