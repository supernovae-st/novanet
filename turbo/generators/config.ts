import type { PlopTypes } from '@turbo/gen';

// =============================================================================
// NOVANET TURBOREPO GENERATORS
// =============================================================================
// Usage:
//   turbo gen node     - Create a new node type (YAML + TypeScript)
//   turbo gen view     - Create a new view definition
//   turbo gen relation - Create a new relation type
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
  // NODE GENERATOR
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
        name: 'scope',
        message: 'Scope (where does this node live?):',
        choices: [
          { name: 'global  - Shared by ALL projects (Locale, LocaleKnowledge)', value: 'global' },
          { name: 'shared  - Independent of projects (SEO, GEO)', value: 'shared' },
          { name: 'project - Per-project instances (Page, Block, Concept)', value: 'project' },
        ],
      },
      {
        type: 'list',
        name: 'subcategory',
        message: 'Subcategory:',
        choices: [
          // Global
          { name: 'config (global)', value: 'config' },
          { name: 'knowledge (global)', value: 'knowledge' },
          // Shared
          { name: 'seo (shared)', value: 'seo' },
          { name: 'geo (shared)', value: 'geo' },
          // Project
          { name: 'foundation (project)', value: 'foundation' },
          { name: 'structure (project)', value: 'structure' },
          { name: 'semantic (project)', value: 'semantic' },
          { name: 'instruction (project)', value: 'instruction' },
          { name: 'output (project)', value: 'output' },
        ],
      },
      {
        type: 'confirm',
        name: 'hasL10n',
        message: 'Has localized variant (e.g., Concept -> ConceptL10n)?',
        default: false,
      },
      {
        type: 'input',
        name: 'description',
        message: 'Short description (for llm_context):',
      },
    ],
    actions: (answers) => {
      const actions: PlopTypes.ActionType[] = [];

      // 1. Create YAML schema file
      actions.push({
        type: 'add',
        path: 'packages/core/models/nodes/{{scope}}/{{kebabCase name}}.yaml',
        templateFile: 'templates/node.yaml.hbs',
      });

      // 2. If hasL10n, create L10n variant
      if (answers?.hasL10n) {
        actions.push({
          type: 'add',
          path: 'packages/core/models/nodes/{{scope}}/{{kebabCase name}}-l10n.yaml',
          templateFile: 'templates/node-l10n.yaml.hbs',
        });
      }

      // 3. Instructions
      actions.push(() => {
        return `
Node created! Next steps:
  1. Edit packages/core/models/nodes/${answers?.scope}/${plop.getHelper('kebabCase')(answers?.name || '')}.yaml
  2. Add properties specific to your node
  3. Run: pnpm --filter=@novanet/core build
  4. Update types in packages/core/src/types/nodes.ts if needed
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
        choices: ['Page', 'Block', 'Concept', 'Locale', 'Project'],
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
          { name: 'Concepts', value: 'concepts', checked: true },
          { name: 'Locale Knowledge', value: 'knowledge', checked: false },
          { name: 'Prompts & Rules', value: 'prompts', checked: false },
          { name: 'SEO Keywords', value: 'seo', checked: false },
          { name: 'GEO Seeds', value: 'geo', checked: false },
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
  3. Test with: pnpm novanet export-view {{name}}
`,
    ],
  });

  // ===========================================================================
  // RELATION GENERATOR
  // ===========================================================================
  plop.setGenerator('relation', {
    description: 'Add a new relation type to NovaNet schema',
    prompts: [
      {
        type: 'input',
        name: 'name',
        message: 'Relation name (UPPER_SNAKE_CASE, e.g., "HAS_AUDIENCE"):',
        validate: (input: string) => {
          if (!input) return 'Relation name is required';
          if (!/^[A-Z][A-Z0-9_]*$/.test(input)) {
            return 'Must be UPPER_SNAKE_CASE (e.g., "HAS_AUDIENCE")';
          }
          return true;
        },
      },
      {
        type: 'input',
        name: 'from',
        message: 'From node type (e.g., "Project"):',
      },
      {
        type: 'input',
        name: 'to',
        message: 'To node type (e.g., "Audience"):',
      },
      {
        type: 'input',
        name: 'description',
        message: 'Relation description:',
      },
      {
        type: 'confirm',
        name: 'hasProperties',
        message: 'Does this relation have properties?',
        default: false,
      },
    ],
    actions: [
      {
        type: 'append',
        path: 'packages/core/models/relations.yaml',
        template: `
# {{name}}
- name: {{name}}
  from: {{from}}
  to: {{to}}
  description: "{{description}}"
  cardinality: "1:N"
{{#if hasProperties}}
  properties:
    - name: created_at
      type: datetime
      description: "When the relation was created"
{{/if}}
`,
      },
      () => `
Relation added to relations.yaml! Next steps:
  1. Review packages/core/models/relations.yaml
  2. Add any additional properties
  3. Update TypeScript types if needed
`,
    ],
  });
}
