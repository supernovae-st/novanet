// Migration 057: Create BlockNatives for 4 orphan blocks
// Generates en-US and fr-FR natives for blocks missing HAS_NATIVE

// Create en-US BlockNatives
MATCH (b:Block)
WHERE b.key IN ["block:qr-code-hero", "block:qr-code-what-is", "block:qr-code-use-cases", "block:qr-code-cta"]
  AND NOT (b)-[:HAS_NATIVE]->(:BlockNative {key: b.key + "@en-US"})
MATCH (l:Locale {key: "en-US"})
CREATE (bn:BlockNative {
  key: b.key + "@en-US",
  display_name: b.display_name + " (en-US)",
  description: b.description,
  content: "<!-- Generated placeholder - needs content generation -->",
  block_type: CASE
    WHEN b.key CONTAINS "hero" THEN "hero"
    WHEN b.key CONTAINS "what-is" THEN "explainer"
    WHEN b.key CONTAINS "use-cases" THEN "features"
    WHEN b.key CONTAINS "cta" THEN "cta"
    ELSE "content"
  END,
  created_at: datetime(),
  updated_at: datetime()
})
MERGE (b)-[:HAS_NATIVE]->(bn)
MERGE (bn)-[:FOR_LOCALE]->(l)
RETURN bn.key AS created_en_us;

// Create fr-FR BlockNatives
MATCH (b:Block)
WHERE b.key IN ["block:qr-code-hero", "block:qr-code-what-is", "block:qr-code-use-cases", "block:qr-code-cta"]
  AND NOT (b)-[:HAS_NATIVE]->(:BlockNative {key: b.key + "@fr-FR"})
MATCH (l:Locale {key: "fr-FR"})
CREATE (bn:BlockNative {
  key: b.key + "@fr-FR",
  display_name: CASE b.key
    WHEN "block:qr-code-hero" THEN "Hero Code QR (fr-FR)"
    WHEN "block:qr-code-what-is" THEN "Qu'est-ce qu'un Code QR (fr-FR)"
    WHEN "block:qr-code-use-cases" THEN "Cas d'utilisation Code QR (fr-FR)"
    WHEN "block:qr-code-cta" THEN "CTA Code QR (fr-FR)"
    ELSE b.display_name + " (fr-FR)"
  END,
  description: b.description,
  content: "<!-- Placeholder généré - nécessite génération de contenu -->",
  block_type: CASE
    WHEN b.key CONTAINS "hero" THEN "hero"
    WHEN b.key CONTAINS "what-is" THEN "explainer"
    WHEN b.key CONTAINS "use-cases" THEN "features"
    WHEN b.key CONTAINS "cta" THEN "cta"
    ELSE "content"
  END,
  created_at: datetime(),
  updated_at: datetime()
})
MERGE (b)-[:HAS_NATIVE]->(bn)
MERGE (bn)-[:FOR_LOCALE]->(l)
RETURN bn.key AS created_fr_fr;

// Summary
MATCH (b:Block)-[:HAS_NATIVE]->(bn:BlockNative)
WHERE b.key IN ["block:qr-code-hero", "block:qr-code-what-is", "block:qr-code-use-cases", "block:qr-code-cta"]
RETURN b.key AS block, count(bn) AS native_count;
