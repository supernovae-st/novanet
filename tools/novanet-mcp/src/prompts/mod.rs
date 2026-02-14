//! MCP Prompts module
//!
//! Phase 3: 6 MCP Prompts implementing Full RAG pattern for AI agents.
//!
//! Prompts:
//! - cypher_query: Natural language → schema-aware Cypher
//! - cypher_explain: Query results → business context explanation
//! - block_generation: Single block generation context
//! - page_generation: Full page orchestration context
//! - entity_analysis: Deep entity analysis with usage context
//! - locale_briefing: Locale voice and culture summary

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Prompt argument definition
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PromptArgument {
    /// Argument name
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Whether this argument is required
    pub required: bool,
}

/// MCP Prompt message
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct PromptMessage {
    /// Message role (user, assistant, system)
    pub role: String,
    /// Message content
    pub content: String,
}

/// Prompt definition for MCP protocol
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct PromptDefinition {
    /// Unique prompt identifier
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Required and optional arguments
    pub arguments: Vec<PromptArgument>,
}

/// Rendered prompt ready for execution
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct RenderedPrompt {
    /// Prompt description for context
    pub description: String,
    /// Messages in the prompt
    pub messages: Vec<PromptMessage>,
}

/// List all available prompts
pub fn list_prompts() -> Vec<PromptDefinition> {
    vec![
        // 1. cypher_query
        PromptDefinition {
            name: "cypher_query".to_string(),
            description: "Generate schema-aware Cypher queries from natural language. Returns a Cypher query that can be executed with novanet_query.".to_string(),
            arguments: vec![
                PromptArgument {
                    name: "intent".to_string(),
                    description: "Natural language description of what to query".to_string(),
                    required: true,
                },
                PromptArgument {
                    name: "constraints".to_string(),
                    description: "Additional constraints (realm, layer, limit)".to_string(),
                    required: false,
                },
            ],
        },
        // 2. cypher_explain
        PromptDefinition {
            name: "cypher_explain".to_string(),
            description: "Explain query results in business context. Takes a Cypher query and its results, returns human-readable explanation.".to_string(),
            arguments: vec![
                PromptArgument {
                    name: "query".to_string(),
                    description: "The Cypher query that was executed".to_string(),
                    required: true,
                },
                PromptArgument {
                    name: "results".to_string(),
                    description: "JSON array of query results".to_string(),
                    required: true,
                },
            ],
        },
        // 3. block_generation
        PromptDefinition {
            name: "block_generation".to_string(),
            description: "Generate context for a single block's content. Orchestrates traverse, assemble, and atoms for focused generation.".to_string(),
            arguments: vec![
                PromptArgument {
                    name: "block_key".to_string(),
                    description: "The block's key".to_string(),
                    required: true,
                },
                PromptArgument {
                    name: "locale".to_string(),
                    description: "Target locale (BCP-47)".to_string(),
                    required: true,
                },
                PromptArgument {
                    name: "token_budget".to_string(),
                    description: "Max tokens for context (default: 10000)".to_string(),
                    required: false,
                },
            ],
        },
        // 4. page_generation
        PromptDefinition {
            name: "page_generation".to_string(),
            description: "Orchestrate full page generation across all blocks. Discovers page structure, assembles context for each block, includes cross-page references.".to_string(),
            arguments: vec![
                PromptArgument {
                    name: "page_key".to_string(),
                    description: "The page's key".to_string(),
                    required: true,
                },
                PromptArgument {
                    name: "locale".to_string(),
                    description: "Target locale (BCP-47)".to_string(),
                    required: true,
                },
                PromptArgument {
                    name: "token_budget".to_string(),
                    description: "Max tokens for context (default: 50000)".to_string(),
                    required: false,
                },
            ],
        },
        // 5. entity_analysis
        PromptDefinition {
            name: "entity_analysis".to_string(),
            description: "Deep analysis of an entity with usage context. Returns definition, locale adaptations, pages/blocks using it, and related entities.".to_string(),
            arguments: vec![
                PromptArgument {
                    name: "entity_key".to_string(),
                    description: "The entity's key".to_string(),
                    required: true,
                },
                PromptArgument {
                    name: "locale".to_string(),
                    description: "Analysis locale".to_string(),
                    required: true,
                },
            ],
        },
        // 6. locale_briefing
        PromptDefinition {
            name: "locale_briefing".to_string(),
            description: "Locale voice and culture summary for content generation. Returns voice guidelines, cultural context, technical formats, and example phrases.".to_string(),
            arguments: vec![
                PromptArgument {
                    name: "locale_key".to_string(),
                    description: "Locale key (BCP-47)".to_string(),
                    required: true,
                },
            ],
        },
    ]
}

/// Get a specific prompt by name
pub fn get_prompt(name: &str) -> Option<PromptDefinition> {
    list_prompts().into_iter().find(|p| p.name == name)
}

/// Render a prompt with arguments
pub fn render_prompt(
    name: &str,
    arguments: &serde_json::Map<String, serde_json::Value>,
) -> Option<RenderedPrompt> {
    match name {
        "cypher_query" => Some(render_cypher_query(arguments)),
        "cypher_explain" => Some(render_cypher_explain(arguments)),
        "block_generation" => Some(render_block_generation(arguments)),
        "page_generation" => Some(render_page_generation(arguments)),
        "entity_analysis" => Some(render_entity_analysis(arguments)),
        "locale_briefing" => Some(render_locale_briefing(arguments)),
        _ => None,
    }
}

/// Render cypher_query prompt
fn render_cypher_query(args: &serde_json::Map<String, serde_json::Value>) -> RenderedPrompt {
    let intent = args
        .get("intent")
        .and_then(|v| v.as_str())
        .unwrap_or("query the graph");
    let constraints = args
        .get("constraints")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let system_content = r#"You are a Cypher query expert for the NovaNet knowledge graph.

## Schema Overview

**Realms (2)**: shared (39 nodes, READ-ONLY), org (21 nodes)
**Layers (10)**: config, locale, geography, knowledge, foundation, structure, semantic, instruction, output
**Traits (5)**: defined, authored, imported, generated, retrieved

**Key Node Types:**
- Entity (org/semantic) - Core semantic entities
- EntityContent (org/semantic) - Locale-specific content for entities
- Page (org/structure) - Website pages
- Block (org/structure) - Content blocks within pages
- Locale (shared/config) - BCP-47 locale definitions
- Class (schema) - Node type definitions with :Schema label (v0.12.0: was Kind)

**Key Relationships:**
- HAS_CONTENT: Entity → EntityContent (ownership)
- HAS_BLOCK: Page → Block (ownership)
- USES_ENTITY: Block → Entity (semantic)
- FOR_LOCALE: EntityContent → Locale (localization)
- OF_CLASS: Instance → Class (schema-bridge, v0.12.0: was OF_KIND)

## Rules

1. Only generate READ-ONLY queries (MATCH, RETURN, WITH, WHERE, ORDER BY, LIMIT)
2. Always include LIMIT (default 100) to prevent unbounded results
3. Use parameterized queries with $param syntax for user inputs
4. Filter by realm/layer when appropriate for performance
5. Use :Schema label for schema queries (v0.12.0: was :Meta)

## Examples

```cypher
-- Get entities with content for a locale
MATCH (e:Entity)-[:HAS_CONTENT]->(ec:EntityContent)-[:FOR_LOCALE]->(l:Locale {key: $locale})
RETURN e.key, e.name, ec.title, ec.description
LIMIT 50

-- Get page structure
MATCH (p:Page {key: $key})-[:HAS_BLOCK]->(b:Block)
OPTIONAL MATCH (b)-[:OF_TYPE]->(bt:BlockType)
RETURN p.key, collect({block: b.key, type: bt.name}) AS blocks

-- Schema overview (v0.12.0: Class, was Kind)
MATCH (c:Class)
WITH c.realm AS realm, c.layer AS layer, collect(c.name) AS classes
RETURN realm, layer, classes ORDER BY realm, layer
```"#;

    let user_content = format!(
        "Generate a Cypher query for: {}\n\n{}",
        intent,
        if constraints.is_empty() {
            String::new()
        } else {
            format!("Additional constraints: {}", constraints)
        }
    );

    RenderedPrompt {
        description: "Generate a schema-aware Cypher query from natural language".to_string(),
        messages: vec![
            PromptMessage {
                role: "system".to_string(),
                content: system_content.to_string(),
            },
            PromptMessage {
                role: "user".to_string(),
                content: user_content,
            },
        ],
    }
}

/// Render cypher_explain prompt
fn render_cypher_explain(args: &serde_json::Map<String, serde_json::Value>) -> RenderedPrompt {
    let query = args
        .get("query")
        .and_then(|v| v.as_str())
        .unwrap_or("MATCH (n) RETURN n LIMIT 10");
    let results = args.get("results").and_then(|v| v.as_str()).unwrap_or("[]");

    let system_content = r#"You are an expert at explaining NovaNet knowledge graph query results in business context.

## NovaNet Context

NovaNet is a native content generation system for multilingual websites. It does NOT translate - it generates content natively per locale from universal entity definitions.

**Key Concepts:**
- Entity: Universal semantic concept (e.g., "QR Code Generator")
- EntityContent: Locale-specific content for an entity
- Page/Block: Structure for generated content
- Locale: BCP-47 locale with voice, culture, formatting settings

## Your Task

Explain the query results in business terms that a content manager or marketer would understand. Focus on:
1. What was queried (entities, pages, locales, etc.)
2. Key insights from the results
3. Actionable observations
4. Any data quality issues noticed"#;

    let user_content = format!(
        "## Query Executed\n```cypher\n{}\n```\n\n## Results\n```json\n{}\n```\n\nExplain these results in business context.",
        query, results
    );

    RenderedPrompt {
        description: "Explain query results in business context".to_string(),
        messages: vec![
            PromptMessage {
                role: "system".to_string(),
                content: system_content.to_string(),
            },
            PromptMessage {
                role: "user".to_string(),
                content: user_content,
            },
        ],
    }
}

/// Render block_generation prompt
fn render_block_generation(args: &serde_json::Map<String, serde_json::Value>) -> RenderedPrompt {
    let block_key = args
        .get("block_key")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown-block");
    let locale = args
        .get("locale")
        .and_then(|v| v.as_str())
        .unwrap_or("en-US");
    let token_budget = args
        .get("token_budget")
        .and_then(|v| v.as_u64())
        .unwrap_or(10000);

    let system_content = r#"You are an AI agent orchestrating content generation for a single block in NovaNet.

## Generation Philosophy

**CRITICAL: Generation, NOT Translation**
Content is generated NATIVELY in the target locale. The LLM receives all context in the target locale and generates natively - there is NO source language.

## Tool Orchestration

Use these MCP tools in sequence:

1. **novanet_traverse** - Get block structure and connected entities
   ```json
   {
     "start_key": "<block_key>",
     "max_depth": 2,
     "direction": "outgoing",
     "arc_families": ["ownership", "semantic"]
   }
   ```

2. **novanet_assemble** - Get semantic context for entities
   ```json
   {
     "focus_key": "<block_key>",
     "locale": "<locale>",
     "token_budget": <budget/2>,
     "include_entities": true
   }
   ```

3. **novanet_atoms** - Get locale-specific knowledge atoms
   ```json
   {
     "locale": "<locale>",
     "atom_type": "all",
     "limit": 50
   }
   ```

Or use the composite **novanet_generate** tool:
```json
{
  "focus_key": "<block_key>",
  "locale": "<locale>",
  "mode": "block",
  "token_budget": <budget>
}
```

## Output Format

After gathering context, generate the block content with:
- Title (if applicable)
- Body content in the target locale
- Any CTAs or special elements
- Use `{{anchor:page_key|display text}}` for cross-page links"#;

    let user_content = format!(
        "Generate content for block `{}` in locale `{}`.\n\nToken budget: {} tokens.\n\nFirst, gather context using the tools, then generate the content.",
        block_key, locale, token_budget
    );

    RenderedPrompt {
        description: format!("Block generation context for {} ({})", block_key, locale),
        messages: vec![
            PromptMessage {
                role: "system".to_string(),
                content: system_content.to_string(),
            },
            PromptMessage {
                role: "user".to_string(),
                content: user_content,
            },
        ],
    }
}

/// Render page_generation prompt
fn render_page_generation(args: &serde_json::Map<String, serde_json::Value>) -> RenderedPrompt {
    let page_key = args
        .get("page_key")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown-page");
    let locale = args
        .get("locale")
        .and_then(|v| v.as_str())
        .unwrap_or("en-US");
    let token_budget = args
        .get("token_budget")
        .and_then(|v| v.as_u64())
        .unwrap_or(50000);

    let system_content = r#"You are an AI orchestrator for full page generation in NovaNet.

## Generation Philosophy

**CRITICAL: Generation, NOT Translation**
All content is generated NATIVELY in the target locale from universal entity definitions. There is no source language.

## Page Generation Flow

1. **Discover Structure** - Get page with all blocks
   ```json
   novanet_traverse {
     "start_key": "<page_key>",
     "max_depth": 3,
     "arc_families": ["ownership"],
     "arc_kinds": ["HAS_BLOCK"]
   }
   ```

2. **For each block**, gather context and generate:
   - Use `novanet_generate` with mode "block" for each block
   - Maintain consistent voice across blocks

3. **Generate page-level metadata**:
   - Meta title
   - Meta description
   - Canonical URL

4. **Handle cross-page references**:
   - Use `{{anchor:page_key|display text}}` syntax
   - Context anchors are resolved via REFERENCES_PAGE arcs

## Composite Approach

For simpler orchestration, use **novanet_generate** with page mode:
```json
{
  "focus_key": "<page_key>",
  "locale": "<locale>",
  "mode": "page",
  "token_budget": <budget>
}
```

This returns a complete context including:
- All block structures
- Entity definitions
- Knowledge atoms
- Context anchors

## Output Format

Return structured page content:
```json
{
  "title": "Page title in locale",
  "meta_description": "SEO description",
  "blocks": [
    { "key": "block-1", "content": "..." },
    { "key": "block-2", "content": "..." }
  ]
}
```"#;

    let user_content = format!(
        "Generate full page content for `{}` in locale `{}`.\n\nToken budget: {} tokens.\n\nOrchestrate the page generation, discovering all blocks and generating content for each.",
        page_key, locale, token_budget
    );

    RenderedPrompt {
        description: format!(
            "Page generation orchestration for {} ({})",
            page_key, locale
        ),
        messages: vec![
            PromptMessage {
                role: "system".to_string(),
                content: system_content.to_string(),
            },
            PromptMessage {
                role: "user".to_string(),
                content: user_content,
            },
        ],
    }
}

/// Render entity_analysis prompt
fn render_entity_analysis(args: &serde_json::Map<String, serde_json::Value>) -> RenderedPrompt {
    let entity_key = args
        .get("entity_key")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown-entity");
    let locale = args
        .get("locale")
        .and_then(|v| v.as_str())
        .unwrap_or("en-US");

    let system_content = r#"You are an entity analyst for the NovaNet knowledge graph.

## Entity Model

In NovaNet:
- **Entity** (defined): Core semantic concept, defined once
- **EntityContent** (authored): Locale-specific content (title, description, etc.)
- **EntityCategory**: Categorical grouping (product, service, concept, etc.)

## Analysis Queries

Use these to gather comprehensive entity data:

1. **Entity definition + category**:
   ```cypher
   MATCH (e:Entity {key: $key})
   OPTIONAL MATCH (e)-[:BELONGS_TO]->(c:EntityCategory)
   RETURN e, c.category_key AS category
   ```

2. **Locale adaptations**:
   ```cypher
   MATCH (e:Entity {key: $key})-[:HAS_CONTENT]->(ec:EntityContent)-[:FOR_LOCALE]->(l:Locale)
   RETURN l.key AS locale, ec.title, ec.description, ec.keywords
   ```

3. **Usage in content**:
   ```cypher
   MATCH (b:Block)-[:USES_ENTITY]->(e:Entity {key: $key})
   MATCH (p:Page)-[:HAS_BLOCK]->(b)
   RETURN DISTINCT p.key AS page, b.key AS block
   ```

4. **Related entities**:
   ```cypher
   MATCH (e:Entity {key: $key})-[r:SIMILAR_TO|REFERENCES]-(related:Entity)
   RETURN related.key, related.name, type(r) AS relationship
   ```

## Output

Provide a comprehensive analysis:
1. Entity definition and category
2. Locale coverage (which locales have content)
3. Content usage (which pages/blocks use this entity)
4. Related entities
5. Recommendations for improvement"#;

    let user_content = format!(
        "Analyze entity `{}` with focus on locale `{}`.\n\nProvide comprehensive analysis including definition, adaptations, usage, and relationships.",
        entity_key, locale
    );

    RenderedPrompt {
        description: format!("Deep analysis of entity {} ({})", entity_key, locale),
        messages: vec![
            PromptMessage {
                role: "system".to_string(),
                content: system_content.to_string(),
            },
            PromptMessage {
                role: "user".to_string(),
                content: user_content,
            },
        ],
    }
}

/// Render locale_briefing prompt
fn render_locale_briefing(args: &serde_json::Map<String, serde_json::Value>) -> RenderedPrompt {
    let locale_key = args
        .get("locale_key")
        .and_then(|v| v.as_str())
        .unwrap_or("en-US");

    let system_content = r#"You are a locale expert providing briefings for content generation in NovaNet.

## Locale Knowledge Structure

In NovaNet, each locale has:
- **Locale** (config): BCP-47 definition (language, region, script)
- **Culture**: Cultural context and references
- **Style**: Voice guidelines (formality, tone)
- **Formatting**: Technical formats (date, number, currency)
- **Knowledge Atoms**: Domain-specific vocabulary
  - Terms: Technical terms with definitions
  - Expressions: Idiomatic expressions
  - Patterns: Text templates
  - CultureRefs: Cultural references to use
  - Taboos: Content to avoid
  - AudienceTraits: Audience characteristics

## Briefing Queries

1. **Locale configuration**:
   ```cypher
   MATCH (l:Locale {key: $key})
   RETURN l.key, l.language, l.region, l.script, l.direction
   ```

2. **Voice and style**:
   ```cypher
   MATCH (l:Locale {key: $key})-[:HAS_STYLE]->(s:Style)
   RETURN s.formality, s.tone, s.vocabulary_level
   ```

3. **Knowledge atom counts**:
   ```cypher
   MATCH (l:Locale {key: $key})
   OPTIONAL MATCH (l)-[:HAS_TERMS]->(ts:TermSet)-[:CONTAINS_TERM]->(t:Term)
   OPTIONAL MATCH (l)-[:HAS_EXPRESSIONS]->(es:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e:Expression)
   RETURN count(DISTINCT t) AS terms, count(DISTINCT e) AS expressions
   ```

4. **Sample expressions**:
   ```cypher
   MATCH (l:Locale {key: $key})-[:HAS_EXPRESSIONS]->(es:ExpressionSet)-[:CONTAINS_EXPRESSION]->(e:Expression)
   RETURN e.text, e.semantic_field LIMIT 10
   ```

## Output

Provide a briefing that includes:
1. Language and region overview
2. Voice guidelines (formality level, tone, style)
3. Cultural considerations
4. Technical formatting (dates, numbers, currency)
5. Example phrases demonstrating the voice
6. Things to avoid (taboos, cultural sensitivities)"#;

    let user_content = format!(
        "Provide a locale briefing for `{}`.\n\nInclude voice guidelines, cultural context, and example phrases for content generation.",
        locale_key
    );

    RenderedPrompt {
        description: format!("Locale briefing for {}", locale_key),
        messages: vec![
            PromptMessage {
                role: "system".to_string(),
                content: system_content.to_string(),
            },
            PromptMessage {
                role: "user".to_string(),
                content: user_content,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_prompts_returns_six() {
        let prompts = list_prompts();
        assert_eq!(prompts.len(), 6);
    }

    #[test]
    fn test_get_prompt_cypher_query() {
        let prompt = get_prompt("cypher_query");
        assert!(prompt.is_some());
        let p = prompt.unwrap();
        assert_eq!(p.name, "cypher_query");
        assert_eq!(p.arguments.len(), 2);
    }

    #[test]
    fn test_get_prompt_not_found() {
        let prompt = get_prompt("nonexistent");
        assert!(prompt.is_none());
    }

    #[test]
    fn test_render_cypher_query() {
        let mut args = serde_json::Map::new();
        args.insert("intent".to_string(), serde_json::json!("find all entities"));

        let rendered = render_prompt("cypher_query", &args);
        assert!(rendered.is_some());
        let r = rendered.unwrap();
        assert_eq!(r.messages.len(), 2);
        assert!(r.messages[1].content.contains("find all entities"));
    }

    #[test]
    fn test_render_block_generation() {
        let mut args = serde_json::Map::new();
        args.insert("block_key".to_string(), serde_json::json!("hero-block"));
        args.insert("locale".to_string(), serde_json::json!("fr-FR"));

        let rendered = render_prompt("block_generation", &args);
        assert!(rendered.is_some());
        let r = rendered.unwrap();
        assert!(r.description.contains("hero-block"));
        assert!(r.description.contains("fr-FR"));
    }

    #[test]
    fn test_render_page_generation() {
        let mut args = serde_json::Map::new();
        args.insert("page_key".to_string(), serde_json::json!("homepage"));
        args.insert("locale".to_string(), serde_json::json!("de-DE"));

        let rendered = render_prompt("page_generation", &args);
        assert!(rendered.is_some());
        let r = rendered.unwrap();
        assert!(r.messages[1].content.contains("homepage"));
        assert!(r.messages[1].content.contains("de-DE"));
    }

    #[test]
    fn test_render_entity_analysis() {
        let mut args = serde_json::Map::new();
        args.insert(
            "entity_key".to_string(),
            serde_json::json!("qr-code-generator"),
        );
        args.insert("locale".to_string(), serde_json::json!("ja-JP"));

        let rendered = render_prompt("entity_analysis", &args);
        assert!(rendered.is_some());
        let r = rendered.unwrap();
        assert!(r.messages[1].content.contains("qr-code-generator"));
    }

    #[test]
    fn test_render_locale_briefing() {
        let mut args = serde_json::Map::new();
        args.insert("locale_key".to_string(), serde_json::json!("es-MX"));

        let rendered = render_prompt("locale_briefing", &args);
        assert!(rendered.is_some());
        let r = rendered.unwrap();
        assert!(r.messages[1].content.contains("es-MX"));
    }

    #[test]
    fn test_all_prompts_have_arguments() {
        for prompt in list_prompts() {
            assert!(
                !prompt.arguments.is_empty(),
                "{} has no arguments",
                prompt.name
            );
            // At least one required argument
            assert!(
                prompt.arguments.iter().any(|a| a.required),
                "{} has no required arguments",
                prompt.name
            );
        }
    }
}
