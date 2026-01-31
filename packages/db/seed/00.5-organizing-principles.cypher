// Organizing Principles Seed v8.3.0
// AUTO-GENERATED from organizing-principles.yaml
// Generated: 2026-01-31
// Run: pnpm schema:generate
//
// Creates: Scope, Subcategory, NodeTypeMeta nodes
// Uses MERGE for idempotent execution

// ═══════════════════════════════════════════════════════════════════════════════
// SCOPES (3)
// ═══════════════════════════════════════════════════════════════════════════════

MERGE (s_global:Scope {key: 'global'})
ON CREATE SET
  s_global.display_name = 'Global',
  s_global.emoji = '🌍',
  s_global.color = '#2aa198',
  s_global.llm_context = 'Shared across ALL projects. Locale-specific knowledge that applies universally: cultural norms, linguistic patterns, voice guidelines, idiomatic expressions. These nodes are READ-ONLY at project level. Changes here affect all projects using that locale.',
  s_global.created_at = datetime()
ON MATCH SET
  s_global.display_name = 'Global',
  s_global.emoji = '🌍',
  s_global.color = '#2aa198',
  s_global.llm_context = 'Shared across ALL projects. Locale-specific knowledge that applies universally: cultural norms, linguistic patterns, voice guidelines, idiomatic expressions. These nodes are READ-ONLY at project level. Changes here affect all projects using that locale.',
  s_global.updated_at = datetime();

MERGE (s_project:Scope {key: 'project'})
ON CREATE SET
  s_project.display_name = 'Project',
  s_project.emoji = '📦',
  s_project.color = '#6c71c4',
  s_project.llm_context = 'Business-specific nodes for a single project. Contains brand identity, page structure, semantic concepts, generation prompts, and localized outputs. These nodes define WHAT content to generate and HOW to structure it for this specific product/service.',
  s_project.created_at = datetime()
ON MATCH SET
  s_project.display_name = 'Project',
  s_project.emoji = '📦',
  s_project.color = '#6c71c4',
  s_project.llm_context = 'Business-specific nodes for a single project. Contains brand identity, page structure, semantic concepts, generation prompts, and localized outputs. These nodes define WHAT content to generate and HOW to structure it for this specific product/service.',
  s_project.updated_at = datetime();

MERGE (s_shared:Scope {key: 'shared'})
ON CREATE SET
  s_shared.display_name = 'Shared',
  s_shared.emoji = '🎯',
  s_shared.color = '#cb4b16',
  s_shared.llm_context = 'Cross-project resources that can be linked to multiple projects. SEO keywords and GEO seeds with their metrics. These enable competitive intelligence sharing across the portfolio.',
  s_shared.created_at = datetime()
ON MATCH SET
  s_shared.display_name = 'Shared',
  s_shared.emoji = '🎯',
  s_shared.color = '#cb4b16',
  s_shared.llm_context = 'Cross-project resources that can be linked to multiple projects. SEO keywords and GEO seeds with their metrics. These enable competitive intelligence sharing across the portfolio.',
  s_shared.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════════
// SUBCATEGORIES (9)
// ═══════════════════════════════════════════════════════════════════════════════

// Global subcategories
MERGE (sub_config:Subcategory {key: 'config'})
ON CREATE SET
  sub_config.display_name = 'Configuration',
  sub_config.emoji = '⚙️',
  sub_config.llm_context = 'Core configuration nodes. Locale definitions with their properties (language code, region, writing direction). Entry point for all locale-specific knowledge traversal.',
  sub_config.created_at = datetime()
ON MATCH SET
  sub_config.display_name = 'Configuration',
  sub_config.emoji = '⚙️',
  sub_config.llm_context = 'Core configuration nodes. Locale definitions with their properties (language code, region, writing direction). Entry point for all locale-specific knowledge traversal.',
  sub_config.updated_at = datetime();

MATCH (s:Scope {key: 'global'}), (sub:Subcategory {key: 'config'})
MERGE (s)-[:HAS_SUBCATEGORY]->(sub);

MERGE (sub_knowledge:Subcategory {key: 'knowledge'})
ON CREATE SET
  sub_knowledge.display_name = 'Locale Knowledge',
  sub_knowledge.emoji = '📚',
  sub_knowledge.llm_context = 'Deep locale-specific knowledge for native content generation. Cultural norms, linguistic patterns, voice guidelines, idiomatic expressions, formatting conventions. This is what makes generated content feel NATIVE rather than translated.',
  sub_knowledge.created_at = datetime()
ON MATCH SET
  sub_knowledge.display_name = 'Locale Knowledge',
  sub_knowledge.emoji = '📚',
  sub_knowledge.llm_context = 'Deep locale-specific knowledge for native content generation. Cultural norms, linguistic patterns, voice guidelines, idiomatic expressions, formatting conventions. This is what makes generated content feel NATIVE rather than translated.',
  sub_knowledge.updated_at = datetime();

MATCH (s:Scope {key: 'global'}), (sub:Subcategory {key: 'knowledge'})
MERGE (s)-[:HAS_SUBCATEGORY]->(sub);

// Project subcategories
MERGE (sub_foundation:Subcategory {key: 'foundation'})
ON CREATE SET
  sub_foundation.display_name = 'Foundation',
  sub_foundation.emoji = '🏛️',
  sub_foundation.llm_context = 'Core project identity. Brand voice, visual identity, value proposition. These nodes anchor ALL content generation for the project - every generated block must align with foundation.',
  sub_foundation.created_at = datetime()
ON MATCH SET
  sub_foundation.display_name = 'Foundation',
  sub_foundation.emoji = '🏛️',
  sub_foundation.llm_context = 'Core project identity. Brand voice, visual identity, value proposition. These nodes anchor ALL content generation for the project - every generated block must align with foundation.',
  sub_foundation.updated_at = datetime();

MATCH (s:Scope {key: 'project'}), (sub:Subcategory {key: 'foundation'})
MERGE (s)-[:HAS_SUBCATEGORY]->(sub);

MERGE (sub_structure:Subcategory {key: 'structure'})
ON CREATE SET
  sub_structure.display_name = 'Structure',
  sub_structure.emoji = '🏗️',
  sub_structure.llm_context = 'Information architecture. Pages, blocks, and their types. Defines the SKELETON of the website - what pages exist, what blocks compose each page, and the rules for each block type.',
  sub_structure.created_at = datetime()
ON MATCH SET
  sub_structure.display_name = 'Structure',
  sub_structure.emoji = '🏗️',
  sub_structure.llm_context = 'Information architecture. Pages, blocks, and their types. Defines the SKELETON of the website - what pages exist, what blocks compose each page, and the rules for each block type.',
  sub_structure.updated_at = datetime();

MATCH (s:Scope {key: 'project'}), (sub:Subcategory {key: 'structure'})
MERGE (s)-[:HAS_SUBCATEGORY]->(sub);

MERGE (sub_semantic:Subcategory {key: 'semantic'})
ON CREATE SET
  sub_semantic.display_name = 'Semantic Layer',
  sub_semantic.emoji = '💡',
  sub_semantic.llm_context = 'Meaning and concepts. Invariant ideas (Concept) that get localized per locale (ConceptL10n). The WHAT of content - pricing tiers, features, benefits, use cases. Concepts link via SEMANTIC_LINK for spreading activation during generation.',
  sub_semantic.created_at = datetime()
ON MATCH SET
  sub_semantic.display_name = 'Semantic Layer',
  sub_semantic.emoji = '💡',
  sub_semantic.llm_context = 'Meaning and concepts. Invariant ideas (Concept) that get localized per locale (ConceptL10n). The WHAT of content - pricing tiers, features, benefits, use cases. Concepts link via SEMANTIC_LINK for spreading activation during generation.',
  sub_semantic.updated_at = datetime();

MATCH (s:Scope {key: 'project'}), (sub:Subcategory {key: 'semantic'})
MERGE (s)-[:HAS_SUBCATEGORY]->(sub);

MERGE (sub_instruction:Subcategory {key: 'instruction'})
ON CREATE SET
  sub_instruction.display_name = 'Instructions',
  sub_instruction.emoji = '📝',
  sub_instruction.llm_context = 'Generation directives. Prompts and rules that guide the LLM during content generation. PagePrompt for page-level guidance, BlockPrompt for block-specific instructions, BlockRules for constraints.',
  sub_instruction.created_at = datetime()
ON MATCH SET
  sub_instruction.display_name = 'Instructions',
  sub_instruction.emoji = '📝',
  sub_instruction.llm_context = 'Generation directives. Prompts and rules that guide the LLM during content generation. PagePrompt for page-level guidance, BlockPrompt for block-specific instructions, BlockRules for constraints.',
  sub_instruction.updated_at = datetime();

MATCH (s:Scope {key: 'project'}), (sub:Subcategory {key: 'instruction'})
MERGE (s)-[:HAS_SUBCATEGORY]->(sub);

MERGE (sub_output:Subcategory {key: 'output'})
ON CREATE SET
  sub_output.display_name = 'Generated Output',
  sub_output.emoji = '✨',
  sub_output.llm_context = 'LLM-generated content. The final localized pages and blocks ready for rendering. These are the RESULTS of the generation pipeline - created by combining foundation, structure, semantic, and instruction nodes with locale knowledge.',
  sub_output.created_at = datetime()
ON MATCH SET
  sub_output.display_name = 'Generated Output',
  sub_output.emoji = '✨',
  sub_output.llm_context = 'LLM-generated content. The final localized pages and blocks ready for rendering. These are the RESULTS of the generation pipeline - created by combining foundation, structure, semantic, and instruction nodes with locale knowledge.',
  sub_output.updated_at = datetime();

MATCH (s:Scope {key: 'project'}), (sub:Subcategory {key: 'output'})
MERGE (s)-[:HAS_SUBCATEGORY]->(sub);

// Shared subcategories
MERGE (sub_seo:Subcategory {key: 'seo'})
ON CREATE SET
  sub_seo.display_name = 'SEO Intelligence',
  sub_seo.emoji = '🔍',
  sub_seo.llm_context = 'Search engine optimization data. Keywords with their localized forms, search volume metrics, and mining run history. Used to inject relevant keywords into generated content for organic search visibility.',
  sub_seo.created_at = datetime()
ON MATCH SET
  sub_seo.display_name = 'SEO Intelligence',
  sub_seo.emoji = '🔍',
  sub_seo.llm_context = 'Search engine optimization data. Keywords with their localized forms, search volume metrics, and mining run history. Used to inject relevant keywords into generated content for organic search visibility.',
  sub_seo.updated_at = datetime();

MATCH (s:Scope {key: 'shared'}), (sub:Subcategory {key: 'seo'})
MERGE (s)-[:HAS_SUBCATEGORY]->(sub);

MERGE (sub_geo:Subcategory {key: 'geo'})
ON CREATE SET
  sub_geo.display_name = 'GEO Intelligence',
  sub_geo.emoji = '📍',
  sub_geo.llm_context = 'Geographic/local SEO data. Location-based seeds with their localized forms, metrics, and mining history. Used for local business visibility and location-specific content generation.',
  sub_geo.created_at = datetime()
ON MATCH SET
  sub_geo.display_name = 'GEO Intelligence',
  sub_geo.emoji = '📍',
  sub_geo.llm_context = 'Geographic/local SEO data. Location-based seeds with their localized forms, metrics, and mining history. Used for local business visibility and location-specific content generation.',
  sub_geo.updated_at = datetime();

MATCH (s:Scope {key: 'shared'}), (sub:Subcategory {key: 'geo'})
MERGE (s)-[:HAS_SUBCATEGORY]->(sub);

// ═══════════════════════════════════════════════════════════════════════════════
// NODE TYPE META (35)
// ═══════════════════════════════════════════════════════════════════════════════

// Configuration (1 types)
MERGE (t_locale:NodeTypeMeta {label: 'Locale'})
ON CREATE SET
  t_locale.display_name = 'Locale',
  t_locale.yaml_path = 'models/nodes/global/config/locale.yaml',
  t_locale.created_at = datetime()
ON MATCH SET
  t_locale.updated_at = datetime();

MATCH (sub:Subcategory {key: 'config'}), (t:NodeTypeMeta {label: 'Locale'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

// Locale Knowledge (14 types)
MERGE (t_constraint:NodeTypeMeta {label: 'Constraint'})
ON CREATE SET
  t_constraint.display_name = 'Constraint',
  t_constraint.yaml_path = 'models/nodes/global/knowledge/constraint.yaml',
  t_constraint.created_at = datetime()
ON MATCH SET
  t_constraint.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'Constraint'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_expression:NodeTypeMeta {label: 'Expression'})
ON CREATE SET
  t_expression.display_name = 'Expression',
  t_expression.yaml_path = 'models/nodes/global/knowledge/expression.yaml',
  t_expression.created_at = datetime()
ON MATCH SET
  t_expression.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'Expression'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_localeculture:NodeTypeMeta {label: 'LocaleCulture'})
ON CREATE SET
  t_localeculture.display_name = 'LocaleCulture',
  t_localeculture.yaml_path = 'models/nodes/global/knowledge/locale-culture.yaml',
  t_localeculture.created_at = datetime()
ON MATCH SET
  t_localeculture.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'LocaleCulture'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_localeculturereferences:NodeTypeMeta {label: 'LocaleCultureReferences'})
ON CREATE SET
  t_localeculturereferences.display_name = 'LocaleCultureReferences',
  t_localeculturereferences.yaml_path = 'models/nodes/global/knowledge/locale-culture-references.yaml',
  t_localeculturereferences.created_at = datetime()
ON MATCH SET
  t_localeculturereferences.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'LocaleCultureReferences'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_localeidentity:NodeTypeMeta {label: 'LocaleIdentity'})
ON CREATE SET
  t_localeidentity.display_name = 'LocaleIdentity',
  t_localeidentity.yaml_path = 'models/nodes/global/knowledge/locale-identity.yaml',
  t_localeidentity.created_at = datetime()
ON MATCH SET
  t_localeidentity.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'LocaleIdentity'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_localelexicon:NodeTypeMeta {label: 'LocaleLexicon'})
ON CREATE SET
  t_localelexicon.display_name = 'LocaleLexicon',
  t_localelexicon.yaml_path = 'models/nodes/global/knowledge/locale-lexicon.yaml',
  t_localelexicon.created_at = datetime()
ON MATCH SET
  t_localelexicon.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'LocaleLexicon'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_localemarket:NodeTypeMeta {label: 'LocaleMarket'})
ON CREATE SET
  t_localemarket.display_name = 'LocaleMarket',
  t_localemarket.yaml_path = 'models/nodes/global/knowledge/locale-market.yaml',
  t_localemarket.created_at = datetime()
ON MATCH SET
  t_localemarket.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'LocaleMarket'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_localerulesadaptation:NodeTypeMeta {label: 'LocaleRulesAdaptation'})
ON CREATE SET
  t_localerulesadaptation.display_name = 'LocaleRulesAdaptation',
  t_localerulesadaptation.yaml_path = 'models/nodes/global/knowledge/locale-rules-adaptation.yaml',
  t_localerulesadaptation.created_at = datetime()
ON MATCH SET
  t_localerulesadaptation.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'LocaleRulesAdaptation'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_localerulesformatting:NodeTypeMeta {label: 'LocaleRulesFormatting'})
ON CREATE SET
  t_localerulesformatting.display_name = 'LocaleRulesFormatting',
  t_localerulesformatting.yaml_path = 'models/nodes/global/knowledge/locale-rules-formatting.yaml',
  t_localerulesformatting.created_at = datetime()
ON MATCH SET
  t_localerulesformatting.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'LocaleRulesFormatting'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_localerulesslug:NodeTypeMeta {label: 'LocaleRulesSlug'})
ON CREATE SET
  t_localerulesslug.display_name = 'LocaleRulesSlug',
  t_localerulesslug.yaml_path = 'models/nodes/global/knowledge/locale-rules-slug.yaml',
  t_localerulesslug.created_at = datetime()
ON MATCH SET
  t_localerulesslug.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'LocaleRulesSlug'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_localevoice:NodeTypeMeta {label: 'LocaleVoice'})
ON CREATE SET
  t_localevoice.display_name = 'LocaleVoice',
  t_localevoice.yaml_path = 'models/nodes/global/knowledge/locale-voice.yaml',
  t_localevoice.created_at = datetime()
ON MATCH SET
  t_localevoice.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'LocaleVoice'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_metaphor:NodeTypeMeta {label: 'Metaphor'})
ON CREATE SET
  t_metaphor.display_name = 'Metaphor',
  t_metaphor.yaml_path = 'models/nodes/global/knowledge/metaphor.yaml',
  t_metaphor.created_at = datetime()
ON MATCH SET
  t_metaphor.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'Metaphor'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_pattern:NodeTypeMeta {label: 'Pattern'})
ON CREATE SET
  t_pattern.display_name = 'Pattern',
  t_pattern.yaml_path = 'models/nodes/global/knowledge/pattern.yaml',
  t_pattern.created_at = datetime()
ON MATCH SET
  t_pattern.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'Pattern'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_reference:NodeTypeMeta {label: 'Reference'})
ON CREATE SET
  t_reference.display_name = 'Reference',
  t_reference.yaml_path = 'models/nodes/global/knowledge/reference.yaml',
  t_reference.created_at = datetime()
ON MATCH SET
  t_reference.updated_at = datetime();

MATCH (sub:Subcategory {key: 'knowledge'}), (t:NodeTypeMeta {label: 'Reference'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

// Foundation (3 types)
MERGE (t_brandidentity:NodeTypeMeta {label: 'BrandIdentity'})
ON CREATE SET
  t_brandidentity.display_name = 'BrandIdentity',
  t_brandidentity.yaml_path = 'models/nodes/project/foundation/brand-identity.yaml',
  t_brandidentity.created_at = datetime()
ON MATCH SET
  t_brandidentity.updated_at = datetime();

MATCH (sub:Subcategory {key: 'foundation'}), (t:NodeTypeMeta {label: 'BrandIdentity'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_project:NodeTypeMeta {label: 'Project'})
ON CREATE SET
  t_project.display_name = 'Project',
  t_project.yaml_path = 'models/nodes/project/foundation/project.yaml',
  t_project.created_at = datetime()
ON MATCH SET
  t_project.updated_at = datetime();

MATCH (sub:Subcategory {key: 'foundation'}), (t:NodeTypeMeta {label: 'Project'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_projectl10n:NodeTypeMeta {label: 'ProjectL10n'})
ON CREATE SET
  t_projectl10n.display_name = 'ProjectL10n',
  t_projectl10n.yaml_path = 'models/nodes/project/foundation/project-l10n.yaml',
  t_projectl10n.created_at = datetime()
ON MATCH SET
  t_projectl10n.updated_at = datetime();

MATCH (sub:Subcategory {key: 'foundation'}), (t:NodeTypeMeta {label: 'ProjectL10n'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

// Structure (2 types)
MERGE (t_block:NodeTypeMeta {label: 'Block'})
ON CREATE SET
  t_block.display_name = 'Block',
  t_block.yaml_path = 'models/nodes/project/structure/block.yaml',
  t_block.created_at = datetime()
ON MATCH SET
  t_block.updated_at = datetime();

MATCH (sub:Subcategory {key: 'structure'}), (t:NodeTypeMeta {label: 'Block'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_page:NodeTypeMeta {label: 'Page'})
ON CREATE SET
  t_page.display_name = 'Page',
  t_page.yaml_path = 'models/nodes/project/structure/page.yaml',
  t_page.created_at = datetime()
ON MATCH SET
  t_page.updated_at = datetime();

MATCH (sub:Subcategory {key: 'structure'}), (t:NodeTypeMeta {label: 'Page'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

// Semantic Layer (2 types)
MERGE (t_concept:NodeTypeMeta {label: 'Concept'})
ON CREATE SET
  t_concept.display_name = 'Concept',
  t_concept.yaml_path = 'models/nodes/project/semantic/concept.yaml',
  t_concept.created_at = datetime()
ON MATCH SET
  t_concept.updated_at = datetime();

MATCH (sub:Subcategory {key: 'semantic'}), (t:NodeTypeMeta {label: 'Concept'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_conceptl10n:NodeTypeMeta {label: 'ConceptL10n'})
ON CREATE SET
  t_conceptl10n.display_name = 'ConceptL10n',
  t_conceptl10n.yaml_path = 'models/nodes/project/semantic/concept-l10n.yaml',
  t_conceptl10n.created_at = datetime()
ON MATCH SET
  t_conceptl10n.updated_at = datetime();

MATCH (sub:Subcategory {key: 'semantic'}), (t:NodeTypeMeta {label: 'ConceptL10n'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

// Instructions (5 types)
MERGE (t_blockprompt:NodeTypeMeta {label: 'BlockPrompt'})
ON CREATE SET
  t_blockprompt.display_name = 'BlockPrompt',
  t_blockprompt.yaml_path = 'models/nodes/project/instruction/block-prompt.yaml',
  t_blockprompt.created_at = datetime()
ON MATCH SET
  t_blockprompt.updated_at = datetime();

MATCH (sub:Subcategory {key: 'instruction'}), (t:NodeTypeMeta {label: 'BlockPrompt'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_blockrules:NodeTypeMeta {label: 'BlockRules'})
ON CREATE SET
  t_blockrules.display_name = 'BlockRules',
  t_blockrules.yaml_path = 'models/nodes/project/instruction/block-rules.yaml',
  t_blockrules.created_at = datetime()
ON MATCH SET
  t_blockrules.updated_at = datetime();

MATCH (sub:Subcategory {key: 'instruction'}), (t:NodeTypeMeta {label: 'BlockRules'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_blocktype:NodeTypeMeta {label: 'BlockType'})
ON CREATE SET
  t_blocktype.display_name = 'BlockType',
  t_blocktype.yaml_path = 'models/nodes/project/instruction/block-type.yaml',
  t_blocktype.created_at = datetime()
ON MATCH SET
  t_blocktype.updated_at = datetime();

MATCH (sub:Subcategory {key: 'instruction'}), (t:NodeTypeMeta {label: 'BlockType'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_pageprompt:NodeTypeMeta {label: 'PagePrompt'})
ON CREATE SET
  t_pageprompt.display_name = 'PagePrompt',
  t_pageprompt.yaml_path = 'models/nodes/project/instruction/page-prompt.yaml',
  t_pageprompt.created_at = datetime()
ON MATCH SET
  t_pageprompt.updated_at = datetime();

MATCH (sub:Subcategory {key: 'instruction'}), (t:NodeTypeMeta {label: 'PagePrompt'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_pagetype:NodeTypeMeta {label: 'PageType'})
ON CREATE SET
  t_pagetype.display_name = 'PageType',
  t_pagetype.yaml_path = 'models/nodes/project/instruction/page-type.yaml',
  t_pagetype.created_at = datetime()
ON MATCH SET
  t_pagetype.updated_at = datetime();

MATCH (sub:Subcategory {key: 'instruction'}), (t:NodeTypeMeta {label: 'PageType'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

// Generated Output (2 types)
MERGE (t_blockl10n:NodeTypeMeta {label: 'BlockL10n'})
ON CREATE SET
  t_blockl10n.display_name = 'BlockL10n',
  t_blockl10n.yaml_path = 'models/nodes/project/output/block-l10n.yaml',
  t_blockl10n.created_at = datetime()
ON MATCH SET
  t_blockl10n.updated_at = datetime();

MATCH (sub:Subcategory {key: 'output'}), (t:NodeTypeMeta {label: 'BlockL10n'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_pagel10n:NodeTypeMeta {label: 'PageL10n'})
ON CREATE SET
  t_pagel10n.display_name = 'PageL10n',
  t_pagel10n.yaml_path = 'models/nodes/project/output/page-l10n.yaml',
  t_pagel10n.created_at = datetime()
ON MATCH SET
  t_pagel10n.updated_at = datetime();

MATCH (sub:Subcategory {key: 'output'}), (t:NodeTypeMeta {label: 'PageL10n'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

// SEO Intelligence (3 types)
MERGE (t_seokeywordl10n:NodeTypeMeta {label: 'SEOKeywordL10n'})
ON CREATE SET
  t_seokeywordl10n.display_name = 'SEOKeywordL10n',
  t_seokeywordl10n.yaml_path = 'models/nodes/shared/seo/seo-keyword-l10n.yaml',
  t_seokeywordl10n.created_at = datetime()
ON MATCH SET
  t_seokeywordl10n.updated_at = datetime();

MATCH (sub:Subcategory {key: 'seo'}), (t:NodeTypeMeta {label: 'SEOKeywordL10n'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_seokeywordmetrics:NodeTypeMeta {label: 'SEOKeywordMetrics'})
ON CREATE SET
  t_seokeywordmetrics.display_name = 'SEOKeywordMetrics',
  t_seokeywordmetrics.yaml_path = 'models/nodes/shared/seo/seo-keyword-metrics.yaml',
  t_seokeywordmetrics.created_at = datetime()
ON MATCH SET
  t_seokeywordmetrics.updated_at = datetime();

MATCH (sub:Subcategory {key: 'seo'}), (t:NodeTypeMeta {label: 'SEOKeywordMetrics'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_seominingrun:NodeTypeMeta {label: 'SEOMiningRun'})
ON CREATE SET
  t_seominingrun.display_name = 'SEOMiningRun',
  t_seominingrun.yaml_path = 'models/nodes/shared/seo/seo-mining-run.yaml',
  t_seominingrun.created_at = datetime()
ON MATCH SET
  t_seominingrun.updated_at = datetime();

MATCH (sub:Subcategory {key: 'seo'}), (t:NodeTypeMeta {label: 'SEOMiningRun'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

// GEO Intelligence (3 types)
MERGE (t_geominingrun:NodeTypeMeta {label: 'GEOMiningRun'})
ON CREATE SET
  t_geominingrun.display_name = 'GEOMiningRun',
  t_geominingrun.yaml_path = 'models/nodes/shared/geo/geo-mining-run.yaml',
  t_geominingrun.created_at = datetime()
ON MATCH SET
  t_geominingrun.updated_at = datetime();

MATCH (sub:Subcategory {key: 'geo'}), (t:NodeTypeMeta {label: 'GEOMiningRun'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_geoseedl10n:NodeTypeMeta {label: 'GEOSeedL10n'})
ON CREATE SET
  t_geoseedl10n.display_name = 'GEOSeedL10n',
  t_geoseedl10n.yaml_path = 'models/nodes/shared/geo/geo-seed-l10n.yaml',
  t_geoseedl10n.created_at = datetime()
ON MATCH SET
  t_geoseedl10n.updated_at = datetime();

MATCH (sub:Subcategory {key: 'geo'}), (t:NodeTypeMeta {label: 'GEOSeedL10n'})
MERGE (sub)-[:DEFINES_TYPE]->(t);

MERGE (t_geoseedmetrics:NodeTypeMeta {label: 'GEOSeedMetrics'})
ON CREATE SET
  t_geoseedmetrics.display_name = 'GEOSeedMetrics',
  t_geoseedmetrics.yaml_path = 'models/nodes/shared/geo/geo-seed-metrics.yaml',
  t_geoseedmetrics.created_at = datetime()
ON MATCH SET
  t_geoseedmetrics.updated_at = datetime();

MATCH (sub:Subcategory {key: 'geo'}), (t:NodeTypeMeta {label: 'GEOSeedMetrics'})
MERGE (sub)-[:DEFINES_TYPE]->(t);
