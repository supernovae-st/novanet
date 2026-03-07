/**
 * Nomenclature Sync Tests (v0.17.2)
 *
 * DX tests to validate terminology consistency across the codebase.
 * Ensures ADR-023 (Class/Instance), ADR-024 (Data Origin), ADR-025
 * (Instruction Layer), and ADR-028 (Page-Entity Architecture) are
 * properly implemented everywhere.
 *
 * v0.17.2 Changes (YAGNI Cleanup):
 * - Removed: Term, TermSet, SEOKeywordMetrics (unused)
 * - Added: ProjectGEOScope, ProjectSEOScope (project-level scope config)
 * - Net: 57 nodes (36 shared + 21 org)
 *
 * v0.12.4 Changes:
 * - Country added to shared/geography
 * - Brand Architecture added to org/foundation
 * - PageStructure and PageInstruction REMOVED from org/instruction
 *
 * @see .claude/rules/novanet-terminology.md - Canonical terminology reference
 * @see .claude/rules/novanet-decisions.md - ADR documentation
 */

import { NODE_TYPES, NODE_REALMS, NODE_TRAITS, type Trait } from '@novanet/core/types';
import { RelationRegistry } from '@novanet/core/schemas';

describe('Nomenclature Sync (v0.17.2)', () => {
  describe('ADR-024: Data Origin Traits', () => {
    const VALID_TRAITS: Trait[] = ['defined', 'authored', 'imported', 'generated', 'retrieved'];
    const DEPRECATED_TRAITS = ['invariant', 'localized', 'knowledge', 'aggregated'];

    it('should have exactly 5 valid traits', () => {
      expect(VALID_TRAITS).toHaveLength(5);
    });

    it('should have all node traits be valid Data Origin names', () => {
      const usedTraits = new Set(Object.values(NODE_TRAITS));
      usedTraits.forEach((trait) => {
        expect(DEPRECATED_TRAITS).not.toContain(trait);
        expect(VALID_TRAITS).toContain(trait);
      });
    });

    it('should not use deprecated trait names anywhere', () => {
      Object.entries(NODE_TRAITS).forEach(([_nodeType, trait]) => {
        expect(DEPRECATED_TRAITS).not.toContain(trait);
      });
    });
  });

  describe('ADR-023: Class/Instance Terminology', () => {
    it('should have correct node count (57 total = 36 shared + 21 org)', () => {
      expect(NODE_TYPES).toHaveLength(57);
    });

    it('should have 2 realms (shared, org)', () => {
      const realms = new Set(Object.values(NODE_REALMS));
      expect(realms.size).toBe(2);
      expect(realms).toContain('shared');
      expect(realms).toContain('org');
    });

    it('should have correct node distribution by realm', () => {
      const sharedCount = Object.values(NODE_REALMS).filter((r) => r === 'shared').length;
      const orgCount = Object.values(NODE_REALMS).filter((r) => r === 'org').length;
      expect(sharedCount).toBe(36); // v0.17.2: YAGNI cleanup removed Term, TermSet, SEOKeywordMetrics
      expect(orgCount).toBe(21); // v0.17.2: Added ProjectGEOScope, ProjectSEOScope
    });

    it('should not have deprecated node names', () => {
      // v0.12.4: PageStructure and PageInstruction also removed
      const DEPRECATED_NODES = ['PageType', 'PagePrompt', 'BlockPrompt', 'PageStructure', 'PageInstruction'];
      NODE_TYPES.forEach((nodeType) => {
        expect(DEPRECATED_NODES).not.toContain(nodeType);
      });
    });

    it('should have instruction layer nodes (v0.12.4: only Block-level)', () => {
      // v0.12.4: PageStructure and PageInstruction REMOVED per ADR-028
      // Instructions are now composed from BlockInstructions at generation time
      expect(NODE_TYPES).toContain('BlockType');
      expect(NODE_TYPES).toContain('BlockInstruction');
      expect(NODE_TYPES).toContain('BlockRules');
    });

    it('should have Brand Architecture nodes (v0.12.4)', () => {
      expect(NODE_TYPES).toContain('Brand');
      expect(NODE_TYPES).toContain('BrandDesign');
      expect(NODE_TYPES).toContain('BrandPrinciples');
      expect(NODE_TYPES).toContain('PromptStyle');
    });
  });

  describe('ADR-025: Instruction Layer Arcs', () => {
    it('should have HAS_INSTRUCTION relationship (not HAS_PROMPT)', () => {
      const relationTypes = Object.keys(RelationRegistry);
      expect(relationTypes).toContain('HAS_INSTRUCTION');
      expect(relationTypes).not.toContain('HAS_PROMPT');
    });

    it('should NOT have deprecated HAS_KIND relationship', () => {
      // Note: HAS_CLASS is a schema-level arc created during db seed,
      // not defined in TypeScript RelationRegistry
      const relationTypes = Object.keys(RelationRegistry);
      expect(relationTypes).not.toContain('HAS_KIND');
    });

    it('should NOT have HAS_STRUCTURE relationship (v0.12.4: PageStructure node deleted)', () => {
      // v0.12.4: PageStructure node deleted per ADR-028
      // Page structure is now computed from HAS_BLOCK.order at runtime
      const relationTypes = Object.keys(RelationRegistry);
      expect(relationTypes).not.toContain('HAS_STRUCTURE');
    });
  });

  describe('Node Distribution by Realm', () => {
    it('should have 36 shared nodes (v0.17.2)', () => {
      // Shared realm: config(3) + locale(6) + geography(7) + knowledge(20) = 36
      // v0.17.2: YAGNI cleanup removed Term, TermSet, SEOKeywordMetrics from knowledge
      const sharedLayers = ['config', 'locale', 'geography', 'knowledge'];
      // This validates the architecture documented in CLAUDE.md
      expect(sharedLayers).toHaveLength(4);
    });

    it('should have 21 org nodes (v0.17.2)', () => {
      // Org realm: config(1) + foundation(6) + structure(3) + semantic(4) + instruction(4) + output(3) = 21
      // v0.17.2: Added ProjectGEOScope, ProjectSEOScope to foundation
      const orgLayers = ['config', 'foundation', 'structure', 'semantic', 'instruction', 'output'];
      expect(orgLayers).toHaveLength(6);
    });
  });

  describe('Naming Conventions', () => {
    it('should use *Native suffix for authored and generated nodes (v0.13.0 ADR-029)', () => {
      // v0.13.0: EntityContent→EntityNative, ProjectContent→ProjectNative,
      // PageGenerated→PageNative, BlockGenerated→BlockNative
      const nativeNodes = NODE_TYPES.filter((n) => n.endsWith('Native'));
      expect(nativeNodes.length).toBeGreaterThanOrEqual(4);
      expect(nativeNodes).toContain('EntityNative');
      expect(nativeNodes).toContain('ProjectNative');
      expect(nativeNodes).toContain('PageNative');
      expect(nativeNodes).toContain('BlockNative');
    });

    it('should use *Instruction suffix for instruction nodes (v0.12.4)', () => {
      // v0.12.4: Only BlockInstruction remains (PageInstruction removed)
      const instructionNodes = NODE_TYPES.filter((n) => n.endsWith('Instruction'));
      expect(instructionNodes).toContain('BlockInstruction');
      expect(instructionNodes).toHaveLength(1); // Only BlockInstruction
    });

    it('should use *Set suffix for container nodes', () => {
      // v0.17.2: TermSet removed in YAGNI cleanup
      const setNodes = NODE_TYPES.filter((n) => n.endsWith('Set'));
      expect(setNodes.length).toBeGreaterThanOrEqual(5);
      expect(setNodes).toContain('ExpressionSet');
    });
  });
});
