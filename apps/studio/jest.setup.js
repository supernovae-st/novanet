require('@testing-library/jest-dom');

// Enable Immer MapSet plugin for tests using Zustand with immer + Map/Set
const { enableMapSet } = require('immer');
enableMapSet();

// Set environment variables for tests (required by neo4j.ts)
process.env.NEO4J_URI = 'bolt://localhost:7687';
process.env.NEO4J_USER = 'neo4j';
process.env.NEO4J_PASSWORD = 'test-password';

// Mock d3-force globally to avoid ESM import issues
// d3-force uses ESM exports which Jest doesn't handle well without complex config
jest.mock('d3-force', () => {
  const mockSimulation = {
    nodes: jest.fn().mockReturnThis(),
    force: jest.fn().mockReturnThis(),
    alphaDecay: jest.fn().mockReturnThis(),
    velocityDecay: jest.fn().mockReturnThis(),
    alphaMin: jest.fn().mockReturnThis(),
    alpha: jest.fn().mockReturnValue(0.001),
    tick: jest.fn(),
    stop: jest.fn(),
  };

  return {
    forceSimulation: jest.fn(() => mockSimulation),
    forceLink: jest.fn(() => ({
      id: jest.fn().mockReturnThis(),
      distance: jest.fn().mockReturnThis(),
      strength: jest.fn().mockReturnThis(),
    })),
    forceManyBody: jest.fn(() => ({
      strength: jest.fn().mockReturnThis(),
      distanceMin: jest.fn().mockReturnThis(),
      distanceMax: jest.fn().mockReturnThis(),
      theta: jest.fn().mockReturnThis(),
    })),
    forceCenter: jest.fn(() => ({
      strength: jest.fn().mockReturnThis(),
    })),
    forceCollide: jest.fn(() => ({
      radius: jest.fn().mockReturnThis(),
      strength: jest.fn().mockReturnThis(),
      iterations: jest.fn().mockReturnThis(),
    })),
    forceX: jest.fn(() => ({
      x: jest.fn().mockReturnThis(),
      strength: jest.fn().mockReturnThis(),
    })),
    forceY: jest.fn(() => ({
      y: jest.fn().mockReturnThis(),
      strength: jest.fn().mockReturnThis(),
    })),
  };
});
