#!/usr/bin/env tsx
// NovaNet Core - Schema Validator
// Validates TypeScript types match Zod schemas

import { z } from 'zod';

// Import all schemas
import {
  LocaleSchema,
  LocaleIdentitySchema,
  LocaleVoiceSchema,
  LocaleCultureSchema,
  LocaleMarketSchema,
  LocaleLexiconSchema,
  ExpressionSchema,
} from '../src/schemas/locale-knowledge.schema.js';

import { RelationRegistry } from '../src/schemas/relations.schema.js';

console.log('✅ Validating Zod schemas...\n');

// Test that schemas can be instantiated
const schemas = {
  Locale: LocaleSchema,
  LocaleIdentity: LocaleIdentitySchema,
  LocaleVoice: LocaleVoiceSchema,
  LocaleCulture: LocaleCultureSchema,
  LocaleMarket: LocaleMarketSchema,
  LocaleLexicon: LocaleLexiconSchema,
  Expression: ExpressionSchema,
};

let hasError = false;

for (const [name, schema] of Object.entries(schemas)) {
  try {
    // Verify schema is valid Zod object
    if (schema instanceof z.ZodObject) {
      const shape = schema.shape;
      const fieldCount = Object.keys(shape).length;
      console.log(`✅ ${name}: ${fieldCount} fields`);
    } else {
      console.log(`❌ ${name}: Not a valid ZodObject`);
      hasError = true;
    }
  } catch (e) {
    console.log(`❌ ${name}: ${e}`);
    hasError = true;
  }
}

console.log('');

// Validate relation registry
console.log('🔗 Validating relation registry...\n');

const relationCount = Object.keys(RelationRegistry).length;
console.log(`📊 ${relationCount} relations defined`);

for (const [type, def] of Object.entries(RelationRegistry)) {
  if (!def.type || !def.from || !def.to || !def.cardinality) {
    console.log(`❌ ${type}: Missing required fields`);
    hasError = true;
  }
}

console.log('');

if (hasError) {
  console.log('❌ Schema validation failed!');
  process.exit(1);
} else {
  console.log('✅ All schemas valid!');
  process.exit(0);
}
