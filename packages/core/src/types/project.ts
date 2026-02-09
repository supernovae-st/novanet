// novanet-core/src/types/project.ts
// Project + nodable architecture types (v7.2.2)
//
// v7.2.2: BrandL10n merged into ProjectContent (CTAs, meta, SEO keywords)
// v7.1.0 STANDARD PROPERTIES:
//   key, display_name, description, llm_context, created_at, updated_at

// ═══════════════════════════════════════════════════════════════════════════════
// PROJECT (invariant) - v7.1.0
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
// PROJECTL10N (localized identity & messaging) - v7.1.0
// ═══════════════════════════════════════════════════════════════════════════════

export interface VoiceTone {
  formal_casual: number;
  serious_playful: number;
  technical_simple: number;
  reserved_enthusiastic: number;
}

export interface ProjectContent {
  // Standard properties (v7.1.0 - L10n nodes don't have key)
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

// ═══════════════════════════════════════════════════════════════════════════════
// BRAND IDENTITY (visual/artistic direction) - v7.1.0
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

export interface BrandIdentity {
  // Standard properties (v7.1.0)
  key: string;
  display_name: string;
  description: string;
  llm_context: string;

  // Colors
  color_primary: string;
  color_secondary: string;
  color_accent: string;
  color_background: string;
  color_text: string;
  color_palette: ColorPaletteItem[];

  // Typography
  font_primary: string;
  font_secondary: string;
  font_mono?: string;
  typography_scale: TypographyScaleItem[];

  // Visual style (for image generation)
  style_keywords: string[];
  style_mood: string;
  style_influences: string[];

  image_style: string;
  image_do: string[];
  image_dont: string[];
  photo_style?: string;

  // Logo & assets
  logo_primary_url?: string;
  logo_icon_url?: string;
  logo_usage_rules: string[];

  // UI patterns
  border_radius: string;
  shadow_style: string;
  animation_style: string;

  created_at: Date;
  updated_at: Date;
}

// Export all types
// v7.2.5: Audience merged into ProjectContent.target_audience
// v7.2.5: ValuePropL10n + SocialProofL10n removed
export type ProjectNode =
  | Project
  | ProjectContent
  | BrandIdentity;
