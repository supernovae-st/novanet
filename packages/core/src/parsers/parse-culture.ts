// novanet-core/src/parsers/parse-culture.ts
import { LocaleCulture } from '../types/locale-knowledge.js';
import { parseMarkdownTable, extractSection } from './utils.js';

/**
 * Parse dominant values from section 1.1
 */
function parseDominantValues(section: string): Array<{ value: string; importance: string; marketing_angle: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    value: row.value || '',
    importance: (row.importance || '').toLowerCase(),
    marketing_angle: row.marketing_angle || '',
  })).filter(r => r.value);
}

/**
 * Parse positive triggers from section 1.2
 */
function parsePositiveTriggers(section: string): Array<{ theme: string; why: string; example: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    theme: row.theme || '',
    why: row.why_it_works || '',
    example: row.example_usage || '',
  })).filter(r => r.theme);
}

/**
 * Parse national pride points from section 1.3
 */
function parseNationalPride(section: string): Array<{ topic: string; sensitivity: string; notes: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    topic: row.pride_point || '',
    sensitivity: (row.sensitivity || '').toLowerCase(),
    notes: row.usage_notes || '',
  })).filter(r => r.topic);
}

/**
 * Parse taboo topics from section 3.1
 */
function parseTabooTopics(section: string): Array<{ topic: string; severity: string; notes: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    topic: row.topic || '',
    severity: (row.severity || '').toLowerCase(),
    notes: row.notes || '',
  })).filter(r => r.topic);
}

/**
 * Parse historical sensitivities from section 3.2
 */
function parseHistoricalSensitivities(section: string): Array<{ event: string; sensitivity: string; handling: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    event: row['period/event'] || row.period || row.event || '',
    sensitivity: (row.sensitivity || '').toLowerCase(),
    handling: row.handling || '',
  })).filter(r => r.event);
}

/**
 * Parse political sensitivities from section 3.3
 */
function parsePoliticalSensitivities(section: string): Array<{ topic: string; sensitivity: string; safe_approach: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    topic: row.topic || '',
    sensitivity: (row.sensitivity || '').toLowerCase(),
    safe_approach: row.safe_approach || '',
  })).filter(r => r.topic);
}

/**
 * Parse content prohibitions from section 4.1
 */
function parseContentProhibitions(section: string): Array<{ category: string; restriction: string; legal_basis: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    category: row.category || '',
    restriction: row.restriction || '',
    legal_basis: row.legal_basis || '',
  })).filter(r => r.category);
}

/**
 * Parse restricted imagery from section 4.2
 */
function parseRestrictedImagery(section: string): Array<{ type: string; restriction: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    type: row.type || '',
    restriction: row.restriction || '',
  })).filter(r => r.type);
}

/**
 * Parse cultural phrases from section 5.1
 */
function parseCulturalPhrases(section: string): Array<{ phrase: string; meaning: string; when_to_use: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    phrase: row.phrase || '',
    meaning: row.meaning || '',
    when_to_use: row.context || '',
  })).filter(r => r.phrase);
}

/**
 * Parse phrases to avoid from section 5.3
 */
function parsePhrasesToAvoid(section: string): Array<{ context: string; avoid: string; reason: string }> {
  const rows = parseMarkdownTable(section);
  return rows.map(row => ({
    context: row.context || '',
    avoid: row.avoid || '',
    reason: row.reason || '',
  })).filter(r => r.context || r.avoid);
}

/**
 * Get section content from header to next section
 * More robust section extraction that doesn't stop at table separators
 */
function getSection(content: string, header: string): string | null {
  const headerPattern = new RegExp(`### ${header.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}`, 'i');
  const headerMatch = content.match(headerPattern);
  if (!headerMatch || headerMatch.index === undefined) return null;

  const startIdx = headerMatch.index;
  const afterHeader = content.slice(startIdx);

  // Find the next section header (## or ###) or standalone divider (---\n at start of line)
  const nextSectionMatch = afterHeader.match(/\n(?=##[^#]|###\s|\n---\n)/);
  const endIdx = nextSectionMatch ? startIdx + (nextSectionMatch.index || afterHeader.length) : content.length;

  return content.slice(startIdx, endIdx);
}

/**
 * Parse the culture norms MD file and extract LocaleCulture data
 * @param content The raw markdown content
 * @returns Partial LocaleCulture object
 */
export function parseCultureMd(content: string): Partial<LocaleCulture> {
  const result: Partial<LocaleCulture> = {};

  // Context level - look for HIGH_CONTEXT, MEDIUM_CONTEXT, or LOW_CONTEXT
  const contextMatch = content.match(/\*\*Type\*\*:\s*(HIGH|MEDIUM|LOW)_CONTEXT/i);
  if (contextMatch) {
    result.context_level = contextMatch[1].toLowerCase() as 'high' | 'medium' | 'low';
  }

  // Hierarchy sensitivity - look for Level pattern in section 2.3
  const hierarchySection = extractSection(content, /### 2\.3 Hierarchy Sensitivity[\s\S]*?(?=###|---|$)/);
  if (hierarchySection) {
    const hierarchyMatch = hierarchySection.match(/\*\*Level\*\*:\s*(HIGH|MEDIUM|LOW)/i);
    if (hierarchyMatch) {
      result.hierarchy_sensitivity = hierarchyMatch[1].toLowerCase() as 'high' | 'medium' | 'low';
    }
  }

  // Parse Section 1.1 - Dominant Values
  const dominantValuesSection = getSection(content, '1.1 Dominant Values');
  if (dominantValuesSection) {
    result.dominant_values = parseDominantValues(dominantValuesSection);
  }

  // Parse Section 1.2 - Positive Triggers
  const positiveTriggersSection = getSection(content, '1.2 Positive Triggers');
  if (positiveTriggersSection) {
    result.positive_triggers = parsePositiveTriggers(positiveTriggersSection);
  }

  // Parse Section 1.3 - National Pride Points
  const nationalPrideSection = getSection(content, '1.3 National Pride Points');
  if (nationalPrideSection) {
    result.national_pride = parseNationalPride(nationalPrideSection);
  }

  // Parse Section 3.1 - Topics to Avoid (Taboos)
  const tabooSection = getSection(content, '3.1 Topics to Avoid');
  if (tabooSection) {
    result.taboo_topics = parseTabooTopics(tabooSection);
  }

  // Parse Section 3.2 - Historical Sensitivities
  const historicalSection = getSection(content, '3.2 Historical Sensitivities');
  if (historicalSection) {
    result.historical_sensitivities = parseHistoricalSensitivities(historicalSection);
  }

  // Parse Section 3.3 - Political Sensitivities
  const politicalSection = getSection(content, '3.3 Political Sensitivities');
  if (politicalSection) {
    result.political_sensitivities = parsePoliticalSensitivities(politicalSection);
  }

  // Parse Section 4.1 - Prohibited Content (if present)
  const prohibitedSection = getSection(content, '4.1 Prohibited Content');
  if (prohibitedSection) {
    result.content_prohibitions = parseContentProhibitions(prohibitedSection);
  }

  // Parse Section 4.2 - Restricted Imagery (if present)
  const restrictedImagerySection = getSection(content, '4.2 Restricted Imagery');
  if (restrictedImagerySection) {
    result.restricted_imagery = parseRestrictedImagery(restrictedImagerySection);
  }

  // Parse Section 5.1 - Common Phrases
  const culturalPhrasesSection = getSection(content, '5.1 Common Phrases');
  if (culturalPhrasesSection) {
    result.cultural_phrases = parseCulturalPhrases(culturalPhrasesSection);
  }

  // Parse Section 5.3 - When NOT to Use
  const phrasesToAvoidSection = getSection(content, '5.3 When NOT to Use');
  if (phrasesToAvoidSection) {
    result.phrases_to_avoid = parsePhrasesToAvoid(phrasesToAvoidSection);
  }

  // Parse Gender Considerations from section 6.1
  const genderSection = getSection(content, '6.1 Gender Considerations');
  if (genderSection) {
    const genderRows = parseMarkdownTable(genderSection);
    const genderConsiderations: Record<string, string> = {};
    genderRows.forEach(row => {
      const aspect = row.aspect || '';
      const norm = row.norm || '';
      const implication = row.marketing_implication || '';
      if (aspect) {
        genderConsiderations[aspect] = `${norm} | ${implication}`.trim();
      }
    });
    result.gender_considerations = genderConsiderations;
  }

  // Parse Age/Seniority from section 6.2
  const ageSection = getSection(content, '6.2 Age/Seniority');
  if (ageSection) {
    const ageRows = parseMarkdownTable(ageSection);
    const ageNorms: Record<string, string> = {};
    ageRows.forEach(row => {
      const context = row.context || '';
      const behavior = row.expected_behavior || '';
      const notes = row.notes || '';
      if (context) {
        ageNorms[context] = `${behavior} | ${notes}`.trim();
      }
    });
    result.age_norms = ageNorms;
  }

  // Parse Time & Scheduling from section 7
  const timeSection = extractSection(content, /### 7\.1 Work Week[\s\S]*?### 7\.3 Punctuality[\s\S]*?(?=---|$)/);
  if (timeSection) {
    result.time_norms = {};

    // Work week
    const workWeekMatch = timeSection.match(/Work days\s*\|\s*([^|]+)/);
    if (workWeekMatch) {
      result.time_norms['work_days'] = workWeekMatch[1].trim();
    }

    // Work hours
    const workHoursMatch = timeSection.match(/Work hours\s*\|\s*([^|]+)/);
    if (workHoursMatch) {
      result.time_norms['work_hours'] = workHoursMatch[1].trim();
    }

    // Punctuality
    const punctualityMatch = timeSection.match(/Business meetings\s*\|\s*([^|]+)/);
    if (punctualityMatch) {
      result.time_norms['punctuality_business'] = punctualityMatch[1].trim();
    }
  }

  return result;
}
