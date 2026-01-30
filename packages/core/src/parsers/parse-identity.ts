// novanet-core/src/parsers/parse-identity.ts
import { LocaleIdentity } from '../types/locale-knowledge.js';
import { parseFieldValueTable, findValue, extractSection } from './utils.js';

/**
 * Parse an identity markdown file and extract LocaleIdentity data
 *
 * @param content - The raw markdown content of an identity file
 * @returns Partial<LocaleIdentity> with parsed fields
 */
export function parseIdentityMd(content: string): Partial<LocaleIdentity> {
  const result: Partial<LocaleIdentity> = {};

  // === Section 1: Locale Identification ===
  // Parse Core Codes section (1.1)
  const coreCodesSection = extractSection(content, /### 1\.1 Core Codes[\s\S]*?(?=###|$)/);
  if (coreCodesSection) {
    const rows = parseFieldValueTable(coreCodesSection);
    result.script_code = findValue(rows, 'script_code') || 'Latn';
  }

  // === Section 2: Script & Writing System ===
  // Parse Script Properties section (2.1)
  const scriptSection = extractSection(content, /### 2\.1 Script Properties[\s\S]*?(?=###|$)/);
  if (scriptSection) {
    const rows = parseFieldValueTable(scriptSection);
    result.script_name = findValue(rows, 'script_name');
    const direction = findValue(rows, 'script_direction');
    result.script_direction = (direction === 'rtl' ? 'rtl' : 'ltr') as 'ltr' | 'rtl';
    result.has_case = findValue(rows, 'has_case').toLowerCase() === 'true';
  }

  // Parse Character Set section (2.2)
  const charSection = extractSection(content, /### 2\.2 Character Set[\s\S]*?(?=###|$)/);
  if (charSection) {
    const rows = parseFieldValueTable(charSection);
    result.special_characters = findValue(rows, 'special_characters');
    result.diacritics = findValue(rows, 'diacritics').toLowerCase() === 'true';
    result.ligatures = findValue(rows, 'ligatures').toLowerCase() === 'true';
  }

  // === Section 3: Geographic Context ===
  // Parse Location Hierarchy section (3.1)
  const locationSection = extractSection(content, /### 3\.1 Location Hierarchy[\s\S]*?(?=###|$)/);
  if (locationSection) {
    const rows = parseFieldValueTable(locationSection);
    result.continent = findValue(rows, 'continent');
    result.region = findValue(rows, 'region');
  }

  // Parse Country Data section (3.2)
  const countrySection = extractSection(content, /### 3\.2 Country Data[\s\S]*?(?=###|$)/);
  if (countrySection) {
    const rows = parseFieldValueTable(countrySection);
    result.capital = findValue(rows, 'capital');
  }

  // Parse Time section (3.3)
  const timeSection = extractSection(content, /### 3\.3 Time[\s\S]*?(?=###|$)/);
  if (timeSection) {
    const rows = parseFieldValueTable(timeSection);
    result.timezone = findValue(rows, 'timezone');
    result.utc_offset = findValue(rows, 'utc_offset');
    result.dst_observed = findValue(rows, 'dst_observed').toLowerCase() === 'true';
  }

  // === Section 4: Language Classification ===
  // Parse Family Tree section (4.1)
  const familySection = extractSection(content, /### 4\.1 Family Tree[\s\S]*?(?=###|$)/);
  if (familySection) {
    const rows = parseFieldValueTable(familySection);
    result.language_family = findValue(rows, 'family');
    const related = findValue(rows, 'related_languages');
    result.related_languages = related ? related.split(',').map(s => s.trim()) : [];
  }

  // Parse Technical Config section (4.2)
  const techSection = extractSection(content, /### 4\.2 Technical Config[\s\S]*?(?=###|$)/);
  if (techSection) {
    const rows = parseFieldValueTable(techSection);
    result.keyboard_layout = findValue(rows, 'keyboard_layout');
    result.encoding = findValue(rows, 'encoding') || 'UTF-8';
  }

  return result;
}
