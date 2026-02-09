import { NextRequest, NextResponse } from 'next/server';
import Anthropic from '@anthropic-ai/sdk';
import { logger } from '@/lib/logger';
import { getSchema, formatSchemaForPrompt } from '@/lib/schemaCache';

// Validate API key at startup
const apiKey = process.env.ANTHROPIC_API_KEY;
if (!apiKey && process.env.NODE_ENV === 'production') {
  logger.error('Security', 'ANTHROPIC_API_KEY not configured');
}

const anthropic = new Anthropic({
  apiKey: apiKey || '',
});

/**
 * System prompt for Cypher generation (v10.1.0 schema)
 * Dynamic schema is injected from Neo4j via schemaCache.ts
 */
const SYSTEM_PROMPT = `You are an AI assistant that helps users explore a Neo4j knowledge graph for a native content generation system called "NovaNet".

## About NovaNet

NovaNet orchestrates **native content generation** (NOT translation) across 200+ locales. Content is generated natively per locale using Entities (invariant) that produce localized L10n nodes.

Key principle: Generation, NOT Translation
- Source → Translate → Target ❌
- Entity (invariant) → Generate natively → L10n (local) ✅

## Graph Schema (v10.1.0 fallback — live schema injected below if available)

### Standard Properties (all nodes)
All nodes have: key, display_name, icon, description, llm_context, created_at, updated_at

### Node Types (43 total, organized by layer)

**Config (3)** — Project-level configuration
- Project: Root node with brand_name, core_values, category
- ProjectContent: Localized identity → FOR_LOCALE
- BrandIdentity: Visual identity (colors, fonts, style)

**Knowledge (7)** — Locale knowledge base
- Locale: BCP47 codes (language_code, country_code, fallback_chain)
- LocaleAdaptation: Formality, emoji policies, tone
- LocaleAudience: Communication style for locale
- LocaleCulture: Cultural norms (taboo_topics, positive_triggers)
- LocaleExpressions: Vocabulary (~17k) per locale
- LocaleFormatting: Date/number/currency formats
- LocaleSlugification: URL/slug patterns

**Foundation (9)** — Reusable atoms & rules
- LocalePatterns: Grammar and style patterns
- LocaleStyle: Writing style preferences
- LocaleTaboos: Topics to avoid
- LocaleTerms: Terminology glossary
- BlockPrompt: Sub-agent instructions
- BlockRules: Template generation rules
- BlockType: Block templates (category, structure)
- PagePrompt: Orchestrator instructions
- Slot: Content placeholder definitions

**Structure (4)** — Content structure
- Entity: Business entities (feature_category, is_core) — v10.3: was Concept
- Page: Website pages (instructions)
- Block: Content blocks (instructions)
- Intent: User search intents

**Semantic (2)** — Localized content
- EntityL10n: Localized entities (title, definition) → FOR_LOCALE — v10.3: was ConceptL10n
- BlockL10n: Localized block content → FOR_LOCALE

**Instruction (3)** — Generation jobs
- PageOutput: Assembled page content → FOR_LOCALE
- BlockOutput: Generated block content → FOR_LOCALE
- GenerationJob: Async generation task

**Output (3)** — Production artifacts
- OutputArtifact: Final content artifact
- EvaluationJob: Quality evaluation task
- EvaluationResult: Evaluation scores

**SEO (4)** — Search optimization
- SEOKeyword: Keywords (value, volume, difficulty, cpc, intent)
- SEOTarget: Page SEO targeting
- SEOKeywordMetrics: Performance metrics
- SEOMiningRun: Mining job metadata

### Key Relationships (61 arc types)

**Ownership**: HAS_CONCEPT, HAS_PAGE, HAS_BLOCK, OF_TYPE, SUPPORTS_LOCALE, HAS_BRAND_IDENTITY
**Localization**: FOR_LOCALE, HAS_L10N, HAS_OUTPUT, FALLBACK_TO, VARIANT_OF
**Semantic**: USES_ENTITY, SEMANTIC_LINK, SUBTOPIC_OF, LINKS_TO, SATISFIES_INTENT
**Generation**: GENERATED, ASSEMBLES, TRIGGERED_BY, USES_PROMPT, COMPILED_FROM
**Mining**: HAS_SEO_TARGET, HAS_METRICS, SEO_MINES, TARGETS_KEYWORD

## Your Task
1. Understand the user's natural language query
2. Generate an appropriate Cypher query
3. Explain what the query does in simple terms

Always respond in this JSON format:
{
  "response": "Human-readable explanation of what was found/queried",
  "cypherQuery": "MATCH (n) RETURN n LIMIT 10"
}

Use LIMIT to prevent overwhelming results. Default to LIMIT 100 for large queries.
`;

export async function POST(request: NextRequest) {
  try {
    // Check request body size (50KB max - prevents DoS)
    const contentLength = parseInt(request.headers.get('content-length') || '0', 10);
    if (contentLength > 50000) {
      return NextResponse.json(
        { error: 'Request body too large' },
        { status: 413 }
      );
    }

    // Validate API key
    if (!apiKey) {
      return NextResponse.json(
        { error: 'AI service not configured' },
        { status: 503 }
      );
    }

    const { message } = await request.json();

    // Validate message
    if (!message || typeof message !== 'string') {
      return NextResponse.json(
        { error: 'Message is required' },
        { status: 400 }
      );
    }

    const trimmedMessage = message.trim();
    if (trimmedMessage.length === 0) {
      return NextResponse.json(
        { error: 'Message cannot be empty' },
        { status: 400 }
      );
    }

    if (trimmedMessage.length > 4000) {
      return NextResponse.json(
        { error: 'Message too long (max 4000 characters)' },
        { status: 400 }
      );
    }

    const startTime = Date.now();

    // Fetch cached schema and inject into prompt
    let dynamicSystemPrompt = SYSTEM_PROMPT;
    try {
      const schema = await getSchema();
      if (schema.nodeTypes.length > 0) {
        const schemaPrompt = formatSchemaForPrompt(schema);
        dynamicSystemPrompt = `${SYSTEM_PROMPT}\n\n${schemaPrompt}`;
        logger.info('API', `Schema injected: ${schema.nodeTypes.length} node types`);
      }
    } catch (error) {
      logger.warn('API', 'Failed to fetch schema, using static prompt', error);
    }

    // Call Claude API
    const response = await anthropic.messages.create({
      model: 'claude-sonnet-4-20250514',
      max_tokens: 1024,
      system: dynamicSystemPrompt,
      messages: [
        {
          role: 'user',
          content: message,
        },
      ],
    });

    // Extract text content
    const textContent = response.content.find((c) => c.type === 'text');
    if (!textContent || textContent.type !== 'text') {
      throw new Error('No text response from Claude');
    }

    // Parse JSON response
    let parsedResponse;
    try {
      // Try to extract JSON from the response
      const jsonMatch = textContent.text.match(/\{[\s\S]*\}/);
      if (jsonMatch) {
        parsedResponse = JSON.parse(jsonMatch[0]);
      } else {
        // Fallback if no JSON found
        parsedResponse = {
          response: textContent.text,
          cypherQuery: null,
        };
      }
    } catch {
      parsedResponse = {
        response: textContent.text,
        cypherQuery: null,
      };
    }

    const duration = Date.now() - startTime;

    return NextResponse.json({
      response: parsedResponse.response,
      cypherQuery: parsedResponse.cypherQuery,
      duration,
    });
  } catch (error) {
    logger.error('API', 'Chat error', error);

    const isDev = process.env.NODE_ENV === 'development';
    return NextResponse.json(
      {
        error: 'Failed to process request',
        ...(isDev && { details: error instanceof Error ? error.message : String(error) })
      },
      { status: 500 }
    );
  }
}
