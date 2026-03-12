// ============================================================================
// Migration 117: Key Prefix - Tier 2 (Language nodes)
// ============================================================================
// LanguageFamily and LanguageBranch depend on Locale (via arcs).
// Pattern: {prefix}:{identifier}
//
// LanguageFamily: langfam:{iso} (e.g., langfam:ine for Indo-European)
// LanguageBranch: langbranch:{iso} (e.g., langbranch:roa for Romance)
// ============================================================================

// --- LanguageFamily: langfam:{code} ---
MATCH (n:LanguageFamily)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'langfam:'
SET n.key = 'langfam:' + toLower(n.key);

// --- LanguageBranch: langbranch:{code} ---
MATCH (n:LanguageBranch)
WHERE n.key IS NOT NULL AND NOT n.key STARTS WITH 'langbranch:'
SET n.key = 'langbranch:' + toLower(n.key);

// --- Verification ---
// MATCH (n:LanguageFamily) WHERE NOT n.key STARTS WITH 'langfam:' RETURN count(n);
// MATCH (n:LanguageBranch) WHERE NOT n.key STARTS WITH 'langbranch:' RETURN count(n);
