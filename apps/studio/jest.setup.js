require('@testing-library/jest-dom');

// Enable Immer MapSet plugin for tests using Zustand with immer + Map/Set
const { enableMapSet } = require('immer');
enableMapSet();
