#!/usr/bin/env node
/**
 * Generate Seed Files for Missing Locales (fil-PH, lo-LA, or-IN)
 *
 * Reads cultural data JSON files and generates Cypher seed files
 */

const fs = require('fs');
const path = require('path');

const LOCALE_FILES = [
  { locale: 'fil-PH', file: path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'fil-ph-cultural-data.json') },
  { locale: 'lo-LA', file: path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'lo-la-cultural-data.json') },
  { locale: 'or-IN', file: path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'or-in-cultural-data.json') },
];

const OUTPUT_FILE = path.join(__dirname, '..', 'seed', '27-missing-locales-knowledge.cypher');

const stats = { expressions: 0, cultureRefs: 0, taboos: 0, patterns: 0, audienceTraits: 0, locales: 0 };

function escapeCypher(text) {
  if (!text) return '';
  return text
    .replace(/\\/g, '\\\\')  // Escape backslashes FIRST
    .replace(/'/g, "\\'")     // Then escape single quotes
    .replace(/\n/g, ' ')
    .substring(0, 1000);
}

function slugify(text) {
  return text
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/[^\w\s-]/g, '')
    .replace(/[\s_]+/g, '-')
    .replace(/-+/g, '-')
    .replace(/^-|-$/g, '')
    .substring(0, 60);
}

function processLocale(localeInfo) {
  const lines = [];
  const { locale, file } = localeInfo;

  if (!fs.existsSync(file)) {
    console.warn(`  ⚠️ File not found: ${file}`);
    return lines;
  }

  const data = JSON.parse(fs.readFileSync(file, 'utf-8'));
  console.log(`  📄 Processing ${locale}: ${path.basename(file)}`);

  stats.locales++;

  lines.push(`// ============================================================================`);
  lines.push(`// ${locale} Knowledge Atoms`);
  lines.push(`// ============================================================================`);
  lines.push('');

  // Process Expressions
  if (data.expressions && data.expressions.length > 0) {
    lines.push(`// --- ${locale} Expressions ---`);
    for (const expr of data.expressions) {
      const key = `expression:${slugify(expr.text || expr.key)}@${locale}`;
      const text = escapeCypher(expr.text);
      const english = escapeCypher(expr.english);
      const context = expr.context || 'general';
      const register = expr.register || 'neutral';
      const domain = expr.domain || 'general';

      lines.push(`MERGE (e:Expression {key: '${key}'})`);
      lines.push(`SET e.text = '${text}',`);
      lines.push(`    e.english = '${english}',`);
      lines.push(`    e.locale = '${locale}',`);
      lines.push(`    e.context = '${context}',`);
      lines.push(`    e.register = '${register}',`);
      lines.push(`    e.domain = '${domain}',`);
      lines.push(`    e.provenance = 'perplexity-research',`);
      lines.push(`    e.confidence = 0.90,`);
      lines.push(`    e.created_at = datetime(),`);
      lines.push(`    e.updated_at = datetime();`);
      lines.push('');

      // Link to ExpressionSet
      lines.push(`MATCH (es:ExpressionSet {key: 'expression-set@${locale}'})`);
      lines.push(`MATCH (e:Expression {key: '${key}'})`);
      lines.push(`MERGE (es)-[:CONTAINS_EXPRESSION]->(e);`);
      lines.push('');

      stats.expressions++;
    }
  }

  // Process CultureRefs
  if (data.culture_refs && data.culture_refs.length > 0) {
    lines.push(`// --- ${locale} CultureRefs ---`);
    for (const ref of data.culture_refs) {
      const key = `culture-ref:${slugify(ref.name)}@${locale}`;
      const name = escapeCypher(ref.name);
      const description = escapeCypher(ref.description);
      const category = ref.category || 'general';

      lines.push(`MERGE (c:CultureRef {key: '${key}'})`);
      lines.push(`SET c.display_name = '${name}',`);
      lines.push(`    c.locale = '${locale}',`);
      lines.push(`    c.description = '${description}',`);
      lines.push(`    c.category = '${category}',`);
      lines.push(`    c.provenance = 'perplexity-research',`);
      lines.push(`    c.confidence = 0.90,`);
      lines.push(`    c.created_at = datetime(),`);
      lines.push(`    c.updated_at = datetime();`);
      lines.push('');

      // Link to CultureSet
      lines.push(`MATCH (cs:CultureSet {key: 'culture-set:values@${locale}'})`);
      lines.push(`MATCH (c:CultureRef {key: '${key}'})`);
      lines.push(`MERGE (cs)-[:CONTAINS_CULTURE_REF]->(c);`);
      lines.push('');

      stats.cultureRefs++;
    }
  }

  // Process Taboos
  if (data.taboos && data.taboos.length > 0) {
    lines.push(`// --- ${locale} Taboos ---`);
    for (const taboo of data.taboos) {
      const key = `taboo:${slugify(taboo.name)}@${locale}`;
      const name = escapeCypher(taboo.name);
      const description = escapeCypher(taboo.description);
      const severity = taboo.severity || 'medium';

      lines.push(`MERGE (t:Taboo {key: '${key}'})`);
      lines.push(`SET t.display_name = '${name}',`);
      lines.push(`    t.locale = '${locale}',`);
      lines.push(`    t.description = '${description}',`);
      lines.push(`    t.severity = '${severity}',`);
      lines.push(`    t.provenance = 'perplexity-research',`);
      lines.push(`    t.confidence = 0.90,`);
      lines.push(`    t.created_at = datetime(),`);
      lines.push(`    t.updated_at = datetime();`);
      lines.push('');

      // Link to TabooSet
      lines.push(`MATCH (ts:TabooSet {key: 'taboo-set:avoid@${locale}'})`);
      lines.push(`MATCH (t:Taboo {key: '${key}'})`);
      lines.push(`MERGE (ts)-[:CONTAINS_TABOO]->(t);`);
      lines.push('');

      stats.taboos++;
    }
  }

  // Process Patterns
  if (data.patterns && data.patterns.length > 0) {
    lines.push(`// --- ${locale} Patterns ---`);
    for (const pattern of data.patterns) {
      const key = `pattern:${slugify(pattern.key || pattern.template)}@${locale}`;
      const template = escapeCypher(pattern.template);
      const patternType = pattern.type || 'cta';
      const context = pattern.context || 'landing';
      const tone = pattern.tone || 'professional';

      lines.push(`MERGE (p:Pattern {key: '${key}'})`);
      lines.push(`SET p.display_name = '${template}',`);
      lines.push(`    p.locale = '${locale}',`);
      lines.push(`    p.template = '${template}',`);
      lines.push(`    p.type = '${patternType}',`);
      lines.push(`    p.context = '${context}',`);
      lines.push(`    p.tone = '${tone}',`);
      lines.push(`    p.provenance = 'perplexity-research',`);
      lines.push(`    p.confidence = 0.90,`);
      lines.push(`    p.created_at = datetime(),`);
      lines.push(`    p.updated_at = datetime();`);
      lines.push('');

      // Link to PatternSet
      lines.push(`MATCH (ps:PatternSet {key: 'pattern-set:cta@${locale}'})`);
      lines.push(`MATCH (p:Pattern {key: '${key}'})`);
      lines.push(`MERGE (ps)-[:CONTAINS_PATTERN]->(p);`);
      lines.push('');

      stats.patterns++;
    }
  }

  // Process AudienceTraits
  if (data.audience_traits) {
    lines.push(`// --- ${locale} AudienceTraits ---`);
    const traits = data.audience_traits;

    // Create individual traits from the object
    const traitEntries = [
      { name: 'formality', value: traits.formality },
      { name: 'directness', value: traits.directness },
      { name: 'decision_making', value: traits.decision_making },
    ];

    for (const trait of traitEntries) {
      if (!trait.value) continue;
      const key = `audience-trait:${slugify(trait.name)}@${locale}`;

      lines.push(`MERGE (a:AudienceTrait {key: '${key}'})`);
      lines.push(`SET a.display_name = '${escapeCypher(trait.name)}',`);
      lines.push(`    a.locale = '${locale}',`);
      lines.push(`    a.description = '${escapeCypher(String(trait.value))}',`);
      lines.push(`    a.category = 'communication',`);
      lines.push(`    a.provenance = 'perplexity-research',`);
      lines.push(`    a.confidence = 0.90,`);
      lines.push(`    a.created_at = datetime(),`);
      lines.push(`    a.updated_at = datetime();`);
      lines.push('');

      // Link to AudienceSet
      lines.push(`MATCH (as:AudienceSet {key: 'audience-set:general@${locale}'})`);
      lines.push(`MATCH (a:AudienceTrait {key: '${key}'})`);
      lines.push(`MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(a);`);
      lines.push('');

      stats.audienceTraits++;
    }

    // Add trust factors and channels as traits
    if (traits.trust_factors && Array.isArray(traits.trust_factors)) {
      for (const factor of traits.trust_factors) {
        const key = `audience-trait:trust-${slugify(factor)}@${locale}`;
        lines.push(`MERGE (a:AudienceTrait {key: '${key}'})`);
        lines.push(`SET a.display_name = '${escapeCypher(factor)}',`);
        lines.push(`    a.locale = '${locale}',`);
        lines.push(`    a.description = 'Trust factor: ${escapeCypher(factor)}',`);
        lines.push(`    a.category = 'trust',`);
        lines.push(`    a.provenance = 'perplexity-research',`);
        lines.push(`    a.confidence = 0.90,`);
        lines.push(`    a.created_at = datetime(),`);
        lines.push(`    a.updated_at = datetime();`);
        lines.push('');
        lines.push(`MATCH (as:AudienceSet {key: 'audience-set:general@${locale}'})`);
        lines.push(`MATCH (a:AudienceTrait {key: '${key}'})`);
        lines.push(`MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(a);`);
        lines.push('');
        stats.audienceTraits++;
      }
    }

    if (traits.communication_channels && Array.isArray(traits.communication_channels)) {
      for (const channel of traits.communication_channels) {
        const key = `audience-trait:channel-${slugify(channel)}@${locale}`;
        lines.push(`MERGE (a:AudienceTrait {key: '${key}'})`);
        lines.push(`SET a.display_name = '${escapeCypher(channel)}',`);
        lines.push(`    a.locale = '${locale}',`);
        lines.push(`    a.description = 'Preferred communication channel: ${escapeCypher(channel)}',`);
        lines.push(`    a.category = 'channel',`);
        lines.push(`    a.provenance = 'perplexity-research',`);
        lines.push(`    a.confidence = 0.90,`);
        lines.push(`    a.created_at = datetime(),`);
        lines.push(`    a.updated_at = datetime();`);
        lines.push('');
        lines.push(`MATCH (as:AudienceSet {key: 'audience-set:general@${locale}'})`);
        lines.push(`MATCH (a:AudienceTrait {key: '${key}'})`);
        lines.push(`MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(a);`);
        lines.push('');
        stats.audienceTraits++;
      }
    }
  }

  return lines;
}

async function main() {
  console.log('🔄 Generating seed files for missing locales (fil-PH, lo-LA, or-IN)...\n');

  const timestamp = new Date().toISOString();

  const header = [
    '// ============================================================================',
    '// Seed 27: Knowledge Atoms for Missing Locales (fil-PH, lo-LA, or-IN)',
    '// ============================================================================',
    '// Generated by: generate-missing-locale-seeds.js',
    `// Generated at: ${timestamp}`,
    '// Source: Perplexity cultural research for QR Code AI',
    '// ============================================================================',
    '',
  ];

  const allLines = [...header];

  for (const localeInfo of LOCALE_FILES) {
    const lines = processLocale(localeInfo);
    allLines.push(...lines);
  }

  fs.writeFileSync(OUTPUT_FILE, allLines.join('\n'));

  console.log(`
═══════════════════════════════════════════════════════════════════
📊 MISSING LOCALES SEED GENERATION COMPLETE
═══════════════════════════════════════════════════════════════════
   Locales processed:    ${stats.locales}
   Expressions added:    ${stats.expressions}
   CultureRefs added:    ${stats.cultureRefs}
   Taboos added:         ${stats.taboos}
   Patterns added:       ${stats.patterns}
   AudienceTraits added: ${stats.audienceTraits}

📁 Output: packages/db/seed/27-missing-locales-knowledge.cypher
`);
}

main().catch(console.error);
