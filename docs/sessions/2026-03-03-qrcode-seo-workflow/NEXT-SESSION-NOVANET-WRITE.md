# Briefing: Session novanet_write

## Contexte

Cette session fait suite au brainstorm sur le workflow SEO Nika + NovaNet.

**Probleme identifie**: Le MCP NovaNet est READ-ONLY. Nika ne peut pas ecrire les SEOKeywords et mettre a jour EntityNative.

**Solution**: Ajouter un tool `novanet_write` au MCP NovaNet.

## Objectif

Implementer `novanet_write` dans `tools/novanet-mcp/` pour permettre:

1. `create_seo_keyword` - Creer un SEOKeyword avec vrais volumes
2. `update_entity_native` - Mettre a jour denomination_forms
3. `create_block_native` - Creer un BlockNative genere

## Fichiers a modifier

```
tools/novanet-mcp/
├── src/
│   ├── tools/
│   │   ├── mod.rs          # Ajouter write module
│   │   └── write.rs        # NOUVEAU: operations d'ecriture
│   └── lib.rs              # Register le nouveau tool
└── Cargo.toml
```

## Reference: Tools existants

Regarder comment sont implementes les autres tools:
- `tools/novanet-mcp/src/tools/query.rs` (lecture Cypher)
- `tools/novanet-mcp/src/tools/describe.rs` (describe entities)

## Securite

- Valider les donnees avant ecriture
- Verifier que les nodes cibles existent
- Retourner des erreurs claires

## Session precedente

Voir: `docs/sessions/2026-03-03-qrcode-seo-workflow/`
