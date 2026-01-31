'use client';

/**
 * ContextPicker - Unified Project + Locale selector in single pill
 *
 * Features:
 * - Single pill with Project | Locale format
 * - Cascade: locale disabled when no project selected
 * - Press animations with Motion
 * - Glass-floating pickers for selection
 * - No internal borders (parent Pill provides border)
 */

import { useState, useCallback, useMemo, memo } from 'react';
import { motion } from 'motion/react';
import { FolderOpen, Globe, ChevronDown } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useFilterStore } from '@/stores/filterStore';
import { useGraphStore } from '@/stores/graphStore';
import { getLocaleInfo, ALL_LOCALES } from '@/config/locales';
import { iconSizes, gapTokens } from '@/design/tokens';
import { Divider } from '@/components/ui/Divider';
import { LocalePicker } from './LocalePicker';
import { ProjectPicker } from './ProjectPicker';

interface ContextPickerProps {
  className?: string;
}

// Selector button - no border, used inside Pill
const SelectorButton = memo(function SelectorButton({
  icon: Icon,
  label,
  sublabel,
  disabled = false,
  onClick,
}: {
  icon: React.ElementType;
  label: string;
  sublabel?: string;
  disabled?: boolean;
  onClick: () => void;
}) {
  return (
    <motion.button
      whileTap={{ scale: 0.97 }}
      onClick={onClick}
      disabled={disabled}
      className={cn(
        cn('flex items-center px-3 py-2 rounded-xl', gapTokens.default),
        'transition-all duration-150',
        disabled
          ? 'opacity-40 cursor-not-allowed'
          : 'hover:bg-white/8 active:bg-white/10'
      )}
    >
      <Icon className={cn(
        iconSizes.md,
        'shrink-0',
        disabled ? 'text-white/40' : 'text-white/50'
      )} />
      <div className="flex flex-col items-start">
        <span className={cn(
          'text-sm font-medium',
          disabled ? 'text-white/40' : 'text-white/90'
        )}>
          {label}
        </span>
        {sublabel && (
          <span className={cn(
            'text-[10px] leading-tight',
            disabled ? 'text-white/40' : 'text-white/50'
          )}>
            {sublabel}
          </span>
        )}
      </div>
      <ChevronDown className={cn(
        iconSizes.sm,
        'ml-1',
        disabled ? 'text-white/40' : 'text-white/50'
      )} />
    </motion.button>
  );
});

export const ContextPicker = memo(function ContextPicker({ className }: ContextPickerProps) {
  const [isProjectPickerOpen, setProjectPickerOpen] = useState(false);
  const [isLocalePickerOpen, setLocalePickerOpen] = useState(false);

  // Store state
  const selectedProject = useFilterStore((s) => s.selectedProject);
  const selectedLocale = useFilterStore((s) => s.selectedLocale);

  // Get available projects from graph nodes
  const nodes = useGraphStore((s) => s.nodes);
  const availableProjects = useMemo(() => {
    return nodes
      .filter((n) => n.type === 'Project')
      .map((n) => ({
        id: n.id,
        name: n.displayName || n.key || n.id,
      }));
  }, [nodes]);

  // Project display
  const projectDisplay = useMemo(() => {
    if (!selectedProject) {
      return {
        label: 'All Projects',
        sublabel: `${availableProjects.length || '—'} available`,
      };
    }
    const project = availableProjects.find((p) => p.id === selectedProject);
    return {
      label: project?.name || selectedProject,
      sublabel: undefined,
    };
  }, [selectedProject, availableProjects]);

  // Locale display
  const localeDisplay = useMemo(() => {
    if (!selectedLocale) {
      return {
        label: '🌐 All',
        sublabel: `${ALL_LOCALES.length} locales`,
      };
    }
    const info = getLocaleInfo(selectedLocale);
    return {
      label: `${info.flag} ${selectedLocale}`,
      sublabel: info.name,
    };
  }, [selectedLocale]);

  // Cascade: locale disabled when no project
  const isLocaleDisabled = !selectedProject;

  // Handlers
  const handleOpenProject = useCallback(() => {
    setProjectPickerOpen(true);
  }, []);

  const handleCloseProject = useCallback(() => {
    setProjectPickerOpen(false);
  }, []);

  const handleOpenLocale = useCallback(() => {
    if (!isLocaleDisabled) {
      setLocalePickerOpen(true);
    }
  }, [isLocaleDisabled]);

  const handleCloseLocale = useCallback(() => {
    setLocalePickerOpen(false);
  }, []);

  return (
    <>
      <div className={cn('flex items-center', className)}>
        {/* Project selector */}
        <SelectorButton
          icon={FolderOpen}
          label={projectDisplay.label}
          sublabel={projectDisplay.sublabel}
          onClick={handleOpenProject}
        />

        <Divider height="lg" />

        {/* Locale selector */}
        <SelectorButton
          icon={Globe}
          label={localeDisplay.label}
          sublabel={localeDisplay.sublabel}
          disabled={isLocaleDisabled}
          onClick={handleOpenLocale}
        />
      </div>

      {/* Pickers */}
      <ProjectPicker
        isOpen={isProjectPickerOpen}
        onClose={handleCloseProject}
        projects={availableProjects}
      />
      <LocalePicker
        isOpen={isLocalePickerOpen}
        onClose={handleCloseLocale}
      />
    </>
  );
});
