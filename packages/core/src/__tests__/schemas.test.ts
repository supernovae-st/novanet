// NovaNet Core - Schema Tests v7.0.0
// TDD: Verify Zod schemas validate correctly

import { describe, it, expect } from 'vitest';
import {
  LocaleSchema,
  LocaleVoiceSchema,
  ExpressionSchema,
} from '../schemas/locale-knowledge.schema.js';
import { RelationType } from '../schemas/relations.schema.js';

describe('Locale Knowledge Schemas', () => {
  describe('LocaleSchema', () => {
    it('should validate valid locale data with all required fields', () => {
      const validLocale = {
        code: 'fr-FR',
        language_code: 'fr',
        country_code: 'FR',
        name_english: 'French (France)',
        name_native: 'Français (France)',
        is_primary: true,
        fallback_chain: ['en-US'],
        created_at: new Date(),
        updated_at: new Date(),
      };

      const result = LocaleSchema.safeParse(validLocale);
      expect(result.success).toBe(true);
    });

    it('should reject invalid locale code format', () => {
      const invalidLocale = {
        code: 'invalid',  // Should be xx-XX format
        language_code: 'fr',
        country_code: 'FR',
        name_english: 'French',
        name_native: 'Français',
        is_primary: true,
        fallback_chain: [],
        created_at: new Date(),
        updated_at: new Date(),
      };

      const result = LocaleSchema.safeParse(invalidLocale);
      expect(result.success).toBe(false);
    });

    it('should reject missing required fields', () => {
      const incompleteLocale = {
        code: 'fr-FR',
        language_code: 'fr',
        // Missing other required fields
      };

      const result = LocaleSchema.safeParse(incompleteLocale);
      expect(result.success).toBe(false);
    });
  });

  describe('LocaleVoiceSchema', () => {
    it('should validate voice characteristics with flat structure', () => {
      const voice = {
        formality_score: 75,
        default_formality: 'formal' as const,
        default_pronoun: 'vous',
        pronoun_rules: { b2b: 'vous' },
        directness_score: 45,
        directness_style: 'indirect' as const,
        softening_patterns: { command: 'Pourriez-vous...' },
        warmth_score: 60,
        warmth_by_stage: { awareness: 50 },
        humor_score: 50,
        humor_types: { wordplay: 'occasional' },
        avg_sentence_length: 20,
        preferred_voice: 'active' as const,
        rhythm_style: 'balanced',
        punctuation_rules: { colon: 'space before' },
        created_at: new Date(),
        updated_at: new Date(),
      };

      const result = LocaleVoiceSchema.safeParse(voice);
      expect(result.success).toBe(true);
    });

    it('should enforce formality score range 0-100', () => {
      const invalidVoice = {
        formality_score: 150, // Invalid: > 100
        default_formality: 'formal',
        default_pronoun: null,
        pronoun_rules: {},
        directness_score: 50,
        directness_style: 'direct',
        softening_patterns: {},
        warmth_score: 50,
        warmth_by_stage: {},
        humor_score: 50,
        humor_types: {},
        avg_sentence_length: 15,
        preferred_voice: 'active',
        rhythm_style: 'balanced',
        punctuation_rules: {},
        created_at: new Date(),
        updated_at: new Date(),
      };

      const result = LocaleVoiceSchema.safeParse(invalidVoice);
      expect(result.success).toBe(false);
    });
  });

  describe('ExpressionSchema', () => {
    it('should validate expression with all required fields', () => {
      const expression = {
        semantic_field: 'success',
        intention: 'encouragement',
        text: "C'est parti !",
        register: 'casual' as const,
        context: 'CTA buttons, onboarding',
        example_sentence: 'Votre QR code est prêt. C\'est parti !',
        created_at: new Date(),
        updated_at: new Date(),
      };

      const result = ExpressionSchema.safeParse(expression);
      expect(result.success).toBe(true);
    });

    it('should enforce register enum values', () => {
      const expression = {
        semantic_field: 'success',
        intention: 'encouragement',
        text: "C'est parti !",
        register: 'invalid', // Invalid: not formal/semi-formal/casual
        context: 'CTA buttons',
        example_sentence: 'Example',
        created_at: new Date(),
        updated_at: new Date(),
      };

      const result = ExpressionSchema.safeParse(expression);
      expect(result.success).toBe(false);
    });

    it('should reject expression with missing text', () => {
      const expression = {
        semantic_field: 'success',
        intention: 'encouragement',
        register: 'casual',
        context: 'CTA buttons',
        example_sentence: 'Example',
        created_at: new Date(),
        updated_at: new Date(),
        // Missing text field
      };

      const result = ExpressionSchema.safeParse(expression);
      expect(result.success).toBe(false);
    });
  });
});

describe('Relation Registry', () => {
  describe('RelationType enum', () => {
    it('should have all v10.3 core relations', () => {
      // Project root (v10.3: HAS_CONCEPT removed)
      expect(RelationType.HAS_PAGE).toBe('HAS_PAGE');
      expect(RelationType.SUPPORTS_LOCALE).toBe('SUPPORTS_LOCALE');

      // Locale
      expect(RelationType.FALLBACK_TO).toBe('FALLBACK_TO');
      expect(RelationType.FOR_LOCALE).toBe('FOR_LOCALE');

      // Locale Knowledge
      expect(RelationType.HAS_IDENTITY).toBe('HAS_IDENTITY');
      expect(RelationType.HAS_VOICE).toBe('HAS_VOICE');
      expect(RelationType.HAS_CULTURE).toBe('HAS_CULTURE');
      expect(RelationType.HAS_MARKET).toBe('HAS_MARKET');
      expect(RelationType.HAS_LEXICON).toBe('HAS_LEXICON');

      // v7.0.0 unified relations
      expect(RelationType.HAS_CONTENT).toBe('HAS_CONTENT');
      expect(RelationType.HAS_GENERATED).toBe('HAS_GENERATED');
      expect(RelationType.USES_ENTITY).toBe('USES_ENTITY');

      // SEO/GEO
      expect(RelationType.TARGETS_SEO).toBe('TARGETS_SEO');
      expect(RelationType.TARGETS_GEO).toBe('TARGETS_GEO');
    });

    it('should use UPPER_SNAKE_CASE naming convention', () => {
      const relations = Object.keys(RelationType);

      relations.forEach((rel) => {
        expect(rel).toMatch(/^[A-Z][A-Z0-9_]*$/);
      });
    });

    it('should have mining relations for SEO/GEO targeting', () => {
      // v11.5: SEO_MINES/GEO_MINES removed — SEO/GEO nodes moved to shared/knowledge
      // Mining relations are now TARGETS_SEO and TARGETS_GEO
      expect(RelationType.TARGETS_SEO).toBe('TARGETS_SEO');
      expect(RelationType.TARGETS_GEO).toBe('TARGETS_GEO');
    });

    it('should have provenance relations', () => {
      expect(RelationType.INFLUENCED_BY).toBe('INFLUENCED_BY');
      // REMOVED v7.9.0: USED_SEO_KEYWORD, USED_GEO_SEED (SEO/GEO is at ConceptL10n level)
      expect(RelationType.GENERATED_FROM).toBe('GENERATED_FROM');
    });
  });

  describe('Relation Naming Conventions', () => {
    it('should use HAS_* for ownership/composition', () => {
      const hasRelations = Object.keys(RelationType).filter((r) => r.startsWith('HAS_'));

      // v10.3: HAS_CONCEPT removed — Entity in org realm
      expect(hasRelations).toContain('HAS_PAGE');
      expect(hasRelations).toContain('HAS_BLOCK');
      expect(hasRelations).toContain('HAS_CONTENT');
      expect(hasRelations).toContain('HAS_GENERATED');
      expect(hasRelations).toContain('HAS_IDENTITY');
      expect(hasRelations).toContain('HAS_VOICE');
      expect(hasRelations.length).toBeGreaterThan(10);
    });

    it('should use TARGETS_* for targeting relations', () => {
      const targetsRelations = Object.keys(RelationType).filter((r) => r.startsWith('TARGETS_'));

      expect(targetsRelations).toContain('TARGETS_SEO');
      expect(targetsRelations).toContain('TARGETS_GEO');
    });

    it('should use TARGETS_* for SEO/GEO mining relations', () => {
      // v11.5: *_MINES patterns removed — SEO/GEO now in shared/knowledge
      // Mining relations use TARGETS_* pattern
      const targetsRelations = Object.keys(RelationType).filter((r) => r.startsWith('TARGETS_'));

      expect(targetsRelations).toContain('TARGETS_SEO');
      expect(targetsRelations).toContain('TARGETS_GEO');
    });

    it('should not have inconsistent verb tenses', () => {
      const relations = Object.keys(RelationType);

      // Check for mixed tenses (e.g., USES vs USED)
      const usesVariants = relations.filter((r) => r.includes('USE'));
      // USES_ENTITY is active (v10.3: renamed from USES_CONCEPT)
      expect(usesVariants).toContain('USES_ENTITY');
    });
  });
});

describe('Relation Naming Conventions', () => {
  it('should use HAS_* for ownership/composition', () => {
    const hasRelations = Object.keys(RelationType).filter((r) => r.startsWith('HAS_'));

    // v10.3: HAS_CONCEPT removed — Entity in org realm
    expect(hasRelations).toContain('HAS_PAGE');
    expect(hasRelations).toContain('HAS_BLOCK');
    expect(hasRelations).toContain('HAS_CONTENT');
    expect(hasRelations).toContain('HAS_GENERATED');
    expect(hasRelations).toContain('HAS_IDENTITY');
    expect(hasRelations).toContain('HAS_VOICE');
  });

  it('should use TARGETS_* for targeting relations', () => {
    const targetsRelations = Object.keys(RelationType).filter((r) => r.startsWith('TARGETS_'));

    expect(targetsRelations).toContain('TARGETS_SEO');
    expect(targetsRelations).toContain('TARGETS_GEO');
  });

  it('should have TARGETS_* for SEO/GEO relations', () => {
    // v11.5: *_MINES patterns removed — SEO/GEO now in shared/knowledge
    expect(RelationType.TARGETS_SEO).toBe('TARGETS_SEO');
    expect(RelationType.TARGETS_GEO).toBe('TARGETS_GEO');
  });
});

// ═══════════════════════════════════════════════════════════════════════════════
// INSTRUCTION SCHEMAS
// ═══════════════════════════════════════════════════════════════════════════════

import { PageInstructionSchema, BlockInstructionSchema, BlockRulesSchema } from '../schemas/prompts.schema.js';

describe('Instruction Schemas (v11.8.0)', () => {
  describe('PageInstructionSchema', () => {
    it('validates valid PageInstruction', () => {
      const valid = {
        display_name: 'Pricing Page Instruction v1.0',
        description: 'Instructions for pricing page generation',
        llm_context: 'USE: orchestration. TRIGGERS: page. NOT: blocks.',
        instruction: '[GENERATE] Create conversion-focused pricing page',
        version: '1.0',
        active: true,
        created_at: new Date(),
        updated_at: new Date(),
      };
      expect(PageInstructionSchema.parse(valid)).toBeDefined();
    });

    it('rejects empty instruction', () => {
      const invalid = {
        display_name: 'Test',
        description: 'Test',
        llm_context: 'USE: x. TRIGGERS: y. NOT: z.',
        instruction: '',
        version: '1.0',
        active: true,
        created_at: new Date(),
        updated_at: new Date(),
      };
      expect(() => PageInstructionSchema.parse(invalid)).toThrow();
    });

    it('rejects invalid llm_context format', () => {
      const invalid = {
        display_name: 'Test',
        description: 'Test',
        llm_context: 'Invalid format without USE/TRIGGERS/NOT',
        instruction: '[GENERATE] Test',
        version: '1.0',
        active: true,
        created_at: new Date(),
        updated_at: new Date(),
      };
      expect(() => PageInstructionSchema.parse(invalid)).toThrow();
    });

    it('rejects invalid version format', () => {
      const invalid = {
        display_name: 'Test',
        description: 'Test',
        llm_context: 'USE: x. TRIGGERS: y. NOT: z.',
        instruction: '[GENERATE] Test',
        version: 'invalid-version',
        active: true,
        created_at: new Date(),
        updated_at: new Date(),
      };
      expect(() => PageInstructionSchema.parse(invalid)).toThrow();
    });

    it('accepts valid semver versions', () => {
      const validVersions = ['1.0', '1.1.0', '2.0', '10.20.30'];
      validVersions.forEach((version) => {
        const valid = {
          display_name: 'Test',
          description: 'Test',
          llm_context: 'USE: x. TRIGGERS: y. NOT: z.',
          instruction: '[GENERATE] Test',
          version,
          active: true,
          created_at: new Date(),
          updated_at: new Date(),
        };
        expect(PageInstructionSchema.parse(valid)).toBeDefined();
      });
    });
  });

  describe('BlockInstructionSchema', () => {
    it('validates valid BlockInstruction', () => {
      const valid = {
        display_name: 'Hero Instruction v1.0',
        description: 'Instructions for hero generation',
        llm_context: 'USE: hero. TRIGGERS: block. NOT: other.',
        instruction: '[GENERATE] Hero highlighting @tier-pro',
        version: '1.0',
        active: true,
        created_at: new Date(),
        updated_at: new Date(),
      };
      expect(BlockInstructionSchema.parse(valid)).toBeDefined();
    });

    it('rejects empty instruction', () => {
      const invalid = {
        display_name: 'Test',
        description: 'Test',
        llm_context: 'USE: x. TRIGGERS: y. NOT: z.',
        instruction: '',
        version: '1.0',
        active: true,
        created_at: new Date(),
        updated_at: new Date(),
      };
      expect(() => BlockInstructionSchema.parse(invalid)).toThrow();
    });
  });

  describe('BlockRulesSchema', () => {
    it('validates valid BlockRules', () => {
      const valid = {
        display_name: 'Hero Rules v1.0',
        description: 'Generation rules for hero',
        llm_context: 'USE: rules. TRIGGERS: hero. NOT: other.',
        rules: 'Title: action verb. Subtitle: value prop.',
        version: '1.0',
        active: true,
        created_at: new Date(),
        updated_at: new Date(),
      };
      expect(BlockRulesSchema.parse(valid)).toBeDefined();
    });

    it('rejects empty rules', () => {
      const invalid = {
        display_name: 'Test',
        description: 'Test',
        llm_context: 'USE: x. TRIGGERS: y. NOT: z.',
        rules: '',
        version: '1.0',
        active: true,
        created_at: new Date(),
        updated_at: new Date(),
      };
      expect(() => BlockRulesSchema.parse(invalid)).toThrow();
    });
  });
});

// ═══════════════════════════════════════════════════════════════════════════════
// RELATIONS - Instruction relations
// ═══════════════════════════════════════════════════════════════════════════════

import { RelationRegistry } from '../schemas/relations.schema.js';

describe('Relations v11.8.0', () => {
  describe('HAS_INSTRUCTION relation', () => {
    it('exists in RelationType and RelationRegistry', () => {
      expect(RelationType.HAS_INSTRUCTION).toBe('HAS_INSTRUCTION');
      expect(RelationRegistry[RelationType.HAS_INSTRUCTION]).toBeDefined();
    });

    it('links Page and Block to PageInstruction and BlockInstruction', () => {
      const rel = RelationRegistry[RelationType.HAS_INSTRUCTION];
      expect(rel.from).toContain('Page');
      expect(rel.from).toContain('Block');
      expect(rel.to).toContain('PageInstruction');
      expect(rel.to).toContain('BlockInstruction');
    });

    it('has 1:N cardinality for versioning', () => {
      const rel = RelationRegistry[RelationType.HAS_INSTRUCTION];
      expect(rel.cardinality).toBe('1:N');
    });
  });

  describe('HAS_RULES relation', () => {
    it('exists in RelationType and RelationRegistry', () => {
      expect(RelationType.HAS_RULES).toBe('HAS_RULES');
      expect(RelationRegistry[RelationType.HAS_RULES]).toBeDefined();
    });

    it('links BlockType to BlockRules', () => {
      const rel = RelationRegistry[RelationType.HAS_RULES];
      expect(rel.from).toBe('BlockType');
      expect(rel.to).toBe('BlockRules');
    });

    it('has 1:N cardinality for versioning', () => {
      const rel = RelationRegistry[RelationType.HAS_RULES];
      expect(rel.cardinality).toBe('1:N');
    });
  });

  describe('GENERATED relation', () => {
    it('exists in RelationType and RelationRegistry', () => {
      expect(RelationType.GENERATED).toBe('GENERATED');
      expect(RelationRegistry[RelationType.GENERATED]).toBeDefined();
    });

    it('links PageInstruction/BlockInstruction to PageGenerated/BlockGenerated', () => {
      const rel = RelationRegistry[RelationType.GENERATED];
      expect(rel.from).toContain('PageInstruction');
      expect(rel.from).toContain('BlockInstruction');
      expect(rel.to).toContain('PageGenerated');
      expect(rel.to).toContain('BlockGenerated');
    });

    it('has N:M cardinality for provenance', () => {
      const rel = RelationRegistry[RelationType.GENERATED];
      expect(rel.cardinality).toBe('N:M');
    });

    it('has generated_at property for timestamp', () => {
      const rel = RelationRegistry[RelationType.GENERATED];
      expect(rel.props).toBeDefined();
    });
  });

  describe('Naming conventions', () => {
    it('should include HAS_INSTRUCTION in HAS_* relations', () => {
      const hasRelations = Object.keys(RelationType).filter((r) => r.startsWith('HAS_'));
      expect(hasRelations).toContain('HAS_INSTRUCTION');
      expect(hasRelations).toContain('HAS_RULES');
    });

  });
});
