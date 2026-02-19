// NovaNet Studio - ESLint Configuration (Flat Config)
// v0.14.1: Strict TypeScript + React rules for production reliability
import coreWebVitals from 'eslint-config-next/core-web-vitals';
import nextTypescript from 'eslint-config-next/typescript';

const config = [
  ...coreWebVitals,
  ...nextTypescript,
  {
    ignores: [
      'coverage/**',
      '.next/**',
      '.turbo/**',
      'node_modules/**',
      '*.config.js',
      '*.config.cjs',
      '*.setup.js',
    ],
  },
  {
    rules: {
      // v0.14.1: Strict TypeScript - error level for production
      '@typescript-eslint/no-unused-vars': ['error', {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
        caughtErrorsIgnorePattern: '^_',
        ignoreRestSiblings: true,
      }],
      '@typescript-eslint/no-explicit-any': 'error', // v0.14.1: error (no any in production)
      '@typescript-eslint/no-non-null-assertion': 'warn',

      // TODO(typed-linting): Re-enable when parserOptions.project is configured
      // These rules require type information which isn't available with current setup
      // See: https://tseslint.com/typed-linting
      // '@typescript-eslint/no-floating-promises': 'error',
      // '@typescript-eslint/no-misused-promises': ['error', { checksConditionals: true, checksVoidReturn: { attributes: false } }],

      // React 19 compiler rules — downgrade to warn for v9.0.0
      // TODO(v9.1): Fix and promote back to error
      'react-hooks/set-state-in-effect': 'warn',
      'react-hooks/purity': 'warn',
      'react-hooks/immutability': 'warn',
      'react-hooks/refs': 'warn',
      'react-hooks/preserve-manual-memoization': 'warn',

      // General
      'no-console': ['warn', { allow: ['warn', 'error'] }], // v0.14.1: warn (was off)
    },
  },
  {
    // Relax rules for test files
    files: ['**/*.test.ts', '**/*.test.tsx', '**/*.spec.ts', '**/*.spec.tsx', '**/__tests__/**/*.ts', '**/__tests__/**/*.tsx'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'warn',
      'no-console': 'off',
    },
  },
  {
    files: ['**/*.config.ts', '**/tailwind.config.ts'],
    rules: {
      '@typescript-eslint/no-require-imports': 'off',
    },
  },
];

export default config;
