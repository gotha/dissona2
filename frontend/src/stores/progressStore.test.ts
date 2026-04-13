import { describe, it, expect, beforeEach, vi, afterEach } from 'vitest';
import { useProgressStore } from './progressStore';

// Mock the api module
vi.mock('../lib/api', () => ({
  api: {
    get: vi.fn(),
    put: vi.fn(),
  },
}));

import { api } from '../lib/api';

describe('progressStore', () => {
  beforeEach(() => {
    // Reset store state
    useProgressStore.setState({
      currentProjectId: null,
      currentChapterId: null,
      currentPositionMs: 0,
      listeningMode: 'blitz',
      isSyncing: false,
      lastSyncedAt: null,
      offlineQueue: [],
    });
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  describe('setPosition', () => {
    it('should update current position', () => {
      useProgressStore.getState().setPosition('project-1', 'chapter-1', 30000);

      const state = useProgressStore.getState();
      expect(state.currentProjectId).toBe('project-1');
      expect(state.currentChapterId).toBe('chapter-1');
      expect(state.currentPositionMs).toBe(30000);
    });
  });

  describe('setMode', () => {
    it('should update listening mode', () => {
      useProgressStore.getState().setMode('full');
      expect(useProgressStore.getState().listeningMode).toBe('full');

      useProgressStore.getState().setMode('blitz');
      expect(useProgressStore.getState().listeningMode).toBe('blitz');
    });
  });

  describe('syncToServer', () => {
    it('should not sync if no current project', async () => {
      await useProgressStore.getState().syncToServer();
      expect(api.put).not.toHaveBeenCalled();
    });

    it('should sync progress to server', async () => {
      (api.put as ReturnType<typeof vi.fn>).mockResolvedValueOnce({ updated_at: '2024-01-01T00:00:00Z' });
      
      useProgressStore.setState({
        currentProjectId: 'project-1',
        currentChapterId: 'chapter-1',
        currentPositionMs: 30000,
        listeningMode: 'blitz',
      });

      await useProgressStore.getState().syncToServer();

      expect(api.put).toHaveBeenCalledWith(
        '/api/projects/project-1/progress',
        {
          chapter_id: 'chapter-1',
          position_ms: 30000,
          listening_mode: 'blitz',
        }
      );
      expect(useProgressStore.getState().lastSyncedAt).not.toBeNull();
    });

    it('should add to offline queue on failure', async () => {
      (api.put as ReturnType<typeof vi.fn>).mockRejectedValueOnce(new Error('Network error'));
      
      useProgressStore.setState({
        currentProjectId: 'project-1',
        currentChapterId: 'chapter-1',
        currentPositionMs: 30000,
        listeningMode: 'blitz',
      });

      await useProgressStore.getState().syncToServer();

      expect(useProgressStore.getState().offlineQueue).toHaveLength(1);
      expect(useProgressStore.getState().offlineQueue[0].projectId).toBe('project-1');
    });

    it('should not sync if already syncing', async () => {
      useProgressStore.setState({
        currentProjectId: 'project-1',
        currentChapterId: 'chapter-1',
        isSyncing: true,
      });

      await useProgressStore.getState().syncToServer();

      expect(api.put).not.toHaveBeenCalled();
    });
  });

  describe('syncFromServer', () => {
    it('should fetch and update progress from server', async () => {
      (api.get as ReturnType<typeof vi.fn>).mockResolvedValueOnce({
        project_id: 'project-1',
        chapter_id: 'chapter-2',
        position_ms: 45000,
        listening_mode: 'full',
        updated_at: '2024-01-01T00:00:00Z',
      });

      const result = await useProgressStore.getState().syncFromServer('project-1');

      expect(result).not.toBeNull();
      expect(result?.chapterId).toBe('chapter-2');
      expect(result?.positionMs).toBe(45000);
      
      const state = useProgressStore.getState();
      expect(state.currentChapterId).toBe('chapter-2');
      expect(state.currentPositionMs).toBe(45000);
      expect(state.listeningMode).toBe('full');
    });

    it('should return null on error', async () => {
      (api.get as ReturnType<typeof vi.fn>).mockRejectedValueOnce(new Error('Network error'));

      const result = await useProgressStore.getState().syncFromServer('project-1');

      expect(result).toBeNull();
    });
  });

  describe('flushOfflineQueue', () => {
    it('should flush offline queue', async () => {
      (api.put as ReturnType<typeof vi.fn>).mockResolvedValue({});
      
      useProgressStore.setState({
        offlineQueue: [
          { projectId: 'p1', chapterId: 'c1', positionMs: 1000, listeningMode: 'blitz', updatedAt: '' },
          { projectId: 'p2', chapterId: 'c2', positionMs: 2000, listeningMode: 'full', updatedAt: '' },
        ],
      });

      await useProgressStore.getState().flushOfflineQueue();

      expect(api.put).toHaveBeenCalledTimes(2);
      expect(useProgressStore.getState().offlineQueue).toHaveLength(0);
    });

    it('should re-add items that fail to sync', async () => {
      (api.put as ReturnType<typeof vi.fn>).mockRejectedValue(new Error('Network error'));
      
      useProgressStore.setState({
        offlineQueue: [
          { projectId: 'p1', chapterId: 'c1', positionMs: 1000, listeningMode: 'blitz', updatedAt: '' },
        ],
      });

      await useProgressStore.getState().flushOfflineQueue();

      expect(useProgressStore.getState().offlineQueue).toHaveLength(1);
    });
  });
});
