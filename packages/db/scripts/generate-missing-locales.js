#!/usr/bin/env node
/**
 * Generate seed file for missing locales: fil-PH, lo-LA, or-IN
 * Task C: Create complete knowledge atoms for orphan locales
 */

const fs = require('fs');
const path = require('path');

const INPUT_FILE = path.join(__dirname, '..', '..', '..', 'tools', 'novanet', 'missing-locales-cultural-data.json');

function escapeCypher(text) {
  if (!text) return '';
  return text
    .replace(/\\/g, '\\\\')
    .replace(/'/g, "\\'")
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

console.log('🔄 Generating seed file for missing locales...\n');

const data = JSON.parse(fs.readFileSync(INPUT_FILE, 'utf8'));
const locales = data.locales || [];

let cypher = `// ============================================================
// 25.5 - Missing Locales: fil-PH, lo-LA, or-IN (Task C)
// Generated: ${new Date().toISOString()}
// ============================================================

`;

let stats = {
  cultureRefs: 0,
  taboos: 0,
  audienceTraits: 0,
  expressions: 0,
  patterns: 0
};

for (const loc of locales) {
  const locale = loc.locale;
  console.log(`  📄 Processing ${locale}: ${loc.language}`);

  cypher += `// === ${locale} - ${loc.language} (${loc.region}) ===\n\n`;

  // CultureRefs
  if (loc.culture_refs) {
    for (const ref of loc.culture_refs) {
      const key = generateKey(ref.value, locale);
      cypher += `MERGE (cr:CultureRef {key: '${escapeCypher(key)}'})
SET cr.locale = '${locale}',
    cr.text = '${escapeCypher(ref.value)}',
    cr.importance = '${ref.importance}',
    cr.expression = '${escapeCypher(ref.expression)}',
    cr.marketing_angle = '${escapeCypher(ref.marketing_angle)}',
    cr.display_name = '${escapeCypher(ref.value)}',
    cr.description = '${escapeCypher(ref.expression)}',
    cr.created_at = datetime(),
    cr.updated_at = datetime()
WITH cr
MATCH (cs:CultureSet {key: 'culture-set:values@${locale}'})
MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);

`;
      stats.cultureRefs++;
    }
  }

  // Taboos
  if (loc.taboos) {
    for (const taboo of loc.taboos) {
      const key = generateKey(taboo.topic, locale);
      cypher += `MERGE (t:Taboo {key: '${escapeCypher(key)}'})
SET t.locale = '${locale}',
    t.text = '${escapeCypher(taboo.topic)}',
    t.severity = '${taboo.severity}',
    t.reason = '${escapeCypher(taboo.reason)}',
    t.alternative = '${escapeCypher(taboo.alternative)}',
    t.display_name = '${escapeCypher(taboo.topic)}',
    t.description = '${escapeCypher(taboo.reason)}',
    t.created_at = datetime(),
    t.updated_at = datetime()
WITH t
MATCH (ts:TabooSet {key: 'taboo-set:avoid@${locale}'})
MERGE (ts)-[:CONTAINS_TABOO]->(t);

`;
      stats.taboos++;
    }
  }

  // AudienceTraits
  if (loc.audience_traits) {
    for (const trait of loc.audience_traits) {
      const key = generateKey(trait.trait, locale);
      cypher += `MERGE (at:AudienceTrait {key: '${escapeCypher(key)}'})
SET at.locale = '${locale}',
    at.text = '${escapeCypher(trait.trait)}',
    at.description = '${escapeCypher(trait.description)}',
    at.display_name = '${escapeCypher(trait.trait)}',
    at.created_at = datetime(),
    at.updated_at = datetime()
WITH at
MATCH (as:AudienceSet {key: 'audience-set:general@${locale}'})
MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);

`;
      stats.audienceTraits++;
    }
  }

  // Expressions
  if (loc.expressions) {
    for (const expr of loc.expressions) {
      const key = generateKey(expr.text, locale);
      cypher += `MERGE (e:Expression {key: '${escapeCypher(key)}'})
SET e.locale = '${locale}',
    e.text = '${escapeCypher(expr.text)}',
    e.register = '${expr.register}',
    e.context = '${escapeCypher(expr.context)}',
    e.display_name = '${escapeCypher(expr.text)}',
    e.description = '${escapeCypher(expr.context)}',
    e.created_at = datetime(),
    e.updated_at = datetime()
WITH e
MATCH (es:ExpressionSet {key: 'expression-set:greetings@${locale}'})
MERGE (es)-[:CONTAINS_EXPRESSION]->(e);

`;
      stats.expressions++;
    }
  }

  // Patterns
  if (loc.patterns) {
    for (const pattern of loc.patterns) {
      const key = generateKey(pattern.type + '-' + pattern.template.substring(0, 20), locale);
      cypher += `MERGE (p:Pattern {key: '${escapeCypher(key)}'})
SET p.locale = '${locale}',
    p.text = '${escapeCypher(pattern.template)}',
    p.type = '${pattern.type}',
    p.translation = '${escapeCypher(pattern.translation)}',
    p.display_name = '${escapeCypher(pattern.type)} pattern',
    p.description = '${escapeCypher(pattern.translation)}',
    p.created_at = datetime(),
    p.updated_at = datetime()
WITH p
MATCH (ps:PatternSet {key: 'pattern-set:cta@${locale}'})
MERGE (ps)-[:CONTAINS_PATTERN]->(p);

`;
      stats.patterns++;
    }
  }
}

// Write output
const outputPath = path.join(__dirname, '..', 'seed', '25.5-missing-locales.cypher');
fs.writeFileSync(outputPath, cypher);

console.log(`
═══════════════════════════════════════════════════════════════════
📊 MISSING LOCALES GENERATION COMPLETE
═══════════════════════════════════════════════════════════════════
   CultureRefs:     ${stats.cultureRefs}
   Taboos:          ${stats.taboos}
   AudienceTraits:  ${stats.audienceTraits}
   Expressions:     ${stats.expressions}
   Patterns:        ${stats.patterns}
   ─────────────────────────────
   Total atoms:     ${stats.cultureRefs + stats.taboos + stats.audienceTraits + stats.expressions + stats.patterns}

📁 Output: packages/db/seed/25.5-missing-locales.cypher
`);
