# Brainstorm Initial

## Le Probleme

### Ce qu'on veut eviter
```
┌─────────────────────────────────────────────────────────────────────────────────┐
│  LE PIEGE                                                                       │
├─────────────────────────────────────────────────────────────────────────────────┤
│                                                                                 │
│  MAUVAISE APPROCHE:                                                             │
│  ┌──────────────────────────────────────────────────────────────────────────┐   │
│  │  "Deviner ou traduire les termes"                                        │   │
│  │  → "code QR" en francais (FAUX! personne ne cherche ca)                  │   │
│  │  → Traduire au lieu de generer nativement                                │   │
│  │  → Inventer des keywords sans data reelle                                │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
│  BONNE APPROCHE:                                                                │
│  ┌──────────────────────────────────────────────────────────────────────────┐   │
│  │  "Aller chercher la VRAIE data via Ahrefs/Perplexity"                    │   │
│  │  → Decouvrir que "QR code" est utilise en francais (pas "code QR")       │   │
│  │  → Voir les vrais volumes de recherche                                   │   │
│  │  → Comprendre les subtilites locales                                     │   │
│  └──────────────────────────────────────────────────────────────────────────┘   │
│                                                                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

## Les concepts cles

### Entity = Invariant (le QUOI)
- Defini une fois en anglais
- Ne change jamais
- `key: "qr-code"`, `display_name: "QR Code"`
- `denomination_forms`: formes prescriptives pour le LLM

### EntityNative = Comment on dit VRAIMENT (par locale)
- Un par locale (fr-FR, de-DE, ja-JP, etc.)
- Contient les VRAIS termes utilises localement
- `denomination_forms.url`: le slug derive des vraies recherches
- Associe aux SEOKeywords avec vrais volumes

### Page = Structure invariante
- Une Page = Une Entity (relation 1:1 via REPRESENTS)
- `slug`: segment URL invariant (anglais)
- Contient des Blocks ordonnes

### Block:head-seo-meta = Le premier bloc (order=0)
- DOIT etre le premier bloc de chaque page
- Possede le slug LOCALISE final
- Contient: `slug`, `meta_title`, `meta_description`

### BlockNative = Contenu genere par locale
- Un par locale
- `content.slug`: le slug localise derive de EntityNative.denomination_forms.url

## Le cycle Nika + NovaNet

```
Nika UTILISE NovaNet (lecture) + AMELIORE NovaNet (ecriture)

1. Nika lit Entity:qr-code depuis NovaNet
2. Nika recherche la vraie data via Ahrefs/Perplexity
3. Nika ECRIT dans NovaNet:
   - SEOKeywords avec vrais volumes
   - EntityNative avec vrais termes
   - Liens [:TARGETS] vers les bons keywords
4. Keywords peuvent etre lies a PLUSIEURS EntityNatives
   (convergence boost dans la formule de choix)
```

## Formule de choix du slug

```
score = volume × sem_coef × convergence_boost
```

| Type de lien | sem_coef | Description |
|--------------|----------|-------------|
| same_as | 1.0 | Synonyme parfait |
| action_for | 0.95 | L'action equivaut a l'outil |
| produces | 0.85 | Le resultat represente l'outil |
| subtopic_of | 0.7 | Sous-theme |
| related_to | 0.5 | Relation generale |
| attribute_of | 0.3 | Attribut (penalise!) |

### Exemple concret

| Keyword | Volume | sem_coef | convergence | Score |
|---------|--------|----------|-------------|-------|
| "qr code gratuit" | 90,000 | 0.3 | 1.0 | 27,000 |
| "creer qr code" | 74,000 | 0.95 | 1.2 | **84,360** |

→ "creer qr code" GAGNE meme si "gratuit" a plus de volume!
→ Car "gratuit" est un ATTRIBUT, pas un synonyme.

## Questions de Thibaut

1. "Il faut toujours que le Block head-seo-meta soit order=0"
   → OUI, confirme dans BlockType definition

2. "Une entite est egale a une page"
   → OUI, relation 1:1 via REPRESENTS (Page → Entity)

3. "Il y a une formule de choix qui n'est pas juste le volume"
   → OUI, `score = volume × sem_coef × convergence_boost`

4. "Nika va utiliser la data existante de NovaNet ET l'ameliorer"
   → OUI, c'est le cycle vertueux:
   - Lire Entity depuis NovaNet
   - Rechercher via Ahrefs/Perplexity
   - Ecrire les nouveaux SEOKeywords + MAJ EntityNative

5. "Les keywords appartiennent a une native et peuvent etre associes a plusieurs"
   → OUI, un SEOKeyword peut etre [:TARGETS] par plusieurs EntityNatives
   → C'est le "convergence boost" dans la formule
