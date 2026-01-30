// novanet-core/scripts/import-locale-knowledge.ts
/**
 * Import Locale Knowledge data from ath-know-l10n MD files into Neo4j
 *
 * Creates the following nodes for each locale:
 * - LocaleIdentity (linked via HAS_IDENTITY)
 * - LocaleVoice (linked via HAS_VOICE)
 * - LocaleCulture (linked via HAS_CULTURE)
 * - LocaleMarket (linked via HAS_MARKET)
 * - LocaleLexicon (linked via HAS_LEXICON)
 * - Expression nodes (linked via HAS_EXPRESSION from LocaleLexicon)
 *
 * Usage:
 *   npm run import:locale-knowledge                    # Import all locales
 *   npm run import:locale-knowledge -- --locale=fr-FR  # Import single locale
 */

import { readdirSync, readFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import neo4j, { Driver, Session } from 'neo4j-driver';

// Parsers
import { parseIdentityMd } from '../src/parsers/parse-identity.js';
import { parseVoiceMd } from '../src/parsers/parse-voice.js';
import { parseCultureMd } from '../src/parsers/parse-culture.js';
import { parseMarketMd } from '../src/parsers/parse-market.js';
import { parseLexiconMd } from '../src/parsers/parse-lexicon.js';

// Types are imported from parsers' return types - no explicit import needed

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Find project root by looking for package.json (works from both scripts/ and dist/scripts/)
function findProjectRoot(startDir: string): string {
  let dir = startDir;
  while (!existsSync(join(dir, 'package.json'))) {
    const parent = dirname(dir);
    if (parent === dir) throw new Error('Could not find project root (package.json)');
    dir = parent;
  }
  return dir;
}

const PROJECT_ROOT = findProjectRoot(__dirname);
const DATA_PATH = process.env.DATA_PATH || join(PROJECT_ROOT, '../ath-know-l10n/outputs/localization-data');

// Neo4j configuration
const NEO4J_URI = process.env.NEO4J_URI || 'bolt://localhost:7687';
const NEO4J_USER = process.env.NEO4J_USER || 'neo4j';
const NEO4J_PASSWORD = process.env.NEO4J_PASSWORD || 'novanetpassword';

/**
 * Safely read and parse a file
 */
function parseFile<T>(path: string, parser: (content: string) => T): T | null {
  try {
    if (!existsSync(path)) {
      console.log(`  [SKIP] File not found: ${path}`);
      return null;
    }
    const content = readFileSync(path, 'utf-8');
    return parser(content);
  } catch (error) {
    console.error(`  [ERROR] Failed to parse ${path}:`, error);
    return null;
  }
}

/**
 * Serialize complex objects/arrays to JSON strings for Neo4j storage
 */
function serializeForNeo4j(obj: Record<string, unknown>): Record<string, unknown> {
  const result: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(obj)) {
    if (value === undefined || value === null) {
      continue;
    }
    if (Array.isArray(value) || (typeof value === 'object' && value !== null)) {
      result[key] = JSON.stringify(value);
    } else {
      result[key] = value;
    }
  }
  return result;
}

/**
 * Import locale knowledge for a single locale
 */
async function importLocale(session: Session, localeCode: string): Promise<{
  identity: boolean;
  voice: boolean;
  culture: boolean;
  market: boolean;
  lexicon: boolean;
  expressions: number;
}> {
  const stats = {
    identity: false,
    voice: false,
    culture: false,
    market: false,
    lexicon: false,
    expressions: 0,
  };

  const file = `${localeCode}.md`;

  // Parse all files for this locale
  const identity = parseFile(join(DATA_PATH, '1-identity', file), parseIdentityMd);
  const voice = parseFile(join(DATA_PATH, '3-voice-style', file), parseVoiceMd);
  const culture = parseFile(join(DATA_PATH, '4-culture-norms', file), parseCultureMd);
  const market = parseFile(join(DATA_PATH, '5-market', file), parseMarketMd);
  const lexiconData = parseFile(join(DATA_PATH, '3-voice-lexicon', file), parseLexiconMd);

  const lexicon = lexiconData?.lexicon || null;
  const expressions = lexiconData?.expressions || [];

  // Check if Locale node exists, create if not
  await session.run(`
    MERGE (l:Locale {key: $key})
    ON CREATE SET
      l.language = $langCode,
      l.region = $countryCode,
      l.name = $name,
      l.created_at = datetime()
    SET l.updated_at = datetime()
    RETURN l
  `, {
    key: localeCode,
    langCode: localeCode.split('-')[0],
    countryCode: localeCode.split('-')[1],
    name: `${localeCode.split('-')[0].toUpperCase()} (${localeCode.split('-')[1]})`,
  });

  // Create LocaleIdentity
  if (identity && Object.keys(identity).length > 0) {
    const props = serializeForNeo4j(identity as Record<string, unknown>);
    await session.run(`
      MATCH (l:Locale {key: $key})
      MERGE (l)-[:HAS_IDENTITY]->(i:LocaleIdentity)
      SET i = $props, i.updated_at = datetime()
      RETURN i
    `, { key: localeCode, props });
    stats.identity = true;
    console.log(`  [OK] LocaleIdentity created`);
  }

  // Create LocaleVoice
  if (voice && Object.keys(voice).length > 0) {
    const props = serializeForNeo4j(voice as Record<string, unknown>);
    await session.run(`
      MATCH (l:Locale {key: $key})
      MERGE (l)-[:HAS_VOICE]->(v:LocaleVoice)
      SET v = $props, v.updated_at = datetime()
      RETURN v
    `, { key: localeCode, props });
    stats.voice = true;
    console.log(`  [OK] LocaleVoice created`);
  }

  // Create LocaleCulture
  if (culture && Object.keys(culture).length > 0) {
    const props = serializeForNeo4j(culture as Record<string, unknown>);
    await session.run(`
      MATCH (l:Locale {key: $key})
      MERGE (l)-[:HAS_CULTURE]->(c:LocaleCulture)
      SET c = $props, c.updated_at = datetime()
      RETURN c
    `, { key: localeCode, props });
    stats.culture = true;
    console.log(`  [OK] LocaleCulture created`);
  }

  // Create LocaleMarket
  if (market && Object.keys(market).length > 0) {
    const props = serializeForNeo4j(market as Record<string, unknown>);
    await session.run(`
      MATCH (l:Locale {key: $key})
      MERGE (l)-[:HAS_MARKET]->(m:LocaleMarket)
      SET m = $props, m.updated_at = datetime()
      RETURN m
    `, { key: localeCode, props });
    stats.market = true;
    console.log(`  [OK] LocaleMarket created`);
  }

  // Create LocaleLexicon and Expressions
  if (lexicon && Object.keys(lexicon).length > 0) {
    const lexiconProps = serializeForNeo4j(lexicon as Record<string, unknown>);
    await session.run(`
      MATCH (l:Locale {key: $key})
      MERGE (l)-[:HAS_LEXICON]->(lex:LocaleLexicon)
      SET lex = $props, lex.updated_at = datetime()
      RETURN lex
    `, { key: localeCode, props: lexiconProps });
    stats.lexicon = true;
    console.log(`  [OK] LocaleLexicon created`);

    // Create Expression nodes
    if (expressions.length > 0) {
      // Delete existing expressions for this locale before re-importing
      await session.run(`
        MATCH (l:Locale {key: $key})-[:HAS_LEXICON]->(lex:LocaleLexicon)-[:HAS_EXPRESSION]->(e:Expression)
        DETACH DELETE e
      `, { key: localeCode });

      // Batch insert expressions
      for (const expr of expressions) {
        const exprProps = serializeForNeo4j(expr as Record<string, unknown>);
        await session.run(`
          MATCH (l:Locale {key: $key})-[:HAS_LEXICON]->(lex:LocaleLexicon)
          CREATE (lex)-[:HAS_EXPRESSION]->(e:Expression)
          SET e = $props, e.created_at = datetime(), e.updated_at = datetime()
        `, { key: localeCode, props: exprProps });
        stats.expressions++;
      }
      console.log(`  [OK] ${stats.expressions} Expression nodes created`);
    }
  }

  return stats;
}

/**
 * Main entry point
 */
async function main(): Promise<void> {
  // Parse command line arguments
  const args = process.argv.slice(2);
  let targetLocale: string | null = null;

  for (const arg of args) {
    if (arg.startsWith('--locale=')) {
      targetLocale = arg.split('=')[1];
    }
  }

  console.log('=== Locale Knowledge Import ===');
  console.log(`Neo4j URI: ${NEO4J_URI}`);
  console.log(`Target: ${targetLocale || 'ALL locales'}`);
  console.log('');

  // Connect to Neo4j
  const driver: Driver = neo4j.driver(
    NEO4J_URI,
    neo4j.auth.basic(NEO4J_USER, NEO4J_PASSWORD)
  );

  const session: Session = driver.session();

  try {
    // Determine which locales to import
    let locales: string[];

    if (targetLocale) {
      locales = [targetLocale];
    } else {
      // Get all locale codes from identity folder
      const identityPath = join(DATA_PATH, '1-identity');
      if (!existsSync(identityPath)) {
        throw new Error(`Data path not found: ${identityPath}`);
      }
      locales = readdirSync(identityPath)
        .filter(f => f.endsWith('.md'))
        .map(f => f.replace('.md', ''));
    }

    console.log(`Found ${locales.length} locale(s) to import\n`);

    // Track totals
    const totals = {
      locales: 0,
      identity: 0,
      voice: 0,
      culture: 0,
      market: 0,
      lexicon: 0,
      expressions: 0,
    };

    // Import each locale
    for (const localeCode of locales) {
      console.log(`Importing ${localeCode}...`);
      const stats = await importLocale(session, localeCode);

      totals.locales++;
      if (stats.identity) totals.identity++;
      if (stats.voice) totals.voice++;
      if (stats.culture) totals.culture++;
      if (stats.market) totals.market++;
      if (stats.lexicon) totals.lexicon++;
      totals.expressions += stats.expressions;

      console.log('');
    }

    // Print summary
    console.log('=== Import Complete ===');
    console.log(`Locales processed: ${totals.locales}`);
    console.log(`LocaleIdentity nodes: ${totals.identity}`);
    console.log(`LocaleVoice nodes: ${totals.voice}`);
    console.log(`LocaleCulture nodes: ${totals.culture}`);
    console.log(`LocaleMarket nodes: ${totals.market}`);
    console.log(`LocaleLexicon nodes: ${totals.lexicon}`);
    console.log(`Expression nodes: ${totals.expressions}`);

  } catch (error) {
    console.error('Import failed:', error);
    throw error;
  } finally {
    await session.close();
    await driver.close();
  }
}

// Run
main().catch(error => {
  console.error('Fatal error:', error);
  process.exit(1);
});
