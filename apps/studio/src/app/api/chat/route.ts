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
 * System prompt for Cypher generation (v11.0 schema)
 * Dynamic schema is injected from Neo4j via schemaCache.ts
 */
const SYSTEM_PROMPT = `You are an AI assistant that helps users explore a Neo4j knowledge graph for a native content generation system called "NovaNet".

## About NovaNet

NovaNet orchestrates **native content generation** (NOT translation) across 200+ locales. Content is generated natively per locale using Entities (defined) that produce EntityContent nodes.

Key principle: Generation, NOT Translation
- Source → Translate → Target ❌
- Entity (defined) → Generate natively → EntityContent (local) ✅

## Data Origin Traits (v11.8 - ADR-024)
- **defined**: Structurally fixed, version-controlled definitions
- **authored**: Human-authored locale-specific content
- **imported**: External data from authoritative sources
- **generated**: LLM-generated output
- **retrieved**: Computed/aggregated from external APIs

## Graph Schema (v11.0 fallback — live schema injected below if available)

### Standard Properties (all nodes)
All nodes have: key, display_name, icon, description, llm_context, created_at, updated_at

### 2-Realm Architecture (v11.0)
- **GLOBAL** (2 layers): config, locale-knowledge — Universal, READ-ONLY
- **TENANT** (7 layers): config, foundation, structure, semantic, instruction, seo, output — Business-specific

### Node Types (65 total, organized by realm/layer)

**Global/Config** — Locale definitions
- Locale: BCP47 codes (language_code, country_code, fallback_chain)
- Formatting: Date/number/currency formats
- Style: Writing style preferences
- Slugification: URL/slug patterns
- Adaptation: Formality, emoji policies, tone
- Market, Culture: Cultural contexts

**Global/Locale-Knowledge** — Knowledge atoms
- Term, TermSet: Terminology glossary
- Expression, ExpressionSet: Vocabulary expressions
- Pattern, PatternSet: Grammar patterns
- Taboo, TabooSet: Topics to avoid
- CultureRef, CultureSet: Cultural references
- AudienceTrait, AudienceSet: Audience characteristics

**Tenant/Config** — Organization setup
- Tenant, Organization: Root tenant node
- Project, ProjectContent: Project with authored identity
- BrandIdentity: Visual identity

**Org/Structure** — Content structure
- Page, Block, ContentSlot: Page and block structure
- PageStructure, BlockType: Templates (v11.8: PageType → PageStructure)

**Tenant/Semantic** — Business content
- Entity: Invariant business entities
- EntityContent: Localized entity content → FOR_LOCALE
- AudiencePersona, ChannelSurface: Targeting

**Tenant/SEO** — Search optimization (v11.0: moved to tenant)
- SEOKeyword, SEOQuestion, SEOComparison, SEOPreposition: Keywords
- GEOQuery, GEOAnswer, GEOMetrics: AI search queries
- SEOKeywordMetrics: Analytics (v11.5: SEO in shared/knowledge)

**Tenant/Output** — Generation results
- PageGenerated, BlockGenerated: Generated content → FOR_LOCALE
- OutputArtifact: Pipeline output (v11.2: job nodes removed)

### Key Relationships (124 arc types)

**Ownership**: HAS_PAGE, HAS_BLOCK, HAS_ENTITY, OF_TYPE, SUPPORTS_LOCALE
**Localization**: FOR_LOCALE, HAS_CONTENT, CONTENT_OF, FALLBACK_TO
**Semantic**: USES_ENTITY, SEMANTIC_LINK, SUBTOPIC_OF, TARGETS
**Generation**: HAS_GENERATED, GENERATED_FOR, USES_PROMPT
**Mining**: HAS_SEO_KEYWORDS, HAS_GEO_QUERIES, TARGETS

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
