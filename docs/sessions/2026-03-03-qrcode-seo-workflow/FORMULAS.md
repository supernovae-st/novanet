# Formules de Scoring SEO

## Formule Principale

```
score = volume x sem_coef x intent_boost x trend_factor
```

---

## Coefficients Semantiques (sem_coef)

Relation entre le keyword et l'Entity cible.

| Relation | Coefficient | Exemple (Entity: qr-code) |
|----------|-------------|---------------------------|
| `same_as` | 1.00 | "qr code" |
| `action_for` | 0.95 | "creer qr code", "generer qr code" |
| `produces` | 0.85 | "generateur qr", "qr code maker" |
| `subtopic_of` | 0.70 | "qr code wifi", "qr code menu" |
| `related_to` | 0.50 | "flashcode", "code-barres 2D" |
| `attribute_of` | 0.30 | "qr code couleur", "qr code taille" |

---

## Intent Boost (intent_boost)

Basé sur l'intention de recherche detectee par Ahrefs.

| Intent | Boost | Description |
|--------|-------|-------------|
| `transactional` | 1.20 | L'utilisateur veut agir (acheter, creer) |
| `commercial` | 1.10 | L'utilisateur compare des options |
| `informational` | 1.00 | L'utilisateur veut apprendre |
| `navigational` | 0.80 | L'utilisateur cherche un site specifique |

---

## Trend Factor (trend_factor)

Basé sur la tendance de recherche.

| Trend | Factor | Description |
|-------|--------|-------------|
| `rising` | 1.15 | Volume en hausse |
| `stable` | 1.00 | Volume stable |
| `declining` | 0.85 | Volume en baisse |

---

## Exemple de Calcul

**Locale**: fr-FR
**Entity**: qr-code

| Keyword | Volume | sem_coef | intent | trend | Score |
|---------|--------|----------|--------|-------|-------|
| "qr code" | 110,000 | 1.00 (same_as) | 1.00 (info) | 1.00 (stable) | **110,000** |
| "creer qr code" | 18,000 | 0.95 (action_for) | 1.20 (trans) | 1.00 | **20,520** |
| "generateur qr" | 22,000 | 0.85 (produces) | 1.20 (trans) | 1.00 | **22,440** |
| "code qr" | 8,500 | 1.00 (same_as) | 1.00 (info) | 0.85 (decl) | **7,225** |
| "flashcode" | 2,100 | 0.50 (related) | 1.00 (info) | 0.85 (decl) | **893** |

**Gagnant**: "qr code" avec score 110,000

---

## Notes

1. **Volume** vient directement d'Ahrefs (moyenne 12 mois)
2. **sem_coef** est determine par analyse semantique (Nika infer)
3. **intent** et **trend** viennent d'Ahrefs
4. **Le keyword gagnant determine**:
   - `EntityNative.denomination_forms.text`
   - `EntityNative.denomination_forms.url` (via slugification)
   - Le ranking des TARGETS arcs

---

## Convergence Boost (optionnel)

Si un keyword est pertinent pour plusieurs Entities, il peut recevoir un boost:

```
convergence_boost = 1 + (0.1 x (entity_count - 1))

Exemple: "qr code gratuit" pertinent pour:
- qr-code (primary)
- free-qr-generator (secondary)
- create-qr-code (secondary)

convergence_boost = 1 + (0.1 x 2) = 1.2
```

Ce boost favorise les keywords qui servent plusieurs pages (economie de contenu).
