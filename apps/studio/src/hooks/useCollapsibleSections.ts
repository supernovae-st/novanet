'use client';

/**
 * useCollapsibleSections - Manage expandable/collapsible section state
 *
 * Extracted from NodeDetailsPanel and EdgeDetailsPanel.
 * Tracks which sections are expanded with toggle functionality.
 */

import { useState, useCallback } from 'react';

export interface UseCollapsibleSectionsReturn {
  /** Set of currently expanded section IDs */
  expandedSections: Set<string>;
  /** Toggle a section's expanded state */
  toggleSection: (sectionId: string) => void;
  /** Check if a section is expanded */
  isExpanded: (sectionId: string) => boolean;
  /** Expand all sections */
  expandAll: (sectionIds: string[]) => void;
  /** Collapse all sections */
  collapseAll: () => void;
}

/**
 * Manage collapsible section state
 *
 * @param defaultExpanded - Array of section IDs that should be expanded by default
 *
 * @example
 * const { isExpanded, toggleSection } = useCollapsibleSections(['main', 'data']);
 *
 * <button onClick={() => toggleSection('main')}>
 *   {isExpanded('main') ? 'Collapse' : 'Expand'}
 * </button>
 */
export function useCollapsibleSections(
  defaultExpanded: string[] = []
): UseCollapsibleSectionsReturn {
  const [expandedSections, setExpandedSections] = useState<Set<string>>(
    () => new Set(defaultExpanded)
  );

  const toggleSection = useCallback((sectionId: string) => {
    setExpandedSections((prev) => {
      const next = new Set(prev);
      if (next.has(sectionId)) {
        next.delete(sectionId);
      } else {
        next.add(sectionId);
      }
      return next;
    });
  }, []);

  const isExpanded = useCallback(
    (sectionId: string) => expandedSections.has(sectionId),
    [expandedSections]
  );

  const expandAll = useCallback((sectionIds: string[]) => {
    setExpandedSections(new Set(sectionIds));
  }, []);

  const collapseAll = useCallback(() => {
    setExpandedSections(new Set());
  }, []);

  return {
    expandedSections,
    toggleSection,
    isExpanded,
    expandAll,
    collapseAll,
  };
}
