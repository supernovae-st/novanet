'use client';

import { memo, useState } from 'react';
import { ChevronDown } from 'lucide-react';
import { cn } from '@/lib/utils';
import { useFilterStore } from '@/stores/filterStore';
import { getLocaleInfo } from '@/config/locales';
import { iconSizes, gapTokens } from '@/design/tokens';
import { LocalePicker } from '@/components/sidebar';

interface LocaleFilterSelectorProps {
  className?: string;
}

/**
 * Compact button to select a locale filter for graph visualization.
 * Shows current locale (flag + code) and opens LocalePicker modal on click.
 */
export const LocaleFilterSelector = memo(function LocaleFilterSelector({
  className,
}: LocaleFilterSelectorProps) {
  const selectedLocale = useFilterStore((state) => state.selectedLocale);
  const [isPickerOpen, setIsPickerOpen] = useState(false);

  const localeInfo = selectedLocale ? getLocaleInfo(selectedLocale) : null;

  return (
    <>
      <button
        onClick={() => setIsPickerOpen(true)}
        className={cn(
          'inline-flex items-center',
          'text-white/60 hover:text-white/80',
          'text-xs font-mono cursor-pointer',
          'pl-2 pr-2 py-1 rounded',
          'hover:bg-white/5 transition-colors',
          'border border-transparent hover:border-white/10',
          gapTokens.tight,
          className
        )}
        title={selectedLocale ? `Filter: ${localeInfo?.name}` : 'Filter by locale'}
      >
        {selectedLocale && localeInfo ? (
          <>
            <span className="text-sm">{localeInfo.flag}</span>
            <span>{selectedLocale}</span>
          </>
        ) : (
          <>
            <span className="text-sm">🌍</span>
            <span>World</span>
          </>
        )}
        <ChevronDown className={cn(iconSizes.xs, 'text-white/40')} />
      </button>

      <LocalePicker
        isOpen={isPickerOpen}
        onClose={() => setIsPickerOpen(false)}
      />
    </>
  );
});
