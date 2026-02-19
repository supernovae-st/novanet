// commitlint configuration for supernovae-agi monorepo
// https://commitlint.js.org

export default {
  extends: ['@commitlint/config-conventional'],
  rules: {
    // Type must be one of the following
    'type-enum': [
      2,
      'always',
      [
        'feat',     // New feature
        'fix',      // Bug fix
        'docs',     // Documentation only
        'style',    // Formatting, missing semicolons, etc.
        'refactor', // Code change that neither fixes a bug nor adds a feature
        'perf',     // Performance improvement
        'test',     // Adding missing tests
        'chore',    // Maintenance tasks
        'ci',       // CI/CD changes
        'build',    // Build system or external dependencies
        'revert',   // Reverts a previous commit
      ],
    ],
    // Scope must be one of the following (optional)
    'scope-enum': [
      1, // Warning only
      'always',
      [
        // Root level
        'deps',
        'release',
        'ci',
        // NovaNet
        'novanet',
        'core',
        'studio',
        'db',
        'mcp',
        'cli',
        'tui',
        'schema',
        // Nika
        'nika',
        'runtime',
        'provider',
        'dag',
        'event',
        'resilience',
        // Shared
        'claude',
        'dx',
        'docs',
        'rules',
        'skills',
        'hooks',
      ],
    ],
    // Subject (description) rules
    'subject-case': [2, 'always', 'lower-case'],
    'subject-empty': [2, 'never'],
    'subject-full-stop': [2, 'never', '.'],
    'subject-max-length': [2, 'always', 72],
    // Header rules
    'header-max-length': [2, 'always', 100],
    // Body rules
    'body-leading-blank': [2, 'always'],
    'body-max-line-length': [1, 'always', 100], // Warning only
    // Footer rules
    'footer-leading-blank': [2, 'always'],
    'footer-max-line-length': [1, 'always', 100], // Warning only
  },
  // Ignore patterns (for automated commits)
  ignores: [
    (commit) => commit.includes('[skip ci]'),
    (commit) => commit.includes('Merge pull request'),
    (commit) => commit.includes('Merge branch'),
  ],
  // Help URL
  helpUrl: 'https://www.conventionalcommits.org/',
};
