import { Routes, Route, Navigate } from 'react-router-dom';

import { useAuthStore } from './stores/authStore';
import Layout from './components/layout/Layout';
import Home from './pages/Home';
import Library from './pages/Library';
import Project from './pages/Project';
import Settings from './pages/Settings';
import Subscription from './pages/Subscription';
import Login from './pages/Login';
import AuthCallback from './pages/AuthCallback';
import Upload from './pages/Upload';

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return <>{children}</>;
}

function OnboardingRedirect({ children }: { children: React.ReactNode }) {
  const user = useAuthStore((state) => state.user);

  // Redirect new users who haven't uploaded yet to the guided upload
  if (user && user.hasCompletedFirstUpload === false) {
    return <Navigate to="/welcome" replace />;
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
        <Route index element={<Navigate to="/library" replace />} />
        <Route path="library" element={<Library />} />
        <Route path="project/:id" element={<Project />} />
        <Route path="upload" element={<Upload />} />
        <Route path="welcome" element={<Upload />} />
        <Route path="settings" element={<Settings />} />
        <Route path="settings/subscription" element={<Subscription />} />
      </Route>
    </Routes>
  );
}
