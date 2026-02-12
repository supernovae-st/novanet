/**
 * Color Synchronization Tests
 *
 * Verifies that colors in nodeTypes.ts match the single source of truth
 * in generated.ts (auto-generated from taxonomy.yaml).
 *
 * If these tests fail, run:
 *   cargo run -- schema generate
 * to regenerate generated.ts from taxonomy.yaml
 */

import { LAYER_COLORS, type LayerKey } from '@/design/colors/generated';
import { NODE_VISUAL_LAYERS } from '@/config/nodeTypes';

describe('Color Synchronization', () => {
  describe('Layer Colors', () => {
    it('should have NODE_VISUAL_LAYERS colors matching LAYER_COLORS from taxonomy.yaml', () => {
      const mismatches: string[] = [];

      for (const layer of NODE_VISUAL_LAYERS) {
        const layerKey = layer.id as LayerKey;
        const expectedColor = LAYER_COLORS[layerKey]?.color;

        if (!expectedColor) {
          mismatches.push(`${layerKey}: layer not found in LAYER_COLORS`);
          continue;
        }

        if (layer.color.toLowerCase() !== expectedColor.toLowerCase()) {
          mismatches.push(
            `${layerKey}: expected ${expectedColor}, got ${layer.color}`
          );
        }
      }

      if (mismatches.length > 0) {
        throw new Error(
          `Color mismatches found (run 'cargo run -- schema generate' to fix):\n${mismatches.join('\n')}`
        );
      }
    });

    it('should have all 9 layers from taxonomy.yaml', () => {
      const expectedLayers: LayerKey[] = [
        'config',
        'locale',
        'geography',
        'knowledge',
        'foundation',
        'structure',
        'semantic',
        'instruction',
        'output',
      ];

      const actualLayers = NODE_VISUAL_LAYERS.map((l) => l.id);

      for (const expected of expectedLayers) {
        expect(actualLayers).toContain(expected);
      }
    });

    it('should have valid hex color format', () => {
      const hexPattern = /^#[0-9a-f]{6}$/i;

      for (const layer of NODE_VISUAL_LAYERS) {
        expect(layer.color).toMatch(hexPattern);
        expect(layer.colorLight).toMatch(hexPattern);
      }
    });

    it('should have colorLight be different from color (lighter variant)', () => {
      for (const layer of NODE_VISUAL_LAYERS) {
        expect(layer.colorLight).not.toBe(layer.color);
      }
    });
  });

  describe('Generated Colors Integrity', () => {
    it('LAYER_COLORS should have all 9 layers', () => {
      const keys = Object.keys(LAYER_COLORS);
      expect(keys).toHaveLength(9);
      expect(keys).toContain('config');
      expect(keys).toContain('locale');
      expect(keys).toContain('geography');
      expect(keys).toContain('knowledge');
      expect(keys).toContain('foundation');
      expect(keys).toContain('structure');
      expect(keys).toContain('semantic');
      expect(keys).toContain('instruction');
      expect(keys).toContain('output');
    });

    it('LAYER_COLORS should have valid ColorTokens structure', () => {
      for (const [key, tokens] of Object.entries(LAYER_COLORS)) {
        expect(tokens).toHaveProperty('color');
        expect(tokens).toHaveProperty('bg');
        expect(tokens).toHaveProperty('text');
        expect(tokens).toHaveProperty('border');
        expect(tokens).toHaveProperty('bgSolid');

        // Verify color is valid hex
        expect(tokens.color).toMatch(/^#[0-9a-f]{6}$/i);

        // Verify Tailwind classes use the color
        expect(tokens.bg).toContain(tokens.color.toLowerCase());
        expect(tokens.text).toContain(tokens.color.toLowerCase());
        expect(tokens.border).toContain(tokens.color.toLowerCase());
        expect(tokens.bgSolid).toContain(tokens.color.toLowerCase());
      }
    });
  });
});
