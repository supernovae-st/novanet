// NovaNet Core Parsers - Barrel Export
// Parse markdown locale knowledge files into typed objects

export { parseIdentityMd } from './parse-identity.js';
export { parseVoiceMd } from './parse-voice.js';
export { parseCultureMd } from './parse-culture.js';
export { parseMarketMd } from './parse-market.js';
export { parseLexiconMd } from './parse-lexicon.js';

// Shared parsing utilities
export {
  parseMarkdownTable,
  parseFieldValueTable,
  parseTableRow,
  findValue,
  extractSection,
  extractScore,
  extractString,
  extractNumber,
  isTableSeparator,
  isTableHeader,
  type TableRow,
  type FieldValueRow,
} from './utils.js';

// Re-export types for convenience
export type {
  LocaleIdentity,
  LocaleVoice,
  LocaleCulture,
  LocaleMarket,
  LocaleLexicon,
  Expression,
} from '../types/locale-knowledge.js';
