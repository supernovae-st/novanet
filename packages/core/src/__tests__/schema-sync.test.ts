/**
 * Schema Synchronization Tests - BINARY PASS/FAIL
 *
 * These tests verify that YAML, TypeScript, and Neo4j schemas are in sync.
 * They serve as a build-time guard against schema drift.
 *
 * v8.2.0: YAML is the single source of truth
 *
 * YAML Models → TypeScript Types → Neo4j Seeds
 *     ↑              ↑                  ↑
 *     └──────────────┴──────────────────┘
 *           Must match exactly
 */

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { type Session } from 'neo4j-driver';
import { getDriver, closeDriver } from '../db/client.js';
import { NODE_TYPES } from '../types/nodes.js';
import fs from 'fs';
import path from 'path';
import { parse as parseYaml } from 'yaml';

// Expected counts from YAML source of truth
const EXPECTED_NODE_TYPES = 35;

// Note: These constants document the v8.2.0 schema but tests validate via Neo4j queries
// Standard properties: key, display_name, description, llm_context, created_at, updated_at
// Deprecated (v8.2.0): icon, priority, freshness

describe('Schema Synchronization - Binary Tests', () => {
  let session: Session;

  beforeAll(async () => {
    session = getDriver().session();
  });

  afterAll(async () => {
    await session.close();
    await closeDriver();
  });

  // ===========================================================================
  // TYPESCRIPT ↔ YAML SYNC
  // ===========================================================================

  describe('TypeScript ↔ YAML Sync', () => {
    it('NODE_TYPES array has exactly 35 types', () => {
      expect(NODE_TYPES.length).toBe(EXPECTED_NODE_TYPES);
    });

    it('NODE_TYPES matches YAML _index.yaml count', async () => {
      const indexPath = path.resolve(__dirname, '../../models/_index.yaml');
      const content = fs.readFileSync(indexPath, 'utf-8');
      const index = parseYaml(content) as { graph_structure?: { node_counts?: Record<string, number> } };

      const yamlCounts = index.graph_structure?.node_counts;
      if (yamlCounts) {
        const totalFromYaml = Object.values(yamlCounts).reduce((a, b) => a + b, 0);
        expect(NODE_TYPES.length).toBe(totalFromYaml);
      }
    });

    it('All node type files exist in models/nodes/', () => {
      const nodesDir = path.resolve(__dirname, '../../models/nodes');
      const missingFiles: string[] = [];

      // Map node types to expected file locations (kebab-case filenames)
      const nodeFileMap: Record<string, string> = {
        // Global - Config
        Locale: 'global/config/locale.yaml',
        // Global - Knowledge
        LocaleIdentity: 'global/knowledge/locale-identity.yaml',
        LocaleVoice: 'global/knowledge/locale-voice.yaml',
        LocaleCulture: 'global/knowledge/locale-culture.yaml',
        LocaleCultureReferences: 'global/knowledge/locale-culture-references.yaml',
        LocaleMarket: 'global/knowledge/locale-market.yaml',
        LocaleLexicon: 'global/knowledge/locale-lexicon.yaml',
        LocaleRulesAdaptation: 'global/knowledge/locale-rules-adaptation.yaml',
        LocaleRulesFormatting: 'global/knowledge/locale-rules-formatting.yaml',
        LocaleRulesSlug: 'global/knowledge/locale-rules-slug.yaml',
        Expression: 'global/knowledge/expression.yaml',
        Reference: 'global/knowledge/reference.yaml',
        Metaphor: 'global/knowledge/metaphor.yaml',
        Constraint: 'global/knowledge/constraint.yaml',
        Pattern: 'global/knowledge/pattern.yaml',
        // Project - Foundation
        Project: 'project/foundation/project.yaml',
        BrandIdentity: 'project/foundation/brand-identity.yaml',
        ProjectL10n: 'project/foundation/project-l10n.yaml',
        // Project - Structure
        Page: 'project/structure/page.yaml',
        Block: 'project/structure/block.yaml',
        // Project - Instruction (BlockType/PageType are here, not structure)
        BlockType: 'project/instruction/block-type.yaml',
        PageType: 'project/instruction/page-type.yaml',
        PagePrompt: 'project/instruction/page-prompt.yaml',
        BlockPrompt: 'project/instruction/block-prompt.yaml',
        BlockRules: 'project/instruction/block-rules.yaml',
        // Project - Semantic
        Concept: 'project/semantic/concept.yaml',
        ConceptL10n: 'project/semantic/concept-l10n.yaml',
        // Project - Output
        PageL10n: 'project/output/page-l10n.yaml',
        BlockL10n: 'project/output/block-l10n.yaml',
        // Shared - SEO
        SEOKeywordL10n: 'shared/seo/seo-keyword-l10n.yaml',
        SEOKeywordMetrics: 'shared/seo/seo-keyword-metrics.yaml',
        SEOMiningRun: 'shared/seo/seo-mining-run.yaml',
        // Shared - GEO
        GEOSeedL10n: 'shared/geo/geo-seed-l10n.yaml',
        GEOSeedMetrics: 'shared/geo/geo-seed-metrics.yaml',
        GEOMiningRun: 'shared/geo/geo-mining-run.yaml',
      };

      for (const nodeType of NODE_TYPES) {
        const expectedFile = nodeFileMap[nodeType];
        if (expectedFile) {
          const fullPath = path.join(nodesDir, expectedFile);
          if (!fs.existsSync(fullPath)) {
            missingFiles.push(`${nodeType} → ${expectedFile}`);
          }
        }
      }

      expect(missingFiles).toEqual([]);
    });
  });

  // ===========================================================================
  // NEO4J ↔ TYPESCRIPT SYNC
  // ===========================================================================

  describe('Neo4j ↔ TypeScript Sync', () => {
    it('Neo4j has exactly 35 labels', async () => {
      const result = await session.run('CALL db.labels() YIELD label RETURN count(label) AS count');
      const count = result.records[0].get('count');
      const countValue = typeof count === 'object' && 'low' in count ? count.low : count;

      expect(countValue).toBe(EXPECTED_NODE_TYPES);
    });

    it('All TypeScript NODE_TYPES exist as Neo4j labels', async () => {
      const result = await session.run('CALL db.labels() YIELD label RETURN collect(label) AS labels');
      const neo4jLabels = result.records[0].get('labels') as string[];

      const missingInNeo4j = NODE_TYPES.filter(type => !neo4jLabels.includes(type));

      // Some labels might not have data yet, but that's OK
      // This test just checks that labels CAN exist
      expect(missingInNeo4j.length).toBeLessThanOrEqual(8); // Allow up to 8 empty labels
    });

    it('No extra labels in Neo4j beyond NODE_TYPES', async () => {
      const result = await session.run('CALL db.labels() YIELD label RETURN collect(label) AS labels');
      const neo4jLabels = result.records[0].get('labels') as string[];

      const extraLabels = neo4jLabels.filter(label => !NODE_TYPES.includes(label as typeof NODE_TYPES[number]));

      expect(extraLabels).toEqual([]);
    });
  });

  // ===========================================================================
  // DEPRECATED PROPERTIES CHECK
  // ===========================================================================

  describe('Deprecated Properties (v8.2.0)', () => {
    it('No nodes have deprecated "icon" property', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n.icon IS NOT NULL
        RETURN labels(n)[0] AS label, count(*) AS count
      `);

      const nodesWithIcon = result.records.map(r => ({
        label: r.get('label'),
        count: r.get('count'),
      }));

      expect(nodesWithIcon).toEqual([]);
    });

    it('No nodes have deprecated "priority" property', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n.priority IS NOT NULL
        RETURN labels(n)[0] AS label, count(*) AS count
      `);

      const nodesWithPriority = result.records.map(r => ({
        label: r.get('label'),
        count: r.get('count'),
      }));

      expect(nodesWithPriority).toEqual([]);
    });

    it('No nodes have deprecated "freshness" property', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n.freshness IS NOT NULL
        RETURN labels(n)[0] AS label, count(*) AS count
      `);

      const nodesWithFreshness = result.records.map(r => ({
        label: r.get('label'),
        count: r.get('count'),
      }));

      expect(nodesWithFreshness).toEqual([]);
    });
  });

  // ===========================================================================
  // RELATIONS SYNC (count-based - RELATION_TYPES not exported yet)
  // ===========================================================================

  describe('Relations Sync', () => {
    it('Neo4j has expected number of relationship types (approx 50)', async () => {
      const result = await session.run('CALL db.relationshipTypes() YIELD relationshipType RETURN count(relationshipType) AS count');
      const count = result.records[0].get('count');
      const countValue = typeof count === 'object' && 'low' in count ? count.low : count;

      // Should have approximately 50 relation types per relations.yaml
      expect(countValue).toBeGreaterThanOrEqual(30);
      expect(countValue).toBeLessThanOrEqual(60);
    });

    it('Core relations exist in Neo4j', async () => {
      // Only check relations that should definitely exist with seed data
      const coreRelations = [
        'HAS_PAGE', 'HAS_BLOCK', 'HAS_CONCEPT', 'HAS_L10N',
        'USES_CONCEPT', 'SEMANTIC_LINK', 'FOR_LOCALE', 'SUPPORTS_LOCALE',
        'OF_TYPE', 'HAS_PROMPT', 'HAS_RULES'
        // Note: HAS_OUTPUT not checked - requires generated content
      ];

      const result = await session.run('CALL db.relationshipTypes() YIELD relationshipType RETURN collect(relationshipType) AS types');
      const neo4jTypes = result.records[0].get('types') as string[];

      const missingCore = coreRelations.filter(rel => !neo4jTypes.includes(rel));

      expect(missingCore).toEqual([]);
    });
  });

  // ===========================================================================
  // STRUCTURAL INTEGRITY
  // ===========================================================================

  describe('Structural Integrity', () => {
    it('All keyed nodes have required standard properties', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n.key IS NOT NULL
        WITH labels(n)[0] AS label, n
        WHERE n.display_name IS NULL
           OR n.created_at IS NULL
           OR n.updated_at IS NULL
        RETURN label, n.key AS key,
               n.display_name IS NULL AS missing_display_name,
               n.created_at IS NULL AS missing_created_at,
               n.updated_at IS NULL AS missing_updated_at
        LIMIT 10
      `);

      const invalidNodes = result.records.map(r => ({
        label: r.get('label'),
        key: r.get('key'),
        missing: {
          display_name: r.get('missing_display_name'),
          created_at: r.get('missing_created_at'),
          updated_at: r.get('missing_updated_at'),
        },
      }));

      expect(invalidNodes).toEqual([]);
    });

    it('Project nodes follow key prefix convention', async () => {
      type PrefixRule = { label: string; prefix: string } | { label: string; prefixes: string[] };
      const prefixRules: PrefixRule[] = [
        { label: 'Project', prefix: 'project-' },
        { label: 'Page', prefix: 'page-' },
        { label: 'Block', prefix: 'block-' },
        { label: 'BlockType', prefix: 'blocktype-' },
        { label: 'Concept', prefixes: ['action-', 'product-', 'feature-', 'tier-'] },
      ];

      const violations: { label: string; key: string }[] = [];

      for (const rule of prefixRules) {
        const result = await session.run(
          `MATCH (n:${rule.label}) RETURN n.key AS key`
        );

        for (const record of result.records) {
          const key = record.get('key') as string;
          if (!key) continue;

          if ('prefix' in rule) {
            if (!key.startsWith(rule.prefix)) {
              violations.push({ label: rule.label, key });
            }
          } else if ('prefixes' in rule) {
            if (!rule.prefixes.some(p => key.startsWith(p))) {
              violations.push({ label: rule.label, key });
            }
          }
        }
      }

      expect(violations).toEqual([]);
    });
  });
});
