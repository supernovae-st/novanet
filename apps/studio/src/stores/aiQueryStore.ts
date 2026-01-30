import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type { AiQuery, SavedQuery } from '@/types';
import { generateId } from '@/lib/utils';
import { useQueryStore } from './queryStore';
import { validateCypher } from '@/lib/cypherValidator';

const MAX_HISTORY_ITEMS = 10;

interface AiQueryStoreState {
  // AI Query History (recent queries from AI)
  queryHistory: AiQuery[];
  isProcessing: boolean;
  isExecuting: boolean;

  // Saved Queries (user's custom library)
  savedQueries: SavedQuery[];

  // Actions - AI Query
  submitAiQuery: (question: string) => Promise<void>;
  executeAiQuery: (id: string) => Promise<void>;
  updateQueryResult: (id: string, result: AiQuery['result']) => void;
  updateQueryError: (id: string, error: string) => void;
  removeFromHistory: (id: string) => void;
  clearHistory: () => void;

  // Actions - Saved Queries
  saveQuery: (query: Omit<SavedQuery, 'id' | 'createdAt' | 'updatedAt'>) => void;
  updateSavedQuery: (id: string, updates: Partial<Omit<SavedQuery, 'id' | 'createdAt'>>) => void;
  deleteSavedQuery: (id: string) => void;
  saveFromHistory: (historyId: string, name: string, icon?: string) => void;
}

export const useAiQueryStore = create<AiQueryStoreState>()(
  persist(
    immer((set, get) => ({
      // Initial state
      queryHistory: [],
      isProcessing: false,
      isExecuting: false,
      savedQueries: [],

      // Submit a natural language query to AI
      submitAiQuery: async (question: string) => {
        const queryId = generateId();

        // Add to history immediately with pending status
        // Use atomic operation to never exceed MAX_HISTORY_ITEMS
        set((state) => {
          state.isProcessing = true;
          const newItem = {
            id: queryId,
            question,
            cypher: '',
            status: 'pending' as const,
            createdAt: new Date().toISOString(),
          };
          // Prepend new item and keep only MAX items (atomic, never exceeds limit)
          state.queryHistory = [newItem, ...state.queryHistory.slice(0, MAX_HISTORY_ITEMS - 1)];
        });

        try {
          // Call AI API
          const response = await fetch('/api/chat', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ message: question }),
          });

          if (!response.ok) {
            throw new Error(`API error: ${response.status}`);
          }

          const data = await response.json();
          const cypherQuery = data.cypherQuery;

          if (!cypherQuery) {
            throw new Error('No Cypher query generated');
          }

          // Update with generated Cypher - ready for preview (no auto-execute)
          set((state) => {
            const query = state.queryHistory.find((q) => q.id === queryId);
            if (query) {
              query.cypher = cypherQuery;
              query.status = 'generated';
            }
            state.isProcessing = false;
          });
        } catch (error) {
          // Update with error
          set((state) => {
            const query = state.queryHistory.find((q) => q.id === queryId);
            if (query) {
              query.status = 'error';
              query.error = error instanceof Error ? error.message : 'Unknown error';
            }
            state.isProcessing = false;
          });
        }
      },

      // Execute a generated query manually (Preview → Run flow)
      executeAiQuery: async (id: string) => {
        // Prevent concurrent execution (race condition fix)
        if (get().isExecuting) {
          return;
        }

        const query = get().queryHistory.find((q) => q.id === id);
        if (!query || !query.cypher || query.status !== 'generated') return;

        // Client-side validation before execution
        const validation = validateCypher(query.cypher);
        if (!validation.valid) {
          set((state) => {
            const q = state.queryHistory.find((item) => item.id === id);
            if (q) {
              q.status = 'error';
              q.error = validation.error || 'Invalid Cypher syntax';
            }
          });
          return;
        }

        // Mark as executing immediately for better UX feedback
        set((state) => {
          const q = state.queryHistory.find((item) => item.id === id);
          if (q) {
            q.status = 'executing';
          }
          state.isExecuting = true;
        });

        try {
          const startTime = Date.now();
          const executeQuery = useQueryStore.getState().executeQuery;
          // Capture result directly from returned value (avoids race condition)
          const result = await executeQuery(query.cypher);
          const duration = Date.now() - startTime;

          set((state) => {
            const q = state.queryHistory.find((item) => item.id === id);
            if (q) {
              q.status = result ? 'success' : 'error';
              if (result) {
                q.result = {
                  nodeCount: result.nodes?.length || 0,
                  edgeCount: result.edges?.length || 0,
                  duration,
                };
              } else {
                q.error = 'Query returned no result';
              }
            }
            state.isExecuting = false;
          });
        } catch (error) {
          set((state) => {
            const q = state.queryHistory.find((item) => item.id === id);
            if (q) {
              q.status = 'error';
              q.error = error instanceof Error ? error.message : 'Execution failed';
            }
            state.isExecuting = false;
          });
        }
      },

      updateQueryResult: (id, result) => {
        set((state) => {
          const query = state.queryHistory.find((q) => q.id === id);
          if (query) {
            query.result = result;
            query.status = 'success';
          }
        });
      },

      updateQueryError: (id, error) => {
        set((state) => {
          const query = state.queryHistory.find((q) => q.id === id);
          if (query) {
            query.error = error;
            query.status = 'error';
          }
        });
      },

      removeFromHistory: (id) => {
        set((state) => {
          state.queryHistory = state.queryHistory.filter((q) => q.id !== id);
        });
      },

      clearHistory: () => {
        set((state) => {
          state.queryHistory = [];
        });
      },

      // Save a new query to library
      saveQuery: (query) => {
        const now = new Date().toISOString();
        set((state) => {
          state.savedQueries.push({
            ...query,
            id: generateId(),
            createdAt: now,
            updatedAt: now,
          });
        });
      },

      // Update an existing saved query
      updateSavedQuery: (id, updates) => {
        set((state) => {
          const query = state.savedQueries.find((q) => q.id === id);
          if (query) {
            Object.assign(query, updates, { updatedAt: new Date().toISOString() });
          }
        });
      },

      // Delete a saved query
      deleteSavedQuery: (id) => {
        set((state) => {
          state.savedQueries = state.savedQueries.filter((q) => q.id !== id);
        });
      },

      // Save from history to library
      saveFromHistory: (historyId, name, icon = '⭐') => {
        const historyQuery = get().queryHistory.find((q) => q.id === historyId);
        if (!historyQuery || !historyQuery.cypher) return;

        const now = new Date().toISOString();
        set((state) => {
          state.savedQueries.push({
            id: generateId(),
            name,
            description: historyQuery.question,
            icon,
            cypher: historyQuery.cypher,
            createdAt: now,
            updatedAt: now,
          });
        });
      },
    })),
    {
      name: 'novanet-ai-queries',
      partialize: (state) => ({
        queryHistory: state.queryHistory.slice(0, MAX_HISTORY_ITEMS),
        savedQueries: state.savedQueries,
      }),
    }
  )
);
