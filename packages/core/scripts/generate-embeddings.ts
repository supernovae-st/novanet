#!/usr/bin/env npx tsx
/**
 * Generate Embeddings Script (v7.8.0)
 *
 * Generates OpenAI embeddings for Concept, ConceptL10n, and Page nodes.
 * Uses text-embedding-3-small (1536 dimensions).
 *
 * Usage:
 *   OPENAI_API_KEY=sk-... npx tsx scripts/generate-embeddings.ts
 *   OPENAI_API_KEY=sk-... npx tsx scripts/generate-embeddings.ts --dry-run
 *   OPENAI_API_KEY=sk-... npx tsx scripts/generate-embeddings.ts --node-type Concept
 */

import OpenAI from 'openai';
import { getDriver, closeDriver } from '../src/db/client.js';

// =============================================================================
// Configuration
// =============================================================================

const BATCH_SIZE = 100;
const MODEL = 'text-embedding-3-small';
const DIMENSIONS = 1536;

interface EmbeddingTarget {
  nodeType: 'Concept' | 'ConceptL10n' | 'Page';
  keyField: string;
  textBuilder: (node: Record<string, unknown>) => string;
  /** Custom Cypher for complex node types (e.g., ConceptL10n needs parent join) */
  customQuery?: string;
}

const TARGETS: EmbeddingTarget[] = [
  {
    nodeType: 'Concept',
    keyField: 'key',
    textBuilder: (n) => {
      const parts = [n.key, n.display_name, n.description, n.llm_context].filter(Boolean);
      return parts.join(' | ');
    },
  },
  {
    nodeType: 'ConceptL10n',
    keyField: 'title',
    // Include parent concept key for better embedding context
    // Pattern: "concept_key | title | definition | summary | purpose"
    textBuilder: (n) => {
      const parts = [n.concept_key, n.title, n.definition, n.summary, n.purpose].filter(Boolean);
      return parts.join(' | ');
    },
    // Custom query joins to parent Concept via L10N_OF relationship
    customQuery: `
      MATCH (n:ConceptL10n)-[:L10N_OF]->(c:Concept)
      WHERE n.embedding IS NULL
      RETURN elementId(n) AS key,
             elementId(n) AS id,
             n { .*, concept_key: c.key } AS data
      LIMIT 1000
    `,
  },
  {
    nodeType: 'Page',
    keyField: 'key',
    textBuilder: (n) => {
      const parts = [n.key, n.display_name, n.description, n.llm_context].filter(Boolean);
      return parts.join(' | ');
    },
  },
];

// =============================================================================
// Main Logic
// =============================================================================

interface NodeRecord {
  key: string;
  id: string;
  data: Record<string, unknown>;
}

async function generateEmbeddings(
  openai: OpenAI,
  target: EmbeddingTarget,
  dryRun: boolean = false
): Promise<number> {
  const driver = getDriver();
  const session = driver.session();
  let processed = 0;

  try {
    // Load nodes without embeddings
    // Use customQuery if available (for nodes needing joins like ConceptL10n)
    const query = target.customQuery ?? `
      MATCH (n:${target.nodeType})
      WHERE n.embedding IS NULL
      RETURN n.${target.keyField} AS key, elementId(n) AS id, n AS data
      LIMIT 1000
    `;

    const result = await session.run(query);

    const nodes: NodeRecord[] = result.records.map((r) => {
      const data = r.get('data');
      // Handle both Node objects and plain objects from map projection
      const properties = data.properties ?? data;
      return {
        key: r.get('key'),
        id: r.get('id'),
        data: properties,
      };
    });

    if (nodes.length === 0) {
      console.log(`[${target.nodeType}] ✓ All nodes already have embeddings`);
      return 0;
    }

    console.log(`[${target.nodeType}] Processing ${nodes.length} nodes...`);

    if (dryRun) {
      console.log(`[${target.nodeType}] Dry run - would process ${nodes.length} nodes`);
      console.log(`[${target.nodeType}] Sample text: "${target.textBuilder(nodes[0].data)}"`);
      return nodes.length;
    }

    // Batch embed
    for (let i = 0; i < nodes.length; i += BATCH_SIZE) {
      const batch = nodes.slice(i, i + BATCH_SIZE);
      const texts = batch.map((n) => target.textBuilder(n.data));

      // Generate embeddings via OpenAI
      const response = await openai.embeddings.create({
        model: MODEL,
        input: texts,
        dimensions: DIMENSIONS,
      });

      // Update nodes in Neo4j
      const writeSession = driver.session();
      try {
        for (let j = 0; j < batch.length; j++) {
          const node = batch[j];
          const embedding = response.data[j].embedding;
          const source = texts[j].substring(0, 500); // Truncate for storage

          await writeSession.run(
            `
            MATCH (n)
            WHERE elementId(n) = $id
            SET n.embedding = $embedding,
                n.embedding_source = $source,
                n.embedding_updated_at = datetime()
          `,
            { id: node.id, embedding, source }
          );
        }
      } finally {
        await writeSession.close();
      }

      processed += batch.length;
      console.log(
        `[${target.nodeType}] Processed ${Math.min(i + BATCH_SIZE, nodes.length)}/${nodes.length}`
      );
    }

    console.log(`[${target.nodeType}] ✓ Completed ${processed} nodes`);
    return processed;
  } finally {
    await session.close();
  }
}

async function main() {
  // Parse args
  const args = process.argv.slice(2);
  const dryRun = args.includes('--dry-run');
  const nodeTypeArg = args.find((a) => a.startsWith('--node-type='))?.split('=')[1];

  // Check OpenAI API key
  if (!process.env.OPENAI_API_KEY) {
    console.error('❌ OPENAI_API_KEY environment variable is required');
    console.error('Usage: OPENAI_API_KEY=sk-... npx tsx scripts/generate-embeddings.ts');
    process.exit(1);
  }

  const openai = new OpenAI();

  console.log('═══════════════════════════════════════════════════════════════');
  console.log('  NovaNet Embedding Generator v7.8.0');
  console.log('═══════════════════════════════════════════════════════════════');
  console.log(`Model: ${MODEL} (${DIMENSIONS} dimensions)`);
  console.log(`Batch size: ${BATCH_SIZE}`);
  if (dryRun) console.log('Mode: DRY RUN (no changes will be made)');
  console.log('');

  // Filter targets if node type specified
  const targets = nodeTypeArg
    ? TARGETS.filter((t) => t.nodeType === nodeTypeArg)
    : TARGETS;

  if (targets.length === 0) {
    console.error(`❌ Unknown node type: ${nodeTypeArg}`);
    console.error(`Valid types: ${TARGETS.map((t) => t.nodeType).join(', ')}`);
    process.exit(1);
  }

  let totalProcessed = 0;

  for (const target of targets) {
    try {
      const count = await generateEmbeddings(openai, target, dryRun);
      totalProcessed += count;
    } catch (error) {
      console.error(`[${target.nodeType}] ❌ Error:`, error);
    }
  }

  console.log('');
  console.log('═══════════════════════════════════════════════════════════════');
  console.log(`  ✓ Total processed: ${totalProcessed} nodes`);
  console.log('═══════════════════════════════════════════════════════════════');

  await closeDriver();
}

main().catch((error) => {
  console.error('Fatal error:', error);
  process.exit(1);
});
