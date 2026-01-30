// NovaNet Core - v8.2.0 Convention Tests
// Verify Prompt nodes and provenance tracking
//
// These tests query Neo4j to verify actual data matches v8.2.0 conventions
// Note: Tests will fail until migration (Task 8) runs to create Prompt nodes
//
// v8.2.0 CHANGES:
// - REMOVED: icon, priority, freshness properties (YAGNI)
// - Standard properties are now: key, display_name, description, llm_context, created_at, updated_at

import { describe, it, expect, beforeAll, afterAll } from 'vitest';
import { type Session } from 'neo4j-driver';
import { getDriver, closeDriver } from '../db/client.js';

// Version format: semver (1.0, 1.0.0, 2.1, 2.1.3)
const VERSION_PATTERN = /^\d+\.\d+(\.\d+)?$/;

// llm_context format: "USE: [when]. TRIGGERS: [keywords]. NOT: [disambiguation]."
const LLM_CONTEXT_PATTERN = /^USE:.*\. TRIGGERS:.*\. NOT:.*\.$/;

describe('v8.2.0 Prompt Conventions', () => {
  let session: Session;

  beforeAll(async () => {
    session = getDriver().session();
  });

  afterAll(async () => {
    await session.close();
    await closeDriver();
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // PROMPT NODES EXISTENCE TESTS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('Prompt Nodes', () => {
    it('All Pages should have at least one PagePrompt', async () => {
      const result = await session.run(`
        MATCH (p:Page)
        OPTIONAL MATCH (p)-[:HAS_PROMPT]->(pp:PagePrompt)
        RETURN p.key AS page, count(pp) AS prompt_count
      `);

      const pagesWithoutPrompts = result.records
        .filter(r => {
          const count = r.get('prompt_count');
          // Handle neo4j Integer type
          const countValue = typeof count === 'object' && count !== null && 'low' in count
            ? (count as { low: number }).low
            : count as number;
          return countValue === 0;
        })
        .map(r => r.get('page') as string);

      expect(pagesWithoutPrompts).toEqual([]);
    });

    it('All Blocks should have at least one BlockPrompt', async () => {
      const result = await session.run(`
        MATCH (b:Block)
        OPTIONAL MATCH (b)-[:HAS_PROMPT]->(bp:BlockPrompt)
        RETURN b.key AS block, count(bp) AS prompt_count
      `);

      const blocksWithoutPrompts = result.records
        .filter(r => {
          const count = r.get('prompt_count');
          const countValue = typeof count === 'object' && count !== null && 'low' in count
            ? (count as { low: number }).low
            : count as number;
          return countValue === 0;
        })
        .map(r => r.get('block') as string);

      expect(blocksWithoutPrompts).toEqual([]);
    });

    it('All BlockTypes should have at least one BlockRules', async () => {
      const result = await session.run(`
        MATCH (bt:BlockType)
        OPTIONAL MATCH (bt)-[:HAS_RULES]->(br:BlockRules)
        RETURN bt.key AS blocktype, count(br) AS rules_count
      `);

      const typesWithoutRules = result.records
        .filter(r => {
          const count = r.get('rules_count');
          const countValue = typeof count === 'object' && count !== null && 'low' in count
            ? (count as { low: number }).low
            : count as number;
          return countValue === 0;
        })
        .map(r => r.get('blocktype') as string);

      expect(typesWithoutRules).toEqual([]);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // VERSION FORMAT TESTS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('Version Format', () => {
    it('Prompt nodes should have valid version format (semver)', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE n:PagePrompt OR n:BlockPrompt OR n:BlockRules
        RETURN labels(n)[0] AS type, n.display_name AS name, n.version AS version
      `);

      const invalidVersions = result.records
        .filter(r => {
          const version = r.get('version') as string | null;
          return !version || !VERSION_PATTERN.test(version);
        })
        .map(r => ({
          type: r.get('type') as string,
          name: r.get('name') as string,
          version: r.get('version') as string | null,
        }));

      expect(invalidVersions).toEqual([]);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // ACTIVE PROMPT UNIQUENESS TESTS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('Active Prompt Uniqueness', () => {
    it('Only one active PagePrompt per Page', async () => {
      const result = await session.run(`
        MATCH (p:Page)-[:HAS_PROMPT]->(pp:PagePrompt {active: true})
        WITH p, count(pp) AS active_count
        WHERE active_count > 1
        RETURN p.key AS page, active_count
      `);

      const pagesWithMultipleActive = result.records.map(r => ({
        page: r.get('page') as string,
        activeCount: r.get('active_count'),
      }));

      expect(pagesWithMultipleActive).toEqual([]);
    });

    it('Only one active BlockPrompt per Block', async () => {
      const result = await session.run(`
        MATCH (b:Block)-[:HAS_PROMPT]->(bp:BlockPrompt {active: true})
        WITH b, count(bp) AS active_count
        WHERE active_count > 1
        RETURN b.key AS block, active_count
      `);

      const blocksWithMultipleActive = result.records.map(r => ({
        block: r.get('block') as string,
        activeCount: r.get('active_count'),
      }));

      expect(blocksWithMultipleActive).toEqual([]);
    });

    it('Only one active BlockRules per BlockType', async () => {
      const result = await session.run(`
        MATCH (bt:BlockType)-[:HAS_RULES]->(br:BlockRules {active: true})
        WITH bt, count(br) AS active_count
        WHERE active_count > 1
        RETURN bt.key AS blocktype, active_count
      `);

      const typesWithMultipleActive = result.records.map(r => ({
        blocktype: r.get('blocktype') as string,
        activeCount: r.get('active_count'),
      }));

      expect(typesWithMultipleActive).toEqual([]);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // LLM_CONTEXT FORMAT TESTS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('llm_context Format (USE/TRIGGERS/NOT)', () => {
    it('PagePrompt nodes should follow USE/TRIGGERS/NOT format', async () => {
      const result = await session.run(`
        MATCH (pp:PagePrompt)
        RETURN pp.display_name AS name, pp.llm_context AS llm_context
      `);

      const invalidContexts = result.records
        .filter(r => {
          const llmContext = r.get('llm_context') as string | null;
          return !llmContext || !LLM_CONTEXT_PATTERN.test(llmContext);
        })
        .map(r => ({
          name: r.get('name') as string,
          llm_context: r.get('llm_context') as string | null,
        }));

      expect(invalidContexts).toEqual([]);
    });

    it('BlockPrompt nodes should follow USE/TRIGGERS/NOT format', async () => {
      const result = await session.run(`
        MATCH (bp:BlockPrompt)
        RETURN bp.display_name AS name, bp.llm_context AS llm_context
      `);

      const invalidContexts = result.records
        .filter(r => {
          const llmContext = r.get('llm_context') as string | null;
          return !llmContext || !LLM_CONTEXT_PATTERN.test(llmContext);
        })
        .map(r => ({
          name: r.get('name') as string,
          llm_context: r.get('llm_context') as string | null,
        }));

      expect(invalidContexts).toEqual([]);
    });

    it('BlockRules nodes should follow USE/TRIGGERS/NOT format', async () => {
      const result = await session.run(`
        MATCH (br:BlockRules)
        RETURN br.display_name AS name, br.llm_context AS llm_context
      `);

      const invalidContexts = result.records
        .filter(r => {
          const llmContext = r.get('llm_context') as string | null;
          return !llmContext || !LLM_CONTEXT_PATTERN.test(llmContext);
        })
        .map(r => ({
          name: r.get('name') as string,
          llm_context: r.get('llm_context') as string | null,
        }));

      expect(invalidContexts).toEqual([]);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // REQUIRED PROPERTIES TESTS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('Required Properties', () => {
    it('All Prompt nodes should have display_name', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE (n:PagePrompt OR n:BlockPrompt OR n:BlockRules)
          AND n.display_name IS NULL
        RETURN labels(n)[0] AS type
      `);

      expect(result.records.length).toBe(0);
    });

    // v8.2.0 REMOVED: icon property test (YAGNI - icons moved to presentation layer)

    it('All Prompt nodes should have description', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE (n:PagePrompt OR n:BlockPrompt OR n:BlockRules)
          AND n.description IS NULL
        RETURN labels(n)[0] AS type, n.display_name AS name
      `);

      expect(result.records.length).toBe(0);
    });

    it('All Prompt nodes should have created_at and updated_at', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE (n:PagePrompt OR n:BlockPrompt OR n:BlockRules)
          AND (n.created_at IS NULL OR n.updated_at IS NULL)
        RETURN labels(n)[0] AS type, n.display_name AS name
      `);

      expect(result.records.length).toBe(0);
    });

    // v8.2.0 REMOVED: priority property test (YAGNI - moved to application layer if needed)
    // v8.2.0 REMOVED: freshness property test (YAGNI - moved to application layer if needed)

    it('PagePrompt and BlockPrompt should have prompt property', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE (n:PagePrompt OR n:BlockPrompt)
          AND (n.prompt IS NULL OR n.prompt = '')
        RETURN labels(n)[0] AS type, n.display_name AS name
      `);

      expect(result.records.length).toBe(0);
    });

    it('BlockRules should have rules property', async () => {
      const result = await session.run(`
        MATCH (n:BlockRules)
        WHERE n.rules IS NULL OR n.rules = ''
        RETURN n.display_name AS name
      `);

      expect(result.records.length).toBe(0);
    });

    it('All Prompt nodes should have active boolean property', async () => {
      const result = await session.run(`
        MATCH (n)
        WHERE (n:PagePrompt OR n:BlockPrompt OR n:BlockRules)
          AND n.active IS NULL
        RETURN labels(n)[0] AS type, n.display_name AS name
      `);

      expect(result.records.length).toBe(0);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // PROVENANCE TRACKING TESTS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('Provenance Tracking', () => {
    it('GENERATED relation should have generated_at timestamp', async () => {
      const result = await session.run(`
        MATCH ()-[r:GENERATED]->()
        WHERE r.generated_at IS NULL
        RETURN count(r) AS missing_count
      `);

      const missingCount = result.records[0]?.get('missing_count');
      const countValue = typeof missingCount === 'object' && missingCount !== null && 'low' in missingCount
        ? (missingCount as { low: number }).low
        : missingCount as number;

      expect(countValue).toBe(0);
    });

    it('GENERATED relation should link Prompt to correct Output type', async () => {
      // PagePrompt should only generate PageL10n
      const pageResult = await session.run(`
        MATCH (pp:PagePrompt)-[:GENERATED]->(output)
        WHERE NOT output:PageL10n
        RETURN pp.display_name AS prompt, labels(output)[0] AS output_type
      `);

      // BlockPrompt should only generate BlockL10n
      const blockResult = await session.run(`
        MATCH (bp:BlockPrompt)-[:GENERATED]->(output)
        WHERE NOT output:BlockL10n
        RETURN bp.display_name AS prompt, labels(output)[0] AS output_type
      `);

      expect(pageResult.records.length).toBe(0);
      expect(blockResult.records.length).toBe(0);
    });
  });
});
