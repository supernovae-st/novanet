'use client';

/**
 * LocalePicker - Full-screen modal for locale selection
 *
 * Design system: Linear-dark (#0d0d12 base, white/10 borders)
 * - Matches QueryPill expanded modal
 * - Rich cards with flag, name, and locale code
 * - "All Languages" first card for clearing filter
 * - Keyboard navigation and accessibility
 */

import { useState, useMemo, useCallback, useEffect, useRef, memo, useDeferredValue } from 'react';
import { createPortal } from 'react-dom';
import { Search, X, Globe, Check } from 'lucide-react';
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
import {
  ALL_LOCALES,
  PRIMARY_LOCALES,
  getLocaleInfo,
} from '@/config/locales';

// Constants
const GRID_COLUMNS = 4;

interface LocalePickerProps {
  isOpen: boolean;
  onClose: () => void;
}

// Rich Locale Card - NovaNet design system
interface LocaleCardProps {
  code: string;
  info: { name: string; flag: string };
  isSelected: boolean;
  isFocused: boolean;
  onSelect: () => void;
  isAllLocales?: boolean; // For the special "All" card
  totalCount?: number;
}

const LocaleCard = memo(function LocaleCard({
  code,
  info,
  isSelected,
  isFocused,
  onSelect,
  isAllLocales = false,
  totalCount,
}: LocaleCardProps) {
  return (
    <button
      onClick={onSelect}
      role="option"
      aria-selected={isSelected}
      aria-label={isAllLocales ? `All languages (${totalCount})` : `${info.name} (${code})`}
      className={cn(
        'flex flex-col items-center justify-center p-4 rounded-xl',
        gapTokens.default,
        'border transition-all duration-150 relative',
        'min-h-[110px]',
        'hover:scale-[1.02] active:scale-[0.98]',
        isSelected
          ? 'bg-novanet-500/20 border-novanet-500/30 text-white'
          : isFocused
            ? 'bg-white/[0.06] border-white/20 text-white'
            : 'bg-white/[0.02] border-transparent hover:bg-white/[0.06] hover:border-white/10 text-white/80',
        // All locales card special styling
        isAllLocales && !isSelected && 'border-dashed border-white/15'
      )}
    >
      {/* Selection indicator */}
      {isSelected && (
        <div className="absolute top-2 right-2 w-5 h-5 rounded-full bg-novanet-500 flex items-center justify-center">
          <Check className={cn(iconSizes.xs, 'text-white')} strokeWidth={3} />
        </div>
      )}

      {/* Flag */}
      <span className="text-3xl">{info.flag}</span>

      {/* Name */}
      <span className="text-sm font-medium text-center">{info.name}</span>

      {/* Locale code */}
      {!isAllLocales && (
        <span className="text-xs text-white/40">{code}</span>
      )}

      {/* Show count for All locales card */}
      {isAllLocales && (
        <span className="text-xs text-white/40">
          {totalCount} locales
        </span>
      )}
    </button>
  );
});

export const LocalePicker = memo(function LocalePicker({
  isOpen,
  onClose,
}: LocalePickerProps) {
  const { selectedLocale, setSelectedLocale } = useFilterStore(
    useShallow((state) => ({
      selectedLocale: state.selectedLocale,
      setSelectedLocale: state.setSelectedLocale,
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

  // Memoized locale info cache
  const localeInfoCache = useMemo(() => {
    const cache = new Map<string, { name: string; flag: string }>();
    ALL_LOCALES.forEach(code => {
      cache.set(code, getLocaleInfo(code));
    });
    return cache;
  }, []);

  // Filter and sort locales - primary markets first
  const filteredLocales = useMemo(() => {
    let locales = ALL_LOCALES;

    if (search.trim()) {
      const query = search.toLowerCase();
      locales = locales.filter((code) => {
        const info = localeInfoCache.get(code)!;
        return (
          code.toLowerCase().includes(query) ||
          info.name.toLowerCase().includes(query)
        );
      });
    }

    const primarySet = new Set(PRIMARY_LOCALES);
    return [...locales].sort((a, b) => {
      const aIsPrimary = primarySet.has(a);
      const bIsPrimary = primarySet.has(b);
      if (aIsPrimary && !bIsPrimary) return -1;
      if (!aIsPrimary && bIsPrimary) return 1;
      return localeInfoCache.get(a)!.name.localeCompare(localeInfoCache.get(b)!.name);
    });
  }, [search, localeInfoCache]);

  const handleSelect = useCallback(
    (code: string | null) => {
      setSelectedLocale(code);
      delayedClose();
    },
    [setSelectedLocale, delayedClose]
  );

  // Grid navigation hook
  const { focusedIndex, handleKeyDown, resetFocus } = useGridNavigation({
    columns: GRID_COLUMNS,
    totalItems: filteredLocales.length + 1,
    gridRef,
    onSelect: (index) => {
      if (index === 0) {
        handleSelect(null);
      } else if (index > 0 && index <= filteredLocales.length) {
        handleSelect(filteredLocales[index - 1]);
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
      <div className={cn(modalClasses.backdrop, 'animate-in fade-in duration-200')} aria-hidden="true" />

      {/* Modal - glass-floating style (matches CommandPalette) */}
      <div
        ref={containerRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="locale-picker-title"
        onKeyDown={handleKeyDown}
        className={cn(
          'relative w-full max-w-4xl max-h-[85vh] overflow-hidden flex flex-col',
          glassClasses.floating,
          'animate-scale-in'
        )}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-white/[0.06]">
          <div className={cn('flex items-center', gapTokens.spacious)}>
            <div className="w-9 h-9 rounded-lg bg-[#111118] border border-white/10 flex items-center justify-center">
              <Globe className={cn(iconSizes.lg, 'text-white/70')} />
            </div>
            <div>
              <h2 id="locale-picker-title" className="text-base font-semibold text-white">
                Select Locale
              </h2>
              <p className="text-xs text-white/40">Choose a language and region</p>
            </div>
          </div>
          <button
            onClick={onClose}
            aria-label="Close"
            className="p-2 rounded-lg hover:bg-white/10 transition-colors text-white/60 hover:text-white"
          >
            <X className={iconSizes.xl} />
          </button>
        </div>

        {/* Search Header - CommandPalette style */}
        <div className={cn('flex items-center p-4 border-b border-white/[0.06]', gapTokens.spacious)}>
          <Search className={cn(iconSizes.xl, 'text-white/40 shrink-0')} />
          <input
            ref={searchRef}
            type="text"
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            placeholder="Search languages..."
            aria-label="Search languages"
            className="flex-1 bg-transparent text-white placeholder-white/40 text-base outline-none"
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
          aria-label="Available languages"
        >
          <div
            ref={gridRef}
            className={cn('grid grid-cols-4', gapTokens.spacious)}
          >
            {/* All Languages card */}
            <LocaleCard
              code=""
              info={{ name: 'All Languages', flag: '🌐' }}
              isSelected={selectedLocale === null}
              isFocused={focusedIndex === 0}
              onSelect={() => handleSelect(null)}
              isAllLocales
              totalCount={ALL_LOCALES.length}
            />

            {/* Locale cards */}
            {filteredLocales.map((code, index) => {
              const info = localeInfoCache.get(code)!;
              return (
                <LocaleCard
                  key={code}
                  code={code}
                  info={info}
                  isSelected={selectedLocale === code}
                  isFocused={focusedIndex === index + 1}
                  onSelect={() => handleSelect(code)}
                />
              );
            })}
          </div>

          {/* No results */}
          {filteredLocales.length === 0 && (
            <div className="text-center py-12 text-white/40">
              <Search className="w-10 h-10 mx-auto mb-3 opacity-30" />
              <p className="text-sm font-medium">No languages found</p>
              <p className="text-xs opacity-60 mt-1">Try a different search term</p>
            </div>
          )}
        </div>

        {/* Footer - CommandPalette style */}
        <div className="px-6 py-3 border-t border-white/[0.06] bg-black/20">
          <div className="flex items-center justify-between text-xs text-white/50">
            <span>{filteredLocales.length + 1} languages</span>
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
