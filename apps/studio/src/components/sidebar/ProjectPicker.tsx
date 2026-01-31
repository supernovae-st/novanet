'use client';

/**
 * ProjectPicker - Full-screen modal for project selection
 *
 * Design system: Linear-dark (#0d0d12 base, white/10 borders)
 * - Matches QueryPill expanded modal and LocalePicker
 * - Rich cards with project info
 * - "All Projects" first card for clearing filter
 * - Keyboard navigation and accessibility
 */

import { useState, useMemo, useCallback, useEffect, useRef, memo, useDeferredValue } from 'react';
import { createPortal } from 'react-dom';
import { Search, X, FolderOpen, Check } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';
import { cn } from '@/lib/utils';
import { glassClasses, modalClasses, iconSizes, gapTokens } from '@/design/tokens';
import { Kbd } from '@/components/ui';
import { useFilterStore } from '@/stores/filterStore';
import {
  useBodyScrollLock,
  useOutsideClick,
  useModalAutoFocus,
  useGridNavigation,
  useTimeoutFn,
  useFocusTrap,
} from '@/hooks';
import { TRANSITION_DURATION_MS } from '@/config/constants';

// Constants
const GRID_COLUMNS = 3;

export interface ProjectInfo {
  id: string;
  name: string;
}

interface ProjectPickerProps {
  isOpen: boolean;
  onClose: () => void;
  projects: ProjectInfo[];
}

// Project Card component - Linear-dark design
const ProjectCard = memo(function ProjectCard({
  project,
  isSelected,
  isFocused,
  onSelect,
  isAllCard = false,
  totalCount,
}: {
  project: ProjectInfo | null;
  isSelected: boolean;
  isFocused: boolean;
  onSelect: () => void;
  isAllCard?: boolean;
  totalCount?: number;
}) {
  return (
    <button
      onClick={onSelect}
      role="option"
      aria-selected={isSelected}
      aria-label={isAllCard ? `All projects (${totalCount})` : project?.name}
      className={cn(
        'flex flex-col items-center justify-center p-4 rounded-xl',
        gapTokens.default,
        'border transition-all duration-150 relative',
        'min-h-[100px]',
        'hover:scale-[1.02] active:scale-[0.98]',
        isSelected
          ? 'bg-emerald-500/15 border-emerald-500/40 text-white'
          : isFocused
            ? 'bg-white/[0.06] border-white/20 text-white'
            : 'bg-[#111118] border-white/[0.08] hover:bg-[#16161f] hover:border-white/15 text-white/90 hover:text-white',
        isAllCard && !isSelected && 'border-dashed border-white/15'
      )}
    >
      {/* Selection indicator */}
      {isSelected && (
        <div className={`absolute top-2.5 right-2.5 ${iconSizes.xl} rounded-full bg-emerald-500 flex items-center justify-center`}>
          <Check className={`${iconSizes.xs} text-white`} strokeWidth={3} />
        </div>
      )}

      {/* Icon */}
      <FolderOpen className={cn(
        iconSizes['2xl'],
        isAllCard ? 'opacity-60 text-white/60' : isSelected ? 'text-emerald-400' : 'text-white/60'
      )} />

      {/* Name */}
      <span className="text-sm font-medium text-center leading-tight">
        {isAllCard ? 'All Projects' : project?.name}
      </span>

      {/* Subtitle */}
      <span className={cn(
        'text-[11px] font-mono',
        isSelected ? 'text-emerald-400' : 'text-white/40'
      )}>
        {isAllCard ? `${totalCount} available` : project?.id}
      </span>
    </button>
  );
});

export const ProjectPicker = memo(function ProjectPicker({
  isOpen,
  onClose,
  projects,
}: ProjectPickerProps) {
  const { selectedProject, setSelectedProject } = useFilterStore(
    useShallow((state) => ({
      selectedProject: state.selectedProject,
      setSelectedProject: state.setSelectedProject,
    }))
  );

  const [searchInput, setSearchInput] = useState('');
  const search = useDeferredValue(searchInput);
  const [mounted, setMounted] = useState(false);

  const searchRef = useRef<HTMLInputElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const gridRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setMounted(true);
  }, []);

  // Delayed close for visual feedback
  const [delayedClose] = useTimeoutFn(onClose, TRANSITION_DURATION_MS);

  const filteredProjects = useMemo(() => {
    if (!search.trim()) return projects;
    const query = search.toLowerCase();
    return projects.filter(
      (p) =>
        p.name.toLowerCase().includes(query) ||
        p.id.toLowerCase().includes(query)
    );
  }, [projects, search]);

  const handleSelect = useCallback(
    (projectId: string | null) => {
      setSelectedProject(projectId);
      delayedClose();
    },
    [setSelectedProject, delayedClose]
  );

  // Grid navigation hook
  const { focusedIndex, handleKeyDown, resetFocus } = useGridNavigation({
    columns: GRID_COLUMNS,
    totalItems: filteredProjects.length + 1,
    gridRef,
    onSelect: (index) => {
      if (index === 0) {
        handleSelect(null);
      } else if (index > 0 && index <= filteredProjects.length) {
        handleSelect(filteredProjects[index - 1].id);
      }
    },
    onEscape: onClose,
    enabled: isOpen,
  });

  // Modal utilities
  useBodyScrollLock(isOpen);
  useOutsideClick(containerRef, onClose, isOpen);
  useFocusTrap(containerRef, isOpen);
  useModalAutoFocus(searchRef, isOpen, {
    delay: 50,
    onReset: () => {
      setSearchInput('');
      resetFocus();
    },
  });

  if (!mounted || !isOpen) return null;

  const content = (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center p-6 animate-in fade-in duration-200"
      role="presentation"
    >
      {/* Backdrop */}
      <div className={modalClasses.backdrop} aria-hidden="true" />

      {/* Modal - glass morphism design */}
      <div
        ref={containerRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="project-picker-title"
        onKeyDown={handleKeyDown}
        className={cn(
          'relative w-full max-w-3xl max-h-[70vh] overflow-hidden flex flex-col rounded-2xl',
          glassClasses.modal,
          'animate-in zoom-in-95 slide-in-from-bottom-4 duration-300'
        )}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-white/[0.06]">
          <div className={cn('flex items-center', gapTokens.spacious)}>
            <div className="w-9 h-9 rounded-lg bg-[#111118] border border-white/10 flex items-center justify-center">
              <FolderOpen className={`${iconSizes.lg} text-white/70`} />
            </div>
            <div>
              <h2 id="project-picker-title" className="text-base font-semibold text-white">
                Select Project
              </h2>
              <p className="text-xs text-white/40">Choose a project to filter by</p>
            </div>
          </div>
          <button
            onClick={onClose}
            aria-label="Close"
            className="p-2 rounded-lg hover:bg-white/10 transition-colors text-white/50 hover:text-white"
          >
            <X className={iconSizes.xl} />
          </button>
        </div>

        {/* Search */}
        <div className={cn('flex items-center px-6 py-4 border-b border-white/[0.06]', gapTokens.spacious)}>
          <Search className={`${iconSizes.lg} text-white/40 shrink-0`} />
          <input
            ref={searchRef}
            type="text"
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            placeholder="Search projects..."
            aria-label="Search projects"
            className="flex-1 bg-transparent text-white placeholder-white/40 text-sm outline-none border-none ring-0 focus:outline-none focus:ring-0"
            autoComplete="off"
            spellCheck={false}
          />
          {searchInput && (
            <button
              onClick={() => setSearchInput('')}
              aria-label="Clear search"
              className="p-1.5 hover:bg-white/10 rounded-lg transition-colors text-white/40 hover:text-white/60"
            >
              <X className={iconSizes.md} />
            </button>
          )}
        </div>

        {/* Grid */}
        <div
          className="flex-1 overflow-y-auto p-5"
          role="listbox"
          aria-label="Available projects"
        >
          <div
            ref={gridRef}
            className={cn('grid grid-cols-3', gapTokens.spacious)}
          >
            {/* All Projects card */}
            <ProjectCard
              project={null}
              isSelected={selectedProject === null}
              isFocused={focusedIndex === 0}
              onSelect={() => handleSelect(null)}
              isAllCard
              totalCount={projects.length}
            />

            {/* Project cards */}
            {filteredProjects.map((project, index) => (
              <ProjectCard
                key={project.id}
                project={project}
                isSelected={selectedProject === project.id}
                isFocused={focusedIndex === index + 1}
                onSelect={() => handleSelect(project.id)}
              />
            ))}
          </div>

          {/* No results */}
          {filteredProjects.length === 0 && (
            <div className="text-center py-12 text-white/40">
              <Search className={`${iconSizes['2xl']} mx-auto mb-3 opacity-30`} />
              <p className="text-sm font-medium">No projects found</p>
              <p className="text-xs opacity-60 mt-1">Try a different search term</p>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="px-6 py-3 border-t border-white/[0.06] flex items-center justify-between text-xs text-white/40">
          <span>{filteredProjects.length} projects</span>
          <div className={cn('flex items-center', gapTokens.large)}>
            <span className={cn('flex items-center', gapTokens.compact)}>
              <Kbd>↑↓←→</Kbd>
              <span>Navigate</span>
            </span>
            <span className={cn('flex items-center', gapTokens.compact)}>
              <Kbd>↵</Kbd>
              <span>Select</span>
            </span>
            <span className={cn('flex items-center', gapTokens.compact)}>
              <Kbd>Esc</Kbd>
              <span>Close</span>
            </span>
          </div>
        </div>
      </div>
    </div>
  );

  return createPortal(content, document.body);
});
