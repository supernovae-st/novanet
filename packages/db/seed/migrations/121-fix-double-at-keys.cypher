// ============================================================================
// Migration 121: Fix Double @ Keys
// ============================================================================
// Migration 119 created malformed keys for nodes that already had a lowercase
// locale in the key (e.g., @sw-ke). This resulted in keys like:
//   cultureref:familia-family@sw-ke@sw-KE
// Should be:
//   cultureref:familia-family@sw-KE
// ============================================================================

// --- CultureRef: Fix double @ ---
MATCH (n:CultureRef)
WHERE n.key IS NOT NULL AND n.key =~ '.*@.*@.*'
WITH n, split(n.key, '@') AS parts
// Take prefix + last part (the proper locale)
SET n.key = parts[0] + '@' + parts[size(parts)-1];

// --- Taboo: Fix double @ ---
MATCH (n:Taboo)
WHERE n.key IS NOT NULL AND n.key =~ '.*@.*@.*'
WITH n, split(n.key, '@') AS parts
SET n.key = parts[0] + '@' + parts[size(parts)-1];

// --- AudienceTrait: Fix double @ ---
MATCH (n:AudienceTrait)
WHERE n.key IS NOT NULL AND n.key =~ '.*@.*@.*'
WITH n, split(n.key, '@') AS parts
SET n.key = parts[0] + '@' + parts[size(parts)-1];

// --- Pattern: Fix double @ ---
MATCH (n:Pattern)
WHERE n.key IS NOT NULL AND n.key =~ '.*@.*@.*'
WITH n, split(n.key, '@') AS parts
SET n.key = parts[0] + '@' + parts[size(parts)-1];

// --- Verification ---
// MATCH (n) WHERE n.key IS NOT NULL AND n.key =~ '.*@.*@.*'
// RETURN labels(n)[0], count(n);
// Should return 0
