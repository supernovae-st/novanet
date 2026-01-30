// NovaNet Core - Type Tests v7.1.0
// TDD: Verify type exports and structure

import { describe, it, expect } from 'vitest';
import type {
  // Standard base
  StandardNodeProperties,

  // Core entities
  Concept,
  ConceptL10n,
  PageL10n,
  BlockL10n,

  // Locale
  Locale,
  LocaleVoice,

  // SEO/GEO (v7.8.2: SEOKeyword → SEOKeywordL10n, v7.8.5: GEOCitation → GEOSeedMetrics)
  SEOKeywordL10n,
  GEOSeedMetrics,

  // Relation props
  SemanticLinkProps,
  UsesConceptProps,
  HasBlockProps,
} from '../types/index.js';

describe('Type Exports', () => {
  describe('StandardNodeProperties', () => {
    it('should have v7.1.0 required properties', () => {
      // Type assertion test - compile-time verification
      const props: StandardNodeProperties = {
        key: 'test-key',
        display_name: 'Test Key',
        icon: '🔷',
        description: 'Test description',
        llm_context: 'USE: testing. TRIGGERS: test. NOT: production.',
        priority: 'medium',
        freshness: 'static',
        created_at: new Date(),
        updated_at: new Date(),
      };

      expect(props.key).toBe('test-key');
      expect(props.display_name).toBe('Test Key');
      expect(props.icon).toBe('🔷');
      expect(props.llm_context).toBeDefined();
      expect(props.priority).toBe('medium');
      expect(props.freshness).toBe('static');
    });
  });

  describe('Core Entities', () => {
    it('Concept should extend StandardNodeProperties', () => {
      const concept: Concept = {
        key: 'action-create-qr',
        display_name: 'Create QR Code',
        icon: '💡',
        description: 'Core QR code concept',
        llm_context: 'USE: creating QR codes. TRIGGERS: create, generate. NOT: editing.',
        priority: 'critical',
        freshness: 'static',
        created_at: new Date(),
        updated_at: new Date(),
        feature_category: 'core',
        is_core: true,
      };

      expect(concept.key).toBe('action-create-qr');
      expect(concept.feature_category).toBe('core');
      expect(concept.priority).toBe('critical');
    });

    it('ConceptL10n should have required localization fields (v7.1.0)', () => {
      const l10n: ConceptL10n = {
        display_name: 'QR Code',
        icon: '💬',
        description: 'Localized concept',
        llm_context: 'USE: French QR code content. TRIGGERS: fr-FR. NOT: translation.',
        priority: 'high',
        freshness: 'static',
        title: 'Code QR',
        definition: 'Un code-barres 2D',
        version: 1,
        influence_count: 0,
        created_at: new Date(),
        updated_at: new Date(),
      };

      expect(l10n.title).toBe('Code QR');
      expect(l10n.version).toBe(1);
      expect(l10n.priority).toBe('high');
    });

    it('PageL10n should have priority/freshness (v7.1.0) and version (v7.11.0)', () => {
      const output: PageL10n = {
        display_name: 'Pricing Output',
        icon: '📃',
        description: 'Generated pricing page',
        llm_context: 'USE: assembled page content. TRIGGERS: render. NOT: regeneration.',
        priority: 'medium',
        freshness: 'daily',
        assembled: { hero: {}, pricing: {} },
        assembled_at: new Date(),
        assembler_version: '1.0.0',
        created_at: new Date(),
        updated_at: new Date(),
        version: 1, // v7.11.0
      };

      expect(output.updated_at).toBeDefined();
      expect(output.priority).toBe('medium');
      expect(output.version).toBe(1);
    });

    it('BlockL10n should have priority/freshness (v7.1.0) and version (v7.11.0)', () => {
      const output: BlockL10n = {
        display_name: 'Hero Output',
        icon: '📝',
        description: 'Generated hero block',
        llm_context: 'USE: hero section content. TRIGGERS: render hero. NOT: regeneration.',
        priority: 'medium',
        freshness: 'daily',
        generated: { title: 'Welcome' },
        generated_at: new Date(),
        generator_version: '1.0.0',
        created_at: new Date(),
        updated_at: new Date(),
        version: 1, // v7.11.0
      };

      expect(output.updated_at).toBeDefined();
      expect(output.priority).toBe('medium');
      expect(output.version).toBe(1);
    });
  });

  describe('Locale Knowledge (v7.1.0)', () => {
    it('Locale should have standard properties with priority/freshness', () => {
      const locale: Locale = {
        key: 'fr-FR',
        display_name: 'French (France)',
        icon: '🇫🇷',
        description: 'Metropolitan French',
        llm_context: 'USE: French content. TRIGGERS: fr-FR. NOT: Canadian French.',
        priority: 'high',
        freshness: 'static',
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
      expect(locale.priority).toBe('high');
    });

    it('LocaleVoice should have formality settings with priority/freshness', () => {
      const voice: LocaleVoice = {
        display_name: 'French Voice',
        icon: '🗣️',
        description: 'Voice characteristics',
        llm_context: 'USE: tone/formality decisions. TRIGGERS: voice, tone. NOT: cultural norms.',
        priority: 'high',
        freshness: 'static',
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
      expect(voice.priority).toBe('high');
    });
  });

  describe('SEO/GEO Types (v7.8.2: SEOKeyword → SEOKeywordL10n)', () => {
    it('SEOKeywordL10n should extend StandardNodeProperties with priority/freshness', () => {
      const keyword: SEOKeywordL10n = {
        key: 'creer-qr-code-fr',
        display_name: 'créer qr code',
        icon: '🔍',
        description: 'Main SEO keyword',
        llm_context: 'USE: SEO targeting. TRIGGERS: keyword research. NOT: GEO seeds.',
        priority: 'high',
        freshness: 'daily',
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
      expect(keyword.priority).toBe('high');
    });

    // REMOVED v8.0.0: PageMetrics (query GA/PostHog directly with date ranges)

    it('GEOSeedMetrics should track AI citations (v7.8.5)', () => {
      const metrics: GEOSeedMetrics = {
        key: 'geometrics-comment-creer-qr-chatgpt-2024-01',
        display_name: 'Citation ChatGPT',
        icon: '📍',
        description: 'Citation check',
        llm_context: 'USE: for citation tracking. TRIGGERS: cited, AI mention. NOT: SEO metrics.',
        priority: 'medium',
        freshness: 'realtime',
        cited: true,
        position: 2,
        sentiment: 'positive',
        platform: 'chatgpt',
        model: 'gpt-4',
        observed_at: new Date(),
        created_at: new Date(),
        updated_at: new Date(),
      };

      expect(metrics.cited).toBe(true);
      expect(metrics.sentiment).toBe('positive');
      expect(metrics.priority).toBe('medium');
    });
  });

  describe('Relation Props', () => {
    it('SemanticLinkProps should have temperature', () => {
      const props: SemanticLinkProps = {
        type: 'includes',
        temperature: 0.85,
      };

      expect(props.temperature).toBe(0.85);
    });

    it('UsesConceptProps should have purpose', () => {
      const props: UsesConceptProps = {
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

describe('Prompt Types (v7.2.0)', () => {
  it('PagePrompt has required properties', () => {
    const prompt: PagePrompt = {
      display_name: 'Pricing Page Prompt v1.0',
      icon: '📝',
      description: 'Instructions for pricing page generation',
      llm_context: 'USE: orchestration. TRIGGERS: page. NOT: blocks.',
      priority: 'high',
      freshness: 'static',
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
    const prompt: BlockPrompt = {
      display_name: 'Pricing Hero Prompt v1.0',
      icon: '📝',
      description: 'Instructions for pricing hero generation',
      llm_context: 'USE: hero generation. TRIGGERS: block hero. NOT: other blocks.',
      priority: 'high',
      freshness: 'static',
      prompt: '[GENERATE] Hero highlighting @tier-pro benefits',
      version: '1.0',
      active: true,
      created_at: new Date(),
      updated_at: new Date(),
    };
    expect(prompt.prompt).toBeDefined();
  });

  it('BlockRules has required properties', () => {
    const rules: BlockRules = {
      display_name: 'Hero Rules v1.0',
      icon: '📏',
      description: 'Generation rules for hero block type',
      llm_context: 'USE: rule validation. TRIGGERS: hero rules. NOT: other types.',
      priority: 'high',
      freshness: 'static',
      rules: 'Title: action verb. Subtitle: value prop. CTA: urgency.',
      version: '1.0',
      active: true,
      created_at: new Date(),
      updated_at: new Date(),
    };
    expect(rules.rules).toBeDefined();
  });
});
