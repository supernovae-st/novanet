// packages/schema-tools/src/utils/filePathToNodeName.ts
// Shared utility for converting YAML file paths to NodeType names

/**
 * Convert kebab-case filename to PascalCase node name.
 * Handles special cases: seo → SEO, geo → GEO, ai → AI, l10n → L10n
 *
 * @param filePath - File path or filename (e.g., "locale-identity.yaml" or "nodes/global/knowledge/locale-identity.yaml")
 * @returns PascalCase node type name (e.g., "LocaleIdentity", "SEOKeywordL10n")
 *
 * @example
 * filePathToNodeName('locale-identity.yaml')        // → 'LocaleIdentity'
 * filePathToNodeName('seo-keyword-l10n.yaml')       // → 'SEOKeywordL10n'
 * filePathToNodeName('nodes/global/config/locale.yaml') // → 'Locale'
 */
export function filePathToNodeName(filePath: string): string {
  // Extract filename from path: "nodes/global/knowledge/locale-identity.yaml" → "locale-identity"
  const filename = filePath.split('/').pop()?.replace('.yaml', '') || '';

  // Handle special cases for acronyms and abbreviations
  return filename
    .split('-')
    .map((part) => {
      // Full acronyms → ALL CAPS
      if (['seo', 'geo', 'ai'].includes(part)) {
        return part.toUpperCase();
      }
      // l10n is special: L10n (not L10N)
      if (part === 'l10n') {
        return 'L10n';
      }
      // PascalCase for normal parts
      return part.charAt(0).toUpperCase() + part.slice(1);
    })
    .join('');
}
