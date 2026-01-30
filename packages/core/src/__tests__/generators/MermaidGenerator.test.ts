// src/__tests__/generators/MermaidGenerator.test.ts
// TDD: Tests FIRST for MermaidGenerator
// Generates Mermaid flowchart from relations.yaml and _index.yaml
import { describe, it, expect, beforeAll } from 'vitest';
import { MermaidGenerator, MermaidGeneratorConfig } from '../../generators/MermaidGenerator.js';
import * as path from 'path';

const RELATIONS_PATH = path.join(process.cwd(), 'models/relations.yaml');
const INDEX_PATH = path.join(process.cwd(), 'models/_index.yaml');

describe('MermaidGenerator', () => {
  // ═══════════════════════════════════════════════════════════════════════════
  // BASIC OUTPUT STRUCTURE
  // ═══════════════════════════════════════════════════════════════════════════

  describe('generate() output structure', () => {
    let mermaid: string;

    beforeAll(async () => {
      mermaid = await MermaidGenerator.generate({
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      });
    });

    it('should return valid flowchart TB declaration', () => {
      expect(mermaid).toMatch(/^flowchart TB\n/);
    });

    it('should include all 5 classDef declarations for locale behaviors', () => {
      // invariant: blue
      expect(mermaid).toContain('classDef invariant fill:#3b82f6,stroke:#1d4ed8,color:#fff');
      // localized: green
      expect(mermaid).toContain('classDef localized fill:#22c55e,stroke:#16a34a,color:#fff');
      // localeKnowledge: purple
      expect(mermaid).toContain('classDef localeKnowledge fill:#8b5cf6,stroke:#7c3aed,color:#fff');
      // derived: gray
      expect(mermaid).toContain('classDef derived fill:#9ca3af,stroke:#6b7280,color:#fff');
      // job: dark gray
      expect(mermaid).toContain('classDef job fill:#6b7280,stroke:#4b5563,color:#fff');
    });

    it('should include subgraphs for each scope (Global, Shared, Project)', () => {
      // Using _LAYER suffix to avoid collision with node names
      expect(mermaid).toContain('subgraph GLOBAL_LAYER');
      expect(mermaid).toContain('subgraph SHARED_LAYER');
      expect(mermaid).toContain('subgraph PROJECT_LAYER');
    });

    it('should close all subgraphs with end keyword', () => {
      const subgraphCount = (mermaid.match(/subgraph /g) || []).length;
      const endCount = (mermaid.match(/^\s*end$/gm) || []).length;
      expect(endCount).toBeGreaterThanOrEqual(subgraphCount);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // NODE CLASSIFICATION BY SCOPE
  // ═══════════════════════════════════════════════════════════════════════════

  describe('nodes grouped by scope', () => {
    let mermaid: string;

    beforeAll(async () => {
      mermaid = await MermaidGenerator.generate({
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      });
    });

    it('should place Locale in Global scope', () => {
      // With nested subgraphs using _LAYER suffix to avoid node name collision
      expect(mermaid).toContain('subgraph GLOBAL_LAYER["🌍 GLOBAL"]');
      expect(mermaid).toMatch(/subgraph GLOBAL_config.*Locale\[/s);
    });

    it('should place LocaleKnowledge nodes in Global scope', () => {
      // Sample of locale knowledge nodes in Global scope
      expect(mermaid).toMatch(/subgraph GLOBAL_knowledge.*LocaleVoice\[/s);
      expect(mermaid).toMatch(/subgraph GLOBAL_knowledge.*LocaleCulture\[/s);
      expect(mermaid).toMatch(/subgraph GLOBAL_knowledge.*Expression\[/s);
    });

    it('should place SEO/GEO nodes in Shared scope', () => {
      expect(mermaid).toContain('subgraph SHARED_LAYER["🎯 SHARED"]');
      expect(mermaid).toMatch(/subgraph SHARED_seo.*SEOKeywordL10n\[/s);
      expect(mermaid).toMatch(/subgraph SHARED_seo.*SEOMiningRun\[/s);
      expect(mermaid).toMatch(/subgraph SHARED_geo.*GEOSeedL10n\[/s);
      expect(mermaid).toMatch(/subgraph SHARED_geo.*GEOMiningRun\[/s);
    });

    it('should place Project, Page, Block, Concept in Project scope', () => {
      expect(mermaid).toContain('subgraph PROJECT_LAYER["📦 PROJECT"]');
      expect(mermaid).toMatch(/subgraph PROJECT_foundation.*Project\[/s);
      expect(mermaid).toMatch(/subgraph PROJECT_structure.*Page\[/s);
      expect(mermaid).toMatch(/subgraph PROJECT_structure.*Block\[/s);
      expect(mermaid).toMatch(/subgraph PROJECT_semantic.*Concept\[/s);
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // NODE STYLING BY LOCALE BEHAVIOR
  // ═══════════════════════════════════════════════════════════════════════════

  describe('nodes styled by locale behavior', () => {
    let mermaid: string;

    beforeAll(async () => {
      mermaid = await MermaidGenerator.generate({
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      });
    });

    it('should apply invariant class to invariant nodes', () => {
      // Invariant nodes: Project, BrandIdentity, Concept, Page, Block, etc.
      expect(mermaid).toContain('class Project invariant');
      expect(mermaid).toContain('class BrandIdentity invariant');
      expect(mermaid).toContain('class Concept invariant');
      expect(mermaid).toContain('class Page invariant');
      expect(mermaid).toContain('class Block invariant');
      expect(mermaid).toContain('class Locale invariant');
    });

    it('should apply localized class to localized nodes', () => {
      // Localized nodes: ProjectL10n, ConceptL10n, PageL10n, BlockL10n, SEOKeywordL10n, GEOSeedL10n
      expect(mermaid).toContain('class ProjectL10n localized');
      expect(mermaid).toContain('class ConceptL10n localized');
      expect(mermaid).toContain('class PageL10n localized');
      expect(mermaid).toContain('class BlockL10n localized');
      expect(mermaid).toContain('class SEOKeywordL10n localized');
      expect(mermaid).toContain('class GEOSeedL10n localized');
    });

    it('should apply localeKnowledge class to locale knowledge nodes', () => {
      expect(mermaid).toContain('class LocaleIdentity localeKnowledge');
      expect(mermaid).toContain('class LocaleVoice localeKnowledge');
      expect(mermaid).toContain('class LocaleCulture localeKnowledge');
      expect(mermaid).toContain('class Expression localeKnowledge');
    });

    it('should apply derived class to derived nodes', () => {
      expect(mermaid).toContain('class SEOKeywordMetrics derived');
      expect(mermaid).toContain('class GEOSeedMetrics derived');
    });

    it('should apply job class to job nodes', () => {
      expect(mermaid).toContain('class SEOMiningRun job');
      expect(mermaid).toContain('class GEOMiningRun job');
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // EDGE RENDERING
  // ═══════════════════════════════════════════════════════════════════════════

  describe('edge rendering', () => {
    let mermaid: string;

    beforeAll(async () => {
      mermaid = await MermaidGenerator.generate({
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      });
    });

    it('should render 66 edges (67 total minus 1 wildcard FOR_LOCALE)', () => {
      // Count edges by pattern: NodeA ARROW|RELATION| NodeB
      // RelationsParser returns 67 edges, but FOR_LOCALE has from: "*" which is filtered out
      // Arrow patterns: --> (solid), -.-> (dashed), ==> (thick), --o (circle)
      // Regex matches any arrow style with relation label
      const edgeMatches = mermaid.match(/\w+ (?:-->|-.->|==>|--o)\|[A-Z0-9_]+\| \w+/g) || [];
      expect(edgeMatches.length).toBe(66);
    });

    it('should render edges with relation labels', () => {
      // Example edges with labels
      expect(mermaid).toContain('Project -->|HAS_CONCEPT| Concept');
      expect(mermaid).toContain('Project -->|HAS_PAGE| Page');
      expect(mermaid).toContain('Page -->|HAS_BLOCK| Block');
    });

    it('should handle self-referential edges (e.g., Concept->Concept)', () => {
      // SEMANTIC_LINK is Concept -> Concept (semantic category → dashed arrow)
      expect(mermaid).toContain('Concept -.->|SEMANTIC_LINK| Concept');
    });

    it('should handle Locale->Locale edges (FALLBACK_TO, VARIANT_OF)', () => {
      // localization category → dashed arrow
      expect(mermaid).toContain('Locale -.->|FALLBACK_TO| Locale');
      expect(mermaid).toContain('Locale -.->|VARIANT_OF| Locale');
    });

    it('should handle Page->Page edges (LINKS_TO, SUBTOPIC_OF)', () => {
      // LINKS_TO is semantic → dashed, SUBTOPIC_OF is hierarchy → solid
      expect(mermaid).toContain('Page -.->|LINKS_TO| Page');
      expect(mermaid).toContain('Page -->|SUBTOPIC_OF| Page');
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // EDGE CASES: WILDCARD AND SPECIAL RELATIONS
  // ═══════════════════════════════════════════════════════════════════════════

  describe('wildcard handling', () => {
    let mermaid: string;

    beforeAll(async () => {
      mermaid = await MermaidGenerator.generate({
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      });
    });

    it('should NOT render edges with wildcard * (FOR_LOCALE uses *)', () => {
      // FOR_LOCALE has from: "*" which is too generic to render
      // We should skip wildcard edges in the diagram
      expect(mermaid).not.toContain('* -->');
      expect(mermaid).not.toContain('--> *');
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // ERROR HANDLING
  // ═══════════════════════════════════════════════════════════════════════════

  describe('error handling', () => {
    it('should throw descriptive error for missing relations file', async () => {
      await expect(
        MermaidGenerator.generate({
          relationsPath: '/non/existent/relations.yaml',
          indexPath: INDEX_PATH,
        })
      ).rejects.toThrow('MermaidGenerator: Failed to load relations');
    });

    it('should throw descriptive error for missing index file', async () => {
      await expect(
        MermaidGenerator.generate({
          relationsPath: RELATIONS_PATH,
          indexPath: '/non/existent/_index.yaml',
        })
      ).rejects.toThrow('MermaidGenerator: Failed to load index');
    });

    it('should throw for empty config paths', async () => {
      await expect(
        MermaidGenerator.generate({
          relationsPath: '',
          indexPath: INDEX_PATH,
        })
      ).rejects.toThrow('MermaidGenerator: relationsPath cannot be empty');

      await expect(
        MermaidGenerator.generate({
          relationsPath: RELATIONS_PATH,
          indexPath: '',
        })
      ).rejects.toThrow('MermaidGenerator: indexPath cannot be empty');
    });
  });

  // ═══════════════════════════════════════════════════════════════════════════
  // INTEGRATION: COMPLETE OUTPUT VALIDATION
  // ═══════════════════════════════════════════════════════════════════════════

  describe('integration: complete Mermaid output', () => {
    it('should generate valid Mermaid syntax (no duplicate node declarations)', async () => {
      const mermaid = await MermaidGenerator.generate({
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      });

      // Each node should only be declared once in class assignments
      const classAssignments = mermaid.match(/class \w+ \w+/g) || [];
      const nodeNames = classAssignments.map(c => c.split(' ')[1]);
      const uniqueNodes = new Set(nodeNames);
      expect(nodeNames.length).toBe(uniqueNodes.size);
    });

    it('should include all 35 nodes from _index.yaml', async () => {
      const mermaid = await MermaidGenerator.generate({
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      });

      // Count class assignments (one per node)
      const classAssignments = mermaid.match(/class \w+ \w+/g) || [];
      expect(classAssignments.length).toBe(35);
    });

    it('should be deterministic (same output for same input)', async () => {
      const config: MermaidGeneratorConfig = {
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      };

      const output1 = await MermaidGenerator.generate(config);
      const output2 = await MermaidGenerator.generate(config);

      expect(output1).toBe(output2);
    });

    it('should not have subgraph names that collide with node names', async () => {
      const mermaid = await MermaidGenerator.generate({
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      });

      // Extract subgraph IDs (first word after "subgraph")
      const subgraphMatches = mermaid.match(/subgraph\s+(\w+)/g) || [];
      const subgraphIds = subgraphMatches.map(m => m.replace('subgraph ', ''));

      // Extract node names from class assignments
      const classMatches = mermaid.match(/class\s+(\w+)\s+\w+/g) || [];
      const nodeNames = classMatches.map(m => m.split(' ')[1]);

      // No subgraph ID should match a node name (would cause cycle error)
      for (const subgraphId of subgraphIds) {
        expect(nodeNames).not.toContain(subgraphId);
      }
    });

    it('should use _LAYER suffix for scope subgraphs to avoid collisions', async () => {
      const mermaid = await MermaidGenerator.generate({
        relationsPath: RELATIONS_PATH,
        indexPath: INDEX_PATH,
      });

      // Scope subgraphs should use _LAYER suffix
      expect(mermaid).toContain('subgraph GLOBAL_LAYER');
      expect(mermaid).toContain('subgraph SHARED_LAYER');
      expect(mermaid).toContain('subgraph PROJECT_LAYER');

      // Should NOT have plain scope names that could collide
      expect(mermaid).not.toMatch(/subgraph Global[^_]/);
      expect(mermaid).not.toMatch(/subgraph Shared[^_]/);
      expect(mermaid).not.toMatch(/subgraph Project[^_]/);
    });
  });
});
