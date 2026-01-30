// novanet-core/src/parsers/parse-lexicon.ts
import { LocaleLexicon, Expression } from '../types/locale-knowledge.js';
import { parseTableRow, isTableSeparator } from './utils.js';

/**
 * Result of parsing a lexicon MD file
 */
export interface ParsedLexicon {
  lexicon: Partial<LocaleLexicon>;
  expressions: Partial<Expression>[];
}

/**
 * Mapping from section titles to semantic field names
 */
const SEMANTIC_FIELD_MAP: Record<string, string> = {
  '1.1 SUCCESS / ACHIEVEMENT': 'success',
  '1.2 SPEED / EFFICIENCY': 'speed',
  '1.3 SIMPLICITY / EASE': 'simplicity',
  '1.4 QUALITY / RELIABILITY': 'quality',
  '1.5 URGENCY / SCARCITY': 'urgency',
  '1.6 TRUST / SAFETY': 'trust',
  '1.7 INNOVATION / MODERNITY': 'innovation',
  '1.8 VALUE / SAVINGS': 'value',
  '1.9 EXCLUSIVITY / PREMIUM': 'exclusivity',
  '1.10 COMMUNITY / BELONGING': 'community',
};

/**
 * Check if a line is a table header row (lexicon-specific patterns)
 */
function isLexiconTableHeader(line: string): boolean {
  const lowerLine = line.toLowerCase();
  return (
    lowerLine.includes('intention') ||
    lowerLine.includes('expression') ||
    lowerLine.includes('register') ||
    lowerLine.includes('type') ||
    lowerLine.includes('loanword') ||
    lowerLine.includes('idiom')
  );
}

/**
 * Parse expressions from a semantic field section
 */
function parseExpressionTable(sectionContent: string, semanticField: string): Partial<Expression>[] {
  const expressions: Partial<Expression>[] = [];
  const lines = sectionContent.split('\n');

  for (const line of lines) {
    // Skip non-table lines, separators, and headers
    if (!line.startsWith('|') || isTableSeparator(line) || isLexiconTableHeader(line)) {
      continue;
    }

    const cells = parseTableRow(line);

    // Expression tables have 5 columns: Intention, Expression, Register, Context, Example Sentence
    if (cells.length >= 5) {
      const rawRegister = cells[2].toLowerCase();
      let register: 'formal' | 'semi-formal' | 'casual';

      if (rawRegister === 'formal') {
        register = 'formal';
      } else if (rawRegister === 'casual') {
        register = 'casual';
      } else {
        // Handles 'semi-formal', 'semi_formal', 'semiformal'
        register = 'semi-formal';
      }

      expressions.push({
        semantic_field: semanticField,
        intention: cells[0],
        text: cells[1],
        register,
        context: cells[3],
        example_sentence: cells[4],
      });
    }
  }

  return expressions;
}

/**
 * Parse connectors table into structured format
 */
function parseConnectorsTable(content: string): Record<string, Record<string, string>> {
  const connectors: Record<string, Record<string, string>> = {};
  const connectorSection = content.match(/## 2\. Connectors & Transitions[\s\S]*?(?=## 3\.|$)/);

  if (!connectorSection) {
    return connectors;
  }

  const lines = connectorSection[0].split('\n');

  for (const line of lines) {
    if (!line.startsWith('|') || isTableSeparator(line) || line.toLowerCase().includes('type')) {
      continue;
    }

    const cells = parseTableRow(line);

    // Connectors table: Type | Formal | Semi-formal | Casual
    if (cells.length >= 4) {
      const connectorType = cells[0].toLowerCase();
      connectors[connectorType] = {
        formal: cells[1],
        'semi-formal': cells[2],
        casual: cells[3],
      };
    }
  }

  return connectors;
}

/**
 * Parse accepted loanwords table
 */
function parseAcceptedLoanwords(content: string): Array<{ word: string; context: string }> {
  const loanwords: Array<{ word: string; context: string }> = [];
  const section = content.match(/### Commonly Accepted[\s\S]*?(?=###|$)/);

  if (!section) {
    return loanwords;
  }

  const lines = section[0].split('\n');

  for (const line of lines) {
    if (!line.startsWith('|') || isTableSeparator(line) || line.toLowerCase().includes('loanword')) {
      continue;
    }

    const cells = parseTableRow(line);

    // Loanwords table: Loanword | Context | Example
    if (cells.length >= 2) {
      loanwords.push({
        word: cells[0],
        context: cells[1],
      });
    }
  }

  return loanwords;
}

/**
 * Parse native alternatives preferred table
 */
function parseNativeAlternatives(content: string): Array<{ loanword: string; native: string; when: string }> {
  const alternatives: Array<{ loanword: string; native: string; when: string }> = [];
  const section = content.match(/### Native Alternatives Preferred[\s\S]*?(?=##|$)/);

  if (!section) {
    return alternatives;
  }

  const lines = section[0].split('\n');

  for (const line of lines) {
    if (!line.startsWith('|') || isTableSeparator(line) || line.toLowerCase().includes('loanword')) {
      continue;
    }

    const cells = parseTableRow(line);

    // Table: Loanword | Native | When to use native
    if (cells.length >= 3) {
      alternatives.push({
        loanword: cells[0],
        native: cells[1],
        when: cells[2],
      });
    }
  }

  return alternatives;
}

/**
 * Parse untranslatable concepts table
 */
function parseUniqueConcepts(content: string): Array<{ expression: string; meaning: string; when: string }> {
  const concepts: Array<{ expression: string; meaning: string; when: string }> = [];
  const section = content.match(/### Untranslatable Concepts[\s\S]*?(?=###|$)/);

  if (!section) {
    return concepts;
  }

  const lines = section[0].split('\n');

  for (const line of lines) {
    if (!line.startsWith('|') || isTableSeparator(line) || line.toLowerCase().includes('expression')) {
      continue;
    }

    const cells = parseTableRow(line);

    // Table: Expression | Meaning | When to use
    if (cells.length >= 3) {
      concepts.push({
        expression: cells[0],
        meaning: cells[1],
        when: cells[2],
      });
    }
  }

  return concepts;
}

/**
 * Parse common idioms table
 */
function parseCommonIdioms(content: string): Array<{ idiom: string; meaning: string; context: string }> {
  const idioms: Array<{ idiom: string; meaning: string; context: string }> = [];
  const section = content.match(/### Common Idioms & Proverbs[\s\S]*?(?=##|$)/);

  if (!section) {
    return idioms;
  }

  const lines = section[0].split('\n');

  for (const line of lines) {
    if (!line.startsWith('|') || isTableSeparator(line) || line.toLowerCase().includes('idiom')) {
      continue;
    }

    const cells = parseTableRow(line);

    // Table: Idiom | Meaning | Usage Context
    if (cells.length >= 3) {
      idioms.push({
        idiom: cells[0],
        meaning: cells[1],
        context: cells[2],
      });
    }
  }

  return idioms;
}

/**
 * Parse a lexicon MD file and extract both lexicon metadata and expressions
 *
 * @param content - The raw markdown content of the lexicon file
 * @returns Parsed lexicon metadata and array of expressions
 */
export function parseLexiconMd(content: string): ParsedLexicon {
  const lexicon: Partial<LocaleLexicon> = {};
  const expressions: Partial<Expression>[] = [];

  // Parse loanwords policy
  const policyMatch = content.match(/\*\*Approach for [^*]+\*\*:\s*(\w+)/);
  if (policyMatch) {
    const policy = policyMatch[1].toLowerCase();
    if (policy.includes('mixed')) {
      lexicon.loanwords_policy = 'mixed';
    } else if (policy.includes('native')) {
      lexicon.loanwords_policy = 'native_only';
    } else {
      lexicon.loanwords_policy = 'english_ok';
    }
  }

  // Parse expression density from Critical Rules section
  const densityMatch = content.match(/Apply expressions to \*\*([^*]+)\*\*/);
  if (densityMatch) {
    lexicon.expression_density = densityMatch[1];
  }

  // Parse rotation rule
  const rotationMatch = content.match(/Rotate expressions \(([^)]+)\)/);
  if (rotationMatch) {
    lexicon.rotation_rule = rotationMatch[1];
  } else if (content.includes('never repeat in same content piece')) {
    lexicon.rotation_rule = 'never repeat in same content piece';
  }

  // Parse register matching rule
  lexicon.register_matching = content.includes('Match register to context');

  // Parse connectors
  lexicon.connectors = parseConnectorsTable(content);

  // Parse accepted loanwords
  lexicon.accepted_loanwords = parseAcceptedLoanwords(content);

  // Parse native alternatives
  lexicon.prefer_native = parseNativeAlternatives(content);

  // Parse unique concepts
  lexicon.unique_concepts = parseUniqueConcepts(content);

  // Parse common idioms
  lexicon.common_idioms = parseCommonIdioms(content);

  // Parse each semantic field section and extract expressions
  for (const [sectionTitle, fieldName] of Object.entries(SEMANTIC_FIELD_MAP)) {
    // Escape special regex characters in the section title
    const escapedTitle = sectionTitle.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

    // Match the section content from ### title to the next ### or ## or end
    const sectionPattern = new RegExp(`### ${escapedTitle}[\\s\\S]*?(?=### \\d|## \\d|$)`);
    const sectionMatch = content.match(sectionPattern);

    if (sectionMatch) {
      const sectionExpressions = parseExpressionTable(sectionMatch[0], fieldName);
      expressions.push(...sectionExpressions);
    }
  }

  return { lexicon, expressions };
}
