/**
 * Locale Utilities
 *
 * Functions for extracting locale from node keys and converting to flag emojis.
 *
 * Key pattern: {type}:{invariant_key}@{locale}
 * Example: entity:qr-code@fr-FR → locale: fr-FR
 */

/**
 * Extract locale from a node key using the composite key pattern.
 *
 * @param key - Node key (e.g., "entity:qr-code@fr-FR")
 * @returns Locale code or undefined if not found
 *
 * @example
 * extractLocaleFromKey("entity:qr-code@fr-FR") // "fr-FR"
 * extractLocaleFromKey("page:homepage@de-DE") // "de-DE"
 * extractLocaleFromKey("Project:my-project") // undefined
 */
export function extractLocaleFromKey(key: string): string | undefined {
  // Pattern: ...@{locale} at the end
  const atIndex = key.lastIndexOf('@');
  if (atIndex === -1) return undefined;

  const locale = key.slice(atIndex + 1);

  // Validate BCP-47 format (basic check: xx-XX or xx)
  if (!/^[a-z]{2}(-[A-Z]{2})?$/.test(locale)) {
    return undefined;
  }

  return locale;
}

/**
 * Convert ISO 3166-1 alpha-2 country code to flag emoji.
 *
 * Uses Unicode regional indicator symbols.
 * Each letter A-Z maps to 🇦-🇿 (U+1F1E6 to U+1F1FF).
 *
 * @param countryCode - ISO 3166-1 alpha-2 country code (e.g., "FR", "US")
 * @returns Flag emoji (e.g., "🇫🇷", "🇺🇸")
 *
 * @example
 * countryCodeToFlag("FR") // "🇫🇷"
 * countryCodeToFlag("US") // "🇺🇸"
 * countryCodeToFlag("JP") // "🇯🇵"
 */
export function countryCodeToFlag(countryCode: string): string {
  const code = countryCode.toUpperCase();

  if (code.length !== 2) {
    return '🏳️'; // White flag fallback
  }

  // Regional indicator symbols start at U+1F1E6 (🇦)
  // A = 65 in ASCII, so we offset by 0x1F1E6 - 65 = 127397
  const offset = 127397;

  return String.fromCodePoint(
    code.charCodeAt(0) + offset,
    code.charCodeAt(1) + offset
  );
}

/**
 * Convert BCP-47 locale code to flag emoji.
 *
 * Extracts the country/region code from the locale and converts to flag.
 *
 * @param locale - BCP-47 locale code (e.g., "fr-FR", "en-US", "ja")
 * @returns Flag emoji or globe for language-only locales
 *
 * @example
 * localeToFlag("fr-FR") // "🇫🇷"
 * localeToFlag("en-US") // "🇺🇸"
 * localeToFlag("pt-BR") // "🇧🇷"
 * localeToFlag("en") // "🌐" (no country code)
 */
export function localeToFlag(locale: string): string {
  // Try to extract country code from locale (e.g., "fr-FR" → "FR")
  const parts = locale.split('-');

  if (parts.length >= 2 && parts[1].length === 2) {
    return countryCodeToFlag(parts[1]);
  }

  // No country code (e.g., "en", "fr") - return globe
  return '🌐';
}

/**
 * Format locale for display with flag emoji.
 *
 * @param locale - BCP-47 locale code
 * @returns Formatted string with flag and code
 *
 * @example
 * formatLocaleWithFlag("fr-FR") // "🇫🇷 fr-FR"
 * formatLocaleWithFlag("en-US") // "🇺🇸 en-US"
 */
export function formatLocaleWithFlag(locale: string): string {
  return `${localeToFlag(locale)} ${locale}`;
}
