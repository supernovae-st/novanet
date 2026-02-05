# NovaNet Publication Strategy

**Status**: Draft - To revisit later
**Date**: 2026-02-05
**License Decision**: AGPL-3.0 (copyleft, forces contributions)

---

## Why AGPL?

- Forces SaaS providers to contribute back
- Prevents private use without sharing improvements
- Strong copyleft for the knowledge graph architecture

---

## Unique Innovations (Publishable)

1. **Knowledge Atoms + Selective Loading**
   - Granular nodes (Term, Expression, Pattern, Taboo, CultureRef, AudienceTrait)
   - Load 50 relevant atoms vs 20K blob
   - `[:USES_TERM]` links on content nodes

2. **Native Generation vs Translation**
   - Entity (invariant) -> Generate -> EntityL10n (locale-native)
   - NOT: Source -> Translate -> Target
   - 200+ locales with different knowledge sizes

3. **Faceted Self-Describing Ontology**
   - 4 axes: Realm/Layer/Trait/ArcFamily
   - Meta-graph that documents itself
   - OWL-inspired arc schema (FROM_KIND, TO_KIND, IN_FAMILY)

4. **Complete Toolchain**
   - Rust CLI + TUI (245 tests)
   - Neo4j infrastructure
   - React Flow Studio visualization

---

## Market Analysis (Feb 2026)

| Existing | What They Do | NovaNet Difference |
|----------|--------------|-------------------|
| Microsoft GraphRAG | Generic retrieval | No faceted ontology, no multi-locale |
| OG-RAG | Ontology-grounded QA | QA focus, not content generation |
| GraphToken, VKGs | KG injection in LLM | No selective atom loading |
| i18n, CAT tools | Translation-based | Not native generation |

**Conclusion**: No one combines all four innovations.

---

## Publication Strategy (Revised)

```
1. Finir NovaNet + intégration QR Code AI
         ↓
2. v0.1 GitHub (AGPL-3.0) + paper PDF dans le repo
         ↓
3. Prouver en prod (metrics, screenshots)
         ↓
4. Hacker News ("Show HN")
         ↓
5. ArXiv
```

---

## ArXiv - Pas besoin de diplôme

**Requis**:
- Email valide
- Paper en LaTeX ou PDF
- "Endorsement" (première soumission uniquement)

**Endorsement**:
- Quelqu'un qui a déjà publié sur ArXiv te "sponsorise" (1 clic)
- OU certaines catégories (cs.CL, cs.AI) sont "auto-endorse"
- Demander sur Twitter/X si besoin - chercheurs sympas endorsent les bons travaux

**Affiliation**:
- "Independent Researcher" ou "SuperNovae Studio"
- Aucun diplôme vérifié - le travail parle de lui-même

---

### Paper Details

**Suggested Title**:
> "NovaNet: A Faceted Knowledge Graph for Selective LLM Context Loading in Multi-Locale Native Content Generation"

**Draft Abstract**:
> We present NovaNet, a self-describing knowledge graph architecture that enables selective context loading for LLM-powered content generation across 200+ locales. Unlike traditional translation-based localization, NovaNet generates content natively using locale-specific knowledge atoms (Terms, Expressions, Patterns, Taboos). Our faceted ontology (Realm/Layer/Trait/ArcFamily) allows loading 50 relevant atoms instead of 20K-token knowledge blobs, reducing context pollution while improving cultural appropriateness. We provide a complete toolchain (Rust CLI, Neo4j, React visualization) and demonstrate the approach on a production QR code landing page generation system.

### Phase 2: Open Source (after ArXiv)

- GitHub release with AGPL-3.0
- README cites the paper
- Contributing guidelines

### Phase 3: Promotion

- Hacker News: "Show HN: NovaNet - Knowledge Graph for Native Content Generation"
- Twitter/X thread
- Link to paper + repo

### Phase 4: Conference (optional)

- EMNLP 2025 or ACL 2025
- Requires benchmarks:
  - Selective loading vs full blob (time, tokens, quality)
  - Native gen vs translation quality metrics

---

## Benchmarks Needed (for conference paper)

1. **Context Efficiency**
   - Tokens loaded: selective (50 atoms) vs blob (20K)
   - Generation time comparison

2. **Quality Metrics**
   - Human evaluation: cultural appropriateness
   - Automated: BLEU/COMET vs reference translations
   - A/B test on real traffic (QR Code AI)

3. **Scalability**
   - 200 locales x N pages generation time
   - Neo4j query performance

---

## Next Steps (when revisiting)

- [ ] **Finir NovaNet** - toutes les features core
- [ ] **Intégrer avec QR Code AI** - prouver que ça marche en prod
- [ ] **v0.1 GitHub** - AGPL-3.0 + paper PDF dans le repo
- [ ] **Collecter metrics** - screenshots, stats de génération
- [ ] **Hacker News** - "Show HN: NovaNet - Knowledge Graph for Native Content Generation"
- [ ] **ArXiv** - trouver endorsement si nécessaire, soumettre
- [ ] (Optionnel) Conférence - EMNLP/ACL si benchmarks solides
