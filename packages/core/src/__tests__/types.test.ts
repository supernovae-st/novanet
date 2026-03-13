// NovaNet Core - Type Tests v0.20.0
// TDD: Verify type exports and structure
// v10.3.0: Entity-Centric Architecture (Concept → Entity, ConceptL10n → EntityNative)
// v8.2.0: Removed icon, priority, freshness from all interfaces (YAML v7.11.0 alignment)
// v0.20.0: Standard properties migrated (description+llm_context -> node_class+content+triggers+provenance)

import { describe, it, expect } from 'vitest';
import type {
  // Standard base
  StandardNodeProperties,

  // Core entities (v10.3: Entity replaces Concept)
  Entity,
  EntityNative,
  PageNative,
  BlockNative,

  // Locale
  Locale,
  LocaleVoice,

  // SEO (v10.3: GEO removed)
  SEOKeyword,

  // Relation props (v10.3: UsesEntityProps replaces UsesConceptProps)
  SemanticLinkProps,
  UsesEntityProps,
  HasBlockProps,
} from '../types/index.js';

describe('Type Exports', () => {
  describe('StandardNodeProperties', () => {
    it('should have v0.20.0 required properties (node_class+content+triggers+provenance)', () => {
      // Type assertion test - compile-time verification
      // v0.20.0: Standard properties migrated (description+llm_context -> node_class+content+triggers+provenance)
      const props: StandardNodeProperties = {
        key: 'test-key',
        display_name: 'Test Key',
        node_class: 'TestNode',
        content: 'Test node for unit tests',
        triggers: ['test', 'unit'],
        provenance: 'seed',
        created_at: new Date(),
        updated_at: new Date(),
      };

      expect(props.key).toBe('test-key');
      expect(props.display_name).toBe('Test Key');
      expect(props.content).toBeDefined();
      expect(props.triggers).toEqual(['test', 'unit']);
      expect(props.created_at).toBeDefined();
      expect(props.updated_at).toBeDefined();
    });
  });

  describe('Core Entities', () => {
    it('Entity should extend StandardNodeProperties (v10.3)', () => {
      // v10.3: Entity replaces Concept (org realm, semantic layer)
      const entity: Entity = {
        key: 'action-create-qr',
        display_name: 'Create QR Code',
        node_class: 'Entity',
        content: 'Core QR code creation entity',
        triggers: ['create', 'generate', 'qr-code'],
        provenance: 'seed',
        created_at: new Date(),
        updated_at: new Date(),
        feature_category: 'core',
        is_core: true,
      };

      expect(entity.key).toBe('action-create-qr');
      expect(entity.feature_category).toBe('core');
      expect(entity.is_core).toBe(true);
    });

    it('EntityNative should have required localization fields (v10.3)', () => {
      // v10.3: EntityNative replaces ConceptL10n (org realm, semantic layer)
      const l10n: EntityNative = {
        display_name: 'QR Code',
        node_class: 'EntityNative',
        content: 'French QR code locale content',
        triggers: ['qr-code', 'entity-native', 'fr-fr'],
        provenance: 'seed',
        title: 'Code QR',
        definition: 'Un code-barres 2D',
        version: 1,
        influence_count: 0,
        created_at: new Date(),
        updated_at: new Date(),
      };

      expect(l10n.title).toBe('Code QR');
      expect(l10n.version).toBe(1);
      expect(l10n.definition).toBe('Un code-barres 2D');
    });

    it('PageNative should have version (v8.2.0 - no icon/priority/freshness)', () => {
      // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
      const output: PageNative = {
        display_name: 'Pricing Output',
        node_class: 'PageNative',
        content: 'Generated pricing page output',
        triggers: ['page-native', 'pricing', 'output'],
        provenance: 'mcp',
        assembled: { hero: {}, pricing: {} },
        assembled_at: new Date(),
        assembler_version: '1.0.0',
        created_at: new Date(),
        updated_at: new Date(),
        version: 1,
      };

      expect(output.updated_at).toBeDefined();
      expect(output.version).toBe(1);
      expect(output.assembler_version).toBe('1.0.0');
    });

    it('BlockNative should have version (v8.2.0 - no icon/priority/freshness)', () => {
      // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
      const output: BlockNative = {
        display_name: 'Hero Output',
        node_class: 'BlockNative',
        content: 'Generated hero block output',
        triggers: ['block-native', 'hero', 'output'],
        provenance: 'mcp',
        generated: { title: 'Welcome' },
        generated_at: new Date(),
        generator_version: '1.0.0',
        created_at: new Date(),
        updated_at: new Date(),
        version: 1,
      };

      expect(output.updated_at).toBeDefined();
      expect(output.version).toBe(1);
      expect(output.generator_version).toBe('1.0.0');
    });
  });

  describe('Locale Knowledge (v8.2.0)', () => {
    it('Locale should have standard properties (no icon/priority/freshness)', () => {
      // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
      const locale: Locale = {
        key: 'fr-FR',
        display_name: 'French (France)',
        node_class: 'Locale',
        content: 'Metropolitan French locale configuration',
        triggers: ['locale', 'french', 'fr-fr'],
        provenance: 'seed',
        language_code: 'fr',
        country_code: 'FR',
        name_native: 'Français (France)',
        is_primary: true,
        fallback_chain: ['en-US'],
        created_at: new Date(),
        updated_at: new Date(),
      };

      expect(locale.key).toBe('fr-FR');
      expect(locale.language_code).toBe('fr');
      expect(locale.is_primary).toBe(true);
    });

    it('LocaleVoice should have formality settings (no icon/priority/freshness)', () => {
      // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
      const voice: LocaleVoice = {
        display_name: 'French Voice',
        node_class: 'LocaleVoice',
        content: 'Voice and tone characteristics for French locale',
        triggers: ['voice', 'tone', 'formality'],
        provenance: 'seed',
        formality_score: 75,
        default_formality: 'formal',
        default_pronoun: 'vous',
        pronoun_rules: {},
        directness_score: 45,
        directness_style: 'indirect',
        softening_patterns: {},
        warmth_score: 60,
        warmth_by_stage: {},
        humor_score: 50,
        humor_types: {},
        avg_sentence_length: 20,
        preferred_voice: 'active',
        rhythm_style: 'balanced',
        punctuation_rules: {},
        created_at: new Date(),
        updated_at: new Date(),
      };

      expect(voice.formality_score).toBe(75);
      expect(voice.default_pronoun).toBe('vous');
      expect(voice.default_formality).toBe('formal');
    });
  });

  describe('SEO Types (v10.3 - GEO removed)', () => {
    it('SEOKeyword should extend StandardNodeProperties', () => {
      const keyword: SEOKeyword = {
        key: 'creer-qr-code-fr',
        display_name: 'créer qr code',
        node_class: 'SEOKeyword',
        content: 'Primary SEO keyword for QR code creation in French',
        triggers: ['seo', 'keyword', 'qr-code', 'french'],
        provenance: 'seed',
        created_at: new Date(),
        updated_at: new Date(),
        value: 'créer qr code gratuit',
        volume: 12100,
        difficulty: 35,
        cpc: 0.85,
        intent: 'transactional',
        platform: 'google',
        source: 'ahrefs',
      };

      expect(keyword.volume).toBe(12100);
      expect(keyword.value).toBe('créer qr code gratuit');
    });

    // REMOVED v8.0.0: PageMetrics (query GA/PostHog directly with date ranges)
    // REMOVED v10.3.0: GEOSeedMetrics (GEO layer removed)
  });

  describe('Relation Props', () => {
    it('SemanticLinkProps should have temperature', () => {
      const props: SemanticLinkProps = {
        type: 'includes',
        temperature: 0.85,
      };

      expect(props.temperature).toBe(0.85);
    });

    it('UsesEntityProps should have purpose (v10.3)', () => {
      const props: UsesEntityProps = {
        purpose: 'primary',
        temperature: 0.9,
      };

      expect(props.purpose).toBe('primary');
    });

    it('HasBlockProps should have position', () => {
      const props: HasBlockProps = {
        position: 1,
      };

      expect(props.position).toBe(1);
    });
  });
});

// ═══════════════════════════════════════════════════════════════════════════════
// INSTRUCTION TYPES (v0.12.4: PageInstruction removed per ADR-028)
// ═══════════════════════════════════════════════════════════════════════════════

import type { BlockInstruction } from '../types/prompts.js';

describe('Instruction Types (v0.12.4)', () => {
  // v0.12.4: PageInstruction removed per ADR-028 - page instructions composed from BlockInstructions
  // v0.19.1: BlockRules removed — merged into BlockType.rules property

  it('BlockInstruction has required properties', () => {
    const instruction: BlockInstruction = {
      display_name: 'Pricing Hero Instruction v1.0',
      node_class: 'BlockInstruction',
      content: 'Instructions for pricing hero generation',
      triggers: ['instruction', 'hero', 'pricing'],
      provenance: 'seed',
      instruction: '[GENERATE] Hero highlighting @tier-pro benefits',
      version: '1.0',
      active: true,
      created_at: new Date(),
      updated_at: new Date(),
    };
    expect(instruction.instruction).toBeDefined();
  });
});
