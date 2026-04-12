/**
 * Progress Store - Cross-device playback position sync
 * 
 * Features:
 * - Periodic save to server (every 10 seconds during playback)
 * - Sync on visibility change (tab focus)
 * - Offline queue for failed saves
 * - Last-write-wins conflict resolution
 */

import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { api } from '../lib/api';

export interface PlaybackProgress {
  projectId: string;
  chapterId: string;
  positionMs: number;
  listeningMode: 'blitz' | 'full';
  updatedAt: string;
}

interface ProgressState {
  // Current playback state
  currentProjectId: string | null;
  currentChapterId: string | null;
  currentPositionMs: number;
  listeningMode: 'blitz' | 'full';
  
  // Sync state
  isSyncing: boolean;
  lastSyncedAt: string | null;
  offlineQueue: PlaybackProgress[];
  
  // Actions
  setPosition: (projectId: string, chapterId: string, positionMs: number) => void;
  setMode: (mode: 'blitz' | 'full') => void;
  syncToServer: () => Promise<void>;
  syncFromServer: (projectId: string) => Promise<PlaybackProgress | null>;
  flushOfflineQueue: () => Promise<void>;
}

const SYNC_DEBOUNCE_MS = 10000; // 10 seconds
let syncTimeout: NodeJS.Timeout | null = null;

export const useProgressStore = create<ProgressState>()(
  persist(
    (set, get) => ({
      currentProjectId: null,
      currentChapterId: null,
      currentPositionMs: 0,
      listeningMode: 'blitz',
      isSyncing: false,
      lastSyncedAt: null,
      offlineQueue: [],

      setPosition: (projectId, chapterId, positionMs) => {
        set({
          currentProjectId: projectId,
          currentChapterId: chapterId,
          currentPositionMs: positionMs,
        });

        // Debounced sync to server
        if (syncTimeout) {
          clearTimeout(syncTimeout);
        }
        syncTimeout = setTimeout(() => {
          get().syncToServer();
        }, SYNC_DEBOUNCE_MS);
      },

      setMode: (mode) => {
        set({ listeningMode: mode });
      },

      syncToServer: async () => {
        const { currentProjectId, currentChapterId, currentPositionMs, listeningMode, isSyncing } = get();

        if (!currentProjectId || !currentChapterId || isSyncing) {
          return;
        }

        set({ isSyncing: true });

        const progress: PlaybackProgress = {
          projectId: currentProjectId,
          chapterId: currentChapterId,
          positionMs: currentPositionMs,
          listeningMode,
          updatedAt: new Date().toISOString(),
        };

        try {
          await api.put(`/api/projects/${currentProjectId}/progress`, {
            chapter_id: currentChapterId,
            position_ms: currentPositionMs,
            listening_mode: listeningMode,
          });

          set({
            isSyncing: false,
            lastSyncedAt: new Date().toISOString(),
          });
        } catch (error) {
          console.error('Failed to sync progress:', error);
          
          // Add to offline queue
          set((state) => ({
            isSyncing: false,
            offlineQueue: [...state.offlineQueue.filter(p => p.projectId !== currentProjectId), progress],
          }));
        }
      },

      syncFromServer: async (projectId) => {
        try {
          const progress = await api.get<{
            project_id: string;
            chapter_id: string;
            position_ms: number;
            listening_mode: 'blitz' | 'full';
            updated_at: string;
          }>(`/api/projects/${projectId}/progress`);

          if (progress && progress.chapter_id) {
            set({
              currentProjectId: progress.project_id,
              currentChapterId: progress.chapter_id,
              currentPositionMs: progress.position_ms,
              listeningMode: progress.listening_mode,
            });

            return {
              projectId: progress.project_id,
              chapterId: progress.chapter_id,
              positionMs: progress.position_ms,
              listeningMode: progress.listening_mode,
              updatedAt: progress.updated_at,
            };
          }
          return null;
        } catch (error) {
          console.error('Failed to fetch progress:', error);
          return null;
        }
      },

      flushOfflineQueue: async () => {
        const { offlineQueue } = get();
        if (offlineQueue.length === 0) return;

        const queue = [...offlineQueue];
        set({ offlineQueue: [] });

        for (const progress of queue) {
          try {
            await api.put(`/api/projects/${progress.projectId}/progress`, {
              chapter_id: progress.chapterId,
              position_ms: progress.positionMs,
              listening_mode: progress.listeningMode,
            });
          } catch (error) {
            console.error('Failed to flush progress:', error);
            // Re-add to queue
            set((state) => ({
              offlineQueue: [...state.offlineQueue, progress],
            }));
          }
        }
      },
    }),
    {
      name: 'progress-storage',
      partialize: (state) => ({
        currentProjectId: state.currentProjectId,
        currentChapterId: state.currentChapterId,
        currentPositionMs: state.currentPositionMs,
        listeningMode: state.listeningMode,
        offlineQueue: state.offlineQueue,
      }),
    }
  )
);

// Sync on visibility change
if (typeof document !== 'undefined') {
  document.addEventListener('visibilitychange', () => {
    if (document.visibilityState === 'visible') {
      const { currentProjectId, flushOfflineQueue } = useProgressStore.getState();
      flushOfflineQueue();
      if (currentProjectId) {
        useProgressStore.getState().syncFromServer(currentProjectId);
      }
    } else {
      // Save immediately when leaving
      useProgressStore.getState().syncToServer();
    }
  });
}
