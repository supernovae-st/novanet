// ═══════════════════════════════════════════════════════════════════════════════
// SEOKeyword fr-FR — 1424 keywords
// Generated: 2026-02-09T00:00:00.000000
// ═══════════════════════════════════════════════════════════════════════════════

// Create index for fast lookup
CREATE INDEX seokeyword_value IF NOT EXISTS FOR (kw:SEOKeyword) ON (kw.value);
CREATE INDEX seokeyword_volume IF NOT EXISTS FOR (kw:SEOKeyword) ON (kw.volume);

// ───────────────────────────────────────────────────────────────────────────────
// SEOKeyword Nodes + TARGETS relationships (grouped by Entity)
// ───────────────────────────────────────────────────────────────────────────────

// --- barcode (9 keywords) ---

MERGE (kw:SEOKeyword {key: "seo-generate-qr-barcode-fr-fr-58c199"})
ON CREATE SET
  kw.value = "generate qr barcode",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "barcode", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generate-qr-barcode-fr-fr-58c199"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-barre-qr-fr-fr-ec83e7"})
ON CREATE SET
  kw.value = "code barre qr",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "barcode", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-barre-qr-fr-fr-ec83e7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-barre-qr-code-fr-fr-969d99"})
ON CREATE SET
  kw.value = "code barre qr code",
  kw.volume = 60,
  kw.difficulty = 14,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "barcode", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-barre-qr-code-fr-fr-969d99"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-barre-fr-fr-e843c1"})
ON CREATE SET
  kw.value = "qr code barre",
  kw.volume = 60,
  kw.difficulty = 35,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "barcode", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-barre-fr-fr-e843c1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-fr-fr-aa74a6"})
ON CREATE SET
  kw.value = "qr code generator",
  kw.volume = 52000,
  kw.difficulty = 93,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-fr-fr-aa74a6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-fr-fr-24baec"})
ON CREATE SET
  kw.value = "créer un qr code",
  kw.volume = 14000,
  kw.difficulty = 28,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-fr-fr-24baec"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-fr-fr-4c0735"})
ON CREATE SET
  kw.value = "générer un qr code",
  kw.volume = 8600,
  kw.difficulty = 31,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-fr-fr-4c0735"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-fr-fr-7780ab"})
ON CREATE SET
  kw.value = "créer qr code",
  kw.volume = 7300,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-fr-fr-7780ab"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-fr-fr-d54a46"})
ON CREATE SET
  kw.value = "générer qr code",
  kw.volume = 7200,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-fr-fr-d54a46"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-qr-code-fr-fr-702823"})
ON CREATE SET
  kw.value = "creer qr code",
  kw.volume = 6300,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-qr-code-fr-fr-702823"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-qr-code-fr-fr-8c09af"})
ON CREATE SET
  kw.value = "creer un qr code",
  kw.volume = 5100,
  kw.difficulty = 34,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-qr-code-fr-fr-8c09af"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuitement-fr-fr-3e3ba3"})
ON CREATE SET
  kw.value = "créer un qr code gratuitement",
  kw.volume = 4200,
  kw.difficulty = 22,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuitement-fr-fr-3e3ba3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-fr-fr-da4d00"})
ON CREATE SET
  kw.value = "comment créer un qr code",
  kw.volume = 3400,
  kw.difficulty = 21,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-fr-fr-da4d00"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-gratuit-fr-fr-fb10fb"})
ON CREATE SET
  kw.value = "générer qr code gratuit",
  kw.volume = 3100,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-gratuit-fr-fr-fb10fb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-gratuit-fr-fr-d42ace"})
ON CREATE SET
  kw.value = "créer qr code gratuit",
  kw.volume = 3000,
  kw.difficulty = 23,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-gratuit-fr-fr-d42ace"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuit-fr-fr-9b8bbf"})
ON CREATE SET
  kw.value = "créer un qr code gratuit",
  kw.volume = 3000,
  kw.difficulty = 34,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuit-fr-fr-9b8bbf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generer-qr-code-fr-fr-66fd76"})
ON CREATE SET
  kw.value = "generer qr code",
  kw.volume = 2600,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generer-qr-code-fr-fr-66fd76"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generator-qr-code-fr-fr-938fb7"})
ON CREATE SET
  kw.value = "generator qr code",
  kw.volume = 2600,
  kw.difficulty = 83,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generator-qr-code-fr-fr-938fb7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-gratuit-fr-fr-629a42"})
ON CREATE SET
  kw.value = "qr code generator gratuit",
  kw.volume = 2500,
  kw.difficulty = 21,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-gratuit-fr-fr-629a42"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generer-un-qr-code-fr-fr-188b44"})
ON CREATE SET
  kw.value = "generer un qr code",
  kw.volume = 1800,
  kw.difficulty = 33,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generer-un-qr-code-fr-fr-188b44"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-generator-fr-fr-2066f7"})
ON CREATE SET
  kw.value = "qr generator",
  kw.volume = 1300,
  kw.difficulty = 91,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-generator-fr-fr-2066f7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-free-fr-fr-35e97e"})
ON CREATE SET
  kw.value = "qr code generator free",
  kw.volume = 1300,
  kw.difficulty = 92,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-free-fr-fr-35e97e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generator-qr-code-gratuit-fr-fr-6efc43"})
ON CREATE SET
  kw.value = "generator qr code gratuit",
  kw.volume = 1200,
  kw.difficulty = 36,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generator-qr-code-gratuit-fr-fr-6efc43"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-générer-un-qr-code-fr-fr-3e25a7"})
ON CREATE SET
  kw.value = "comment générer un qr code",
  kw.volume = 1100,
  kw.difficulty = 21,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-générer-un-qr-code-fr-fr-3e25a7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-gratuit-fr-fr-a44c7a"})
ON CREATE SET
  kw.value = "générer un qr code gratuit",
  kw.volume = 1100,
  kw.difficulty = 34,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-gratuit-fr-fr-a44c7a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-free-qr-code-generator-fr-fr-254f63"})
ON CREATE SET
  kw.value = "free qr code generator",
  kw.volume = 1100,
  kw.difficulty = 92,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-free-qr-code-generator-fr-fr-254f63"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-à-partir-dun-lien-fr-fr-c708c6"})
ON CREATE SET
  kw.value = "créer un qr code à partir d\'un lien",
  kw.volume = 1000,
  kw.difficulty = 6,
  kw.cpc = 0.45,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-à-partir-dun-lien-fr-fr-c708c6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuit-sans-inscription-fr-fr-b13f49"})
ON CREATE SET
  kw.value = "créer un qr code gratuit sans inscription",
  kw.volume = 1000,
  kw.difficulty = 12,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuit-sans-inscription-fr-fr-b13f49"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-create-qr-code-fr-fr-032faa"})
ON CREATE SET
  kw.value = "create qr code",
  kw.volume = 1000,
  kw.difficulty = 37,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-create-qr-code-fr-fr-032faa"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-avec-un-lien-fr-fr-f4b107"})
ON CREATE SET
  kw.value = "créer un qr code avec un lien",
  kw.volume = 900,
  kw.difficulty = 21,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-avec-un-lien-fr-fr-f4b107"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-create-a-qr-code-fr-fr-015565"})
ON CREATE SET
  kw.value = "how to create a qr code",
  kw.volume = 800,
  kw.difficulty = 90,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-create-a-qr-code-fr-fr-015565"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-creer-un-qr-code-fr-fr-c475ed"})
ON CREATE SET
  kw.value = "comment creer un qr code",
  kw.volume = 600,
  kw.difficulty = 22,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-creer-un-qr-code-fr-fr-c475ed"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-maker-fr-fr-4a523b"})
ON CREATE SET
  kw.value = "qr code maker",
  kw.volume = 600,
  kw.difficulty = 65,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-maker-fr-fr-4a523b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-à-partir-dun-lien-fr-fr-a2d273"})
ON CREATE SET
  kw.value = "générer un qr code à partir d\'un lien",
  kw.volume = 450,
  kw.difficulty = 21,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-à-partir-dun-lien-fr-fr-a2d273"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-qr-code-gratuit-fr-fr-0ccc8e"})
ON CREATE SET
  kw.value = "creer un qr code gratuit",
  kw.volume = 400,
  kw.difficulty = 35,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-qr-code-gratuit-fr-fr-0ccc8e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-online-qr-generator-fr-fr-d56264"})
ON CREATE SET
  kw.value = "online qr generator",
  kw.volume = 400,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-online-qr-generator-fr-fr-d56264"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-qr-code-gratuit-fr-fr-b6f767"})
ON CREATE SET
  kw.value = "creer qr code gratuit",
  kw.volume = 400,
  kw.difficulty = 34,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-qr-code-gratuit-fr-fr-b6f767"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-adobe-qr-code-generator-fr-fr-314a29"})
ON CREATE SET
  kw.value = "adobe qr code generator",
  kw.volume = 350,
  kw.difficulty = 2,
  kw.cpc = 0.7,
  kw.intent = "Informational,Navigational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-adobe-qr-code-generator-fr-fr-314a29"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr-generator-fr-fr-3514ed"})
ON CREATE SET
  kw.value = "code qr generator",
  kw.volume = 350,
  kw.difficulty = 35,
  kw.cpc = 0.35,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr-generator-fr-fr-3514ed"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuit-pdf-fr-fr-18423a"})
ON CREATE SET
  kw.value = "créer un qr code gratuit pdf",
  kw.volume = 350,
  kw.difficulty = 13,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuit-pdf-fr-fr-18423a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-gratuitement-fr-fr-989744"})
ON CREATE SET
  kw.value = "générer un qr code gratuitement",
  kw.volume = 350,
  kw.difficulty = 35,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-gratuitement-fr-fr-989744"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-code-qr-fr-fr-0dd471"})
ON CREATE SET
  kw.value = "créer un code qr",
  kw.volume = 300,
  kw.difficulty = 22,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-code-qr-fr-fr-0dd471"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-qr-code-avec-canva-fr-fr-5b9b5f"})
ON CREATE SET
  kw.value = "creer un qr code avec canva",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-qr-code-avec-canva-fr-fr-5b9b5f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-gratuit-fr-fr-6c5f59"})
ON CREATE SET
  kw.value = "comment créer un qr code gratuit",
  kw.volume = 250,
  kw.difficulty = 21,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-gratuit-fr-fr-6c5f59"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-sur-canva-fr-fr-71e21a"})
ON CREATE SET
  kw.value = "comment créer un qr code sur canva",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-sur-canva-fr-fr-71e21a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-fr-fr-ffb3af"})
ON CREATE SET
  kw.value = "qr-code-generator",
  kw.volume = 250,
  kw.difficulty = 93,
  kw.cpc = 0.25,
  kw.intent = "Navigational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-fr-fr-ffb3af"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-son-qr-code-fr-fr-8f8988"})
ON CREATE SET
  kw.value = "créer son qr code",
  kw.volume = 250,
  kw.difficulty = 20,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-son-qr-code-fr-fr-8f8988"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-canva-fr-fr-596038"})
ON CREATE SET
  kw.value = "créer qr code canva",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-canva-fr-fr-596038"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-createur-de-qr-code-fr-fr-63b64c"})
ON CREATE SET
  kw.value = "createur de qr code",
  kw.volume = 250,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-createur-de-qr-code-fr-fr-63b64c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generer-qr-code-gratuit-fr-fr-895942"})
ON CREATE SET
  kw.value = "generer qr code gratuit",
  kw.volume = 250,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generer-qr-code-gratuit-fr-fr-895942"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-createur-qr-code-fr-fr-d9ecc9"})
ON CREATE SET
  kw.value = "createur qr code",
  kw.volume = 200,
  kw.difficulty = 50,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-createur-qr-code-fr-fr-d9ecc9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-adobe-fr-fr-8c65bc"})
ON CREATE SET
  kw.value = "qr code generator adobe",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Navigational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-adobe-fr-fr-8c65bc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-barcode-generator-fr-fr-36158d"})
ON CREATE SET
  kw.value = "qr barcode generator",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-barcode-generator-fr-fr-36158d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-barcode-maker-fr-fr-f18838"})
ON CREATE SET
  kw.value = "qr barcode maker",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-barcode-maker-fr-fr-f18838"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-canva-qr-code-generator-fr-fr-08ee21"})
ON CREATE SET
  kw.value = "canva qr code generator",
  kw.volume = 200,
  kw.difficulty = 7,
  kw.cpc = 0.5,
  kw.intent = "Informational,Navigational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-canva-qr-code-generator-fr-fr-08ee21"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generer-un-qr-code-gratuit-fr-fr-f60a45"})
ON CREATE SET
  kw.value = "generer un qr code gratuit",
  kw.volume = 200,
  kw.difficulty = 34,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generer-un-qr-code-gratuit-fr-fr-f60a45"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-qr-code-pour-un-pdf-fr-fr-8dff47"})
ON CREATE SET
  kw.value = "creer un qr code pour un pdf",
  kw.volume = 200,
  kw.difficulty = 10,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-qr-code-pour-un-pdf-fr-fr-8dff47"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-canva-fr-fr-dcd00e"})
ON CREATE SET
  kw.value = "générer qr code canva",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-canva-fr-fr-dcd00e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-online-qr-code-generator-fr-fr-43e5d1"})
ON CREATE SET
  kw.value = "online qr code generator",
  kw.volume = 200,
  kw.difficulty = 93,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-online-qr-code-generator-fr-fr-43e5d1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-pour-une-vidéo-fr-fr-1b0762"})
ON CREATE SET
  kw.value = "créer un qr code pour une vidéo",
  kw.volume = 200,
  kw.difficulty = 12,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-pour-une-vidéo-fr-fr-1b0762"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-code-generator-fr-fr-fd0a45"})
ON CREATE SET
  kw.value = "qr code code generator",
  kw.volume = 200,
  kw.difficulty = 92,
  kw.cpc = 0.0,
  kw.intent = "Navigational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-code-generator-fr-fr-fd0a45"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-online-fr-fr-c84987"})
ON CREATE SET
  kw.value = "qr code generator online",
  kw.volume = 200,
  kw.difficulty = 91,
  kw.cpc = 0.4,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-online-fr-fr-c84987"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-code-fr-fr-79d6f4"})
ON CREATE SET
  kw.value = "qr code generator code",
  kw.volume = 150,
  kw.difficulty = 93,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-code-fr-fr-79d6f4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-create-qr-code-free-fr-fr-95b2d3"})
ON CREATE SET
  kw.value = "create qr code free",
  kw.volume = 150,
  kw.difficulty = 92,
  kw.cpc = 0.45,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-create-qr-code-free-fr-fr-95b2d3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-sur-canva-fr-fr-794524"})
ON CREATE SET
  kw.value = "créer un qr code sur canva",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-sur-canva-fr-fr-794524"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuit-et-illimité-fr-fr-a2b9e6"})
ON CREATE SET
  kw.value = "créer un qr code gratuit et illimité",
  kw.volume = 150,
  kw.difficulty = 24,
  kw.cpc = 0.45,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-gratuit-et-illimité-fr-fr-a2b9e6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-pro-fr-fr-60209a"})
ON CREATE SET
  kw.value = "qr code generator pro",
  kw.volume = 150,
  kw.difficulty = 42,
  kw.cpc = 0.9,
  kw.intent = "Informational,Commercial,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-pro-fr-fr-60209a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-carte-de-visite-fr-fr-b32f8f"})
ON CREATE SET
  kw.value = "créer qr code carte de visite",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-carte-de-visite-fr-fr-b32f8f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-des-qr-code-fr-fr-5f70e6"})
ON CREATE SET
  kw.value = "générer des qr code",
  kw.volume = 150,
  kw.difficulty = 31,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-des-qr-code-fr-fr-5f70e6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-barcode-code-generator-fr-fr-748fae"})
ON CREATE SET
  kw.value = "qr barcode code generator",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-barcode-code-generator-fr-fr-748fae"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-qr-code-fr-fr-f32dc1"})
ON CREATE SET
  kw.value = "comment créer qr code",
  kw.volume = 150,
  kw.difficulty = 22,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-qr-code-fr-fr-f32dc1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-son-qr-code-carte-de-visite-fr-fr-18211e"})
ON CREATE SET
  kw.value = "créer son qr code carte de visite",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-son-qr-code-carte-de-visite-fr-fr-18211e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-canva-fr-fr-267376"})
ON CREATE SET
  kw.value = "générer un qr code canva",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-canva-fr-fr-267376"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-carte-de-visite-gratuit-fr-fr-283ccf"})
ON CREATE SET
  kw.value = "créer un qr code carte de visite gratuit",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-carte-de-visite-gratuit-fr-fr-283ccf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-créer-fr-fr-60b97b"})
ON CREATE SET
  kw.value = "qr code créer",
  kw.volume = 150,
  kw.difficulty = 23,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-créer-fr-fr-60b97b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-qr-code-gratuitement-fr-fr-d9917c"})
ON CREATE SET
  kw.value = "creer un qr code gratuitement",
  kw.volume = 150,
  kw.difficulty = 35,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-qr-code-gratuitement-fr-fr-d9917c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-gratuit-illimité-fr-fr-250ce4"})
ON CREATE SET
  kw.value = "qr code generator gratuit illimité",
  kw.volume = 150,
  kw.difficulty = 37,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-gratuit-illimité-fr-fr-250ce4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-generer-un-qr-code-fr-fr-2c19c5"})
ON CREATE SET
  kw.value = "comment generer un qr code",
  kw.volume = 150,
  kw.difficulty = 24,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-generer-un-qr-code-fr-fr-2c19c5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-google-qr-code-generator-fr-fr-ead5cf"})
ON CREATE SET
  kw.value = "google qr code generator",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-google-qr-code-generator-fr-fr-ead5cf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-des-qr-code-fr-fr-a71726"})
ON CREATE SET
  kw.value = "créer des qr code",
  kw.volume = 150,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-des-qr-code-fr-fr-a71726"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-create-a-qr-code-fr-fr-cf6fdb"})
ON CREATE SET
  kw.value = "create a qr code",
  kw.volume = 150,
  kw.difficulty = 91,
  kw.cpc = 0.35,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-create-a-qr-code-fr-fr-cf6fdb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-google-fr-fr-64400b"})
ON CREATE SET
  kw.value = "qr code generator google",
  kw.volume = 150,
  kw.difficulty = 39,
  kw.cpc = 0.45,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-google-fr-fr-64400b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-code-qr-fr-fr-9b1d14"})
ON CREATE SET
  kw.value = "générer un code qr",
  kw.volume = 150,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-code-qr-fr-fr-9b1d14"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-à-partir-dun-lien-fr-fr-2b54fd"})
ON CREATE SET
  kw.value = "comment créer un qr code à partir d\'un lien",
  kw.volume = 150,
  kw.difficulty = 6,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-à-partir-dun-lien-fr-fr-2b54fd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-code-qr-carte-de-visite-fr-fr-a6c5ae"})
ON CREATE SET
  kw.value = "créer code qr carte de visite",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-code-qr-carte-de-visite-fr-fr-a6c5ae"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-à-partir-dun-lien-fr-fr-f29cd9"})
ON CREATE SET
  kw.value = "créer qr code à partir d\'un lien",
  kw.volume = 150,
  kw.difficulty = 17,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-à-partir-dun-lien-fr-fr-f29cd9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-carte-de-visite-fr-fr-27c1cb"})
ON CREATE SET
  kw.value = "créer un qr code carte de visite",
  kw.volume = 150,
  kw.difficulty = 7,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-carte-de-visite-fr-fr-27c1cb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-canva-fr-fr-5dea54"})
ON CREATE SET
  kw.value = "créer un qr code canva",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-canva-fr-fr-5dea54"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-dbl-fr-fr-24a003"})
ON CREATE SET
  kw.value = "qr code generator dbl",
  kw.volume = 150,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-dbl-fr-fr-24a003"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-générer-fr-fr-d1403d"})
ON CREATE SET
  kw.value = "qr code générer",
  kw.volume = 150,
  kw.difficulty = 34,
  kw.cpc = 0.4,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-générer-fr-fr-d1403d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-gratuitement-fr-fr-867eab"})
ON CREATE SET
  kw.value = "comment créer un qr code gratuitement",
  kw.volume = 100,
  kw.difficulty = 22,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-gratuitement-fr-fr-867eab"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-creer-fr-fr-001005"})
ON CREATE SET
  kw.value = "qr code creer",
  kw.volume = 100,
  kw.difficulty = 34,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-creer-fr-fr-001005"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-the-qr-code-generator-fr-fr-ab63aa"})
ON CREATE SET
  kw.value = "the qr code generator",
  kw.volume = 100,
  kw.difficulty = 91,
  kw.cpc = 0.3,
  kw.intent = "Informational,Navigational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-the-qr-code-generator-fr-fr-ab63aa"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-pour-une-vidéo-fr-fr-239432"})
ON CREATE SET
  kw.value = "comment créer un qr code pour une vidéo",
  kw.volume = 100,
  kw.difficulty = 5,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-pour-une-vidéo-fr-fr-239432"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-à-partir-dun-pdf-fr-fr-1873d9"})
ON CREATE SET
  kw.value = "créer un qr code à partir d\'un pdf",
  kw.volume = 100,
  kw.difficulty = 6,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-à-partir-dun-pdf-fr-fr-1873d9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-code-qr-fr-fr-51d64e"})
ON CREATE SET
  kw.value = "créer code qr",
  kw.volume = 100,
  kw.difficulty = 22,
  kw.cpc = 0.35,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-code-qr-fr-fr-51d64e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-with-logo-fr-fr-b6939e"})
ON CREATE SET
  kw.value = "qr code generator with logo",
  kw.volume = 100,
  kw.difficulty = 78,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-with-logo-fr-fr-b6939e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-monkey-qr-code-generator-fr-fr-a69d8d"})
ON CREATE SET
  kw.value = "monkey qr code generator",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-monkey-qr-code-generator-fr-fr-a69d8d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-free-online-fr-fr-aaf07f"})
ON CREATE SET
  kw.value = "qr code generator free online",
  kw.volume = 100,
  kw.difficulty = 94,
  kw.cpc = 0.4,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-free-online-fr-fr-aaf07f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-code-qr-fr-fr-8b310f"})
ON CREATE SET
  kw.value = "creer code qr",
  kw.volume = 100,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-code-qr-fr-fr-8b310f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-monkey-fr-fr-2571b2"})
ON CREATE SET
  kw.value = "qr code generator monkey",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-monkey-fr-fr-2571b2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-create-free-ai-qr-fr-fr-906909"})
ON CREATE SET
  kw.value = "create free ai qr",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-create-free-ai-qr-fr-fr-906909"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-gratuit-sans-inscription-fr-fr-d07298"})
ON CREATE SET
  kw.value = "générer qr code gratuit sans inscription",
  kw.volume = 100,
  kw.difficulty = 22,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-gratuit-sans-inscription-fr-fr-d07298"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-create-qr-code-from-url-fr-fr-f3894c"})
ON CREATE SET
  kw.value = "create qr code from url",
  kw.volume = 100,
  kw.difficulty = 89,
  kw.cpc = 0.5,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-create-qr-code-from-url-fr-fr-f3894c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-generator-free-fr-fr-18e6d0"})
ON CREATE SET
  kw.value = "qr generator free",
  kw.volume = 100,
  kw.difficulty = 89,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-generator-free-fr-fr-18e6d0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-qr-code-video-fr-fr-c4892f"})
ON CREATE SET
  kw.value = "creer qr code video",
  kw.volume = 100,
  kw.difficulty = 12,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-qr-code-video-fr-fr-c4892f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-instagram-fr-fr-61956b"})
ON CREATE SET
  kw.value = "générer qr code instagram",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-instagram-fr-fr-61956b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-creer-un-qr-code-pour-un-lien-fr-fr-6f2791"})
ON CREATE SET
  kw.value = "comment creer un qr code pour un lien",
  kw.volume = 100,
  kw.difficulty = 6,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-creer-un-qr-code-pour-un-lien-fr-fr-6f2791"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-qr-code-pour-carte-de-visite-fr-fr-d3ca8e"})
ON CREATE SET
  kw.value = "creer un qr code pour carte de visite",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-qr-code-pour-carte-de-visite-fr-fr-d3ca8e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generer-fr-fr-067baf"})
ON CREATE SET
  kw.value = "qr code generer",
  kw.volume = 100,
  kw.difficulty = 93,
  kw.cpc = 0.4,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generer-fr-fr-067baf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-qr-code-video-fr-fr-56f502"})
ON CREATE SET
  kw.value = "creer un qr code video",
  kw.volume = 100,
  kw.difficulty = 12,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-qr-code-video-fr-fr-56f502"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-dbl-qr-code-generator-fr-fr-f5e95c"})
ON CREATE SET
  kw.value = "dbl qr code generator",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-dbl-qr-code-generator-fr-fr-f5e95c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-illustrator-fr-fr-5e9da6"})
ON CREATE SET
  kw.value = "générer qr code illustrator",
  kw.volume = 100,
  kw.difficulty = 23,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-illustrator-fr-fr-5e9da6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-fabriquer-un-qr-code-fr-fr-90e2ab"})
ON CREATE SET
  kw.value = "fabriquer un qr code",
  kw.volume = 100,
  kw.difficulty = 21,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-fabriquer-un-qr-code-fr-fr-90e2ab"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-generator-gratuit-fr-fr-924e7a"})
ON CREATE SET
  kw.value = "qr generator gratuit",
  kw.volume = 100,
  kw.difficulty = 24,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-generator-gratuit-fr-fr-924e7a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-des-qr-codes-fr-fr-7232d0"})
ON CREATE SET
  kw.value = "creer des qr codes",
  kw.volume = 90,
  kw.difficulty = 29,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-des-qr-codes-fr-fr-7232d0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-codes-generator-fr-fr-1ba229"})
ON CREATE SET
  kw.value = "qr codes generator",
  kw.volume = 90,
  kw.difficulty = 87,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-codes-generator-fr-fr-1ba229"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-son-qr-code-fr-fr-9448cb"})
ON CREATE SET
  kw.value = "creer son qr code",
  kw.volume = 90,
  kw.difficulty = 33,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-son-qr-code-fr-fr-9448cb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-kód-generator-fr-fr-df51b3"})
ON CREATE SET
  kw.value = "qr kód generator",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-kód-generator-fr-fr-df51b3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-sur-canva-fr-fr-473c20"})
ON CREATE SET
  kw.value = "générer un qr code sur canva",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-sur-canva-fr-fr-473c20"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-gratuit-illimité-fr-fr-af7c18"})
ON CREATE SET
  kw.value = "générer un qr code gratuit illimité",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-gratuit-illimité-fr-fr-af7c18"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generer-un-qr-code-pour-un-lien-fr-fr-95345d"})
ON CREATE SET
  kw.value = "generer un qr code pour un lien",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generer-un-qr-code-pour-un-lien-fr-fr-95345d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-pour-une-vidéo-mp4-fr-fr-1cdff2"})
ON CREATE SET
  kw.value = "créer un qr code pour une vidéo mp4",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-pour-une-vidéo-mp4-fr-fr-1cdff2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-tiger-qr-code-generator-fr-fr-52f2e1"})
ON CREATE SET
  kw.value = "qr tiger qr code generator",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-tiger-qr-code-generator-fr-fr-52f2e1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-code-qr-fr-fr-2ac312"})
ON CREATE SET
  kw.value = "comment créer un code qr",
  kw.volume = 80,
  kw.difficulty = 22,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-code-qr-fr-fr-2ac312"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-gratuit-illimité-fr-fr-429824"})
ON CREATE SET
  kw.value = "créer qr code gratuit illimité",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-gratuit-illimité-fr-fr-429824"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-free-no-expiration-fr-fr-7f0609"})
ON CREATE SET
  kw.value = "qr code generator free no expiration",
  kw.volume = 80,
  kw.difficulty = 78,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-free-no-expiration-fr-fr-7f0609"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-free-generator-fr-fr-4db260"})
ON CREATE SET
  kw.value = "qr code free generator",
  kw.volume = 80,
  kw.difficulty = 90,
  kw.cpc = 0.25,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-free-generator-fr-fr-4db260"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-son-qr-code-fr-fr-c27da2"})
ON CREATE SET
  kw.value = "comment créer son qr code",
  kw.volume = 80,
  kw.difficulty = 22,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-son-qr-code-fr-fr-c27da2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-canva-fr-fr-e53b85"})
ON CREATE SET
  kw.value = "qr code generator canva",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-canva-fr-fr-e53b85"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-qr-code-avec-un-lien-fr-fr-510210"})
ON CREATE SET
  kw.value = "creer qr code avec un lien",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-qr-code-avec-un-lien-fr-fr-510210"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-avec-lien-fr-fr-61e1fc"})
ON CREATE SET
  kw.value = "créer qr code avec lien",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-avec-lien-fr-fr-61e1fc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-linkedin-fr-fr-60f8f5"})
ON CREATE SET
  kw.value = "générer qr code linkedin",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-linkedin-fr-fr-60f8f5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-avis-google-fr-fr-f6e2f2"})
ON CREATE SET
  kw.value = "créer qr code avis google",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Commercial,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-avis-google-fr-fr-f6e2f2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-best-free-qr-code-generator-fr-fr-4ae836"})
ON CREATE SET
  kw.value = "best free qr code generator",
  kw.volume = 80,
  kw.difficulty = 90,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-best-free-qr-code-generator-fr-fr-4ae836"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-maker-fr-fr-a25051"})
ON CREATE SET
  kw.value = "qr maker",
  kw.volume = 80,
  kw.difficulty = 53,
  kw.cpc = 0.25,
  kw.intent = "Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-maker-fr-fr-a25051"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-qr-code-avec-un-lien-fr-fr-db0560"})
ON CREATE SET
  kw.value = "creer un qr code avec un lien",
  kw.volume = 80,
  kw.difficulty = 5,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-qr-code-avec-un-lien-fr-fr-db0560"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-instagram-fr-fr-4433ca"})
ON CREATE SET
  kw.value = "générer un qr code instagram",
  kw.volume = 80,
  kw.difficulty = 2,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-instagram-fr-fr-4433ca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-pour-partager-des-photos-fr-fr-1875cd"})
ON CREATE SET
  kw.value = "créer un qr code pour partager des photos",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-pour-partager-des-photos-fr-fr-1875cd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-contact-gratuit-fr-fr-483587"})
ON CREATE SET
  kw.value = "créer un qr code contact gratuit",
  kw.volume = 80,
  kw.difficulty = 34,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-contact-gratuit-fr-fr-483587"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-maker-free-fr-fr-cdaaeb"})
ON CREATE SET
  kw.value = "qr code maker free",
  kw.volume = 80,
  kw.difficulty = 92,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-maker-free-fr-fr-cdaaeb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-mon-qr-code-fr-fr-127a33"})
ON CREATE SET
  kw.value = "créer mon qr code",
  kw.volume = 70,
  kw.difficulty = 21,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-mon-qr-code-fr-fr-127a33"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generer-un-qr-code-gratuitement-fr-fr-c03536"})
ON CREATE SET
  kw.value = "generer un qr code gratuitement",
  kw.volume = 70,
  kw.difficulty = 22,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generer-un-qr-code-gratuitement-fr-fr-c03536"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-générer-un-qr-code-sur-canva-fr-fr-dd4c0d"})
ON CREATE SET
  kw.value = "comment générer un qr code sur canva",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-générer-un-qr-code-sur-canva-fr-fr-dd4c0d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-de-paiement-fr-fr-35ef89"})
ON CREATE SET
  kw.value = "comment créer un qr code de paiement",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-de-paiement-fr-fr-35ef89"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generator-qr-fr-fr-0e2c61"})
ON CREATE SET
  kw.value = "generator qr",
  kw.volume = 70,
  kw.difficulty = 38,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generator-qr-fr-fr-0e2c61"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-qr-code-pour-une-photo-fr-fr-764573"})
ON CREATE SET
  kw.value = "creer un qr code pour une photo",
  kw.volume = 70,
  kw.difficulty = 23,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-qr-code-pour-une-photo-fr-fr-764573"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-gratuitement-fr-fr-b28875"})
ON CREATE SET
  kw.value = "créer qr code gratuitement",
  kw.volume = 70,
  kw.difficulty = 35,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-gratuitement-fr-fr-b28875"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-create-a-qr-code-for-a-url-fr-fr-050bec"})
ON CREATE SET
  kw.value = "create a qr code for a url",
  kw.volume = 70,
  kw.difficulty = 90,
  kw.cpc = 0.35,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-create-a-qr-code-for-a-url-fr-fr-050bec"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-code-qr-fr-fr-fe0514"})
ON CREATE SET
  kw.value = "générer code qr",
  kw.volume = 70,
  kw.difficulty = 29,
  kw.cpc = 0.25,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-code-qr-fr-fr-fe0514"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-son-qr-code-gratuitement-fr-fr-90ceae"})
ON CREATE SET
  kw.value = "creer son qr code gratuitement",
  kw.volume = 70,
  kw.difficulty = 34,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-son-qr-code-gratuitement-fr-fr-90ceae"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-wifi-fr-fr-c92693"})
ON CREATE SET
  kw.value = "générer qr code wifi",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-wifi-fr-fr-c92693"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generer-qr-code-canva-fr-fr-670eac"})
ON CREATE SET
  kw.value = "generer qr code canva",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generer-qr-code-canva-fr-fr-670eac"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-avec-canva-fr-fr-3d9efe"})
ON CREATE SET
  kw.value = "créer un qr code avec canva",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-avec-canva-fr-fr-3d9efe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-open-source-fr-fr-7c4df6"})
ON CREATE SET
  kw.value = "qr code generator open source",
  kw.volume = 60,
  kw.difficulty = 64,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-open-source-fr-fr-7c4df6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-avis-google-fr-fr-3ac2a9"})
ON CREATE SET
  kw.value = "générer qr code avis google",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-avis-google-fr-fr-3ac2a9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-instagram-fr-fr-52a06f"})
ON CREATE SET
  kw.value = "créer un qr code instagram",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-instagram-fr-fr-52a06f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-avec-un-lien-fr-fr-1d347a"})
ON CREATE SET
  kw.value = "comment créer un qr code avec un lien",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-avec-un-lien-fr-fr-1d347a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-adobe-fr-fr-8cc7d2"})
ON CREATE SET
  kw.value = "générer qr code adobe",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-adobe-fr-fr-8cc7d2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-qr-code-canva-fr-fr-f11cf8"})
ON CREATE SET
  kw.value = "creer qr code canva",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-qr-code-canva-fr-fr-f11cf8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-create-a-qr-code-for-a-link-fr-fr-f0eea2"})
ON CREATE SET
  kw.value = "how to create a qr code for a link",
  kw.volume = 60,
  kw.difficulty = 89,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-create-a-qr-code-for-a-link-fr-fr-f0eea2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-gratuit-sans-inscription-fr-fr-850614"})
ON CREATE SET
  kw.value = "générer un qr code gratuit sans inscription",
  kw.volume = 60,
  kw.difficulty = 24,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-gratuit-sans-inscription-fr-fr-850614"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-facebook-fr-fr-27b618"})
ON CREATE SET
  kw.value = "créer un qr code facebook",
  kw.volume = 60,
  kw.difficulty = 5,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-facebook-fr-fr-27b618"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-qr-code-pour-avis-google-fr-fr-ae0375"})
ON CREATE SET
  kw.value = "creer qr code pour avis google",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-qr-code-pour-avis-google-fr-fr-ae0375"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-create-free-qr-code-fr-fr-531e17"})
ON CREATE SET
  kw.value = "create free qr code",
  kw.volume = 60,
  kw.difficulty = 90,
  kw.cpc = 0.45,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-create-free-qr-code-fr-fr-531e17"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-instagram-fr-fr-38cb68"})
ON CREATE SET
  kw.value = "créer qr code instagram",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-instagram-fr-fr-38cb68"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-gratuitement-fr-fr-56b447"})
ON CREATE SET
  kw.value = "générer qr code gratuitement",
  kw.volume = 60,
  kw.difficulty = 34,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-gratuitement-fr-fr-56b447"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-svg-fr-fr-93cf89"})
ON CREATE SET
  kw.value = "qr code generator svg",
  kw.volume = 60,
  kw.difficulty = 84,
  kw.cpc = 0.5,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-svg-fr-fr-93cf89"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-instagram-fr-fr-ccdb8c"})
ON CREATE SET
  kw.value = "comment créer un qr code instagram",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-instagram-fr-fr-ccdb8c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-générer-un-qr-code-gratuitement-fr-fr-34310b"})
ON CREATE SET
  kw.value = "comment générer un qr code gratuitement",
  kw.volume = 60,
  kw.difficulty = 22,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-générer-un-qr-code-gratuitement-fr-fr-34310b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-fabriquer-un-qr-code-gratuit-fr-fr-93e78a"})
ON CREATE SET
  kw.value = "fabriquer un qr code gratuit",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-fabriquer-un-qr-code-gratuit-fr-fr-93e78a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-gratuit-illimité-fr-fr-83ff33"})
ON CREATE SET
  kw.value = "générer qr code gratuit illimité",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-gratuit-illimité-fr-fr-83ff33"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-fabriquer-un-qr-code-fr-fr-1b421d"})
ON CREATE SET
  kw.value = "comment fabriquer un qr code",
  kw.volume = 60,
  kw.difficulty = 7,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-fabriquer-un-qr-code-fr-fr-1b421d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-pour-un-pdf-fr-fr-b4f536"})
ON CREATE SET
  kw.value = "comment créer un qr code pour un pdf",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code-pour-un-pdf-fr-fr-b4f536"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generator-qr-code-dragon-ball-legends-fr-fr-3aed41"})
ON CREATE SET
  kw.value = "generator qr code dragon ball legends",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generator-qr-code-dragon-ball-legends-fr-fr-3aed41"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-wifi-generator-fr-fr-ce63b1"})
ON CREATE SET
  kw.value = "qr code wifi generator",
  kw.volume = 60,
  kw.difficulty = 7,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-wifi-generator-fr-fr-ce63b1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-generator-ai-fr-fr-7abc59"})
ON CREATE SET
  kw.value = "qr generator ai",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-generator-ai-fr-fr-7abc59"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-un-code-qr-fr-fr-a7f851"})
ON CREATE SET
  kw.value = "creer un code qr",
  kw.volume = 60,
  kw.difficulty = 22,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-un-code-qr-fr-fr-a7f851"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-pdf-fr-fr-99a170"})
ON CREATE SET
  kw.value = "qr code generator pdf",
  kw.volume = 60,
  kw.difficulty = 61,
  kw.cpc = 0.35,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-pdf-fr-fr-99a170"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-create-fr-fr-608098"})
ON CREATE SET
  kw.value = "qr code create",
  kw.volume = 50,
  kw.difficulty = 91,
  kw.cpc = 0.0,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-create-fr-fr-608098"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-avec-un-lien-fr-fr-a9fd1a"})
ON CREATE SET
  kw.value = "générer un qr code avec un lien",
  kw.volume = 50,
  kw.difficulty = 21,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-avec-un-lien-fr-fr-a9fd1a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-createur-fr-fr-9cbd09"})
ON CREATE SET
  kw.value = "qr code createur",
  kw.volume = 50,
  kw.difficulty = 51,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-createur-fr-fr-9cbd09"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generer-code-qr-fr-fr-16e987"})
ON CREATE SET
  kw.value = "generer code qr",
  kw.volume = 50,
  kw.difficulty = 31,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generer-code-qr-fr-fr-16e987"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creer-qr-code-lien-internet-fr-fr-c25e37"})
ON CREATE SET
  kw.value = "creer qr code lien internet",
  kw.volume = 50,
  kw.difficulty = 21,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creer-qr-code-lien-internet-fr-fr-c25e37"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-avis-google-fr-fr-bf1eb8"})
ON CREATE SET
  kw.value = "créer un qr code avis google",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-avis-google-fr-fr-bf1eb8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-pour-un-pdf-fr-fr-78fe85"})
ON CREATE SET
  kw.value = "générer un qr code pour un pdf",
  kw.volume = 50,
  kw.difficulty = 12,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-pour-un-pdf-fr-fr-78fe85"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-wifi-fr-fr-4d2aa3"})
ON CREATE SET
  kw.value = "qr code generator wifi",
  kw.volume = 50,
  kw.difficulty = 21,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-wifi-fr-fr-4d2aa3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-un-qr-code-google-form-fr-fr-8c8109"})
ON CREATE SET
  kw.value = "générer un qr code google form",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-un-qr-code-google-form-fr-fr-8c8109"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-un-qr-code-avis-google-gratuit-fr-fr-613b96"})
ON CREATE SET
  kw.value = "créer un qr code avis google gratuit",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-un-qr-code-avis-google-gratuit-fr-fr-613b96"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-facebook-fr-fr-207d8c"})
ON CREATE SET
  kw.value = "générer qr code facebook",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-facebook-fr-fr-207d8c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-fabriquer-qr-code-fr-fr-c7860d"})
ON CREATE SET
  kw.value = "fabriquer qr code",
  kw.volume = 50,
  kw.difficulty = 33,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-fabriquer-qr-code-fr-fr-c7860d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-wifi-qr-code-generator-fr-fr-66fc86"})
ON CREATE SET
  kw.value = "wifi qr code generator",
  kw.volume = 50,
  kw.difficulty = 22,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-wifi-qr-code-generator-fr-fr-66fc86"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-best-qr-code-generator-fr-fr-5baee4"})
ON CREATE SET
  kw.value = "best qr code generator",
  kw.volume = 50,
  kw.difficulty = 82,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-best-qr-code-generator-fr-fr-5baee4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generator-fr-fr-9456f2"})
ON CREATE SET
  kw.value = "qr-code generator",
  kw.volume = 50,
  kw.difficulty = 93,
  kw.cpc = 0.0,
  kw.intent = "Navigational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generator-fr-fr-9456f2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générer-qr-code-wifi-gratuit-fr-fr-8b2cd4"})
ON CREATE SET
  kw.value = "générer qr code wifi gratuit",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générer-qr-code-wifi-gratuit-fr-fr-8b2cd4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-wifi-fr-fr-ea9f67"})
ON CREATE SET
  kw.value = "créer qr code wifi",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-wifi-fr-fr-ea9f67"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code--fr-fr-38eb04"})
ON CREATE SET
  kw.value = "comment créer un qr code ?",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-créer-un-qr-code--fr-fr-38eb04"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-create-qr-code-from-link-fr-fr-5f9c8e"})
ON CREATE SET
  kw.value = "create qr code from link",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.45,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-create-qr-code-from-link-fr-fr-5f9c8e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créer-qr-code-page-facebook-gratuit-fr-fr-8e02f0"})
ON CREATE SET
  kw.value = "créer qr code page facebook gratuit",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "create-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créer-qr-code-page-facebook-gratuit-fr-fr-8e02f0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-personnalisé-fr-fr-ce3476"})
ON CREATE SET
  kw.value = "qr code personnalisé",
  kw.volume = 350,
  kw.difficulty = 36,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "custom-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-personnalisé-fr-fr-ce3476"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-qr-code-personnalisé-fr-fr-3024f7"})
ON CREATE SET
  kw.value = "carte de visite qr code personnalisé",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "custom-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-qr-code-personnalisé-fr-fr-3024f7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-custom-qr-code-fr-fr-d741ac"})
ON CREATE SET
  kw.value = "custom qr code",
  kw.volume = 80,
  kw.difficulty = 35,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "custom-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-custom-qr-code-fr-fr-d741ac"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-t-shirt-qr-code-personnalisé-fr-fr-cd722a"})
ON CREATE SET
  kw.value = "t shirt qr code personnalisé",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "custom-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-t-shirt-qr-code-personnalisé-fr-fr-cd722a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-datamatrix-vs-qr-code-fr-fr-853d3c"})
ON CREATE SET
  kw.value = "datamatrix vs qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "data-matrix", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-datamatrix-vs-qr-code-fr-fr-853d3c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-data-matrix-vs-qr-code-fr-fr-f49e05"})
ON CREATE SET
  kw.value = "data matrix vs qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "data-matrix", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-data-matrix-vs-qr-code-fr-fr-f49e05"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-qr-code-fr-fr-2a20b5"})
ON CREATE SET
  kw.value = "télécharger qr code",
  kw.volume = 450,
  kw.difficulty = 6,
  kw.cpc = 0.2,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "download-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-qr-code-fr-fr-2a20b5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-télécharger-un-qr-code-fr-fr-95acb9"})
ON CREATE SET
  kw.value = "comment télécharger un qr code",
  kw.volume = 200,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "download-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-télécharger-un-qr-code-fr-fr-95acb9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-un-qr-code-fr-fr-4c61a7"})
ON CREATE SET
  kw.value = "télécharger un qr code",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "download-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-un-qr-code-fr-fr-4c61a7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-application-qr-code-fr-fr-6c5808"})
ON CREATE SET
  kw.value = "télécharger application qr code",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "download-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-application-qr-code-fr-fr-6c5808"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-le-qr-code-fr-fr-ad1b39"})
ON CREATE SET
  kw.value = "télécharger le qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "download-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-le-qr-code-fr-fr-ad1b39"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-lapplication-qr-code-fr-fr-79c9a1"})
ON CREATE SET
  kw.value = "télécharger l\'application qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "download-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-lapplication-qr-code-fr-fr-79c9a1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-dynamique-fr-fr-c22719"})
ON CREATE SET
  kw.value = "qr code dynamique",
  kw.volume = 400,
  kw.difficulty = 3,
  kw.cpc = 1.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "dynamic-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-dynamique-fr-fr-c22719"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-dynamique-gratuit-fr-fr-c90e1c"})
ON CREATE SET
  kw.value = "qr code dynamique gratuit",
  kw.volume = 200,
  kw.difficulty = 7,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "dynamic-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-dynamique-gratuit-fr-fr-c90e1c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-dynamic-qr-code-fr-fr-e8edbb"})
ON CREATE SET
  kw.value = "dynamic qr code",
  kw.volume = 100,
  kw.difficulty = 2,
  kw.cpc = 1.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "dynamic-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-dynamic-qr-code-fr-fr-e8edbb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-facebook-fr-fr-9fa049"})
ON CREATE SET
  kw.value = "qr code facebook",
  kw.volume = 500,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "facebook", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-facebook-fr-fr-9fa049"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-page-facebook-fr-fr-96aa40"})
ON CREATE SET
  kw.value = "qr code page facebook",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "facebook", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-page-facebook-fr-fr-96aa40"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-facebook-gratuit-fr-fr-c9ea5b"})
ON CREATE SET
  kw.value = "qr code facebook gratuit",
  kw.volume = 100,
  kw.difficulty = 5,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "facebook", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-facebook-gratuit-fr-fr-c9ea5b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-facebook-ou-le-trouver-fr-fr-0ea906"})
ON CREATE SET
  kw.value = "qr code facebook ou le trouver",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "facebook", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-facebook-ou-le-trouver-fr-fr-0ea906"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-facebook-qr-code-fr-fr-5f29f4"})
ON CREATE SET
  kw.value = "facebook qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "facebook", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-facebook-qr-code-fr-fr-5f29f4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-facebook-page-fr-fr-898766"})
ON CREATE SET
  kw.value = "qr code facebook page",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "facebook", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-facebook-page-fr-fr-898766"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-instagram-fr-fr-0f58b4"})
ON CREATE SET
  kw.value = "qr code instagram",
  kw.volume = 900,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-instagram-fr-fr-0f58b4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-instagram-qr-code-fr-fr-2f8cf2"})
ON CREATE SET
  kw.value = "instagram qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-instagram-qr-code-fr-fr-2f8cf2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-instagram-gratuit-fr-fr-c8bce6"})
ON CREATE SET
  kw.value = "qr code instagram gratuit",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-instagram-gratuit-fr-fr-c8bce6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-compte-instagram-fr-fr-0d6212"})
ON CREATE SET
  kw.value = "qr code compte instagram",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-compte-instagram-fr-fr-0d6212"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-je-ne-trouve-pas-mon-qr-code-instagram-fr-fr-796c51"})
ON CREATE SET
  kw.value = "je ne trouve pas mon qr code instagram",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-je-ne-trouve-pas-mon-qr-code-instagram-fr-fr-796c51"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ou-trouver-le-qr-code-instagram-fr-fr-753fdc"})
ON CREATE SET
  kw.value = "ou trouver le qr code instagram",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ou-trouver-le-qr-code-instagram-fr-fr-753fdc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ou-trouver-qr-code-instagram-fr-fr-483f23"})
ON CREATE SET
  kw.value = "ou trouver qr code instagram",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ou-trouver-qr-code-instagram-fr-fr-483f23"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-instagram-ou-trouver-fr-fr-ba7754"})
ON CREATE SET
  kw.value = "qr code instagram ou trouver",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-instagram-ou-trouver-fr-fr-ba7754"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-profil-instagram-fr-fr-c7113c"})
ON CREATE SET
  kw.value = "qr code profil instagram",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-profil-instagram-fr-fr-c7113c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-instagram-profil-fr-fr-50b0c6"})
ON CREATE SET
  kw.value = "qr code instagram profil",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "instagram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-instagram-profil-fr-fr-50b0c6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-linkedin-fr-fr-f18402"})
ON CREATE SET
  kw.value = "qr code linkedin",
  kw.volume = 2100,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "linkedin", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-linkedin-fr-fr-f18402"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-linkedin-qr-code-fr-fr-16cda9"})
ON CREATE SET
  kw.value = "linkedin qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "linkedin", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-linkedin-qr-code-fr-fr-16cda9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-linkedin-cv-fr-fr-8dbaca"})
ON CREATE SET
  kw.value = "qr code linkedin cv",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "linkedin", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-linkedin-cv-fr-fr-8dbaca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-profil-linkedin-fr-fr-698642"})
ON CREATE SET
  kw.value = "qr code profil linkedin",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "linkedin", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-profil-linkedin-fr-fr-698642"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-vistaprint-qr-code-fr-fr-b59cf8"})
ON CREATE SET
  kw.value = "vistaprint qr code",
  kw.volume = 350,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Commercial,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "print-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-vistaprint-qr-code-fr-fr-b59cf8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vistaprint-fr-fr-d16daf"})
ON CREATE SET
  kw.value = "qr code vistaprint",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Commercial,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "print-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vistaprint-fr-fr-d16daf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-vista-print-qr-code-fr-fr-84cdca"})
ON CREATE SET
  kw.value = "vista print qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "print-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-vista-print-qr-code-fr-fr-84cdca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-imprimer-étiquette-mondial-relay-avec-qr-code-fr-fr-d06667"})
ON CREATE SET
  kw.value = "imprimer étiquette mondial relay avec qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "print-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-imprimer-étiquette-mondial-relay-avec-qr-code-fr-fr-d06667"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vista-print-fr-fr-06f061"})
ON CREATE SET
  kw.value = "qr code vista print",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Commercial,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "print-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vista-print-fr-fr-06f061"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-imprimer-qr-code-fr-fr-8abe2b"})
ON CREATE SET
  kw.value = "imprimer qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "print-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-imprimer-qr-code-fr-fr-8abe2b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-imprimer-qr-code-autocollant-fr-fr-c4126f"})
ON CREATE SET
  kw.value = "imprimer qr code autocollant",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "print-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-imprimer-qr-code-autocollant-fr-fr-c4126f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-imprimer-un-qr-code-fr-fr-010d33"})
ON CREATE SET
  kw.value = "imprimer un qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "print-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-imprimer-un-qr-code-fr-fr-010d33"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-fr-fr-0b9d04"})
ON CREATE SET
  kw.value = "qr code",
  kw.volume = 115000,
  kw.difficulty = 47,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-fr-fr-0b9d04"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-fr-fr-9b5ed5"})
ON CREATE SET
  kw.value = "qr code gratuit",
  kw.volume = 32000,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-fr-fr-9b5ed5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-de-qr-code-fr-fr-16440a"})
ON CREATE SET
  kw.value = "générateur de qr code",
  kw.volume = 7000,
  kw.difficulty = 35,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-de-qr-code-fr-fr-16440a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-qr-code-fr-fr-0a6f1b"})
ON CREATE SET
  kw.value = "générateur qr code",
  kw.volume = 6100,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-qr-code-fr-fr-0a6f1b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-qr-code-fr-fr-08bed1"})
ON CREATE SET
  kw.value = "création qr code",
  kw.volume = 5600,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-qr-code-fr-fr-08bed1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generateur-qr-code-fr-fr-389171"})
ON CREATE SET
  kw.value = "generateur qr code",
  kw.volume = 5000,
  kw.difficulty = 30,
  kw.cpc = 0.25,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generateur-qr-code-fr-fr-389171"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-monkey-fr-fr-62e6f5"})
ON CREATE SET
  kw.value = "qr code monkey",
  kw.volume = 5000,
  kw.difficulty = 0,
  kw.cpc = 0.05,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-monkey-fr-fr-62e6f5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-illimité-fr-fr-4c287f"})
ON CREATE SET
  kw.value = "qr code gratuit illimité",
  kw.volume = 3100,
  kw.difficulty = 23,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-illimité-fr-fr-4c287f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generateur-de-qr-code-fr-fr-5391d0"})
ON CREATE SET
  kw.value = "generateur de qr code",
  kw.volume = 2500,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generateur-de-qr-code-fr-fr-5391d0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-qr-code-fr-fr-0a2b84"})
ON CREATE SET
  kw.value = "application qr code",
  kw.volume = 2400,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-qr-code-fr-fr-0a2b84"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creation-qr-code-fr-fr-d3da6c"})
ON CREATE SET
  kw.value = "creation qr code",
  kw.volume = 2400,
  kw.difficulty = 35,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creation-qr-code-fr-fr-d3da6c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-png-fr-fr-486e60"})
ON CREATE SET
  kw.value = "qr code png",
  kw.volume = 2200,
  kw.difficulty = 21,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-png-fr-fr-486e60"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-qr-code-gratuit-fr-fr-d35efa"})
ON CREATE SET
  kw.value = "création qr code gratuit",
  kw.volume = 2200,
  kw.difficulty = 20,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-qr-code-gratuit-fr-fr-d35efa"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faire-un-qr-code-fr-fr-b064cb"})
ON CREATE SET
  kw.value = "faire un qr code",
  kw.volume = 2100,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faire-un-qr-code-fr-fr-b064cb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-de-qr-code-gratuit-fr-fr-8d26b5"})
ON CREATE SET
  kw.value = "générateur de qr code gratuit",
  kw.volume = 2000,
  kw.difficulty = 34,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-de-qr-code-gratuit-fr-fr-8d26b5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-adobe-qr-code-fr-fr-366af0"})
ON CREATE SET
  kw.value = "adobe qr code",
  kw.volume = 1900,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-adobe-qr-code-fr-fr-366af0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-monkey-qr-code-fr-fr-8ad93f"})
ON CREATE SET
  kw.value = "monkey qr code",
  kw.volume = 1700,
  kw.difficulty = 8,
  kw.cpc = 0.02,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-monkey-qr-code-fr-fr-8ad93f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-canva-fr-fr-059de7"})
ON CREATE SET
  kw.value = "qr code canva",
  kw.volume = 1700,
  kw.difficulty = 1,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-canva-fr-fr-059de7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-qr-code-gratuit-fr-fr-fbdc23"})
ON CREATE SET
  kw.value = "générateur qr code gratuit",
  kw.volume = 1400,
  kw.difficulty = 35,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-qr-code-gratuit-fr-fr-fbdc23"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecture-qr-code-fr-fr-ae42c0"})
ON CREATE SET
  kw.value = "lecture qr code",
  kw.volume = 1300,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecture-qr-code-fr-fr-ae42c0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-en-ligne-fr-fr-de1b39"})
ON CREATE SET
  kw.value = "qr code en ligne",
  kw.volume = 1300,
  kw.difficulty = 26,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-en-ligne-fr-fr-de1b39"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-canva-qr-code-fr-fr-358973"})
ON CREATE SET
  kw.value = "canva qr code",
  kw.volume = 1200,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-canva-qr-code-fr-fr-358973"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr-gratuit-fr-fr-3ae9ef"})
ON CREATE SET
  kw.value = "code qr gratuit",
  kw.volume = 1200,
  kw.difficulty = 26,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr-gratuit-fr-fr-3ae9ef"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-adobe-fr-fr-e97052"})
ON CREATE SET
  kw.value = "qr code adobe",
  kw.volume = 1200,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-adobe-fr-fr-e97052"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-qr-code-gratuit-fr-fr-6783c5"})
ON CREATE SET
  kw.value = "application qr code gratuit",
  kw.volume = 1200,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-qr-code-gratuit-fr-fr-6783c5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-faire-un-qr-code-fr-fr-8768c5"})
ON CREATE SET
  kw.value = "comment faire un qr code",
  kw.volume = 1200,
  kw.difficulty = 11,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-faire-un-qr-code-fr-fr-8768c5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-creator-fr-fr-917719"})
ON CREATE SET
  kw.value = "qr code creator",
  kw.volume = 1200,
  kw.difficulty = 52,
  kw.cpc = 0.25,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-creator-fr-fr-917719"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-avis-google-fr-fr-7d7f07"})
ON CREATE SET
  kw.value = "qr code avis google",
  kw.volume = 1100,
  kw.difficulty = 0,
  kw.cpc = 0.9,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-avis-google-fr-fr-7d7f07"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generate-qr-code-fr-fr-47d616"})
ON CREATE SET
  kw.value = "generate qr code",
  kw.volume = 1100,
  kw.difficulty = 91,
  kw.cpc = 0.25,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generate-qr-code-fr-fr-47d616"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-point-relais-colis-amazon-retour-qr-code-fr-fr-ebb076"})
ON CREATE SET
  kw.value = "point relais colis amazon retour qr code",
  kw.volume = 1100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-point-relais-colis-amazon-retour-qr-code-fr-fr-ebb076"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-de-qr-code-fr-fr-4513cc"})
ON CREATE SET
  kw.value = "création de qr code",
  kw.volume = 900,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-de-qr-code-fr-fr-4513cc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-animal-crossing-fr-fr-68e125"})
ON CREATE SET
  kw.value = "qr code animal crossing",
  kw.volume = 900,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-animal-crossing-fr-fr-68e125"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-brawl-star-fr-fr-137032"})
ON CREATE SET
  kw.value = "qr code brawl star",
  kw.volume = 900,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-brawl-star-fr-fr-137032"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-sans-inscription-fr-fr-06e081"})
ON CREATE SET
  kw.value = "qr code gratuit sans inscription",
  kw.volume = 800,
  kw.difficulty = 22,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-sans-inscription-fr-fr-06e081"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-appli-qr-code-fr-fr-f500c0"})
ON CREATE SET
  kw.value = "appli qr code",
  kw.volume = 700,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-appli-qr-code-fr-fr-f500c0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-photo-mariage-fr-fr-11ae4b"})
ON CREATE SET
  kw.value = "qr code photo mariage",
  kw.volume = 700,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-photo-mariage-fr-fr-11ae4b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-gratuit-fr-fr-6726a7"})
ON CREATE SET
  kw.value = "qr gratuit©",
  kw.volume = 700,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-gratuit-fr-fr-6726a7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-transformer-un-lien-en-qr-code-fr-fr-f09e6e"})
ON CREATE SET
  kw.value = "transformer un lien en qr code",
  kw.volume = 700,
  kw.difficulty = 29,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-transformer-un-lien-en-qr-code-fr-fr-f09e6e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-https-messagesgooglecomweb-qr-code-fr-fr-caa360"})
ON CREATE SET
  kw.value = "https //messages.google.com/web qr code",
  kw.volume = 700,
  kw.difficulty = 24,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-https-messagesgooglecomweb-qr-code-fr-fr-caa360"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-google-fr-fr-8e0fb1"})
ON CREATE SET
  kw.value = "qr code google",
  kw.volume = 700,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-google-fr-fr-8e0fb1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-sans-abonnement-fr-fr-1383bd"})
ON CREATE SET
  kw.value = "qr code gratuit sans abonnement",
  kw.volume = 600,
  kw.difficulty = 13,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-sans-abonnement-fr-fr-1383bd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-fr-fr-aece4d"})
ON CREATE SET
  kw.value = "qr code yo kai watch 2",
  kw.volume = 600,
  kw.difficulty = 0,
  kw.cpc = 0.05,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-fr-fr-aece4d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-dbl-fr-fr-cedcd1"})
ON CREATE SET
  kw.value = "qr code dbl",
  kw.volume = 600,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-dbl-fr-fr-cedcd1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faux-qr-code-fr-fr-bfdad1"})
ON CREATE SET
  kw.value = "faux qr code",
  kw.volume = 600,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faux-qr-code-fr-fr-bfdad1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créateur-de-qr-code-fr-fr-a3a22a"})
ON CREATE SET
  kw.value = "créateur de qr code",
  kw.volume = 600,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créateur-de-qr-code-fr-fr-a3a22a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-link-to-qr-code-fr-fr-fac78d"})
ON CREATE SET
  kw.value = "link to qr code",
  kw.volume = 500,
  kw.difficulty = 91,
  kw.cpc = 0.15,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-link-to-qr-code-fr-fr-fac78d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-la-digitale-qr-code-fr-fr-020304"})
ON CREATE SET
  kw.value = "la digitale qr code",
  kw.volume = 500,
  kw.difficulty = 3,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-la-digitale-qr-code-fr-fr-020304"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-vérifier-ticket-pmu-qr-code-fr-fr-207439"})
ON CREATE SET
  kw.value = "vérifier ticket pmu qr code",
  kw.volume = 500,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-vérifier-ticket-pmu-qr-code-fr-fr-207439"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-dbl-qr-code-fr-fr-51f0fd"})
ON CREATE SET
  kw.value = "dbl qr code",
  kw.volume = 500,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-dbl-qr-code-fr-fr-51f0fd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-3-fr-fr-3373c3"})
ON CREATE SET
  kw.value = "qr code yo kai watch 3",
  kw.volume = 500,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-3-fr-fr-3373c3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-qr-code-gratuit-fr-fr-26c519"})
ON CREATE SET
  kw.value = "installer qr code gratuit",
  kw.volume = 500,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-qr-code-gratuit-fr-fr-26c519"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lien-en-qr-code-fr-fr-f76225"})
ON CREATE SET
  kw.value = "lien en qr code",
  kw.volume = 500,
  kw.difficulty = 22,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lien-en-qr-code-fr-fr-f76225"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generateur-qr-code-gratuit-fr-fr-1490cd"})
ON CREATE SET
  kw.value = "generateur qr code gratuit",
  kw.volume = 500,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generateur-qr-code-gratuit-fr-fr-1490cd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faire-un-qr-code-gratuit-fr-fr-935fcd"})
ON CREATE SET
  kw.value = "faire un qr code gratuit",
  kw.volume = 500,
  kw.difficulty = 36,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faire-un-qr-code-gratuit-fr-fr-935fcd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-enter-your-qr-code-content-fr-fr-dfbd20"})
ON CREATE SET
  kw.value = "enter your qr code content",
  kw.volume = 450,
  kw.difficulty = 93,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-enter-your-qr-code-content-fr-fr-dfbd20"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pmu-gratuit-fr-fr-d3f229"})
ON CREATE SET
  kw.value = "qr code pmu gratuit",
  kw.volume = 450,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pmu-gratuit-fr-fr-d3f229"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-qr-code-gratuit-pour-android-fr-fr-b3d3e5"})
ON CREATE SET
  kw.value = "télécharger qr code gratuit pour android",
  kw.volume = 450,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-qr-code-gratuit-pour-android-fr-fr-b3d3e5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-google-lens-qr-code-fr-fr-d03077"})
ON CREATE SET
  kw.value = "google lens qr code",
  kw.volume = 450,
  kw.difficulty = 11,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-google-lens-qr-code-fr-fr-d03077"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-blanc-fr-fr-0f0126"})
ON CREATE SET
  kw.value = "qr code blanc",
  kw.volume = 450,
  kw.difficulty = 50,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-blanc-fr-fr-0f0126"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-générator-fr-fr-52ba34"})
ON CREATE SET
  kw.value = "qr code générator",
  kw.volume = 400,
  kw.difficulty = 50,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-générator-fr-fr-52ba34"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-appli-qr-code-gratuit-fr-fr-cf077a"})
ON CREATE SET
  kw.value = "appli qr code gratuit",
  kw.volume = 400,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-appli-qr-code-gratuit-fr-fr-cf077a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-exemple-qr-code-fr-fr-709620"})
ON CREATE SET
  kw.value = "exemple qr code",
  kw.volume = 400,
  kw.difficulty = 6,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-exemple-qr-code-fr-fr-709620"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-outlook-fr-fr-b65194"})
ON CREATE SET
  kw.value = "qr code outlook",
  kw.volume = 400,
  kw.difficulty = 0,
  kw.cpc = 0.04,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-outlook-fr-fr-b65194"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-générateur-fr-fr-bb4c9b"})
ON CREATE SET
  kw.value = "qr code générateur",
  kw.volume = 400,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-générateur-fr-fr-bb4c9b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-me-qr-code-fr-fr-708348"})
ON CREATE SET
  kw.value = "me qr code",
  kw.volume = 400,
  kw.difficulty = 3,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-me-qr-code-fr-fr-708348"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generateur-fr-fr-9470c6"})
ON CREATE SET
  kw.value = "qr code generateur",
  kw.volume = 400,
  kw.difficulty = 91,
  kw.cpc = 0.25,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generateur-fr-fr-9470c6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-adobe-express-qr-code-fr-fr-c3fed9"})
ON CREATE SET
  kw.value = "adobe express qr code",
  kw.volume = 400,
  kw.difficulty = 0,
  kw.cpc = 0.45,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-adobe-express-qr-code-fr-fr-c3fed9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-dun-qr-code-fr-fr-609236"})
ON CREATE SET
  kw.value = "création d\'un qr code",
  kw.volume = 400,
  kw.difficulty = 21,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-dun-qr-code-fr-fr-609236"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-https-tf1frtvpairing-qr-code-fr-fr-e2e466"})
ON CREATE SET
  kw.value = "https //tf1.fr/tv/pairing qr code",
  kw.volume = 400,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-https-tf1frtvpairing-qr-code-fr-fr-e2e466"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-qr-code-fr-fr-2b507b"})
ON CREATE SET
  kw.value = "installer qr code",
  kw.volume = 400,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-qr-code-fr-fr-2b507b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-utiliser-un-qr-code-fr-fr-2a98b6"})
ON CREATE SET
  kw.value = "comment utiliser un qr code",
  kw.volume = 400,
  kw.difficulty = 3,
  kw.cpc = 0.03,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-utiliser-un-qr-code-fr-fr-2a98b6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-déchetterie-fr-fr-83090e"})
ON CREATE SET
  kw.value = "qr code déchetterie",
  kw.volume = 400,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-déchetterie-fr-fr-83090e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-génération-qr-code-fr-fr-6c0575"})
ON CREATE SET
  kw.value = "génération qr code",
  kw.volume = 400,
  kw.difficulty = 23,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-génération-qr-code-fr-fr-6c0575"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-image-qr-code-fr-fr-878bd6"})
ON CREATE SET
  kw.value = "image qr code",
  kw.volume = 400,
  kw.difficulty = 16,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-image-qr-code-fr-fr-878bd6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-samsung-fr-fr-084056"})
ON CREATE SET
  kw.value = "qr code samsung",
  kw.volume = 400,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-samsung-fr-fr-084056"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mii-tomodachi-life-fr-fr-001a0c"})
ON CREATE SET
  kw.value = "qr code mii tomodachi life",
  kw.volume = 350,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mii-tomodachi-life-fr-fr-001a0c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faire-qr-code-fr-fr-da0f4e"})
ON CREATE SET
  kw.value = "faire qr code",
  kw.volume = 350,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faire-qr-code-fr-fr-da0f4e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-fdj-fr-fr-824fc4"})
ON CREATE SET
  kw.value = "qr code fdj",
  kw.volume = 350,
  kw.difficulty = 30,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-fdj-fr-fr-824fc4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-flash-qr-code-fr-fr-46f957"})
ON CREATE SET
  kw.value = "flash qr code",
  kw.volume = 350,
  kw.difficulty = 4,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-flash-qr-code-fr-fr-46f957"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faire-un-qr-code-avec-un-lien-fr-fr-64e937"})
ON CREATE SET
  kw.value = "faire un qr code avec un lien",
  kw.volume = 350,
  kw.difficulty = 7,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faire-un-qr-code-avec-un-lien-fr-fr-64e937"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-free-fr-fr-5750c4"})
ON CREATE SET
  kw.value = "qr code free",
  kw.volume = 350,
  kw.difficulty = 49,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-free-fr-fr-5750c4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-test-fr-fr-f3d6ad"})
ON CREATE SET
  kw.value = "qr code test",
  kw.volume = 350,
  kw.difficulty = 4,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-test-fr-fr-f3d6ad"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-qr-code-gratuite-fr-fr-83a5a8"})
ON CREATE SET
  kw.value = "application qr code gratuite",
  kw.volume = 350,
  kw.difficulty = 7,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-qr-code-gratuite-fr-fr-83a5a8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-io-fr-fr-abc292"})
ON CREATE SET
  kw.value = "qr code io",
  kw.volume = 350,
  kw.difficulty = 36,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-io-fr-fr-abc292"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-exemple-fr-fr-d5fd05"})
ON CREATE SET
  kw.value = "qr code exemple",
  kw.volume = 350,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-exemple-fr-fr-d5fd05"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-transformer-lien-en-qr-code-fr-fr-8fa800"})
ON CREATE SET
  kw.value = "transformer lien en qr code",
  kw.volume = 350,
  kw.difficulty = 21,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-transformer-lien-en-qr-code-fr-fr-8fa800"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-clash-royal-fr-fr-510ea6"})
ON CREATE SET
  kw.value = "qr code clash royal",
  kw.volume = 350,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-clash-royal-fr-fr-510ea6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-image-fr-fr-11136a"})
ON CREATE SET
  kw.value = "qr code image",
  kw.volume = 350,
  kw.difficulty = 11,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-image-fr-fr-11136a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-android-fr-fr-a87f34"})
ON CREATE SET
  kw.value = "qr code android",
  kw.volume = 350,
  kw.difficulty = 3,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-android-fr-fr-a87f34"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-parions-sport-qr-code-fr-fr-2f6dbf"})
ON CREATE SET
  kw.value = "parions sport qr code",
  kw.volume = 350,
  kw.difficulty = 9,
  kw.cpc = 0.8,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-parions-sport-qr-code-fr-fr-2f6dbf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-moco-fr-fr-2b6b2b"})
ON CREATE SET
  kw.value = "qr code mo.co",
  kw.volume = 350,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-moco-fr-fr-2b6b2b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-chimp-fr-fr-ad42f4"})
ON CREATE SET
  kw.value = "qr code chimp",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-chimp-fr-fr-ad42f4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-vérifier-euromillion-qr-code-fr-fr-87a433"})
ON CREATE SET
  kw.value = "vérifier euromillion qr code",
  kw.volume = 300,
  kw.difficulty = 49,
  kw.cpc = 0.8,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-vérifier-euromillion-qr-code-fr-fr-87a433"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sushi-fr-fr-95f2ac"})
ON CREATE SET
  kw.value = "qr code sushi",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sushi-fr-fr-95f2ac"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-google-qr-code-fr-fr-71d9d2"})
ON CREATE SET
  kw.value = "google qr code",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-google-qr-code-fr-fr-71d9d2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-qr-code-gratuit-fr-fr-ed1f6f"})
ON CREATE SET
  kw.value = "télécharger qr code gratuit",
  kw.volume = 300,
  kw.difficulty = 7,
  kw.cpc = 0.15,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-qr-code-gratuit-fr-fr-ed1f6f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-crée-un-qr-code-fr-fr-ad7ae6"})
ON CREATE SET
  kw.value = "crée un qr code",
  kw.volume = 300,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-crée-un-qr-code-fr-fr-ad7ae6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creation-de-qr-code-fr-fr-846308"})
ON CREATE SET
  kw.value = "creation de qr code",
  kw.volume = 300,
  kw.difficulty = 34,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creation-de-qr-code-fr-fr-846308"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-déchetterie-amiens-fr-fr-d787fa"})
ON CREATE SET
  kw.value = "qr code déchetterie amiens",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-déchetterie-amiens-fr-fr-d787fa"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mariage-fr-fr-7898e6"})
ON CREATE SET
  kw.value = "qr code mariage",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mariage-fr-fr-7898e6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-le-grand-quiz-du-qi-tf1-qr-code-fr-fr-a89df5"})
ON CREATE SET
  kw.value = "le grand quiz du qi tf1 qr code",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-le-grand-quiz-du-qi-tf1-qr-code-fr-fr-a89df5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-fr-fr-dc6516"})
ON CREATE SET
  kw.value = "qr code yo kai watch",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-fr-fr-dc6516"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-bs-fr-fr-7fb2be"})
ON CREATE SET
  kw.value = "qr code bs",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.09,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-bs-fr-fr-7fb2be"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generateur-de-qr-code-gratuit-fr-fr-d91bb4"})
ON CREATE SET
  kw.value = "generateur de qr code gratuit",
  kw.volume = 300,
  kw.difficulty = 35,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generateur-de-qr-code-gratuit-fr-fr-d91bb4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creation-qr-code-gratuit-fr-fr-249451"})
ON CREATE SET
  kw.value = "creation qr code gratuit",
  kw.volume = 300,
  kw.difficulty = 35,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creation-qr-code-gratuit-fr-fr-249451"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-fdj-qr-code-fr-fr-ad0199"})
ON CREATE SET
  kw.value = "fdj qr code",
  kw.volume = 300,
  kw.difficulty = 26,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-fdj-qr-code-fr-fr-ad0199"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-yo-kai-watch-qr-code-fr-fr-586c35"})
ON CREATE SET
  kw.value = "yo kai watch qr code",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-yo-kai-watch-qr-code-fr-fr-586c35"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-photo-qr-code-fr-fr-fade45"})
ON CREATE SET
  kw.value = "photo qr code",
  kw.volume = 250,
  kw.difficulty = 6,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-photo-qr-code-fr-fr-fade45"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-online-fr-fr-5d856b"})
ON CREATE SET
  kw.value = "qr code online",
  kw.volume = 250,
  kw.difficulty = 60,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-online-fr-fr-5d856b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-en-ligne-fr-fr-534393"})
ON CREATE SET
  kw.value = "qr code gratuit en ligne",
  kw.volume = 250,
  kw.difficulty = 22,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-en-ligne-fr-fr-534393"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-free-qr-code-fr-fr-c543eb"})
ON CREATE SET
  kw.value = "free qr code",
  kw.volume = 250,
  kw.difficulty = 37,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-free-qr-code-fr-fr-c543eb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-de-qr-code-gratuit-fr-fr-9c0c76"})
ON CREATE SET
  kw.value = "création de qr code gratuit",
  kw.volume = 250,
  kw.difficulty = 35,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-de-qr-code-gratuit-fr-fr-9c0c76"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-adidas-qr-code-vérification-fr-fr-2404b9"})
ON CREATE SET
  kw.value = "adidas qr code vérification",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-adidas-qr-code-vérification-fr-fr-2404b9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-signification-fr-fr-780877"})
ON CREATE SET
  kw.value = "qr code signification",
  kw.volume = 250,
  kw.difficulty = 3,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-signification-fr-fr-780877"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-promo-burger-king-fr-fr-1d3ae1"})
ON CREATE SET
  kw.value = "qr code promo burger king",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Commercial,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-promo-burger-king-fr-fr-1d3ae1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mariage-photo-fr-fr-f19beb"})
ON CREATE SET
  kw.value = "qr code mariage photo",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mariage-photo-fr-fr-f19beb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-que-veut-dire-qr-code-fr-fr-3c9c95"})
ON CREATE SET
  kw.value = "que veut dire qr code",
  kw.volume = 250,
  kw.difficulty = 6,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-que-veut-dire-qr-code-fr-fr-3c9c95"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-genrator-fr-fr-7d9574"})
ON CREATE SET
  kw.value = "qr code genrator",
  kw.volume = 250,
  kw.difficulty = 93,
  kw.cpc = 0.0,
  kw.intent = "Navigational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-genrator-fr-fr-7d9574"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-fdj-qr-code-fr-fr-045158"})
ON CREATE SET
  kw.value = "application fdj qr code",
  kw.volume = 250,
  kw.difficulty = 24,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-fdj-qr-code-fr-fr-045158"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-application-fr-fr-9777ad"})
ON CREATE SET
  kw.value = "qr code application",
  kw.volume = 250,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-application-fr-fr-9777ad"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vin-fr-fr-95c653"})
ON CREATE SET
  kw.value = "qr code vin",
  kw.volume = 250,
  kw.difficulty = 1,
  kw.cpc = 0.9,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vin-fr-fr-95c653"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-de-qr-code-gratuit-et-illimité-fr-fr-ca60f5"})
ON CREATE SET
  kw.value = "générateur de qr code gratuit et illimité",
  kw.volume = 250,
  kw.difficulty = 38,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-de-qr-code-gratuit-et-illimité-fr-fr-ca60f5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-lidl-plus-fr-fr-55e9f4"})
ON CREATE SET
  kw.value = "qr code lidl plus",
  kw.volume = 250,
  kw.difficulty = 5,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-lidl-plus-fr-fr-55e9f4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-yo-kai-watch-3-qr-code-fr-fr-6b7fa7"})
ON CREATE SET
  kw.value = "yo kai watch 3 qr code",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-yo-kai-watch-3-qr-code-fr-fr-6b7fa7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-https-tf1frtv-qr-code-fr-fr-934b31"})
ON CREATE SET
  kw.value = "https //tf1.fr/tv qr code",
  kw.volume = 250,
  kw.difficulty = 12,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-https-tf1frtv-qr-code-fr-fr-934b31"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-iphone-fr-fr-a2e650"})
ON CREATE SET
  kw.value = "qr code iphone",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.08,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-iphone-fr-fr-a2e650"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-clash-royal-qr-code-fr-fr-601788"})
ON CREATE SET
  kw.value = "clash royal qr code",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-clash-royal-qr-code-fr-fr-601788"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-affiche-qr-code-fr-fr-d73928"})
ON CREATE SET
  kw.value = "affiche qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-affiche-qr-code-fr-fr-d73928"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-db-legends-fr-fr-d8f1a1"})
ON CREATE SET
  kw.value = "qr code db legends",
  kw.volume = 200,
  kw.difficulty = 3,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-db-legends-fr-fr-d8f1a1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generation-qr-code-fr-fr-a0e976"})
ON CREATE SET
  kw.value = "generation qr code",
  kw.volume = 200,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generation-qr-code-fr-fr-a0e976"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-loto-fr-fr-eac989"})
ON CREATE SET
  kw.value = "qr code loto",
  kw.volume = 200,
  kw.difficulty = 19,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-loto-fr-fr-eac989"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-rond-fr-fr-bfa7bd"})
ON CREATE SET
  kw.value = "qr code rond",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-rond-fr-fr-bfa7bd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-esim-free-fr-fr-425f76"})
ON CREATE SET
  kw.value = "qr code esim free",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-esim-free-fr-fr-425f76"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-en-ligne-gratuit-fr-fr-1e1c45"})
ON CREATE SET
  kw.value = "qr code en ligne gratuit",
  kw.volume = 200,
  kw.difficulty = 25,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-en-ligne-gratuit-fr-fr-1e1c45"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créateur-qr-code-fr-fr-8b69b0"})
ON CREATE SET
  kw.value = "créateur qr code",
  kw.volume = 200,
  kw.difficulty = 50,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créateur-qr-code-fr-fr-8b69b0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-affiche-avec-qr-code-fr-fr-0b9e32"})
ON CREATE SET
  kw.value = "affiche avec qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-affiche-avec-qr-code-fr-fr-0b9e32"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-1-fr-fr-99722a"})
ON CREATE SET
  kw.value = "qr code yo kai watch 1",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-1-fr-fr-99722a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-musique-fr-fr-d22aa8"})
ON CREATE SET
  kw.value = "qr code musique",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-musique-fr-fr-d22aa8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-dragon-ball-legends-fr-fr-85764f"})
ON CREATE SET
  kw.value = "qr code dragon ball legends",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.02,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-dragon-ball-legends-fr-fr-85764f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-steam-fr-fr-7cbb81"})
ON CREATE SET
  kw.value = "qr code steam",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-steam-fr-fr-7cbb81"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-prendre-un-qr-code-fr-fr-3ddf47"})
ON CREATE SET
  kw.value = "comment prendre un qr code",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-prendre-un-qr-code-fr-fr-3ddf47"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-brawl-star-qr-code-fr-fr-30d20f"})
ON CREATE SET
  kw.value = "brawl star qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.03,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-brawl-star-qr-code-fr-fr-30d20f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-insta-fr-fr-4002d4"})
ON CREATE SET
  kw.value = "qr code insta",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.45,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-insta-fr-fr-4002d4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-lidl-fr-fr-acc240"})
ON CREATE SET
  kw.value = "qr code lidl",
  kw.volume = 200,
  kw.difficulty = 5,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-lidl-fr-fr-acc240"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-blanc-png-fr-fr-dc679b"})
ON CREATE SET
  kw.value = "qr code blanc png",
  kw.volume = 200,
  kw.difficulty = 5,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-blanc-png-fr-fr-dc679b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-où-se-trouve-le-qr-code-sur-samsung-fr-fr-325795"})
ON CREATE SET
  kw.value = "où se trouve le qr code sur samsung",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-où-se-trouve-le-qr-code-sur-samsung-fr-fr-325795"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ou-trouver-le-qr-code-dun-telephone-samsung-fr-fr-4a46cc"})
ON CREATE SET
  kw.value = "ou trouver le qr code d\'un telephone samsung",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ou-trouver-le-qr-code-dun-telephone-samsung-fr-fr-4a46cc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-crer-un-qr-code-fr-fr-902645"})
ON CREATE SET
  kw.value = "crer un qr code",
  kw.volume = 200,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-crer-un-qr-code-fr-fr-902645"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-blaster-fr-fr-f808d1"})
ON CREATE SET
  kw.value = "qr code yo kai watch blaster",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-blaster-fr-fr-f808d1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-mon-profil-aftral-qr-code-fr-fr-2b419d"})
ON CREATE SET
  kw.value = "mon profil aftral qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-mon-profil-aftral-qr-code-fr-fr-2b419d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-comment-ça-marche-fr-fr-f6493f"})
ON CREATE SET
  kw.value = "qr code comment ça marche",
  kw.volume = 200,
  kw.difficulty = 7,
  kw.cpc = 0.02,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-comment-ça-marche-fr-fr-f6493f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-euromillions-fr-fr-3637a7"})
ON CREATE SET
  kw.value = "qr code euromillions",
  kw.volume = 200,
  kw.difficulty = 30,
  kw.cpc = 0.9,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-euromillions-fr-fr-3637a7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-imprimante-qr-code-fr-fr-244a55"})
ON CREATE SET
  kw.value = "imprimante qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-imprimante-qr-code-fr-fr-244a55"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-retour-box-bouygues-relais-colis-qr-code-fr-fr-dff132"})
ON CREATE SET
  kw.value = "retour box bouygues relais colis qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-retour-box-bouygues-relais-colis-qr-code-fr-fr-dff132"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-qr-code-gratuit-samsung-fr-fr-1e3c0a"})
ON CREATE SET
  kw.value = "installer qr code gratuit samsung",
  kw.volume = 200,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-qr-code-gratuit-samsung-fr-fr-1e3c0a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecture-qr-code-gratuit-fr-fr-17e3a0"})
ON CREATE SET
  kw.value = "lecture qr code gratuit",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecture-qr-code-gratuit-fr-fr-17e3a0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-creation-fr-fr-b0d4db"})
ON CREATE SET
  kw.value = "qr code creation",
  kw.volume = 200,
  kw.difficulty = 34,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-creation-fr-fr-b0d4db"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-convertisseur-qr-code-fr-fr-d1ad95"})
ON CREATE SET
  kw.value = "convertisseur qr code",
  kw.volume = 200,
  kw.difficulty = 34,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-convertisseur-qr-code-fr-fr-d1ad95"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-fonctionne-un-qr-code-fr-fr-697274"})
ON CREATE SET
  kw.value = "comment fonctionne un qr code",
  kw.volume = 200,
  kw.difficulty = 10,
  kw.cpc = 0.06,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-fonctionne-un-qr-code-fr-fr-697274"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-fake-qr-code-fr-fr-aa44e8"})
ON CREATE SET
  kw.value = "fake qr code",
  kw.volume = 200,
  kw.difficulty = 45,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-fake-qr-code-fr-fr-aa44e8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-tiger-fr-fr-187530"})
ON CREATE SET
  kw.value = "qr code tiger",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-tiger-fr-fr-187530"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-tombe-fr-fr-9172bf"})
ON CREATE SET
  kw.value = "qr code tombe",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-tombe-fr-fr-9172bf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-sticker-qr-code-fr-fr-749180"})
ON CREATE SET
  kw.value = "sticker qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-sticker-qr-code-fr-fr-749180"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-installer-le-qr-code-sur-samsung-fr-fr-c4c799"})
ON CREATE SET
  kw.value = "comment installer le qr code sur samsung",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-installer-le-qr-code-sur-samsung-fr-fr-c4c799"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-photo-mariage-gratuit-fr-fr-4b1e18"})
ON CREATE SET
  kw.value = "qr code photo mariage gratuit",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-photo-mariage-gratuit-fr-fr-4b1e18"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-retour-amazon-mondial-relay-qr-code-fr-fr-29849d"})
ON CREATE SET
  kw.value = "retour amazon mondial relay qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-retour-amazon-mondial-relay-qr-code-fr-fr-29849d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-google-form-fr-fr-df4c88"})
ON CREATE SET
  kw.value = "qr code google form",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-google-form-fr-fr-df4c88"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-moco-qr-code-fr-fr-ca804b"})
ON CREATE SET
  kw.value = "mo.co qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-moco-qr-code-fr-fr-ca804b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-yo-kai-watch-2-qr-code-fr-fr-7c2796"})
ON CREATE SET
  kw.value = "yo kai watch 2 qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-yo-kai-watch-2-qr-code-fr-fr-7c2796"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-visit-japan-web-qr-code-fr-fr-104262"})
ON CREATE SET
  kw.value = "visit japan web qr code",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.7,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-visit-japan-web-qr-code-fr-fr-104262"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-google-form-qr-code-fr-fr-d9731e"})
ON CREATE SET
  kw.value = "google form qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-google-form-qr-code-fr-fr-d9731e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-appareil-photo-qr-code-fr-fr-ce3a2d"})
ON CREATE SET
  kw.value = "appareil photo qr code",
  kw.volume = 200,
  kw.difficulty = 4,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-appareil-photo-qr-code-fr-fr-ce3a2d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-my-qr-code-fr-fr-30cf32"})
ON CREATE SET
  kw.value = "my qr code",
  kw.volume = 200,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-my-qr-code-fr-fr-30cf32"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-lien-fr-fr-e90807"})
ON CREATE SET
  kw.value = "qr code lien",
  kw.volume = 200,
  kw.difficulty = 28,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-lien-fr-fr-e90807"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-svg-fr-fr-98b052"})
ON CREATE SET
  kw.value = "qr code svg",
  kw.volume = 200,
  kw.difficulty = 24,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-svg-fr-fr-98b052"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecture-qr-code-en-ligne-fr-fr-c43bc5"})
ON CREATE SET
  kw.value = "lecture qr code en ligne",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecture-qr-code-en-ligne-fr-fr-c43bc5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-avis-google-qr-code-fr-fr-530954"})
ON CREATE SET
  kw.value = "avis google qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 1.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-avis-google-qr-code-fr-fr-530954"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-google-avis-fr-fr-f8f19d"})
ON CREATE SET
  kw.value = "qr code google avis",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 1.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-google-avis-fr-fr-f8f19d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lien-qr-code-fr-fr-57b02d"})
ON CREATE SET
  kw.value = "lien qr code",
  kw.volume = 200,
  kw.difficulty = 29,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lien-qr-code-fr-fr-57b02d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-générateur-gratuit-fr-fr-5108ea"})
ON CREATE SET
  kw.value = "qr code générateur gratuit",
  kw.volume = 150,
  kw.difficulty = 35,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-générateur-gratuit-fr-fr-5108ea"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-discord-fr-fr-766eed"})
ON CREATE SET
  kw.value = "qr code discord",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-discord-fr-fr-766eed"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-fitness-park-fr-fr-4c5ea5"})
ON CREATE SET
  kw.value = "qr code fitness park",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-fitness-park-fr-fr-4c5ea5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-bracelet-qr-code-fr-fr-98ed54"})
ON CREATE SET
  kw.value = "bracelet qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-bracelet-qr-code-fr-fr-98ed54"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-contact-fr-fr-04f5a1"})
ON CREATE SET
  kw.value = "qr code contact",
  kw.volume = 150,
  kw.difficulty = 6,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-contact-fr-fr-04f5a1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-moco-fr-fr-ab35f6"})
ON CREATE SET
  kw.value = "qr code moco",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-moco-fr-fr-ab35f6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-indesign-fr-fr-742049"})
ON CREATE SET
  kw.value = "qr code indesign",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-indesign-fr-fr-742049"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-zebulon-qr-code-fr-fr-390932"})
ON CREATE SET
  kw.value = "zebulon qr code",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-zebulon-qr-code-fr-fr-390932"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-entreprise-fr-fr-4c1da0"})
ON CREATE SET
  kw.value = "qr code entreprise",
  kw.volume = 150,
  kw.difficulty = 23,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-entreprise-fr-fr-4c1da0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-stickers-qr-code-fr-fr-7ef48b"})
ON CREATE SET
  kw.value = "stickers qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.45,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-stickers-qr-code-fr-fr-7ef48b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-payer-avec-un-qr-code-fr-fr-ac448c"})
ON CREATE SET
  kw.value = "payer avec un qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-payer-avec-un-qr-code-fr-fr-ac448c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-builder-fr-fr-5609d6"})
ON CREATE SET
  kw.value = "qr code builder",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-builder-fr-fr-5609d6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vidéo-gratuit-fr-fr-7643d0"})
ON CREATE SET
  kw.value = "qr code vidéo gratuit",
  kw.volume = 150,
  kw.difficulty = 8,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vidéo-gratuit-fr-fr-7643d0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-qr-code-gratuit-illimité-fr-fr-26124d"})
ON CREATE SET
  kw.value = "création qr code gratuit illimité",
  kw.volume = 150,
  kw.difficulty = 28,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-qr-code-gratuit-illimité-fr-fr-26124d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faire-qr-code-gratuit-fr-fr-a34af6"})
ON CREATE SET
  kw.value = "faire qr code gratuit",
  kw.volume = 150,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faire-qr-code-gratuit-fr-fr-a34af6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-test-qr-code-fr-fr-2ee965"})
ON CREATE SET
  kw.value = "test qr code",
  kw.volume = 150,
  kw.difficulty = 7,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-test-qr-code-fr-fr-2ee965"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-enregistrer-un-qr-code-fr-fr-6c3200"})
ON CREATE SET
  kw.value = "comment enregistrer un qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-enregistrer-un-qr-code-fr-fr-6c3200"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mii-fr-fr-9fe9fb"})
ON CREATE SET
  kw.value = "qr code mii",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mii-fr-fr-9fe9fb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-site-internet-fr-fr-037f34"})
ON CREATE SET
  kw.value = "qr code site internet",
  kw.volume = 150,
  kw.difficulty = 21,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-site-internet-fr-fr-037f34"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-lego-fr-fr-7043e7"})
ON CREATE SET
  kw.value = "qr code lego",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-lego-fr-fr-7043e7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-cree-un-qr-code-fr-fr-3561ed"})
ON CREATE SET
  kw.value = "comment cree un qr code",
  kw.volume = 150,
  kw.difficulty = 22,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-cree-un-qr-code-fr-fr-3561ed"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-avis-google-gratuit-fr-fr-73a711"})
ON CREATE SET
  kw.value = "qr code avis google gratuit",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-avis-google-gratuit-fr-fr-73a711"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-utiliser-qr-code-fr-fr-2fad8f"})
ON CREATE SET
  kw.value = "comment utiliser qr code",
  kw.volume = 150,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-utiliser-qr-code-fr-fr-2fad8f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-convertir-lien-en-qr-code-fr-fr-72e0de"})
ON CREATE SET
  kw.value = "convertir lien en qr code",
  kw.volume = 150,
  kw.difficulty = 6,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-convertir-lien-en-qr-code-fr-fr-72e0de"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-site-qr-code-fr-fr-159476"})
ON CREATE SET
  kw.value = "site qr code",
  kw.volume = 150,
  kw.difficulty = 40,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-site-qr-code-fr-fr-159476"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-appli-qr-code-gratuit-android-fr-fr-102b4a"})
ON CREATE SET
  kw.value = "appli qr code gratuit android",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-appli-qr-code-gratuit-android-fr-fr-102b4a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-rick-roll-fr-fr-db5bdc"})
ON CREATE SET
  kw.value = "qr code rick roll",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-rick-roll-fr-fr-db5bdc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-app-fr-fr-f66bc1"})
ON CREATE SET
  kw.value = "qr code app",
  kw.volume = 150,
  kw.difficulty = 4,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-app-fr-fr-f66bc1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-taille-minimale-qr-code-fr-fr-b7e96a"})
ON CREATE SET
  kw.value = "taille minimale qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 1.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-taille-minimale-qr-code-fr-fr-b7e96a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-photographier-un-qr-code-fr-fr-9388f0"})
ON CREATE SET
  kw.value = "comment photographier un qr code",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-photographier-un-qr-code-fr-fr-9388f0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gen-fr-fr-b8f8b9"})
ON CREATE SET
  kw.value = "qr code gen",
  kw.volume = 150,
  kw.difficulty = 35,
  kw.cpc = 0.35,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gen-fr-fr-b8f8b9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-que-signifie-qr-code-fr-fr-96d8bc"})
ON CREATE SET
  kw.value = "que signifie qr code",
  kw.volume = 150,
  kw.difficulty = 6,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-que-signifie-qr-code-fr-fr-96d8bc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-free-qr-code-trimrly-fr-fr-037425"})
ON CREATE SET
  kw.value = "free qr code trimrly",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-free-qr-code-trimrly-fr-fr-037425"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-payer-avec-un-qr-code-fr-fr-f971e5"})
ON CREATE SET
  kw.value = "comment payer avec un qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-payer-avec-un-qr-code-fr-fr-f971e5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pour-avis-google-fr-fr-a7956a"})
ON CREATE SET
  kw.value = "qr code pour avis google",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pour-avis-google-fr-fr-a7956a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-création-fr-fr-dc9921"})
ON CREATE SET
  kw.value = "qr code création",
  kw.volume = 150,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-création-fr-fr-dc9921"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-douchette-qr-code-fr-fr-71128e"})
ON CREATE SET
  kw.value = "douchette qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-douchette-qr-code-fr-fr-71128e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-support-qr-code-fr-fr-c48d1f"})
ON CREATE SET
  kw.value = "support qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.9,
  kw.intent = "Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-support-qr-code-fr-fr-c48d1f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-pour-qr-code-fr-fr-53637d"})
ON CREATE SET
  kw.value = "application pour qr code",
  kw.volume = 150,
  kw.difficulty = 7,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-pour-qr-code-fr-fr-53637d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-un-qr-code-fr-fr-f9c60b"})
ON CREATE SET
  kw.value = "un qr code",
  kw.volume = 150,
  kw.difficulty = 21,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-un-qr-code-fr-fr-f9c60b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ai-fr-fr-4919fe"})
ON CREATE SET
  kw.value = "qr code ai",
  kw.volume = 150,
  kw.difficulty = 55,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ai-fr-fr-4919fe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-facture-castorama-qr-code-fr-fr-46c07c"})
ON CREATE SET
  kw.value = "facture castorama qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-facture-castorama-qr-code-fr-fr-46c07c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-brawl-pass-gratuit-qr-code-fr-fr-077a72"})
ON CREATE SET
  kw.value = "brawl pass gratuit qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-brawl-pass-gratuit-qr-code-fr-fr-077a72"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-https-tf1-fr-tv-pairing-qr-code-fr-fr-bed25c"})
ON CREATE SET
  kw.value = "https tf1 fr tv pairing qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-https-tf1-fr-tv-pairing-qr-code-fr-fr-bed25c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generate-a-qr-code-fr-fr-13f450"})
ON CREATE SET
  kw.value = "generate a qr code",
  kw.volume = 150,
  kw.difficulty = 59,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generate-a-qr-code-fr-fr-13f450"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-exemple-de-qr-code-fr-fr-a54055"})
ON CREATE SET
  kw.value = "exemple de qr code",
  kw.volume = 150,
  kw.difficulty = 6,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-exemple-de-qr-code-fr-fr-a54055"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-payer-par-qr-code-fr-fr-29cee6"})
ON CREATE SET
  kw.value = "payer par qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-payer-par-qr-code-fr-fr-29cee6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pickup-ne-fonctionne-pas-fr-fr-71a8b7"})
ON CREATE SET
  kw.value = "qr code pickup ne fonctionne pas",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pickup-ne-fonctionne-pas-fr-fr-71a8b7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-taille-minimum-qr-code-fr-fr-071e89"})
ON CREATE SET
  kw.value = "taille minimum qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.09,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-taille-minimum-qr-code-fr-fr-071e89"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pour-photo-mariage-fr-fr-0e5fe8"})
ON CREATE SET
  kw.value = "qr code pour photo mariage",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pour-photo-mariage-fr-fr-0e5fe8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-hay-day-fr-fr-c8226c"})
ON CREATE SET
  kw.value = "qr code hay day",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-hay-day-fr-fr-c8226c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-qr-code-gratuit-illimité-fr-fr-4289f0"})
ON CREATE SET
  kw.value = "générateur qr code gratuit illimité",
  kw.volume = 150,
  kw.difficulty = 23,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-qr-code-gratuit-illimité-fr-fr-4289f0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-transformer-un-lien-en-qr-code-fr-fr-93a2ce"})
ON CREATE SET
  kw.value = "comment transformer un lien en qr code",
  kw.volume = 150,
  kw.difficulty = 6,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-transformer-un-lien-en-qr-code-fr-fr-93a2ce"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-animal-crossing-qr-code-fr-fr-e733f5"})
ON CREATE SET
  kw.value = "animal crossing qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-animal-crossing-qr-code-fr-fr-e733f5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-tatouage-qr-code-fr-fr-7168b6"})
ON CREATE SET
  kw.value = "tatouage qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.06,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-tatouage-qr-code-fr-fr-7168b6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-illimité-gratuit-fr-fr-5e746e"})
ON CREATE SET
  kw.value = "qr code illimité gratuit",
  kw.volume = 150,
  kw.difficulty = 23,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-illimité-gratuit-fr-fr-5e746e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-design-fr-fr-5cd70c"})
ON CREATE SET
  kw.value = "qr code design",
  kw.volume = 150,
  kw.difficulty = 35,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-design-fr-fr-5cd70c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-faire-un-qr-code-avec-un-lien-fr-fr-bdb442"})
ON CREATE SET
  kw.value = "comment faire un qr code avec un lien",
  kw.volume = 150,
  kw.difficulty = 7,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-faire-un-qr-code-avec-un-lien-fr-fr-bdb442"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-le-grand-quiz-tf1-qr-code-fr-fr-ce1eeb"})
ON CREATE SET
  kw.value = "le grand quiz tf1 qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-le-grand-quiz-tf1-qr-code-fr-fr-ce1eeb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-tomodachi-life-fr-fr-a93a62"})
ON CREATE SET
  kw.value = "qr code tomodachi life",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-tomodachi-life-fr-fr-a93a62"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mp3-fr-fr-a91614"})
ON CREATE SET
  kw.value = "qr code mp3",
  kw.volume = 150,
  kw.difficulty = 12,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mp3-fr-fr-a91614"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-beyblade-burst-fr-fr-2d17c2"})
ON CREATE SET
  kw.value = "qr code beyblade burst",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.05,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-beyblade-burst-fr-fr-2d17c2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-genere-qr-code-fr-fr-72e674"})
ON CREATE SET
  kw.value = "genere qr code",
  kw.volume = 150,
  kw.difficulty = 33,
  kw.cpc = 0.0,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-genere-qr-code-fr-fr-72e674"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ralph-lauren-fr-fr-bee05c"})
ON CREATE SET
  kw.value = "qr code ralph lauren",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ralph-lauren-fr-fr-bee05c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-basic-fit-fr-fr-437091"})
ON CREATE SET
  kw.value = "qr code basic fit",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-basic-fit-fr-fr-437091"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créateur-de-qr-code-gratuit-fr-fr-800306"})
ON CREATE SET
  kw.value = "créateur de qr code gratuit",
  kw.volume = 150,
  kw.difficulty = 35,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créateur-de-qr-code-gratuit-fr-fr-800306"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-jeu-aldi-qr-code-fr-fr-e20d0c"})
ON CREATE SET
  kw.value = "jeu aldi qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-jeu-aldi-qr-code-fr-fr-e20d0c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-dun-qr-code-gratuit-fr-fr-9aa517"})
ON CREATE SET
  kw.value = "création d\'un qr code gratuit",
  kw.volume = 150,
  kw.difficulty = 35,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-dun-qr-code-gratuit-fr-fr-9aa517"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-etiquette-qr-code-fr-fr-9f5d45"})
ON CREATE SET
  kw.value = "etiquette qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-etiquette-qr-code-fr-fr-9f5d45"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-où-se-trouve-le-qr-code-de-mon-téléphone-fr-fr-a94af9"})
ON CREATE SET
  kw.value = "où se trouve le qr code de mon téléphone",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-où-se-trouve-le-qr-code-de-mon-téléphone-fr-fr-a94af9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-prix-qr-code-fr-fr-f8ffaf"})
ON CREATE SET
  kw.value = "prix qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-prix-qr-code-fr-fr-f8ffaf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-make-a-qr-code-fr-fr-384823"})
ON CREATE SET
  kw.value = "make a qr code",
  kw.volume = 150,
  kw.difficulty = 86,
  kw.cpc = 0.45,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-make-a-qr-code-fr-fr-384823"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ou-trouver-le-qr-code-sur-iphone-fr-fr-b86ac7"})
ON CREATE SET
  kw.value = "ou trouver le qr code sur iphone",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ou-trouver-le-qr-code-sur-iphone-fr-fr-b86ac7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-plaque-qr-code-fr-fr-5dc650"})
ON CREATE SET
  kw.value = "plaque qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.9,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-plaque-qr-code-fr-fr-5dc650"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installation-qr-code-fr-fr-7f8fc5"})
ON CREATE SET
  kw.value = "installation qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installation-qr-code-fr-fr-7f8fc5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-revolut-fr-fr-e649bb"})
ON CREATE SET
  kw.value = "qr code revolut",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-revolut-fr-fr-e649bb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-codeio-fr-fr-adf17a"})
ON CREATE SET
  kw.value = "qr code.io",
  kw.volume = 150,
  kw.difficulty = 37,
  kw.cpc = 0.5,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-codeio-fr-fr-adf17a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-installer-un-qr-code-fr-fr-7fd4b9"})
ON CREATE SET
  kw.value = "comment installer un qr code",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-installer-un-qr-code-fr-fr-7fd4b9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-basic-fit-gratuit-fr-fr-c89fa3"})
ON CREATE SET
  kw.value = "qr code basic fit gratuit",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-basic-fit-gratuit-fr-fr-c89fa3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-hi-i-am-a-qr-code-fr-fr-3f7736"})
ON CREATE SET
  kw.value = "hi i am a qr code",
  kw.volume = 150,
  kw.difficulty = 48,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-hi-i-am-a-qr-code-fr-fr-3f7736"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-snap-fr-fr-d62dfe"})
ON CREATE SET
  kw.value = "qr code snap",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-snap-fr-fr-d62dfe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-appli-qr-code-gratuite-fr-fr-447f78"})
ON CREATE SET
  kw.value = "appli qr code gratuite",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-appli-qr-code-gratuite-fr-fr-447f78"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-france-identité-qr-code-fr-fr-1c1c7f"})
ON CREATE SET
  kw.value = "france identité qr code",
  kw.volume = 150,
  kw.difficulty = 6,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-france-identité-qr-code-fr-fr-1c1c7f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-gratuit-fr-fr-1b1448"})
ON CREATE SET
  kw.value = "qr gratuit",
  kw.volume = 150,
  kw.difficulty = 22,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-gratuit-fr-fr-1b1448"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-make-qr-code-fr-fr-b2bbe9"})
ON CREATE SET
  kw.value = "make qr code",
  kw.volume = 150,
  kw.difficulty = 38,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-make-qr-code-fr-fr-b2bbe9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-permanent-gratuit-fr-fr-a8f44d"})
ON CREATE SET
  kw.value = "qr code permanent gratuit",
  kw.volume = 150,
  kw.difficulty = 24,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-permanent-gratuit-fr-fr-a8f44d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-netanyahu-fr-fr-9351fb"})
ON CREATE SET
  kw.value = "qr code netanyahu",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-netanyahu-fr-fr-9351fb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installation-qr-code-gratuit-fr-fr-613dcd"})
ON CREATE SET
  kw.value = "installation qr code gratuit",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installation-qr-code-gratuit-fr-fr-613dcd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-le-grand-quiz-tf1-fr-fr-0d3df7"})
ON CREATE SET
  kw.value = "qr code le grand quiz tf1",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-le-grand-quiz-tf1-fr-fr-0d3df7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-ouvrir-un-qr-code-fr-fr-77e076"})
ON CREATE SET
  kw.value = "comment ouvrir un qr code",
  kw.volume = 150,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-ouvrir-un-qr-code-fr-fr-77e076"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faux-courriers-assurance-maladie-qr-code-fr-fr-6e1854"})
ON CREATE SET
  kw.value = "faux courriers assurance maladie qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faux-courriers-assurance-maladie-qr-code-fr-fr-6e1854"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-steam-qr-code-fr-fr-6702c0"})
ON CREATE SET
  kw.value = "steam qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 1.7,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-steam-qr-code-fr-fr-6702c0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-google-gratuit-fr-fr-36e0f0"})
ON CREATE SET
  kw.value = "qr code google gratuit",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-google-gratuit-fr-fr-36e0f0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-qr-code-gratuit-android-fr-fr-7c3da6"})
ON CREATE SET
  kw.value = "application qr code gratuit android",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-qr-code-gratuit-android-fr-fr-7c3da6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-réseaux-sociaux-fr-fr-f10dc9"})
ON CREATE SET
  kw.value = "qr code réseaux sociaux",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-réseaux-sociaux-fr-fr-f10dc9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-utiliser-le-qr-code-fr-fr-dd64e2"})
ON CREATE SET
  kw.value = "comment utiliser le qr code",
  kw.volume = 150,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-utiliser-le-qr-code-fr-fr-dd64e2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-telephone-fr-fr-5d8958"})
ON CREATE SET
  kw.value = "qr code telephone",
  kw.volume = 100,
  kw.difficulty = 3,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-telephone-fr-fr-5d8958"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-cree-un-qr-code-fr-fr-6fbbfb"})
ON CREATE SET
  kw.value = "cree un qr code",
  kw.volume = 100,
  kw.difficulty = 34,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-cree-un-qr-code-fr-fr-6fbbfb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-mondial-relay-qr-code-amazon-fr-fr-566df1"})
ON CREATE SET
  kw.value = "mondial relay qr code amazon",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.9,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-mondial-relay-qr-code-amazon-fr-fr-566df1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-sans-pub-fr-fr-3ed7cb"})
ON CREATE SET
  kw.value = "qr code gratuit sans pub",
  kw.volume = 100,
  kw.difficulty = 23,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-sans-pub-fr-fr-3ed7cb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-arrêt-de-travail-sans-qr-code-fr-fr-20fe9e"})
ON CREATE SET
  kw.value = "arrêt de travail sans qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-arrêt-de-travail-sans-qr-code-fr-fr-20fe9e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-payer-par-qr-code-fr-fr-e404d9"})
ON CREATE SET
  kw.value = "comment payer par qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-payer-par-qr-code-fr-fr-e404d9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-shopify-qr-code-fr-fr-f12854"})
ON CREATE SET
  kw.value = "shopify qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 1.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-shopify-qr-code-fr-fr-f12854"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-def-fr-fr-3896ca"})
ON CREATE SET
  kw.value = "qr code def",
  kw.volume = 100,
  kw.difficulty = 3,
  kw.cpc = 0.03,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-def-fr-fr-3896ca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-xiaomi-fr-fr-1b3ab0"})
ON CREATE SET
  kw.value = "qr code xiaomi",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-xiaomi-fr-fr-1b3ab0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-db-legends-2025-fr-fr-d00f34"})
ON CREATE SET
  kw.value = "qr code db legends 2025",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-db-legends-2025-fr-fr-d00f34"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-mon-qr-code-fr-fr-5a0a88"})
ON CREATE SET
  kw.value = "mon qr code",
  kw.volume = 100,
  kw.difficulty = 27,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-mon-qr-code-fr-fr-5a0a88"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-loto-qr-code-fr-fr-8c44a4"})
ON CREATE SET
  kw.value = "loto qr code",
  kw.volume = 100,
  kw.difficulty = 22,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-loto-qr-code-fr-fr-8c44a4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-meaning-fr-fr-5a6aeb"})
ON CREATE SET
  kw.value = "qr code meaning",
  kw.volume = 100,
  kw.difficulty = 57,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-meaning-fr-fr-5a6aeb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-tf1-fr-fr-9a8e02"})
ON CREATE SET
  kw.value = "qr code tf1",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-tf1-fr-fr-9a8e02"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-yo-kai-watch-blasters-qr-code-fr-fr-5e5a2a"})
ON CREATE SET
  kw.value = "yo kai watch blasters qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-yo-kai-watch-blasters-qr-code-fr-fr-5e5a2a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-stickers-fr-fr-533402"})
ON CREATE SET
  kw.value = "qr code stickers",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-stickers-fr-fr-533402"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-payer-avec-qr-code-fr-fr-cc26de"})
ON CREATE SET
  kw.value = "payer avec qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-payer-avec-qr-code-fr-fr-cc26de"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-fr-fr-307d9d"})
ON CREATE SET
  kw.value = "qr code.",
  kw.volume = 100,
  kw.difficulty = 50,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-fr-fr-307d9d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-t-shirt-qr-code-fr-fr-38048b"})
ON CREATE SET
  kw.value = "t shirt qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-t-shirt-qr-code-fr-fr-38048b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-fictif-fr-fr-11b8e5"})
ON CREATE SET
  kw.value = "qr code fictif",
  kw.volume = 100,
  kw.difficulty = 35,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-fictif-fr-fr-11b8e5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-bitly-qr-code-fr-fr-20a320"})
ON CREATE SET
  kw.value = "bitly qr code",
  kw.volume = 100,
  kw.difficulty = 4,
  kw.cpc = 0.8,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-bitly-qr-code-fr-fr-20a320"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-black-mirror-fr-fr-ec7f14"})
ON CREATE SET
  kw.value = "qr code black mirror",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-black-mirror-fr-fr-ec7f14"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ia-fr-fr-f5705f"})
ON CREATE SET
  kw.value = "qr code ia",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ia-fr-fr-f5705f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-app-qr-code-gratuit-fr-fr-f1d335"})
ON CREATE SET
  kw.value = "app qr code gratuit",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-app-qr-code-gratuit-fr-fr-f1d335"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-original-fr-fr-cc3318"})
ON CREATE SET
  kw.value = "qr code original",
  kw.volume = 100,
  kw.difficulty = 50,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-original-fr-fr-cc3318"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-genere-un-qr-code-fr-fr-6719c1"})
ON CREATE SET
  kw.value = "genere un qr code",
  kw.volume = 100,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-genere-un-qr-code-fr-fr-6719c1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pc-fr-fr-21d61e"})
ON CREATE SET
  kw.value = "qr code pc",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pc-fr-fr-21d61e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ne-fonctionne-pas-samsung-fr-fr-b24ef6"})
ON CREATE SET
  kw.value = "qr code ne fonctionne pas samsung",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ne-fonctionne-pas-samsung-fr-fr-b24ef6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-le-qr-code-fr-fr-cbd822"})
ON CREATE SET
  kw.value = "installer le qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-le-qr-code-fr-fr-cbd822"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-déchetterie-amiens-qr-code-fr-fr-1fb744"})
ON CREATE SET
  kw.value = "déchetterie amiens qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-déchetterie-amiens-qr-code-fr-fr-1fb744"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-pourquoi-mon-qr-code-ne-fonctionne-pas-fr-fr-93de2b"})
ON CREATE SET
  kw.value = "pourquoi mon qr code ne fonctionne pas",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-pourquoi-mon-qr-code-ne-fonctionne-pas-fr-fr-93de2b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-un-qr-code-fr-fr-6de431"})
ON CREATE SET
  kw.value = "installer un qr code",
  kw.volume = 100,
  kw.difficulty = 5,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-un-qr-code-fr-fr-6de431"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecture-qr-code-samsung-fr-fr-fea070"})
ON CREATE SET
  kw.value = "lecture qr code samsung",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecture-qr-code-samsung-fr-fr-fea070"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-visit-japan-qr-code-fr-fr-127f60"})
ON CREATE SET
  kw.value = "visit japan qr code",
  kw.volume = 100,
  kw.difficulty = 32,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-visit-japan-qr-code-fr-fr-127f60"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecture-qr-code-android-fr-fr-5340fc"})
ON CREATE SET
  kw.value = "lecture qr code android",
  kw.volume = 100,
  kw.difficulty = 3,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecture-qr-code-android-fr-fr-5340fc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-arret-de-travail-fr-fr-496424"})
ON CREATE SET
  kw.value = "qr code arret de travail",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-arret-de-travail-fr-fr-496424"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creation-dun-qr-code-fr-fr-9dcb7c"})
ON CREATE SET
  kw.value = "creation d\'un qr code",
  kw.volume = 100,
  kw.difficulty = 22,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creation-dun-qr-code-fr-fr-9dcb7c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-transformer-une-vidéo-en-qr-code-fr-fr-910a28"})
ON CREATE SET
  kw.value = "transformer une vidéo en qr code",
  kw.volume = 100,
  kw.difficulty = 9,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-transformer-une-vidéo-en-qr-code-fr-fr-910a28"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sur-samsung-fr-fr-803722"})
ON CREATE SET
  kw.value = "qr code sur samsung",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sur-samsung-fr-fr-803722"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-impression-qr-code-fr-fr-6a5a4a"})
ON CREATE SET
  kw.value = "impression qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-impression-qr-code-fr-fr-6a5a4a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-papier-fr-fr-cfa602"})
ON CREATE SET
  kw.value = "qr code papier",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-papier-fr-fr-cfa602"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generate-qr-code-free-fr-fr-2d498a"})
ON CREATE SET
  kw.value = "generate qr code free",
  kw.volume = 100,
  kw.difficulty = 92,
  kw.cpc = 0.25,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generate-qr-code-free-fr-fr-2d498a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-read-qr-code-fr-fr-516c4d"})
ON CREATE SET
  kw.value = "read qr code",
  kw.volume = 100,
  kw.difficulty = 62,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-read-qr-code-fr-fr-516c4d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-samsung-qr-code-fr-fr-e2faf5"})
ON CREATE SET
  kw.value = "samsung qr code",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-samsung-qr-code-fr-fr-e2faf5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-qr-code-canva-fr-fr-68b1b1"})
ON CREATE SET
  kw.value = "création qr code canva",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-qr-code-canva-fr-fr-68b1b1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-a-vie-fr-fr-971777"})
ON CREATE SET
  kw.value = "qr code gratuit a vie",
  kw.volume = 100,
  kw.difficulty = 23,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-a-vie-fr-fr-971777"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sans-fond-fr-fr-b451b4"})
ON CREATE SET
  kw.value = "qr code sans fond",
  kw.volume = 100,
  kw.difficulty = 6,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sans-fond-fr-fr-b451b4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-japon-fr-fr-73ffc6"})
ON CREATE SET
  kw.value = "qr code japon",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-japon-fr-fr-73ffc6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-linktree-qr-code-fr-fr-d84034"})
ON CREATE SET
  kw.value = "linktree qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-linktree-qr-code-fr-fr-d84034"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-paypal-fr-fr-9d8392"})
ON CREATE SET
  kw.value = "qr code paypal",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-paypal-fr-fr-9d8392"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-un-qr-code-gratuit-fr-fr-5b4f84"})
ON CREATE SET
  kw.value = "installer un qr code gratuit",
  kw.volume = 100,
  kw.difficulty = 10,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-un-qr-code-gratuit-fr-fr-5b4f84"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-digitale-qr-code-fr-fr-b5c7e8"})
ON CREATE SET
  kw.value = "digitale qr code",
  kw.volume = 100,
  kw.difficulty = 3,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-digitale-qr-code-fr-fr-b5c7e8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mii-spéciaux-fr-fr-26d8b4"})
ON CREATE SET
  kw.value = "qr code mii spéciaux",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mii-spéciaux-fr-fr-26d8b4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-illustrator-fr-fr-016889"})
ON CREATE SET
  kw.value = "qr code illustrator",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-illustrator-fr-fr-016889"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-faire-un-qr-code-sur-canva-fr-fr-e6f2d7"})
ON CREATE SET
  kw.value = "comment faire un qr code sur canva",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-faire-un-qr-code-sur-canva-fr-fr-e6f2d7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-relais-colis-qr-code-fr-fr-b30296"})
ON CREATE SET
  kw.value = "relais colis qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-relais-colis-qr-code-fr-fr-b30296"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-autocollant-fr-fr-11da23"})
ON CREATE SET
  kw.value = "qr code autocollant",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-autocollant-fr-fr-11da23"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-esim-fr-fr-f0e0e5"})
ON CREATE SET
  kw.value = "qr code esim",
  kw.volume = 100,
  kw.difficulty = 3,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-esim-fr-fr-f0e0e5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-test-qi-tf1-qr-code-fr-fr-7c8c19"})
ON CREATE SET
  kw.value = "test qi tf1 qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-test-qi-tf1-qr-code-fr-fr-7c8c19"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-this-is-a-qr-code-fr-fr-c9fb99"})
ON CREATE SET
  kw.value = "this is a qr code",
  kw.volume = 100,
  kw.difficulty = 39,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-this-is-a-qr-code-fr-fr-c9fb99"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-decode-qr-code-fr-fr-58250a"})
ON CREATE SET
  kw.value = "decode qr code",
  kw.volume = 100,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-decode-qr-code-fr-fr-58250a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-google-authenticator-qr-code-fr-fr-42ed7d"})
ON CREATE SET
  kw.value = "google authenticator qr code",
  kw.volume = 100,
  kw.difficulty = 38,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-google-authenticator-qr-code-fr-fr-42ed7d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-convertir-un-lien-en-qr-code-fr-fr-299662"})
ON CREATE SET
  kw.value = "convertir un lien en qr code",
  kw.volume = 100,
  kw.difficulty = 20,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-convertir-un-lien-en-qr-code-fr-fr-299662"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-to-link-fr-fr-6965e7"})
ON CREATE SET
  kw.value = "qr code to link",
  kw.volume = 100,
  kw.difficulty = 20,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-to-link-fr-fr-6965e7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-et-illimité-fr-fr-cdfe2d"})
ON CREATE SET
  kw.value = "qr code gratuit et illimité",
  kw.volume = 100,
  kw.difficulty = 14,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-et-illimité-fr-fr-cdfe2d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-erstellen-fr-fr-c8ac49"})
ON CREATE SET
  kw.value = "qr code erstellen",
  kw.volume = 100,
  kw.difficulty = 65,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-erstellen-fr-fr-c8ac49"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-lecture-qr-code-fr-fr-5c42c4"})
ON CREATE SET
  kw.value = "application lecture qr code",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-lecture-qr-code-fr-fr-5c42c4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-read-qr-code-online-fr-fr-24ea09"})
ON CREATE SET
  kw.value = "read qr code online",
  kw.volume = 100,
  kw.difficulty = 73,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-read-qr-code-online-fr-fr-24ea09"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-fiche-contact-fr-fr-7f5d4c"})
ON CREATE SET
  kw.value = "qr code fiche contact",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-fiche-contact-fr-fr-7f5d4c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-payer-qr-code-fr-fr-762348"})
ON CREATE SET
  kw.value = "payer qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-payer-qr-code-fr-fr-762348"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-canada-goose-fr-fr-d8e112"})
ON CREATE SET
  kw.value = "qr code canada goose",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-canada-goose-fr-fr-d8e112"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-unitag-qr-code-fr-fr-ad3fd8"})
ON CREATE SET
  kw.value = "unitag qr code",
  kw.volume = 100,
  kw.difficulty = 14,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-unitag-qr-code-fr-fr-ad3fd8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-decoder-fr-fr-eadcc2"})
ON CREATE SET
  kw.value = "qr code decoder",
  kw.volume = 100,
  kw.difficulty = 48,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-decoder-fr-fr-eadcc2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pierre-tombale-fr-fr-67c781"})
ON CREATE SET
  kw.value = "qr code pierre tombale",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pierre-tombale-fr-fr-67c781"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-signification-qr-code-fr-fr-0d6c2b"})
ON CREATE SET
  kw.value = "signification qr code",
  kw.volume = 100,
  kw.difficulty = 3,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-signification-qr-code-fr-fr-0d6c2b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-génération-de-qr-code-fr-fr-87713d"})
ON CREATE SET
  kw.value = "génération de qr code",
  kw.volume = 100,
  kw.difficulty = 34,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-génération-de-qr-code-fr-fr-87713d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-nike-fr-fr-e938b7"})
ON CREATE SET
  kw.value = "qr code nike",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-nike-fr-fr-e938b7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-flashez-le-qr-code-fr-fr-0765a5"})
ON CREATE SET
  kw.value = "flashez le qr code",
  kw.volume = 100,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-flashez-le-qr-code-fr-fr-0765a5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-example-fr-fr-555831"})
ON CREATE SET
  kw.value = "qr code example",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-example-fr-fr-555831"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-app-qr-code-fr-fr-a3e4f3"})
ON CREATE SET
  kw.value = "app qr code",
  kw.volume = 100,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-app-qr-code-fr-fr-a3e4f3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-dragon-ball-legends-qr-code-fr-fr-a0683a"})
ON CREATE SET
  kw.value = "dragon ball legends qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-dragon-ball-legends-qr-code-fr-fr-a0683a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-plaque-qr-code-avis-google-fr-fr-d58346"})
ON CREATE SET
  kw.value = "plaque qr code avis google",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-plaque-qr-code-avis-google-fr-fr-d58346"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-cr2er-un-qr-code-fr-fr-3b52b0"})
ON CREATE SET
  kw.value = "cr2er un qr code",
  kw.volume = 100,
  kw.difficulty = 33,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-cr2er-un-qr-code-fr-fr-3b52b0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ordonnance-sécurisée-qr-code-fr-fr-5b21ad"})
ON CREATE SET
  kw.value = "ordonnance sécurisée qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ordonnance-sécurisée-qr-code-fr-fr-5b21ad"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-text-to-qr-code-fr-fr-7b1d4c"})
ON CREATE SET
  kw.value = "text to qr code",
  kw.volume = 100,
  kw.difficulty = 71,
  kw.cpc = 0.15,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-text-to-qr-code-fr-fr-7b1d4c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-la-poste-fr-fr-be3e29"})
ON CREATE SET
  kw.value = "qr code la poste",
  kw.volume = 100,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-la-poste-fr-fr-be3e29"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-site-fr-fr-c9b2fb"})
ON CREATE SET
  kw.value = "qr code site",
  kw.volume = 100,
  kw.difficulty = 29,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-site-fr-fr-c9b2fb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-crée-qr-code-fr-fr-79f62f"})
ON CREATE SET
  kw.value = "crée qr code",
  kw.volume = 100,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-crée-qr-code-fr-fr-79f62f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-excel-fr-fr-c78481"})
ON CREATE SET
  kw.value = "qr code excel",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-excel-fr-fr-c78481"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lien-to-qr-code-fr-fr-66b751"})
ON CREATE SET
  kw.value = "lien to qr code",
  kw.volume = 100,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lien-to-qr-code-fr-fr-66b751"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-qr-code-samsung-fr-fr-06fd95"})
ON CREATE SET
  kw.value = "application qr code samsung",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-qr-code-samsung-fr-fr-06fd95"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-microsoft-authenticator-qr-code-fr-fr-df1537"})
ON CREATE SET
  kw.value = "microsoft authenticator qr code",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-microsoft-authenticator-qr-code-fr-fr-df1537"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-badge-qr-code-fr-fr-a19b79"})
ON CREATE SET
  kw.value = "badge qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 1.2,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-badge-qr-code-fr-fr-a19b79"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-d-un-qr-code-fr-fr-8ce589"})
ON CREATE SET
  kw.value = "création d un qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-d-un-qr-code-fr-fr-8ce589"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-de-mon-téléphone-fr-fr-93b37a"})
ON CREATE SET
  kw.value = "qr code de mon téléphone",
  kw.volume = 90,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-de-mon-téléphone-fr-fr-93b37a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-dolce-gusto-fr-fr-c161b5"})
ON CREATE SET
  kw.value = "qr code dolce gusto",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.04,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-dolce-gusto-fr-fr-c161b5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-japan-fr-fr-f83480"})
ON CREATE SET
  kw.value = "qr code japan",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-japan-fr-fr-f83480"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ordonnance-fr-fr-02e5ae"})
ON CREATE SET
  kw.value = "qr code ordonnance",
  kw.volume = 90,
  kw.difficulty = 3,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ordonnance-fr-fr-02e5ae"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-google-lens-qr-code-gratuit-fr-fr-746956"})
ON CREATE SET
  kw.value = "google lens qr code gratuit",
  kw.volume = 90,
  kw.difficulty = 6,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-google-lens-qr-code-gratuit-fr-fr-746956"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-qr-code-gratuit-iphone-fr-fr-f2d777"})
ON CREATE SET
  kw.value = "application qr code gratuit iphone",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-qr-code-gratuit-iphone-fr-fr-f2d777"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-site-qr-code-gratuit-fr-fr-f9be55"})
ON CREATE SET
  kw.value = "site qr code gratuit",
  kw.volume = 90,
  kw.difficulty = 23,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-site-qr-code-gratuit-fr-fr-f9be55"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generation-fr-fr-d91157"})
ON CREATE SET
  kw.value = "qr code generation",
  kw.volume = 90,
  kw.difficulty = 90,
  kw.cpc = 0.0,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generation-fr-fr-d91157"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-faire-un-qr-code-gratuit-fr-fr-35a701"})
ON CREATE SET
  kw.value = "comment faire un qr code gratuit",
  kw.volume = 90,
  kw.difficulty = 34,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-faire-un-qr-code-gratuit-fr-fr-35a701"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-get-qr-code-fr-fr-86ef7f"})
ON CREATE SET
  kw.value = "get qr code",
  kw.volume = 90,
  kw.difficulty = 56,
  kw.cpc = 0.0,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-get-qr-code-fr-fr-86ef7f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-https-tf1-fr-tv-qr-code-fr-fr-7d7e6e"})
ON CREATE SET
  kw.value = "https tf1 fr tv qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-https-tf1-fr-tv-qr-code-fr-fr-7d7e6e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-le-grand-quiz-fr-fr-1937d3"})
ON CREATE SET
  kw.value = "qr code le grand quiz",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-le-grand-quiz-fr-fr-1937d3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-business-card-fr-fr-11ca22"})
ON CREATE SET
  kw.value = "qr code business card",
  kw.volume = 90,
  kw.difficulty = 14,
  kw.cpc = 0.5,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-business-card-fr-fr-11ca22"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-où-trouver-le-qr-code-de-mon-téléphone-fr-fr-9bff70"})
ON CREATE SET
  kw.value = "où trouver le qr code de mon téléphone",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-où-trouver-le-qr-code-de-mon-téléphone-fr-fr-9bff70"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-crer-qr-code-fr-fr-fa02e1"})
ON CREATE SET
  kw.value = "crer qr code",
  kw.volume = 90,
  kw.difficulty = 35,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-crer-qr-code-fr-fr-fa02e1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lien-vers-qr-code-fr-fr-8916f6"})
ON CREATE SET
  kw.value = "lien vers qr code",
  kw.volume = 90,
  kw.difficulty = 23,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lien-vers-qr-code-fr-fr-8916f6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-hay-day-2025-fr-fr-8840a3"})
ON CREATE SET
  kw.value = "qr code hay day 2025",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-hay-day-2025-fr-fr-8840a3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-db-legends-qr-code-fr-fr-e4cbf9"})
ON CREATE SET
  kw.value = "db legends qr code",
  kw.volume = 90,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-db-legends-qr-code-fr-fr-e4cbf9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sur-iphone-fr-fr-79b062"})
ON CREATE SET
  kw.value = "qr code sur iphone",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sur-iphone-fr-fr-79b062"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-permanent-fr-fr-dcf3c2"})
ON CREATE SET
  kw.value = "qr code permanent",
  kw.volume = 90,
  kw.difficulty = 24,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-permanent-fr-fr-dcf3c2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-réinitialiser-caméra-netatmo-sans-qr-code-fr-fr-56d0af"})
ON CREATE SET
  kw.value = "réinitialiser caméra netatmo sans qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-réinitialiser-caméra-netatmo-sans-qr-code-fr-fr-56d0af"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ouvrir-qr-code-fr-fr-ad9a46"})
ON CREATE SET
  kw.value = "ouvrir qr code",
  kw.volume = 90,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ouvrir-qr-code-fr-fr-ad9a46"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-flyer-avec-qr-code-fr-fr-8720e8"})
ON CREATE SET
  kw.value = "flyer avec qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-flyer-avec-qr-code-fr-fr-8720e8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-honor-fr-fr-10dcab"})
ON CREATE SET
  kw.value = "qr code honor",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-honor-fr-fr-10dcab"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-cv-avec-qr-code-fr-fr-12fb58"})
ON CREATE SET
  kw.value = "cv avec qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-cv-avec-qr-code-fr-fr-12fb58"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-image-gratuit-fr-fr-4fd08b"})
ON CREATE SET
  kw.value = "qr code image gratuit",
  kw.volume = 90,
  kw.difficulty = 36,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-image-gratuit-fr-fr-4fd08b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-v-bucks-fr-fr-93ff5f"})
ON CREATE SET
  kw.value = "qr code v bucks",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-v-bucks-fr-fr-93ff5f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-augmenté-gs1-fr-fr-23684f"})
ON CREATE SET
  kw.value = "qr code augmenté gs1",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-augmenté-gs1-fr-fr-23684f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-payer-en-qr-code-fr-fr-ec690a"})
ON CREATE SET
  kw.value = "payer en qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-payer-en-qr-code-fr-fr-ec690a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-adobe-express-fr-fr-04e4ca"})
ON CREATE SET
  kw.value = "qr code adobe express",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-adobe-express-fr-fr-04e4ca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mo-co-fr-fr-10c851"})
ON CREATE SET
  kw.value = "qr code mo co",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mo-co-fr-fr-10c851"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-photo-fr-fr-965fb2"})
ON CREATE SET
  kw.value = "qr code photo",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-photo-fr-fr-965fb2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-avis-google-my-business-fr-fr-03921c"})
ON CREATE SET
  kw.value = "qr code avis google my business",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-avis-google-my-business-fr-fr-03921c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-apple-fr-fr-45b0fd"})
ON CREATE SET
  kw.value = "qr code apple",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-apple-fr-fr-45b0fd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generateur-gratuit-fr-fr-88692f"})
ON CREATE SET
  kw.value = "qr code generateur gratuit",
  kw.volume = 90,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generateur-gratuit-fr-fr-88692f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-flyer-qr-code-fr-fr-2ba5c1"})
ON CREATE SET
  kw.value = "flyer qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-flyer-qr-code-fr-fr-2ba5c1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-rick-roll-qr-code-fr-fr-d97f08"})
ON CREATE SET
  kw.value = "rick roll qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-rick-roll-qr-code-fr-fr-d97f08"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-cv-fr-fr-bee7b4"})
ON CREATE SET
  kw.value = "qr code cv",
  kw.volume = 90,
  kw.difficulty = 5,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-cv-fr-fr-bee7b4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pizza-fr-fr-ab8e7d"})
ON CREATE SET
  kw.value = "qr code pizza",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pizza-fr-fr-ab8e7d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-exemple-qr-code-test-fr-fr-0cd596"})
ON CREATE SET
  kw.value = "exemple qr code test",
  kw.volume = 90,
  kw.difficulty = 3,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-exemple-qr-code-test-fr-fr-0cd596"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-make-a-qr-code-fr-fr-cf9a3b"})
ON CREATE SET
  kw.value = "how to make a qr code",
  kw.volume = 90,
  kw.difficulty = 92,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-make-a-qr-code-fr-fr-cf9a3b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-appli-fr-fr-bab459"})
ON CREATE SET
  kw.value = "qr code appli",
  kw.volume = 90,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-appli-fr-fr-bab459"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-dernier-instant-qr-code-fr-fr-68eeee"})
ON CREATE SET
  kw.value = "dernier instant qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-dernier-instant-qr-code-fr-fr-68eeee"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-moncler-fr-fr-09ee59"})
ON CREATE SET
  kw.value = "qr code moncler",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-moncler-fr-fr-09ee59"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-esim-orange-fr-fr-384069"})
ON CREATE SET
  kw.value = "qr code esim orange",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-esim-orange-fr-fr-384069"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-faire-qr-code-fr-fr-81d0a9"})
ON CREATE SET
  kw.value = "comment faire qr code",
  kw.volume = 80,
  kw.difficulty = 11,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-faire-qr-code-fr-fr-81d0a9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-questionnaire-qr-code-fr-fr-45b1fe"})
ON CREATE SET
  kw.value = "questionnaire qr code",
  kw.volume = 80,
  kw.difficulty = 5,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-questionnaire-qr-code-fr-fr-45b1fe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-db-legend-fr-fr-f724fd"})
ON CREATE SET
  kw.value = "qr code db legend",
  kw.volume = 80,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-db-legend-fr-fr-f724fd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-microsoft-authenticator-fr-fr-5e0ece"})
ON CREATE SET
  kw.value = "qr code microsoft authenticator",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-microsoft-authenticator-fr-fr-5e0ece"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ouvrir-un-qr-code-sur-pc-fr-fr-f3471a"})
ON CREATE SET
  kw.value = "ouvrir un qr code sur pc",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ouvrir-un-qr-code-sur-pc-fr-fr-f3471a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créateur-qr-code-gratuit-fr-fr-45c409"})
ON CREATE SET
  kw.value = "créateur qr code gratuit",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créateur-qr-code-gratuit-fr-fr-45c409"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-thailande-fr-fr-ee32c2"})
ON CREATE SET
  kw.value = "qr code thailande",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-thailande-fr-fr-ee32c2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-discord-qr-code-fr-fr-8d417e"})
ON CREATE SET
  kw.value = "discord qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-discord-qr-code-fr-fr-8d417e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-cest-quoi-un-qr-code-fr-fr-9fdfe6"})
ON CREATE SET
  kw.value = "c\'est quoi un qr code",
  kw.volume = 80,
  kw.difficulty = 11,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-cest-quoi-un-qr-code-fr-fr-9fdfe6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-parions-sport-fr-fr-fca2db"})
ON CREATE SET
  kw.value = "qr code parions sport",
  kw.volume = 80,
  kw.difficulty = 9,
  kw.cpc = 0.7,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-parions-sport-fr-fr-fca2db"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-obtenir-un-qr-code-fr-fr-557f8e"})
ON CREATE SET
  kw.value = "obtenir un qr code",
  kw.volume = 80,
  kw.difficulty = 33,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-obtenir-un-qr-code-fr-fr-557f8e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generation-de-qr-code-fr-fr-706e7c"})
ON CREATE SET
  kw.value = "generation de qr code",
  kw.volume = 80,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generation-de-qr-code-fr-fr-706e7c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr-code-fr-fr-c2fbb1"})
ON CREATE SET
  kw.value = "code qr code",
  kw.volume = 80,
  kw.difficulty = 37,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr-code-fr-fr-c2fbb1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-tee-shirt-qr-code-fr-fr-bb036b"})
ON CREATE SET
  kw.value = "tee shirt qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-tee-shirt-qr-code-fr-fr-bb036b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-gen-qr-code-fr-fr-30cf77"})
ON CREATE SET
  kw.value = "gen qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-gen-qr-code-fr-fr-30cf77"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-euromillion-fr-fr-dbd5e3"})
ON CREATE SET
  kw.value = "qr code euromillion",
  kw.volume = 80,
  kw.difficulty = 27,
  kw.cpc = 0.6,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-euromillion-fr-fr-dbd5e3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-historique-qr-code-fr-fr-b1ce77"})
ON CREATE SET
  kw.value = "historique qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.09,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-historique-qr-code-fr-fr-b1ce77"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-le-qr-code-fr-fr-f61e8d"})
ON CREATE SET
  kw.value = "le qr code",
  kw.volume = 80,
  kw.difficulty = 16,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-le-qr-code-fr-fr-f61e8d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-me-fr-fr-e722c0"})
ON CREATE SET
  kw.value = "qr code me",
  kw.volume = 80,
  kw.difficulty = 6,
  kw.cpc = 0.0,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-me-fr-fr-e722c0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sushi-brawl-star-fr-fr-4b28c7"})
ON CREATE SET
  kw.value = "qr code sushi brawl star",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sushi-brawl-star-fr-fr-4b28c7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-creator-free-fr-fr-d58fcd"})
ON CREATE SET
  kw.value = "qr code creator free",
  kw.volume = 80,
  kw.difficulty = 89,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-creator-free-fr-fr-d58fcd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-tiger-qr-code-fr-fr-8dd860"})
ON CREATE SET
  kw.value = "tiger qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Commercial,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-tiger-qr-code-fr-fr-8dd860"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-google-forms-qr-code-fr-fr-ee0a01"})
ON CREATE SET
  kw.value = "google forms qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-google-forms-qr-code-fr-fr-ee0a01"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-japan-web-qr-code-fr-fr-51b4c0"})
ON CREATE SET
  kw.value = "japan web qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-japan-web-qr-code-fr-fr-51b4c0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-plusieurs-liens-fr-fr-008d7c"})
ON CREATE SET
  kw.value = "qr code plusieurs liens",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-plusieurs-liens-fr-fr-008d7c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sushi-gratuit-fr-fr-8b1fb9"})
ON CREATE SET
  kw.value = "qr code sushi gratuit",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sushi-gratuit-fr-fr-8b1fb9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-wero-fr-fr-18908d"})
ON CREATE SET
  kw.value = "qr code wero",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-wero-fr-fr-18908d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-to-text-fr-fr-83b9c3"})
ON CREATE SET
  kw.value = "qr code to text",
  kw.volume = 80,
  kw.difficulty = 47,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-to-text-fr-fr-83b9c3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-bouteille-de-vin-fr-fr-424447"})
ON CREATE SET
  kw.value = "qr code bouteille de vin",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-bouteille-de-vin-fr-fr-424447"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-application-gratuite-fr-fr-25b778"})
ON CREATE SET
  kw.value = "qr code application gratuite",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-application-gratuite-fr-fr-25b778"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-avoir-un-qr-code-fr-fr-e805bb"})
ON CREATE SET
  kw.value = "comment avoir un qr code",
  kw.volume = 80,
  kw.difficulty = 9,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-avoir-un-qr-code-fr-fr-e805bb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-moco-qr-code-fr-fr-58d85a"})
ON CREATE SET
  kw.value = "moco qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-moco-qr-code-fr-fr-58d85a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-gratuite-qr-code-fr-fr-dd2da8"})
ON CREATE SET
  kw.value = "application gratuite qr code",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-gratuite-qr-code-fr-fr-dd2da8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-genrateur-qr-code-fr-fr-3c40e4"})
ON CREATE SET
  kw.value = "genrateur qr code",
  kw.volume = 80,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-genrateur-qr-code-fr-fr-3c40e4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-fonctionne-le-qr-code-fr-fr-432a78"})
ON CREATE SET
  kw.value = "comment fonctionne le qr code",
  kw.volume = 80,
  kw.difficulty = 8,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-fonctionne-le-qr-code-fr-fr-432a78"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-animal-crossing-new-horizon-fr-fr-9ec429"})
ON CREATE SET
  kw.value = "qr code animal crossing new horizon",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-animal-crossing-new-horizon-fr-fr-9ec429"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-billet-sncf-fr-fr-2a35ca"})
ON CREATE SET
  kw.value = "qr code billet sncf",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-billet-sncf-fr-fr-2a35ca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-link-fr-fr-242c9d"})
ON CREATE SET
  kw.value = "qr code link",
  kw.volume = 80,
  kw.difficulty = 83,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-link-fr-fr-242c9d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-génerer-qr-code-fr-fr-5b5ae7"})
ON CREATE SET
  kw.value = "génerer qr code",
  kw.volume = 80,
  kw.difficulty = 35,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-génerer-qr-code-fr-fr-5b5ae7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-se-servir-dun-qr-code-fr-fr-35db18"})
ON CREATE SET
  kw.value = "comment se servir d\'un qr code",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-se-servir-dun-qr-code-fr-fr-35db18"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pornhub-fr-fr-e57afc"})
ON CREATE SET
  kw.value = "qr code pornhub",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pornhub-fr-fr-e57afc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-augmenté-fr-fr-bee92a"})
ON CREATE SET
  kw.value = "qr code augmenté",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-augmenté-fr-fr-bee92a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-codes-gratuits-fr-fr-2722b3"})
ON CREATE SET
  kw.value = "qr codes gratuits",
  kw.volume = 80,
  kw.difficulty = 20,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-codes-gratuits-fr-fr-2722b3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-brawl-star-2025-fr-fr-653069"})
ON CREATE SET
  kw.value = "qr code brawl star 2025",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-brawl-star-2025-fr-fr-653069"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-your-qr-code-fr-fr-b90d4c"})
ON CREATE SET
  kw.value = "your qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-your-qr-code-fr-fr-b90d4c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-tax-free-qr-code-japan-fr-fr-058eae"})
ON CREATE SET
  kw.value = "tax free qr code japan",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-tax-free-qr-code-japan-fr-fr-058eae"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gs1-fr-fr-39f132"})
ON CREATE SET
  kw.value = "qr code gs1",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gs1-fr-fr-39f132"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-wix-qr-code-fr-fr-2b4db0"})
ON CREATE SET
  kw.value = "wix qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-wix-qr-code-fr-fr-2b4db0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pierre-tombale-prix-fr-fr-dab603"})
ON CREATE SET
  kw.value = "qr code pierre tombale prix",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pierre-tombale-prix-fr-fr-dab603"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-connecter-une-caméra-sans-qr-code-fr-fr-0ecb80"})
ON CREATE SET
  kw.value = "comment connecter une caméra sans qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-connecter-une-caméra-sans-qr-code-fr-fr-0ecb80"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-autocollant-qr-code-fr-fr-9d299c"})
ON CREATE SET
  kw.value = "autocollant qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-autocollant-qr-code-fr-fr-9d299c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-monkey-gratuit-fr-fr-8377c6"})
ON CREATE SET
  kw.value = "qr code monkey gratuit",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.45,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-monkey-gratuit-fr-fr-8377c6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-dragon-ball-legends-2025-fr-fr-7c0501"})
ON CREATE SET
  kw.value = "qr code dragon ball legends 2025",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-dragon-ball-legends-2025-fr-fr-7c0501"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-cimetiere-fr-fr-593f88"})
ON CREATE SET
  kw.value = "qr code cimetiere",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-cimetiere-fr-fr-593f88"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-imprimante-qr-code-autocollant-fr-fr-2e5d22"})
ON CREATE SET
  kw.value = "imprimante qr code autocollant",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-imprimante-qr-code-autocollant-fr-fr-2e5d22"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ouvrir-un-qr-code-fr-fr-ff0c9c"})
ON CREATE SET
  kw.value = "ouvrir un qr code",
  kw.volume = 80,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ouvrir-un-qr-code-fr-fr-ff0c9c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-définition-fr-fr-eb20d3"})
ON CREATE SET
  kw.value = "qr code définition",
  kw.volume = 80,
  kw.difficulty = 7,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-définition-fr-fr-eb20d3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-euromillion-qr-code-fr-fr-6664b3"})
ON CREATE SET
  kw.value = "euromillion qr code",
  kw.volume = 80,
  kw.difficulty = 21,
  kw.cpc = 1.6,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-euromillion-qr-code-fr-fr-6664b3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-steam-guard-fr-fr-8590e3"})
ON CREATE SET
  kw.value = "qr code steam guard",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-steam-guard-fr-fr-8590e3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-vinted-go-qr-code-fr-fr-664ecf"})
ON CREATE SET
  kw.value = "vinted go qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-vinted-go-qr-code-fr-fr-664ecf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-euromillions-qr-code-fr-fr-dcc370"})
ON CREATE SET
  kw.value = "euromillions qr code",
  kw.volume = 70,
  kw.difficulty = 28,
  kw.cpc = 1.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-euromillions-qr-code-fr-fr-dcc370"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creation-de-qr-code-gratuit-fr-fr-dd4927"})
ON CREATE SET
  kw.value = "creation de qr code gratuit",
  kw.volume = 70,
  kw.difficulty = 33,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creation-de-qr-code-gratuit-fr-fr-dd4927"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ferrero-rocher-fr-fr-315a25"})
ON CREATE SET
  kw.value = "qr code ferrero rocher",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ferrero-rocher-fr-fr-315a25"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-snapseed-qr-code-fr-fr-6bc51e"})
ON CREATE SET
  kw.value = "snapseed qr code",
  kw.volume = 70,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-snapseed-qr-code-fr-fr-6bc51e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecture-de-qr-code-fr-fr-da2c00"})
ON CREATE SET
  kw.value = "lecture de qr code",
  kw.volume = 70,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecture-de-qr-code-fr-fr-da2c00"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-edu-qr-code-fr-fr-97198d"})
ON CREATE SET
  kw.value = "edu qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-edu-qr-code-fr-fr-97198d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-cv-qr-code-fr-fr-467c46"})
ON CREATE SET
  kw.value = "cv qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-cv-qr-code-fr-fr-467c46"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vidéo-fr-fr-41ac38"})
ON CREATE SET
  kw.value = "qr code vidéo",
  kw.volume = 70,
  kw.difficulty = 21,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vidéo-fr-fr-41ac38"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-steam-guard-qr-code-fr-fr-beecda"})
ON CREATE SET
  kw.value = "steam guard qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-steam-guard-qr-code-fr-fr-beecda"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-freebox-révolution-fr-fr-7bb32d"})
ON CREATE SET
  kw.value = "qr code freebox révolution",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-freebox-révolution-fr-fr-7bb32d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ami-paris-fr-fr-ee5c9a"})
ON CREATE SET
  kw.value = "qr code ami paris",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ami-paris-fr-fr-ee5c9a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-recherche-qr-code-fr-fr-eb66fa"})
ON CREATE SET
  kw.value = "recherche qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-recherche-qr-code-fr-fr-eb66fa"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-db-legends-qr-code-fr-fr-b72539"})
ON CREATE SET
  kw.value = "code db legends qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-db-legends-qr-code-fr-fr-b72539"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecture-qr-code-pc-fr-fr-f2448f"})
ON CREATE SET
  kw.value = "lecture qr code pc",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.9,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecture-qr-code-pc-fr-fr-f2448f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-fdj-gratuit-fr-fr-0d2625"})
ON CREATE SET
  kw.value = "qr code fdj gratuit",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-fdj-gratuit-fr-fr-0d2625"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-random-qr-code-fr-fr-a3467d"})
ON CREATE SET
  kw.value = "random qr code",
  kw.volume = 70,
  kw.difficulty = 5,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-random-qr-code-fr-fr-a3467d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-esim-sfr-fr-fr-38ebeb"})
ON CREATE SET
  kw.value = "qr code esim sfr",
  kw.volume = 70,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-esim-sfr-fr-fr-38ebeb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-mii-tomodachi-life-qr-code-fr-fr-fce561"})
ON CREATE SET
  kw.value = "mii tomodachi life qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-mii-tomodachi-life-qr-code-fr-fr-fce561"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-applications-qr-code-fr-fr-72f81c"})
ON CREATE SET
  kw.value = "applications qr code",
  kw.volume = 70,
  kw.difficulty = 5,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-applications-qr-code-fr-fr-72f81c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-meilleur-qr-code-gratuit-fr-fr-b336b2"})
ON CREATE SET
  kw.value = "meilleur qr code gratuit",
  kw.volume = 70,
  kw.difficulty = 11,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-meilleur-qr-code-gratuit-fr-fr-b336b2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-mmmbook-qr-code-gratuit-fr-fr-b5763b"})
ON CREATE SET
  kw.value = "mmmbook qr code gratuit",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 1.5,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-mmmbook-qr-code-gratuit-fr-fr-b5763b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-mii-qr-code-fr-fr-841cdb"})
ON CREATE SET
  kw.value = "mii qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-mii-qr-code-fr-fr-841cdb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-doigt-dhonneur-fr-fr-e702d9"})
ON CREATE SET
  kw.value = "qr code doigt d\'honneur",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-doigt-dhonneur-fr-fr-e702d9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-quiz-qr-code-fr-fr-31215d"})
ON CREATE SET
  kw.value = "quiz qr code",
  kw.volume = 70,
  kw.difficulty = 4,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-quiz-qr-code-fr-fr-31215d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-utiliser-un-qr-code-sur-iphone-fr-fr-38ec1e"})
ON CREATE SET
  kw.value = "comment utiliser un qr code sur iphone",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-utiliser-un-qr-code-sur-iphone-fr-fr-38ec1e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-a-quoi-sert-un-qr-code-fr-fr-9d3f33"})
ON CREATE SET
  kw.value = "a quoi sert un qr code",
  kw.volume = 70,
  kw.difficulty = 8,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-a-quoi-sert-un-qr-code-fr-fr-9d3f33"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-wero-qr-code-fr-fr-bc8ddf"})
ON CREATE SET
  kw.value = "wero qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-wero-qr-code-fr-fr-bc8ddf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-application-qr-code-fr-fr-add1ef"})
ON CREATE SET
  kw.value = "installer application qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-application-qr-code-fr-fr-add1ef"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-netanyahu-qr-code-fr-fr-6eaa85"})
ON CREATE SET
  kw.value = "netanyahu qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-netanyahu-qr-code-fr-fr-6eaa85"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-qr-code-canva-fr-fr-e9d544"})
ON CREATE SET
  kw.value = "générateur qr code canva",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-qr-code-canva-fr-fr-e9d544"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-pour-qr-code-gratuit-fr-fr-6834d4"})
ON CREATE SET
  kw.value = "application pour qr code gratuit",
  kw.volume = 70,
  kw.difficulty = 7,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-pour-qr-code-gratuit-fr-fr-6834d4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-jeu-concours-qr-code-fr-fr-0f8f44"})
ON CREATE SET
  kw.value = "jeu concours qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-jeu-concours-qr-code-fr-fr-0f8f44"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-versailles-fr-fr-0b0ece"})
ON CREATE SET
  kw.value = "qr code versailles",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-versailles-fr-fr-0b0ece"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-fitness-park-qr-code-fr-fr-9d73ae"})
ON CREATE SET
  kw.value = "fitness park qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-fitness-park-qr-code-fr-fr-9d73ae"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-burger-king-fr-fr-2ec15b"})
ON CREATE SET
  kw.value = "qr code burger king",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Commercial,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-burger-king-fr-fr-2ec15b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecture-qr-code-iphone-fr-fr-95a21d"})
ON CREATE SET
  kw.value = "lecture qr code iphone",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecture-qr-code-iphone-fr-fr-95a21d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-en-anglais-fr-fr-3c5221"})
ON CREATE SET
  kw.value = "qr code en anglais",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-en-anglais-fr-fr-3c5221"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-adidas-fr-fr-c8777c"})
ON CREATE SET
  kw.value = "qr code adidas",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-adidas-fr-fr-c8777c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ou-se-trouve-le-qr-code-fr-fr-e48d86"})
ON CREATE SET
  kw.value = "ou se trouve le qr code",
  kw.volume = 70,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ou-se-trouve-le-qr-code-fr-fr-e48d86"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-syded-lot-qr-code-fr-fr-647e58"})
ON CREATE SET
  kw.value = "syded lot qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-syded-lot-qr-code-fr-fr-647e58"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-appareil-photo-fr-fr-641c65"})
ON CREATE SET
  kw.value = "qr code appareil photo",
  kw.volume = 70,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-appareil-photo-fr-fr-641c65"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-texte-fr-fr-a8d39f"})
ON CREATE SET
  kw.value = "qr code texte",
  kw.volume = 70,
  kw.difficulty = 18,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-texte-fr-fr-a8d39f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-واتساب-ويب-qr-code-fr-fr-d692e1"})
ON CREATE SET
  kw.value = "واتساب ويب qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-واتساب-ويب-qr-code-fr-fr-d692e1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-rickroll-qr-code-fr-fr-20486e"})
ON CREATE SET
  kw.value = "rickroll qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-rickroll-qr-code-fr-fr-20486e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-prendre-un-qr-code-en-photo-fr-fr-ab9085"})
ON CREATE SET
  kw.value = "comment prendre un qr code en photo",
  kw.volume = 70,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-prendre-un-qr-code-en-photo-fr-fr-ab9085"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-questionnaire-avec-qr-code-gratuit-fr-fr-05e381"})
ON CREATE SET
  kw.value = "questionnaire avec qr code gratuit",
  kw.volume = 70,
  kw.difficulty = 4,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-questionnaire-avec-qr-code-gratuit-fr-fr-05e381"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-sans-limite-de-temps-fr-fr-f0d82b"})
ON CREATE SET
  kw.value = "qr code gratuit sans limite de temps",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-sans-limite-de-temps-fr-fr-f0d82b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generatir-fr-fr-d43817"})
ON CREATE SET
  kw.value = "qr code generatir",
  kw.volume = 70,
  kw.difficulty = 93,
  kw.cpc = 0.0,
  kw.intent = "Navigational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generatir-fr-fr-d43817"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ordonnance-qr-code-fr-fr-edf213"})
ON CREATE SET
  kw.value = "ordonnance qr code",
  kw.volume = 70,
  kw.difficulty = 6,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ordonnance-qr-code-fr-fr-edf213"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-wwe-supercard-qr-code-fr-fr-a9a52a"})
ON CREATE SET
  kw.value = "wwe supercard qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-wwe-supercard-qr-code-fr-fr-a9a52a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-fonctionnement-qr-code-fr-fr-694a7a"})
ON CREATE SET
  kw.value = "fonctionnement qr code",
  kw.volume = 70,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-fonctionnement-qr-code-fr-fr-694a7a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-retrouver-un-qr-code-sur-samsung-fr-fr-c3374e"})
ON CREATE SET
  kw.value = "comment retrouver un qr code sur samsung",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-retrouver-un-qr-code-sur-samsung-fr-fr-c3374e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-chronopost-pickup-qr-code-ne-fonctionne-pas-fr-fr-edddc9"})
ON CREATE SET
  kw.value = "chronopost pickup qr code ne fonctionne pas",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-chronopost-pickup-qr-code-ne-fonctionne-pas-fr-fr-edddc9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-installer-le-qr-code-fr-fr-79b4b5"})
ON CREATE SET
  kw.value = "comment installer le qr code",
  kw.volume = 70,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-installer-le-qr-code-fr-fr-79b4b5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-logiciel-qr-code-fr-fr-63d726"})
ON CREATE SET
  kw.value = "logiciel qr code",
  kw.volume = 70,
  kw.difficulty = 32,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-logiciel-qr-code-fr-fr-63d726"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-gratuit-de-qr-code-fr-fr-420a0d"})
ON CREATE SET
  kw.value = "générateur gratuit de qr code",
  kw.volume = 70,
  kw.difficulty = 34,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-gratuit-de-qr-code-fr-fr-420a0d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sticker-fr-fr-fdd16a"})
ON CREATE SET
  kw.value = "qr code sticker",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sticker-fr-fr-fdd16a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-cree-qr-code-fr-fr-0d346e"})
ON CREATE SET
  kw.value = "cree qr code",
  kw.volume = 70,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-cree-qr-code-fr-fr-0d346e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-photo-mariage-qr-code-fr-fr-b494b2"})
ON CREATE SET
  kw.value = "application photo mariage qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-photo-mariage-qr-code-fr-fr-b494b2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-esim-red-sfr-fr-fr-553c77"})
ON CREATE SET
  kw.value = "qr code esim red sfr",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-esim-red-sfr-fr-fr-553c77"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-généré-un-qr-code-fr-fr-068e6f"})
ON CREATE SET
  kw.value = "généré un qr code",
  kw.volume = 70,
  kw.difficulty = 33,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-généré-un-qr-code-fr-fr-068e6f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-paypal-qr-code-fr-fr-b6c7f2"})
ON CREATE SET
  kw.value = "paypal qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-paypal-qr-code-fr-fr-b6c7f2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ouvrir-qr-code-sur-pc-fr-fr-805516"})
ON CREATE SET
  kw.value = "ouvrir qr code sur pc",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ouvrir-qr-code-sur-pc-fr-fr-805516"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-qr-gratuit-fr-fr-2bbfe9"})
ON CREATE SET
  kw.value = "qr qr gratuit",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-qr-gratuit-fr-fr-2bbfe9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-de-qr-code-canva-fr-fr-b9bbe3"})
ON CREATE SET
  kw.value = "générateur de qr code canva",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-de-qr-code-canva-fr-fr-b9bbe3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mii-3ds-fr-fr-97c2bf"})
ON CREATE SET
  kw.value = "qr code mii 3ds",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mii-3ds-fr-fr-97c2bf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ins-non-signé-fr-fr-36f4f1"})
ON CREATE SET
  kw.value = "qr code ins non signé",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ins-non-signé-fr-fr-36f4f1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-beyblade-qr-code-fr-fr-890ad5"})
ON CREATE SET
  kw.value = "beyblade qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-beyblade-qr-code-fr-fr-890ad5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-affiche-fr-fr-99edb1"})
ON CREATE SET
  kw.value = "qr code affiche",
  kw.volume = 70,
  kw.difficulty = 16,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-affiche-fr-fr-99edb1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-hay-day-qr-code-fr-fr-866849"})
ON CREATE SET
  kw.value = "hay day qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-hay-day-qr-code-fr-fr-866849"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-orange-fr-fr-bc63e8"})
ON CREATE SET
  kw.value = "qr code orange",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-orange-fr-fr-bc63e8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-authenticator-qr-code-fr-fr-ea3294"})
ON CREATE SET
  kw.value = "authenticator qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-authenticator-qr-code-fr-fr-ea3294"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-mettre-un-qr-code-fr-fr-6f5e3f"})
ON CREATE SET
  kw.value = "comment mettre un qr code",
  kw.volume = 60,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-mettre-un-qr-code-fr-fr-6f5e3f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-magearna-fr-fr-c31db1"})
ON CREATE SET
  kw.value = "qr code magearna",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-magearna-fr-fr-c31db1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-google-lens-fr-fr-000c22"})
ON CREATE SET
  kw.value = "qr code google lens",
  kw.volume = 60,
  kw.difficulty = 5,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-google-lens-fr-fr-000c22"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-tui-fr-fr-af3231"})
ON CREATE SET
  kw.value = "qr code tui",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-tui-fr-fr-af3231"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-singe-doigt-dhonneur-fr-fr-398081"})
ON CREATE SET
  kw.value = "qr code singe doigt d\'honneur",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-singe-doigt-dhonneur-fr-fr-398081"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-tomodachi-life-mii-qr-code-fr-fr-fdd6e0"})
ON CREATE SET
  kw.value = "tomodachi life mii qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-tomodachi-life-mii-qr-code-fr-fr-fdd6e0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-japan-douane-fr-fr-b73903"})
ON CREATE SET
  kw.value = "qr code japan douane",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-japan-douane-fr-fr-b73903"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-un-qr-code-gratuit-fr-fr-a46a73"})
ON CREATE SET
  kw.value = "télécharger un qr code gratuit",
  kw.volume = 60,
  kw.difficulty = 8,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-un-qr-code-gratuit-fr-fr-a46a73"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-à-quoi-sert-le-qr-code-fr-fr-92edb2"})
ON CREATE SET
  kw.value = "à quoi sert le qr code",
  kw.volume = 60,
  kw.difficulty = 9,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-à-quoi-sert-le-qr-code-fr-fr-92edb2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-huawei-fr-fr-fd9da4"})
ON CREATE SET
  kw.value = "qr code huawei",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-huawei-fr-fr-fd9da4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-qr-code-android-fr-fr-d6ae65"})
ON CREATE SET
  kw.value = "application qr code android",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-qr-code-android-fr-fr-d6ae65"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-piece-clinquante-fr-fr-9a7e04"})
ON CREATE SET
  kw.value = "qr code yo kai watch 2 piece clinquante",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-piece-clinquante-fr-fr-9a7e04"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-yo-kai-watch-1-qr-code-fr-fr-ae4bed"})
ON CREATE SET
  kw.value = "yo kai watch 1 qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-yo-kai-watch-1-qr-code-fr-fr-ae4bed"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-relais-colis-fr-fr-0f8bda"})
ON CREATE SET
  kw.value = "qr code relais colis",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-relais-colis-fr-fr-0f8bda"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-drole-fr-fr-7bd1ce"})
ON CREATE SET
  kw.value = "qr code drole",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-drole-fr-fr-7bd1ce"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-photos-mariage-fr-fr-417b73"})
ON CREATE SET
  kw.value = "qr code photos mariage",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-photos-mariage-fr-fr-417b73"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-api-fr-fr-4ba094"})
ON CREATE SET
  kw.value = "qr code api",
  kw.volume = 60,
  kw.difficulty = 5,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-api-fr-fr-4ba094"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vin-réglementation-fr-fr-359a48"})
ON CREATE SET
  kw.value = "qr code vin réglementation",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vin-réglementation-fr-fr-359a48"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-digital-qr-code-fr-fr-f371d4"})
ON CREATE SET
  kw.value = "digital qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-digital-qr-code-fr-fr-f371d4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-netflix-fr-fr-5973b0"})
ON CREATE SET
  kw.value = "qr code netflix",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-netflix-fr-fr-5973b0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-icone-qr-code-fr-fr-ea3fa1"})
ON CREATE SET
  kw.value = "icone qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-icone-qr-code-fr-fr-ea3fa1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vectoriel-fr-fr-e501fc"})
ON CREATE SET
  kw.value = "qr code vectoriel",
  kw.volume = 60,
  kw.difficulty = 14,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vectoriel-fr-fr-e501fc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-bois-fr-fr-de9d43"})
ON CREATE SET
  kw.value = "qr code bois",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-bois-fr-fr-de9d43"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-gratuit-qr-code-fr-fr-b301bc"})
ON CREATE SET
  kw.value = "générateur gratuit qr code",
  kw.volume = 60,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-gratuit-qr-code-fr-fr-b301bc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lens-qr-code-fr-fr-74b787"})
ON CREATE SET
  kw.value = "lens qr code",
  kw.volume = 60,
  kw.difficulty = 5,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lens-qr-code-fr-fr-74b787"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-japan-qr-code-fr-fr-1fe30d"})
ON CREATE SET
  kw.value = "japan qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-japan-qr-code-fr-fr-1fe30d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-caméra-ne-reconnaît-pas-le-qr-code-fr-fr-e1b500"})
ON CREATE SET
  kw.value = "caméra ne reconnaît pas le qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-caméra-ne-reconnaît-pas-le-qr-code-fr-fr-e1b500"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-installer-qr-code-fr-fr-4e43c7"})
ON CREATE SET
  kw.value = "comment installer qr code",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-installer-qr-code-fr-fr-4e43c7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-esim-qr-code-fr-fr-58ffd0"})
ON CREATE SET
  kw.value = "esim qr code",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-esim-qr-code-fr-fr-58ffd0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-quest-ce-quun-qr-code-fr-fr-d10e1c"})
ON CREATE SET
  kw.value = "qu\'est-ce qu\'un qr code",
  kw.volume = 60,
  kw.difficulty = 12,
  kw.cpc = 0.05,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-quest-ce-quun-qr-code-fr-fr-d10e1c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-resultat-loto-qr-code-fr-fr-3f2d8a"})
ON CREATE SET
  kw.value = "resultat loto qr code",
  kw.volume = 60,
  kw.difficulty = 36,
  kw.cpc = 1.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-resultat-loto-qr-code-fr-fr-3f2d8a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mp3-gratuit-fr-fr-0b66b0"})
ON CREATE SET
  kw.value = "qr code mp3 gratuit",
  kw.volume = 60,
  kw.difficulty = 7,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mp3-gratuit-fr-fr-0b66b0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-comment-faire-fr-fr-a2c85c"})
ON CREATE SET
  kw.value = "qr code comment faire",
  kw.volume = 60,
  kw.difficulty = 12,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-comment-faire-fr-fr-a2c85c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-linktree-fr-fr-e1c973"})
ON CREATE SET
  kw.value = "qr code linktree",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-linktree-fr-fr-e1c973"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-activer-esim-sans-qr-code-fr-fr-13e398"})
ON CREATE SET
  kw.value = "activer esim sans qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-activer-esim-sans-qr-code-fr-fr-13e398"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-taille-qr-code-fr-fr-a592b0"})
ON CREATE SET
  kw.value = "taille qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-taille-qr-code-fr-fr-a592b0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-piece-frisson-lune-fr-fr-649673"})
ON CREATE SET
  kw.value = "qr code yo kai watch 2 piece frisson lune",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-piece-frisson-lune-fr-fr-649673"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-funéraire-fr-fr-d13ee3"})
ON CREATE SET
  kw.value = "qr code funéraire",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-funéraire-fr-fr-d13ee3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-png-blanc-fr-fr-e4258f"})
ON CREATE SET
  kw.value = "qr code png blanc",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-png-blanc-fr-fr-e4258f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-factice-fr-fr-0ec936"})
ON CREATE SET
  kw.value = "qr code factice",
  kw.volume = 60,
  kw.difficulty = 14,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-factice-fr-fr-0ec936"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-jeu-qr-code-fr-fr-317adc"})
ON CREATE SET
  kw.value = "jeu qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-jeu-qr-code-fr-fr-317adc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vector-fr-fr-585f8c"})
ON CREATE SET
  kw.value = "qr code vector",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 0.9,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vector-fr-fr-585f8c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-new-balance-fr-fr-ea0c43"})
ON CREATE SET
  kw.value = "qr code new balance",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-new-balance-fr-fr-ea0c43"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-téléchargement-qr-code-fr-fr-c808a4"})
ON CREATE SET
  kw.value = "téléchargement qr code",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-téléchargement-qr-code-fr-fr-c808a4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lien-en-qr-code-gratuit-fr-fr-26ac13"})
ON CREATE SET
  kw.value = "lien en qr code gratuit",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lien-en-qr-code-gratuit-fr-fr-26ac13"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-installer-un-qr-code-gratuit-fr-fr-9acb0e"})
ON CREATE SET
  kw.value = "comment installer un qr code gratuit",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-installer-un-qr-code-gratuit-fr-fr-9acb0e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuits-fr-fr-dbba5a"})
ON CREATE SET
  kw.value = "qr code gratuits",
  kw.volume = 60,
  kw.difficulty = 35,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuits-fr-fr-dbba5a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-avec-lien-fr-fr-f679e3"})
ON CREATE SET
  kw.value = "qr code avec lien",
  kw.volume = 60,
  kw.difficulty = 21,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-avec-lien-fr-fr-f679e3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-inpost-qr-code-fr-fr-ebcee1"})
ON CREATE SET
  kw.value = "inpost qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-inpost-qr-code-fr-fr-ebcee1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mail-fr-fr-125b96"})
ON CREATE SET
  kw.value = "qr code mail",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mail-fr-fr-125b96"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-generate-fr-fr-5aa140"})
ON CREATE SET
  kw.value = "qr code generate",
  kw.volume = 60,
  kw.difficulty = 93,
  kw.cpc = 0.0,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-generate-fr-fr-5aa140"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-dessin-fr-fr-75fb8c"})
ON CREATE SET
  kw.value = "qr code dessin",
  kw.volume = 60,
  kw.difficulty = 10,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-dessin-fr-fr-75fb8c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-clash-of-clans-fr-fr-bb7540"})
ON CREATE SET
  kw.value = "qr code clash of clans",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-clash-of-clans-fr-fr-bb7540"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-faux-fr-fr-02dbe3"})
ON CREATE SET
  kw.value = "qr code faux",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-faux-fr-fr-02dbe3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-syded-du-lot-qr-code-fr-fr-ec711f"})
ON CREATE SET
  kw.value = "syded du lot qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-syded-du-lot-qr-code-fr-fr-ec711f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-gs1-qr-code-fr-fr-c5924f"})
ON CREATE SET
  kw.value = "gs1 qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-gs1-qr-code-fr-fr-c5924f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-photo-avec-qr-code-fr-fr-a60a26"})
ON CREATE SET
  kw.value = "photo avec qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-photo-avec-qr-code-fr-fr-a60a26"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-stylisé-fr-fr-f46878"})
ON CREATE SET
  kw.value = "qr code stylisé",
  kw.volume = 60,
  kw.difficulty = 31,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-stylisé-fr-fr-f46878"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-piece-special-fr-fr-7c2a08"})
ON CREATE SET
  kw.value = "qr code yo kai watch 2 piece special",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-piece-special-fr-fr-7c2a08"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-stone-island-fr-fr-02c094"})
ON CREATE SET
  kw.value = "qr code stone island",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-stone-island-fr-fr-02c094"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creation-qr-code-lien-fr-fr-f1fad5"})
ON CREATE SET
  kw.value = "creation qr code lien",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creation-qr-code-lien-fr-fr-f1fad5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-retour-mondial-relay-qr-code-fr-fr-84e740"})
ON CREATE SET
  kw.value = "retour mondial relay qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-retour-mondial-relay-qr-code-fr-fr-84e740"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-prendre-en-photo-un-qr-code-fr-fr-07f9fd"})
ON CREATE SET
  kw.value = "comment prendre en photo un qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-prendre-en-photo-un-qr-code-fr-fr-07f9fd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-lecture-fr-fr-12b11e"})
ON CREATE SET
  kw.value = "qr code lecture",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-lecture-fr-fr-12b11e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vinted-go-fr-fr-c82eee"})
ON CREATE SET
  kw.value = "qr code vinted go",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vinted-go-fr-fr-c82eee"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-composter-un-billet-de-train-qr-code-fr-fr-7c1317"})
ON CREATE SET
  kw.value = "comment composter un billet de train qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-composter-un-billet-de-train-qr-code-fr-fr-7c1317"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-photos-qr-code-fr-fr-a131c2"})
ON CREATE SET
  kw.value = "photos qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-photos-qr-code-fr-fr-a131c2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-exemple-affiche-qr-code-fr-fr-31a00c"})
ON CREATE SET
  kw.value = "exemple affiche qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-exemple-affiche-qr-code-fr-fr-31a00c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-acheter-un-qr-code-fr-fr-18d44a"})
ON CREATE SET
  kw.value = "acheter un qr code",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 0.5,
  kw.intent = "Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-acheter-un-qr-code-fr-fr-18d44a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-commande-qr-code-fr-fr-27e7df"})
ON CREATE SET
  kw.value = "commande qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-commande-qr-code-fr-fr-27e7df"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-créé-un-qr-code-fr-fr-85371e"})
ON CREATE SET
  kw.value = "créé un qr code",
  kw.volume = 60,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-créé-un-qr-code-fr-fr-85371e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-obtenir-un-qr-code-fr-fr-8cab80"})
ON CREATE SET
  kw.value = "comment obtenir un qr code",
  kw.volume = 60,
  kw.difficulty = 8,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-obtenir-un-qr-code-fr-fr-8cab80"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-historique-qr-code-iphone-fr-fr-19c0ea"})
ON CREATE SET
  kw.value = "historique qr code iphone",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-historique-qr-code-iphone-fr-fr-19c0ea"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-maxi-blackjack-qr-code-fr-fr-421f61"})
ON CREATE SET
  kw.value = "maxi blackjack qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-maxi-blackjack-qr-code-fr-fr-421f61"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-google-authenticator-fr-fr-0d0602"})
ON CREATE SET
  kw.value = "qr code google authenticator",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-google-authenticator-fr-fr-0d0602"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-crée-un-qr-code-fr-fr-ac1080"})
ON CREATE SET
  kw.value = "comment crée un qr code",
  kw.volume = 60,
  kw.difficulty = 22,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-crée-un-qr-code-fr-fr-ac1080"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-beyblade-burst-qr-code-fr-fr-22c89e"})
ON CREATE SET
  kw.value = "beyblade burst qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-beyblade-burst-qr-code-fr-fr-22c89e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-qr-code-gratuit-sans-inscription-fr-fr-63a63c"})
ON CREATE SET
  kw.value = "générateur qr code gratuit sans inscription",
  kw.volume = 60,
  kw.difficulty = 24,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-qr-code-gratuit-sans-inscription-fr-fr-63a63c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-tshirt-qr-code-fr-fr-057c9f"})
ON CREATE SET
  kw.value = "tshirt qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-tshirt-qr-code-fr-fr-057c9f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-tomodachi-life-qr-code-fr-fr-93d927"})
ON CREATE SET
  kw.value = "tomodachi life qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-tomodachi-life-qr-code-fr-fr-93d927"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-generate-a-qr-code-fr-fr-9350ca"})
ON CREATE SET
  kw.value = "how to generate a qr code",
  kw.volume = 60,
  kw.difficulty = 92,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-generate-a-qr-code-fr-fr-9350ca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-impossible-de-connecter-caméra-xiaomi-qr-code-fr-fr-90188a"})
ON CREATE SET
  kw.value = "impossible de connecter caméra xiaomi qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-impossible-de-connecter-caméra-xiaomi-qr-code-fr-fr-90188a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-génerer-un-qr-code-fr-fr-3df2db"})
ON CREATE SET
  kw.value = "génerer un qr code",
  kw.volume = 60,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-génerer-un-qr-code-fr-fr-3df2db"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qui-a-inventé-le-qr-code-fr-fr-aff7fd"})
ON CREATE SET
  kw.value = "qui a inventé le qr code",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qui-a-inventé-le-qr-code-fr-fr-aff7fd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-icon-fr-fr-8c7d7e"})
ON CREATE SET
  kw.value = "qr code icon",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-icon-fr-fr-8c7d7e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-amazon-fr-fr-d61fa0"})
ON CREATE SET
  kw.value = "qr code amazon",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-amazon-fr-fr-d61fa0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-transformer-un-lien-en-qr-code-gratuit-fr-fr-eaade3"})
ON CREATE SET
  kw.value = "transformer un lien en qr code gratuit",
  kw.volume = 60,
  kw.difficulty = 32,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-transformer-un-lien-en-qr-code-gratuit-fr-fr-eaade3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-de-qr-code-gratuit-sans-inscription-fr-fr-1acaa9"})
ON CREATE SET
  kw.value = "générateur de qr code gratuit sans inscription",
  kw.volume = 50,
  kw.difficulty = 22,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-de-qr-code-gratuit-sans-inscription-fr-fr-1acaa9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-inventaire-par-qr-code-fr-fr-0063f4"})
ON CREATE SET
  kw.value = "inventaire par qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-inventaire-par-qr-code-fr-fr-0063f4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-simple-fr-fr-6e0cbe"})
ON CREATE SET
  kw.value = "qr code simple",
  kw.volume = 50,
  kw.difficulty = 34,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-simple-fr-fr-6e0cbe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-colissimo-fr-fr-f8caad"})
ON CREATE SET
  kw.value = "qr code colissimo",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-colissimo-fr-fr-f8caad"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-3d-fr-fr-cd7b16"})
ON CREATE SET
  kw.value = "qr code 3d",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-3d-fr-fr-cd7b16"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-wwwakamsphonelinkqrc-qr-code-windows-10-fr-fr-385966"})
ON CREATE SET
  kw.value = "www.aka.ms/phonelinkqrc qr code windows 10",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-wwwakamsphonelinkqrc-qr-code-windows-10-fr-fr-385966"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-transformer-une-vidéo-en-qr-code-gratuit-fr-fr-3df296"})
ON CREATE SET
  kw.value = "transformer une vidéo en qr code gratuit",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-transformer-une-vidéo-en-qr-code-gratuit-fr-fr-3df296"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-make-qr-code-fr-fr-1fc500"})
ON CREATE SET
  kw.value = "how to make qr code",
  kw.volume = 50,
  kw.difficulty = 90,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-make-qr-code-fr-fr-1fc500"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-mariage-gratuit-fr-fr-123007"})
ON CREATE SET
  kw.value = "qr code mariage gratuit",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-mariage-gratuit-fr-fr-123007"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-kodawari-ramen-qr-code-fr-fr-d41f61"})
ON CREATE SET
  kw.value = "kodawari ramen qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-kodawari-ramen-qr-code-fr-fr-d41f61"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-maken-fr-fr-2d4e0b"})
ON CREATE SET
  kw.value = "qr code maken",
  kw.volume = 50,
  kw.difficulty = 81,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-maken-fr-fr-2d4e0b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-g2n2rateur-qr-code-fr-fr-4f2d45"})
ON CREATE SET
  kw.value = "g2n2rateur qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-g2n2rateur-qr-code-fr-fr-4f2d45"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-firefoxcompair-qr-code-fr-fr-653439"})
ON CREATE SET
  kw.value = "firefox.com/pair qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-firefoxcompair-qr-code-fr-fr-653439"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lego-qr-code-fr-fr-a828a0"})
ON CREATE SET
  kw.value = "lego qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lego-qr-code-fr-fr-a828a0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-enregistrer-un-qr-code-sur-samsung-fr-fr-ee6530"})
ON CREATE SET
  kw.value = "comment enregistrer un qr code sur samsung",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-enregistrer-un-qr-code-sur-samsung-fr-fr-ee6530"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-récupérer-un-qr-code-fr-fr-5b0884"})
ON CREATE SET
  kw.value = "comment récupérer un qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-récupérer-un-qr-code-fr-fr-5b0884"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-genrateur-de-qr-code-fr-fr-3e2e79"})
ON CREATE SET
  kw.value = "genrateur de qr code",
  kw.volume = 50,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-genrateur-de-qr-code-fr-fr-3e2e79"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-utiliser-le-qr-code-sur-samsung-fr-fr-665d0d"})
ON CREATE SET
  kw.value = "comment utiliser le qr code sur samsung",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-utiliser-le-qr-code-sur-samsung-fr-fr-665d0d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-analyse-qr-code-fr-fr-a3e161"})
ON CREATE SET
  kw.value = "analyse qr code",
  kw.volume = 50,
  kw.difficulty = 3,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-analyse-qr-code-fr-fr-a3e161"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-tablette-samsung-fr-fr-de1bd9"})
ON CREATE SET
  kw.value = "qr code tablette samsung",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-tablette-samsung-fr-fr-de1bd9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sans-pub-fr-fr-66c196"})
ON CREATE SET
  kw.value = "qr code sans pub",
  kw.volume = 50,
  kw.difficulty = 20,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sans-pub-fr-fr-66c196"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-vérifier-loto-qr-code-fr-fr-014897"})
ON CREATE SET
  kw.value = "vérifier loto qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-vérifier-loto-qr-code-fr-fr-014897"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-micro-qr-code-fr-fr-46db94"})
ON CREATE SET
  kw.value = "micro qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-micro-qr-code-fr-fr-46db94"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faire-son-qr-code-fr-fr-f07df1"})
ON CREATE SET
  kw.value = "faire son qr code",
  kw.volume = 50,
  kw.difficulty = 33,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faire-son-qr-code-fr-fr-f07df1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-pingid-qr-code-fr-fr-647961"})
ON CREATE SET
  kw.value = "pingid qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-pingid-qr-code-fr-fr-647961"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-evenement-fr-fr-5a779c"})
ON CREATE SET
  kw.value = "qr code evenement",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-evenement-fr-fr-5a779c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-étiquette-qr-code-fr-fr-648583"})
ON CREATE SET
  kw.value = "étiquette qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.45,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-étiquette-qr-code-fr-fr-648583"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-jo-fr-fr-03e35d"})
ON CREATE SET
  kw.value = "qr code jo",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 1.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-jo-fr-fr-03e35d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-jeux-3ds-fr-fr-f96948"})
ON CREATE SET
  kw.value = "qr code jeux 3ds",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-jeux-3ds-fr-fr-f96948"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sur-pc-fr-fr-ef0a24"})
ON CREATE SET
  kw.value = "qr code sur pc",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sur-pc-fr-fr-ef0a24"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-sample-qr-code-fr-fr-2e8f20"})
ON CREATE SET
  kw.value = "sample qr code",
  kw.volume = 50,
  kw.difficulty = 34,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-sample-qr-code-fr-fr-2e8f20"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-autocollants-qr-code-fr-fr-dc9933"})
ON CREATE SET
  kw.value = "autocollants qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-autocollants-qr-code-fr-fr-dc9933"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-générateur-de-qr-code-en-ligne-fr-fr-760004"})
ON CREATE SET
  kw.value = "générateur de qr code en ligne",
  kw.volume = 50,
  kw.difficulty = 40,
  kw.cpc = 0.3,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-générateur-de-qr-code-en-ligne-fr-fr-760004"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-modifiable-fr-fr-47528d"})
ON CREATE SET
  kw.value = "qr code modifiable",
  kw.volume = 50,
  kw.difficulty = 47,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-modifiable-fr-fr-47528d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-réaliser-un-qr-code-fr-fr-b9db19"})
ON CREATE SET
  kw.value = "réaliser un qr code",
  kw.volume = 50,
  kw.difficulty = 22,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-réaliser-un-qr-code-fr-fr-b9db19"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-blasters-fr-fr-e935c5"})
ON CREATE SET
  kw.value = "qr code yo kai watch blasters",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-blasters-fr-fr-e935c5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-3ds-fr-fr-9c2fa9"})
ON CREATE SET
  kw.value = "qr code 3ds",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-3ds-fr-fr-9c2fa9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-dpd-fr-fr-9814aa"})
ON CREATE SET
  kw.value = "qr code dpd",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-dpd-fr-fr-9814aa"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-quiz-avec-qr-code-fr-fr-0dec64"})
ON CREATE SET
  kw.value = "quiz avec qr code",
  kw.volume = 50,
  kw.difficulty = 4,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-quiz-avec-qr-code-fr-fr-0dec64"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-généré-qr-code-fr-fr-3c2772"})
ON CREATE SET
  kw.value = "généré qr code",
  kw.volume = 50,
  kw.difficulty = 31,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-généré-qr-code-fr-fr-3c2772"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-image-to-qr-code-fr-fr-648f15"})
ON CREATE SET
  kw.value = "image to qr code",
  kw.volume = 50,
  kw.difficulty = 10,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-image-to-qr-code-fr-fr-648f15"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-beyblade-fr-fr-33fe50"})
ON CREATE SET
  kw.value = "qr code beyblade",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-beyblade-fr-fr-33fe50"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ou-est-le-qr-code-sur-samsung-fr-fr-1846da"})
ON CREATE SET
  kw.value = "ou est le qr code sur samsung",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ou-est-le-qr-code-sur-samsung-fr-fr-1846da"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-monkey-avis-fr-fr-a7b74a"})
ON CREATE SET
  kw.value = "qr code monkey avis",
  kw.volume = 50,
  kw.difficulty = 2,
  kw.cpc = 0.09,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-monkey-avis-fr-fr-a7b74a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-kbis-avec-qr-code-fr-fr-8faa78"})
ON CREATE SET
  kw.value = "kbis avec qr code",
  kw.volume = 50,
  kw.difficulty = 31,
  kw.cpc = 2.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-kbis-avec-qr-code-fr-fr-8faa78"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-contact-téléphone-fr-fr-8a3ad1"})
ON CREATE SET
  kw.value = "qr code contact téléphone",
  kw.volume = 50,
  kw.difficulty = 6,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-contact-téléphone-fr-fr-8a3ad1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-acnl-fr-fr-5740ed"})
ON CREATE SET
  kw.value = "qr code acnl",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-acnl-fr-fr-5740ed"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-online-qr-code-fr-fr-5fd089"})
ON CREATE SET
  kw.value = "online qr code",
  kw.volume = 50,
  kw.difficulty = 58,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-online-qr-code-fr-fr-5fd089"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-io-fr-fr-5a6cc8"})
ON CREATE SET
  kw.value = "qr code .io",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-io-fr-fr-5a6cc8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-le-grand-quiz-qr-code-fr-fr-172edb"})
ON CREATE SET
  kw.value = "le grand quiz qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-le-grand-quiz-qr-code-fr-fr-172edb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-quinté-fr-fr-1b92e1"})
ON CREATE SET
  kw.value = "qr code quinté",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 1.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-quinté-fr-fr-1b92e1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faux-qr-code-png-fr-fr-68aa15"})
ON CREATE SET
  kw.value = "faux qr code png",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faux-qr-code-png-fr-fr-68aa15"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-g2n2rer-un-qr-code-fr-fr-632766"})
ON CREATE SET
  kw.value = "g2n2rer un qr code",
  kw.volume = 50,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-g2n2rer-un-qr-code-fr-fr-632766"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faire-qr-code-avec-lien-fr-fr-9a5ac7"})
ON CREATE SET
  kw.value = "faire qr code avec lien",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faire-qr-code-avec-lien-fr-fr-9a5ac7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-cigarette-fr-fr-3ab9bc"})
ON CREATE SET
  kw.value = "qr code cigarette",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-cigarette-fr-fr-3ab9bc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-mini-qr-code-fr-fr-14786a"})
ON CREATE SET
  kw.value = "mini qr code",
  kw.volume = 50,
  kw.difficulty = 29,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-mini-qr-code-fr-fr-14786a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-criar-qr-code-fr-fr-5e9ac9"})
ON CREATE SET
  kw.value = "criar qr code",
  kw.volume = 50,
  kw.difficulty = 67,
  kw.cpc = 0.35,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-criar-qr-code-fr-fr-5e9ac9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-dragon-ball-legends-qr-code-chrono-crystals-fr-fr-c36d6f"})
ON CREATE SET
  kw.value = "dragon ball legends qr code chrono crystals",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-dragon-ball-legends-qr-code-chrono-crystals-fr-fr-c36d6f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-bs-2025-fr-fr-e6dfaf"})
ON CREATE SET
  kw.value = "qr code bs 2025",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-bs-2025-fr-fr-e6dfaf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-philippines-fr-fr-f55053"})
ON CREATE SET
  kw.value = "qr code philippines",
  kw.volume = 50,
  kw.difficulty = 30,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-philippines-fr-fr-f55053"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ساخت-qr-code-fr-fr-f5b925"})
ON CREATE SET
  kw.value = "ساخت qr code",
  kw.volume = 50,
  kw.difficulty = 26,
  kw.cpc = 0.1,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ساخت-qr-code-fr-fr-f5b925"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-netflix-qr-code-fr-fr-db3fab"})
ON CREATE SET
  kw.value = "netflix qr code",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-netflix-qr-code-fr-fr-db3fab"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-rickroll-fr-fr-e9bed6"})
ON CREATE SET
  kw.value = "qr code rickroll",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-rickroll-fr-fr-e9bed6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vierge-fr-fr-457cdd"})
ON CREATE SET
  kw.value = "qr code vierge",
  kw.volume = 50,
  kw.difficulty = 36,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vierge-fr-fr-457cdd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-exemple-exemple-qr-code-fr-fr-1abeba"})
ON CREATE SET
  kw.value = "exemple exemple qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-exemple-exemple-qr-code-fr-fr-1abeba"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-black-mirror-qr-code-fr-fr-f1ca90"})
ON CREATE SET
  kw.value = "black mirror qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-black-mirror-qr-code-fr-fr-f1ca90"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-fc-mobile-fr-fr-b24598"})
ON CREATE SET
  kw.value = "qr code fc mobile",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-fc-mobile-fr-fr-b24598"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-la-poste-qr-code-fr-fr-83aac3"})
ON CREATE SET
  kw.value = "la poste qr code",
  kw.volume = 50,
  kw.difficulty = 5,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-la-poste-qr-code-fr-fr-83aac3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ouvrir-qr-code-photo-fr-fr-ed856f"})
ON CREATE SET
  kw.value = "ouvrir qr code photo",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ouvrir-qr-code-photo-fr-fr-ed856f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-https-firefoxcompair-qr-code-fr-fr-36a897"})
ON CREATE SET
  kw.value = "https //firefox.com/pair qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-https-firefoxcompair-qr-code-fr-fr-36a897"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ouvrir-un-qr-code-en-photo-fr-fr-c54f8b"})
ON CREATE SET
  kw.value = "ouvrir un qr code en photo",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ouvrir-un-qr-code-en-photo-fr-fr-c54f8b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-from-image-fr-fr-b3df00"})
ON CREATE SET
  kw.value = "qr code from image",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-from-image-fr-fr-b3df00"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-française-des-jeux-fr-fr-225aee"})
ON CREATE SET
  kw.value = "qr code française des jeux",
  kw.volume = 50,
  kw.difficulty = 15,
  kw.cpc = 0.25,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-française-des-jeux-fr-fr-225aee"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sur-photo-fr-fr-ef3202"})
ON CREATE SET
  kw.value = "qr code sur photo",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sur-photo-fr-fr-ef3202"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-transformer-une-photo-en-qr-code-gratuit-fr-fr-31441e"})
ON CREATE SET
  kw.value = "transformer une photo en qr code gratuit",
  kw.volume = 50,
  kw.difficulty = 11,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-transformer-une-photo-en-qr-code-gratuit-fr-fr-31441e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-visite-virtuelle-qr-code-fr-fr-f9ef53"})
ON CREATE SET
  kw.value = "visite virtuelle qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-visite-virtuelle-qr-code-fr-fr-f9ef53"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-acnl-qr-code-robe-fr-fr-373d39"})
ON CREATE SET
  kw.value = "acnl qr code robe",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-acnl-qr-code-robe-fr-fr-373d39"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-editer-un-qr-code-fr-fr-86d280"})
ON CREATE SET
  kw.value = "editer un qr code",
  kw.volume = 50,
  kw.difficulty = 34,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-editer-un-qr-code-fr-fr-86d280"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-telecharger-qr-code-fr-fr-3cdb7e"})
ON CREATE SET
  kw.value = "telecharger qr code",
  kw.volume = 50,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-telecharger-qr-code-fr-fr-3cdb7e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-etiquette-vin-fr-fr-d04cc0"})
ON CREATE SET
  kw.value = "qr code etiquette vin",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-etiquette-vin-fr-fr-d04cc0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-mettre-un-lien-en-qr-code-fr-fr-328bee"})
ON CREATE SET
  kw.value = "mettre un lien en qr code",
  kw.volume = 50,
  kw.difficulty = 29,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-mettre-un-lien-en-qr-code-fr-fr-328bee"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-livre-fr-fr-e46db8"})
ON CREATE SET
  kw.value = "qr code livre",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-livre-fr-fr-e46db8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-piece-5-etoile-fr-fr-e025d7"})
ON CREATE SET
  kw.value = "qr code yo kai watch 2 piece 5 etoile",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-yo-kai-watch-2-piece-5-etoile-fr-fr-e025d7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creation-qr-code-avis-google-fr-fr-126506"})
ON CREATE SET
  kw.value = "creation qr code avis google",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creation-qr-code-avis-google-fr-fr-126506"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-photomaton-qr-code-fr-fr-3d07c2"})
ON CREATE SET
  kw.value = "photomaton qr code",
  kw.volume = 50,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-photomaton-qr-code-fr-fr-3d07c2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ou-est-le-qr-code-xiaomi-fr-fr-b17dc5"})
ON CREATE SET
  kw.value = "ou est le qr code xiaomi",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ou-est-le-qr-code-xiaomi-fr-fr-b17dc5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ne-fonctionne-pas-fr-fr-a0b4e5"})
ON CREATE SET
  kw.value = "qr code ne fonctionne pas",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ne-fonctionne-pas-fr-fr-a0b4e5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-ouvrir-un-qr-code-sur-pc-fr-fr-20f1e4"})
ON CREATE SET
  kw.value = "comment ouvrir un qr code sur pc",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-ouvrir-un-qr-code-sur-pc-fr-fr-20f1e4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-paris-fr-fr-2fc302"})
ON CREATE SET
  kw.value = "qr code paris",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-paris-fr-fr-2fc302"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-définition-qr-code-fr-fr-6c3c3d"})
ON CREATE SET
  kw.value = "définition qr code",
  kw.volume = 50,
  kw.difficulty = 7,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-définition-qr-code-fr-fr-6c3c3d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lidl-qr-code-fr-fr-990093"})
ON CREATE SET
  kw.value = "lidl qr code",
  kw.volume = 50,
  kw.difficulty = 6,
  kw.cpc = 0.15,
  kw.intent = "Informational,Commercial,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lidl-qr-code-fr-fr-990093"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ugg-fr-fr-0a6970"})
ON CREATE SET
  kw.value = "qr code ugg",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ugg-fr-fr-0a6970"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-panneau-de-chantier-qr-code-fr-fr-885574"})
ON CREATE SET
  kw.value = "panneau de chantier qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-panneau-de-chantier-qr-code-fr-fr-885574"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-free-mobile-fr-fr-9b83f3"})
ON CREATE SET
  kw.value = "qr code free mobile",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-free-mobile-fr-fr-9b83f3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-ou-trouver-qr-code-samsung-fr-fr-0968fe"})
ON CREATE SET
  kw.value = "ou trouver qr code samsung",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-ou-trouver-qr-code-samsung-fr-fr-0968fe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-douchette-qr-code-sans-fil-fr-fr-75d88c"})
ON CREATE SET
  kw.value = "douchette qr code sans fil",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-douchette-qr-code-sans-fil-fr-fr-75d88c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-génération-qr-code-gratuit-fr-fr-3c32a8"})
ON CREATE SET
  kw.value = "génération qr code gratuit",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-génération-qr-code-gratuit-fr-fr-3c32a8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-freebox-fr-fr-cf26ea"})
ON CREATE SET
  kw.value = "qr code freebox",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-freebox-fr-fr-cf26ea"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-avis-fr-fr-0c615c"})
ON CREATE SET
  kw.value = "qr code avis",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-avis-fr-fr-0c615c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-imprimante-étiquette-qr-code-fr-fr-41f525"})
ON CREATE SET
  kw.value = "imprimante étiquette qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-imprimante-étiquette-qr-code-fr-fr-41f525"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creator-qr-code-fr-fr-578ff9"})
ON CREATE SET
  kw.value = "creator qr code",
  kw.volume = 50,
  kw.difficulty = 51,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creator-qr-code-fr-fr-578ff9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-installer-fr-fr-963c5e"})
ON CREATE SET
  kw.value = "qr code installer",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-installer-fr-fr-963c5e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-creat-qr-code-fr-fr-8748d4"})
ON CREATE SET
  kw.value = "creat qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-creat-qr-code-fr-fr-8748d4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-illimité-fr-fr-698f46"})
ON CREATE SET
  kw.value = "qr code illimité",
  kw.volume = 50,
  kw.difficulty = 8,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-illimité-fr-fr-698f46"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generate-qr-code-online-fr-fr-d0a36e"})
ON CREATE SET
  kw.value = "generate qr code online",
  kw.volume = 50,
  kw.difficulty = 91,
  kw.cpc = 0.4,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generate-qr-code-online-fr-fr-d0a36e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-quest-ce-quun-qr-code-fr-fr-0ea0a2"})
ON CREATE SET
  kw.value = "qu\'est ce qu\'un qr code",
  kw.volume = 50,
  kw.difficulty = 12,
  kw.cpc = 0.05,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-quest-ce-quun-qr-code-fr-fr-0ea0a2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-obtenir-un-qr-code-gratuit-fr-fr-42fbcd"})
ON CREATE SET
  kw.value = "obtenir un qr code gratuit",
  kw.volume = 50,
  kw.difficulty = 22,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-obtenir-un-qr-code-gratuit-fr-fr-42fbcd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuitement-fr-fr-1f123d"})
ON CREATE SET
  kw.value = "qr code gratuitement",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuitement-fr-fr-1f123d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-wooclap-qr-code-fr-fr-8c4d15"})
ON CREATE SET
  kw.value = "wooclap qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-wooclap-qr-code-fr-fr-8c4d15"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-déchetterie-saint-etienne-fr-fr-4bde34"})
ON CREATE SET
  kw.value = "qr code déchetterie saint etienne",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-déchetterie-saint-etienne-fr-fr-4bde34"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-affiche-qr-code-avis-google-fr-fr-d0eb7c"})
ON CREATE SET
  kw.value = "affiche qr code avis google",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Commercial,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-affiche-qr-code-avis-google-fr-fr-d0eb7c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-type-de-qr-code-fr-fr-ef29aa"})
ON CREATE SET
  kw.value = "type de qr code",
  kw.volume = 50,
  kw.difficulty = 6,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-type-de-qr-code-fr-fr-ef29aa"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-labubu-fr-fr-a03ba1"})
ON CREATE SET
  kw.value = "qr code labubu",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-labubu-fr-fr-a03ba1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-cest-quoi-fr-fr-3105cb"})
ON CREATE SET
  kw.value = "qr code c\'est quoi",
  kw.volume = 50,
  kw.difficulty = 12,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-cest-quoi-fr-fr-3105cb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-open-source-fr-fr-04f9e7"})
ON CREATE SET
  kw.value = "qr code open source",
  kw.volume = 50,
  kw.difficulty = 63,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-open-source-fr-fr-04f9e7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-à-vie-fr-fr-5d090b"})
ON CREATE SET
  kw.value = "qr code gratuit à vie",
  kw.volume = 50,
  kw.difficulty = 23,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-à-vie-fr-fr-5d090b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-taille-minimum-fr-fr-f1a5c8"})
ON CREATE SET
  kw.value = "qr code taille minimum",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 2.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-taille-minimum-fr-fr-f1a5c8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-samsung-galaxy-fr-fr-09a8ae"})
ON CREATE SET
  kw.value = "qr code samsung galaxy",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-samsung-galaxy-fr-fr-09a8ae"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-chevalet-qr-code-fr-fr-fda655"})
ON CREATE SET
  kw.value = "chevalet qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-chevalet-qr-code-fr-fr-fda655"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generation-qr-code-gratuit-fr-fr-922750"})
ON CREATE SET
  kw.value = "generation qr code gratuit",
  kw.volume = 50,
  kw.difficulty = 34,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generation-qr-code-gratuit-fr-fr-922750"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lien-qr-code-gratuit-fr-fr-5e4a70"})
ON CREATE SET
  kw.value = "lien qr code gratuit",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lien-qr-code-gratuit-fr-fr-5e4a70"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-shopify-fr-fr-c450da"})
ON CREATE SET
  kw.value = "qr code shopify",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 1.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-shopify-fr-fr-c450da"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-analyser-fr-fr-17b455"})
ON CREATE SET
  kw.value = "qr code analyser",
  kw.volume = 50,
  kw.difficulty = 7,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-analyser-fr-fr-17b455"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-ordinateur-fr-fr-b77163"})
ON CREATE SET
  kw.value = "qr code ordinateur",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-ordinateur-fr-fr-b77163"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-chronopost-non-reçu-fr-fr-26aef5"})
ON CREATE SET
  kw.value = "qr code chronopost non reçu",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-chronopost-non-reçu-fr-fr-26aef5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-qr-code-fr-fr-547d79"})
ON CREATE SET
  kw.value = "carte de visite qr code",
  kw.volume = 1000,
  kw.difficulty = 0,
  kw.cpc = 1.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-qr-code-fr-fr-547d79"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-carte-de-visite-fr-fr-cf47a2"})
ON CREATE SET
  kw.value = "qr code carte de visite",
  kw.volume = 600,
  kw.difficulty = 0,
  kw.cpc = 1.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-carte-de-visite-fr-fr-cf47a2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-avec-qr-code-fr-fr-6265ce"})
ON CREATE SET
  kw.value = "carte de visite avec qr code",
  kw.volume = 600,
  kw.difficulty = 0,
  kw.cpc = 1.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-avec-qr-code-fr-fr-6265ce"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-code-qr-fr-fr-e47b25"})
ON CREATE SET
  kw.value = "carte de visite code qr",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 1.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-code-qr-fr-fr-e47b25"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faire-un-qr-code-à-partir-dun-lien-fr-fr-056e21"})
ON CREATE SET
  kw.value = "faire un qr code à partir d\'un lien",
  kw.volume = 200,
  kw.difficulty = 34,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faire-un-qr-code-à-partir-dun-lien-fr-fr-056e21"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-visite-avec-qr-code-fr-fr-cf7b02"})
ON CREATE SET
  kw.value = "carte visite avec qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 1.1,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-visite-avec-qr-code-fr-fr-cf7b02"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pour-carte-de-visite-fr-fr-8f3947"})
ON CREATE SET
  kw.value = "qr code pour carte de visite",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pour-carte-de-visite-fr-fr-8f3947"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-virtuelle-qr-code-fr-fr-9ff582"})
ON CREATE SET
  kw.value = "carte de visite virtuelle qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-virtuelle-qr-code-fr-fr-9ff582"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-visite-qr-code-fr-fr-26c9f9"})
ON CREATE SET
  kw.value = "carte visite qr code",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 1.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-visite-qr-code-fr-fr-26c9f9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-avec-code-qr-fr-fr-9f8569"})
ON CREATE SET
  kw.value = "carte de visite avec code qr",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 1.1,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-avec-code-qr-fr-fr-9f8569"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-qr-code-carte-de-visite-fr-fr-c5bdd2"})
ON CREATE SET
  kw.value = "création qr code carte de visite",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-qr-code-carte-de-visite-fr-fr-c5bdd2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-création-qr-code-carte-de-visite-gratuit-fr-fr-579109"})
ON CREATE SET
  kw.value = "création qr code carte de visite gratuit",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-création-qr-code-carte-de-visite-gratuit-fr-fr-579109"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr-sur-carte-de-visite-fr-fr-59e869"})
ON CREATE SET
  kw.value = "code qr sur carte de visite",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr-sur-carte-de-visite-fr-fr-59e869"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-partage-photo-mariage-fr-fr-fccaab"})
ON CREATE SET
  kw.value = "qr code partage photo mariage",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-partage-photo-mariage-fr-fr-fccaab"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-qr-code-fr-fr-82aa20"})
ON CREATE SET
  kw.value = "carte qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.7,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-qr-code-fr-fr-82aa20"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-carte-de-visite-gratuit-fr-fr-3b3032"})
ON CREATE SET
  kw.value = "qr code carte de visite gratuit",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.9,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-carte-de-visite-gratuit-fr-fr-3b3032"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr-carte-de-visite-fr-fr-6af2e6"})
ON CREATE SET
  kw.value = "code qr carte de visite",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr-carte-de-visite-fr-fr-6af2e6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-avec-qr-code-gratuit-fr-fr-f9549e"})
ON CREATE SET
  kw.value = "carte de visite avec qr code gratuit",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-avec-qr-code-gratuit-fr-fr-f9549e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-carte-vitale-fr-fr-708e6f"})
ON CREATE SET
  kw.value = "qr code carte vitale",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-carte-vitale-fr-fr-708e6f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-qr-code-gratuit-fr-fr-e43b9f"})
ON CREATE SET
  kw.value = "carte de visite qr code gratuit",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.9,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-qr-code-gratuit-fr-fr-e43b9f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-carte-visite-gratuit-fr-fr-0059c1"})
ON CREATE SET
  kw.value = "qr code carte visite gratuit",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-carte-visite-gratuit-fr-fr-0059c1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-exemple-carte-de-visite-avec-qr-code-fr-fr-98aa3d"})
ON CREATE SET
  kw.value = "exemple carte de visite avec qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-exemple-carte-de-visite-avec-qr-code-fr-fr-98aa3d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-sur-carte-de-visite-fr-fr-73ee2e"})
ON CREATE SET
  kw.value = "qr code sur carte de visite",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-sur-carte-de-visite-fr-fr-73ee2e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-partage-photo-mariage-qr-code-fr-fr-a582de"})
ON CREATE SET
  kw.value = "partage photo mariage qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-partage-photo-mariage-qr-code-fr-fr-a582de"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-retrouver-un-qr-code-sur-smartphone-fr-fr-ae1eab"})
ON CREATE SET
  kw.value = "comment retrouver un qr code sur smartphone",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-retrouver-un-qr-code-sur-smartphone-fr-fr-ae1eab"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-à-partir-dun-lien-fr-fr-fff876"})
ON CREATE SET
  kw.value = "qr code à partir d\'un lien",
  kw.volume = 90,
  kw.difficulty = 22,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-à-partir-dun-lien-fr-fr-fff876"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-qr-code-gratuit-sur-smartphone-fr-fr-a7c8f7"})
ON CREATE SET
  kw.value = "installer qr code gratuit sur smartphone",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-qr-code-gratuit-sur-smartphone-fr-fr-a7c8f7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-art-fr-fr-832eaa"})
ON CREATE SET
  kw.value = "qr code art",
  kw.volume = 80,
  kw.difficulty = 22,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-art-fr-fr-832eaa"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-pvc-qr-code-fr-fr-3e3c48"})
ON CREATE SET
  kw.value = "carte pvc qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-pvc-qr-code-fr-fr-3e3c48"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-digitale-qr-code-fr-fr-cff770"})
ON CREATE SET
  kw.value = "carte de visite digitale qr code",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.9,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-digitale-qr-code-fr-fr-cff770"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-faire-un-qr-code-carte-de-visite-fr-fr-4aedc1"})
ON CREATE SET
  kw.value = "faire un qr code carte de visite",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-faire-un-qr-code-carte-de-visite-fr-fr-4aedc1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-vitale-qr-code-fr-fr-a3187d"})
ON CREATE SET
  kw.value = "carte vitale qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-vitale-qr-code-fr-fr-a3187d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-restaurant-qr-code-fr-fr-1d604d"})
ON CREATE SET
  kw.value = "carte restaurant qr code",
  kw.volume = 70,
  kw.difficulty = 5,
  kw.cpc = 1.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-restaurant-qr-code-fr-fr-1d604d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-smart-switch-fr-fr-d6b070"})
ON CREATE SET
  kw.value = "qr code smart switch",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-smart-switch-fr-fr-d6b070"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-partage-photo-fr-fr-ce00a7"})
ON CREATE SET
  kw.value = "qr code partage photo",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-partage-photo-fr-fr-ce00a7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-carte-restaurant-fr-fr-6199a1"})
ON CREATE SET
  kw.value = "qr code carte restaurant",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 1.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-carte-restaurant-fr-fr-6199a1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pour-partager-des-photos-gratuit-fr-fr-00e9b4"})
ON CREATE SET
  kw.value = "qr code pour partager des photos gratuit",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pour-partager-des-photos-gratuit-fr-fr-00e9b4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-artistique-fr-fr-5fdb77"})
ON CREATE SET
  kw.value = "qr code artistique",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-artistique-fr-fr-5fdb77"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-carte-lidl-plus-fr-fr-675721"})
ON CREATE SET
  kw.value = "qr code carte lidl plus",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-carte-lidl-plus-fr-fr-675721"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-smart-switch-qr-code-fr-fr-6e3c60"})
ON CREATE SET
  kw.value = "smart switch qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-smart-switch-qr-code-fr-fr-6e3c60"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pour-partager-des-photos-fr-fr-a06a42"})
ON CREATE SET
  kw.value = "qr code pour partager des photos",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pour-partager-des-photos-fr-fr-a06a42"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-carte-de-visite-numérique-qr-code-fr-fr-4f3f01"})
ON CREATE SET
  kw.value = "carte de visite numérique qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-art", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-carte-de-visite-numérique-qr-code-fr-fr-4f3f01"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-audio-fr-fr-57ff60"})
ON CREATE SET
  kw.value = "qr code audio",
  kw.volume = 70,
  kw.difficulty = 5,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-audio", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-audio-fr-fr-57ff60"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-couleur-fr-fr-2d7496"})
ON CREATE SET
  kw.value = "qr code couleur",
  kw.volume = 70,
  kw.difficulty = 20,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-color", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-couleur-fr-fr-2d7496"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-menu-fr-fr-dac676"})
ON CREATE SET
  kw.value = "qr code menu",
  kw.volume = 150,
  kw.difficulty = 4,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-menu-fr-fr-dac676"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-menu-restaurant-fr-fr-37b424"})
ON CREATE SET
  kw.value = "qr code menu restaurant",
  kw.volume = 150,
  kw.difficulty = 7,
  kw.cpc = 1.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-menu-restaurant-fr-fr-37b424"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-menu-qr-code-fr-fr-8bfd5a"})
ON CREATE SET
  kw.value = "menu qr code",
  kw.volume = 150,
  kw.difficulty = 4,
  kw.cpc = 1.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-menu-qr-code-fr-fr-8bfd5a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-menu-qr-code-restaurant-fr-fr-f4cb0f"})
ON CREATE SET
  kw.value = "menu qr code restaurant",
  kw.volume = 100,
  kw.difficulty = 5,
  kw.cpc = 1.7,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-menu-qr-code-restaurant-fr-fr-f4cb0f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-restaurant-menu-fr-fr-988aa2"})
ON CREATE SET
  kw.value = "qr code restaurant menu",
  kw.volume = 80,
  kw.difficulty = 4,
  kw.cpc = 1.5,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-restaurant-menu-fr-fr-988aa2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-menu-restaurant-fr-fr-97f327"})
ON CREATE SET
  kw.value = "qr menu restaurant",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 1.3,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-menu-restaurant-fr-fr-97f327"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-menu-fr-fr-80e406"})
ON CREATE SET
  kw.value = "qr menu",
  kw.volume = 70,
  kw.difficulty = 5,
  kw.cpc = 1.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-menu-fr-fr-80e406"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-restaurant-menu-gratuit-fr-fr-644569"})
ON CREATE SET
  kw.value = "qr code restaurant menu gratuit",
  kw.volume = 70,
  kw.difficulty = 1,
  kw.cpc = 0.9,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-restaurant-menu-gratuit-fr-fr-644569"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-menu-restaurant-qr-code-fr-fr-0f591d"})
ON CREATE SET
  kw.value = "menu restaurant qr code",
  kw.volume = 50,
  kw.difficulty = 4,
  kw.cpc = 1.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-menu", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-menu-restaurant-qr-code-fr-fr-0f591d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-paiement-par-qr-code-fr-fr-5ce536"})
ON CREATE SET
  kw.value = "paiement par qr code",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.9,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-payment", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-paiement-par-qr-code-fr-fr-5ce536"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-paiement-fr-fr-1bfab7"})
ON CREATE SET
  kw.value = "qr code paiement",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-payment", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-paiement-fr-fr-1bfab7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-paiement-qr-code-fr-fr-be29cd"})
ON CREATE SET
  kw.value = "paiement qr code",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 1.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-payment", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-paiement-qr-code-fr-fr-be29cd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-paiement-par-code-qr-fr-fr-51a6ff"})
ON CREATE SET
  kw.value = "paiement par code qr",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-payment", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-paiement-par-code-qr-fr-fr-51a6ff"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-paiement-qr-code-fr-fr-4d4353"})
ON CREATE SET
  kw.value = "application paiement qr code",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-payment", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-paiement-qr-code-fr-fr-4d4353"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-paiement-par-qr-code-comment-ça-marche-fr-fr-719cc0"})
ON CREATE SET
  kw.value = "paiement par qr code comment ça marche",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-payment", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-paiement-par-qr-code-comment-ça-marche-fr-fr-719cc0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-système-paiement-qr-code-fr-fr-c07609"})
ON CREATE SET
  kw.value = "système paiement qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-payment", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-système-paiement-qr-code-fr-fr-c07609"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pdf-fr-fr-216e51"})
ON CREATE SET
  kw.value = "qr code pdf",
  kw.volume = 500,
  kw.difficulty = 11,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pdf-fr-fr-216e51"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-pdf-en-qr-code-fr-fr-938a2f"})
ON CREATE SET
  kw.value = "pdf en qr code",
  kw.volume = 200,
  kw.difficulty = 6,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-pdf-en-qr-code-fr-fr-938a2f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pdf-gratuit-fr-fr-9f4978"})
ON CREATE SET
  kw.value = "qr code pdf gratuit",
  kw.volume = 150,
  kw.difficulty = 27,
  kw.cpc = 0.3,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pdf-gratuit-fr-fr-9f4978"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-pdf-to-qr-code-fr-fr-8d4b8f"})
ON CREATE SET
  kw.value = "pdf to qr code",
  kw.volume = 150,
  kw.difficulty = 5,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-pdf-to-qr-code-fr-fr-8d4b8f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-transformer-un-pdf-en-qr-code-gratuit-fr-fr-6abaf3"})
ON CREATE SET
  kw.value = "transformer un pdf en qr code gratuit",
  kw.volume = 100,
  kw.difficulty = 9,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-transformer-un-pdf-en-qr-code-gratuit-fr-fr-6abaf3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-gratuit-pdf-fr-fr-d797e2"})
ON CREATE SET
  kw.value = "qr code gratuit pdf",
  kw.volume = 90,
  kw.difficulty = 11,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-gratuit-pdf-fr-fr-d797e2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-pdf-qr-code-fr-fr-a96847"})
ON CREATE SET
  kw.value = "pdf qr code",
  kw.volume = 80,
  kw.difficulty = 11,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-pdf-qr-code-fr-fr-a96847"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pour-pdf-fr-fr-ccfb3c"})
ON CREATE SET
  kw.value = "qr code pour pdf",
  kw.volume = 80,
  kw.difficulty = 6,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pour-pdf-fr-fr-ccfb3c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vers-pdf-gratuit-fr-fr-11e601"})
ON CREATE SET
  kw.value = "qr code vers pdf gratuit",
  kw.volume = 70,
  kw.difficulty = 7,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vers-pdf-gratuit-fr-fr-11e601"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vers-pdf-fr-fr-0beb6e"})
ON CREATE SET
  kw.value = "qr code vers pdf",
  kw.volume = 70,
  kw.difficulty = 5,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vers-pdf-fr-fr-0beb6e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-transformer-un-pdf-en-qr-code-fr-fr-31fb66"})
ON CREATE SET
  kw.value = "transformer un pdf en qr code",
  kw.volume = 50,
  kw.difficulty = 9,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-pdf", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-transformer-un-pdf-en-qr-code-fr-fr-31fb66"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-fond-transparent-fr-fr-763486"})
ON CREATE SET
  kw.value = "qr code fond transparent",
  kw.volume = 80,
  kw.difficulty = 21,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-transparent-background", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-fond-transparent-fr-fr-763486"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-transparent-fr-fr-5fd0df"})
ON CREATE SET
  kw.value = "qr code transparent",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-transparent-background", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-transparent-fr-fr-5fd0df"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-url-to-qr-code-fr-fr-0cd3ca"})
ON CREATE SET
  kw.value = "url to qr code",
  kw.volume = 300,
  kw.difficulty = 83,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-url", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-url-to-qr-code-fr-fr-0cd3ca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-multi-url-fr-fr-09188f"})
ON CREATE SET
  kw.value = "qr code multi url",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-url", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-multi-url-fr-fr-09188f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-generate-qr-code-from-url-fr-fr-ed38b8"})
ON CREATE SET
  kw.value = "generate qr code from url",
  kw.volume = 150,
  kw.difficulty = 85,
  kw.cpc = 0.45,
  kw.intent = "Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-url", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-generate-qr-code-from-url-fr-fr-ed38b8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-url-en-qr-code-fr-fr-6f600d"})
ON CREATE SET
  kw.value = "url en qr code",
  kw.volume = 100,
  kw.difficulty = 21,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-url", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-url-en-qr-code-fr-fr-6f600d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-url-fr-fr-7f483c"})
ON CREATE SET
  kw.value = "qr code url",
  kw.volume = 100,
  kw.difficulty = 45,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-url", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-url-fr-fr-7f483c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-transformer-url-en-qr-code-fr-fr-f8e439"})
ON CREATE SET
  kw.value = "transformer url en qr code",
  kw.volume = 90,
  kw.difficulty = 21,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-url", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-transformer-url-en-qr-code-fr-fr-f8e439"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-url-qr-code-fr-fr-5f2fb2"})
ON CREATE SET
  kw.value = "url qr code",
  kw.volume = 80,
  kw.difficulty = 34,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-url", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-url-qr-code-fr-fr-5f2fb2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-vcard-qr-code-fr-fr-0d80ad"})
ON CREATE SET
  kw.value = "vcard qr code",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-vcard", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-vcard-qr-code-fr-fr-0d80ad"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vcard-fr-fr-89b031"})
ON CREATE SET
  kw.value = "qr code vcard",
  kw.volume = 250,
  kw.difficulty = 1,
  kw.cpc = 0.8,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-vcard", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vcard-fr-fr-89b031"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-vcard-gratuit-fr-fr-78e657"})
ON CREATE SET
  kw.value = "qr code vcard gratuit",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-vcard", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-vcard-gratuit-fr-fr-78e657"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-vcard-qr-code-gratuit-fr-fr-b7cf71"})
ON CREATE SET
  kw.value = "vcard qr code gratuit",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-vcard", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-vcard-qr-code-gratuit-fr-fr-b7cf71"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-video-fr-fr-766add"})
ON CREATE SET
  kw.value = "qr code video",
  kw.volume = 300,
  kw.difficulty = 23,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-video", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-video-fr-fr-766add"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-video-qr-code-fr-fr-c5067d"})
ON CREATE SET
  kw.value = "video qr code",
  kw.volume = 70,
  kw.difficulty = 12,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-video", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-video-qr-code-fr-fr-c5067d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-video-en-qr-code-fr-fr-988b34"})
ON CREATE SET
  kw.value = "video en qr code",
  kw.volume = 70,
  kw.difficulty = 12,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-video", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-video-en-qr-code-fr-fr-988b34"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-video-youtube-fr-fr-d20563"})
ON CREATE SET
  kw.value = "qr code video youtube",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-video", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-video-youtube-fr-fr-d20563"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pour-video-fr-fr-54f650"})
ON CREATE SET
  kw.value = "qr code pour video",
  kw.volume = 50,
  kw.difficulty = 14,
  kw.cpc = 0.35,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-video", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pour-video-fr-fr-54f650"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-wifi-fr-fr-2cd966"})
ON CREATE SET
  kw.value = "qr code wifi",
  kw.volume = 400,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-wifi-fr-fr-2cd966"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-wifi-qr-code-fr-fr-9a931d"})
ON CREATE SET
  kw.value = "wifi qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-wifi-qr-code-fr-fr-9a931d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-wifi-gratuit-fr-fr-8890dd"})
ON CREATE SET
  kw.value = "qr code wifi gratuit",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-wifi-gratuit-fr-fr-8890dd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-decoder-qr-code-wifi-fr-fr-740e3a"})
ON CREATE SET
  kw.value = "decoder qr code wifi",
  kw.volume = 100,
  kw.difficulty = 24,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-decoder-qr-code-wifi-fr-fr-740e3a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr-wifi-fr-fr-ab9b05"})
ON CREATE SET
  kw.value = "code qr wifi",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr-wifi-fr-fr-ab9b05"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-partager-wifi-iphone-qr-code-fr-fr-a12c4b"})
ON CREATE SET
  kw.value = "partager wifi iphone qr code",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-partager-wifi-iphone-qr-code-fr-fr-a12c4b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-afficher-mot-de-passe-wifi-code-qr-fr-fr-b66404"})
ON CREATE SET
  kw.value = "afficher mot de passe wifi code qr",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.03,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-afficher-mot-de-passe-wifi-code-qr-fr-fr-b66404"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-wifi-iphone-fr-fr-3ba355"})
ON CREATE SET
  kw.value = "qr code wifi iphone",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-wifi-iphone-fr-fr-3ba355"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-connexion-wifi-fr-fr-51a3b1"})
ON CREATE SET
  kw.value = "qr code connexion wifi",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-wifi", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-connexion-wifi-fr-fr-51a3b1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-logo-fr-fr-73c05d"})
ON CREATE SET
  kw.value = "qr code logo",
  kw.volume = 200,
  kw.difficulty = 22,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-with-logo", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-logo-fr-fr-73c05d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-avec-logo-fr-fr-6a80ca"})
ON CREATE SET
  kw.value = "qr code avec logo",
  kw.volume = 150,
  kw.difficulty = 22,
  kw.cpc = 0.7,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-with-logo", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-avec-logo-fr-fr-6a80ca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-logo-qr-code-fr-fr-21635f"})
ON CREATE SET
  kw.value = "logo qr code",
  kw.volume = 100,
  kw.difficulty = 20,
  kw.cpc = 0.5,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-with-logo", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-logo-qr-code-fr-fr-21635f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-with-logo-fr-fr-44a428"})
ON CREATE SET
  kw.value = "qr code with logo",
  kw.volume = 80,
  kw.difficulty = 74,
  kw.cpc = 0.6,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-with-logo", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-with-logo-fr-fr-44a428"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-avec-logo-intégré-fr-fr-acf7e3"})
ON CREATE SET
  kw.value = "qr code avec logo intégré",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.5,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-with-logo", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-avec-logo-intégré-fr-fr-acf7e3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-avec-logo-gratuit-fr-fr-e485d1"})
ON CREATE SET
  kw.value = "qr code avec logo gratuit",
  kw.volume = 50,
  kw.difficulty = 23,
  kw.cpc = 0.45,
  kw.intent = "Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "qr-code-with-logo", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-avec-logo-gratuit-fr-fr-e485d1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-restaurant-fr-fr-bbaad4"})
ON CREATE SET
  kw.value = "qr code restaurant",
  kw.volume = 2500,
  kw.difficulty = 4,
  kw.cpc = 1.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "restaurants", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-restaurant-fr-fr-bbaad4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-table-restaurant-fr-fr-912330"})
ON CREATE SET
  kw.value = "qr code table restaurant",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 2.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "restaurants", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-table-restaurant-fr-fr-912330"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-pour-restaurant-fr-fr-68ba49"})
ON CREATE SET
  kw.value = "qr code pour restaurant",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 1.3,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "restaurants", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-pour-restaurant-fr-fr-68ba49"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-fr-fr-df5c96"})
ON CREATE SET
  kw.value = "comment scanner un qr code",
  kw.volume = 18000,
  kw.difficulty = 3,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-fr-fr-df5c96"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-fr-fr-d3ef28"})
ON CREATE SET
  kw.value = "scanner qr code",
  kw.volume = 17000,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-fr-fr-d3ef28"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-fr-fr-5ddd5b"})
ON CREATE SET
  kw.value = "scan qr code",
  kw.volume = 7900,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-fr-fr-5ddd5b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner-fr-fr-037f24"})
ON CREATE SET
  kw.value = "qr code scanner",
  kw.volume = 7000,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner-fr-fr-037f24"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-fr-fr-82e7d7"})
ON CREATE SET
  kw.value = "lecteur qr code",
  kw.volume = 4800,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-fr-fr-82e7d7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-fr-fr-e5bb54"})
ON CREATE SET
  kw.value = "scanner un qr code",
  kw.volume = 4100,
  kw.difficulty = 4,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-fr-fr-e5bb54"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-gratuit-fr-fr-aeeefe"})
ON CREATE SET
  kw.value = "scanner qr code gratuit",
  kw.volume = 3200,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-gratuit-fr-fr-aeeefe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-fr-fr-4bf22e"})
ON CREATE SET
  kw.value = "lecteur qr code gratuit",
  kw.volume = 2700,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-fr-fr-4bf22e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-fr-fr-24cc7a"})
ON CREATE SET
  kw.value = "lire qr code",
  kw.volume = 2600,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-fr-fr-24cc7a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-propre-téléphon-fr-fr-c489ce"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur son propre téléphone",
  kw.volume = 2400,
  kw.difficulty = 2,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-propre-téléphon-fr-fr-c489ce"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-fr-fr-df44a7"})
ON CREATE SET
  kw.value = "comment lire un qr code",
  kw.volume = 2400,
  kw.difficulty = 2,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-fr-fr-df44a7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-fr-fr-330569"})
ON CREATE SET
  kw.value = "comment flasher un qr code",
  kw.volume = 2400,
  kw.difficulty = 5,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-fr-fr-330569"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-en-ligne-fr-fr-7cbada"})
ON CREATE SET
  kw.value = "scanner qr code en ligne",
  kw.volume = 1600,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-en-ligne-fr-fr-7cbada"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-un-qr-code-fr-fr-278c00"})
ON CREATE SET
  kw.value = "lire un qr code",
  kw.volume = 1500,
  kw.difficulty = 4,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-un-qr-code-fr-fr-278c00"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-scanner-fr-fr-e3e4eb"})
ON CREATE SET
  kw.value = "qr scanner",
  kw.volume = 1500,
  kw.difficulty = 42,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-scanner-fr-fr-e3e4eb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-show-your-qr-on-the-reader-fr-fr-68669a"})
ON CREATE SET
  kw.value = "show your qr on the reader",
  kw.volume = 1200,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-show-your-qr-on-the-reader-fr-fr-68669a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-iphone-fr-fr-6ba9a0"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur iphone",
  kw.volume = 1100,
  kw.difficulty = 0,
  kw.cpc = 0.05,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-iphone-fr-fr-6ba9a0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-android-fr-fr-87d127"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur android",
  kw.volume = 1100,
  kw.difficulty = 3,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-android-fr-fr-87d127"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-en-ligne-fr-fr-7afc9f"})
ON CREATE SET
  kw.value = "lecteur qr code en ligne",
  kw.volume = 1100,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-en-ligne-fr-fr-7afc9f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-samsung-fr-fr-90000c"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur samsung",
  kw.volume = 1100,
  kw.difficulty = 4,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-samsung-fr-fr-90000c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-muestra-tu-qr-en-el-lector-fr-fr-7b0b8c"})
ON CREATE SET
  kw.value = "muestra tu qr en el lector",
  kw.volume = 1000,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-muestra-tu-qr-en-el-lector-fr-fr-7b0b8c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-fr-fr-b22a53"})
ON CREATE SET
  kw.value = "lecteur de qr code",
  kw.volume = 1000,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-fr-fr-b22a53"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-reader-fr-fr-e05ed7"})
ON CREATE SET
  kw.value = "qr code reader",
  kw.volume = 1000,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-reader-fr-fr-e05ed7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-flasher-un-qr-code-fr-fr-6e92c5"})
ON CREATE SET
  kw.value = "flasher un qr code",
  kw.volume = 1000,
  kw.difficulty = 3,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-flasher-un-qr-code-fr-fr-6e92c5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-en-ligne-fr-fr-008c8f"})
ON CREATE SET
  kw.value = "scan qr code en ligne",
  kw.volume = 1000,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-en-ligne-fr-fr-008c8f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-faire-pour-scanner-un-qr-code-fr-fr-0417ad"})
ON CREATE SET
  kw.value = "comment faire pour scanner un qr code",
  kw.volume = 800,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-faire-pour-scanner-un-qr-code-fr-fr-0417ad"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-photo-fr-fr-8fed64"})
ON CREATE SET
  kw.value = "scanner qr code photo",
  kw.volume = 700,
  kw.difficulty = 3,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-photo-fr-fr-8fed64"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-fr-fr-8294db"})
ON CREATE SET
  kw.value = "scanner qr",
  kw.volume = 700,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-fr-fr-8294db"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-flasher-qr-code-fr-fr-1b9412"})
ON CREATE SET
  kw.value = "flasher qr code",
  kw.volume = 700,
  kw.difficulty = 4,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-flasher-qr-code-fr-fr-1b9412"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-scan-qr-code-on-android-fr-fr-0d97d8"})
ON CREATE SET
  kw.value = "how to scan qr code on android",
  kw.volume = 600,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-scan-qr-code-on-android-fr-fr-0d97d8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-gratuit-fr-fr-271641"})
ON CREATE SET
  kw.value = "lecteur de qr code gratuit",
  kw.volume = 600,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-gratuit-fr-fr-271641"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner-gratuit-fr-fr-8057a4"})
ON CREATE SET
  kw.value = "qr code scanner gratuit",
  kw.volume = 600,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner-gratuit-fr-fr-8057a4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-scanner-qr-code-fr-fr-20620d"})
ON CREATE SET
  kw.value = "application scanner qr code",
  kw.volume = 500,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-scanner-qr-code-fr-fr-20620d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-gratuit-fr-fr-46f97e"})
ON CREATE SET
  kw.value = "scan qr code gratuit",
  kw.volume = 500,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-gratuit-fr-fr-46f97e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-de-qr-code-fr-fr-165130"})
ON CREATE SET
  kw.value = "scanner de qr code",
  kw.volume = 500,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-de-qr-code-fr-fr-165130"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-gratuit-android-fr-fr-eb39b9"})
ON CREATE SET
  kw.value = "scanner qr code gratuit android",
  kw.volume = 500,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-gratuit-android-fr-fr-eb39b9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-android-fr-fr-05f514"})
ON CREATE SET
  kw.value = "scanner qr code android",
  kw.volume = 500,
  kw.difficulty = 3,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-android-fr-fr-05f514"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-sans-pub-fr-fr-61c720"})
ON CREATE SET
  kw.value = "lecteur qr code gratuit sans pub",
  kw.volume = 450,
  kw.difficulty = 6,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-sans-pub-fr-fr-61c720"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scan-fr-fr-ccafc3"})
ON CREATE SET
  kw.value = "qr code scan",
  kw.volume = 450,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scan-fr-fr-ccafc3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanneur-de-qr-code-fr-fr-fdd0d1"})
ON CREATE SET
  kw.value = "scanneur de qr code",
  kw.volume = 450,
  kw.difficulty = 2,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanneur-de-qr-code-fr-fr-fdd0d1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-pmu-fr-fr-39af63"})
ON CREATE SET
  kw.value = "scan qr code pmu",
  kw.volume = 450,
  kw.difficulty = 7,
  kw.cpc = 0.6,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-pmu-fr-fr-39af63"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-iphone-fr-fr-a721ff"})
ON CREATE SET
  kw.value = "scanner qr code iphone",
  kw.volume = 450,
  kw.difficulty = 0,
  kw.cpc = 0.08,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-iphone-fr-fr-a721ff"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-online-fr-fr-6e43e7"})
ON CREATE SET
  kw.value = "scan qr code online",
  kw.volume = 450,
  kw.difficulty = 45,
  kw.cpc = 0.05,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-online-fr-fr-6e43e7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-fr-fr-701d2c"})
ON CREATE SET
  kw.value = "comment scanner qr code",
  kw.volume = 450,
  kw.difficulty = 1,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-fr-fr-701d2c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-scanner-gratuit-fr-fr-cb4afe"})
ON CREATE SET
  kw.value = "qr scanner gratuit",
  kw.volume = 400,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-scanner-gratuit-fr-fr-cb4afe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-pour-scanner-qr-code-fr-fr-63151f"})
ON CREATE SET
  kw.value = "application pour scanner qr code",
  kw.volume = 400,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-pour-scanner-qr-code-fr-fr-63151f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-code-qr-fr-fr-034aeb"})
ON CREATE SET
  kw.value = "scanner code qr",
  kw.volume = 400,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-code-qr-fr-fr-034aeb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-mon-téléphone-fr-fr-a48c65"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec mon téléphone",
  kw.volume = 400,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-mon-téléphone-fr-fr-a48c65"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-photo-fr-fr-2a7e25"})
ON CREATE SET
  kw.value = "lire qr code photo",
  kw.volume = 400,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-photo-fr-fr-2a7e25"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-lecteur-fr-fr-97e3d1"})
ON CREATE SET
  kw.value = "qr code lecteur",
  kw.volume = 400,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-lecteur-fr-fr-97e3d1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-fr-fr-0d4d83"})
ON CREATE SET
  kw.value = "scan qr",
  kw.volume = 400,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-fr-fr-0d4d83"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-le-qr-code-fr-fr-c3bdc1"})
ON CREATE SET
  kw.value = "scanner le qr code",
  kw.volume = 350,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-le-qr-code-fr-fr-c3bdc1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-reader-online-fr-fr-ee7f1f"})
ON CREATE SET
  kw.value = "qr code reader online",
  kw.volume = 350,
  kw.difficulty = 70,
  kw.cpc = 0.02,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-reader-online-fr-fr-ee7f1f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner---scanner-qr-fr-fr-2d318d"})
ON CREATE SET
  kw.value = "qr code scanner - scanner qr",
  kw.volume = 350,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner---scanner-qr-fr-fr-2d318d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner-online-fr-fr-453227"})
ON CREATE SET
  kw.value = "qr code scanner online",
  kw.volume = 350,
  kw.difficulty = 61,
  kw.cpc = 0.05,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner-online-fr-fr-453227"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-sur-pc-fr-fr-ffc33e"})
ON CREATE SET
  kw.value = "lire qr code sur pc",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.02,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-sur-pc-fr-fr-ffc33e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scannez-le-code-qr-fr-fr-cad8af"})
ON CREATE SET
  kw.value = "scannez le code qr",
  kw.volume = 300,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scannez-le-code-qr-fr-fr-cad8af"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-carte-vitale-qr-code-fr-fr-8ba05a"})
ON CREATE SET
  kw.value = "lecteur carte vitale qr code",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-carte-vitale-qr-code-fr-fr-8ba05a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-android-fr-fr-c1edbe"})
ON CREATE SET
  kw.value = "lecteur qr code android",
  kw.volume = 300,
  kw.difficulty = 4,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-android-fr-fr-c1edbe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-gratuitement-fr-fr-be8685"})
ON CREATE SET
  kw.value = "comment scanner un qr code gratuitement",
  kw.volume = 300,
  kw.difficulty = 2,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-gratuitement-fr-fr-be8685"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-propre-téléphon-fr-fr-878d09"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur son propre téléphone samsung",
  kw.volume = 300,
  kw.difficulty = 3,
  kw.cpc = 0.09,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-propre-téléphon-fr-fr-878d09"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-android-fr-fr-f87e9d"})
ON CREATE SET
  kw.value = "scan qr code android",
  kw.volume = 300,
  kw.difficulty = 3,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-android-fr-fr-f87e9d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-propre-téléphon-fr-fr-3a54ba"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur son propre téléphone iphone",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-propre-téléphon-fr-fr-3a54ba"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-un-qr-code-sur-pc-fr-fr-725382"})
ON CREATE SET
  kw.value = "lire un qr code sur pc",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.02,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-un-qr-code-sur-pc-fr-fr-725382"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-gratuit-fr-fr-04d3a9"})
ON CREATE SET
  kw.value = "scanner un qr code gratuit",
  kw.volume = 300,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-gratuit-fr-fr-04d3a9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-samsung-fr-fr-ee9232"})
ON CREATE SET
  kw.value = "scanner qr code samsung",
  kw.volume = 300,
  kw.difficulty = 4,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-samsung-fr-fr-ee9232"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-fdj-fr-fr-9a9741"})
ON CREATE SET
  kw.value = "scan qr code fdj",
  kw.volume = 300,
  kw.difficulty = 31,
  kw.cpc = 0.5,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-fdj-fr-fr-9a9741"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-google-fr-fr-867b2d"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec google",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-google-fr-fr-867b2d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-gratuit-fr-fr-081afe"})
ON CREATE SET
  kw.value = "lire qr code gratuit",
  kw.volume = 250,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-gratuit-fr-fr-081afe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-sur-son-téléphone-fr-fr-e9239b"})
ON CREATE SET
  kw.value = "comment flasher un qr code sur son téléphone",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-sur-son-téléphone-fr-fr-e9239b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-une-photo-fr-fr-3aa948"})
ON CREATE SET
  kw.value = "scanner un qr code sur une photo",
  kw.volume = 250,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-une-photo-fr-fr-3aa948"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-pc-fr-fr-ce0fab"})
ON CREATE SET
  kw.value = "scanner un qr code sur pc",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.02,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-pc-fr-fr-ce0fab"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-pc-fr-fr-c6a3a7"})
ON CREATE SET
  kw.value = "scan qr code pc",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.02,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-pc-fr-fr-c6a3a7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-mon-portable-fr-fr-bef2d7"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec mon portable",
  kw.volume = 250,
  kw.difficulty = 1,
  kw.cpc = 0.06,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-mon-portable-fr-fr-bef2d7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-ordinateur-fr-fr-e442a7"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur ordinateur",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.02,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-ordinateur-fr-fr-e442a7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-iphone-fr-fr-c7dc72"})
ON CREATE SET
  kw.value = "scanner un qr code iphone",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.06,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-iphone-fr-fr-c7dc72"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-pc-fr-fr-8c73c4"})
ON CREATE SET
  kw.value = "scanner qr code pc",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.02,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-pc-fr-fr-8c73c4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-son-téléphone-fr-fr-87f6db"})
ON CREATE SET
  kw.value = "comment lire un qr code sur son téléphone",
  kw.volume = 250,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-son-téléphone-fr-fr-87f6db"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-meilleur-lecteur-qr-code-gratuit-fr-fr-373898"})
ON CREATE SET
  kw.value = "meilleur lecteur qr code gratuit",
  kw.volume = 250,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-meilleur-lecteur-qr-code-gratuit-fr-fr-373898"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-un-samsung-fr-fr-e64d1c"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur un samsung",
  kw.volume = 250,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-un-samsung-fr-fr-e64d1c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr-scan-fr-fr-912c0a"})
ON CREATE SET
  kw.value = "code qr scan",
  kw.volume = 250,
  kw.difficulty = 3,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr-scan-fr-fr-912c0a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-code-qr-sur-android-fr-fr-51cad3"})
ON CREATE SET
  kw.value = "comment lire un code qr sur android",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-code-qr-sur-android-fr-fr-51cad3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-appli-scan-qr-code-fr-fr-6e6196"})
ON CREATE SET
  kw.value = "appli scan qr code",
  kw.volume = 200,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-appli-scan-qr-code-fr-fr-6e6196"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-carte-vitale-fr-fr-90f7b1"})
ON CREATE SET
  kw.value = "lecteur qr code carte vitale",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-carte-vitale-fr-fr-90f7b1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-en-ligne-fr-fr-a4c4ea"})
ON CREATE SET
  kw.value = "lire qr code en ligne",
  kw.volume = 200,
  kw.difficulty = 3,
  kw.cpc = 0.07,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-en-ligne-fr-fr-a4c4ea"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-gratuit-samsung-fr-fr-ec3280"})
ON CREATE SET
  kw.value = "scanner qr code gratuit samsung",
  kw.volume = 200,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-gratuit-samsung-fr-fr-ec3280"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner---reader-fr-fr-b1c021"})
ON CREATE SET
  kw.value = "qr code scanner - reader",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner---reader-fr-fr-b1c021"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-son-propre-téléphone-fr-fr-ed6c46"})
ON CREATE SET
  kw.value = "comment lire un qr code sur son propre téléphone",
  kw.volume = 200,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-son-propre-téléphone-fr-fr-ed6c46"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-sur-iphone-fr-fr-c5a164"})
ON CREATE SET
  kw.value = "comment scanner qr code sur iphone",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-sur-iphone-fr-fr-c5a164"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-fait-on-pour-scanner-un-qr-code-fr-fr-d454ba"})
ON CREATE SET
  kw.value = "comment fait on pour scanner un qr code",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-fait-on-pour-scanner-un-qr-code-fr-fr-d454ba"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-avec-son-téléphone-fr-fr-00c179"})
ON CREATE SET
  kw.value = "comment flasher un qr code avec son téléphone",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-avec-son-téléphone-fr-fr-00c179"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-reader-fr-fr-0df0e3"})
ON CREATE SET
  kw.value = "qr reader",
  kw.volume = 200,
  kw.difficulty = 44,
  kw.cpc = 0.1,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-reader-fr-fr-0df0e3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-le-qr-code-fr-fr-e83230"})
ON CREATE SET
  kw.value = "comment scanner le qr code",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-le-qr-code-fr-fr-e83230"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-un-qr-code-sur-une-photo-fr-fr-07aacd"})
ON CREATE SET
  kw.value = "lire un qr code sur une photo",
  kw.volume = 200,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-un-qr-code-sur-une-photo-fr-fr-07aacd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sans-application-fr-fr-a1669a"})
ON CREATE SET
  kw.value = "comment scanner un qr code sans application",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.05,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sans-application-fr-fr-a1669a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-iphone-fr-fr-c55467"})
ON CREATE SET
  kw.value = "comment lire un qr code sur iphone",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.06,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-iphone-fr-fr-c55467"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-samsung-fr-fr-6949c9"})
ON CREATE SET
  kw.value = "comment lire un qr code sur samsung",
  kw.volume = 200,
  kw.difficulty = 1,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-samsung-fr-fr-6949c9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-une-capture-décran-fr-fr-5e0938"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur une capture d\'écran",
  kw.volume = 200,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-une-capture-décran-fr-fr-5e0938"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-pour-scanner-un-qr-code-fr-fr-1274d5"})
ON CREATE SET
  kw.value = "pour scanner un qr code",
  kw.volume = 150,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-pour-scanner-un-qr-code-fr-fr-1274d5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-a-partir-dune-photo-iphone-fr-fr-322abc"})
ON CREATE SET
  kw.value = "lire qr code a partir d\'une photo iphone",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-a-partir-dune-photo-iphone-fr-fr-322abc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-scanner-qr-code-fr-fr-d16f6d"})
ON CREATE SET
  kw.value = "installer scanner qr code",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-scanner-qr-code-fr-fr-d16f6d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-code-qr-fr-fr-647cc5"})
ON CREATE SET
  kw.value = "comment scanner un code qr",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-code-qr-fr-fr-647cc5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-fait-on-pour-scanner-un-qr-code-fr-fr-f0cc1a"})
ON CREATE SET
  kw.value = "comment fait-on pour scanner un qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-fait-on-pour-scanner-un-qr-code-fr-fr-f0cc1a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner-google-fr-fr-4d27ef"})
ON CREATE SET
  kw.value = "qr code scanner google",
  kw.volume = 150,
  kw.difficulty = 3,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner-google-fr-fr-4d27ef"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-iphone-fr-fr-45addc"})
ON CREATE SET
  kw.value = "lire qr code iphone",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-iphone-fr-fr-45addc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-code-qr-fr-fr-ef62c8"})
ON CREATE SET
  kw.value = "scanner un code qr",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-code-qr-fr-fr-ef62c8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-samsung-fr-fr-29c44c"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec samsung",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-samsung-fr-fr-29c44c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-code-qr-fr-fr-4bf053"})
ON CREATE SET
  kw.value = "lecteur code qr",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-code-qr-fr-fr-4bf053"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-un-qr-code-en-ligne-fr-fr-852f8a"})
ON CREATE SET
  kw.value = "lire un qr code en ligne",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-un-qr-code-en-ligne-fr-fr-852f8a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-pc-fr-fr-8436ad"})
ON CREATE SET
  kw.value = "lecteur qr code pc",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.04,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-pc-fr-fr-8436ad"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-codes-fr-fr-b0d5aa"})
ON CREATE SET
  kw.value = "scanner qr codes",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-codes-fr-fr-b0d5aa"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner-en-ligne-fr-fr-4f1957"})
ON CREATE SET
  kw.value = "qr code scanner en ligne",
  kw.volume = 150,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner-en-ligne-fr-fr-4f1957"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-en-ligne-fr-fr-e887d8"})
ON CREATE SET
  kw.value = "lecteur de qr code en ligne",
  kw.volume = 150,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-en-ligne-fr-fr-e887d8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-un-iphone-fr-fr-8f080f"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec un iphone",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.04,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-un-iphone-fr-fr-8f080f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-samsung-fr-fr-9a5fdc"})
ON CREATE SET
  kw.value = "scanner un qr code sur samsung",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-samsung-fr-fr-9a5fdc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr--scanner-code-barres-fr-fr-92e27e"})
ON CREATE SET
  kw.value = "code qr & scanner code-barres",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr--scanner-code-barres-fr-fr-92e27e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-appli-pour-scanner-qr-code-fr-fr-2e80f8"})
ON CREATE SET
  kw.value = "appli pour scanner qr code",
  kw.volume = 150,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-appli-pour-scanner-qr-code-fr-fr-2e80f8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-on-scanne-un-qr-code-fr-fr-c27201"})
ON CREATE SET
  kw.value = "comment on scanne un qr code",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-on-scanne-un-qr-code-fr-fr-c27201"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-iphone-fr-fr-547ab7"})
ON CREATE SET
  kw.value = "scan qr code iphone",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-iphone-fr-fr-547ab7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-sans-application-fr-fr-0b81ed"})
ON CREATE SET
  kw.value = "lire qr code sans application",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-sans-application-fr-fr-0b81ed"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-en-ligne-sans-application-fr-fr-125e24"})
ON CREATE SET
  kw.value = "lire qr code en ligne sans application",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-en-ligne-sans-application-fr-fr-125e24"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-en-ligne-fr-fr-b6cc7b"})
ON CREATE SET
  kw.value = "scanner un qr code en ligne",
  kw.volume = 150,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-en-ligne-fr-fr-b6cc7b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-avec-samsung-fr-fr-f334db"})
ON CREATE SET
  kw.value = "comment scanner qr code avec samsung",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-avec-samsung-fr-fr-f334db"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-qr-code-fr-fr-9dbd2e"})
ON CREATE SET
  kw.value = "comment lire qr code",
  kw.volume = 150,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-qr-code-fr-fr-9dbd2e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-on-fait-pour-scanner-un-qr-code-fr-fr-51e4a7"})
ON CREATE SET
  kw.value = "comment on fait pour scanner un qr code",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-on-fait-pour-scanner-un-qr-code-fr-fr-51e4a7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-un-samsung-fr-fr-1e087e"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec un samsung",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-un-samsung-fr-fr-1e087e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-son-propre-telephone-fr-fr-de70f6"})
ON CREATE SET
  kw.value = "scanner un qr code sur son propre telephone",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-son-propre-telephone-fr-fr-de70f6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-codes-fr-fr-4ddf7a"})
ON CREATE SET
  kw.value = "scan qr codes",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-codes-fr-fr-4ddf7a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-lecteur-qr-code-fr-fr-e1222c"})
ON CREATE SET
  kw.value = "installer lecteur qr code",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-lecteur-qr-code-fr-fr-e1222c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-une-photo-fr-fr-30872e"})
ON CREATE SET
  kw.value = "comment lire un qr code sur une photo",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-une-photo-fr-fr-30872e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scannable-qr-code-fr-fr-caabd6"})
ON CREATE SET
  kw.value = "scannable qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scannable-qr-code-fr-fr-caabd6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-iphone-gratuit-fr-fr-fc6ee7"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur iphone gratuit",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-iphone-gratuit-fr-fr-fc6ee7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-scan-fr-fr-7a4f8f"})
ON CREATE SET
  kw.value = "qr scan",
  kw.volume = 150,
  kw.difficulty = 37,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-scan-fr-fr-7a4f8f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-scanner-qr-code-gratuit-fr-fr-a2aef9"})
ON CREATE SET
  kw.value = "application scanner qr code gratuit",
  kw.volume = 150,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-scanner-qr-code-gratuit-fr-fr-a2aef9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner-app-fr-fr-408077"})
ON CREATE SET
  kw.value = "qr code scanner app",
  kw.volume = 100,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner-app-fr-fr-408077"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-un-qr-code-en-photo-fr-fr-05201d"})
ON CREATE SET
  kw.value = "lire un qr code en photo",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-un-qr-code-en-photo-fr-fr-05201d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-faire-pour-flasher-un-qr-code-fr-fr-9d6a45"})
ON CREATE SET
  kw.value = "comment faire pour flasher un qr code",
  kw.volume = 100,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-faire-pour-flasher-un-qr-code-fr-fr-9d6a45"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-de-code-qr-fr-fr-565535"})
ON CREATE SET
  kw.value = "lecteur de code qr",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-de-code-qr-fr-fr-565535"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-à-scanner-fr-fr-cf0738"})
ON CREATE SET
  kw.value = "qr code à scanner",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-à-scanner-fr-fr-cf0738"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-samsung-fr-fr-99a79d"})
ON CREATE SET
  kw.value = "lecteur qr code samsung",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-samsung-fr-fr-99a79d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-pour-scanner-un-qr-code-fr-fr-cdb34e"})
ON CREATE SET
  kw.value = "application pour scanner un qr code",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-pour-scanner-un-qr-code-fr-fr-cdb34e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-escanear-codigo-qr-fr-fr-4c7d2f"})
ON CREATE SET
  kw.value = "escanear codigo qr",
  kw.volume = 100,
  kw.difficulty = 26,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-escanear-codigo-qr-fr-fr-4c7d2f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr-et-scanner-code-barres-fr-fr-e0e66a"})
ON CREATE SET
  kw.value = "code qr et scanner code-barres",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr-et-scanner-code-barres-fr-fr-e0e66a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-new-balance-qr-code-scanner-fr-fr-d9df12"})
ON CREATE SET
  kw.value = "new balance qr code scanner",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-new-balance-qr-code-scanner-fr-fr-d9df12"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-scanner-qr-code-gratuit-fr-fr-7f48ef"})
ON CREATE SET
  kw.value = "installer scanner qr code gratuit",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-scanner-qr-code-gratuit-fr-fr-7f48ef"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-fr-fr-6c505f"})
ON CREATE SET
  kw.value = "lecteur qr",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-fr-fr-6c505f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-iphone-fr-fr-46b38f"})
ON CREATE SET
  kw.value = "lecteur qr code iphone",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-iphone-fr-fr-46b38f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-pc-fr-fr-806da9"})
ON CREATE SET
  kw.value = "comment lire un qr code sur pc",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-pc-fr-fr-806da9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-sans-pub-android-fr-fr-2865fd"})
ON CREATE SET
  kw.value = "lecteur qr code gratuit sans pub android",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-sans-pub-android-fr-fr-2865fd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-gratuit-en-ligne-fr-fr-a3e6b9"})
ON CREATE SET
  kw.value = "scanner qr code gratuit en ligne",
  kw.volume = 100,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-gratuit-en-ligne-fr-fr-a3e6b9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-mon-portable-fr-fr-47d2fb"})
ON CREATE SET
  kw.value = "comment lire un qr code sur mon portable",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-mon-portable-fr-fr-47d2fb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-scanner-fr-fr-b5182f"})
ON CREATE SET
  kw.value = "scan qr code scanner",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-scanner-fr-fr-b5182f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-mon-téléphone-samsung-fr-fr-2ff7c4"})
ON CREATE SET
  kw.value = "comment lire un qr code sur mon téléphone samsung",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-mon-téléphone-samsung-fr-fr-2ff7c4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-de-qr-code-fr-fr-cc3a62"})
ON CREATE SET
  kw.value = "scan de qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-de-qr-code-fr-fr-cc3a62"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scannez-le-qr-code-fr-fr-3a6c06"})
ON CREATE SET
  kw.value = "scannez le qr code",
  kw.volume = 100,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scannez-le-qr-code-fr-fr-3a6c06"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-installer-lecteur-qr-code-gratuit-fr-fr-3036d3"})
ON CREATE SET
  kw.value = "installer lecteur qr code gratuit",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-installer-lecteur-qr-code-gratuit-fr-fr-3036d3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-android-fr-fr-5597bd"})
ON CREATE SET
  kw.value = "lire qr code android",
  kw.volume = 100,
  kw.difficulty = 3,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-android-fr-fr-5597bd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-un-qr-code-iphone-fr-fr-196457"})
ON CREATE SET
  kw.value = "lire un qr code iphone",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-un-qr-code-iphone-fr-fr-196457"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-avec-samsung-fr-fr-8f7322"})
ON CREATE SET
  kw.value = "scanner un qr code avec samsung",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-avec-samsung-fr-fr-8f7322"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-gratuit-pour-android-fr-fr-025210"})
ON CREATE SET
  kw.value = "lecteur de qr code gratuit pour android",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-gratuit-pour-android-fr-fr-025210"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-scanner-qr-code-fr-fr-b368b0"})
ON CREATE SET
  kw.value = "télécharger scanner qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-scanner-qr-code-fr-fr-b368b0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-this-qr-code-fr-fr-a87ee0"})
ON CREATE SET
  kw.value = "scan this qr code",
  kw.volume = 100,
  kw.difficulty = 11,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-this-qr-code-fr-fr-a87ee0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-android-fr-fr-139381"})
ON CREATE SET
  kw.value = "comment lire un qr code sur android",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-sur-android-fr-fr-139381"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-avec-samsung-fr-fr-fcb747"})
ON CREATE SET
  kw.value = "scanner qr code avec samsung",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-avec-samsung-fr-fr-fcb747"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-un-qr-code-gratuit-fr-fr-1bc43d"})
ON CREATE SET
  kw.value = "lire un qr code gratuit",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-un-qr-code-gratuit-fr-fr-1bc43d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-code-qr-sans-application-fr-fr-480348"})
ON CREATE SET
  kw.value = "comment scanner un code qr sans application",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-code-qr-sans-application-fr-fr-480348"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-avec-samsung-fr-fr-a1b6e5"})
ON CREATE SET
  kw.value = "comment flasher un qr code avec samsung",
  kw.volume = 100,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-avec-samsung-fr-fr-a1b6e5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scannen-fr-fr-24f29c"})
ON CREATE SET
  kw.value = "qr code scannen",
  kw.volume = 100,
  kw.difficulty = 4,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scannen-fr-fr-24f29c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-sur-iphone-fr-fr-61e5c3"})
ON CREATE SET
  kw.value = "comment flasher un qr code sur iphone",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-sur-iphone-fr-fr-61e5c3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-from-image-fr-fr-aae015"})
ON CREATE SET
  kw.value = "scan qr code from image",
  kw.volume = 100,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-from-image-fr-fr-aae015"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-parions-sport-fr-fr-7435cc"})
ON CREATE SET
  kw.value = "scan qr code parions sport",
  kw.volume = 100,
  kw.difficulty = 24,
  kw.cpc = 0.25,
  kw.intent = "Informational,Transactional,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-parions-sport-fr-fr-7435cc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-scanner-online-fr-fr-1b97ec"})
ON CREATE SET
  kw.value = "qr scanner online",
  kw.volume = 90,
  kw.difficulty = 68,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-scanner-online-fr-fr-1b97ec"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-son-téléphone-fr-fr-2dd48e"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec son téléphone",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-son-téléphone-fr-fr-2dd48e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-avec-iphone-fr-fr-0870b2"})
ON CREATE SET
  kw.value = "comment flasher un qr code avec iphone",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-avec-iphone-fr-fr-0870b2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-fdj-scanner-qr-code-fr-fr-c985ec"})
ON CREATE SET
  kw.value = "fdj scanner qr code",
  kw.volume = 90,
  kw.difficulty = 30,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-fdj-scanner-qr-code-fr-fr-c985ec"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-son-ecran-fr-fr-3b960c"})
ON CREATE SET
  kw.value = "scanner un qr code sur son ecran",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-son-ecran-fr-fr-3b960c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-pour-lire-qr-code-fr-fr-33ad94"})
ON CREATE SET
  kw.value = "application pour lire qr code",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-pour-lire-qr-code-fr-fr-33ad94"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-telecharger-lecteur-qr-code-fr-fr-f5d76c"})
ON CREATE SET
  kw.value = "telecharger lecteur qr code",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-telecharger-lecteur-qr-code-fr-fr-f5d76c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-avec-iphone-fr-fr-b79911"})
ON CREATE SET
  kw.value = "scanner qr code avec iphone",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-avec-iphone-fr-fr-b79911"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-appli-scanner-qr-code-fr-fr-b28a08"})
ON CREATE SET
  kw.value = "appli scanner qr code",
  kw.volume = 90,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-appli-scanner-qr-code-fr-fr-b28a08"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-de-qr-code-gratuit-fr-fr-46ff61"})
ON CREATE SET
  kw.value = "scanner de qr code gratuit",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-de-qr-code-gratuit-fr-fr-46ff61"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-meilleur-lecteur-qr-code-iphone-gratuit-fr-fr-499a80"})
ON CREATE SET
  kw.value = "meilleur lecteur qr code iphone gratuit",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-meilleur-lecteur-qr-code-iphone-gratuit-fr-fr-499a80"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-code-qr-fr-fr-1fdb78"})
ON CREATE SET
  kw.value = "lire code qr",
  kw.volume = 90,
  kw.difficulty = 3,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-code-qr-fr-fr-1fdb78"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-lecteur-gratuit-fr-fr-a89279"})
ON CREATE SET
  kw.value = "qr code lecteur gratuit",
  kw.volume = 90,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-lecteur-gratuit-fr-fr-a89279"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-ordinateur-fr-fr-5a68f6"})
ON CREATE SET
  kw.value = "scanner qr code ordinateur",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-ordinateur-fr-fr-5a68f6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-de-codes-qr-français-fr-fr-2ac02f"})
ON CREATE SET
  kw.value = "scanner de codes qr (français)",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-de-codes-qr-français-fr-fr-2ac02f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-pc-fr-fr-6c3d9c"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur pc",
  kw.volume = 90,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-pc-fr-fr-6c3d9c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-retrouver-un-qr-code-déjà-scanner-fr-fr-3c9253"})
ON CREATE SET
  kw.value = "comment retrouver un qr code déjà scanner",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-retrouver-un-qr-code-déjà-scanner-fr-fr-3c9253"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner-pc-fr-fr-a0d1da"})
ON CREATE SET
  kw.value = "qr code scanner pc",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner-pc-fr-fr-a0d1da"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scannez-moi-qr-code-fr-fr-be939b"})
ON CREATE SET
  kw.value = "scannez moi qr code",
  kw.volume = 80,
  kw.difficulty = 3,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scannez-moi-qr-code-fr-fr-be939b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-de-qr-code-en-ligne-fr-fr-adf2f8"})
ON CREATE SET
  kw.value = "scanner de qr code en ligne",
  kw.volume = 80,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-de-qr-code-en-ligne-fr-fr-adf2f8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-huawei-fr-fr-487a1f"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur huawei",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-huawei-fr-fr-487a1f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-propre-iphone-fr-fr-1f8b59"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur son propre iphone",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-propre-iphone-fr-fr-1f8b59"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-scanner-app-fr-fr-8e2067"})
ON CREATE SET
  kw.value = "qr scanner app",
  kw.volume = 80,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-scanner-app-fr-fr-8e2067"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scaner-qr-code-fr-fr-27d17b"})
ON CREATE SET
  kw.value = "scaner qr code",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scaner-qr-code-fr-fr-27d17b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-un-ordinateur-fr-fr-53cbf5"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur un ordinateur",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-un-ordinateur-fr-fr-53cbf5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-les-qr-codes-fr-fr-ae3865"})
ON CREATE SET
  kw.value = "comment lire les qr codes",
  kw.volume = 80,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-les-qr-codes-fr-fr-ae3865"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-photo-fr-fr-3899c5"})
ON CREATE SET
  kw.value = "scan qr code photo",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-photo-fr-fr-3899c5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-me-qr-code-fr-fr-ea6acf"})
ON CREATE SET
  kw.value = "scan me qr code",
  kw.volume = 80,
  kw.difficulty = 31,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-me-qr-code-fr-fr-ea6acf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-ordinateur-fr-fr-e7883d"})
ON CREATE SET
  kw.value = "scanner qr code sur ordinateur",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-ordinateur-fr-fr-e7883d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-android-samsung-fr-fr-64319d"})
ON CREATE SET
  kw.value = "scanner qr code android samsung",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-android-samsung-fr-fr-64319d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-mon-téléphone-fr-fr-8ab0e9"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur mon téléphone",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-mon-téléphone-fr-fr-8ab0e9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-avec-iphone-fr-fr-f1dfbb"})
ON CREATE SET
  kw.value = "scanner un qr code avec iphone",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-avec-iphone-fr-fr-f1dfbb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-lire-fr-fr-ecdc71"})
ON CREATE SET
  kw.value = "qr code lire",
  kw.volume = 80,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-lire-fr-fr-ecdc71"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-android-fr-fr-f9908f"})
ON CREATE SET
  kw.value = "scanner un qr code sur android",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.1,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-android-fr-fr-f9908f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-the-qr-code-from-your-receipt-fr-fr-b7fccc"})
ON CREATE SET
  kw.value = "scan the qr code from your receipt",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.4,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-the-qr-code-from-your-receipt-fr-fr-b7fccc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-iphone-fr-fr-014f10"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec iphone",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-iphone-fr-fr-014f10"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-scanner-qr-code-gratuit-fr-fr-3458ad"})
ON CREATE SET
  kw.value = "télécharger scanner qr code gratuit",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-scanner-qr-code-gratuit-fr-fr-3458ad"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-android-fr-fr-20ec7c"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec android",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-android-fr-fr-20ec7c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-code-qr-gratuit-fr-fr-b94735"})
ON CREATE SET
  kw.value = "scanner code qr gratuit",
  kw.volume = 80,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-code-qr-gratuit-fr-fr-b94735"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lector-qr-fr-fr-7e94d2"})
ON CREATE SET
  kw.value = "lector qr",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lector-qr-fr-fr-7e94d2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-online-qr-code-reader-fr-fr-475611"})
ON CREATE SET
  kw.value = "online qr code reader",
  kw.volume = 80,
  kw.difficulty = 52,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-online-qr-code-reader-fr-fr-475611"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-fdj-fr-fr-e08582"})
ON CREATE SET
  kw.value = "scanner qr code fdj",
  kw.volume = 80,
  kw.difficulty = 15,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-fdj-fr-fr-e08582"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner-android-fr-fr-332235"})
ON CREATE SET
  kw.value = "qr code scanner android",
  kw.volume = 70,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner-android-fr-fr-332235"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanne-qr-code-fr-fr-16dc2a"})
ON CREATE SET
  kw.value = "scanne qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanne-qr-code-fr-fr-16dc2a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-iscannercomqr-fr-fr-942d06"})
ON CREATE SET
  kw.value = "iscanner.com/qr",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-iscannercomqr-fr-fr-942d06"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-flasher-un-qr-code-sur-son-propre-telephone-fr-fr-247e76"})
ON CREATE SET
  kw.value = "flasher un qr code sur son propre telephone",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-flasher-un-qr-code-sur-son-propre-telephone-fr-fr-247e76"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-en-ligne-fr-fr-685413"})
ON CREATE SET
  kw.value = "lecteur qr code gratuit en ligne",
  kw.volume = 70,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-en-ligne-fr-fr-685413"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-a-scanner-fr-fr-191206"})
ON CREATE SET
  kw.value = "qr code a scanner",
  kw.volume = 70,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-a-scanner-fr-fr-191206"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-où-se-trouve-le-lecteur-qr-code-sur-samsung-fr-fr-49b71d"})
ON CREATE SET
  kw.value = "où se trouve le lecteur qr code sur samsung",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-où-se-trouve-le-lecteur-qr-code-sur-samsung-fr-fr-49b71d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-pour-lire-les-qr-codes-fr-fr-cc2998"})
ON CREATE SET
  kw.value = "application pour lire les qr codes",
  kw.volume = 70,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-pour-lire-les-qr-codes-fr-fr-cc2998"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-samsung-fr-fr-ed761e"})
ON CREATE SET
  kw.value = "scan qr code samsung",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-samsung-fr-fr-ed761e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-avec-un-iphone-fr-fr-22185a"})
ON CREATE SET
  kw.value = "comment flasher un qr code avec un iphone",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-flasher-un-qr-code-avec-un-iphone-fr-fr-22185a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-quelle-application-pour-scanner-un-qr-code-fr-fr-8d6798"})
ON CREATE SET
  kw.value = "quelle application pour scanner un qr code",
  kw.volume = 70,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-quelle-application-pour-scanner-un-qr-code-fr-fr-8d6798"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-le-code-qr-fr-fr-1b1242"})
ON CREATE SET
  kw.value = "scanner le code qr",
  kw.volume = 70,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-le-code-qr-fr-fr-1b1242"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-pc-fr-fr-af257c"})
ON CREATE SET
  kw.value = "scanner qr code sur pc",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-pc-fr-fr-af257c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-pour-scanner-les-qr-codes-fr-fr-f1ea59"})
ON CREATE SET
  kw.value = "application pour scanner les qr codes",
  kw.volume = 70,
  kw.difficulty = 1,
  kw.cpc = 0.3,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-pour-scanner-les-qr-codes-fr-fr-f1ea59"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-photo-fr-fr-c02fa4"})
ON CREATE SET
  kw.value = "scanner qr code sur photo",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-photo-fr-fr-c02fa4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-scan-a-qr-code-on-your-screen-fr-fr-4a67be"})
ON CREATE SET
  kw.value = "how to scan a qr code on your screen",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-scan-a-qr-code-on-your-screen-fr-fr-4a67be"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-fr-fr-050ad9"})
ON CREATE SET
  kw.value = "comment scanner un qr",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-fr-fr-050ad9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-téléphone-fr-fr-00b159"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur son téléphone",
  kw.volume = 70,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-son-téléphone-fr-fr-00b159"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-online-qr-code-scanner-fr-fr-8bb8de"})
ON CREATE SET
  kw.value = "online qr code scanner",
  kw.volume = 70,
  kw.difficulty = 70,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-online-qr-code-scanner-fr-fr-8bb8de"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-xiaomi-fr-fr-4a3a0e"})
ON CREATE SET
  kw.value = "scanner qr code xiaomi",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-xiaomi-fr-fr-4a3a0e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-sur-samsung-fr-fr-92b889"})
ON CREATE SET
  kw.value = "comment scanner qr code sur samsung",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-sur-samsung-fr-fr-92b889"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-https-tf1frtv-scanner-qr-code-fr-fr-b6b958"})
ON CREATE SET
  kw.value = "https //tf1.fr/tv scanner qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-https-tf1frtv-scanner-qr-code-fr-fr-b6b958"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanning-qr-code-fr-fr-5d4bdf"})
ON CREATE SET
  kw.value = "scanning qr code",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanning-qr-code-fr-fr-5d4bdf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-samsung-fr-fr-4f64e5"})
ON CREATE SET
  kw.value = "scanner un qr code samsung",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-samsung-fr-fr-4f64e5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code--fr-fr-513738"})
ON CREATE SET
  kw.value = "comment scanner un qr code ?",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code--fr-fr-513738"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scannez-moi-qr-code-fr-fr-bfbad8"})
ON CREATE SET
  kw.value = "scannez-moi qr code",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scannez-moi-qr-code-fr-fr-bfbad8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-pc-fr-fr-b2999f"})
ON CREATE SET
  kw.value = "lire qr code pc",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-pc-fr-fr-b2999f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-mac-fr-fr-318f5c"})
ON CREATE SET
  kw.value = "scanner un qr code sur mac",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-un-qr-code-sur-mac-fr-fr-318f5c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-appli-lecteur-qr-code-fr-fr-c49263"})
ON CREATE SET
  kw.value = "appli lecteur qr code",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-appli-lecteur-qr-code-fr-fr-c49263"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-android-fr-fr-f50515"})
ON CREATE SET
  kw.value = "lecteur qr code gratuit android",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-qr-code-gratuit-android-fr-fr-f50515"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-un-qr-code-sur-son-propre-telephone-fr-fr-d471de"})
ON CREATE SET
  kw.value = "lire un qr code sur son propre telephone",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-un-qr-code-sur-son-propre-telephone-fr-fr-d471de"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-xiaomi-scanner-qr-code-fr-fr-ee79ca"})
ON CREATE SET
  kw.value = "xiaomi scanner qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-xiaomi-scanner-qr-code-fr-fr-ee79ca"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanneur-qr-code-fr-fr-4607c6"})
ON CREATE SET
  kw.value = "scanneur qr code",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanneur-qr-code-fr-fr-4607c6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-mac-fr-fr-11aff6"})
ON CREATE SET
  kw.value = "scanner qr code mac",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-mac-fr-fr-11aff6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-pour-pc-fr-fr-31a808"})
ON CREATE SET
  kw.value = "lecteur de qr code pour pc",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-pour-pc-fr-fr-31a808"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanette-qr-code-fr-fr-870961"})
ON CREATE SET
  kw.value = "scanette qr code",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Commercial,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanette-qr-code-fr-fr-870961"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-qr-code-samsung-fr-fr-4e2a62"})
ON CREATE SET
  kw.value = "lire qr code samsung",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-qr-code-samsung-fr-fr-4e2a62"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-code-qr-scanner-fr-fr-4b9f40"})
ON CREATE SET
  kw.value = "code qr scanner",
  kw.volume = 60,
  kw.difficulty = 3,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-code-qr-scanner-fr-fr-4b9f40"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-huawei-fr-fr-34a40c"})
ON CREATE SET
  kw.value = "scanner qr code huawei",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-huawei-fr-fr-34a40c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-lecteur-fr-fr-8838e4"})
ON CREATE SET
  kw.value = "qr lecteur",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-lecteur-fr-fr-8838e4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sans-appareil-photo-fr-fr-910aab"})
ON CREATE SET
  kw.value = "comment scanner un qr code sans appareil photo",
  kw.volume = 60,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sans-appareil-photo-fr-fr-910aab"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-scan-qr-code-fr-fr-38178b"})
ON CREATE SET
  kw.value = "how to scan qr code",
  kw.volume = 60,
  kw.difficulty = 19,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-scan-qr-code-fr-fr-38178b"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-samsung-scanner-qr-code-fr-fr-433591"})
ON CREATE SET
  kw.value = "samsung scanner qr code",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-samsung-scanner-qr-code-fr-fr-433591"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-faire-pour-lire-un-qr-code-fr-fr-3249fd"})
ON CREATE SET
  kw.value = "comment faire pour lire un qr code",
  kw.volume = 60,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-faire-pour-lire-un-qr-code-fr-fr-3249fd"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-google-scan-qr-code-fr-fr-3ce912"})
ON CREATE SET
  kw.value = "google scan qr code",
  kw.volume = 60,
  kw.difficulty = 9,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-google-scan-qr-code-fr-fr-3ce912"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lire-les-qr-code-fr-fr-79dce9"})
ON CREATE SET
  kw.value = "lire les qr code",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lire-les-qr-code-fr-fr-79dce9"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-google-scanner-qr-code-fr-fr-20f9e5"})
ON CREATE SET
  kw.value = "google scanner qr code",
  kw.volume = 60,
  kw.difficulty = 5,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-google-scanner-qr-code-fr-fr-20f9e5"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-lecteur-qr-code-fr-fr-4627ba"})
ON CREATE SET
  kw.value = "application lecteur qr code",
  kw.volume = 60,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-lecteur-qr-code-fr-fr-4627ba"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-online-fr-fr-b139b7"})
ON CREATE SET
  kw.value = "scanner qr code online",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-online-fr-fr-b139b7"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-samsung-fr-fr-d29d72"})
ON CREATE SET
  kw.value = "lecteur de qr code samsung",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-de-qr-code-samsung-fr-fr-d29d72"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-sur-android-fr-fr-732692"})
ON CREATE SET
  kw.value = "comment scanner qr code sur android",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-qr-code-sur-android-fr-fr-732692"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-la-télé-fr-fr-2823e2"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur la télé",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-la-télé-fr-fr-2823e2"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-son-propre-telephone-fr-fr-33e6fe"})
ON CREATE SET
  kw.value = "scanner qr code sur son propre telephone",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-son-propre-telephone-fr-fr-33e6fe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-lecteur-code-qr-gratuit-fr-fr-13148a"})
ON CREATE SET
  kw.value = "lecteur code qr gratuit",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-lecteur-code-qr-gratuit-fr-fr-13148a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-scan-a-qr-code-fr-fr-1c02a1"})
ON CREATE SET
  kw.value = "how to scan a qr code",
  kw.volume = 50,
  kw.difficulty = 47,
  kw.cpc = 0.09,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-scan-a-qr-code-fr-fr-1c02a1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-télécharger-lecteur-qr-code-gratuit-fr-fr-0639fe"})
ON CREATE SET
  kw.value = "télécharger lecteur qr code gratuit",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-télécharger-lecteur-qr-code-gratuit-fr-fr-0639fe"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-samsung-fr-fr-5e6ea4"})
ON CREATE SET
  kw.value = "scanner qr code sur samsung",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-samsung-fr-fr-5e6ea4"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-flasher-qr-code-fr-fr-8ee749"})
ON CREATE SET
  kw.value = "comment flasher qr code",
  kw.volume = 50,
  kw.difficulty = 2,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-flasher-qr-code-fr-fr-8ee749"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-photo-scanner-fr-fr-0070a0"})
ON CREATE SET
  kw.value = "qr photo scanner",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-photo-scanner-fr-fr-0070a0"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-que-veut-dire-scanner-un-qr-code-fr-fr-2fee52"})
ON CREATE SET
  kw.value = "que veut dire scanner un qr code",
  kw.volume = 50,
  kw.difficulty = 5,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-que-veut-dire-scanner-un-qr-code-fr-fr-2fee52"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-en-photo-fr-fr-7c0e3e"})
ON CREATE SET
  kw.value = "comment lire un qr code en photo",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-lire-un-qr-code-en-photo-fr-fr-7c0e3e"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-application-scan-qr-code-fr-fr-2dd0a1"})
ON CREATE SET
  kw.value = "application scan qr code",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-application-scan-qr-code-fr-fr-2dd0a1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-honor-fr-fr-8a52c6"})
ON CREATE SET
  kw.value = "scanner qr code honor",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-honor-fr-fr-8a52c6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-de-code-qr-fr-fr-aac92d"})
ON CREATE SET
  kw.value = "scanner de code qr",
  kw.volume = 50,
  kw.difficulty = 3,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-de-code-qr-fr-fr-aac92d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-les-qr-codes-fr-fr-3b544d"})
ON CREATE SET
  kw.value = "scanner les qr codes",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-les-qr-codes-fr-fr-3b544d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-une-photo-fr-fr-05244f"})
ON CREATE SET
  kw.value = "scanner qr code sur une photo",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scanner-qr-code-sur-une-photo-fr-fr-05244f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-un-smartphone-fr-fr-c32740"})
ON CREATE SET
  kw.value = "comment scanner un qr code avec un smartphone",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-avec-un-smartphone-fr-fr-c32740"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-telecharger-qr-scanner-fr-fr-09ddcf"})
ON CREATE SET
  kw.value = "telecharger qr scanner",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-telecharger-qr-scanner-fr-fr-09ddcf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-reader-online-fr-fr-f223a1"})
ON CREATE SET
  kw.value = "qr reader online",
  kw.volume = 50,
  kw.difficulty = 69,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-reader-online-fr-fr-f223a1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-scan-a-qr-code-on-your-phone-fr-fr-0a2887"})
ON CREATE SET
  kw.value = "how to scan a qr code on your phone",
  kw.volume = 50,
  kw.difficulty = 8,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-scan-a-qr-code-on-your-phone-fr-fr-0a2887"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-1-qr-code-fr-fr-7f8edf"})
ON CREATE SET
  kw.value = "comment scanner 1 qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-1-qr-code-fr-fr-7f8edf"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-how-to-scan-qr-code-on-iphone-fr-fr-c95879"})
ON CREATE SET
  kw.value = "how to scan qr code on iphone",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-how-to-scan-qr-code-on-iphone-fr-fr-c95879"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-les-qr-codes-fr-fr-480f13"})
ON CREATE SET
  kw.value = "comment scanner les qr codes",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-les-qr-codes-fr-fr-480f13"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-free-qr-code-scanner-fr-fr-a6c71c"})
ON CREATE SET
  kw.value = "free qr code scanner",
  kw.volume = 50,
  kw.difficulty = 55,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-free-qr-code-scanner-fr-fr-a6c71c"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-une-photo-fr-fr-5f346a"})
ON CREATE SET
  kw.value = "comment scanner un qr code sur une photo",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-comment-scanner-un-qr-code-sur-une-photo-fr-fr-5f346a"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scannez-moi-fr-fr-200b67"})
ON CREATE SET
  kw.value = "qr code scannez moi",
  kw.volume = 50,
  kw.difficulty = 1,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scannez-moi-fr-fr-200b67"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-iphone-scanner-qr-code-fr-fr-885e02"})
ON CREATE SET
  kw.value = "iphone scanner qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-iphone-scanner-qr-code-fr-fr-885e02"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scanner-moi-fr-fr-696ba8"})
ON CREATE SET
  kw.value = "qr code scanner moi",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scanner-moi-fr-fr-696ba8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-qr-code-iphone-gratuit-fr-fr-c2eaf3"})
ON CREATE SET
  kw.value = "scan qr code iphone gratuit",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-qr-code-iphone-gratuit-fr-fr-c2eaf3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-reader-gratuit-fr-fr-cd7a8f"})
ON CREATE SET
  kw.value = "qr code reader gratuit",
  kw.volume = 50,
  kw.difficulty = 2,
  kw.cpc = 0.15,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-reader-gratuit-fr-fr-cd7a8f"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-exemple-de-qr-code-à-scanner-fr-fr-a1dbd1"})
ON CREATE SET
  kw.value = "exemple de qr code à scanner",
  kw.volume = 50,
  kw.difficulty = 6,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-exemple-de-qr-code-à-scanner-fr-fr-a1dbd1"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-scan-me-fr-fr-2354ce"})
ON CREATE SET
  kw.value = "qr code scan me",
  kw.volume = 50,
  kw.difficulty = 4,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-scan-me-fr-fr-2354ce"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr--barcode-scanner-fr-fr-0965cb"})
ON CREATE SET
  kw.value = "qr & barcode scanner",
  kw.volume = 50,
  kw.difficulty = 58,
  kw.cpc = 0.25,
  kw.intent = "Informational,Commercial,Transactional,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr--barcode-scanner-fr-fr-0965cb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-scan-code-qr-fr-fr-fa3201"})
ON CREATE SET
  kw.value = "scan code qr",
  kw.volume = 50,
  kw.difficulty = 2,
  kw.cpc = 0.2,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "scan-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-scan-code-qr-fr-fr-fa3201"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-snapchat-fr-fr-40a430"})
ON CREATE SET
  kw.value = "qr code snapchat",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "snapchat", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-snapchat-fr-fr-40a430"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-snapchat-qr-code-fr-fr-4348cb"})
ON CREATE SET
  kw.value = "snapchat qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "snapchat", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-snapchat-qr-code-fr-fr-4348cb"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-spotify-fr-fr-fda594"})
ON CREATE SET
  kw.value = "qr code spotify",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.35,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "spotify", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-spotify-fr-fr-fda594"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-spotify-qr-code-fr-fr-dc41d8"})
ON CREATE SET
  kw.value = "spotify qr code",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "spotify", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-spotify-qr-code-fr-fr-dc41d8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-musique-spotify-fr-fr-3ebcf3"})
ON CREATE SET
  kw.value = "qr code musique spotify",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "spotify", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-musique-spotify-fr-fr-3ebcf3"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-statique-gratuit-fr-fr-bbb190"})
ON CREATE SET
  kw.value = "qr code statique gratuit",
  kw.volume = 100,
  kw.difficulty = 12,
  kw.cpc = 0.25,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "static-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-statique-gratuit-fr-fr-bbb190"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-statique-fr-fr-f3eca9"})
ON CREATE SET
  kw.value = "qr code statique",
  kw.volume = 90,
  kw.difficulty = 6,
  kw.cpc = 0.45,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "static-qr-code", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-statique-fr-fr-f3eca9"})
MERGE (el)-[:TARGETS]->(kw);



// --- telegram (8 keywords) ---

MERGE (kw:SEOKeyword {key: "seo-qr-code-telegram-fr-fr-9b1969"})
ON CREATE SET
  kw.value = "qr code telegram",
  kw.volume = 80,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "telegram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-telegram-fr-fr-9b1969"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-telegram-qr-code-fr-fr-2df207"})
ON CREATE SET
  kw.value = "telegram qr code",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "telegram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-telegram-qr-code-fr-fr-2df207"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-getdesktoptelegramorg-qr-code-fr-fr-22f874"})
ON CREATE SET
  kw.value = "getdesktop.telegram.org qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "telegram", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-getdesktoptelegramorg-qr-code-fr-fr-22f874"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-tiktok-fr-fr-17056d"})
ON CREATE SET
  kw.value = "qr code tiktok",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "tiktok", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-tiktok-fr-fr-17056d"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-tiktok-qr-code-fr-fr-34ff33"})
ON CREATE SET
  kw.value = "tiktok qr code",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "tiktok", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-tiktok-qr-code-fr-fr-34ff33"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-whatsapp-web-qr-code-fr-fr-5cb3b8"})
ON CREATE SET
  kw.value = "whatsapp web qr code",
  kw.volume = 350,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "whatsapp", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-whatsapp-web-qr-code-fr-fr-5cb3b8"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-whatsapp-fr-fr-fab5f6"})
ON CREATE SET
  kw.value = "qr code whatsapp",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.45,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "whatsapp", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-whatsapp-fr-fr-fab5f6"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-groupe-whatsapp-fr-fr-576669"})
ON CREATE SET
  kw.value = "qr code groupe whatsapp",
  kw.volume = 150,
  kw.difficulty = 0,
  kw.cpc = 0.3,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "whatsapp", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-groupe-whatsapp-fr-fr-576669"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-whatsapp-qr-code-fr-fr-e0e962"})
ON CREATE SET
  kw.value = "whatsapp qr code",
  kw.volume = 100,
  kw.difficulty = 0,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "whatsapp", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-whatsapp-qr-code-fr-fr-e0e962"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-whatsapp-groupe-fr-fr-a83034"})
ON CREATE SET
  kw.value = "qr code whatsapp groupe",
  kw.volume = 70,
  kw.difficulty = 0,
  kw.cpc = 0.0,
  kw.intent = "Informational,Non-branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "whatsapp", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-whatsapp-groupe-fr-fr-a83034"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-youtube-fr-fr-16aabc"})
ON CREATE SET
  kw.value = "qr code youtube",
  kw.volume = 300,
  kw.difficulty = 0,
  kw.cpc = 0.25,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "youtube", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-youtube-fr-fr-16aabc"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-youtube-qr-code-fr-fr-c42e25"})
ON CREATE SET
  kw.value = "youtube qr code",
  kw.volume = 80,
  kw.difficulty = 5,
  kw.cpc = 0.2,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "youtube", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-youtube-qr-code-fr-fr-c42e25"})
MERGE (el)-[:TARGETS]->(kw);


MERGE (kw:SEOKeyword {key: "seo-qr-code-musique-youtube-fr-fr-29805b"})
ON CREATE SET
  kw.value = "qr code musique youtube",
  kw.volume = 50,
  kw.difficulty = 0,
  kw.cpc = 0.15,
  kw.intent = "Informational,Branded,Non-local",
  kw.created_at = datetime(),
  kw.updated_at = datetime();

MATCH (el:EntityNative {entity_key: "youtube", locale_key: "fr-FR"})
MATCH (kw:SEOKeyword {key: "seo-qr-code-musique-youtube-fr-fr-29805b"})
MERGE (el)-[:TARGETS]->(kw);

