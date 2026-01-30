/**
 * Animation Budget Controller
 *
 * Priority-based animation limiting.
 * Ensures max N edges animate simultaneously for 60fps.
 */

import type { EdgePriority, AnimationBudgetConfig } from '../types';
import { DEFAULT_ANIMATION_BUDGET, EDGE_PRIORITIES } from '../constants';

// =============================================================================
// Types
// =============================================================================

interface BudgetEntry {
  edgeId: string;
  priority: number;
  timestamp: number;
}

// =============================================================================
// Animation Budget Manager
// =============================================================================

/**
 * Manages animation budget across all edges
 */
export class AnimationBudgetManager {
  private config: AnimationBudgetConfig;
  private activeEdges: Map<string, BudgetEntry> = new Map();
  private waitingEdges: Map<string, BudgetEntry> = new Map();

  constructor(config: AnimationBudgetConfig = DEFAULT_ANIMATION_BUDGET) {
    this.config = config;
  }

  /**
   * Update configuration
   */
  setConfig(config: Partial<AnimationBudgetConfig>): void {
    this.config = { ...this.config, ...config };
    this.rebalance();
  }

  /**
   * Get priority value for an edge state
   */
  getPriority(state: EdgePriority): number {
    return this.config.priorities[state];
  }

  /**
   * Request animation slot for an edge
   * Returns true if edge can animate, false if it should wait
   */
  requestSlot(edgeId: string, state: EdgePriority): boolean {
    const priority = this.getPriority(state);
    const entry: BudgetEntry = {
      edgeId,
      priority,
      timestamp: Date.now(),
    };

    // High priority edges (selected/highlighted) always get a slot
    if (priority >= EDGE_PRIORITIES.highlighted) {
      this.activeEdges.set(edgeId, entry);
      this.waitingEdges.delete(edgeId);
      this.evictIfNeeded();
      return true;
    }

    // Check if we have budget
    if (this.activeEdges.size < this.config.maxConcurrent) {
      this.activeEdges.set(edgeId, entry);
      this.waitingEdges.delete(edgeId);
      return true;
    }

    // Check if we can evict a lower priority edge
    const lowestActive = this.getLowestPriorityActive();
    if (lowestActive && lowestActive.priority < priority) {
      // Evict lowest and take its slot
      this.activeEdges.delete(lowestActive.edgeId);
      this.waitingEdges.set(lowestActive.edgeId, lowestActive);
      this.activeEdges.set(edgeId, entry);
      return true;
    }

    // Add to waiting queue
    this.waitingEdges.set(edgeId, entry);
    return false;
  }

  /**
   * Release animation slot when edge no longer needs it
   */
  releaseSlot(edgeId: string): void {
    this.activeEdges.delete(edgeId);
    this.waitingEdges.delete(edgeId);
    this.promoteWaiting();
  }

  /**
   * Update priority for an edge (e.g., when hover state changes)
   */
  updatePriority(edgeId: string, newState: EdgePriority): boolean {
    const newPriority = this.getPriority(newState);
    const isActive = this.activeEdges.has(edgeId);
    const isWaiting = this.waitingEdges.has(edgeId);

    if (!isActive && !isWaiting) {
      // Not tracked, request a new slot
      return this.requestSlot(edgeId, newState);
    }

    const entry: BudgetEntry = {
      edgeId,
      priority: newPriority,
      timestamp: Date.now(),
    };

    if (isActive) {
      // Update priority in active set
      this.activeEdges.set(edgeId, entry);
      return true;
    }

    // In waiting queue - try to promote
    this.waitingEdges.set(edgeId, entry);
    return this.tryPromote(edgeId, newPriority);
  }

  /**
   * Check if an edge is currently allowed to animate
   */
  canAnimate(edgeId: string): boolean {
    return this.activeEdges.has(edgeId);
  }

  /**
   * Get current active count
   */
  getActiveCount(): number {
    return this.activeEdges.size;
  }

  /**
   * Get waiting count
   */
  getWaitingCount(): number {
    return this.waitingEdges.size;
  }

  /**
   * Get budget utilization (0-1)
   */
  getUtilization(): number {
    return this.activeEdges.size / this.config.maxConcurrent;
  }

  /**
   * Clear all tracked edges
   */
  clear(): void {
    this.activeEdges.clear();
    this.waitingEdges.clear();
  }

  /**
   * Get statistics about the budget
   */
  getStats(): { current: number; max: number; waiting: number; utilization: number } {
    return {
      current: this.activeEdges.size,
      max: this.config.maxConcurrent,
      waiting: this.waitingEdges.size,
      utilization: this.getUtilization(),
    };
  }

  // ─── Private Methods ───

  private getLowestPriorityActive(): BudgetEntry | null {
    let lowest: BudgetEntry | null = null;

    for (const entry of this.activeEdges.values()) {
      // Don't evict high priority edges
      if (entry.priority >= EDGE_PRIORITIES.highlighted) continue;

      if (!lowest || entry.priority < lowest.priority) {
        lowest = entry;
      }
    }

    return lowest;
  }

  private evictIfNeeded(): void {
    while (this.activeEdges.size > this.config.maxConcurrent) {
      const lowest = this.getLowestPriorityActive();
      if (!lowest) break; // All are high priority

      this.activeEdges.delete(lowest.edgeId);
      this.waitingEdges.set(lowest.edgeId, lowest);
    }
  }

  private promoteWaiting(): void {
    if (this.activeEdges.size >= this.config.maxConcurrent) return;
    if (this.waitingEdges.size === 0) return;

    // Find highest priority waiting edge
    let highest: BudgetEntry | null = null;
    for (const entry of this.waitingEdges.values()) {
      if (!highest || entry.priority > highest.priority) {
        highest = entry;
      }
    }

    if (highest) {
      this.waitingEdges.delete(highest.edgeId);
      this.activeEdges.set(highest.edgeId, highest);
    }
  }

  private tryPromote(edgeId: string, priority: number): boolean {
    if (this.activeEdges.size < this.config.maxConcurrent) {
      const entry = this.waitingEdges.get(edgeId);
      if (entry) {
        this.waitingEdges.delete(edgeId);
        this.activeEdges.set(edgeId, entry);
        return true;
      }
    }

    const lowest = this.getLowestPriorityActive();
    if (lowest && lowest.priority < priority) {
      const entry = this.waitingEdges.get(edgeId);
      if (entry) {
        this.activeEdges.delete(lowest.edgeId);
        this.waitingEdges.set(lowest.edgeId, lowest);
        this.waitingEdges.delete(edgeId);
        this.activeEdges.set(edgeId, entry);
        return true;
      }
    }

    return false;
  }

  private rebalance(): void {
    // Re-evaluate all edges after config change
    this.evictIfNeeded();
    this.promoteWaiting();
  }
}

/**
 * Singleton budget manager instance
 */
export const animationBudget = new AnimationBudgetManager();

// =============================================================================
// Utility Functions
// =============================================================================

/**
 * Determine edge priority from state flags
 */
export function getEdgePriority(
  isSelected: boolean,
  isHighlighted: boolean,
  isConnectedToSelected: boolean,
): EdgePriority {
  if (isSelected) return 'selected';
  if (isHighlighted) return 'highlighted';
  if (isConnectedToSelected) return 'connected';
  return 'default';
}
