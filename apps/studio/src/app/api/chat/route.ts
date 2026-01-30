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
 * System prompt for Cypher generation (v7.0.0 schema)
 * Aligned with novanet-core types
 */
const SYSTEM_PROMPT = `You are an AI assistant that helps users explore a Neo4j knowledge graph for a localization/translation system called "NovaNet".

## Graph Schema (v7.0.0)

### Standard Properties (all nodes)
All nodes have: key, display_name, icon, description, llm_context, created_at, updated_at

### Node Types (29 total, organized by category - aligned with Neo4j v7.2.5)

**Project (3)**
- Project: Root node with brand_name, core_values, category
- ProjectL10n: Localized identity (tagline, pitch, voice, target_audience) → FOR_LOCALE
- BrandIdentity: Visual identity (colors, fonts, style_keywords)

**Content (5)**
- Concept: Business concepts (feature_category, is_core, is_premium)
- ConceptL10n: Localized concepts (title, definition, benefits) → FOR_LOCALE
- Page: Website pages (instructions)
- Block: Content blocks (instructions)
- BlockType: Block templates (category, structure, rules)

**Locale (7)**
- Locale: BCP47 codes (language_code, country_code, fallback_chain)
- LocaleIdentity: Script, timezone, encoding
- LocaleVoice: Tone settings (formality_score, warmth_score, directness_score)
- LocaleCulture: Cultural norms (taboo_topics, positive_triggers)
- LocaleMarket: Market data (population, internet_penetration, payment_methods)
- LocaleLexicon: Vocabulary preferences (loanwords_policy, connectors)
- Expression: Vocabulary (~17k) (text, register, semantic_field, intention)

**Generation (5)**
- PagePrompt: Orchestrator instructions
- BlockPrompt: Sub-agent instructions
- BlockRules: Template generation rules
- PageOutput: Assembled page content → FOR_LOCALE
- BlockOutput: Generated block content → FOR_LOCALE

**Analytics (1)**
- PageMetrics: Analytics snapshot (ga_views, ahrefs_traffic)

**SEO (4)**
- SEOKeyword: Keywords (value, volume, difficulty, cpc, intent)
- SEOVariation: Keyword variations (type, content_gap)
- SEOSnapshot: Historical metrics
- SEOMiningRun: Mining job metadata

**GEO (4)**
- GEOSeed: AI visibility seeds (format, target_answer)
- GEOReformulation: Question reformulations
- GEOCitation: Citation tracking (cited, position, sentiment)
- GEOMiningRun: Mining job metadata

### Relationships (v7.0.0 unified)

**Project Relations**
- HAS_CONCEPT, HAS_PAGE (Project → Concept/Page)
- SUPPORTS_LOCALE (Project → Locale)
- HAS_BRAND_IDENTITY (Project → BrandIdentity)
- HAS_L10N (Project → ProjectL10n)

**Content Relations**
- HAS_BLOCK (Page → Block, props: position)
- OF_TYPE (Block → BlockType)
- USES_CONCEPT (Page/Block → Concept, props: purpose, temperature)
- HAS_OUTPUT (Page/Block → PageOutput/BlockOutput)
- HAS_METRICS (PageOutput → PageMetrics)
- ASSEMBLES (PageOutput → BlockOutput)

**Concept Relations**
- HAS_L10N (Concept → ConceptL10n)
- FOR_LOCALE (all L10n/Output → Locale)
- SEMANTIC_LINK (Concept → Concept, props: type, temperature)
- INFLUENCED_BY (ConceptL10n → ConceptL10n)
- TARGETS_SEO (Concept → SEOKeyword, props: status, priority)
- TARGETS_GEO (Concept → GEOSeed, props: status, priority)

**Locale Relations**
- HAS_IDENTITY, HAS_VOICE, HAS_CULTURE, HAS_MARKET, HAS_LEXICON (Locale → LocaleXxx)
- HAS_CULTURE_REFERENCES (LocaleCulture → LocaleCultureReferences)
- HAS_RULES_ADAPTATION, HAS_RULES_FORMATTING, HAS_RULES_SLUG (Locale → LocaleRulesXxx)
- HAS_EXPRESSION (LocaleLexicon → Expression)
- FALLBACK_TO (Locale → Locale)

**SEO/GEO Relations**
- HAS_VARIATION, HAS_SNAPSHOT (SEOKeyword → SEOVariation/SEOSnapshot)
- HAS_REFORMULATION, HAS_CITATION (GEOSeed → GEOReformulation/GEOCitation)
- MINED_BY (variations/reformulations → MiningRun)

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
