import { create } from 'zustand';
import { persist } from 'zustand/middleware';

type AudioType = 'chapter_summary' | 'key_point_summary' | 'full_narration' | 'podcast_episode';
type ListeningMode = 'blitz' | 'full';

interface PlayerState {
  // Current playback
  projectId: string | null;
  chapterId: string | null;
  audioType: AudioType;
  keyPointNumber: number | null;

  // Playback state
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  playbackRate: number;
  volume: number;

  // Mode
  listeningMode: ListeningMode;
  temporaryDeep: boolean;

  // Actions
  play: () => void;
  pause: () => void;
  seek: (time: number) => void;
  setPlaybackRate: (rate: number) => void;
  setVolume: (volume: number) => void;

  goDeep: (temporary: boolean) => void;
  returnToBlitz: () => void;

  loadTrack: (projectId: string, chapterId: string, audioType: AudioType) => void;
  setTimeUpdate: (time: number, duration: number) => void;
  reset: () => void;
}

export const usePlayerStore = create<PlayerState>()(
  persist(
    (set, get) => ({
      projectId: null,
      chapterId: null,
      audioType: 'chapter_summary',
      keyPointNumber: null,
      isPlaying: false,
      currentTime: 0,
      duration: 0,
      playbackRate: 1,
      volume: 1,
      listeningMode: 'blitz',
      temporaryDeep: false,

      play: () => set({ isPlaying: true }),
      pause: () => set({ isPlaying: false }),
      seek: (time) => set({ currentTime: time }),
      setPlaybackRate: (rate) => set({ playbackRate: rate }),
      setVolume: (volume) => set({ volume: volume }),

      goDeep: (temporary) => {
        set({
          audioType: 'full_narration',
          temporaryDeep: temporary,
          listeningMode: temporary ? 'blitz' : 'full',
        });
      },

      returnToBlitz: () => {
        set({
          audioType: 'chapter_summary',
          temporaryDeep: false,
          listeningMode: 'blitz',
        });
      },

      loadTrack: (projectId, chapterId, audioType) => {
        set({
          projectId,
          chapterId,
          audioType,
          currentTime: 0,
          duration: 0,
          isPlaying: false,
        });
      },

      setTimeUpdate: (time, duration) => {
        set({ currentTime: time, duration });
      },

      reset: () => {
        set({
          projectId: null,
          chapterId: null,
          audioType: 'chapter_summary',
          keyPointNumber: null,
          isPlaying: false,
          currentTime: 0,
          duration: 0,
        });
      },
    }),
    {
      name: 'player-storage',
      partialize: (state) => ({
        playbackRate: state.playbackRate,
        volume: state.volume,
        listeningMode: state.listeningMode,
      }),
    }
  )
);
