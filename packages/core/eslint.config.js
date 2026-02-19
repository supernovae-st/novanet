// NovaNet Core - ESLint Configuration (Flat Config)
// v0.14.1: Strict TypeScript rules for production reliability
import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  ...tseslint.configs.strict,
  {
    files: ['src/**/*.ts', 'scripts/**/*.ts'],
    languageOptions: {
      parserOptions: {
        project: './tsconfig.json',
      },
    },
    rules: {
      // TypeScript strict - v0.14.1: Upgraded for production reliability
      '@typescript-eslint/no-unused-vars': ['error', {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
        caughtErrorsIgnorePattern: '^_',
        ignoreRestSiblings: true,
      }],
      '@typescript-eslint/no-explicit-any': 'error', // v0.14.1: error (was warn)
      '@typescript-eslint/explicit-function-return-type': 'off',
      '@typescript-eslint/no-non-null-assertion': 'warn',

      // v0.14.1: New strict rules for async/promise safety
      '@typescript-eslint/no-floating-promises': 'error',
      '@typescript-eslint/no-misused-promises': ['error', {
        checksConditionals: true,
        checksVoidReturn: true,
      }],
      '@typescript-eslint/strict-boolean-expressions': ['error', {
        allowString: false,
        allowNumber: false,
        allowNullableObject: false,
        allowNullableBoolean: true,
        allowNullableString: false,
        allowNullableNumber: false,
        allowAny: false,
      }],

      // General
      'no-console': ['warn', { allow: ['warn', 'error'] }], // v0.14.1: warn (was off)
      'prefer-const': 'error',
      'no-var': 'error',
      'no-unreachable': 'error',
    },
  },
  {
    // Relax rules for test files
    files: ['**/*.test.ts', '**/*.spec.ts', '__tests__/**/*.ts'],
    rules: {
      '@typescript-eslint/no-explicit-any': 'warn', // Allow in tests
      '@typescript-eslint/strict-boolean-expressions': 'off', // Allow in tests
      'no-console': 'off',
    },
  },
  {
    ignores: ['dist/', 'node_modules/', '*.js', '!eslint.config.js'],
  }
);
