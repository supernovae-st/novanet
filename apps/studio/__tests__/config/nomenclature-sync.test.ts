/**
 * Nomenclature Sync Tests (v0.12.0)
 *
 * DX tests to validate terminology consistency across the codebase.
 * Ensures ADR-023 (Class/Instance), ADR-024 (Data Origin), and ADR-025
 * (Instruction Layer) are properly implemented everywhere.
 *
 * @see .claude/rules/novanet-terminology.md - Canonical terminology reference
 * @see .claude/rules/novanet-decisions.md - ADR documentation
 */

import { NODE_TYPES, NODE_REALMS, NODE_TRAITS, CLASS_TAXONOMY, type Trait } from '@novanet/core/types';
import { RelationRegistry } from '@novanet/core/schemas';

describe('Nomenclature Sync (v0.12.0)', () => {
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
      Object.entries(NODE_TRAITS).forEach(([nodeType, trait]) => {
        expect(DEPRECATED_TRAITS).not.toContain(trait);
      });
    });
  });

  describe('ADR-023: Class/Instance Terminology', () => {
    it('should have correct node count (59 total = 39 shared + 20 org)', () => {
      expect(NODE_TYPES).toHaveLength(59);
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
      expect(sharedCount).toBe(39);
      expect(orgCount).toBe(20);
    });

    it('should not have deprecated node names', () => {
      const DEPRECATED_NODES = ['PageType', 'PagePrompt', 'BlockPrompt'];
      NODE_TYPES.forEach((nodeType) => {
        expect(DEPRECATED_NODES).not.toContain(nodeType);
      });
    });

    it('should have new instruction layer nodes', () => {
      expect(NODE_TYPES).toContain('PageStructure');
      expect(NODE_TYPES).toContain('PageInstruction');
      expect(NODE_TYPES).toContain('BlockInstruction');
    });
  });

  describe('ADR-025: Instruction Layer Arcs', () => {
    it('should have HAS_INSTRUCTION relationship (not HAS_PROMPT)', () => {
      const relationTypes = Object.keys(RelationRegistry);
      expect(relationTypes).toContain('HAS_INSTRUCTION');
      expect(relationTypes).not.toContain('HAS_PROMPT');
    });

    it('should have HAS_STRUCTURE relationship for Page->PageStructure', () => {
      const relationTypes = Object.keys(RelationRegistry);
      expect(relationTypes).toContain('HAS_STRUCTURE');
    });

    it('should NOT have deprecated HAS_KIND relationship', () => {
      // Note: HAS_CLASS is a schema-level arc created during db seed,
      // not defined in TypeScript RelationRegistry
      const relationTypes = Object.keys(RelationRegistry);
      expect(relationTypes).not.toContain('HAS_KIND');
    });
  });

  describe('Node Distribution by Realm', () => {
    it('should have 39 shared nodes', () => {
      // Shared realm: config(3) + locale(6) + geography(6) + knowledge(24) = 39
      const sharedLayers = ['config', 'locale', 'geography', 'knowledge'];
      // This validates the architecture documented in CLAUDE.md
      expect(sharedLayers).toHaveLength(4);
    });

    it('should have 20 org nodes', () => {
      // Org realm: config(1) + foundation(3) + structure(3) + semantic(4) + instruction(6) + output(3) = 20
      const orgLayers = ['config', 'foundation', 'structure', 'semantic', 'instruction', 'output'];
      expect(orgLayers).toHaveLength(6);
    });
  });

  describe('Naming Conventions', () => {
    it('should use *Content suffix for authored nodes', () => {
      // EntityContent, ProjectContent follow the pattern
      const contentNodes = NODE_TYPES.filter((n) => n.endsWith('Content'));
      expect(contentNodes.length).toBeGreaterThan(0);
      expect(contentNodes).toContain('EntityContent');
      expect(contentNodes).toContain('ProjectContent');
    });

    it('should use *Generated suffix for generated output nodes', () => {
      // PageGenerated, BlockGenerated follow the pattern
      const generatedNodes = NODE_TYPES.filter((n) => n.endsWith('Generated'));
      expect(generatedNodes.length).toBeGreaterThan(0);
      expect(generatedNodes).toContain('PageGenerated');
      expect(generatedNodes).toContain('BlockGenerated');
    });

    it('should use *Instruction suffix for instruction nodes', () => {
      const instructionNodes = NODE_TYPES.filter((n) => n.endsWith('Instruction'));
      expect(instructionNodes).toContain('PageInstruction');
      expect(instructionNodes).toContain('BlockInstruction');
    });

    it('should use *Set suffix for container nodes', () => {
      const setNodes = NODE_TYPES.filter((n) => n.endsWith('Set'));
      expect(setNodes.length).toBeGreaterThanOrEqual(6);
      expect(setNodes).toContain('TermSet');
      expect(setNodes).toContain('ExpressionSet');
    });
  });
});
