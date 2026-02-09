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
        name: 'realm',
        message: 'Realm (where does this node live?):',
        choices: [
          // v11.0: 2 realms (GLOBAL + TENANT)
          { name: 'global - Universal locale knowledge (READ-ONLY)', value: 'global' },
          { name: 'tenant - Business-specific content (Page, Block, Entity)', value: 'tenant' },
        ],
      },
      {
        type: 'list',
        name: 'layer',
        message: 'Layer:',
        choices: [
          // GLOBAL (2 layers)
          { name: 'config (global)', value: 'config' },
          { name: 'locale-knowledge (global)', value: 'locale-knowledge' },
          // TENANT (7 layers)
          { name: 'config (tenant)', value: 'config' },
          { name: 'foundation (tenant)', value: 'foundation' },
          { name: 'structure (tenant)', value: 'structure' },
          { name: 'semantic (tenant)', value: 'semantic' },
          { name: 'instruction (tenant)', value: 'instruction' },
          { name: 'seo (tenant - v11.0)', value: 'seo' },
          { name: 'output (tenant)', value: 'output' },
        ],
      },
      {
        type: 'confirm',
        name: 'hasContent',
        message: 'Has localized content variant (e.g., Entity -> EntityContent)?',
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
        path: 'packages/core/models/nodes/{{realm}}/{{layer}}/{{kebabCase name}}.yaml',
        templateFile: 'templates/node.yaml.hbs',
      });

      // 2. If hasContent, create Content variant (v11.0: EntityContent pattern)
      if (answers?.hasContent) {
        actions.push({
          type: 'add',
          path: 'packages/core/models/nodes/{{realm}}/{{layer}}/{{kebabCase name}}-content.yaml',
          templateFile: 'templates/node-content.yaml.hbs',
        });
      }

      // 3. Instructions
      actions.push(() => {
        return `
Node created! Next steps:
  1. Edit packages/core/models/nodes/${answers?.realm}/${answers?.layer}/${plop.getHelper('kebabCase')(answers?.name || '')}.yaml
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
