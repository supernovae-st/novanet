// novanet-core/src/types/project.ts
// Project + nodable architecture types (v0.13.0)
//
// v0.13.0: ProjectNative → ProjectNative (ADR-029 *Native pattern)
// v7.2.2: BrandL10n merged into ProjectNative (CTAs, meta, SEO keywords) - L10n suffix deprecated v10.9
// v7.1.0 STANDARD PROPERTIES:
//   key, display_name, description, llm_context, created_at, updated_at

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT (defined) - v7.1.0, trait renamed v0.12.0 ADR-024
// ═══════════════════════════════════════════════════════════════════════════════

export interface Project {
  // Standard properties (v7.1.0)
  key: string;
  display_name: string;
  description: string;
  llm_context: string;

  // Project-specific properties
  brand_name: string;
  brand_name_legal: string;
  founding_year: number;
  website_url: string;
  core_values: string[];
  category: string;
  competitors: string[];

  created_at: Date;
  updated_at: Date;
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECTNATIVE (authored identity & messaging) - v0.13.0 ADR-029, trait v0.12.0 ADR-024
// ═══════════════════════════════════════════════════════════════════════════════

export interface VoiceTone {
  formal_casual: number;
  serious_playful: number;
  technical_simple: number;
  reserved_enthusiastic: number;
}

/**
 * ProjectNative - Project content per locale (v0.13.0 ADR-029).
 * Locale-specific project messaging, identity, and voice.
 */
export interface ProjectNative {
  // Standard properties (v7.1.0 - Content nodes don't have key)
  display_name: string;
  description: string;
  llm_context: string;

  // v7.1.0: Locale is linked via :FOR_LOCALE relation, not stored as property

  // Identity
  what_short: string;
  what_medium: string;
  what_long: string;

  // Messaging
  tagline: string;
  pitch_one_liner: string;
  pitch_elevator: string;
  pitch_detailed: string;

  // Voice
  voice_personality: string[];
  voice_tone: VoiceTone;
  voice_do: string[];
  voice_dont: string[];

  // CTAs (merged from BrandL10n v7.2.2)
  cta_primary: string;
  cta_secondary: string;
  cta_tertiary?: string;

  // SEO (merged from BrandL10n v7.2.2)
  meta_description: string;
  primary_keywords?: string[];
  secondary_keywords?: string[];

  created_at: Date;
  updated_at: Date;
}

/** @deprecated Use ProjectNative (v0.13.0 ADR-029) */
export type ProjectContent = ProjectNative;

// ═══════════════════════════════════════════════════════════════════════════════
// BRAND ARCHITECTURE (v0.12.4 ADR-028)
// Brand delegates to: BrandDesign (visual), BrandPrinciples (voice), PromptStyle (LLM hints)
// ═══════════════════════════════════════════════════════════════════════════════

export interface ColorPaletteItem {
  name: string;
  hex: string;
  usage: string;
}

export interface TypographyScaleItem {
  name: string;
  size: string;
  weight: string;
  line_height: string;
}

/**
 * Brand - Central brand identity node (ADR-028)
 * Owns: BrandDesign, BrandPrinciples, PromptStyle
 */
export interface Brand {
  // Standard properties
  key: string;
  display_name: string;
  description: string;
  llm_context: string;

  // Core Identity
  brand_name: string;
  tagline?: string;
  brand_story?: string;

  // Logo & Assets
  logo_primary_url?: string;
  logo_icon_url?: string;
  logo_usage_rules?: string[];

  // Market Position
  target_market?: string;
  value_proposition?: string;

  created_at: Date;
  updated_at: Date;
}

/**
 * BrandDesign - Visual design system (ADR-028)
 * Colors, typography, UI patterns
 */
export interface BrandDesign {
  // Standard properties
  key: string;
  display_name: string;
  description: string;
  llm_context: string;

  // Colors
  color_primary: string;
  color_secondary?: string;
  color_accent?: string;
  color_background?: string;
  color_text?: string;
  color_palette?: ColorPaletteItem[];

  // Typography
  font_primary: string;
  font_secondary?: string;
  font_mono?: string;
  typography_scale?: TypographyScaleItem[];

  // UI patterns
  border_radius?: string;
  shadow_style?: 'subtle' | 'pronounced' | 'none';
  animation_style?: 'smooth' | 'snappy' | 'playful';

  created_at: Date;
  updated_at: Date;
}

/**
 * BrandPrinciples - Voice, tone, editorial guidelines (ADR-028)
 */
export interface BrandPrinciples {
  // Standard properties
  key: string;
  display_name: string;
  description: string;
  llm_context: string;

  // Voice & Tone
  voice_attributes: string[];
  tone_default: string;
  tone_variations?: Record<string, string>;

  // Writing Guidelines
  writing_do?: string[];
  writing_dont?: string[];

  // Messaging
  key_messages?: string[];
  differentiators?: string[];

  // Terminology
  preferred_terms?: Record<string, string>;
  forbidden_terms?: string[];

  created_at: Date;
  updated_at: Date;
}

/**
 * PromptStyle - LLM generation style hints (ADR-028)
 * Can be locale or region specific for cultural adaptation
 */
export interface PromptStyle {
  // Standard properties
  key: string;
  display_name: string;
  description: string;
  llm_context: string;

  // Visual Style
  style_keywords: string[];
  style_mood?: string;
  style_influences?: string[];

  // Image Generation
  image_style?: string;
  image_do?: string[];
  image_dont?: string[];
  photo_style?: string;

  // Cultural Adaptation
  cultural_style?: string;
  visual_prompt?: string;
  formality_level?: 'formal' | 'neutral' | 'casual';
  humor_style?: string;

  created_at: Date;
  updated_at: Date;
}

// Export all types
// v7.2.5: Audience merged into ProjectNative.target_audience
// v7.2.5: ValuePropL10n + SocialProofL10n removed
// v0.12.4: BrandIdentity → Brand + BrandDesign + BrandPrinciples + PromptStyle (ADR-028)
// v0.13.0: ProjectNative → ProjectNative (ADR-029)
export type ProjectNode =
  | Project
  | ProjectNative
  | Brand
  | BrandDesign
  | BrandPrinciples
  | PromptStyle;
