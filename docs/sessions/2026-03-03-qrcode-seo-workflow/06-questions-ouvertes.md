# Questions Ouvertes

## Questions a resoudre

### 1. Ecriture dans NovaNet via MCP

**Question**: Le MCP NovaNet actuel est READ-ONLY. Faut-il ajouter un tool `novanet_write`?

**Options**:
- A) Ajouter `novanet_write` au MCP NovaNet
- B) Utiliser Neo4j MCP directement pour les ecritures
- C) Workflow en deux phases (recherche puis ecriture separee)

**Recommandation**: Option A (session separee necessaire)

**Action**: Ouvrir une session pour implementer `novanet_write`

---

### 2. Configuration Ahrefs MCP

**Question**: La cle API Ahrefs est-elle configuree?

**Verification**:
```bash
cat ~/.spn/mcp.yaml | grep -A5 ahrefs
```

**Action**: Configurer dans `~/.spn/mcp.yaml` si absent

---

### 3. Scope du test initial

**Question**: Tester sur 1 locale (fr-FR) ou plusieurs?

**Options**:
- 1 locale: fr-FR (plus simple, valide le concept)
- 5 locales: fr-FR, de-DE, es-ES, ja-JP, zh-CN (couvre cas varies)

**Recommandation**: Commencer par fr-FR, puis etendre

---

### 4. Les 10 keywords pour qr-code

**Question**: Quels keywords rechercher exactement?

**Methode**: Le workflow Nika va les DECOUVRIR via Ahrefs, pas les definir a l'avance.

**Seed**: "qr code" → Ahrefs retourne les 20 premiers keywords associes

---

### 5. Formule de choix: coefficients a valider

**Question**: Les coefficients sem_coef sont-ils corrects?

| Type | Coefficient |
|------|-------------|
| same_as | 1.0 |
| action_for | 0.95 |
| produces | 0.85 |
| subtopic_of | 0.7 |
| related_to | 0.5 |
| attribute_of | 0.3 |

**Source**: `docs/plans/2026-02-14-v0125-architecture-visual.md`

**Action**: Valider avec Thibaut

---

## Actions pour la prochaine session

1. [ ] Configurer Ahrefs MCP dans `~/.spn/mcp.yaml`
2. [ ] Ouvrir session pour implementer `novanet_write`
3. [ ] Creer le workflow Nika de base (lecture seule d'abord)
4. [ ] Tester recherche Ahrefs sur fr-FR
5. [ ] Valider le concept avant d'etendre
