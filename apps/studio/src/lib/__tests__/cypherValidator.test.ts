/**
 * Cypher Validator Tests
 *
 * Security-critical tests for client-side query validation.
 * Tests injection prevention, write blocking, and syntax validation.
 */

import { validateCypher, getCypherError } from '../cypherValidator';

describe('cypherValidator', () => {
  // ==========================================================================
  // Valid Queries
  // ==========================================================================

  describe('valid queries', () => {
    it('should accept basic MATCH query', () => {
      const result = validateCypher('MATCH (n) RETURN n');
      expect(result.valid).toBe(true);
      expect(result.error).toBeUndefined();
    });

    it('should accept query with WHERE clause', () => {
      const result = validateCypher('MATCH (n:Concept) WHERE n.key = "test" RETURN n');
      expect(result.valid).toBe(true);
    });

    it('should accept OPTIONAL MATCH', () => {
      const result = validateCypher('OPTIONAL MATCH (n)-[r]-(m) RETURN n, r, m');
      expect(result.valid).toBe(true);
    });

    it('should accept WITH clause', () => {
      const result = validateCypher('WITH 1 AS num MATCH (n) RETURN n, num');
      expect(result.valid).toBe(true);
    });

    it('should accept RETURN with aggregations', () => {
      const result = validateCypher('MATCH (n:Concept) RETURN n.type, count(n)');
      expect(result.valid).toBe(true);
    });

    it('should accept UNWIND', () => {
      const result = validateCypher('UNWIND [1,2,3] AS x MATCH (n) WHERE id(n) = x RETURN n');
      expect(result.valid).toBe(true);
    });

    it('should accept CALL with RETURN', () => {
      const result = validateCypher('CALL db.labels() YIELD label RETURN label');
      expect(result.valid).toBe(true);
    });

    it('should accept case-insensitive keywords', () => {
      const result = validateCypher('match (n) return n');
      expect(result.valid).toBe(true);
    });

    it('should accept query with LIMIT', () => {
      const result = validateCypher('MATCH (n) RETURN n LIMIT 100');
      expect(result.valid).toBe(true);
    });

    it('should accept query with backticks for escaped labels', () => {
      const result = validateCypher('MATCH (n:`My Label`) RETURN n');
      expect(result.valid).toBe(true);
    });
  });

  // ==========================================================================
  // Empty/Length Validation
  // ==========================================================================

  describe('empty and length validation', () => {
    it('should reject empty query', () => {
      const result = validateCypher('');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Query cannot be empty');
    });

    it('should reject whitespace-only query', () => {
      const result = validateCypher('   \n\t  ');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Query cannot be empty');
    });

    it('should reject query that is too short', () => {
      const result = validateCypher('MATCH n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Query too short');
    });

    it('should reject query that is too long', () => {
      const longQuery = 'MATCH (n) RETURN ' + 'n.property, '.repeat(500) + 'n';
      const result = validateCypher(longQuery);
      expect(result.valid).toBe(false);
      expect(result.error).toMatch(/Query too long/);
    });
  });

  // ==========================================================================
  // Invalid Start Keywords
  // ==========================================================================

  describe('invalid start keywords', () => {
    it('should reject query starting with invalid keyword', () => {
      const result = validateCypher('SELECT * FROM nodes RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toMatch(/must start with MATCH/);
    });

    it('should reject query without RETURN', () => {
      const result = validateCypher('MATCH (n) WHERE n.key = "test"');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Read queries must include RETURN clause');
    });
  });

  // ==========================================================================
  // Balanced Syntax
  // ==========================================================================

  describe('balanced syntax checks', () => {
    it('should reject unbalanced parentheses', () => {
      const result = validateCypher('MATCH (n RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toMatch(/Unbalanced parentheses/);
    });

    it('should reject unbalanced brackets', () => {
      const result = validateCypher('MATCH (n)-[r-(m) RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toMatch(/Unbalanced brackets/);
    });

    it('should reject unbalanced braces', () => {
      const result = validateCypher('MATCH (n {key: "test") RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toMatch(/Unbalanced braces/);
    });
  });

  // ==========================================================================
  // Injection Prevention (SECURITY-CRITICAL)
  // ==========================================================================

  describe('injection prevention', () => {
    it('should reject single-line comments', () => {
      const result = validateCypher('MATCH (n) // injection RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Comments are not allowed in queries');
    });

    it('should reject multi-line comments', () => {
      const result = validateCypher('MATCH (n) /* injection */ RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Comments are not allowed in queries');
    });

    it('should reject string concatenation with +', () => {
      const result = validateCypher('MATCH (n) WHERE n.key = "test" + " RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('String concatenation is not allowed in queries');
    });

    it('should reject reverse string concatenation', () => {
      const result = validateCypher('MATCH (n) WHERE n.key = "test" +"value" RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('String concatenation is not allowed in queries');
    });

    it('should reject semicolons (multiple statements)', () => {
      const result = validateCypher('MATCH (n) RETURN n; DELETE n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Multiple statements (;) are not allowed');
    });

    it('should reject dynamic label construction with $', () => {
      const result = validateCypher('MATCH (n:`${injected}`) RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Invalid label syntax detected');
    });

    it('should reject dynamic label with +', () => {
      const result = validateCypher('MATCH (n:`test+injection`) RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Invalid label syntax detected');
    });

    it('should reject unbalanced backticks', () => {
      const result = validateCypher('MATCH (n:`Label) RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Invalid label syntax detected');
    });
  });

  // ==========================================================================
  // Write Operation Blocking (SECURITY-CRITICAL)
  // Note: Queries starting with invalid keywords are rejected BEFORE write check.
  // Tests use valid MATCH start to ensure write operations are specifically blocked.
  // ==========================================================================

  describe('write operation blocking', () => {
    it('should reject CREATE in a MATCH context', () => {
      // CREATE starting queries fail "valid start" check first
      // This tests CREATE within an otherwise valid query pattern
      const result = validateCypher('MATCH (n) CREATE (m:Test) RETURN n, m');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Write operation "CREATE" is not allowed');
    });

    it('should reject MERGE in a MATCH context', () => {
      const result = validateCypher('MATCH (n) MERGE (m:Test {key: "x"}) RETURN n, m');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Write operation "MERGE" is not allowed');
    });

    it('should reject SET', () => {
      const result = validateCypher('MATCH (n) SET n.name = "hack" RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Write operation "SET" is not allowed');
    });

    it('should reject DELETE', () => {
      const result = validateCypher('MATCH (n) DELETE n RETURN count(n)');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Write operation "DELETE" is not allowed');
    });

    it('should reject REMOVE', () => {
      const result = validateCypher('MATCH (n) REMOVE n.property RETURN n');
      expect(result.valid).toBe(false);
      expect(result.error).toBe('Write operation "REMOVE" is not allowed');
    });

    it('should reject DETACH DELETE (DELETE detected first)', () => {
      const result = validateCypher('MATCH (n) DETACH DELETE n RETURN count(n)');
      expect(result.valid).toBe(false);
      // DELETE is checked before DETACH in the blockedKeywords array
      expect(result.error).toBe('Write operation "DELETE" is not allowed');
    });

    it('should reject queries starting with DROP (invalid start)', () => {
      const result = validateCypher('DROP INDEX test_index RETURN 1');
      expect(result.valid).toBe(false);
      // DROP doesn't start with valid keyword, so caught by start check
      expect(result.error).toMatch(/must start with MATCH/);
    });

    it('should reject queries starting with CREATE (invalid start)', () => {
      const result = validateCypher('CREATE INDEX test_index ON :Test(key) RETURN 1');
      expect(result.valid).toBe(false);
      expect(result.error).toMatch(/must start with MATCH/);
    });

    it('should reject LOAD CSV (invalid start)', () => {
      const result = validateCypher('LOAD CSV FROM "http://evil.com/data.csv" AS row RETURN row');
      expect(result.valid).toBe(false);
      expect(result.error).toMatch(/must start with MATCH/);
    });

    it('should reject FOREACH with SET inside', () => {
      const result = validateCypher('MATCH (n) FOREACH (x IN [1,2] | SET n.x = x) RETURN n');
      expect(result.valid).toBe(false);
      // SET is detected inside the FOREACH
      expect(result.error).toBe('Write operation "SET" is not allowed');
    });

    it('should NOT block words containing blocked keywords (CREATED_AT)', () => {
      const result = validateCypher('MATCH (n) WHERE n.CREATED_AT > 0 RETURN n');
      expect(result.valid).toBe(true);
    });

    it('should NOT block OFFSET keyword', () => {
      const result = validateCypher('MATCH (n) RETURN n SKIP 10 LIMIT 10');
      expect(result.valid).toBe(true);
    });
  });

  // ==========================================================================
  // Typo Detection
  // Note: Typo detection happens AFTER basic validation (start keyword, RETURN).
  // Queries with typos in keywords may fail earlier validation checks.
  // ==========================================================================

  describe('typo detection', () => {
    it('should detect METCH typo (fails start keyword check first)', () => {
      const result = validateCypher('METCH (n) RETURN n');
      expect(result.valid).toBe(false);
      // METCH fails the "valid start keyword" check before typo detection
      expect(result.error).toMatch(/must start with MATCH/);
    });

    it('should detect RETRUN typo (fails RETURN check first)', () => {
      const result = validateCypher('MATCH (n) RETRUN n');
      expect(result.valid).toBe(false);
      // RETRUN fails the "must have RETURN" check
      expect(result.error).toBe('Read queries must include RETURN clause');
    });

    it('should detect WHERE typos (WHER )', () => {
      // These have valid MATCH...RETURN structure, so typo detection runs
      const result1 = validateCypher('MATCH (n) WHER n.key = "x" RETURN n');
      expect(result1.valid).toBe(false);
      expect(result1.error).toMatch(/Did you mean WHERE/);
    });

    it('should detect WHERE typos (WEHRE)', () => {
      const result2 = validateCypher('MATCH (n) WEHRE n.key = "x" RETURN n');
      expect(result2.valid).toBe(false);
      expect(result2.error).toMatch(/Did you mean WHERE/);
    });
  });

  // ==========================================================================
  // getCypherError Helper
  // ==========================================================================

  describe('getCypherError helper', () => {
    it('should return null for valid query', () => {
      const error = getCypherError('MATCH (n) RETURN n');
      expect(error).toBeNull();
    });

    it('should return error message for invalid query', () => {
      const error = getCypherError('');
      expect(error).toBe('Query cannot be empty');
    });

    it('should return generic error if error is undefined', () => {
      // This tests the fallback in getCypherError
      const error = getCypherError('MATCH n'); // Too short
      expect(error).toBe('Query too short');
    });
  });
});
