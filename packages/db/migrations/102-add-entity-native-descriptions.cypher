// ============================================================================
// Migration 102: Add Missing EntityNative Descriptions
// ============================================================================
// Purpose: Add localized descriptions to 17 EntityNative nodes
// ============================================================================

// --- QR Code ---
MATCH (en:EntityNative {key: 'qr-code@en-US'})
SET en.description = 'A two-dimensional barcode that stores data as a pattern of black and white squares, scannable by smartphones and cameras.',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'qr-code@fr-FR'})
SET en.description = 'Un code-barres bidimensionnel qui stocke des données sous forme de carrés noirs et blancs, lisible par smartphones et appareils photo.',
    en.updated_at = datetime();

// --- Dynamic QR Code ---
MATCH (en:EntityNative {key: 'dynamic-qr-code@en-US'})
SET en.description = 'An editable QR code that encodes a short link, allowing destination changes without reprinting. Includes analytics tracking.',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'dynamic-qr-code@fr-FR'})
SET en.description = 'Un QR code modifiable qui encode un lien court, permettant de changer la destination sans réimpression. Inclut le suivi analytique.',
    en.updated_at = datetime();

// --- Static QR Code ---
MATCH (en:EntityNative {key: 'static-qr-code@en-US'})
SET en.description = 'A fixed QR code with data encoded directly into the pattern. Cannot be edited after creation.',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'static-qr-code@fr-FR'})
SET en.description = 'Un QR code fixe avec les données encodées directement dans le motif. Non modifiable après création.',
    en.updated_at = datetime();

// --- Custom QR Code ---
MATCH (en:EntityNative {key: 'custom-qr-code@en-US'})
SET en.description = 'A parametric QR code with user-configured visual elements including colors, logos, shapes, and frames.',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'custom-qr-code@fr-FR'})
SET en.description = 'Un QR code paramétrable avec éléments visuels personnalisés : couleurs, logos, formes et cadres.',
    en.updated_at = datetime();

// --- QR Code Art ---
MATCH (en:EntityNative {key: 'qr-code-art@en-US'})
SET en.description = 'An AI-generated artistic QR code where the scannable pattern is seamlessly fused into artwork using generative AI.',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'qr-code-art@fr-FR'})
SET en.description = 'Un QR code artistique généré par IA où le motif scannable est intégré harmonieusement dans une œuvre grâce à l\'IA générative.',
    en.updated_at = datetime();

// --- QR Code Generator ---
MATCH (en:EntityNative {key: 'qr-code-generator@en-US'})
SET en.description = 'Primary QR code creation tool supporting all formats, customization options, and export capabilities.',
    en.updated_at = datetime();

// --- Barcode ---
MATCH (en:EntityNative {key: 'barcode@en-US'})
SET en.description = 'One-dimensional linear barcode formats including EAN-13, UPC-A, Code 128, and Code 39.',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'barcode@fr-FR'})
SET en.description = 'Formats de codes-barres linéaires unidimensionnels incluant EAN-13, UPC-A, Code 128 et Code 39.',
    en.updated_at = datetime();

// --- Smart Link ---
MATCH (en:EntityNative {key: 'smart-link@en-US'})
SET en.description = 'An intelligent shortened URL with conditional routing rules, device detection, and analytics.',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'smart-link@fr-FR'})
SET en.description = 'Une URL raccourcie intelligente avec règles de routage conditionnel, détection d\'appareil et analytiques.',
    en.updated_at = datetime();

// --- Landing Page ---
MATCH (en:EntityNative {key: 'landing-page@en-US'})
SET en.description = 'A mobile-optimized destination page created via drag-and-drop builder. No coding required.',
    en.updated_at = datetime();

MATCH (en:EntityNative {key: 'landing-page@fr-FR'})
SET en.description = 'Une page de destination optimisée mobile créée via un constructeur glisser-déposer. Aucun code requis.',
    en.updated_at = datetime();

