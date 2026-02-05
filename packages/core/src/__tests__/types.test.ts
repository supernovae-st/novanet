// NovaNet Core - Type Tests v10.3.0
// TDD: Verify type exports and structure
// v10.3.0: Entity-Centric Architecture (Concept → Entity, ConceptL10n → EntityL10n)
// v8.2.0: Removed icon, priority, freshness from all interfaces (YAML v7.11.0 alignment)

import { describe, it, expect } from 'vitest';
import type {
  // Standard base
  StandardNodeProperties,

  // Core entities (v10.3: Entity replaces Concept)
  Entity,
  EntityL10n,
  PageL10n,
  BlockL10n,

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
    it('should have v8.2.0 required properties (no icon/priority/freshness)', () => {
      // Type assertion test - compile-time verification
      // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
      const props: StandardNodeProperties = {
        key: 'test-key',
        display_name: 'Test Key',
        description: 'Test description',
        llm_context: 'USE: testing. TRIGGERS: test. NOT: production.',
        created_at: new Date(),
        updated_at: new Date(),
      };

      expect(props.key).toBe('test-key');
      expect(props.display_name).toBe('Test Key');
      expect(props.llm_context).toBeDefined();
      expect(props.created_at).toBeDefined();
      expect(props.updated_at).toBeDefined();
    });
  });

  describe('Core Entities', () => {
    it('Entity should extend StandardNodeProperties (v10.3)', () => {
      // v10.3: Entity replaces Concept (global realm, knowledge layer)
      const entity: Entity = {
        key: 'action-create-qr',
        display_name: 'Create QR Code',
        description: 'Core QR code entity',
        llm_context: 'USE: creating QR codes. TRIGGERS: create, generate. NOT: editing.',
        created_at: new Date(),
        updated_at: new Date(),
        feature_category: 'core',
        is_core: true,
      };

      expect(entity.key).toBe('action-create-qr');
      expect(entity.feature_category).toBe('core');
      expect(entity.is_core).toBe(true);
    });

    it('EntityL10n should have required localization fields (v10.3)', () => {
      // v10.3: EntityL10n replaces ConceptL10n (global realm, knowledge layer)
      const l10n: EntityL10n = {
        display_name: 'QR Code',
        description: 'Localized entity',
        llm_context: 'USE: French QR code content. TRIGGERS: fr-FR. NOT: translation.',
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

    it('PageL10n should have version (v8.2.0 - no icon/priority/freshness)', () => {
      // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
      const output: PageL10n = {
        display_name: 'Pricing Output',
        description: 'Generated pricing page',
        llm_context: 'USE: assembled page content. TRIGGERS: render. NOT: regeneration.',
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

    it('BlockL10n should have version (v8.2.0 - no icon/priority/freshness)', () => {
      // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
      const output: BlockL10n = {
        display_name: 'Hero Output',
        description: 'Generated hero block',
        llm_context: 'USE: hero section content. TRIGGERS: render hero. NOT: regeneration.',
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
        description: 'Metropolitan French',
        llm_context: 'USE: French content. TRIGGERS: fr-FR. NOT: Canadian French.',
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
        description: 'Voice characteristics',
        llm_context: 'USE: tone/formality decisions. TRIGGERS: voice, tone. NOT: cultural norms.',
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
        description: 'Main SEO keyword',
        llm_context: 'USE: SEO targeting. TRIGGERS: keyword research. NOT: other.',
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
// PROMPT TYPES (v7.2.0)
// ═══════════════════════════════════════════════════════════════════════════════

import type { PagePrompt, BlockPrompt, BlockRules } from '../types/prompts.js';

describe('Prompt Types (v8.2.0 - no icon/priority/freshness)', () => {
  it('PagePrompt has required properties', () => {
    // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
    const prompt: PagePrompt = {
      display_name: 'Pricing Page Prompt v1.0',
      description: 'Instructions for pricing page generation',
      llm_context: 'USE: orchestration. TRIGGERS: page. NOT: blocks.',
      prompt: '[GENERATE] Create conversion-focused pricing page',
      version: '1.0',
      active: true,
      created_at: new Date(),
      updated_at: new Date(),
    };
    expect(prompt.prompt).toBeDefined();
    expect(prompt.version).toBe('1.0');
    expect(prompt.active).toBe(true);
  });

  it('BlockPrompt has required properties', () => {
    // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
    const prompt: BlockPrompt = {
      display_name: 'Pricing Hero Prompt v1.0',
      description: 'Instructions for pricing hero generation',
      llm_context: 'USE: hero generation. TRIGGERS: block hero. NOT: other blocks.',
      prompt: '[GENERATE] Hero highlighting @tier-pro benefits',
      version: '1.0',
      active: true,
      created_at: new Date(),
      updated_at: new Date(),
    };
    expect(prompt.prompt).toBeDefined();
  });

  it('BlockRules has required properties', () => {
    // v8.2.0: Removed icon, priority, freshness (YAML v7.11.0 alignment)
    const rules: BlockRules = {
      display_name: 'Hero Rules v1.0',
      description: 'Generation rules for hero block type',
      llm_context: 'USE: rule validation. TRIGGERS: hero rules. NOT: other types.',
      rules: 'Title: action verb. Subtitle: value prop. CTA: urgency.',
      version: '1.0',
      active: true,
      created_at: new Date(),
      updated_at: new Date(),
    };
    expect(rules.rules).toBeDefined();
  });
});
