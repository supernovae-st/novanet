//! Internationalization (i18n) for Nexus content.
//!
//! Provides bilingual content (English/French) for all Nexus tabs:
//! - Tutorial steps and tasks
//! - Glossary concepts
//! - Quiz questions
//! - Intro pages
//! - UI labels and tips
//!
//! Toggle language with Shift+I.

use super::NexusLocale;

// =============================================================================
// UI LABELS
// =============================================================================

/// Get section name for locale.
pub fn section_learn(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "LEARN",
        NexusLocale::Fr => "APPRENDRE",
    }
}

pub fn section_explore(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "EXPLORE",
        NexusLocale::Fr => "EXPLORER",
    }
}

pub fn section_practice(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "PRACTICE",
        NexusLocale::Fr => "PRATIQUER",
    }
}

/// Get tab labels for locale.
pub fn tab_intro(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "Intro",
        NexusLocale::Fr => "Intro",
    }
}

pub fn tab_glossary(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "Glossary",
        NexusLocale::Fr => "Glossaire",
    }
}

pub fn tab_tutorial(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "Tutorial",
        NexusLocale::Fr => "Tutoriel",
    }
}

pub fn tab_traits(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "Traits",
        NexusLocale::Fr => "Traits",
    }
}

pub fn tab_layers(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "Layers",
        NexusLocale::Fr => "Couches",
    }
}

pub fn tab_arcs(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "Arcs",
        NexusLocale::Fr => "Arcs",
    }
}

pub fn tab_pipeline(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "Pipeline",
        NexusLocale::Fr => "Pipeline",
    }
}

pub fn tab_quiz(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "Quiz",
        NexusLocale::Fr => "Quiz",
    }
}

pub fn tab_views(locale: NexusLocale) -> &'static str {
    match locale {
        NexusLocale::En => "Views",
        NexusLocale::Fr => "Vues",
    }
}

// =============================================================================
// TIPS
// =============================================================================

/// Educational tips shown at the bottom of Nexus mode.
pub fn tips(locale: NexusLocale) -> &'static [&'static str] {
    match locale {
        NexusLocale::En => &[
            "Imported is INPUT (savoir) - Authored is OUTPUT (generated)",
            "Layers define WHAT a node does, Traits define HOW it behaves with locale",
            "Content/Generated nodes have defined parents (Entity→EntityContent, Page→PageGenerated)",
            "Generation, NOT translation: Imported + Structure -> Native content",
            "Shared realm is READ-ONLY - all business content lives in Org",
            "Quick jump: gd=defined, ga=authored, gi=imported, gg=generated, gr=retrieved",
            "Imported nodes exist ONLY where needed (fr-FR: 20K Terms, sw-KE: 500)",
            "Arc families: ownership, localization, semantic, generation, mining",
            "Defined = structure (solid border), Authored = output (dashed border)",
            "Press 'n' to see the next tip!",
        ],
        NexusLocale::Fr => &[
            "Imported = INPUT (savoir) - Authored = OUTPUT (généré)",
            "Les Layers définissent CE QUE fait un nœud, les Traits définissent COMMENT il se comporte avec la locale",
            "Les nœuds Content/Generated ont des parents définis (Entity→EntityContent, Page→PageGenerated)",
            "Génération, PAS traduction : Imported + Structure -> Contenu natif",
            "Le realm Shared est en LECTURE SEULE - tout le contenu métier vit dans Org",
            "Saut rapide : gd=defined, ga=authored, gi=imported, gg=generated, gr=retrieved",
            "Les nœuds Imported n'existent QUE là où ils sont nécessaires (fr-FR: 20K Terms, sw-KE: 500)",
            "Familles d'arcs : ownership, localization, semantic, generation, mining",
            "Defined = structure (bordure pleine), Authored = sortie (bordure pointillée)",
            "Appuyez sur 'n' pour voir le prochain conseil !",
        ],
    }
}

// =============================================================================
// INTRO
// =============================================================================

/// Intro page titles.
pub fn intro_titles(locale: NexusLocale) -> &'static [&'static str] {
    match locale {
        NexusLocale::En => &[
            "What is NovaNet?",
            "The Generation Philosophy",
            "The Graph Structure",
        ],
        NexusLocale::Fr => &[
            "Qu'est-ce que NovaNet ?",
            "La Philosophie de Génération",
            "La Structure du Graphe",
        ],
    }
}

/// Intro page 1 content.
pub fn intro_page1(locale: NexusLocale) -> &'static [&'static str] {
    match locale {
        NexusLocale::En => &[
            "NovaNet is a knowledge graph that orchestrates NATIVE CONTENT",
            "GENERATION across 200+ locales.",
            "",
            "Problem: Traditional translation loses cultural nuance.",
            "         200 locales = 200× translation cost.",
            "",
            "Solution: Generate content NATIVELY per locale.",
            "          Entity definitions written once;",
            "          content generated 200× with cultural context.",
            "",
            "Result: Native-quality content at a fraction of the cost.",
        ],
        NexusLocale::Fr => &[
            "NovaNet est un graphe de connaissances qui orchestre la",
            "GÉNÉRATION DE CONTENU NATIF dans plus de 200 locales.",
            "",
            "Problème : La traduction traditionnelle perd les nuances culturelles.",
            "           200 locales = 200× le coût de traduction.",
            "",
            "Solution : Générer le contenu NATIVEMENT par locale.",
            "           Les définitions d'entités sont écrites une fois ;",
            "           le contenu est généré 200× avec le contexte culturel.",
            "",
            "Résultat : Contenu de qualité native à une fraction du coût.",
        ],
    }
}

/// Intro page 2 content.
pub fn intro_page2(locale: NexusLocale) -> &'static [&'static str] {
    match locale {
        NexusLocale::En => &[
            "GENERATION, NOT TRANSLATION",
            "",
            "❌ WRONG:  Source → Translate → Target",
            "✓  RIGHT:  Entity (defined) → Generate → Content (locale-native)",
            "",
            "The LLM doesn't translate - it generates natively",
            "using Imported atoms (Terms, Expressions, Patterns)",
            "that exist ONLY in the target locale.",
            "",
            "Example: fr-FR may have 20,000 Terms",
            "         sw-KE may have 500 Terms",
            "         Each locale has exactly what it needs.",
        ],
        NexusLocale::Fr => &[
            "GÉNÉRATION, PAS TRADUCTION",
            "",
            "❌ FAUX :   Source → Traduire → Cible",
            "✓  JUSTE :  Entity (defined) → Générer → Content (natif locale)",
            "",
            "Le LLM ne traduit pas - il génère nativement",
            "en utilisant les atomes Imported (Terms, Expressions, Patterns)",
            "qui n'existent QUE dans la locale cible.",
            "",
            "Exemple : fr-FR peut avoir 20 000 Terms",
            "          sw-KE peut avoir 500 Terms",
            "          Chaque locale a exactement ce dont elle a besoin.",
        ],
    }
}

/// Intro page 3 content.
pub fn intro_page3(locale: NexusLocale) -> &'static [&'static str] {
    match locale {
        NexusLocale::En => &[
            "THE GRAPH STRUCTURE (v0.12.4)",
            "",
            "61 node types organized by:",
            "  • Realm (WHERE): shared (40) | org (21)",
            "  • Layer (WHAT): 10 functional layers",
            "  • Trait (HOW): defined | authored | imported | generated | retrieved",
            "",
            "114 arc types organized by:",
            "  • Family: ownership | localization | semantic | generation | mining",
            "  • Scope: intra_realm | cross_realm",
            "",
            "Navigate with [1]LEARN [2]EXPLORE [3]PRACTICE",
        ],
        NexusLocale::Fr => &[
            "LA STRUCTURE DU GRAPHE (v0.12.4)",
            "",
            "61 types de nœuds organisés par :",
            "  • Realm (OÙ) : shared (40) | org (21)",
            "  • Layer (QUOI) : 10 couches fonctionnelles",
            "  • Trait (COMMENT) : defined | authored | imported | generated | retrieved",
            "",
            "114 types d'arcs organisés par :",
            "  • Family : ownership | localization | semantic | generation | mining",
            "  • Scope : intra_realm | cross_realm",
            "",
            "Naviguez avec [1]APPRENDRE [2]EXPLORER [3]PRATIQUER",
        ],
    }
}

// =============================================================================
// TUTORIAL
// =============================================================================

/// Tutorial step structure for i18n.
pub struct TutorialStepI18n {
    pub title: &'static str,
    pub description: &'static str,
    pub tasks: &'static [&'static str],
}

/// Get tutorial steps for locale.
pub fn tutorial_steps(locale: NexusLocale) -> &'static [TutorialStepI18n] {
    match locale {
        NexusLocale::En => &TUTORIAL_STEPS_EN,
        NexusLocale::Fr => &TUTORIAL_STEPS_FR,
    }
}

static TUTORIAL_STEPS_EN: [TutorialStepI18n; 5] = [
    TutorialStepI18n {
        title: "Understanding Realms",
        description: "NovaNet has 2 realms: SHARED (universal knowledge, read-only) and ORG (organization-specific content).",
        tasks: &[
            "Press [2] to go to EXPLORE section",
            "Press 'l' for Layers tab",
            "Use h/l to switch between Shared and Org realms",
        ],
    },
    TutorialStepI18n {
        title: "Exploring Layers",
        description: "Each realm has layers. Shared has 4 layers (config, locale, geography, knowledge). Org has 6 layers.",
        tasks: &[
            "In Layers tab, use j/k to navigate layers",
            "Notice the node count per layer",
            "Press Enter to see layer details",
        ],
    },
    TutorialStepI18n {
        title: "Understanding Traits",
        description: "Traits define HOW a node behaves with locale: defined (structure), authored (output), imported (input), generated, retrieved.",
        tasks: &[
            "Press 't' for Traits tab",
            "Use j/k to navigate the 5 traits",
            "Read each trait's description",
        ],
    },
    TutorialStepI18n {
        title: "Arc Families",
        description: "Arcs connect nodes. 5 families: ownership (hierarchy), localization (locale links), semantic (meaning), generation (output), mining (SEO/GEO).",
        tasks: &[
            "Press 'a' for Arcs tab",
            "Navigate arc families with j/k",
            "See the relationships each family defines",
        ],
    },
    TutorialStepI18n {
        title: "Test Your Knowledge",
        description: "Time to test what you've learned! The Quiz has 15 questions about NovaNet's architecture.",
        tasks: &[
            "Press [3] to go to PRACTICE section",
            "Press 'q' for Quiz tab",
            "Complete the quiz with 80%+ score",
        ],
    },
];

static TUTORIAL_STEPS_FR: [TutorialStepI18n; 5] = [
    TutorialStepI18n {
        title: "Comprendre les Realms",
        description: "NovaNet a 2 realms : SHARED (connaissances universelles, lecture seule) et ORG (contenu spécifique à l'organisation).",
        tasks: &[
            "Appuyez sur [2] pour aller à la section EXPLORER",
            "Appuyez sur 'l' pour l'onglet Couches",
            "Utilisez h/l pour basculer entre les realms Shared et Org",
        ],
    },
    TutorialStepI18n {
        title: "Explorer les Couches",
        description: "Chaque realm a des couches. Shared a 4 couches (config, locale, geography, knowledge). Org en a 6.",
        tasks: &[
            "Dans l'onglet Couches, utilisez j/k pour naviguer",
            "Remarquez le nombre de nœuds par couche",
            "Appuyez sur Entrée pour voir les détails",
        ],
    },
    TutorialStepI18n {
        title: "Comprendre les Traits",
        description: "Les Traits définissent COMMENT un nœud se comporte avec la locale : defined (structure), authored (sortie), imported (entrée), generated, retrieved.",
        tasks: &[
            "Appuyez sur 't' pour l'onglet Traits",
            "Utilisez j/k pour naviguer les 5 traits",
            "Lisez la description de chaque trait",
        ],
    },
    TutorialStepI18n {
        title: "Familles d'Arcs",
        description: "Les arcs connectent les nœuds. 5 familles : ownership (hiérarchie), localization (liens locale), semantic (sens), generation (sortie), mining (SEO/GEO).",
        tasks: &[
            "Appuyez sur 'a' pour l'onglet Arcs",
            "Naviguez les familles d'arcs avec j/k",
            "Voyez les relations que chaque famille définit",
        ],
    },
    TutorialStepI18n {
        title: "Testez Vos Connaissances",
        description: "C'est le moment de tester ce que vous avez appris ! Le Quiz a 15 questions sur l'architecture NovaNet.",
        tasks: &[
            "Appuyez sur [3] pour aller à la section PRATIQUER",
            "Appuyez sur 'q' pour l'onglet Quiz",
            "Complétez le quiz avec 80%+ de score",
        ],
    },
];

// =============================================================================
// GLOSSARY
// =============================================================================

/// Glossary concept structure for i18n.
pub struct GlossaryConceptI18n {
    pub name: &'static str,
    pub short_desc: &'static str,
    pub full_desc: &'static str,
}

/// Glossary category structure for i18n.
pub struct GlossaryCategoryI18n {
    pub name: &'static str,
    pub concepts: &'static [GlossaryConceptI18n],
}

/// Get glossary categories for locale.
pub fn glossary_categories(locale: NexusLocale) -> &'static [GlossaryCategoryI18n] {
    match locale {
        NexusLocale::En => &GLOSSARY_EN,
        NexusLocale::Fr => &GLOSSARY_FR,
    }
}

static GLOSSARY_EN: [GlossaryCategoryI18n; 4] = [
    GlossaryCategoryI18n {
        name: "Realms",
        concepts: &[
            GlossaryConceptI18n {
                name: "Shared",
                short_desc: "Universal knowledge (40 nodes)",
                full_desc: "READ-ONLY realm containing universal locale knowledge. 4 layers: config, locale, geography, knowledge. Contains BCP-47 locales, knowledge atoms (Terms, Expressions), geographic data.",
            },
            GlossaryConceptI18n {
                name: "Org",
                short_desc: "Organization content (21 nodes)",
                full_desc: "Business-specific content realm. 6 layers: config, foundation, structure, semantic, instruction, output. Contains entities, pages, blocks, and generated content.",
            },
        ],
    },
    GlossaryCategoryI18n {
        name: "Layers",
        concepts: &[
            GlossaryConceptI18n {
                name: "Config",
                short_desc: "Configuration & definitions",
                full_desc: "Holds configuration and definition nodes. In shared: Locale, EntityCategory. In org: OrgConfig. These are foundational nodes that other layers reference.",
            },
            GlossaryConceptI18n {
                name: "Knowledge",
                short_desc: "Semantic knowledge atoms",
                full_desc: "Contains knowledge atoms that LLMs use for native generation: TermSet/Term, ExpressionSet/Expression, PatternSet/Pattern, CultureSet/CultureRef, TabooSet/Taboo, AudienceSet/AudienceTrait.",
            },
            GlossaryConceptI18n {
                name: "Semantic",
                short_desc: "Business meaning layer",
                full_desc: "Contains Entity and EntityContent - the core semantic units that represent business concepts. EntityContent stores locale-specific meaning for each Entity.",
            },
            GlossaryConceptI18n {
                name: "Output",
                short_desc: "Generated artifacts",
                full_desc: "Contains LLM-generated content: PageGenerated, BlockGenerated, OutputArtifact. These are the final outputs produced by the generation pipeline.",
            },
        ],
    },
    GlossaryCategoryI18n {
        name: "Traits",
        concepts: &[
            GlossaryConceptI18n {
                name: "Defined",
                short_desc: "Structure (solid border)",
                full_desc: "Nodes that don't change with locale. They define structure and relationships. Examples: Page, Block, Entity, Locale. Written once, used everywhere.",
            },
            GlossaryConceptI18n {
                name: "Authored",
                short_desc: "Output (dashed border)",
                full_desc: "Nodes that store locale-specific content. Examples: EntityContent, ProjectContent. One instance per locale, linked to defined parent.",
            },
            GlossaryConceptI18n {
                name: "Imported",
                short_desc: "Input atoms (double border)",
                full_desc: "Locale-specific knowledge for LLM generation. Terms, Expressions, Patterns exist ONLY where needed. fr-FR may have 20K Terms; sw-KE may have 500.",
            },
            GlossaryConceptI18n {
                name: "Generated",
                short_desc: "LLM output (dotted border)",
                full_desc: "Content produced by LLM generation. PageGenerated, BlockGenerated, OutputArtifact. Created from defined + imported inputs.",
            },
            GlossaryConceptI18n {
                name: "Retrieved",
                short_desc: "Computed metrics (thin dotted)",
                full_desc: "Nodes containing computed/retrieved data. GEOMetrics, SEOKeywordMetrics. Derived from mining operations, not LLM generation.",
            },
        ],
    },
    GlossaryCategoryI18n {
        name: "Architecture",
        concepts: &[
            GlossaryConceptI18n {
                name: "Native Generation",
                short_desc: "Generate, don't translate",
                full_desc: "NovaNet's core philosophy: content is GENERATED natively per locale, NOT translated. The LLM uses locale-specific knowledge atoms to produce culturally appropriate content.",
            },
            GlossaryConceptI18n {
                name: "Query-First",
                short_desc: "Cypher is source of truth",
                full_desc: "Architecture where the graph visualization is determined solely by the executed Cypher query. Views are parameterized Cypher templates defined in YAML.",
            },
        ],
    },
];

static GLOSSARY_FR: [GlossaryCategoryI18n; 4] = [
    GlossaryCategoryI18n {
        name: "Realms",
        concepts: &[
            GlossaryConceptI18n {
                name: "Shared",
                short_desc: "Connaissances universelles (39 nœuds)",
                full_desc: "Realm en LECTURE SEULE contenant les connaissances universelles de locale. 4 couches : config, locale, geography, knowledge. Contient les locales BCP-47, les atomes de connaissance (Terms, Expressions), les données géographiques.",
            },
            GlossaryConceptI18n {
                name: "Org",
                short_desc: "Contenu organisation (20 nœuds)",
                full_desc: "Realm de contenu spécifique à l'entreprise. 6 couches : config, foundation, structure, semantic, instruction, output. Contient les entités, pages, blocks et contenu généré.",
            },
        ],
    },
    GlossaryCategoryI18n {
        name: "Couches",
        concepts: &[
            GlossaryConceptI18n {
                name: "Config",
                short_desc: "Configuration & définitions",
                full_desc: "Contient les nœuds de configuration et définition. Dans shared : Locale, EntityCategory. Dans org : OrgConfig. Ce sont les nœuds fondamentaux que les autres couches référencent.",
            },
            GlossaryConceptI18n {
                name: "Knowledge",
                short_desc: "Atomes de connaissance sémantique",
                full_desc: "Contient les atomes de connaissance que les LLMs utilisent pour la génération native : TermSet/Term, ExpressionSet/Expression, PatternSet/Pattern, CultureSet/CultureRef, TabooSet/Taboo, AudienceSet/AudienceTrait.",
            },
            GlossaryConceptI18n {
                name: "Semantic",
                short_desc: "Couche de sens métier",
                full_desc: "Contient Entity et EntityContent - les unités sémantiques de base qui représentent les concepts métier. EntityContent stocke le sens spécifique à la locale pour chaque Entity.",
            },
            GlossaryConceptI18n {
                name: "Output",
                short_desc: "Artefacts générés",
                full_desc: "Contient le contenu généré par LLM : PageGenerated, BlockGenerated, OutputArtifact. Ce sont les sorties finales produites par le pipeline de génération.",
            },
        ],
    },
    GlossaryCategoryI18n {
        name: "Traits",
        concepts: &[
            GlossaryConceptI18n {
                name: "Defined",
                short_desc: "Structure (bordure pleine)",
                full_desc: "Nœuds qui ne changent pas avec la locale. Ils définissent la structure et les relations. Exemples : Page, Block, Entity, Locale. Écrits une fois, utilisés partout.",
            },
            GlossaryConceptI18n {
                name: "Authored",
                short_desc: "Sortie (bordure pointillée)",
                full_desc: "Nœuds qui stockent le contenu spécifique à la locale. Exemples : EntityContent, ProjectContent. Une instance par locale, liée au parent défini.",
            },
            GlossaryConceptI18n {
                name: "Imported",
                short_desc: "Atomes d'entrée (double bordure)",
                full_desc: "Connaissances spécifiques à la locale pour la génération LLM. Terms, Expressions, Patterns n'existent QUE là où c'est nécessaire. fr-FR peut avoir 20K Terms ; sw-KE peut avoir 500.",
            },
            GlossaryConceptI18n {
                name: "Generated",
                short_desc: "Sortie LLM (bordure pointillée)",
                full_desc: "Contenu produit par la génération LLM. PageGenerated, BlockGenerated, OutputArtifact. Créé à partir des entrées defined + imported.",
            },
            GlossaryConceptI18n {
                name: "Retrieved",
                short_desc: "Métriques calculées (pointillé fin)",
                full_desc: "Nœuds contenant des données calculées/récupérées. GEOMetrics, SEOKeywordMetrics. Dérivés des opérations de mining, pas de la génération LLM.",
            },
        ],
    },
    GlossaryCategoryI18n {
        name: "Architecture",
        concepts: &[
            GlossaryConceptI18n {
                name: "Génération Native",
                short_desc: "Générer, pas traduire",
                full_desc: "Philosophie centrale de NovaNet : le contenu est GÉNÉRÉ nativement par locale, PAS traduit. Le LLM utilise les atomes de connaissance spécifiques à la locale pour produire un contenu culturellement approprié.",
            },
            GlossaryConceptI18n {
                name: "Query-First",
                short_desc: "Cypher est la source de vérité",
                full_desc: "Architecture où la visualisation du graphe est déterminée uniquement par la requête Cypher exécutée. Les vues sont des templates Cypher paramétrés définis en YAML.",
            },
        ],
    },
];

// =============================================================================
// QUIZ
// =============================================================================

/// Quiz question structure for i18n.
pub struct QuizQuestionI18n {
    pub question: &'static str,
    pub options: &'static [&'static str],
    pub correct: usize,
    pub explanation: &'static str,
}

/// Get quiz questions for locale.
pub fn quiz_questions(locale: NexusLocale) -> &'static [QuizQuestionI18n] {
    match locale {
        NexusLocale::En => &QUIZ_EN,
        NexusLocale::Fr => &QUIZ_FR,
    }
}

static QUIZ_EN: [QuizQuestionI18n; 15] = [
    QuizQuestionI18n {
        question: "What are the two realms in NovaNet?",
        options: &[
            "Global and Local",
            "Shared and Org",
            "Public and Private",
            "Core and Edge",
        ],
        correct: 1,
        explanation: "Shared (universal, read-only) and Org (organization-specific).",
    },
    QuizQuestionI18n {
        question: "How many node types are in NovaNet v0.12.4?",
        options: &["45", "61", "75", "90"],
        correct: 1,
        explanation: "61 nodes: 40 in Shared realm + 21 in Org realm.",
    },
    QuizQuestionI18n {
        question: "What does the 'defined' trait mean?",
        options: &[
            "Changes per locale",
            "Structure that doesn't change",
            "Generated by LLM",
            "Computed metrics",
        ],
        correct: 1,
        explanation: "Defined nodes define structure and don't change with locale.",
    },
    QuizQuestionI18n {
        question: "Which trait do EntityContent and ProjectContent have?",
        options: &["Defined", "Authored", "Imported", "Generated"],
        correct: 1,
        explanation: "Authored nodes store locale-specific content for defined parents.",
    },
    QuizQuestionI18n {
        question: "What is NovaNet's core philosophy?",
        options: &[
            "Translation at scale",
            "Native generation per locale",
            "Machine translation",
            "Manual localization",
        ],
        correct: 1,
        explanation: "Generation, NOT translation: content is generated natively using locale knowledge.",
    },
    QuizQuestionI18n {
        question: "How many layers does the Shared realm have?",
        options: &["2", "4", "6", "8"],
        correct: 1,
        explanation: "Shared has 4 layers: config, locale, geography, knowledge.",
    },
    QuizQuestionI18n {
        question: "Which layer contains Terms, Expressions, and Patterns?",
        options: &["Config", "Semantic", "Knowledge", "Output"],
        correct: 2,
        explanation: "Knowledge layer contains knowledge atoms used by LLMs for generation.",
    },
    QuizQuestionI18n {
        question: "What are the 5 arc families?",
        options: &[
            "create, read, update, delete, query",
            "ownership, localization, semantic, generation, mining",
            "parent, child, sibling, cousin, ancestor",
            "input, output, transform, validate, store",
        ],
        correct: 1,
        explanation: "Arc families: ownership, localization, semantic, generation, mining.",
    },
    QuizQuestionI18n {
        question: "What does 'cross_realm' arc scope mean?",
        options: &[
            "Arc within same realm",
            "Arc between different realms",
            "Arc to external system",
            "Arc to database",
        ],
        correct: 1,
        explanation: "Cross_realm arcs connect nodes in different realms (e.g., Org→Shared).",
    },
    QuizQuestionI18n {
        question: "Which nodes have the 'generated' trait?",
        options: &[
            "Entity, Page, Block",
            "PageGenerated, BlockGenerated",
            "Term, Expression, Pattern",
            "Locale, Region, Country",
        ],
        correct: 1,
        explanation: "Generated trait marks LLM output: PageGenerated, BlockGenerated, OutputArtifact.",
    },
    QuizQuestionI18n {
        question: "What is the 'retrieved' trait for?",
        options: &[
            "Structure nodes",
            "LLM-generated content",
            "Computed metrics",
            "Locale content",
        ],
        correct: 2,
        explanation: "Retrieved nodes contain computed data: GEOMetrics, SEOKeywordMetrics.",
    },
    QuizQuestionI18n {
        question: "How many layers does the Org realm have?",
        options: &["4", "5", "6", "7"],
        correct: 2,
        explanation: "Org has 6 layers: config, foundation, structure, semantic, instruction, output.",
    },
    QuizQuestionI18n {
        question: "What is Query-First architecture?",
        options: &[
            "Database-first design",
            "Cypher is source of truth for display",
            "REST API pattern",
            "GraphQL approach",
        ],
        correct: 1,
        explanation: "Query-First: graph visualization is determined solely by the executed Cypher query.",
    },
    QuizQuestionI18n {
        question: "Where does EntityCategory live?",
        options: &[
            "org/semantic",
            "shared/knowledge",
            "shared/config",
            "org/config",
        ],
        correct: 2,
        explanation: "EntityCategory is in shared/config - it's a universal definition.",
    },
    QuizQuestionI18n {
        question: "Why might fr-FR have 20K Terms but sw-KE only 500?",
        options: &[
            "Translation budget",
            "Knowledge exists only where needed",
            "Different alphabet",
            "Population size",
        ],
        correct: 1,
        explanation: "Knowledge atoms exist ONLY where needed - each locale has exactly what it requires.",
    },
];

static QUIZ_FR: [QuizQuestionI18n; 15] = [
    QuizQuestionI18n {
        question: "Quels sont les deux realms dans NovaNet ?",
        options: &[
            "Global et Local",
            "Shared et Org",
            "Public et Privé",
            "Core et Edge",
        ],
        correct: 1,
        explanation: "Shared (universel, lecture seule) et Org (spécifique à l'organisation).",
    },
    QuizQuestionI18n {
        question: "Combien de types de nœuds dans NovaNet v0.12.4 ?",
        options: &["45", "61", "75", "90"],
        correct: 1,
        explanation: "61 nœuds : 40 dans Shared + 21 dans Org.",
    },
    QuizQuestionI18n {
        question: "Que signifie le trait 'defined' ?",
        options: &[
            "Change par locale",
            "Structure qui ne change pas",
            "Généré par LLM",
            "Métriques calculées",
        ],
        correct: 1,
        explanation: "Les nœuds defined définissent la structure et ne changent pas avec la locale.",
    },
    QuizQuestionI18n {
        question: "Quel trait ont EntityContent et ProjectContent ?",
        options: &["Defined", "Authored", "Imported", "Generated"],
        correct: 1,
        explanation: "Les nœuds Authored stockent le contenu spécifique à la locale pour les parents defined.",
    },
    QuizQuestionI18n {
        question: "Quelle est la philosophie centrale de NovaNet ?",
        options: &[
            "Traduction à grande échelle",
            "Génération native par locale",
            "Traduction automatique",
            "Localisation manuelle",
        ],
        correct: 1,
        explanation: "Génération, PAS traduction : le contenu est généré nativement en utilisant les connaissances de la locale.",
    },
    QuizQuestionI18n {
        question: "Combien de couches a le realm Shared ?",
        options: &["2", "4", "6", "8"],
        correct: 1,
        explanation: "Shared a 4 couches : config, locale, geography, knowledge.",
    },
    QuizQuestionI18n {
        question: "Quelle couche contient Terms, Expressions et Patterns ?",
        options: &["Config", "Semantic", "Knowledge", "Output"],
        correct: 2,
        explanation: "La couche Knowledge contient les atomes de connaissance utilisés par les LLMs.",
    },
    QuizQuestionI18n {
        question: "Quelles sont les 5 familles d'arcs ?",
        options: &[
            "create, read, update, delete, query",
            "ownership, localization, semantic, generation, mining",
            "parent, child, sibling, cousin, ancestor",
            "input, output, transform, validate, store",
        ],
        correct: 1,
        explanation: "Familles d'arcs : ownership, localization, semantic, generation, mining.",
    },
    QuizQuestionI18n {
        question: "Que signifie le scope 'cross_realm' ?",
        options: &[
            "Arc dans le même realm",
            "Arc entre différents realms",
            "Arc vers système externe",
            "Arc vers base de données",
        ],
        correct: 1,
        explanation: "Les arcs cross_realm connectent des nœuds de différents realms (ex: Org→Shared).",
    },
    QuizQuestionI18n {
        question: "Quels nœuds ont le trait 'generated' ?",
        options: &[
            "Entity, Page, Block",
            "PageGenerated, BlockGenerated",
            "Term, Expression, Pattern",
            "Locale, Region, Country",
        ],
        correct: 1,
        explanation: "Le trait generated marque les sorties LLM : PageGenerated, BlockGenerated, OutputArtifact.",
    },
    QuizQuestionI18n {
        question: "À quoi sert le trait 'retrieved' ?",
        options: &[
            "Nœuds de structure",
            "Contenu généré par LLM",
            "Métriques calculées",
            "Contenu de locale",
        ],
        correct: 2,
        explanation: "Les nœuds retrieved contiennent des données calculées : GEOMetrics, SEOKeywordMetrics.",
    },
    QuizQuestionI18n {
        question: "Combien de couches a le realm Org ?",
        options: &["4", "5", "6", "7"],
        correct: 2,
        explanation: "Org a 6 couches : config, foundation, structure, semantic, instruction, output.",
    },
    QuizQuestionI18n {
        question: "Qu'est-ce que l'architecture Query-First ?",
        options: &[
            "Conception database-first",
            "Cypher est la source de vérité",
            "Pattern REST API",
            "Approche GraphQL",
        ],
        correct: 1,
        explanation: "Query-First : la visualisation du graphe est déterminée uniquement par la requête Cypher.",
    },
    QuizQuestionI18n {
        question: "Où vit EntityCategory ?",
        options: &[
            "org/semantic",
            "shared/knowledge",
            "shared/config",
            "org/config",
        ],
        correct: 2,
        explanation: "EntityCategory est dans shared/config - c'est une définition universelle.",
    },
    QuizQuestionI18n {
        question: "Pourquoi fr-FR peut avoir 20K Terms mais sw-KE seulement 500 ?",
        options: &[
            "Budget de traduction",
            "Le knowledge n'existe que là où c'est nécessaire",
            "Alphabet différent",
            "Taille de la population",
        ],
        correct: 1,
        explanation: "Les atomes de knowledge n'existent QUE là où c'est nécessaire - chaque locale a exactement ce dont elle a besoin.",
    },
];

// =============================================================================
// STATS (v0.12.0)
// =============================================================================

/// Stats tab i18n strings.
pub struct StatsI18n {
    pub title: &'static str,
    pub avg_score: &'static str,
    pub streak: &'static str,
    pub best: &'static str,
    pub quizzes: &'static str,
    pub achievements: &'static str,
    pub score_history: &'static str,
    pub last: &'static str,
    pub scores: &'static str,
    pub category_mastery: &'static str,
    pub achievements_title: &'static str,
    pub achievement_hint: &'static str,
    category_labels: &'static [&'static str],
}

impl StatsI18n {
    /// Create StatsI18n for given locale.
    pub fn new(locale: super::NexusLocale) -> Self {
        match locale {
            super::NexusLocale::En => Self {
                title: "Learning Stats",
                avg_score: "Avg:",
                streak: "Streak:",
                best: "best",
                quizzes: "Quizzes:",
                achievements: "Achievements:",
                score_history: "Score History",
                last: "Last",
                scores: "scores",
                category_mastery: "Category Mastery",
                achievements_title: "Achievements",
                achievement_hint: "Complete quizzes and build streaks to unlock achievements!",
                category_labels: &["Traits", "Layers", "Arcs", "Generation", "Architecture"],
            },
            super::NexusLocale::Fr => Self {
                title: "Stats d'Apprentissage",
                avg_score: "Moy:",
                streak: "Série:",
                best: "meilleur",
                quizzes: "Quiz:",
                achievements: "Succès:",
                score_history: "Historique des Scores",
                last: "Derniers",
                scores: "scores",
                category_mastery: "Maîtrise par Catégorie",
                achievements_title: "Succès",
                achievement_hint: "Complétez des quiz et construisez des séries pour débloquer des succès !",
                category_labels: &["Traits", "Couches", "Arcs", "Génération", "Architecture"],
            },
        }
    }

    /// Get category labels for current locale.
    pub fn category_labels(&self) -> &'static [&'static str] {
        self.category_labels
    }
}
