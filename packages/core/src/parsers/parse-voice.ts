// novanet-core/src/parsers/parse-voice.ts
import { LocaleVoice } from '../types/locale-knowledge.js';
import { parseMarkdownTable, extractScore, extractString } from './utils.js';

/**
 * Parse punctuation rules from section 5
 */
function parsePunctuationRules(content: string): Record<string, string> {
  const rules: Record<string, string> = {};

  const punctuationSection = content.match(/### 5\.1 Punctuation Style[\s\S]*?(?=###|$)/);
  if (punctuationSection) {
    const rows = parseMarkdownTable(punctuationSection[0]);
    for (const row of rows) {
      const mark = row['mark'] || '';
      const spacing = row['spacing rule'] || '';
      if (mark && spacing) {
        // Extract the punctuation character from the mark (e.g., "Period (.)" -> ".")
        const charMatch = mark.match(/\((.)\)/);
        const char = charMatch ? charMatch[1] : mark;
        rules[char] = spacing;
      }
    }
  }

  return rules;
}

/**
 * Parse honorific system from section 2
 */
function parseHonorificSystem(content: string): Record<string, unknown> | undefined {
  const honorificSection = content.match(/## 2\. Honorific System[\s\S]*?(?=## \d|$)/);
  if (!honorificSection || honorificSection[0].includes('[CONDITIONAL]')) {
    // Check if section exists with actual content
    const levelsSection = content.match(/### 2\.1 Levels and Usage[\s\S]*?(?=###|## |$)/);
    if (levelsSection) {
      const rows = parseMarkdownTable(levelsSection[0]);
      if (rows.length > 0) {
        const honorifics: Record<string, { level: string; when_to_use: string; example: string }> = {};
        for (const row of rows) {
          const honorific = row['honorific'];
          if (honorific) {
            honorifics[honorific] = {
              level: row['level'] || '',
              when_to_use: row['when to use'] || '',
              example: row['example'] || '',
            };
          }
        }
        return honorifics;
      }
    }
  }
  return undefined;
}

/**
 * Parse warmth by stage from section 3.2
 */
function parseWarmthByStage(content: string): Record<string, number> {
  const warmthByStage: Record<string, number> = {};

  const warmthSection = content.match(/### 3\.2 Warmth Level[\s\S]*?(?=###|$)/);
  if (warmthSection) {
    const rows = parseMarkdownTable(warmthSection[0]);
    for (const row of rows) {
      const stage = row['relationship stage'];
      const warmthLevel = row['warmth level'];
      if (stage && warmthLevel) {
        const scoreMatch = warmthLevel.match(/(\d+)\/100/);
        if (scoreMatch) {
          warmthByStage[stage.toLowerCase().replace(/\s+/g, '_')] = parseInt(scoreMatch[1], 10);
        }
      }
    }
  }

  return warmthByStage;
}

/**
 * Parse humor types from section 3.3
 */
function parseHumorTypes(content: string): Record<string, string> {
  const humorTypes: Record<string, string> = {};

  const humorSection = content.match(/### 3\.3 Humor Appropriateness[\s\S]*?(?=###|## |$)/);
  if (humorSection) {
    const rows = parseMarkdownTable(humorSection[0]);
    for (const row of rows) {
      const type = row['type'];
      const status = row['status'];
      if (type && status) {
        humorTypes[type.toLowerCase().replace(/\s+/g, '_')] = status.toLowerCase();
      }
    }
  }

  return humorTypes;
}

/**
 * Parse pronoun rules from section 1.2
 */
function parsePronounRules(content: string): Record<string, unknown> {
  const pronounRules: Record<string, unknown> = {};

  const pronounSection = content.match(/### 1\.2 Pronoun Usage[\s\S]*?(?=###|$)/);
  if (pronounSection) {
    const rows = parseMarkdownTable(pronounSection[0]);
    for (const row of rows) {
      const pronoun = row['pronoun'];
      const type = row['type'];
      const when = row['when to use'];
      if (pronoun && type) {
        pronounRules[pronoun.toLowerCase()] = {
          type: type.toLowerCase(),
          when_to_use: when || '',
        };
      }
    }

    // Extract core principle
    const principleMatch = pronounSection[0].match(/\*\*Core Principle\*\*:\s*([^\n]+)/);
    if (principleMatch) {
      pronounRules['core_principle'] = principleMatch[1].trim();
    }

    // Extract critical rules
    const criticalRulesMatch = pronounSection[0].match(/\*\*Critical Rules\*\*:([\s\S]*?)(?=###|$)/);
    if (criticalRulesMatch) {
      const rules = criticalRulesMatch[1]
        .split('\n')
        .filter(l => l.trim().startsWith('-'))
        .map(l => l.replace(/^-\s*\*\*/, '').replace(/\*\*.*$/, '').trim());
      pronounRules['critical_rules'] = rules;
    }
  }

  return pronounRules;
}

/**
 * Parse softening patterns from section 3.1
 */
function parseSofteningPatterns(content: string): Record<string, string> {
  const patterns: Record<string, string> = {};

  const directnessSection = content.match(/### 3\.1 Directness Level[\s\S]*?(?=###|$)/);
  if (directnessSection) {
    const localeChars = directnessSection[0].match(/\*\*Locale Characteristics\*\*:([\s\S]*?)(?=###|$)/);
    if (localeChars) {
      const items = localeChars[1].split('\n').filter(l => l.trim().startsWith('-'));
      for (const item of items) {
        // Extract pattern type and examples
        if (item.includes('conditional mood')) {
          const examples = item.match(/"([^"]+)"/g);
          if (examples) {
            patterns['conditional_mood'] = examples.map(e => e.replace(/"/g, '')).join(', ');
          }
        } else if (item.includes('negatives')) {
          const examples = item.match(/"([^"]+)"/g);
          if (examples) {
            patterns['soften_negatives'] = examples.map(e => e.replace(/"/g, '')).join(', ');
          }
        } else if (item.includes('alternatives')) {
          const examples = item.match(/"([^"]+)"/g);
          if (examples) {
            patterns['suggest_alternatives'] = examples.map(e => e.replace(/"/g, '')).join(', ');
          }
        }
      }
    }
  }

  return patterns;
}

/**
 * Determine directness style from score
 */
function getDirectnessStyle(score: number): 'direct' | 'indirect' | 'balanced' {
  if (score < 40) return 'indirect';
  if (score > 60) return 'direct';
  return 'balanced';
}

/**
 * Parse a Voice Style MD file and extract LocaleVoice data
 *
 * @param content - Raw markdown content of the voice style file
 * @returns Partial<LocaleVoice> with extracted values
 */
export function parseVoiceMd(content: string): Partial<LocaleVoice> {
  const result: Partial<LocaleVoice> = {};

  // 1.1 Formality Score
  result.formality_score = extractScore(content, /\*\*Overall Formality Score\*\*:\s*(\d+)\/100/);

  // 1.1 Default Formality Mode
  const formalityMode = extractString(content, /\*\*Default Formality Mode\*\*:\s*(\w+)/);
  if (formalityMode) {
    result.default_formality = formalityMode.toLowerCase() as 'formal' | 'casual' | 'mixed';
  }

  // 1.2 Default Pronoun (from table - find the formal one as default)
  const pronounSection = content.match(/### 1\.2 Pronoun Usage[\s\S]*?(?=###|$)/);
  if (pronounSection) {
    const rows = parseMarkdownTable(pronounSection[0]);
    const formalRow = rows.find(r => r['type']?.toLowerCase() === 'formal');
    if (formalRow) {
      result.default_pronoun = formalRow['pronoun']?.toLowerCase() || null;
    }
  }

  // 1.2 Pronoun Rules
  result.pronoun_rules = parsePronounRules(content);

  // 3.1 Directness Score
  result.directness_score = extractScore(content, /### 3\.1 Directness Level[\s\S]*?\*\*Score\*\*:\s*(\d+)\/100/);
  result.directness_style = getDirectnessStyle(result.directness_score);

  // 3.1 Softening Patterns
  result.softening_patterns = parseSofteningPatterns(content);

  // 3.2 Warmth Score
  result.warmth_score = extractScore(content, /### 3\.2 Warmth Level[\s\S]*?\*\*Score\*\*:\s*(\d+)\/100/);

  // 3.2 Warmth by Stage
  result.warmth_by_stage = parseWarmthByStage(content);

  // 3.3 Humor Score
  result.humor_score = extractScore(content, /### 3\.3 Humor Appropriateness[\s\S]*?\*\*Score\*\*:\s*(\d+)\/100/);

  // 3.3 Humor Types
  result.humor_types = parseHumorTypes(content);

  // 4.1 Average Sentence Length
  result.avg_sentence_length = extractScore(content, /\*\*Average Target\*\*:\s*(\d+)\s*words/);

  // 4.3 Preferred Voice
  const voice = extractString(content, /\*\*Preferred Voice\*\*:\s*(\w+)/);
  if (voice) {
    result.preferred_voice = voice.toLowerCase() as 'active' | 'passive' | 'mixed';
  }

  // 4.2 Rhythm Style
  const rhythm = extractString(content, /\*\*Rhythm\*\*:\s*([^\n]+)/);
  if (rhythm) {
    result.rhythm_style = rhythm;
  }

  // 5. Punctuation Rules
  result.punctuation_rules = parsePunctuationRules(content);

  // 2. Honorific System (conditional)
  result.honorific_system = parseHonorificSystem(content);

  return result;
}
