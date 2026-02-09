import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { immer } from 'zustand/middleware/immer';
import type { AiQuery } from '@/types';
import { generateId } from '@/lib/utils';
import { useQueryStore } from './queryStore';
import { validateCypher } from '@/lib/cypherValidator';

const MAX_HISTORY_ITEMS = 10;

interface AiQueryStoreState {
  // AI Query History (recent queries from AI)
  queryHistory: AiQuery[];
  isExecuting: boolean;

  // Actions - AI Query
  submitAiQuery: (question: string) => Promise<void>;
  executeAiQuery: (id: string) => Promise<void>;
}

export const useAiQueryStore = create<AiQueryStoreState>()(
  persist(
    immer((set, get) => ({
      // Initial state
      queryHistory: [],
      isExecuting: false,

      // Submit a natural language query to AI
      submitAiQuery: async (question: string) => {
        const queryId = generateId();

        // Add to history immediately with pending status
        set((state) => {
          const newItem = {
            id: queryId,
            question,
            cypher: '',
            status: 'pending' as const,
            createdAt: new Date().toISOString(),
          };
          // Prepend new item and keep only MAX items
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
          });
        } catch (error) {
          // Update with error
          set((state) => {
            const query = state.queryHistory.find((q) => q.id === queryId);
            if (query) {
              query.status = 'error';
              query.error = error instanceof Error ? error.message : 'Unknown error';
            }
          });
        }
      },

      // Execute a generated query manually (Preview → Run flow)
      executeAiQuery: async (id: string) => {
        // Prevent concurrent execution
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
    })),
    {
      name: 'novanet-ai-queries',
      partialize: (state) => ({
        queryHistory: state.queryHistory.slice(0, MAX_HISTORY_ITEMS),
      }),
    }
  )
);
