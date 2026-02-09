import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import type { ChatMessage, ChatState } from '@/types';
import { generateId } from '@/lib/utils';

interface ChatStoreState extends ChatState {
  addMessage: (role: 'user' | 'assistant', content: string, metadata?: ChatMessage['metadata']) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearMessages: () => void;
}

export const useChatStore = create<ChatStoreState>()(
  immer((set) => ({
    messages: [],
    isLoading: false,
    error: null,

    addMessage: (role, content, metadata) => {
      const message: ChatMessage = {
        id: generateId(),
        role,
        content,
        timestamp: new Date(),
        metadata,
      };

      set((state) => {
        state.messages.push(message);
        state.error = null;
      });
    },

    setLoading: (loading) => {
      set((state) => {
        state.isLoading = loading;
      });
    },

    setError: (error) => {
      set((state) => {
        state.error = error;
        state.isLoading = false;
      });
    },

    clearMessages: () => {
      set((state) => {
        state.messages = [];
        state.error = null;
      });
    },
  }))
);
