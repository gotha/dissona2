import { Outlet } from 'react-router-dom';

import Header from './Header';
import PlayerBar from '../player/PlayerBar';
import NotificationPrompt from '../notifications/NotificationPrompt';
import { usePlayerStore } from '../../stores/playerStore';

export default function Layout() {
  const projectId = usePlayerStore((state) => state.projectId);

  return (
    <div className="flex flex-col h-full">
      <Header />
      <main className="flex-1 overflow-auto">
        <Outlet />
      </main>
      {projectId && <PlayerBar />}
      <NotificationPrompt />
    </div>
  );
}
