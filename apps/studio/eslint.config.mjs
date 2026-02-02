import coreWebVitals from 'eslint-config-next/core-web-vitals';
import nextTypescript from 'eslint-config-next/typescript';

const config = [
  ...coreWebVitals,
  ...nextTypescript,
  {
    ignores: [
      'coverage/**',
      '.next/**',
      'node_modules/**',
      '*.config.js',
      '*.config.cjs',
      '*.setup.js',
    ],
  },
  {
    rules: {
      // Allow _prefixed unused vars (destructuring, callback signatures)
      '@typescript-eslint/no-unused-vars': ['warn', {
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
      }],
      // React 19 compiler rules — downgrade to warn for v9.0.0
      // TODO(v9.1): Fix and promote back to error
      'react-hooks/set-state-in-effect': 'warn',
      'react-hooks/purity': 'warn',
      'react-hooks/immutability': 'warn',
      'react-hooks/refs': 'warn',
      'react-hooks/preserve-manual-memoization': 'warn',
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
