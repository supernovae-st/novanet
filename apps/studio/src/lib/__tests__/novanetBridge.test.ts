/**
 * novanetBridge Tests — subprocess wrapper for Rust binary
 *
 * Mocks node:child_process execFile to test bridge logic in isolation.
 * Note: We use execFile (NOT exec) to prevent shell injection — see novanetBridge.ts.
 *
 * Jest hoists jest.mock() above all other statements, so mock factories
 * cannot reference outer `const` variables. We use promisify → identity
 * pattern to get a controllable mock reference after imports.
 */

// Mock factories (hoisted by Jest — no outer variable references)
jest.mock('node:child_process', () => ({
  execFile: jest.fn(),
}));

jest.mock('node:util', () => ({
  // Return the function as-is so execFileAsync === our mocked execFile
  promisify: jest.fn((fn: unknown) => fn),
}));

jest.mock('node:fs', () => ({
  existsSync: jest.fn((p: string) => p.includes('release/novanet')),
}));

jest.mock('../logger', () => ({
  logger: { warn: jest.fn(), error: jest.fn(), info: jest.fn(), debug: jest.fn() },
}));

import { buildCypherViaRust, NovanetBridgeError, resolveBinaryPath, type FacetFilter } from '../novanetBridge';

// Get reference to the mock after imports (safe — mocks are already in place)
// eslint-disable-next-line @typescript-eslint/no-require-imports
const mockExecFile = require('node:child_process').execFile as jest.Mock;

// =============================================================================
// resolveBinaryPath
// =============================================================================

describe('resolveBinaryPath', () => {
  it('resolves to a path containing novanet', () => {
    const p = resolveBinaryPath();
    expect(p).toContain('novanet');
    expect(p).toContain('release');
  });
});

// =============================================================================
// buildCypherViaRust
// =============================================================================

describe('buildCypherViaRust', () => {
  const baseFacets: FacetFilter = {
    realms: [],
    layers: [],
    traits: [],
    arc_families: [],
    classes: [],  // v0.12.0 ADR-023: kinds→classes
  };

  beforeEach(() => {
    mockExecFile.mockReset();
  });

  it('returns Cypher string from stdout', async () => {
    mockExecFile.mockResolvedValue({
      stdout: 'MATCH (n) WHERE NOT n:Schema RETURN n\n',  // v0.12.0 ADR-023: Meta→Schema
      stderr: '',
    });

    const cypher = await buildCypherViaRust(baseFacets);
    expect(cypher).toBe('MATCH (n) WHERE NOT n:Schema RETURN n');
  });

  it('passes correct JSON to stdin', async () => {
    mockExecFile.mockResolvedValue({ stdout: 'MATCH (n) RETURN n', stderr: '' });

    const facets: FacetFilter = {
      realms: ['org'],
      layers: ['structure'],
      traits: ['defined'],  // v0.12.0: renamed from invariant
      arc_families: [],
      classes: [],  // v0.12.0 ADR-023: kinds→classes
    };

    await buildCypherViaRust(facets);

    expect(mockExecFile).toHaveBeenCalledWith(
      expect.stringContaining('novanet'),
      ['filter', 'build'],
      expect.objectContaining({
        input: JSON.stringify(facets),
        encoding: 'utf-8',
        timeout: 5_000,
        maxBuffer: 1024 * 1024,
      }),
    );
  });

  it('throws NovanetBridgeError on empty output', async () => {
    mockExecFile.mockResolvedValue({ stdout: '', stderr: '' });

    await expect(buildCypherViaRust(baseFacets)).rejects.toThrow(NovanetBridgeError);
    await expect(buildCypherViaRust(baseFacets)).rejects.toThrow('Empty Cypher output');
  });

  it('throws NovanetBridgeError on whitespace-only output', async () => {
    mockExecFile.mockResolvedValue({ stdout: '  \n  \n', stderr: '' });

    await expect(buildCypherViaRust(baseFacets)).rejects.toThrow(NovanetBridgeError);
  });

  it('throws NovanetBridgeError with exit code on process failure', async () => {
    mockExecFile.mockRejectedValue({
      message: 'Command failed',
      code: 1,
      stderr: 'error: invalid input',
    });

    try {
      await buildCypherViaRust(baseFacets);
      fail('Should have thrown');
    } catch (err) {
      expect(err).toBeInstanceOf(NovanetBridgeError);
      expect((err as NovanetBridgeError).exitCode).toBe(1);
      expect((err as NovanetBridgeError).stderr).toBe('error: invalid input');
    }
  });

  it('throws NovanetBridgeError on timeout (killed process)', async () => {
    mockExecFile.mockRejectedValue({
      killed: true,
      message: 'Process killed',
    });

    await expect(buildCypherViaRust(baseFacets)).rejects.toThrow('killed');
  });

  it('trims trailing whitespace from Cypher', async () => {
    mockExecFile.mockResolvedValue({
      stdout: '  MATCH (n) RETURN n  \n',
      stderr: 'novanet filter build (reading JSON from stdin)',
    });

    const cypher = await buildCypherViaRust(baseFacets);
    expect(cypher).toBe('MATCH (n) RETURN n');
  });
});
