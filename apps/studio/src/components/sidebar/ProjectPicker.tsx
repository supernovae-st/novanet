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
import { pickerClasses, iconSizes, gapTokens, getCardStagger } from '@/design/tokens';
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

// Project Card component - uses pickerClasses tokens + stagger animation
const ProjectCard = memo(function ProjectCard({
  project,
  isSelected,
  isFocused,
  onSelect,
  isAllCard = false,
  totalCount,
  index,
}: {
  project: ProjectInfo | null;
  isSelected: boolean;
  isFocused: boolean;
  onSelect: () => void;
  isAllCard?: boolean;
  totalCount?: number;
  index: number;
}) {
  return (
    <button
      onClick={onSelect}
      role="option"
      aria-selected={isSelected}
      aria-label={isAllCard ? `All projects (${totalCount})` : project?.name}
      className={cn(
        pickerClasses.cardBase,
        'min-h-[100px]',
        isSelected
          ? 'bg-emerald-500/15 border-emerald-500/40 text-white'
          : isFocused
            ? pickerClasses.cardFocused
            : pickerClasses.cardIdle,
        isAllCard && !isSelected && pickerClasses.cardAll,
        pickerClasses.cardAnimation,
        getCardStagger(index),
      )}
    >
      {/* Selection indicator */}
      {isSelected && (
        <div className={cn('absolute top-3 right-3', iconSizes.xl, 'rounded-full bg-emerald-500 flex items-center justify-center')}>
          <Check className={cn(iconSizes.xs, 'text-white')} strokeWidth={3} />
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
    <div className={pickerClasses.container} role="presentation">
      {/* Backdrop - Raycast blur ramp */}
      <div className={pickerClasses.backdrop} aria-hidden="true" />

      {/* Modal shell - compact size for fewer items */}
      <div
        ref={containerRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="project-picker-title"
        onKeyDown={handleKeyDown}
        className={cn(pickerClasses.shell, pickerClasses.sizeCompact, pickerClasses.maxHeight)}
      >
        {/* Header */}
        <div className={pickerClasses.header}>
          <div className={cn('flex items-center', gapTokens.spacious)}>
            <div className={cn(pickerClasses.headerIconBox, 'bg-white/[0.04]')}>
              <FolderOpen className={cn(iconSizes.lg, 'text-white/70')} />
            </div>
            <div>
              <h2 id="project-picker-title" className={pickerClasses.headerTitle}>
                Select Project
              </h2>
              <p className={pickerClasses.headerSubtitle}>Choose a project to filter by</p>
            </div>
          </div>
          <button onClick={onClose} aria-label="Close" className={pickerClasses.closeButton}>
            <X className={iconSizes.xl} />
          </button>
        </div>

        {/* Search */}
        <div className={pickerClasses.searchBar}>
          <Search className={cn(iconSizes.lg, 'text-white/40 shrink-0')} />
          <input
            ref={searchRef}
            type="text"
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            placeholder="Search projects\u2026"
            aria-label="Search projects"
            aria-describedby="project-picker-hint"
            className={pickerClasses.searchInput}
            autoComplete="off"
            spellCheck={false}
          />
          <span id="project-picker-hint" className="sr-only">Type to filter available projects</span>
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
          className={pickerClasses.grid}
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
              index={0}
            />

            {/* Project cards */}
            {filteredProjects.map((project, index) => (
              <ProjectCard
                key={project.id}
                project={project}
                isSelected={selectedProject === project.id}
                isFocused={focusedIndex === index + 1}
                onSelect={() => handleSelect(project.id)}
                index={index + 1}
              />
            ))}
          </div>

          {/* No results */}
          {filteredProjects.length === 0 && (
            <div className={pickerClasses.emptyState}>
              <Search className={cn(iconSizes['2xl'], 'mx-auto mb-3 opacity-30')} />
              <p className="text-sm font-medium">No projects found</p>
              <p className="text-xs opacity-60 mt-1">Try a different search term</p>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className={pickerClasses.footer}>
          <div className={pickerClasses.footerContent}>
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
    </div>
  );

  return createPortal(content, document.body);
});
