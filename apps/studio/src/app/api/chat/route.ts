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
 * System prompt for Cypher generation (v0.12.0 schema)
 * Dynamic schema is injected from Neo4j via schemaCache.ts
 */
const SYSTEM_PROMPT = `You are an AI assistant that helps users explore a Neo4j knowledge graph for a native content generation system called "NovaNet".

## About NovaNet

NovaNet orchestrates **native content generation** (NOT translation) across 200+ locales. Content is generated natively per locale using Entities (defined) that produce EntityNative nodes.

Key principle: Generation, NOT Translation
- Source → Translate → Target ❌
- Entity (defined) → Generate natively → EntityNative (local) ✅

## Data Origin Traits (v0.12.0 - ADR-024)
- **defined**: Structurally fixed, version-controlled definitions
- **authored**: Human-authored locale-specific content
- **imported**: External data from authoritative sources
- **generated**: LLM-generated output
- **retrieved**: Computed/aggregated from external APIs

## Graph Schema (v0.12.0 fallback — live schema injected below if available)

### Standard Properties (all nodes)
All nodes have: key, display_name, node_class, content, triggers, provenance, created_at, updated_at

### 2-Realm Architecture (v0.12.0)
- **SHARED** (4 layers): config, locale, geography, knowledge — Universal, READ-ONLY (40 nodes)
- **ORG** (6 layers): config, foundation, structure, semantic, instruction, output (21 nodes)

### Node Types (61 total, organized by realm/layer)

**Shared/Config** — Locale and entity category definitions
- Locale: BCP47 codes (language_code, country_code, fallback_chain)
- EntityCategory: Entity type taxonomy

**Shared/Locale** — Locale settings
- Formatting, Style, Slugification, Adaptation: Locale preferences
- LocaleVoice, LocaleCulture: Voice and cultural context

**Shared/Geography** — Geographic hierarchy
- Continent, GeoRegion, Country, SubRegion, City, GeoFeature

**Shared/Knowledge** — Knowledge atoms and SEO/GEO intelligence
- Term, TermSet: Terminology glossary
- Expression, ExpressionSet: Vocabulary expressions
- Pattern, PatternSet: Grammar patterns
- CultureRef, CultureSet: Cultural references
- Taboo, TabooSet: Topics to avoid
- AudienceTrait, AudienceSet: Audience characteristics
- SEOKeyword, SEOKeywordMetrics: SEO data
- GEOQuery, GEOAnswer: AI search data (GEO = Generative Engine Optimization)

**Org/Config** — Organization setup
- OrgConfig: Root organization configuration

**Org/Foundation** — Project identity (v0.12.4: Brand Architecture)
- Project, Brand, BrandDesign, BrandPrinciples, PromptStyle, ProjectNative

**Org/Structure** — Content structure
- Page, Block, ContentSlot: Page and block hierarchy

**Org/Semantic** — Business content
- Entity: Defined business entities
- EntityNative: Authored entity content → FOR_LOCALE
- AudiencePersona, ChannelSurface: Targeting

**Org/Instruction** — Generation instructions (v0.12.4: PageInstruction removed)
- BlockType, BlockInstruction: Block templates and instructions
- PromptArtifact: Generated prompt artifacts

**Org/Output** — Generation results
- PageNative, BlockNative: Generated content → FOR_LOCALE
- OutputArtifact: Pipeline output

### Key Relationships (159 arc types — v0.20.0)

**Ownership**: HAS_PAGE, HAS_BLOCK, HAS_ENTITY, HAS_BRAND, SUPPORTS_LOCALE
**Localization**: FOR_LOCALE, HAS_NATIVE, NATIVE_OF, FALLBACK_TO
**Semantic**: USES_ENTITY, SEMANTIC_LINK, REFERENCES, HAS_KEYWORD, TARGETS
**Generation**: HAS_NATIVE, NATIVE_OF, HAS_INSTRUCTION
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
