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
import { pickerClasses, iconSizes, gapTokens, getCardStagger } from '@/design/tokens';
import { Kbd } from '@/components/ui';
import { useFilterStore } from '@/stores/filterStore';
import { useQueryStore } from '@/stores/queryStore';
import { injectFilters } from '@/lib/cypher/injectFilters';
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

// Rich Locale Card - uses pickerClasses tokens + stagger animation
interface LocaleCardProps {
  code: string;
  info: { name: string; flag: string };
  isSelected: boolean;
  isFocused: boolean;
  onSelect: () => void;
  isAllLocales?: boolean;
  totalCount?: number;
  index: number;
}

const LocaleCard = memo(function LocaleCard({
  code,
  info,
  isSelected,
  isFocused,
  onSelect,
  isAllLocales = false,
  totalCount,
  index,
}: LocaleCardProps) {
  return (
    <button
      onClick={onSelect}
      role="option"
      aria-selected={isSelected}
      aria-label={isAllLocales ? `All languages (${totalCount})` : `${info.name} (${code})`}
      className={cn(
        pickerClasses.cardBase,
        'min-h-[110px]',
        isSelected
          ? 'bg-novanet-500/20 border-novanet-500/30 text-white'
          : isFocused
            ? pickerClasses.cardFocused
            : pickerClasses.cardIdle,
        isAllLocales && !isSelected && pickerClasses.cardAll,
        pickerClasses.cardAnimation,
        getCardStagger(index),
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
  const { selectedLocale, setSelectedLocale, displayLimit } = useFilterStore(
    useShallow((state) => ({
      selectedLocale: state.selectedLocale,
      setSelectedLocale: state.setSelectedLocale,
      displayLimit: state.displayLimit,
    }))
  );

  // v12.1: Query-First - re-execute current query when locale changes
  const currentQuery = useQueryStore((state) => state.currentQuery);
  const executeQuery = useQueryStore((state) => state.executeQuery);

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

  // v12.1: Query-First - re-execute current query with new locale
  const handleSelect = useCallback(
    (code: string | null) => {
      setSelectedLocale(code);
      // Re-execute current query with locale filter applied
      if (currentQuery) {
        const modifiedQuery = injectFilters(currentQuery, {
          displayLimit,
          localeKey: code || undefined,
        });
        executeQuery(modifiedQuery);
      }
      delayedClose();
    },
    [setSelectedLocale, delayedClose, currentQuery, executeQuery, displayLimit]
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
    <div className={pickerClasses.container} role="presentation">
      {/* Backdrop - Raycast blur ramp */}
      <div className={pickerClasses.backdrop} aria-hidden="true" />

      {/* Modal shell */}
      <div
        ref={containerRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="locale-picker-title"
        onKeyDown={handleKeyDown}
        className={cn(pickerClasses.shell, pickerClasses.sizeLarge, 'h-[80vh]')}
      >
        {/* Header */}
        <div className={pickerClasses.header}>
          <div className={cn('flex items-center', gapTokens.spacious)}>
            <div className={cn(pickerClasses.headerIconBox, 'bg-teal-500/20 border-teal-500/30')}>
              <Globe className={cn(iconSizes.xl, 'text-teal-400')} />
            </div>
            <div>
              <h2 id="locale-picker-title" className={pickerClasses.headerTitle}>
                Select Locale
              </h2>
              <p className={pickerClasses.headerSubtitle}>Choose a language and region</p>
            </div>
          </div>
          <button onClick={onClose} aria-label="Close" className={pickerClasses.closeButton}>
            <X className={iconSizes.xl} />
          </button>
        </div>

        {/* Search */}
        <div className={pickerClasses.searchBar}>
          <Search className={cn(iconSizes.xl, 'text-white/40 shrink-0')} />
          <input
            ref={searchRef}
            type="text"
            value={searchInput}
            onChange={(e) => setSearchInput(e.target.value)}
            placeholder="Search languages\u2026"
            aria-label="Search languages"
            aria-describedby="locale-picker-hint"
            className={pickerClasses.searchInput}
            autoComplete="off"
            spellCheck={false}
          />
          <span id="locale-picker-hint" className="sr-only">Type to filter available languages</span>
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
          aria-label="Available languages"
        >
          <div
            ref={gridRef}
            className={cn('grid grid-cols-4', gapTokens.xl)}
          >
            {/* World card (all locales) */}
            <LocaleCard
              code=""
              info={{ name: 'World', flag: '🌍' }}
              isSelected={selectedLocale === null}
              isFocused={focusedIndex === 0}
              onSelect={() => handleSelect(null)}
              isAllLocales
              totalCount={ALL_LOCALES.length}
              index={0}
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
                  index={index + 1}
                />
              );
            })}
          </div>

          {/* No results */}
          {filteredLocales.length === 0 && (
            <div className={pickerClasses.emptyState}>
              <Search className={cn(iconSizes.xl, 'mx-auto mb-3 opacity-30')} />
              <p className="text-sm font-medium">No languages found</p>
              <p className="text-xs opacity-60 mt-1">Try a different search term</p>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className={pickerClasses.footer}>
          <div className={pickerClasses.footerContent}>
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
