// Neo4j Client for NovaNet Core

import neo4j, { Driver, Session, type QueryResult, type ManagedTransaction } from 'neo4j-driver';

export interface Neo4jConfig {
  uri: string;
  user: string;
  password: string;
}

function getDefaultConfig(): Neo4jConfig {
  const uri = process.env.NEO4J_URI || 'bolt://localhost:7687';
  const user = process.env.NEO4J_USER || 'neo4j';
  const password = process.env.NEO4J_PASSWORD;

  // In production, require explicit password (no fallback)
  if (!password && process.env.NODE_ENV === 'production') {
    throw new Error('NEO4J_PASSWORD environment variable is required in production');
  }

  return {
    uri,
    user,
    password: password || 'novanetpassword', // Fallback only for development
  };
}

const DEFAULT_CONFIG: Neo4jConfig = getDefaultConfig();

let driver: Driver | null = null;

export function getDriver(config: Neo4jConfig = DEFAULT_CONFIG): Driver {
  if (!driver) {
    driver = neo4j.driver(
      config.uri,
      neo4j.auth.basic(config.user, config.password)
    );
  }
  return driver;
}

export function getSession(): Session {
  return getDriver().session();
}

export async function runQuery<T = Record<string, unknown>>(
  query: string,
  params: Record<string, unknown> = {}
): Promise<T[]> {
  const session = getSession();
  try {
    const result: QueryResult = await session.run(query, params);
    return result.records.map((record) => record.toObject() as T);
  } finally {
    await session.close();
  }
}

export async function runWrite(
  query: string,
  params: Record<string, unknown> = {}
): Promise<void> {
  const session = getSession();
  try {
    await session.run(query, params);
  } finally {
    await session.close();
  }
}

export async function closeDriver(): Promise<void> {
  if (driver) {
    await driver.close();
    driver = null;
  }
}

// Session helper - provides session lifecycle management
export async function withSession<T>(
  fn: (session: Session) => Promise<T>
): Promise<T> {
  const session = getSession();
  try {
    return await fn(session);
  } finally {
    await session.close();
  }
}

// Transaction helper - provides actual transactional guarantees
export async function withWriteTransaction<T>(
  fn: (tx: ManagedTransaction) => Promise<T>
): Promise<T> {
  const session = getSession();
  try {
    return await session.executeWrite((tx) => fn(tx));
  } finally {
    await session.close();
  }
}

// Check if Neo4j is running and accessible
export async function isNeo4jAvailable(): Promise<boolean> {
  try {
    const d = getDriver();
    const session = d.session();
    await session.run('RETURN 1 AS ping');
    await session.close();
    return true;
  } catch {
    return false;
  }
}
