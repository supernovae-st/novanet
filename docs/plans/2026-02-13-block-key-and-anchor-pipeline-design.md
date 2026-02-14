# Design: Block Key, Anchor & Pipeline Architecture

**Date**: 2026-02-13
**Status**: Implemented
**Ticket**: N/A (Feature designed and implemented in live session)

## 1. Summary

This document summarizes the new architecture for Block identification, URL anchoring, and the creation of a comprehensive data pipeline for page generation, as designed and implemented during our session.

## 2. Problem Statement

1.  The previous key generation convention for `:Block` nodes was not robust and could lead to non-unique keys.
2.  There was no mechanism for creating user-facing URL anchor links (`#section`) for specific blocks on a page.
3.  The data pipeline for generating a page and its relationship to all influencing context (Locale, SEO, Rules, etc.) was not explicitly documented or visualized.

## 3. Architectural Solution

We designed and implemented a new, robust architecture based on the following principles:

### 3.1. Block Identification and Anchoring

-   **Invariant `Block` Node:**
    -   **`key`**: A new, unique, stable primary key with the convention `blk-{page_key}-{slugified_display_name}` (e.g., `blk-custom-qr-code-design-features`).
    -   **`anchor_id`**: A new property holding the canonical, non-translated anchor slug (e.g., `design-features`). This serves as a stable bridge between locales.

-   **Localized `BlockGenerated` Node:**
    -   **`anchor_slug`**: A new property holding the localized, public-facing anchor slug (e.g., `fonctionnalites-design`). This is the value used for the `id` attribute in the final HTML.

### 3.2. End-to-End Data Pipeline

-   A complete vertical slice of data has been created in a new Cypher seed file to serve as a reference implementation.
-   This data includes all nodes from the invariant blueprint (Page, Block, BlockType, BlockRules, Instructions) to the rich localized context (EntityContent, Culture, Market, SEOKeywords) and the final generated output (BlockGenerated, PageGenerated).

### 3.3. Visualization View

-   A new comprehensive Cypher query was created to visualize this entire pipeline.
-   This query has been added to the project as a new "View" for easy access and validation.

## 4. Implementation Artifacts

The following files were created or modified to implement this architecture:

1.  **Schema Definitions (`packages/core/models/node-classes/`):**
    -   `.../structure/page.yaml`: Documentation improved.
    -   `.../structure/block.yaml`: Schema updated with `anchor_id` and new key convention; documentation improved.
    -   `.../instruction/block-type.yaml`: Documentation improved.
    -   `.../instruction/block-rules.yaml`: Documentation improved.
    -   `.../output/block-generated.yaml`: Schema updated with `anchor_slug`.

2.  **Data Seeding (`packages/db/seed/entities/`):**
    -   `qrcode-ai-phase-16-custom-qr-page-pipeline.cypher`: A new, comprehensive seed file was created to provide a concrete example of the full pipeline for the `custom-qr-code` page.

3.  **Views (`packages/core/models/views/`):**
    -   `page-pipeline-overview.yaml`: A new view was created containing the "ultimate" Cypher query for visualization.
    -   `_registry.yaml`: Updated to include the new view.

4.  **Architectural Documentation (`docs/book/architecture/`):**
    -   `01_page-assembly-pipeline.md`: A new document containing detailed diagrams and explanations of the generation pipeline was created.
