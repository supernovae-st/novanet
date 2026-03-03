# Plan du Workflow Nika

## Objectif

Creer un workflow Nika qui:
1. Lit l'Entity depuis NovaNet
2. Recherche les vrais termes via Ahrefs/Perplexity
3. Ecrit les SEOKeywords dans NovaNet
4. Met a jour EntityNative.denomination_forms

## Workflow propose

```yaml
# recherche-seo.nika.yaml
schema: nika/workflow@0.9
workflow: recherche-seo-entity

description: |
  Recherche les vrais termes SEO pour une Entity dans une Locale,
  puis met a jour NovaNet avec les resultats.

inputs:
  entity_key:
    type: string
    description: Cle de l'Entity (ex: "qr-code")
  locale:
    type: string
    description: Code locale BCP47 (ex: "fr-FR")

mcp:
  servers:
    - novanet      # depuis ~/.spn/mcp.yaml
    - perplexity
    - ahrefs

tasks:
  # Etape 1: Lire l'Entity depuis NovaNet
  - id: load_entity
    invoke:
      mcp: novanet
      tool: novanet_describe
      params:
        describe: entity
        entity_key: $inputs.entity_key
    use.ctx: entity_data

  # Etape 2: Rechercher keywords via Ahrefs
  - id: search_ahrefs
    invoke:
      mcp: ahrefs
      tool: keywords_search
      params:
        seed: $entity_data.display_name
        country: $inputs.locale.split('-')[1].lower()
        limit: 20
    use.ctx: ahrefs_results

  # Etape 3: Valider avec Perplexity
  - id: validate_perplexity
    infer: |
      Analyse les resultats Ahrefs pour $inputs.locale:
      $ahrefs_results

      Questions:
      1. Quel terme est VRAIMENT utilise dans cette locale?
      2. Y a-t-il des anglicismes preferes aux traductions?
      3. Quels sont les termes a eviter?

      Retourne un JSON avec:
      - primary_term: le terme principal
      - slug_form: la forme slug recommandee
      - avoid_terms: les termes a eviter
    use.ctx: perplexity_validation

  # Etape 4: Creer les SEOKeywords (TODO: besoin write access)
  - id: create_keywords
    invoke:
      mcp: novanet
      tool: novanet_write  # ⚠️ N'EXISTE PAS ENCORE
      params:
        operation: create_seo_keywords
        data: $ahrefs_results
        locale: $inputs.locale
    use.ctx: created_keywords

  # Etape 5: Mettre a jour EntityNative (TODO: besoin write access)
  - id: update_entity_native
    invoke:
      mcp: novanet
      tool: novanet_write  # ⚠️ N'EXISTE PAS ENCORE
      params:
        operation: update_entity_native
        entity_key: $inputs.entity_key
        locale: $inputs.locale
        denomination_forms:
          url: $perplexity_validation.slug_form
    use.ctx: updated_entity

outputs:
  entity: $entity_data
  keywords: $ahrefs_results
  validation: $perplexity_validation
  updated_native: $updated_entity
```

## Probleme: Ecriture dans NovaNet

Le workflow ci-dessus a besoin d'ecrire dans NovaNet, mais le MCP actuel est READ-ONLY.

### Option A: Ajouter `novanet_write` au MCP

```rust
// tools/novanet-mcp/src/tools/write.rs
pub async fn novanet_write(params: WriteParams) -> Result<WriteResult> {
    match params.operation {
        "create_seo_keyword" => create_seo_keyword(params.data),
        "update_entity_native" => update_entity_native(params.entity_key, params.data),
        "create_block_native" => create_block_native(params.data),
        _ => Err(Error::UnknownOperation),
    }
}
```

### Option B: Utiliser Neo4j MCP directement

Nika pourrait utiliser le MCP Neo4j standard (`@neo4j/mcp-server`) pour les ecritures:

```yaml
mcp:
  servers:
    - novanet      # lecture (tools intelligents)
    - neo4j        # ecriture (Cypher brut)
```

### Option C: Workflow en deux phases

1. **Phase recherche** (Nika): collecte les donnees
2. **Phase ecriture** (CLI NovaNet ou script): ecrit dans Neo4j

## Recommandation

**Option A** est la meilleure pour le long terme:
- Garde la logique metier dans NovaNet
- Valide les donnees avant ecriture
- Maintient la coherence du graphe

**Necessaire**: Ouvrir une session separee pour implementer `novanet_write`.
