import { create } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import type { ChatMessage, ChatState } from '@/types';
import { generateId } from '@/lib/utils';

interface ChatStoreState extends ChatState {
  // Actions
  addMessage: (role: 'user' | 'assistant', content: string, metadata?: ChatMessage['metadata']) => void;
  updateLastMessage: (content: string, metadata?: ChatMessage['metadata']) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  clearMessages: () => void;

  // Streaming support
  streamingMessageId: string | null;
  startStreaming: () => string;
  appendToStream: (content: string) => void;
  endStreaming: (metadata?: ChatMessage['metadata']) => void;
}

export const useChatStore = create<ChatStoreState>()(
  immer((set) => ({
    // Initial state
    messages: [],
    isLoading: false,
    error: null,
    streamingMessageId: null,

    // Actions
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

    updateLastMessage: (content, metadata) => {
      set((state) => {
        const lastMessage = state.messages[state.messages.length - 1];
        if (lastMessage) {
          lastMessage.content = content;
          if (metadata) {
            lastMessage.metadata = { ...lastMessage.metadata, ...metadata };
          }
        }
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

    // Streaming support
    startStreaming: () => {
      const id = generateId();
      const message: ChatMessage = {
        id,
        role: 'assistant',
        content: '',
        timestamp: new Date(),
      };

      set((state) => {
        state.messages.push(message);
        state.streamingMessageId = id;
        state.isLoading = true;
      });

      return id;
    },

    appendToStream: (content) => {
      set((state) => {
        const streamId = state.streamingMessageId;
        if (streamId) {
          const message = state.messages.find((m) => m.id === streamId);
          if (message) {
            message.content += content;
          }
        }
      });
    },

    endStreaming: (metadata) => {
      set((state) => {
        const streamId = state.streamingMessageId;
        if (streamId) {
          const message = state.messages.find((m) => m.id === streamId);
          if (message && metadata) {
            message.metadata = metadata;
          }
        }
        state.streamingMessageId = null;
        state.isLoading = false;
      });
    },
  }))
);

// =============================================================================
// SELECTORS - Use these for optimized subscriptions
// =============================================================================

export const selectMessages = (state: ChatStoreState) => state.messages;
export const selectIsLoading = (state: ChatStoreState) => state.isLoading;
export const selectChatError = (state: ChatStoreState) => state.error;
export const selectStreamingMessageId = (state: ChatStoreState) => state.streamingMessageId;
