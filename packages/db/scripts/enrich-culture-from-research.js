#!/usr/bin/env node
/**
 * Enrich CultureRefs and Taboos from detailed research files
 * Task B: Adds detailed data from Perplexity research
 */

const fs = require('fs');
const path = require('path');

const RESEARCH_FILES = [
  { file: path.join(__dirname, '..', 'seed', 'data', 'germanic-locales-research.json'), name: 'germanic' },
  { file: path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'asian-locales-cultural-research.json'), name: 'asian' },
  { file: path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'locale_cultural_data_other.json'), name: 'other' },
  { file: path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'romance-locales-cultural-data.json'), name: 'romance' },
];

function escapeCypher(text) {
  if (!text) return '';
  return text
    .replace(/\\/g, '\\\\')  // Escape backslashes FIRST
    .replace(/'/g, "\\'")     // Then escape single quotes
    .replace(/\n/g, ' ')
    .substring(0, 500);
}

function generateKey(text, locale) {
  const slug = text
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-|-$/g, '')
    .substring(0, 40);
  return `${slug}@${locale}`;
}

console.log('🔄 Enriching CultureRefs and Taboos from research files...\n');

let cultureRefs = [];
let taboos = [];
let existingKeys = new Set();

// Process each research file
for (const { file, name } of RESEARCH_FILES) {
  if (!fs.existsSync(file)) {
    console.log(`  ⚠️  Skipping ${name}: file not found`);
    continue;
  }

  const data = JSON.parse(fs.readFileSync(file, 'utf8'));
  // Handle both array and object formats
  let locales = data.locales || [];
  if (!Array.isArray(locales)) {
    locales = Object.values(locales);
  }
  console.log(`  📄 Processing ${name}: ${locales.length} locales`);

  for (const loc of locales) {
    const locale = loc.locale;

    // Process culture_refs with full context
    if (loc.culture_refs) {
      for (const ref of loc.culture_refs) {
        const key = generateKey(ref.value, locale);
        if (existingKeys.has(key)) continue;
        existingKeys.add(key);

        cultureRefs.push({
          key,
          locale,
          text: ref.value,
          importance: ref.importance || 'medium',
          expression: ref.expression || '',
          marketing_angle: ref.marketing_angle || '',
        });
      }
    }

    // Process taboos with full context
    if (loc.taboos) {
      for (const taboo of loc.taboos) {
        const key = generateKey(taboo.topic || taboo.value, locale);
        if (existingKeys.has(key)) continue;
        existingKeys.add(key);

        taboos.push({
          key,
          locale,
          text: taboo.topic || taboo.value,
          severity: taboo.severity || 'medium',
          reason: taboo.reason || taboo.explanation || '',
          alternative: taboo.alternative || '',
        });
      }
    }
  }
}

// Generate Cypher for enriched CultureRefs
let cypher = `// ============================================================
// 25.4 - Enriched CultureRefs from Research (Task B)
// Generated: ${new Date().toISOString()}
// ============================================================

`;

cypher += `// --- CultureRefs with full context ---\n`;
for (const ref of cultureRefs) {
  cypher += `MERGE (cr:CultureRef {key: '${escapeCypher(ref.key)}'})
SET cr.locale = '${ref.locale}',
    cr.text = '${escapeCypher(ref.text)}',
    cr.importance = '${ref.importance}',
    cr.expression = '${escapeCypher(ref.expression)}',
    cr.marketing_angle = '${escapeCypher(ref.marketing_angle)}',
    cr.display_name = '${escapeCypher(ref.text)}',
    cr.description = '${escapeCypher(ref.expression)}',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@${ref.locale}'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

`;
}

cypher += `\n// --- Taboos with full context ---\n`;
for (const taboo of taboos) {
  cypher += `MERGE (t:Taboo {key: '${escapeCypher(taboo.key)}'})
SET t.locale = '${taboo.locale}',
    t.text = '${escapeCypher(taboo.text)}',
    t.severity = '${taboo.severity}',
    t.reason = '${escapeCypher(taboo.reason)}',
    t.alternative = '${escapeCypher(taboo.alternative)}',
    t.display_name = '${escapeCypher(taboo.text)}',
    t.description = '${escapeCypher(taboo.reason)}',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@${taboo.locale}'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

`;
}

// Write output
const outputPath = path.join(__dirname, '..', 'seed', '25.4-enriched-culture.cypher');
fs.writeFileSync(outputPath, cypher);

console.log(`
═══════════════════════════════════════════════════════════════════
📊 CULTURE ENRICHMENT COMPLETE
═══════════════════════════════════════════════════════════════════
   CultureRefs enriched:  ${cultureRefs.length}
   Taboos enriched:       ${taboos.length}
   Total atoms:           ${cultureRefs.length + taboos.length}

📁 Output: packages/db/seed/25.4-enriched-culture.cypher
`);
