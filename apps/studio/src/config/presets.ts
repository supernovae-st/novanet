/**
 * Default preset ID (v9.0.0)
 *
 * Built-in presets are defined as VIEW_PRESETS in @/lib/filterAdapter
 * using the faceted NovaNetFilter API (byLayer, byTypes, etc.).
 * Custom presets use the FilterPreset interface from @/types.
 */
export const DEFAULT_PRESET = {
  id: 'project-structure',
  name: 'Project Structure',
} as const;
