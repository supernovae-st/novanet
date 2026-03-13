// Fulltext and other indexes for NovaNet
// ═══════════════════════════════════════════════════════════════════════════════
// Creates fulltext index required by novanet_search MCP tool.
// Runs AFTER all data seeds, BEFORE autowire.
// Idempotent: CREATE ... IF NOT EXISTS.
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────
// Fulltext index: novanet_fulltext
// Used by: novanet_search (fulltext + hybrid modes)
// Properties: key, display_name, content
// ─────────────────────────────────────────────────────────────────────────────

CREATE FULLTEXT INDEX novanet_fulltext IF NOT EXISTS
FOR (n:Entity|EntityNative|Page|PageNative|Block|BlockNative|Project|ProjectNative|Brand|BrandDesign|SEOKeyword|Expression|Pattern|CultureRef|Taboo|AudienceTrait|Locale|EntityCategory|BlockType|ContentSlot|PromptStyle|PromptArtifact|OutputArtifact|OrgConfig|GEOQuery|GEOAnswer)
ON EACH [n.key, n.display_name, n.content];
