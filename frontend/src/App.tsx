import { Routes, Route, Navigate } from 'react-router-dom';

import { useAuthStore } from './stores/authStore';
import Layout from './components/layout/Layout';
import Home from './pages/Home';
import Library from './pages/Library';
import Project from './pages/Project';
import Settings from './pages/Settings';
import Login from './pages/Login';
import AuthCallback from './pages/AuthCallback';

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
}

export default function App() {
  return (
    <Routes>
      {/* Public routes */}
      <Route path="/login" element={<Login />} />
      <Route path="/auth/callback" element={<AuthCallback />} />

      {/* Protected routes */}
      <Route
        path="/"
        element={
          <ProtectedRoute>
            <Layout />
          </ProtectedRoute>
        }
      >
        <Route index element={<Home />} />
        <Route path="library" element={<Library />} />
        <Route path="project/:id" element={<Project />} />
        <Route path="settings" element={<Settings />} />
      </Route>
    </Routes>
  );
}
