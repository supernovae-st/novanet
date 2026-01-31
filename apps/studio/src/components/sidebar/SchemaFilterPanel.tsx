'use client';

/**
 * SchemaFilterPanel - Schema Browser using unified Sidebar components
 *
 * Uses Sidebar.Content for consistent skeleton across all tabs:
 * - Same header structure
 * - Same body padding (p-3)
 * - Same row heights and spacing
 *
 * Now unified with Data Explorer: AI Search + Tabs (Types | Rels)
 */

import { memo, useCallback, useMemo, useState } from 'react';
import {
  Boxes,
  Landmark,
  Layers,
  Lightbulb,
  FileText,
  FileOutput,
  Settings,
  Brain,
  Search,
  Globe2,
  Package,
  Globe,
  Target,
  type LucideIcon,
} from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { SCOPE_HIERARCHY } from '@novanet/core/graph';
import type { Subcategory } from '@novanet/core/graph';
import { RelationType, type Scope } from '@/types';
import { useFilterStore } from '@/stores/filterStore';
import { useAiQueryStore } from '@/stores/aiQueryStore';
import { cn } from '@/lib/utils';
import { scopeAccents, iconSizes } from '@/design/tokens';
import { calculateCheckboxState } from '@/hooks';
import type { CheckboxState } from '@/components/ui/TriStateCheckbox';
import { SegmentedTabs } from '@/components/ui/SegmentedTabs';
import { Sidebar } from './SidebarContent';
import { AiSearchInput } from './AiSearchInput';

// Tab definitions
type SchemaTabId = 'types' | 'rels';

// Scope to Lucide icon mapping
const SCOPE_ICONS: Record<Scope, LucideIcon> = {
  Project: Package,
  Global: Globe,
  Shared: Target,
};

// Subcategory to Lucide icon mapping
const SUBCATEGORY_ICONS: Record<Subcategory, LucideIcon> = {
  foundation: Landmark,
  structure: Layers,
  semantic: Lightbulb,
  instruction: FileText,
  output: FileOutput,
  config: Settings,
  knowledge: Brain,
  seo: Search,
  geo: Globe2,
};

// Ordered scopes for consistent rendering
const SCOPE_ORDER: Scope[] = ['Project', 'Global', 'Shared'];

// Map scope names to accent keys
const SCOPE_ACCENT_MAP: Record<Scope, keyof typeof scopeAccents> = {
  Project: 'project',
  Global: 'global',
  Shared: 'shared',
};

export interface SchemaFilterPanelProps {
  className?: string;
}

export const SchemaFilterPanel = memo(function SchemaFilterPanel({
  className,
}: SchemaFilterPanelProps) {
  const [activeTab, setActiveTab] = useState<SchemaTabId>('types');

  const {
    toggleSubcategoryCollapsed,
    isSubcategoryCollapsed,
    setSubcategoryCollapsed,
  } = useFilterStore(
    useShallow((state) => ({
      toggleSubcategoryCollapsed: state.toggleSubcategoryCollapsed,
      isSubcategoryCollapsed: state.isSubcategoryCollapsed,
      setSubcategoryCollapsed: state.setSubcategoryCollapsed,
    }))
  );

  // AI Query state
  const { submitAiQuery, isProcessing } = useAiQueryStore(
    useShallow((state) => ({
      submitAiQuery: state.submitAiQuery,
      isProcessing: state.isProcessing,
    }))
  );

  // Handle AI search
  const handleAiSubmit = useCallback(async (question: string) => {
    await submitAiQuery(question);
  }, [submitAiQuery]);

  // Relationship count
  const relCount = useMemo(() => {
    return Object.keys(RelationType).length;
  }, []);

  // Tab definitions
  const tabs = useMemo(() => [
    { id: 'types' as const, label: 'Types', count: 35 },
    { id: 'rels' as const, label: 'Rels', count: relCount },
  ], [relCount]);

  // Memoize scope data
  const scopeData = useMemo(() => {
    return SCOPE_ORDER.map((scope) => {
      const scopeDef = SCOPE_HIERARCHY[scope];
      const accentKey = SCOPE_ACCENT_MAP[scope];
      const accent = scopeAccents[accentKey];
      const subcategories = Object.entries(scopeDef.subcategories) as [
        Subcategory,
        (typeof scopeDef.subcategories)[Subcategory],
      ][];
      const nodeCount = subcategories.reduce(
        (sum, [, subcat]) => sum + subcat.nodeTypes.length,
        0
      );

      return {
        scope,
        scopeDef,
        accent,
        subcategories,
        nodeCount,
      };
    });
  }, []);

  // Get visible subcategories as a Set for checkbox state calculation
  const getVisibleSubcategories = useCallback(
    (scope: Scope): Set<string> => {
      const scopeDef = SCOPE_HIERARCHY[scope];
      const subcatNames = Object.keys(scopeDef.subcategories) as Subcategory[];
      const visible = new Set<string>();
      subcatNames.forEach((name) => {
        if (!isSubcategoryCollapsed(scope, name)) {
          visible.add(name);
        }
      });
      return visible;
    },
    [isSubcategoryCollapsed]
  );

  // Calculate checkbox state for a scope
  const getScopeCheckboxState = useCallback(
    (scope: Scope): CheckboxState => {
      const scopeDef = SCOPE_HIERARCHY[scope];
      const subcatNames = Object.keys(scopeDef.subcategories) as Subcategory[];
      const visible = getVisibleSubcategories(scope);
      return calculateCheckboxState(subcatNames, visible);
    },
    [getVisibleSubcategories]
  );

  // Handle scope checkbox click
  const handleScopeCheckboxClick = useCallback(
    (scope: Scope) => {
      const scopeDef = SCOPE_HIERARCHY[scope];
      const subcatNames = Object.keys(scopeDef.subcategories) as Subcategory[];
      const currentState = getScopeCheckboxState(scope);

      const shouldCollapse = currentState !== 'none';
      subcatNames.forEach((name) => {
        setSubcategoryCollapsed(scope, name, shouldCollapse);
      });
    },
    [getScopeCheckboxState, setSubcategoryCollapsed]
  );

  // Render Types tab content
  const renderTypesContent = () => (
    <Sidebar.Tree showProgressBars={false} maxCount={35}>
      {scopeData.map(({ scope, scopeDef, accent, subcategories, nodeCount }) => {
        const ScopeIcon = SCOPE_ICONS[scope];
        return (
          <Sidebar.Section
            key={scope}
            id={scope.toLowerCase()}
            label={scopeDef.label}
            icon={<ScopeIcon className={iconSizes.sm} />}
            color={accent.color}
            checkboxState={getScopeCheckboxState(scope)}
            onCheckboxClick={() => handleScopeCheckboxClick(scope)}
            count={nodeCount}
            defaultExpanded
          >
            {subcategories.map(([subcatName, subcatMeta]) => {
              const isVisible = !isSubcategoryCollapsed(scope, subcatName);
              const SubcatIcon = SUBCATEGORY_ICONS[subcatName];

              return (
                <Sidebar.Row
                  key={subcatName}
                  id={`${scope}-${subcatName}`}
                  label={subcatMeta.label}
                  icon={<SubcatIcon className={iconSizes.xs} />}
                  color={accent.color}
                  isSelected={isVisible}
                  onToggle={() => toggleSubcategoryCollapsed(scope, subcatName)}
                  count={subcatMeta.nodeTypes.length}
                />
              );
            })}
          </Sidebar.Section>
        );
      })}
    </Sidebar.Tree>
  );

  // Render Rels tab content (placeholder for now)
  const renderRelsContent = () => (
    <div className="flex items-center justify-center h-32 text-white/40 text-sm">
      Relationships browser coming soon
    </div>
  );

  return (
    <Sidebar.Content
      testId="schema-filter-panel"
      className={className}
      header={{
        icon: <Boxes className={cn(iconSizes.md, 'text-violet-400')} />,
        iconGradient: { from: '#a78bfa', to: '#10b981' },
        title: 'Schema Browser',
        subtitle: '35 node types · 3 scopes',
      }}
      toolbar={
        <>
          {/* AI Search */}
          <div className="px-3 pt-3">
            <AiSearchInput
              onSubmit={handleAiSubmit}
              isLoading={isProcessing}
              placeholder="Ask AI about the schema..."
            />
          </div>
          {/* Segmented Tabs */}
          <div className="px-3 py-3">
            <SegmentedTabs
              tabs={tabs}
              activeTab={activeTab}
              onTabChange={(id) => setActiveTab(id as SchemaTabId)}
            />
          </div>
        </>
      }
    >
      {activeTab === 'types' ? renderTypesContent() : renderRelsContent()}
    </Sidebar.Content>
  );
});
