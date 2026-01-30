# NovaNet Neo4j Architecture (v7.0.0)

Architecture complète du graphe Neo4j avec données réelles du projet QR Code AI.

> **v7.0.0**: Standard node properties (key, display_name, icon, description, llm_context). Graph-native Locale with :FOR_LOCALE. Locale Knowledge nodes (LocaleIdentity, LocaleVoice, LocaleCulture, LocaleMarket, LocaleLexicon). :TARGETS split to :TARGETS_SEO/:TARGETS_GEO.
> **v7.2.2**: BrandL10n merged into ProjectL10n. VoiceExampleL10n removed (redundant with LocaleVoice).
>
> **v6.8**: ConceptContent → ConceptL10n, BlockContent → BlockL10n, PageContent → PageL10n, :HAS_CONTENT split into :HAS_L10N and :HAS_OUTPUT
>
> **v6.7**: Project node added, :ASSEMBLES removed
>
> **v6.5**: L10NType REMOVED, SEMANTIC_LINK simplified (10 types)

---

## 1. Vue d'ensemble

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#3b82f6', 'primaryTextColor': '#1e293b', 'primaryBorderColor': '#1d4ed8', 'lineColor': '#64748b', 'secondaryColor': '#f0f9ff', 'tertiaryColor': '#f8fafc', 'background': '#ffffff' }}}%%
flowchart TB
    subgraph PROJECT_ROOT["🏢 PROJECT"]
        direction TB
        PRJ[Project]
    end

    subgraph PAGE_GRAPH["📄 PAGE GRAPH"]
        direction TB
        P[Page]
        PO[PageL10n]
        PM[PageMetrics]
        B[Block]
        BO[BlockL10n]
        BT[BlockType]
    end

    subgraph CONCEPT_GRAPH["💡 CONCEPT GRAPH"]
        direction TB
        C[Concept]
        CL[ConceptL10n]
        SEO[SEOKeywordL10n]
        GEO[GEOSeedL10n]
    end

    subgraph L10N_SYSTEM["🌍 L10N SYSTEM (v6.5 simplified)"]
        direction TB
        LC[L10NContent]
    end

    %% Project Relations (v6.7)
    PRJ -->|:HAS_PAGE| P
    PRJ -->|:HAS_CONCEPT| C
    PRJ -->|:HAS_L10N| LC

    %% Page Graph Relations (v6.8: :HAS_OUTPUT replaces :HAS_CONTENT for Page/Block)
    P -->|:HAS_OUTPUT| PO
    P -->|:HAS_METRICS| PM
    P -->|:HAS_BLOCK| B
    P -->|:USES_CONCEPT| C
    B -->|:OF_TYPE| BT
    B -->|:HAS_OUTPUT| BO

    %% v6.5: BlockType.l10n_categories[] references L10NContent.category (no relation)
    BT -.->|"l10n_categories[]"| LC

    %% Block → Concept
    B -->|:USES_CONCEPT| C

    %% Concept Graph Relations (v7.0.0: :HAS_L10N, :TARGETS_SEO, :TARGETS_GEO)
    C -->|:HAS_L10N| CL
    C -->|:SEMANTIC_LINK| C
    C -->|:TARGETS_SEO| SEO
    C -->|:TARGETS_GEO| GEO

    classDef projectNode fill:#fce7f3,stroke:#db2777,stroke-width:2px,color:#9d174d
    classDef pageNode fill:#dbeafe,stroke:#2563eb,stroke-width:2px,color:#1e40af
    classDef conceptNode fill:#dcfce7,stroke:#16a34a,stroke-width:2px,color:#166534
    classDef l10nNode fill:#fef3c7,stroke:#d97706,stroke-width:2px,color:#92400e
    classDef queryNode fill:#f3e8ff,stroke:#9333ea,stroke-width:2px,color:#6b21a8
    classDef metricsNode fill:#e0e7ff,stroke:#6366f1,stroke-width:2px,color:#4338ca

    class PRJ projectNode
    class P,PO,B,BO,BT pageNode
    class PM metricsNode
    class C,CL conceptNode
    class LC l10nNode
    class SEO,GEO queryNode
```

### Légende des couleurs

| Couleur | Domaine | Nodes |
|---------|---------|-------|
| 🩷 Rose | Project | Project (v6.7: new root node) |
| 🔵 Bleu | Page Graph | Page, PageL10n, Block, BlockL10n, BlockType |
| 🟣 Indigo | Metrics | PageMetrics (v6.8: new) |
| 🟢 Vert | Concept Graph | Concept, ConceptL10n (v6.8: renamed from ConceptContent) |
| 🟡 Orange | L10N System | L10NContent (v6.5: L10NType removed) |
| 💜 Violet | SEO/GEO | SEOKeywordL10n, GEOSeedL10n (v6.6: renamed) |

---

## 2. Page Graph - Données réelles

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#3b82f6', 'primaryTextColor': '#1e293b', 'primaryBorderColor': '#1d4ed8', 'lineColor': '#64748b', 'secondaryColor': '#f0f9ff', 'tertiaryColor': '#f8fafc' }}}%%
flowchart TB
    subgraph PROJECT["🏢 Project: qrcode-ai"]
        PRJ["<b>Project</b><br/>key: project-qrcode-ai<br/>name: QR Code AI"]
    end

    subgraph PRICING_PAGE["📄 Page: pricing"]
        P_PRICING["<b>Page</b><br/>key: pricing<br/>instructions: [GENERATE] pricing overview...<br/>llm_context: Triggers: pricing, tarifs..."]

        subgraph PRICING_OUTPUT["PageL10n par locale"]
            PO_FR["<b>PageL10n fr-FR</b><br/>slug: tarifs<br/>meta_title: Tarifs QR Code AI..."]
            PO_EN["<b>PageL10n en-US</b><br/>slug: pricing<br/>meta_title: QR Code AI Pricing..."]
        end

        subgraph PRICING_METRICS["PageMetrics (v6.8)"]
            PM_FR["<b>PageMetrics fr-FR</b><br/>views: 12,450<br/>conversions: 234"]
        end

        subgraph PRICING_BLOCKS["Blocks"]
            B_HERO["<b>Block: hero-pricing</b><br/>position: 1<br/>instructions: [GENERATE] title..."]
            B_FAQ["<b>Block: faq-pricing</b><br/>position: 2<br/>instructions: [GENERATE] questions..."]
        end
    end

    subgraph BLOCK_TYPES["BlockTypes"]
        BT_HERO["<b>BlockType: hero_main</b><br/>category: hero<br/>rules: TITLE max 70 chars..."]
        BT_FAQ["<b>BlockType: faq</b><br/>category: footer<br/>rules: 5-8 questions..."]
    end

    subgraph BLOCK_OUTPUT["BlockL10n générés (v6.8: renamed from BlockContent)"]
        BO_HERO_FR["<b>BlockL10n fr-FR</b><br/>position: 1<br/>title: Choisissez le plan...<br/>button: Commencer gratuitement"]
        BO_HERO_EN["<b>BlockL10n en-US</b><br/>position: 1<br/>title: Choose the QR Code AI...<br/>button: Start for Free"]
    end

    %% Project Relations (v6.7)
    PRJ -->|:HAS_PAGE| P_PRICING

    %% Relations (v6.8: :HAS_OUTPUT replaces :HAS_CONTENT)
    P_PRICING -->|:HAS_OUTPUT| PO_FR
    P_PRICING -->|:HAS_OUTPUT| PO_EN
    P_PRICING -->|:HAS_METRICS| PM_FR
    P_PRICING -->|:HAS_BLOCK| B_HERO
    P_PRICING -->|:HAS_BLOCK| B_FAQ

    B_HERO -->|:OF_TYPE| BT_HERO
    B_FAQ -->|:OF_TYPE| BT_FAQ

    B_HERO -->|:HAS_OUTPUT| BO_HERO_FR
    B_HERO -->|:HAS_OUTPUT| BO_HERO_EN

    classDef projectNode fill:#fce7f3,stroke:#db2777,stroke-width:2px,color:#9d174d
    classDef pageNode fill:#dbeafe,stroke:#2563eb,stroke-width:2px,color:#1e40af
    classDef outputNode fill:#e0f2fe,stroke:#1d4ed8,stroke-width:1px,color:#075985
    classDef metricsNode fill:#e0e7ff,stroke:#6366f1,stroke-width:1px,color:#4338ca
    classDef blockNode fill:#bfdbfe,stroke:#3b82f6,stroke-width:2px,color:#1d4ed8
    classDef typeNode fill:#c7d2fe,stroke:#6366f1,stroke-width:2px,color:#4338ca

    class PRJ projectNode
    class P_PRICING pageNode
    class PO_FR,PO_EN,BO_HERO_FR,BO_HERO_EN outputNode
    class PM_FR metricsNode
    class B_HERO,B_FAQ blockNode
    class BT_HERO,BT_FAQ typeNode
```

### Instructions des Blocks

```yaml
# blk_pricing_hero
instructions: |
  [GENERATE] title: catchy for pricing, use @tier-pro and @tier-free
  [GENERATE] description: reassuring, mention free trial
  [FIXED] button.url: /signup

# blk_pricing_faq
instructions: "[GENERATE] questions: 5-6 frequently asked questions about pricing. Use @tier-pro, @tier-free, @analytics"
```

---

## 3. Concept Graph avec Spreading Activation

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#22c55e', 'primaryTextColor': '#1e293b', 'primaryBorderColor': '#16a34a', 'lineColor': '#64748b' }}}%%
flowchart LR
    subgraph CONCEPTS["💡 Concepts"]
        C_CREATE["<b>create-qr-code</b><br/>Triggers: create, generate, make"]
        C_GEN["<b>qr-code-generator</b><br/>Triggers: generator, tool, maker"]
        C_ANALYTICS["<b>analytics</b><br/>Triggers: track, stats, scans"]
        C_PRO["<b>tier-pro</b><br/>Triggers: pro, premium, paid"]
        C_FREE["<b>tier-free</b><br/>Triggers: free, gratuit"]
        C_BRAND["<b>qrcode-ai</b><br/>Triggers: qrcode ai, brand"]
        C_SECURITY["<b>security</b><br/>Triggers: secure, safe"]
    end

    %% Semantic Links avec températures
    C_CREATE -->|"is_action_on<br/>temp: 0.95"| C_GEN
    C_PRO -->|"includes<br/>temp: 0.85"| C_ANALYTICS
    C_GEN -->|"type_of<br/>temp: 0.90"| C_BRAND

    classDef conceptNode fill:#dcfce7,stroke:#16a34a,stroke-width:2px,color:#166534
    classDef hotLink stroke:#ef4444,stroke-width:3px
    classDef warmLink stroke:#f97316,stroke-width:2px
    classDef coolLink stroke:#3b82f6,stroke-width:1px

    class C_CREATE,C_GEN,C_ANALYTICS,C_PRO,C_FREE,C_BRAND,C_SECURITY conceptNode
```

### Spreading Activation - Exemple

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#22c55e', 'lineColor': '#64748b' }}}%%
flowchart LR
    subgraph ACTIVATION["🔥 Spreading depuis tier-pro"]
        START["<b>tier-pro</b><br/>activation: 1.0<br/>🔴 HOT"]
        HOP1["<b>analytics</b><br/>activation: 0.85<br/>🟠 WARM"]
        HOP2["<b>create-qr-code</b><br/>activation: 0.72<br/>🟡 MEDIUM"]
        CUTOFF["<b>autres concepts</b><br/>activation: < 0.3<br/>⚪ CUTOFF"]
    end

    START -->|"includes × 0.85"| HOP1
    HOP1 -->|"related × 0.85"| HOP2
    HOP2 -.->|"< cutoff 0.3"| CUTOFF

    classDef hot fill:#fecaca,stroke:#dc2626,stroke-width:3px,color:#991b1b
    classDef warm fill:#fed7aa,stroke:#ea580c,stroke-width:2px,color:#9a3412
    classDef medium fill:#fef08a,stroke:#ca8a04,stroke-width:2px,color:#854d0e
    classDef cutoff fill:#f1f5f9,stroke:#94a3b8,stroke-width:1px,color:#64748b,stroke-dasharray: 5 5

    class START hot
    class HOP1 warm
    class HOP2 medium
    class CUTOFF cutoff
```

### Configuration Spreading

```yaml
spreading_config:
  cutoff: 0.3        # Stop si activation < 0.3
  max_depth: 2       # Maximum 2 sauts
  decay: 0.9         # Multiplicateur optionnel par saut
```

---

## 4. ConceptL10n - Données localisées (v6.8)

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#22c55e', 'lineColor': '#64748b' }}}%%
flowchart TB
    subgraph CONCEPT_CREATE["💡 Concept: create-qr-code"]
        C["<b>Concept</b><br/>key: create-qr-code<br/>llm_context: Represents action of creating QR..."]

        subgraph L10N_FR["🇫🇷 ConceptL10n fr-FR"]
            CL_FR["<b>ConceptL10n</b><br/>locale: fr-FR<br/>title: Créer un QR Code<br/>definition: L'action de générer un QR code...<br/>purpose: Convertir une information...<br/>benefits: [Instantané, Sans compétence...]<br/>use_cases: [QR site web, vCard, WiFi...]<br/>cultural_notes: 'QR Code' > 'Code QR'"]
        end

        subgraph L10N_EN["🇺🇸 ConceptL10n en-US"]
            CL_EN["<b>ConceptL10n</b><br/>locale: en-US<br/>title: Create a QR Code<br/>definition: The action of generating...<br/>purpose: Convert information...<br/>benefits: [Instant, No technical skills...]<br/>use_cases: [Website QR, vCard QR...]<br/>cultural_notes: null"]
        end
    end

    C -->|:HAS_L10N| CL_FR
    C -->|:HAS_L10N| CL_EN

    classDef conceptNode fill:#dcfce7,stroke:#16a34a,stroke-width:2px,color:#166534
    classDef frL10n fill:#dbeafe,stroke:#2563eb,stroke-width:1px,color:#1e40af
    classDef enL10n fill:#fef3c7,stroke:#d97706,stroke-width:1px,color:#92400e

    class C conceptNode
    class CL_FR frL10n
    class CL_EN enL10n
```

---

## 5. L10N System - Règles de localisation (v6.5 Simplified)

> **v6.5**: L10NType node REMOVED. L10NContent is now standalone with `category` field. BlockType.l10n_categories[] references categories directly.

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#f59e0b', 'lineColor': '#64748b' }}}%%
flowchart TB
    subgraph CONTENT_FR["🇫🇷 L10NContent fr-FR (standalone)"]
        LF1_FR["<b>identity</b><br/>category: identity<br/>locale: fr-FR<br/>stability: high<br/>timezone: Europe/Paris"]
        LF2_FR["<b>formatting</b><br/>category: formatting<br/>locale: fr-FR<br/>stability: high<br/>decimal: ,"]
        LF3_FR["<b>voice-style</b><br/>category: voice-style<br/>locale: fr-FR<br/>stability: medium<br/>formality: 70"]
        LF4_FR["<b>voice-lexicon</b><br/>category: voice-lexicon<br/>locale: fr-FR<br/>stability: medium"]
        LF5_FR["<b>culture-norms</b><br/>category: culture-norms<br/>locale: fr-FR<br/>stability: medium"]
    end

    subgraph CONTENT_EN["🇺🇸 L10NContent en-US (standalone)"]
        LF1_EN["<b>identity</b><br/>category: identity<br/>locale: en-US<br/>stability: high"]
        LF2_EN["<b>formatting</b><br/>category: formatting<br/>locale: en-US<br/>stability: high"]
        LF3_EN["<b>voice-style</b><br/>category: voice-style<br/>locale: en-US<br/>stability: medium"]
    end

    classDef frContent fill:#dbeafe,stroke:#2563eb,stroke-width:1px,color:#1e40af
    classDef enContent fill:#dcfce7,stroke:#16a34a,stroke-width:1px,color:#166534

    class LF1_FR,LF2_FR,LF3_FR,LF4_FR,LF5_FR frContent
    class LF1_EN,LF2_EN,LF3_EN enContent
```

### L10N Categories (v6.5)

| Category | Description | Stability |
|----------|-------------|-----------|
| `identity` | Codes locale, script, timezone, fallback | High |
| `formatting` | Dates, nombres, monnaie, téléphones | High |
| `slug` | Règles de génération slugs | High |
| `voice-style` | Formalité, ton, vouvoiement/tutoiement | Medium |
| `voice-lexicon` | Vocabulaire préféré/évité | Medium |
| `culture-norms` | Normes culturelles, tabous | Medium |
| `culture-references` | Célébrités, événements locaux | Low |
| `market` | Contexte marché, concurrents | Low |

### BlockType → L10N (v6.5 - Property-based)

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#6366f1', 'lineColor': '#64748b' }}}%%
flowchart LR
    subgraph BLOCK_TYPES["BlockTypes with l10n_categories[]"]
        BT_SEO["<b>header_seo</b><br/>l10n_categories:<br/>[identity, slug, voice-style]"]
        BT_HERO["<b>hero_main</b><br/>l10n_categories:<br/>[voice-style, voice-lexicon, formatting]"]
        BT_FAQ["<b>faq</b><br/>l10n_categories:<br/>[voice-style, voice-lexicon, culture-norms]"]
        BT_STATS["<b>content_stats</b><br/>l10n_categories:<br/>[formatting, voice-style]"]
    end

    subgraph L10N["L10NContent (queried by category)"]
        LC_ID["identity"]
        LC_SLUG["slug"]
        LC_FMT["formatting"]
        LC_VOICE["voice-style"]
        LC_LEX["voice-lexicon"]
        LC_CULT["culture-norms"]
    end

    BT_SEO -.->|"IN categories"| LC_ID
    BT_SEO -.->|"IN categories"| LC_SLUG
    BT_SEO -.->|"IN categories"| LC_VOICE

    BT_HERO -.->|"IN categories"| LC_VOICE
    BT_HERO -.->|"IN categories"| LC_LEX
    BT_HERO -.->|"IN categories"| LC_FMT

    BT_FAQ -.->|"IN categories"| LC_VOICE
    BT_FAQ -.->|"IN categories"| LC_LEX
    BT_FAQ -.->|"IN categories"| LC_CULT

    BT_STATS -.->|"IN categories"| LC_FMT
    BT_STATS -.->|"IN categories"| LC_VOICE

    classDef typeNode fill:#c7d2fe,stroke:#6366f1,stroke-width:2px,color:#4338ca
    classDef l10nNode fill:#fef3c7,stroke:#d97706,stroke-width:1px,color:#92400e

    class BT_SEO,BT_HERO,BT_FAQ,BT_STATS typeNode
    class LC_ID,LC_SLUG,LC_FMT,LC_VOICE,LC_LEX,LC_CULT l10nNode
```

### L10N Query Pattern (v6.5)

```cypher
// Get L10N rules for a BlockType - NO graph traversal!
MATCH (bt:BlockType {key: $blockTypeKey})
MATCH (lc:L10NContent {locale: $locale})
WHERE lc.category IN bt.l10n_categories
RETURN lc.category, lc.content
```

---

## 6. SEO Keywords - SEOKeywordL10n (v6.6 :TARGETS)

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#8b5cf6', 'lineColor': '#64748b' }}}%%
flowchart TB
    subgraph CONCEPT_KEYWORDS["💡 Concept: create-qr-code → SEOKeywordL10ns (v6.6 :TARGETS)"]
        C_CREATE["<b>Concept</b><br/>create-qr-code"]

        subgraph KEYWORDS_FR["🇫🇷 SEOKeywordL10n fr-FR"]
            SEO1["<b>qr code gratuit</b><br/>kind: term<br/>intent: transactional<br/>volume: 18,000<br/>difficulty: 45"]
            SEO2["<b>comment créer un qr code</b><br/>kind: question<br/>intent: informational<br/>volume: 2,400<br/>difficulty: 32"]
        end

        subgraph KEYWORDS_EN["🇺🇸 SEOKeywordL10n en-US"]
            SEO6["<b>qr code generator</b><br/>kind: term<br/>intent: transactional<br/>volume: 823,000<br/>difficulty: 78"]
            SEO7["<b>free qr code generator</b><br/>kind: term<br/>intent: transactional<br/>volume: 135,000<br/>difficulty: 65"]
            SEO3["<b>how to create a qr code</b><br/>kind: question<br/>intent: informational<br/>volume: 33,100<br/>difficulty: 38"]
        end
    end

    subgraph SNAPSHOTS["📊 SEOKeywordL10nSnapshot (historique)"]
        SEOS1["Dec 2024<br/>volume: 750,000"]
        SEOS2["Jan 1 2025<br/>volume: 800,000"]
        SEOS3["Jan 15 2025<br/>volume: 823,000"]
    end

    C_CREATE -->|:TARGETS| SEO1
    C_CREATE -->|:TARGETS| SEO2
    C_CREATE -->|:TARGETS| SEO6
    C_CREATE -->|:TARGETS| SEO7
    C_CREATE -->|:TARGETS| SEO3

    SEO6 -->|:HAS_SNAPSHOT| SEOS1
    SEO6 -->|:HAS_SNAPSHOT| SEOS2
    SEO6 -->|:HAS_SNAPSHOT| SEOS3

    classDef conceptNode fill:#dcfce7,stroke:#16a34a,stroke-width:2px,color:#166534
    classDef keywordNode fill:#ede9fe,stroke:#7c3aed,stroke-width:1px,color:#5b21b6
    classDef snapshotNode fill:#f1f5f9,stroke:#64748b,stroke-width:1px,color:#475569

    class C_CREATE conceptNode
    class SEO1,SEO2,SEO3,SEO6,SEO7 keywordNode
    class SEOS1,SEOS2,SEOS3 snapshotNode
```

---

## 7. GEO Seeds - GEOSeedL10n & Mining (v6.6)

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#ec4899', 'lineColor': '#64748b' }}}%%
flowchart TB
    subgraph GEO_SEEDS["🤖 GEOSeedL10n - GEO (Generative Engine Optimization)"]
        GEO1["<b>geo_1</b><br/>c'est quoi un QR code custom ?<br/>format: question<br/>intent: informational<br/>strategy: Define + mention AI design"]
        GEO2["<b>geo_2</b><br/>what is the best free QR code generator?<br/>format: question<br/>intent: decision<br/>strategy: List top options, position QR Code AI first"]
        GEO4["<b>geo_4</b><br/>quel générateur de QR code pour mon restaurant ?<br/>format: recommendation<br/>intent: decision<br/>strategy: Recommend QR Code AI for dynamic menus"]
    end

    subgraph REFORMULATIONS["🔄 GEOReformulation (mined)"]
        GEORF1["<b>search_query</b><br/>custom QR code generator with logo<br/>platform: chatgpt<br/>frequency: 4<br/>content_gap: false"]
        GEORF2["<b>sub_question</b><br/>difference static vs dynamic QR code<br/>platform: perplexity<br/>frequency: 3<br/>content_gap: true ⚠️"]
        GEORF3["<b>related_topic</b><br/>QR code security best practices<br/>platform: claude<br/>frequency: 2<br/>content_gap: true ⚠️"]
    end

    subgraph CITATIONS["📍 GEOCitation (tracking)"]
        GEOCT1["<b>perplexity</b><br/>cited: ✅<br/>position: 2<br/>sentiment: positive"]
        GEOCT2["<b>chatgpt</b><br/>cited: ✅<br/>position: 3<br/>sentiment: neutral"]
        GEOCT3["<b>ai_overview</b><br/>cited: ❌<br/>competitors: wikipedia, qr-code-generator.com"]
    end

    GEO1 -->|:HAS_REFORMULATION| GEORF1
    GEO1 -->|:HAS_REFORMULATION| GEORF2
    GEO2 -->|:HAS_REFORMULATION| GEORF3

    GEO1 -->|:HAS_CITATION| GEOCT1
    GEO2 -->|:HAS_CITATION| GEOCT2
    GEO2 -->|:HAS_CITATION| GEOCT3

    classDef seedNode fill:#fce7f3,stroke:#db2777,stroke-width:2px,color:#9d174d
    classDef reformNode fill:#fdf4ff,stroke:#c026d3,stroke-width:1px,color:#86198f
    classDef citedNode fill:#dcfce7,stroke:#16a34a,stroke-width:2px,color:#166534
    classDef notCitedNode fill:#fee2e2,stroke:#dc2626,stroke-width:2px,color:#991b1b
    classDef gapNode fill:#fef3c7,stroke:#d97706,stroke-width:2px,color:#92400e

    class GEO1,GEO2,GEO4 seedNode
    class GEORF1 reformNode
    class GEORF2,GEORF3 gapNode
    class GEOCT1,GEOCT2 citedNode
    class GEOCT3 notCitedNode
```

### Mining Run

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#ec4899', 'lineColor': '#64748b' }}}%%
flowchart LR
    subgraph MINING["⛏️ GEOMiningRun"]
        GEOMR1["<b>geomr_1</b><br/>seeds: [geo_1]<br/>platforms: chatgpt, perplexity, claude<br/>locale: fr-FR<br/>total: 15 reformulations<br/>unique: 8<br/>status: ✅ completed"]
        GEOMR2["<b>geomr_2</b><br/>seeds: [geo_2, geo_3]<br/>platforms: chatgpt, claude, gemini<br/>locale: en-US<br/>total: 22 reformulations<br/>unique: 12<br/>status: ✅ completed"]
    end

    classDef miningNode fill:#fdf4ff,stroke:#c026d3,stroke-width:2px,color:#86198f

    class GEOMR1,GEOMR2 miningNode
```

---

## 8. Flux Orchestrateur - Génération de contenu

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#3b82f6', 'lineColor': '#64748b' }}}%%
sequenceDiagram
    participant O as Orchestrateur
    participant N as Neo4j
    participant SA as Sub-Agent
    participant LLM as LLM

    Note over O,LLM: Phase 1: Compilation du contexte

    O->>N: GET Page(pricing) + Blocks
    N-->>O: Page + [hero-pricing, faq-pricing]

    O->>N: GET BlockType(hero_main).rules + l10n_categories
    N-->>O: Rules SEO/UX + [voice-style, voice-lexicon, formatting]

    O->>N: GET L10NContent WHERE category IN l10n_categories (v6.5)
    N-->>O: voice-style, voice-lexicon, formatting content

    O->>N: GET Block.USES_CONCEPT → Concept → ConceptL10n(fr-FR)
    N-->>O: tier-pro, tier-free definitions

    Note over O,LLM: Phase 2: Dispatch par block

    O->>SA: Contexte hero-pricing + instructions

    Note over SA,LLM: Phase 3: Sub-Agent enrichit

    SA->>N: Spreading(tier-pro, depth=2, cutoff=0.3)
    N-->>SA: analytics (0.85), bulk-create (0.72)

    SA->>N: GET Concept → SEOKeywordL10n (v6.6 :TARGETS)
    N-->>SA: "qr code gratuit" (18k), "tarifs qr code"...

    SA->>N: GET Concept → GEOSeedL10n (v6.6 :TARGETS)
    N-->>SA: "quel plan choisir?"...

    SA->>LLM: Prompt complet + contexte enrichi
    LLM-->>SA: BlockL10n JSON

    SA->>N: CREATE BlockL10n(fr-FR)
    SA-->>O: ✅ hero-pricing généré

    Note over O,LLM: Répéter pour chaque block
```

### Contexte chargé par niveau

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#3b82f6', 'lineColor': '#64748b' }}}%%
flowchart TB
    subgraph ORCHESTRATOR["📋 Orchestrateur compile"]
        O1["ADN projet"]
        O2["Page + Blocks (awareness)"]
        O3["Page.instructions (v6.8)"]
        O4["Page → Concepts via :USES_CONCEPT (v6.8)"]
        O5["Block → Concepts via :USES_CONCEPT"]
        O6["ConceptL10n (locale)"]
        O7["L10N via BlockType.l10n_categories[] (v6.5)"]
        O8["Rules SEO/UX"]
    end

    subgraph SUBAGENT["🤖 Sub-Agent charge"]
        S1["Spreading (concepts liés)"]
        S2["Concept → SEOKeywordL10n (direct)"]
        S3["Concept → GEOSeedL10n (direct)"]
    end

    subgraph GENERATION["✨ Génération"]
        G["BlockL10n JSON"]
    end

    O1 --> SUBAGENT
    O2 --> SUBAGENT
    O3 --> SUBAGENT
    O4 --> SUBAGENT
    O5 --> SUBAGENT
    O6 --> SUBAGENT
    O7 --> SUBAGENT
    O8 --> SUBAGENT

    S1 --> G
    S2 --> G
    S3 --> G

    classDef orchNode fill:#dbeafe,stroke:#2563eb,stroke-width:1px,color:#1e40af
    classDef subNode fill:#dcfce7,stroke:#16a34a,stroke-width:1px,color:#166534
    classDef genNode fill:#fef3c7,stroke:#d97706,stroke-width:2px,color:#92400e

    class O1,O2,O3,O4,O5,O6,O7,O8 orchNode
    class S1,S2,S3 subNode
    class G genNode
```

---

## 9. Block → Concept via :USES_CONCEPT

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#3b82f6', 'lineColor': '#64748b' }}}%%
flowchart LR
    subgraph BLOCK["📦 Block: hero-pricing"]
        B["instructions:<br/>[GENERATE] title: use @tier-pro and @tier-free<br/>[GENERATE] description: mention free trial"]
    end

    subgraph CONCEPTS["💡 Concepts"]
        C_PRO["<b>tier-pro</b><br/>Pro subscription tier"]
        C_FREE["<b>tier-free</b><br/>Free tier"]
    end

    subgraph PARSED["🔍 Relations extraites"]
        R1[":USES_CONCEPT<br/>purpose: primary<br/>temp: 1.0"]
        R2[":USES_CONCEPT<br/>purpose: primary<br/>temp: 1.0"]
    end

    B -->|"@tier-pro"| R1
    B -->|"@tier-free"| R2
    R1 --> C_PRO
    R2 --> C_FREE

    classDef blockNode fill:#bfdbfe,stroke:#3b82f6,stroke-width:2px,color:#1d4ed8
    classDef conceptNode fill:#dcfce7,stroke:#16a34a,stroke-width:2px,color:#166534
    classDef relNode fill:#f1f5f9,stroke:#64748b,stroke-width:1px,color:#475569

    class B blockNode
    class C_PRO,C_FREE conceptNode
    class R1,R2 relNode
```

### FAQ Block avec plusieurs concepts

```mermaid
%%{init: {'theme': 'base', 'themeVariables': { 'primaryColor': '#3b82f6', 'lineColor': '#64748b' }}}%%
flowchart LR
    subgraph BLOCK["📦 Block: faq-pricing"]
        B["instructions:<br/>[GENERATE] questions: 5-6 FAQs<br/>Use @tier-pro, @tier-free, @analytics"]
    end

    subgraph USES["🔗 :USES_CONCEPT"]
        U1["purpose: primary<br/>temp: 1.0"]
        U2["purpose: primary<br/>temp: 1.0"]
        U3["purpose: secondary<br/>temp: 0.8"]
    end

    subgraph CONCEPTS["💡 Concepts"]
        C_PRO["tier-pro"]
        C_FREE["tier-free"]
        C_ANALYTICS["analytics"]
    end

    B --> U1 --> C_PRO
    B --> U2 --> C_FREE
    B --> U3 --> C_ANALYTICS

    classDef blockNode fill:#bfdbfe,stroke:#3b82f6,stroke-width:2px,color:#1d4ed8
    classDef conceptNode fill:#dcfce7,stroke:#16a34a,stroke-width:2px,color:#166534
    classDef primaryRel fill:#dcfce7,stroke:#16a34a,stroke-width:2px,color:#166534
    classDef secondaryRel fill:#fef3c7,stroke:#d97706,stroke-width:1px,color:#92400e

    class B blockNode
    class C_PRO,C_FREE,C_ANALYTICS conceptNode
    class U1,U2 primaryRel
    class U3 secondaryRel
```

---

## 10. Résumé des relations (v6.8)

| Relation | From | To | Props | Description |
|----------|------|----|-------|-------------|
| `:HAS_PAGE` | Project | Page | - | **v6.7**: Project owns pages |
| `:HAS_CONCEPT` | Project | Concept | - | **v6.7**: Project owns concepts |
| `:HAS_L10N` | Project | L10NContent | - | **v6.7**: Project owns L10N rules |
| `:HAS_L10N` | Concept | ConceptL10n | - | **v6.8**: Concept localized data (replaces :HAS_CONTENT) |
| `:HAS_OUTPUT` | Page | PageL10n | - | **v6.8**: Page generated output (replaces :HAS_CONTENT) |
| `:HAS_OUTPUT` | Block | BlockL10n | - | **v6.8**: Block generated output (replaces :HAS_CONTENT) |
| `:HAS_METRICS` | Page | PageMetrics | - | **v6.8**: Page performance metrics |
| ~~`:HAS_CONTENT`~~ | ~~Page/Block/Concept~~ | ~~*Content~~ | - | **v6.8 REMOVED** → use `:HAS_OUTPUT` or `:HAS_L10N` |
| ~~`:ASSEMBLES`~~ | ~~PageL10n~~ | ~~BlockL10n~~ | ~~position~~ | **v6.7 REMOVED** → use `BlockL10n.position` |
| `:HAS_BLOCK` | Page | Block | - | Structure de page (position sur Block) |
| `:OF_TYPE` | Block | BlockType | - | Type du block |
| `:USES_CONCEPT` | Page | Concept | purpose, temperature | **v6.8**: Page-level concept references |
| `:USES_CONCEPT` | Block | Concept | purpose, temperature | Concepts référencés |
| ~~`:REQUIRES_L10N`~~ | ~~BlockType~~ | ~~L10NType~~ | - | **v6.5 REMOVED** → use `BlockType.l10n_categories[]` |
| `:SEMANTIC_LINK` | Concept | Concept | type, temperature | Spreading activation (v6.5: 10 types) |
| `:TARGETS` | Concept | SEOKeywordL10n/GEOSeedL10n | type | v6.6: renamed from :HAS_QUERY |
| `:HAS_SNAPSHOT` | SEOKeywordL10n | SEOKeywordL10nSnapshot | - | Historique SEO |
| `:HAS_REFORMULATION` | GEOSeedL10n | GEOReformulation | - | Mining LLM |
| `:HAS_CITATION` | GEOSeedL10n | GEOCitation | - | Tracking citations |

### SEMANTIC_LINK Types (v6.5 - Simplified)

| Type | Inverse | Temperature | Description |
|------|---------|-------------|-------------|
| `is_action_on` | `has_action` | 0.95 | Verb-noun (create → qr-code) |
| `has_action` | `is_action_on` | 0.90 | Noun-verb inverse |
| `includes` | `included_in` | 0.85 | Container (tier-pro → analytics) |
| `included_in` | `includes` | 0.80 | Part-whole inverse |
| `type_of` | `has_type` | 0.90 | Taxonomy (vcard → qr-code-type) |
| `has_type` | `type_of` | 0.85 | Is-a inverse |
| `requires` | `required_by` | 0.80 | Dependency |
| `required_by` | `requires` | 0.75 | Dependency inverse |
| `related` | symmetric | 0.60 | Generic association |
| `opposite` | symmetric | 0.40 | Contrast |

---

## Queries Cypher utiles

### Charger contexte complet d'un Block (v6.8)

```cypher
// v6.8: L10N via BlockType.l10n_categories[] property (no graph traversal)
// ConceptL10n replaces ConceptContent, :HAS_L10N replaces :HAS_CONTENT
MATCH (b:Block {key: "hero-pricing"})
MATCH (b)-[:USES_CONCEPT]->(c:Concept)-[:HAS_L10N]->(cl:ConceptL10n {locale: "fr-FR"})
MATCH (b)-[:OF_TYPE]->(bt:BlockType)
MATCH (lc:L10NContent {locale: "fr-FR"})
WHERE lc.category IN bt.l10n_categories
RETURN b.instructions,
       collect(DISTINCT {concept: c.key, title: cl.title, definition: cl.definition}) AS concepts,
       collect(DISTINCT {category: lc.category, content: lc.content}) AS l10n,
       bt.rules
```

### Page complète par locale (v6.8: via BlockL10n.position)

```cypher
// v6.8: PageL10n/BlockL10n replace PageContent/BlockContent
// :HAS_OUTPUT replaces :HAS_CONTENT for Page/Block
MATCH (p:Page {key: "pricing"})-[:HAS_OUTPUT]->(po:PageL10n {locale: "fr-FR"})
MATCH (p)-[:HAS_BLOCK]->(b:Block)-[:HAS_OUTPUT]->(bo:BlockL10n {locale: "fr-FR"})
RETURN p.instructions, po.slug, po.meta_title,
       collect({position: bo.position, content: bo}) AS blocks
ORDER BY bo.position
```

### Spreading Activation

```cypher
MATCH (c:Concept {key: "tier-pro"})-[r:SEMANTIC_LINK*1..2]->(c2:Concept)
WHERE ALL(rel IN r WHERE rel.temperature >= 0.3)
WITH c2, reduce(activation = 1.0, rel IN r | activation * rel.temperature) AS activation
WHERE activation >= 0.3
RETURN c2.key, activation
ORDER BY activation DESC
```

### Content Gaps (GEO)

```cypher
MATCH (georf:GEOReformulation {content_gap: true})
RETURN georf.value, georf.intent, georf.frequency
ORDER BY georf.frequency DESC
```
