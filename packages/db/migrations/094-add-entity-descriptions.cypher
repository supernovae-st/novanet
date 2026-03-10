// ============================================================================
// PLAN B - Migration 094: Add Descriptions to Entity Nodes
// ============================================================================
// Priority: STRUCTURE (Entity documentation)
// Fixes: Entities missing description field
// CSR Impact: Enables entity documentation and LLM context
// Note: Entity keys use "entity:" prefix
// ============================================================================

// Update qr-code entity
MATCH (e:Entity {key: 'entity:qr-code'})
SET e.description = COALESCE(e.description,
  'A two-dimensional barcode that stores data as a pattern of black and white squares, scannable by smartphones and cameras'),
    e.updated_at = datetime();

// Update custom-qr-code entity
MATCH (e:Entity {key: 'entity:custom-qr-code'})
SET e.description = COALESCE(e.description,
  'A parametric QR code with user-configured visual elements including colors, logos, shapes, and frames'),
    e.updated_at = datetime();

// Update qr-code-art entity
MATCH (e:Entity {key: 'entity:qr-code-art'})
SET e.description = COALESCE(e.description,
  'An AI-generated artistic QR code where the scannable pattern is seamlessly fused into artwork using generative AI'),
    e.updated_at = datetime();

// Update dynamic-qr-code entity
MATCH (e:Entity {key: 'entity:dynamic-qr-code'})
SET e.description = COALESCE(e.description,
  'An editable QR code that encodes a short link, allowing destination changes without reprinting. Includes analytics tracking.'),
    e.updated_at = datetime();

// Update static-qr-code entity
MATCH (e:Entity {key: 'entity:static-qr-code'})
SET e.description = COALESCE(e.description,
  'A fixed QR code with data encoded directly into the pattern. Cannot be edited after creation.'),
    e.updated_at = datetime();

// Update smart-link entity
MATCH (e:Entity {key: 'entity:smart-link'})
SET e.description = COALESCE(e.description,
  'An intelligent shortened URL with conditional routing rules, device detection, and analytics.'),
    e.updated_at = datetime();

// Update landing-page entity
MATCH (e:Entity {key: 'entity:landing-page'})
SET e.description = COALESCE(e.description,
  'A mobile-optimized destination page created via drag-and-drop builder. No coding required.'),
    e.updated_at = datetime();

// Update barcode entity
MATCH (e:Entity {key: 'entity:barcode'})
SET e.description = COALESCE(e.description,
  'One-dimensional linear barcode formats including EAN-13, UPC-A, Code 128, and Code 39.'),
    e.updated_at = datetime();

// Update qr-code-generator entity
MATCH (e:Entity {key: 'entity:qr-code-generator'})
SET e.description = COALESCE(e.description,
  'Primary QR code creation tool supporting all formats, customization options, and export capabilities.'),
    e.updated_at = datetime();

// Verify entity descriptions
MATCH (e:Entity)
RETURN e.key AS entity,
       CASE WHEN e.description IS NOT NULL THEN substring(e.description, 0, 50) + '...' ELSE 'MISSING' END AS description,
       CASE WHEN e.llm_context IS NOT NULL THEN 'HAS_CONTEXT' ELSE 'MISSING' END AS llm_context,
       CASE WHEN e.description IS NOT NULL AND e.llm_context IS NOT NULL THEN 'COMPLETE' ELSE 'INCOMPLETE' END AS status
ORDER BY e.key;
