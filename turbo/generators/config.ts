import type { PlopTypes } from '@turbo/gen';

// =============================================================================
// NOVANET TURBOREPO GENERATORS (v11.3)
// =============================================================================
// Usage:
//   turbo gen node - Create a new node type (YAML + TypeScript)
//   turbo gen view - Create a new view definition
//   turbo gen arc  - Create a new arc type (replaces 'relation')
// =============================================================================

export default function generator(plop: PlopTypes.NodePlopAPI): void {
  // ===========================================================================
  // HELPERS
  // ===========================================================================

  // Convert to kebab-case (e.g., "LocaleVoice" -> "locale-voice")
  plop.setHelper('kebabCase', (text: string) => {
    return text
      .replace(/([a-z])([A-Z])/g, '$1-$2')
      .replace(/[\s_]+/g, '-')
      .toLowerCase();
  });

  // Convert to snake_case (e.g., "LocaleVoice" -> "locale_voice")
  plop.setHelper('snakeCase', (text: string) => {
    return text
      .replace(/([a-z])([A-Z])/g, '$1_$2')
      .replace(/[\s-]+/g, '_')
      .toLowerCase();
  });

  // ===========================================================================
  // NODE GENERATOR (v11.3)
  // ===========================================================================
  plop.setGenerator('node', {
    description: 'Create a new NovaNet node type with YAML schema and TypeScript types',
    prompts: [
      {
        type: 'input',
        name: 'name',
        message: 'Node name (PascalCase, e.g., "Audience", "LocaleVoice"):',
        validate: (input: string) => {
          if (!input) return 'Node name is required';
          if (!/^[A-Z][a-zA-Z0-9]*$/.test(input)) {
            return 'Must be PascalCase (e.g., "Audience", "LocaleVoice")';
          }
          return true;
        },
      },
      {
        type: 'list',
        name: 'realm',
        message: 'Realm (where does this node live?):',
        choices: [
          // v11.3: 2 realms (SHARED + ORG)
          { name: 'shared - Universal locale knowledge (READ-ONLY)', value: 'shared' },
          { name: 'org - Organization-specific content (Page, Block, Entity)', value: 'org' },
        ],
      },
      {
        type: 'list',
        name: 'layer',
        message: 'Layer:',
        choices: [
          // SHARED (3 layers) - v11.3
          { name: 'locale (shared) - Locale definitions', value: 'locale' },
          { name: 'geography (shared) - Geographic regions', value: 'geography' },
          { name: 'knowledge (shared) - Terms, expressions, patterns', value: 'knowledge' },
          // ORG (8 layers) - v11.3
          { name: 'config (org) - Organization config', value: 'config' },
          { name: 'foundation (org) - Project, content roots', value: 'foundation' },
          { name: 'structure (org) - Page, Block structure', value: 'structure' },
          { name: 'semantic (org) - Entity, meaning', value: 'semantic' },
          { name: 'instruction (org) - Prompts, rules', value: 'instruction' },
          { name: 'seo (org) - SEO keywords', value: 'seo' },
          { name: 'geo (org) - GEO intelligence', value: 'geo' },
          { name: 'output (org) - Generated content', value: 'output' },
        ],
      },
      {
        type: 'confirm',
        name: 'hasNative',
        message: 'Has locale-native variant (e.g., Entity -> EntityNative)?',
        default: false,
      },
      {
        type: 'input',
        name: 'description',
        message: 'Short content description (what this node IS):',
      },
    ],
    actions: (answers) => {
      const actions: PlopTypes.ActionType[] = [];

      // 1. Create YAML schema file (v11.3 path: node-classes/{realm}/{layer}/)
      actions.push({
        type: 'add',
        path: 'packages/core/models/node-classes/{{realm}}/{{layer}}/{{kebabCase name}}.yaml',
        templateFile: 'templates/node.yaml.hbs',
      });

      // 2. If hasNative, create Native variant (v0.13.0 ADR-029: EntityNative pattern)
      if (answers?.hasNative) {
        actions.push({
          type: 'add',
          path: 'packages/core/models/node-classes/{{realm}}/{{layer}}/{{kebabCase name}}-native.yaml',
          templateFile: 'templates/node-native.yaml.hbs',
        });
      }

      // 3. Instructions
      actions.push(() => {
        return `
Node created! Next steps:
  1. Edit packages/core/models/node-classes/${answers?.realm}/${answers?.layer}/${plop.getHelper('kebabCase')(answers?.name || '')}.yaml
  2. Add properties specific to your node
  3. Run: cargo run -- schema generate
  4. Run: cargo run -- schema validate
`;
      });

      return actions;
    },
  });

  // ===========================================================================
  // VIEW GENERATOR
  // ===========================================================================
  plop.setGenerator('view', {
    description: 'Create a new NovaNet view definition for graph queries',
    prompts: [
      {
        type: 'input',
        name: 'name',
        message: 'View name (kebab-case, e.g., "page-generation-context"):',
        validate: (input: string) => {
          if (!input) return 'View name is required';
          if (!/^[a-z][a-z0-9-]*$/.test(input)) {
            return 'Must be kebab-case (e.g., "page-generation-context")';
          }
          return true;
        },
      },
      {
        type: 'list',
        name: 'rootType',
        message: 'Root node type:',
        choices: ['Page', 'Block', 'Entity', 'Locale', 'Project'],
      },
      {
        type: 'input',
        name: 'description',
        message: 'View description:',
      },
      {
        type: 'checkbox',
        name: 'includes',
        message: 'What to include in the view:',
        choices: [
          { name: 'Blocks', value: 'blocks', checked: true },
          { name: 'Entities', value: 'entities', checked: true },
          { name: 'Locale Knowledge', value: 'knowledge', checked: false },
          { name: 'Prompts & Rules', value: 'prompts', checked: false },
          { name: 'SEO Keywords', value: 'seo', checked: false },
          { name: 'GEO Queries', value: 'geo', checked: false },
        ],
      },
    ],
    actions: [
      {
        type: 'add',
        path: 'packages/core/models/views/{{name}}.yaml',
        templateFile: 'templates/view.yaml.hbs',
      },
      () => `
View created! Next steps:
  1. Edit packages/core/models/views/{{name}}.yaml
  2. Customize the Cypher query
  3. Test with: cargo run -- doc generate --view={{name}}
`,
    ],
  });

  // ===========================================================================
  // ARC GENERATOR (v11.3 - replaces 'relation' generator)
  // ===========================================================================
  plop.setGenerator('arc', {
    description: 'Add a new arc type to NovaNet schema (v11.3)',
    prompts: [
      {
        type: 'input',
        name: 'name',
        message: 'Arc name (UPPER_SNAKE_CASE, e.g., "HAS_AUDIENCE"):',
        validate: (input: string) => {
          if (!input) return 'Arc name is required';
          if (!/^[A-Z][A-Z0-9_]*$/.test(input)) {
            return 'Must be UPPER_SNAKE_CASE (e.g., "HAS_AUDIENCE")';
          }
          return true;
        },
      },
      {
        type: 'list',
        name: 'family',
        message: 'Arc family:',
        choices: [
          { name: 'ownership - Parent owns child', value: 'ownership' },
          { name: 'localization - Locale relationships', value: 'localization' },
          { name: 'semantic - Meaning connections', value: 'semantic' },
          { name: 'generation - LLM output links', value: 'generation' },
          { name: 'mining - SEO/GEO data links', value: 'mining' },
        ],
      },
      {
        type: 'input',
        name: 'source',
        message: 'Source node type (e.g., "Project"):',
      },
      {
        type: 'input',
        name: 'target',
        message: 'Target node type (e.g., "Audience"):',
      },
      {
        type: 'list',
        name: 'cardinality',
        message: 'Cardinality:',
        choices: [
          { name: '1:1 (one to one)', value: 'one_to_one' },
          { name: '1:N (one to many)', value: 'one_to_many' },
          { name: 'N:1 (many to one)', value: 'many_to_one' },
          { name: 'N:M (many to many)', value: 'many_to_many' },
        ],
      },
      {
        type: 'input',
        name: 'description',
        message: 'Arc content description (what this arc DOES):',
      },
    ],
    actions: [
      {
        type: 'add',
        path: 'packages/core/models/arc-classes/{{family}}/{{kebabCase name}}.yaml',
        template: `arc:
  name: {{name}}
  family: {{family}}
  scope: intra_realm
  source: {{source}}
  target: {{target}}
  cardinality: {{cardinality}}
  content: "{{description}}"
`,
      },
      () => `
Arc created! Next steps:
  1. Review packages/core/models/arc-classes/{{family}}/{{kebabCase name}}.yaml
  2. Verify scope (intra_realm or cross_realm)
  3. Run: cargo run -- schema generate
  4. Run: cargo run -- schema validate
`,
    ],
  });
}
