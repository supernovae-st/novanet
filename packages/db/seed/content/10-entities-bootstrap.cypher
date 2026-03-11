// ═══════════════════════════════════════════════════════════════════════════════
// 10-entities-bootstrap.cypher
// Bootstrap 9 pillar Entity nodes for QR Code AI
// v0.19.0 - Standard properties: node_class + provenance (ADR-042)
// v0.17.3 - Remove content.features (use HAS_FEATURE arcs instead)
// NOTE: Features are expressed via HAS_FEATURE arcs to Entity (category: feature) nodes
// ═══════════════════════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────────────────────────
// PILLAR 1: qr-code (THE core concept)
// ─────────────────────────────────────────────────────────────────────────────────
MERGE (e:Entity {key: 'entity:qr-code'})
SET e.display_name = 'QR Code',
    e.content = '{"definition": "Two-dimensional matrix barcode that encodes data in a scannable visual pattern. Core technology enabling all QR Code AI features.", "context": "Pillar entity of QR Code AI. All platform features connect to this core concept.", "technical": ["Data types: URLs, plain text, WiFi credentials, vCard", "Error correction: L/M/Q/H levels for partial damage recovery", "Module sizes: 21x21 to 177x177"]}',
    e.is_pillar = true,
    e.audience_segment = 'consumer',
    e.denomination_forms = '[{"type": "text", "value": "qr code"}, {"type": "title", "value": "QR Code"}, {"type": "abbrev", "value": "qr"}]',
    e.schema_org_type = 'SoftwareApplication',
    e.workflow_id = 'bootstrap',
    e.llm_context = '{"use":"When discussing QR codes, 2D barcodes, mobile scanning, or QR code technology fundamentals on QR Code AI.","triggers":["qr code","qr","barcode","scan","matrix code","quick response","2d barcode","scannable"],"not_for":["Specific QR code types (use dynamic-qr-code, static-qr-code)","1D barcodes (use barcode entity)"]}',
    e.node_class = 'Entity',
    e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// ─────────────────────────────────────────────────────────────────────────────────
// PILLAR 2: custom-qr-code (Customization capability)
// ─────────────────────────────────────────────────────────────────────────────────
MERGE (e:Entity {key: 'entity:custom-qr-code'})
SET e.display_name = 'Custom QR Code',
    e.content = '{"definition": "Parametric QR code with user-configured visual elements including colors, logos, shapes, and frames.", "context": "Customization pillar of QR Code AI. Enables branded QR codes without AI generation.", "technical": ["Logo safe zone: center 30% of QR code", "Color contrast: minimum 40% for scannability", "Export formats: PNG, SVG, PDF, EPS"]}',
    e.is_pillar = true,
    e.audience_segment = 'professional',
    e.denomination_forms = '[{"type": "text", "value": "custom qr code"}, {"type": "title", "value": "Custom QR Code"}, {"type": "abbrev", "value": "custom qr"}]',
    e.schema_org_type = 'SoftwareApplication',
    e.workflow_id = 'bootstrap',
    e.llm_context = '{"use":"When discussing QR code customization, branding, colors, logos, or visual design on QR Code AI.","triggers":["custom","personalized","branded","logo","colors","design","style","brand"],"not_for":["AI-generated artistic QR codes (use qr-code-art)"]}',
    e.node_class = 'Entity',
    e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// ─────────────────────────────────────────────────────────────────────────────────
// PILLAR 3: qr-code-art (AI differentiator)
// ─────────────────────────────────────────────────────────────────────────────────
MERGE (e:Entity {key: 'entity:qr-code-art'})
SET e.display_name = 'QR Code Art',
    e.content = '{"definition": "AI-generated artistic QR code where the scannable pattern is seamlessly fused into artwork using generative AI.", "context": "AI Art pillar of QR Code AI. Key differentiator using Stable Diffusion + ControlNet.", "technical": ["AI model: Stable Diffusion XL + QR ControlNet", "Generation time: 15-30 seconds", "Resolution: up to 1024x1024", "Scannability guarantee: 99.9% scan rate"]}',
    e.is_pillar = true,
    e.audience_segment = 'professional',
    e.denomination_forms = '[{"type": "text", "value": "qr code art"}, {"type": "title", "value": "QR Code Art"}, {"type": "abbrev", "value": "qr art"}]',
    e.schema_org_type = 'SoftwareApplication',
    e.workflow_id = 'bootstrap',
    e.llm_context = '{"use":"When discussing AI-generated QR codes, artistic QR codes, or generative AI for QR design on QR Code AI.","triggers":["art","artistic","AI","generative","creative","beautiful","image","stable diffusion"],"not_for":["Simple color/logo customization (use custom-qr-code)"]}',
    e.node_class = 'Entity',
    e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// ─────────────────────────────────────────────────────────────────────────────────
// PILLAR 4: dynamic-qr-code (Key feature - editable)
// ─────────────────────────────────────────────────────────────────────────────────
MERGE (e:Entity {key: 'entity:dynamic-qr-code'})
SET e.display_name = 'Dynamic QR Code',
    e.content = '{"definition": "Editable QR code that encodes a short link, allowing destination changes without reprinting. Includes analytics tracking.", "context": "Core monetization feature of QR Code AI. Enables subscription-based QR codes with analytics.", "technical": ["Redirect latency: <50ms average", "Analytics retention: 2 years", "Scan limit: unlimited on paid plans"]}',
    e.is_pillar = true,
    e.audience_segment = 'professional',
    e.denomination_forms = '[{"type": "text", "value": "dynamic qr code"}, {"type": "title", "value": "Dynamic QR Code"}, {"type": "abbrev", "value": "dynamic qr"}]',
    e.schema_org_type = 'SoftwareApplication',
    e.workflow_id = 'bootstrap',
    e.llm_context = '{"use":"When discussing editable QR codes, trackable QR codes, or QR codes with analytics on QR Code AI.","triggers":["dynamic","editable","changeable","trackable","analytics","statistics","update URL"],"not_for":["Permanent/fixed QR codes (use static-qr-code)"]}',
    e.node_class = 'Entity',
    e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// ─────────────────────────────────────────────────────────────────────────────────
// PILLAR 5: static-qr-code (Basic alternative)
// ─────────────────────────────────────────────────────────────────────────────────
MERGE (e:Entity {key: 'entity:static-qr-code'})
SET e.display_name = 'Static QR Code',
    e.content = '{"definition": "Fixed QR code with data encoded directly into the pattern. Cannot be edited after creation.", "context": "Free tier offering of QR Code AI. Entry point for user acquisition.", "technical": ["Max data capacity: 2953 bytes (alphanumeric)", "No server dependency: works offline", "No tracking: complete privacy"]}',
    e.is_pillar = false,
    e.audience_segment = 'consumer',
    e.denomination_forms = '[{"type": "text", "value": "static qr code"}, {"type": "title", "value": "Static QR Code"}, {"type": "abbrev", "value": "static qr"}]',
    e.schema_org_type = 'SoftwareApplication',
    e.workflow_id = 'bootstrap',
    e.llm_context = '{"use":"When discussing permanent QR codes, free QR codes, or QR codes without tracking on QR Code AI.","triggers":["static","permanent","fixed","free","simple","basic","no tracking"],"not_for":["Editable/trackable QR codes (use dynamic-qr-code)"]}',
    e.node_class = 'Entity',
    e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// ─────────────────────────────────────────────────────────────────────────────────
// SECONDARY 1: smart-link (URL shortener)
// ─────────────────────────────────────────────────────────────────────────────────
MERGE (e:Entity {key: 'entity:smart-link'})
SET e.display_name = 'Smart Link',
    e.content = '{"definition": "Intelligent shortened URL with conditional routing rules, device detection, and analytics.", "context": "Link management feature powering dynamic QR codes. Enables advanced routing logic.", "technical": ["Redirect speed: <30ms globally via CDN", "Custom domains: bring your own domain", "API access: programmatic link creation"]}',
    e.is_pillar = true,
    e.audience_segment = 'professional',
    e.denomination_forms = '[{"type": "text", "value": "smart link"}, {"type": "title", "value": "Smart Link"}, {"type": "abbrev", "value": "link"}]',
    e.schema_org_type = 'SoftwareApplication',
    e.workflow_id = 'bootstrap',
    e.llm_context = '{"use":"When discussing URL shortening, link management, conditional routing, or link analytics on QR Code AI.","triggers":["smart link","short URL","link","routing","redirect","device detection","geo-targeting"],"not_for":["QR codes themselves (use qr-code)"]}',
    e.node_class = 'Entity',
    e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// ─────────────────────────────────────────────────────────────────────────────────
// SECONDARY 2: landing-page (Page builder)
// ─────────────────────────────────────────────────────────────────────────────────
MERGE (e:Entity {key: 'entity:landing-page'})
SET e.display_name = 'Landing Page',
    e.content = '{"definition": "Mobile-optimized destination page created via drag-and-drop builder. No coding required.", "context": "Page builder feature of QR Code AI. Creates dedicated mobile-first destinations for QR scans.", "technical": ["Load time: <2 seconds globally", "Hosting: included with subscription", "Custom domains: connect your domain"]}',
    e.is_pillar = true,
    e.audience_segment = 'professional',
    e.denomination_forms = '[{"type": "text", "value": "landing page"}, {"type": "title", "value": "Landing Page"}, {"type": "abbrev", "value": "page"}]',
    e.schema_org_type = 'WebPage',
    e.workflow_id = 'bootstrap',
    e.llm_context = '{"use":"When discussing QR code destinations, mobile pages, or page builders on QR Code AI.","triggers":["landing page","destination","mobile page","page builder","no-code","conversion"],"not_for":["General website pages"]}',
    e.node_class = 'Entity',
    e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// ─────────────────────────────────────────────────────────────────────────────────
// SECONDARY 3: barcode (Barcode support)
// ─────────────────────────────────────────────────────────────────────────────────
MERGE (e:Entity {key: 'entity:barcode'})
SET e.display_name = 'Barcode',
    e.content = '{"definition": "One-dimensional linear barcode formats including EAN-13, UPC-A, Code 128, and Code 39.", "context": "Legacy format support on QR Code AI. Enables traditional barcode generation alongside QR codes.", "technical": ["EAN-13: 13-digit product codes", "UPC-A: 12-digit US retail codes", "Code 128: alphanumeric with full ASCII"]}',
    e.is_pillar = false,
    e.audience_segment = 'professional',
    e.denomination_forms = '[{"type": "text", "value": "barcode"}, {"type": "title", "value": "Barcode"}, {"type": "abbrev", "value": "barcode"}]',
    e.schema_org_type = 'SoftwareApplication',
    e.workflow_id = 'bootstrap',
    e.llm_context = '{"use":"When discussing 1D barcodes, product codes, or traditional barcode formats on QR Code AI.","triggers":["barcode","EAN","UPC","Code 128","Code 39","1D","linear","product code"],"not_for":["2D codes like QR codes (use qr-code)"]}',
    e.node_class = 'Entity',
    e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// ─────────────────────────────────────────────────────────────────────────────────
// SECONDARY 4: qr-code-generator (The main tool)
// ─────────────────────────────────────────────────────────────────────────────────
MERGE (e:Entity {key: 'entity:qr-code-generator'})
SET e.display_name = 'QR Code Generator',
    e.content = '{"definition": "Primary QR code creation tool supporting all formats, customization options, and export capabilities.", "context": "Main product entry point for QR Code AI. The tool that generates all QR code types.", "technical": ["Browser-based: no software installation", "API access: REST API for developers", "Offline capable: PWA support"]}',
    e.is_pillar = true,
    e.audience_segment = 'consumer',
    e.denomination_forms = '[{"type": "text", "value": "qr code generator"}, {"type": "title", "value": "QR Code Generator"}, {"type": "abbrev", "value": "generator"}]',
    e.schema_org_type = 'SoftwareApplication',
    e.workflow_id = 'bootstrap',
    e.llm_context = '{"use":"When discussing QR code creation tools, making QR codes, or the generator application on QR Code AI.","triggers":["generator","create","make","tool","app","free","online","generate"],"not_for":["QR code concepts (use qr-code entity)"]}',
    e.node_class = 'Entity',
    e.provenance = '{"source":"seed","file":"10-entities-bootstrap.cypher"}',
    e.created_at = coalesce(e.created_at, datetime()),
    e.updated_at = datetime();

// ═══════════════════════════════════════════════════════════════════════════════
// LINK ENTITIES TO PROJECT
// ═══════════════════════════════════════════════════════════════════════════════
MATCH (p:Project {key: 'qrcode-ai'})
MATCH (e:Entity)
WHERE e.key IN ['entity:qr-code', 'entity:custom-qr-code', 'entity:qr-code-art', 'entity:dynamic-qr-code', 'entity:static-qr-code', 'entity:smart-link', 'entity:landing-page', 'entity:barcode', 'entity:qr-code-generator']
MERGE (p)-[:HAS_ENTITY]->(e);

// ═══════════════════════════════════════════════════════════════════════════════
// SEMANTIC LINKS (pillar hierarchy)
// ═══════════════════════════════════════════════════════════════════════════════

// qr-code is the root pillar
MATCH (qr:Entity {key: 'entity:qr-code'})
MATCH (custom:Entity {key: 'entity:custom-qr-code'})
MATCH (art:Entity {key: 'entity:qr-code-art'})
MATCH (dynamic:Entity {key: 'entity:dynamic-qr-code'})
MATCH (static:Entity {key: 'entity:static-qr-code'})
MATCH (gen:Entity {key: 'entity:qr-code-generator'})

// custom-qr-code is a variant of qr-code
MERGE (custom)-[:SEMANTIC_LINK {link_type: 'variant_of', temperature: 0.9}]->(qr)

// qr-code-art is a variant of custom-qr-code
MERGE (art)-[:SEMANTIC_LINK {link_type: 'variant_of', temperature: 0.85}]->(custom)

// dynamic and static are types of qr-code
MERGE (dynamic)-[:SEMANTIC_LINK {link_type: 'type_of', temperature: 0.95}]->(qr)
MERGE (static)-[:SEMANTIC_LINK {link_type: 'type_of', temperature: 0.95}]->(qr)

// dynamic contrasts with static
MERGE (dynamic)-[:SEMANTIC_LINK {link_type: 'contrasts', temperature: 0.9}]->(static)

// generator enables qr-code creation
MERGE (gen)-[:SEMANTIC_LINK {link_type: 'enables', temperature: 0.95}]->(qr);

// smart-link and landing-page are related tools
MATCH (smart:Entity {key: 'entity:smart-link'})
MATCH (landing:Entity {key: 'entity:landing-page'})
MATCH (dynamic:Entity {key: 'entity:dynamic-qr-code'})

// dynamic qr codes require smart links
MERGE (dynamic)-[:SEMANTIC_LINK {link_type: 'requires', temperature: 0.9}]->(smart)

// smart links can point to landing pages
MERGE (smart)-[:SEMANTIC_LINK {link_type: 'enables', temperature: 0.8}]->(landing);
