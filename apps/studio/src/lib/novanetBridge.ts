/**
 * novanetBridge.ts — Subprocess bridge to the Rust `novanet` binary.
 *
 * Calls `novanet filter build` via execFile (no shell injection risk).
 * Input: JSON on stdin. Output: Cypher on stdout.
 *
 * @module novanetBridge
 */

import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import path from 'node:path';
import fs from 'node:fs';
import { logger } from './logger';

// Node.js supports `input` on promisified execFile but @types/node lacks the overload.
// Explicit typing to match runtime behavior (execFile, NOT exec — no shell injection).
const execFileAsync = promisify(execFile) as (
  file: string,
  args: string[],
  options: { input?: string; encoding?: string; timeout?: number; maxBuffer?: number },
) => Promise<{ stdout: string; stderr: string }>;

// =============================================================================
// TYPES
// =============================================================================

export interface FacetFilter {
  realms: string[];
  layers: string[];
  traits: string[];
  arc_families: string[];
  classes: string[];  // v0.12.0 ADR-023: kinds→classes
}

export class NovanetBridgeError extends Error {
  constructor(
    message: string,
    public readonly exitCode?: number,
    public readonly stderr?: string,
  ) {
    super(message);
    this.name = 'NovanetBridgeError';
  }
}

// =============================================================================
// BINARY RESOLUTION (cached at module load)
// =============================================================================

let cachedBinaryPath: string | null = null;

/**
 * Resolve the novanet binary path from the monorepo root.
 * Prefers release build, falls back to debug.
 */
export function resolveBinaryPath(): string {
  if (cachedBinaryPath) return cachedBinaryPath;

  // From: apps/studio/src/lib/novanetBridge.ts
  // To:   tools/novanet/target/{release,debug}/novanet
  const monorepoRoot = path.resolve(__dirname, '../../../../');

  const releaseBinary = path.join(monorepoRoot, 'tools/novanet/target/release/novanet');
  const debugBinary = path.join(monorepoRoot, 'tools/novanet/target/debug/novanet');

  if (fs.existsSync(releaseBinary)) {
    cachedBinaryPath = releaseBinary;
  } else if (fs.existsSync(debugBinary)) {
    cachedBinaryPath = debugBinary;
    logger.warn('novanetBridge', 'Using debug binary (slower than release)');
  } else {
    throw new NovanetBridgeError(
      'novanet binary not found. Build it first:\n' +
      '  cargo build --release --manifest-path tools/novanet/Cargo.toml',
    );
  }

  return cachedBinaryPath;
}

// =============================================================================
// BRIDGE
// =============================================================================

const TIMEOUT_MS = 5_000;
const MAX_BUFFER = 1024 * 1024; // 1 MB

/**
 * Call `novanet filter build` to generate a Cypher query from facet filters.
 *
 * @param facets - Facet filter specification (realms, layers, traits, etc.)
 * @returns Cypher query string ready for Neo4j execution
 * @throws NovanetBridgeError on binary failure, timeout, or empty output
 */
export async function buildCypherViaRust(facets: FacetFilter): Promise<string> {
  const binaryPath = resolveBinaryPath();
  const input = JSON.stringify(facets);

  try {
    const { stdout } = await execFileAsync(
      binaryPath,
      ['filter', 'build'],
      {
        input,
        encoding: 'utf-8',
        timeout: TIMEOUT_MS,
        maxBuffer: MAX_BUFFER,
      },
    );

    // Cypher is on stdout (may have trailing newline)
    const cypher = stdout.trim();

    if (!cypher) {
      throw new NovanetBridgeError('Empty Cypher output from novanet filter build');
    }

    return cypher;
  } catch (error: unknown) {
    // Already a bridge error (empty output case)
    if (error instanceof NovanetBridgeError) throw error;

    const err = error as { killed?: boolean; code?: number; stderr?: string; message?: string };

    if (err.killed) {
      throw new NovanetBridgeError(`novanet process killed (timeout ${TIMEOUT_MS}ms)`);
    }

    throw new NovanetBridgeError(
      `novanet filter build failed: ${err.message ?? 'unknown error'}`,
      err.code ?? undefined,
      err.stderr ?? undefined,
    );
  }
}
