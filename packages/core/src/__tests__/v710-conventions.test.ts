// NovaNet Core - v8.2.0 Convention Tests
// TDD: Verify key naming, llm_context format
//
// These tests query Neo4j to verify actual data matches v8.2.0 conventions
//
// v8.2.0 CHANGES:
// - REMOVED: icon, priority, freshness properties (YAGNI)
// - Standard properties are now: key, display_name, description, llm_context, created_at, updated_at

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { type Session } from 'neo4j-driver';
import { getDriver, closeDriver } from '../db/client.js';

// v8.2.0 CONVENTIONS
const KEY_PREFIXES: Record<string, string[] | null> = {
  Concept: ['action-', 'product-', 'feature-', 'tier-'],
  Page: ['page-'],
  Block: ['block-'],
  BlockType: ['blocktype-'],
  Project: ['project-'],
  Locale: null, // BCP 47 format (en-US, fr-FR, etc.)
};

// llm_context format: "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."
const LLM_CONTEXT_PATTERN = /^USE:.*\. TRIGGERS:.*\. NOT:.*\.$/;

describe('v8.2.0 Conventions', () => {
  let session: Session;

  beforeAll(async () => {
    session = getDriver().session();
  });

  afterAll(async () => {
    await session.close();
    await closeDriver();
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // KEY NAMING CONVENTION TESTS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('Key Naming Convention', () => {
    it('Concept keys should have semantic prefix (action-, product-, feature-, tier-)', async () => {
      const result = await session.run(`
        MATCH (c:Concept)
        RETURN c.key AS key
      `);

      const keys = result.records.map(r => r.get('key') as string);
      const invalidKeys: string[] = [];

      for (const key of keys) {
        const prefixes = KEY_PREFIXES.Concept;
        const hasValidPrefix = prefixes?.some(prefix => key.startsWith(prefix)) ?? false;
        if (!hasValidPrefix) {
          invalidKeys.push(key);
        }
      }

      expect(invalidKeys).toEqual([]);
      expect(keys.length).toBeGreaterThan(0);
    });

    it('Page keys should have page- prefix', async () => {
      const result = await session.run(`
        MATCH (p:Page)
        RETURN p.key AS key
      `);

      const keys = result.records.map(r => r.get('key') as string);
      const invalidKeys = keys.filter(key => !key.startsWith('page-'));

      expect(invalidKeys).toEqual([]);
    });

    it('Block keys should have block- prefix', async () => {
      const result = await session.run(`
        MATCH (b:Block)
        RETURN b.key AS key
      `);

      const keys = result.records.map(r => r.get('key') as string);
      const invalidKeys = keys.filter(key => !key.startsWith('block-'));

      expect(invalidKeys).toEqual([]);
    });

    it('BlockType keys should have blocktype- prefix', async () => {
      const result = await session.run(`
        MATCH (bt:BlockType)
        RETURN bt.key AS key
      `);

      const keys = result.records.map(r => r.get('key') as string);
      const invalidKeys = keys.filter(key => !key.startsWith('blocktype-'));

      expect(invalidKeys).toEqual([]);
    });

    it('Project keys should have project- prefix', async () => {
      const result = await session.run(`
        MATCH (p:Project)
        RETURN p.key AS key
      `);

      const keys = result.records.map(r => r.get('key') as string);
      const invalidKeys = keys.filter(key => !key.startsWith('project-'));

      expect(invalidKeys).toEqual([]);
    });

    it('Locale keys should be BCP 47 format (xx-XX or xxx-XX)', async () => {
      const result = await session.run(`
        MATCH (l:Locale)
        RETURN l.key AS key
      `);

      const keys = result.records.map(r => r.get('key') as string);
      // BCP 47 allows 2 or 3 letter language codes
      const bcp47Pattern = /^[a-z]{2,3}-[A-Z]{2}$/;
      const invalidKeys = keys.filter(key => !bcp47Pattern.test(key));

      expect(invalidKeys).toEqual([]);
      expect(keys.length).toBeGreaterThan(0);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // LLM_CONTEXT FORMAT TESTS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('llm_context Format (USE/TRIGGERS/NOT)', () => {
    it('Concept llm_context should follow USE/TRIGGERS/NOT format', async () => {
      const result = await session.run(`
        MATCH (c:Concept)
        RETURN c.key AS key, c.llm_context AS llm_context
      `);

      const invalidContexts: Array<{ key: string; llm_context: string }> = [];

      for (const record of result.records) {
        const key = record.get('key') as string;
        const llmContext = record.get('llm_context') as string;

        if (!LLM_CONTEXT_PATTERN.test(llmContext)) {
          invalidContexts.push({ key, llm_context: llmContext });
        }
      }

      expect(invalidContexts).toEqual([]);
    });

    it('Locale Knowledge nodes should follow USE/TRIGGERS/NOT format', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n:LocaleIdentity OR n:LocaleVoice OR n:LocaleCulture OR n:LocaleMarket OR n:LocaleLexicon
        RETURN labels(n)[0] AS label, n.display_name AS name, n.llm_context AS llm_context
      `);

      const invalidContexts: Array<{ label: string; name: string; llm_context: string }> = [];

      for (const record of result.records) {
        const label = record.get('label') as string;
        const name = record.get('name') as string;
        const llmContext = record.get('llm_context') as string;

        if (!LLM_CONTEXT_PATTERN.test(llmContext)) {
          invalidContexts.push({ label, name, llm_context: llmContext });
        }
      }

      expect(invalidContexts).toEqual([]);
    });

    it('Page/Block/BlockType llm_context should follow USE/TRIGGERS/NOT format', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n:Page OR n:Block OR n:BlockType
        RETURN labels(n)[0] AS label, n.key AS key, n.llm_context AS llm_context
      `);

      const invalidContexts: Array<{ label: string; key: string; llm_context: string }> = [];

      for (const record of result.records) {
        const label = record.get('label') as string;
        const key = record.get('key') as string;
        const llmContext = record.get('llm_context') as string;

        if (!LLM_CONTEXT_PATTERN.test(llmContext)) {
          invalidContexts.push({ label, key, llm_context: llmContext });
        }
      }

      expect(invalidContexts).toEqual([]);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // v8.2.0 REMOVED: Priority & Freshness tests (YAGNI - moved to application layer)
  // ═══════════════════════════════════════════════════════════════════════════

  // ═══════════════════════════════════════════════════════════════════════════
  // CONSISTENCY TESTS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('Consistency', () => {
    it('All Concepts should have ConceptL10n for seeded locales', async () => {
      const result = await session.run(`
        MATCH (c:Concept)
        OPTIONAL MATCH (c)-[:HAS_L10N]->(cl:ConceptL10n)
        RETURN c.key AS concept, count(cl) AS l10n_count
      `);

      const conceptsWithoutL10n: string[] = [];

      for (const record of result.records) {
        const concept = record.get('concept') as string;
        const count = (record.get('l10n_count') as { low: number }).low;

        if (count === 0) {
          conceptsWithoutL10n.push(concept);
        }
      }

      expect(conceptsWithoutL10n).toEqual([]);
    });

    it('All Locales with knowledge should have all 5 knowledge nodes', async () => {
      const result = await session.run(`
        MATCH (l:Locale)
        WHERE l.key IN ['en-US', 'fr-FR', 'ja-JP']
        OPTIONAL MATCH (l)-[:HAS_IDENTITY]->(li:LocaleIdentity)
        OPTIONAL MATCH (l)-[:HAS_VOICE]->(lv:LocaleVoice)
        OPTIONAL MATCH (l)-[:HAS_CULTURE]->(lc:LocaleCulture)
        OPTIONAL MATCH (l)-[:HAS_MARKET]->(lm:LocaleMarket)
        OPTIONAL MATCH (l)-[:HAS_LEXICON]->(ll:LocaleLexicon)
        RETURN l.key AS locale,
               li IS NOT NULL AS has_identity,
               lv IS NOT NULL AS has_voice,
               lc IS NOT NULL AS has_culture,
               lm IS NOT NULL AS has_market,
               ll IS NOT NULL AS has_lexicon
      `);

      const incompleteLocales: Array<{ locale: string; missing: string[] }> = [];

      for (const record of result.records) {
        const locale = record.get('locale') as string;
        const missing: string[] = [];

        if (!record.get('has_identity')) missing.push('identity');
        if (!record.get('has_voice')) missing.push('voice');
        if (!record.get('has_culture')) missing.push('culture');
        if (!record.get('has_market')) missing.push('market');
        if (!record.get('has_lexicon')) missing.push('lexicon');

        if (missing.length > 0) {
          incompleteLocales.push({ locale, missing });
        }
      }

      expect(incompleteLocales).toEqual([]);
    });

    it('SEMANTIC_LINK temperature should be between 0 and 1', async () => {
      const result = await session.run(`
        MATCH ()-[r:SEMANTIC_LINK]->()
        RETURN r.temperature AS temp, r.type AS type
      `);

      const invalidTemps: Array<{ type: string; temp: number }> = [];

      for (const record of result.records) {
        const temp = record.get('temp') as number;
        const type = record.get('type') as string;

        if (temp < 0 || temp > 1) {
          invalidTemps.push({ type, temp });
        }
      }

      expect(invalidTemps).toEqual([]);
    });

    it('ConceptL10n llm_context should follow USE/TRIGGERS/NOT format', async () => {
      const result = await session.run(`
        MATCH (c:Concept)-[:HAS_L10N]->(cl:ConceptL10n)-[:FOR_LOCALE]->(l:Locale)
        RETURN c.key AS concept, l.key AS locale, cl.llm_context AS llm_context
      `);

      const invalidContexts: Array<{ concept: string; locale: string; llm_context: string }> = [];

      for (const record of result.records) {
        const concept = record.get('concept') as string;
        const locale = record.get('locale') as string;
        const llmContext = record.get('llm_context') as string;

        if (!LLM_CONTEXT_PATTERN.test(llmContext)) {
          invalidContexts.push({ concept, locale, llm_context: llmContext });
        }
      }

      expect(invalidContexts).toEqual([]);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // PROPERTY ORDER (Documentation compliance)
  // ═══════════════════════════════════════════════════════════════════════════

  describe('Required Properties', () => {
    it('All keyed nodes should have display_name', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n.key IS NOT NULL AND n.display_name IS NULL
        RETURN labels(n)[0] AS label, n.key AS key
      `);

      expect(result.records.length).toBe(0);
    });

    // v8.2.0 REMOVED: icon property test (YAGNI - icons moved to presentation layer)

    it('All keyed nodes should have description', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n.key IS NOT NULL AND n.description IS NULL
        RETURN labels(n)[0] AS label, n.key AS key
      `);

      expect(result.records.length).toBe(0);
    });

    it('All keyed nodes should have created_at and updated_at', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n.key IS NOT NULL AND (n.created_at IS NULL OR n.updated_at IS NULL)
        RETURN labels(n)[0] AS label, n.key AS key
      `);

      expect(result.records.length).toBe(0);
    });
  });
});
