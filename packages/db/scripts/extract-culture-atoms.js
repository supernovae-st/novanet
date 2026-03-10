#!/usr/bin/env node
/**
 * Extract Culture Atoms from 24-culture.cypher
 *
 * This script parses the Culture seed file and generates:
 * - 25-culture-refs.cypher (CultureRef nodes)
 * - 25.1-taboos.cypher (Taboo nodes)
 * - 25.2-audience-traits.cypher (AudienceTrait nodes)
 *
 * Usage: node extract-culture-atoms.js
 */

const fs = require('fs');
const path = require('path');

const SEED_DIR = path.join(__dirname, '..');
const INPUT_FILE = path.join(SEED_DIR, 'seed', '24-culture.cypher');
const OUTPUT_DIR = path.join(SEED_DIR, 'seed');

// Regex patterns to extract data from Cypher
const LOCALE_PATTERN = /MERGE \(c:Culture \{key: '([a-z]{2,3}-[A-Z]{2})'\}\)/g;
const VALUES_PATTERN = /c\.values = '(\[.*?\])'/gs;
const TABOOS_PATTERN = /c\.taboos_summary = '([^']+)'/g;
const COMM_NORMS_PATTERN = /c\.communication_norms = '(\{[^']+\})'/g;

// Track statistics
const stats = {
  locales: 0,
  cultureRefs: 0,
  taboos: 0,
  audienceTraits: 0
};

/**
 * Parse the Culture seed file and extract all cultures
 */
function parseCultureSeed(content) {
  const cultures = [];

  // Split by MERGE statements to get individual culture blocks
  const blocks = content.split(/MERGE \(c:Culture/);

  for (const block of blocks) {
    if (!block.trim()) continue;

    // Extract locale key
    const keyMatch = block.match(/\{key: '([a-z]{2,3}-[A-Z]{2})'\}/);
    if (!keyMatch) continue;

    const locale = keyMatch[1];
    const culture = { locale };

    // Extract values (JSON array of cultural values)
    const valuesMatch = block.match(/c\.values = '([\s\S]*?)(?:',\s*c\.|';)/);
    if (valuesMatch) {
      try {
        // Clean up the JSON string - handle escaped quotes
        let valuesStr = valuesMatch[1];
        // Fix common JSON issues
        valuesStr = valuesStr.replace(/\\'/g, "'");
        culture.values = JSON.parse(valuesStr);
      } catch (e) {
        // Try alternative parsing
        try {
          let valuesStr = valuesMatch[1];
          // Some values have unescaped single quotes inside, try to fix
          valuesStr = valuesStr.replace(/'/g, '"').replace(/\\""/g, "'");
          culture.values = JSON.parse(valuesStr);
        } catch (e2) {
          console.warn(`Warning: Could not parse values for ${locale}: ${e2.message}`);
          culture.values = [];
        }
      }
    } else {
      culture.values = [];
    }

    // Extract taboos summary
    const taboosMatch = block.match(/c\.taboos_summary = '([^']+)'/);
    if (taboosMatch) {
      culture.taboos_summary = taboosMatch[1];
    }

    // Extract communication norms
    const commNormsMatch = block.match(/c\.communication_norms = '(\{[^']+\})'/);
    if (commNormsMatch) {
      try {
        culture.communication_norms = JSON.parse(commNormsMatch[1]);
      } catch (e) {
        console.warn(`Warning: Could not parse communication_norms for ${locale}`);
        culture.communication_norms = {};
      }
    } else {
      culture.communication_norms = {};
    }

    cultures.push(culture);
  }

  return cultures;
}

/**
 * Generate CultureRef Cypher seed
 */
function generateCultureRefsSeed(cultures) {
  const lines = [];

  lines.push('// ============================================================================');
  lines.push('// CULTURE REFS SEED - Extracted from Culture.values');
  lines.push(`// Generated: ${new Date().toISOString()}`);
  lines.push(`// Source: 24-culture.cypher`);
  lines.push('// ============================================================================');
  lines.push('');
  lines.push('// Note: Each CultureRef represents a core cultural value for content generation');
  lines.push('// These are used by novanet_generate to provide locale-specific context');
  lines.push('');

  for (const culture of cultures) {
    if (!culture.values || culture.values.length === 0) continue;

    lines.push(`// ----------------------------------------------------------------------------`);
    lines.push(`// ${culture.locale} CultureRefs`);
    lines.push(`// ----------------------------------------------------------------------------`);
    lines.push('');

    // First ensure CultureSet exists and is linked
    // Note: CultureSet keys use format 'culture-set:values@locale'
    lines.push(`MATCH (l:Locale {key: '${culture.locale}'})`);
    lines.push(`MATCH (cs:CultureSet {key: 'culture-set:values@${culture.locale}'})`);
    lines.push(`MERGE (l)-[:HAS_CULTURE]->(cs);`);
    lines.push('');

    // Filter out "Pride Point" entries which are section headers, not real values
    const realValues = culture.values.filter(v =>
      v.value &&
      v.importance !== 'Sensitivity' &&
      !v.value.includes('Pride Point')
    );

    for (let i = 0; i < realValues.length; i++) {
      const value = realValues[i];
      const key = slugify(value.value);
      const fullKey = `${key}@${culture.locale}`;

      // Escape single quotes in strings
      const displayName = escapeCypher(value.value);
      const importance = value.importance?.toLowerCase() || 'medium';
      const expression = escapeCypher(value.expression || '');
      const marketingAngle = escapeCypher(value.marketing_angle || '');

      lines.push(`MERGE (cr:CultureRef {key: '${fullKey}'})`);
      lines.push(`SET cr.display_name = '${displayName}',`);
      lines.push(`    cr.locale = '${culture.locale}',`);
      lines.push(`    cr.importance = '${importance}',`);
      lines.push(`    cr.expression = '${expression}',`);
      lines.push(`    cr.marketing_angle = '${marketingAngle}',`);
      lines.push(`    cr.llm_context = 'USE: when generating content for ${culture.locale}. VALUE: ${displayName} (${importance} importance). EXPRESSION: ${expression.substring(0, 100)}...',`);
      lines.push(`    cr.provenance = 'ath-know-l10n',`);
      lines.push(`    cr.confidence = 0.9,`);
      lines.push(`    cr.created_at = datetime(),`);
      lines.push(`    cr.updated_at = datetime();`);
      lines.push('');

      // Link to CultureSet (key format: culture-set:values@locale)
      lines.push(`MATCH (cs:CultureSet {key: 'culture-set:values@${culture.locale}'})`);
      lines.push(`MATCH (cr:CultureRef {key: '${fullKey}'})`);
      lines.push(`MERGE (cs)-[:CONTAINS_CULTURE_REF]->(cr);`);
      lines.push('');

      stats.cultureRefs++;
    }
  }

  return lines.join('\n');
}

/**
 * Generate Taboo Cypher seed
 */
function generateTaboosSeed(cultures) {
  const lines = [];

  lines.push('// ============================================================================');
  lines.push('// TABOOS SEED - Extracted from Culture.taboos_summary');
  lines.push(`// Generated: ${new Date().toISOString()}`);
  lines.push(`// Source: 24-culture.cypher`);
  lines.push('// ============================================================================');
  lines.push('');
  lines.push('// Note: Each Taboo represents a topic to avoid in content generation');
  lines.push('// These are CRITICAL for avoiding cultural/legal issues');
  lines.push('');

  for (const culture of cultures) {
    if (!culture.taboos_summary) continue;

    lines.push(`// ----------------------------------------------------------------------------`);
    lines.push(`// ${culture.locale} Taboos`);
    lines.push(`// ----------------------------------------------------------------------------`);
    lines.push('');

    // First ensure TabooSet exists and is linked
    lines.push(`MATCH (l:Locale {key: '${culture.locale}'})`);
    lines.push(`MATCH (ts:TabooSet {key: 'taboo-set:avoid@${culture.locale}'})`);
    lines.push(`MERGE (l)-[:HAS_TABOOS]->(ts);`);
    lines.push('');

    // Parse taboos from the summary
    // Format: "CRITICAL TABOOS: X, Y, Z. Always avoid these topics."
    const taboos = parseTaboosSummary(culture.taboos_summary);

    for (let i = 0; i < taboos.length; i++) {
      const taboo = taboos[i];
      const key = slugify(taboo);
      const fullKey = `${key}@${culture.locale}`;

      const displayName = escapeCypher(taboo);

      lines.push(`MERGE (t:Taboo {key: '${fullKey}'})`);
      lines.push(`SET t.display_name = '${displayName}',`);
      lines.push(`    t.locale = '${culture.locale}',`);
      lines.push(`    t.term = '${displayName}',`);
      lines.push(`    t.type = 'topic',`);
      lines.push(`    t.severity = 'critical',`);
      lines.push(`    t.category = 'cultural',`);
      lines.push(`    t.reason = 'Extracted from locale culture norms - critical taboo to avoid',`);
      lines.push(`    t.alternatives = [],`);
      lines.push(`    t.llm_context = 'AVOID: ${displayName} in ${culture.locale} content. This is a CRITICAL taboo that can cause serious issues.',`);
      lines.push(`    t.provenance = 'ath-know-l10n',`);
      lines.push(`    t.confidence = 0.95,`);
      lines.push(`    t.created_at = datetime(),`);
      lines.push(`    t.updated_at = datetime();`);
      lines.push('');

      // Link to TabooSet
      lines.push(`MATCH (ts:TabooSet {key: 'taboo-set:avoid@${culture.locale}'})`);
      lines.push(`MATCH (t:Taboo {key: '${fullKey}'})`);
      lines.push(`MERGE (ts)-[:CONTAINS_TABOO]->(t);`);
      lines.push('');

      stats.taboos++;
    }
  }

  return lines.join('\n');
}

/**
 * Generate AudienceTrait Cypher seed
 */
function generateAudienceTraitsSeed(cultures) {
  const lines = [];

  lines.push('// ============================================================================');
  lines.push('// AUDIENCE TRAITS SEED - Extracted from Culture.communication_norms');
  lines.push(`// Generated: ${new Date().toISOString()}`);
  lines.push(`// Source: 24-culture.cypher`);
  lines.push('// ============================================================================');
  lines.push('');
  lines.push('// Note: Each AudienceTrait describes communication style for a locale');
  lines.push('// These inform tone, formality, and content structure');
  lines.push('');

  for (const culture of cultures) {
    if (!culture.communication_norms || Object.keys(culture.communication_norms).length === 0) continue;

    lines.push(`// ----------------------------------------------------------------------------`);
    lines.push(`// ${culture.locale} AudienceTrait`);
    lines.push(`// ----------------------------------------------------------------------------`);
    lines.push('');

    // First ensure AudienceSet exists and is linked
    lines.push(`MATCH (l:Locale {key: '${culture.locale}'})`);
    lines.push(`MATCH (as:AudienceSet {key: 'audience-set:general@${culture.locale}'})`);
    lines.push(`MERGE (l)-[:HAS_AUDIENCE]->(as);`);
    lines.push('');

    const norms = culture.communication_norms;
    const key = `communication-style@${culture.locale}`;

    // Map context type to audience segment (approximation)
    const contextType = norms.context_type || 'MODERATE_CONTEXT';
    const directness = (norms.directness || 'balanced').toLowerCase();
    const hierarchy = (norms.hierarchy_sensitivity || 'medium').toLowerCase();

    // Determine appropriate formality based on context
    let formality = 'moderate';
    if (hierarchy === 'high') formality = 'formal';
    else if (hierarchy === 'low') formality = 'informal';

    // Build JSON strings - keep double quotes (safe in single-quoted Cypher strings)
    const behaviorJson = JSON.stringify({
      context_type: contextType,
      directness: directness,
      hierarchy_sensitivity: hierarchy
    });

    const preferencesJson = JSON.stringify({
      formality: formality,
      communication_style: directness
    });

    const contentLength = contextType === 'HIGH_CONTEXT' ? 'moderate' : 'detailed';

    lines.push(`MERGE (at:AudienceTrait {key: '${key}'})`);
    lines.push(`SET at.display_name = 'Communication Style for ${culture.locale}',`);
    lines.push(`    at.locale = '${culture.locale}',`);
    lines.push(`    at.segment = 'general',`);
    lines.push(`    at.behavior = '${behaviorJson}',`);
    lines.push(`    at.preferences = '${preferencesJson}',`);
    lines.push(`    at.content_length = '${contentLength}',`);
    lines.push(`    at.llm_context = 'USE: when generating content for ${culture.locale}. STYLE: ${directness} communication, ${hierarchy} hierarchy importance, ${contextType}.',`);
    lines.push(`    at.provenance = 'ath-know-l10n',`);
    lines.push(`    at.confidence = 0.9,`);
    lines.push(`    at.created_at = datetime(),`);
    lines.push(`    at.updated_at = datetime();`);
    lines.push('');

    // Link to AudienceSet
    lines.push(`MATCH (as:AudienceSet {key: 'audience-set:general@${culture.locale}'})`);
    lines.push(`MATCH (at:AudienceTrait {key: '${key}'})`);
    lines.push(`MERGE (as)-[:CONTAINS_AUDIENCE_TRAIT]->(at);`);
    lines.push('');

    stats.audienceTraits++;
  }

  return lines.join('\n');
}

/**
 * Helper: Parse taboos from summary string
 */
function parseTaboosSummary(summary) {
  // Format: "CRITICAL TABOOS: X, Y, Z. Always avoid these topics."
  const match = summary.match(/CRITICAL TABOOS:\s*(.+?)\.?\s*Always/i);
  if (!match) {
    // Try alternative format
    const altMatch = summary.match(/CRITICAL TABOOS:\s*(.+)/i);
    if (!altMatch) return [];
    return altMatch[1].split(/,\s*/).map(t => t.trim()).filter(t => t);
  }
  return match[1].split(/,\s*/).map(t => t.trim()).filter(t => t);
}

/**
 * Helper: Create URL-safe slug from text
 */
function slugify(text) {
  if (!text) return 'unknown';
  return text
    .toLowerCase()
    .replace(/[^\w\s-]/g, '') // Remove non-word chars (keeps accented chars)
    .replace(/[\s_]+/g, '-')   // Replace spaces/underscores with hyphens
    .replace(/-+/g, '-')       // Replace multiple hyphens with single
    .replace(/^-|-$/g, '')     // Trim hyphens from ends
    .substring(0, 50);         // Limit length
}

/**
 * Helper: Escape single quotes for Cypher strings
 */
function escapeCypher(text) {
  if (!text) return '';
  return text
    .replace(/\\/g, '\\\\')  // Escape backslashes FIRST
    .replace(/'/g, "\\'")     // Then escape single quotes
    .replace(/\n/g, ' ')
    .substring(0, 500); // Limit length to avoid huge strings
}

// Main execution
async function main() {
  console.log('🔄 Reading Culture seed file...');

  const content = fs.readFileSync(INPUT_FILE, 'utf-8');
  console.log(`📄 Read ${Math.round(content.length / 1024)}KB`);

  console.log('🔍 Parsing cultures...');
  const cultures = parseCultureSeed(content);
  stats.locales = cultures.length;
  console.log(`✅ Found ${cultures.length} locales`);

  // Generate CultureRefs
  console.log('📝 Generating CultureRefs seed...');
  const cultureRefsSeed = generateCultureRefsSeed(cultures);
  fs.writeFileSync(path.join(OUTPUT_DIR, '25-culture-refs.cypher'), cultureRefsSeed);
  console.log(`✅ Created 25-culture-refs.cypher (${stats.cultureRefs} CultureRefs)`);

  // Generate Taboos
  console.log('📝 Generating Taboos seed...');
  const taboosSeed = generateTaboosSeed(cultures);
  fs.writeFileSync(path.join(OUTPUT_DIR, '25.1-taboos.cypher'), taboosSeed);
  console.log(`✅ Created 25.1-taboos.cypher (${stats.taboos} Taboos)`);

  // Generate AudienceTraits
  console.log('📝 Generating AudienceTraits seed...');
  const audienceTraitsSeed = generateAudienceTraitsSeed(cultures);
  fs.writeFileSync(path.join(OUTPUT_DIR, '25.2-audience-traits.cypher'), audienceTraitsSeed);
  console.log(`✅ Created 25.2-audience-traits.cypher (${stats.audienceTraits} AudienceTraits)`);

  // Summary
  console.log('');
  console.log('═══════════════════════════════════════════════════════════════════');
  console.log('📊 EXTRACTION COMPLETE');
  console.log('═══════════════════════════════════════════════════════════════════');
  console.log(`   Locales processed:    ${stats.locales}`);
  console.log(`   CultureRefs created:  ${stats.cultureRefs}`);
  console.log(`   Taboos created:       ${stats.taboos}`);
  console.log(`   AudienceTraits:       ${stats.audienceTraits}`);
  console.log('');
  console.log('📁 Output files:');
  console.log('   - packages/db/seed/25-culture-refs.cypher');
  console.log('   - packages/db/seed/25.1-taboos.cypher');
  console.log('   - packages/db/seed/25.2-audience-traits.cypher');
  console.log('');
  console.log('🚀 Next: Run `pnpm infra:seed` to load into Neo4j');
}

main().catch(err => {
  console.error('❌ Error:', err);
  process.exit(1);
});
