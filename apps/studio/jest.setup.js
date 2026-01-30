require('@testing-library/jest-dom');

// Enable Immer MapSet plugin for tests using Zustand with immer + Map/Set
const { enableMapSet } = require('immer');
enableMapSet();

// Set environment variables for tests (required by neo4j.ts)
process.env.NEO4J_URI = 'bolt://localhost:7687';
process.env.NEO4J_USER = 'neo4j';
process.env.NEO4J_PASSWORD = 'test-password';
