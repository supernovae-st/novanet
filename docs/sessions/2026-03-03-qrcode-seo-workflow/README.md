# Session: QR Code SEO Workflow avec Nika + NovaNet

**Date**: 2026-03-03 | **Status**: Design Complete, Implementation Pending

---

## Quick Start

```bash
# Lire le resume executif
cat docs/sessions/2026-03-03-qrcode-seo-workflow/00-EXECUTIVE-SUMMARY.md

# Reprendre le contexte
cat docs/sessions/2026-03-03-qrcode-seo-workflow/.claude/context.md

# Voir Neo4j
cargo run -- tui
```

---

## Objectif

Prouver que NovaNet + Nika peut:
1. **Rechercher** les VRAIS termes SEO par locale (pas deviner "code QR")
2. **Ecrire** les resultats dans NovaNet (SEOKeyword, EntityNative, Term)
3. **Generer** du contenu avec les vrais termes ("QR code" en francais)

---

## Structure des Fichiers

```
docs/sessions/2026-03-03-qrcode-seo-workflow/
├── 00-EXECUTIVE-SUMMARY.md      ← LIRE EN PREMIER
├── README.md                    ← Ce fichier
├── .claude/
│   ├── context.md               ← Contexte pour reprendre
│   └── skills/
│       └── seo-workflow.md      ← Skill pour le workflow
│
├── CONCEPTION/
│   ├── 01-brainstorm.md         ← Probleme et concepts cles
│   ├── 02-architecture.md       ← Schemas ASCII
│   └── 07-deep-architecture.md  ← Architecture profonde NovaNet
│
├── ETAT/
│   ├── 03-etat-neo4j.md         ← Snapshot base de donnees
│   └── 04-mcp-tools.md          ← Outils MCP disponibles
│
├── IMPLEMENTATION/
│   ├── 05-workflow-plan.md      ← Plan workflow Nika
│   ├── 06-questions-ouvertes.md ← Questions a resoudre
│   ├── 07-brainstorm-novanet-write.md ← Design novanet_write
│   └── NEXT-SESSION-NOVANET-WRITE.md  ← Briefing autre terminal
│
└── FORMULAS.md                  ← Formules de scoring (nouveau)
```

---

## Etat Actuel

### Neo4j
| Node | Status |
|------|--------|
| Entity:qr-code | Existe (130+ entities QR) |
| EntityNative:qr-code@fr-FR | N'existe pas |
| SEOKeyword fr-FR | 0 (vide) |
| Term fr-FR | 0 (vide) |
| Block:qr-code-head-seo-meta | Cree (order=0) |
| Locale:fr-FR → Slugification | slug_rule: latin_preserve |

### Bloquant
- **novanet_write** n'existe pas → autre terminal en cours

---

## Le Piege a Eviter

```
FAUX: Claude devine "code QR" basé sur son training
VRAI: Ahrefs montre que "QR code" = 110K/mois en France

Le workflow Nika doit DECOUVRIR, pas DEVINER.
```

---

## Prochaines Actions

1. [ ] novanet_write (autre terminal)
2. [ ] Configurer Ahrefs MCP
3. [ ] Test fr-FR avec qr-code
4. [ ] Scale a 200 locales

---

## Commandes Utiles

```bash
# Verifier Neo4j
cargo run -- tui

# Lancer MCP NovaNet
cd tools/novanet-mcp && cargo run

# Voir config MCP
cat ~/.spn/mcp.yaml
```
