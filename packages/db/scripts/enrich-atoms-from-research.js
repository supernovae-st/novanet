#!/usr/bin/env node
/**
 * Enrich Knowledge Atoms from Perplexity Research JSON files
 *
 * Reads: germanic, asian, other, romance research files
 * Generates: 25.4-research-culture-refs.cypher, 25.5-research-taboos.cypher, 25.6-research-audience.cypher
 */

const fs = require('fs');
const path = require('path');

const RESEARCH_FILES = [
  path.join(__dirname, '..', 'seed', 'data', 'germanic-locales-research.json'),
  path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'asian-locales-cultural-research.json'),
  path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'locale_cultural_data_other.json'),
  path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'romance-locales-cultural-data.json'),
];

const OUTPUT_CULTURE = path.join(__dirname, '..', 'seed', '25.4-research-culture-refs.cypher');
const OUTPUT_TABOOS = path.join(__dirname, '..', 'seed', '25.5-research-taboos.cypher');
const OUTPUT_AUDIENCE = path.join(__dirname, '..', 'seed', '25.6-research-audience.cypher');

const stats = { cultureRefs: 0, taboos: 0, audienceTraits: 0, locales: 0 };

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

function processResearchFile(filePath) {
  const cultureLines = [];
  const tabooLines = [];
  const audienceLines = [];

  if (!fs.existsSync(filePath)) {
    console.warn(`  ⚠️ File not found: ${filePath}`);
    return { cultureLines, tabooLines, audienceLines };
  }

  const content = JSON.parse(fs.readFileSync(filePath, 'utf-8'));

  // Handle both array and object formats
  let localesArray = [];
  if (Array.isArray(content.locales)) {
    localesArray = content.locales;
  } else if (typeof content.locales === 'object' && content.locales !== null) {
    localesArray = Object.entries(content.locales).map(([localeCode, data]) => ({
      locale: localeCode,
      ...data
    }));
  }

  console.log(`  📄 Processing ${path.basename(filePath)}: ${localesArray.length} locales`);

  for (const localeData of localesArray) {
    const locale = localeData.locale;
    if (!locale) continue;

    stats.locales++;

    // Process CultureRefs
    if (localeData.culture_refs && localeData.culture_refs.length > 0) {
      cultureLines.push(`// --- ${locale} CultureRefs from Research ---`);

      for (const ref of localeData.culture_refs) {
        const name = ref.name || ref.reference || ref.topic || `culture-${stats.cultureRefs}`;
        const key = `culture-ref:research:${slugify(name)}@${locale}`;
        const description = escapeCypher(ref.description || ref.context || name);
        const category = ref.category || 'general';
        const usage = escapeCypher(ref.usage || ref.usage_context || '');

        cultureLines.push(`MERGE (c:CultureRef {key: '${key}'})`);
        cultureLines.push(`SET c.display_name = '${escapeCypher(name)}',`);
        cultureLines.push(`    c.locale = '${locale}',`);
        cultureLines.push(`    c.description = '${description}',`);
        cultureLines.push(`    c.category = '${category}',`);
        cultureLines.push(`    c.usage_context = '${usage}',`);
        cultureLines.push(`    c.provenance = 'perplexity-research',`);
        cultureLines.push(`    c.confidence = 0.85,`);
        cultureLines.push(`    c.created_at = datetime(),`);
        cultureLines.push(`    c.updated_at = datetime();`);
        cultureLines.push('');

        // Link to CultureSet
        cultureLines.push(`MATCH (cs:CultureSet {key: 'culture-set:values@${locale}'})`);
        cultureLines.push(`MATCH (c:CultureRef {key: '${key}'})`);
        cultureLines.push(`MERGE (cs)-[:CONTAINS_CULTURE_REF]->(c);`);
        cultureLines.push('');

        stats.cultureRefs++;
      }
    }

    // Process Taboos
    if (localeData.taboos && localeData.taboos.length > 0) {
      tabooLines.push(`// --- ${locale} Taboos from Research ---`);

      for (const taboo of localeData.taboos) {
        const name = taboo.name || taboo.topic || taboo.avoid || `taboo-${stats.taboos}`;
        const key = `taboo:research:${slugify(name)}@${locale}`;
        const description = escapeCypher(taboo.description || taboo.reason || name);
        const severity = taboo.severity || 'moderate';
        const context = escapeCypher(taboo.context || '');

        tabooLines.push(`MERGE (t:Taboo {key: '${key}'})`);
        tabooLines.push(`SET t.display_name = '${escapeCypher(name)}',`);
        tabooLines.push(`    t.locale = '${locale}',`);
        tabooLines.push(`    t.description = '${description}',`);
        tabooLines.push(`    t.severity = '${severity}',`);
        tabooLines.push(`    t.context = '${context}',`);
        tabooLines.push(`    t.provenance = 'perplexity-research',`);
        tabooLines.push(`    t.confidence = 0.85,`);
        tabooLines.push(`    t.created_at = datetime(),`);
        tabooLines.push(`    t.updated_at = datetime();`);
        tabooLines.push('');

        // Link to TabooSet
        tabooLines.push(`MATCH (ts:TabooSet {key: 'taboo-set:avoid@${locale}'})`);
        tabooLines.push(`MATCH (t:Taboo {key: '${key}'})`);
        tabooLines.push(`MERGE (ts)-[:CONTAINS_TABOO]->(t);`);
        tabooLines.push('');

        stats.taboos++;
      }
    }

    // Process AudienceTraits (handles both array and object format)
    if (localeData.audience_traits) {
      let traitsArray = [];

      if (Array.isArray(localeData.audience_traits)) {
        traitsArray = localeData.audience_traits;
      } else if (typeof localeData.audience_traits === 'object') {
        // Convert object format { "formality": "High", ... } to array
        traitsArray = Object.entries(localeData.audience_traits).map(([traitName, value]) => ({
          name: traitName.replace(/_/g, ' '),
          description: typeof value === 'string' ? value : JSON.stringify(value),
          category: 'communication'
        }));
      }

      if (traitsArray.length > 0) {
        audienceLines.push(`// --- ${locale} AudienceTraits from Research ---`);

        for (const trait of traitsArray) {
          const name = trait.name || trait.trait || trait.characteristic || `trait-${stats.audienceTraits}`;
          const key = `audience-trait:research:${slugify(name)}@${locale}`;
          const description = escapeCypher(trait.description || trait.impact || name);
          const category = trait.category || 'general';

          audienceLines.push(`MERGE (a:AudienceTrait {key: '${key}'})`);
          audienceLines.push(`SET a.display_name = '${escapeCypher(name)}',`);
          audienceLines.push(`    a.locale = '${locale}',`);
          audienceLines.push(`    a.description = '${description}',`);
          audienceLines.push(`    a.category = '${category}',`);
          audienceLines.push(`    a.provenance = 'perplexity-research',`);
          audienceLines.push(`    a.confidence = 0.85,`);
          audienceLines.push(`    a.created_at = datetime(),`);
          audienceLines.push(`    a.updated_at = datetime();`);
          audienceLines.push('');

          // Link to AudienceSet
          audienceLines.push(`MATCH (as:AudienceSet {key: 'audience-set:general@${locale}'})`);
          audienceLines.push(`MATCH (a:AudienceTrait {key: '${key}'})`);
          audienceLines.push(`MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(a);`);
          audienceLines.push('');

          stats.audienceTraits++;
        }
      }
    }
  }

  return { cultureLines, tabooLines, audienceLines };
}

async function main() {
  console.log('🔄 Enriching Knowledge Atoms from research files...\n');

  const timestamp = new Date().toISOString();

  const cultureHeader = [
    '// ============================================================================',
    '// Seed 25.4: CultureRefs from Perplexity Research',
    '// ============================================================================',
    '// Generated by: enrich-atoms-from-research.js',
    `// Generated at: ${timestamp}`,
    '// ============================================================================',
    '',
  ];

  const tabooHeader = [
    '// ============================================================================',
    '// Seed 25.5: Taboos from Perplexity Research',
    '// ============================================================================',
    '// Generated by: enrich-atoms-from-research.js',
    `// Generated at: ${timestamp}`,
    '// ============================================================================',
    '',
  ];

  const audienceHeader = [
    '// ============================================================================',
    '// Seed 25.6: AudienceTraits from Perplexity Research',
    '// ============================================================================',
    '// Generated by: enrich-atoms-from-research.js',
    `// Generated at: ${timestamp}`,
    '// ============================================================================',
    '',
  ];

  const allCulture = [...cultureHeader];
  const allTaboos = [...tabooHeader];
  const allAudience = [...audienceHeader];

  for (const file of RESEARCH_FILES) {
    const { cultureLines, tabooLines, audienceLines } = processResearchFile(file);
    allCulture.push(...cultureLines);
    allTaboos.push(...tabooLines);
    allAudience.push(...audienceLines);
  }

  fs.writeFileSync(OUTPUT_CULTURE, allCulture.join('\n'));
  fs.writeFileSync(OUTPUT_TABOOS, allTaboos.join('\n'));
  fs.writeFileSync(OUTPUT_AUDIENCE, allAudience.join('\n'));

  console.log(`
═══════════════════════════════════════════════════════════════════
📊 RESEARCH ENRICHMENT COMPLETE
═══════════════════════════════════════════════════════════════════
   Locales processed:    ${stats.locales}
   CultureRefs added:    ${stats.cultureRefs}
   Taboos added:         ${stats.taboos}
   AudienceTraits added: ${stats.audienceTraits}

📁 Output files:
   • packages/db/seed/25.4-research-culture-refs.cypher
   • packages/db/seed/25.5-research-taboos.cypher
   • packages/db/seed/25.6-research-audience.cypher
`);
}

main().catch(console.error);
