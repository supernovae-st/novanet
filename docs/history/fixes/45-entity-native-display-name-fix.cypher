// packages/db/seed/45-entity-native-display-name-fix.cypher
// v0.13.0 - EntityNative.display_name corrections for fr-FR locale
//
// FRAMEWORK DE DECISION (validated by SEO data):
// ============================================================================
// 1. MARQUES = ANGLAIS (Instagram, Facebook, HubSpot, etc.)
// 2. STANDARDS TECHNIQUES = ANGLAIS (QR Code, EAN-13, Code 128, etc.)
// 3. VERBES/ACTIONS = FRANCAIS ("Créer", "Scanner", "Télécharger")
// 4. CONCEPTS UI = FRANCAIS si volume SEO similaire
// 5. JAMAIS DE FRANGLAIS (ordre anglais + mots français)
// 6. GRAMMAIRE FRANCAISE (prépositions correctes)
//
// SEO DATA REFERENCE (fr-FR):
// - "qr code" (EN) = 115,000 → garder "QR Code"
// - "qr code generator" (EN) = 52,000 vs "générateur qr code" = 5,000
// - "scanner qr code" = 17,000 vs "lecteur qr code" = 4,800
// - "carte de visite" = 1,000 (français préféré à "vCard")
// ============================================================================

// ============================================================================
// SECTION 1: FRANGLAIS → FRANÇAIS CORRECT (ordre des mots)
// ============================================================================
// Problème: ordre anglais "Noun Adjective" → français "Adjective de Noun"

// Générateurs
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-generator'})
SET en.display_name = 'Générateur de QR Code';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'barcode-generator'})
SET en.display_name = 'Générateur de Code-barres';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'batch-qr-generator'})
SET en.display_name = 'Générateur de QR Code en Lot';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'vcard-generator'})
SET en.display_name = 'Générateur de vCard';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'wifi-qr-generator'})
SET en.display_name = 'Générateur de QR Code WiFi';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'free-vs-paid-qr-generator'})
SET en.display_name = 'Générateur QR Code Gratuit vs Premium';

// Scanners
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'barcode-scanner'})
SET en.display_name = 'Scanner de Code-barres';

// Formats (ordre français)
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'barcode-format'})
SET en.display_name = 'Format de Code-barres';

// Guides
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'dynamic-vs-static-guide'})
SET en.display_name = 'Guide Dynamique vs Statique';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'dynamic-vs-static-qr-code'})
SET en.display_name = 'QR Code Dynamique vs Statique';

// Builders → Créateurs
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'landing-page-builder'})
SET en.display_name = 'Créateur de Page de Destination';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'landing-page-type'})
SET en.display_name = 'Type de Page de Destination';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'link-in-bio-builder'})
SET en.display_name = 'Créateur de Lien en Bio';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'menu-builder'})
SET en.display_name = 'Créateur de Menu';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'utm-builder'})
SET en.display_name = 'Créateur UTM';

// ============================================================================
// SECTION 2: HYBRIDES BIZARRES → FRANÇAIS CORRECT
// ============================================================================
// Problème: mélange incohérent anglais/français

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'click-tracking'})
SET en.display_name = 'Suivi des Clics';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'geo-tracking'})
SET en.display_name = 'Suivi Géographique';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'change-colors'})
SET en.display_name = 'Changer les Couleurs';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'bulk-creation'})
SET en.display_name = 'Création en Masse';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'custom-link-preview'})
SET en.display_name = 'Aperçu de Lien Personnalisé';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'event-management'})
SET en.display_name = 'Gestion d''Événements';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'event-rsvp'})
SET en.display_name = 'RSVP Événement';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'product-labels'})
SET en.display_name = 'Étiquettes Produit';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'product-packaging'})
SET en.display_name = 'Emballage Produit';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-contactless-payment'})
SET en.display_name = 'Paiement Sans Contact';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-social'})
SET en.display_name = 'QR Code Réseaux Sociaux';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-video-platform'})
SET en.display_name = 'QR Code Plateforme Vidéo';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-packaging-label'})
SET en.display_name = 'QR Code Emballage et Étiquette';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-event-checkin'})
SET en.display_name = 'QR Code Check-in Événement';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-analytics-guide'})
SET en.display_name = 'Guide Analytiques QR Code';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-print-guide'})
SET en.display_name = 'Guide Impression QR Code';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'time-series'})
SET en.display_name = 'Analytiques Séries Temporelles';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-app'})
SET en.display_name = 'Application QR Code';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-app-download'})
SET en.display_name = 'Télécharger Application QR Code';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-file'})
SET en.display_name = 'QR Code Téléchargement Fichier';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'scan-counting'})
SET en.display_name = 'Comptage des Scans';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'track-scans'})
SET en.display_name = 'Suivre les Scans';

// ============================================================================
// SECTION 3: 100% ANGLAIS → FRANÇAIS
// ============================================================================
// Termes qui devraient être en français

// Actions
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'add-logo'})
SET en.display_name = 'Ajouter un Logo';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'edit-destination'})
SET en.display_name = 'Modifier la Destination';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'shorten-url'})
SET en.display_name = 'Raccourcir URL';

// Catégories professionnelles
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'agencies'})
SET en.display_name = 'Agences';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'creative-agencies'})
SET en.display_name = 'Agences Créatives';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'marketing-agencies'})
SET en.display_name = 'Agences Marketing';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'developers'})
SET en.display_name = 'Développeurs';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'enterprise'})
SET en.display_name = 'Entreprise';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'small-business'})
SET en.display_name = 'Petites Entreprises';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'freelancers'})
SET en.display_name = 'Freelances';

// Concepts UI
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'announcement'})
SET en.display_name = 'Annonce';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'consulting'})
SET en.display_name = 'Conseil';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'contextual-routing'})
SET en.display_name = 'Routage Contextuel';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'data-capacity'})
SET en.display_name = 'Capacité de Données';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'device-detection'})
SET en.display_name = 'Détection d''Appareil';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'direct-mail'})
SET en.display_name = 'Publipostage';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'emails'})
SET en.display_name = 'E-mails';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'encoding-mode'})
SET en.display_name = 'Mode d''Encodage';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'error-correction'})
SET en.display_name = 'Correction d''Erreur';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'expiration'})
SET en.display_name = 'Expiration du Lien';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'finder-pattern'})
SET en.display_name = 'Motif de Repérage';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'fitness'})
SET en.display_name = 'Fitness';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'flyers'})
SET en.display_name = 'Flyers';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'forms'})
SET en.display_name = 'Formulaires';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'funny-qr-codes'})
SET en.display_name = 'QR Codes Amusants';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'newspapers'})
SET en.display_name = 'Journaux';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'password-protection'})
SET en.display_name = 'Protection par Mot de Passe';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'presentations'})
SET en.display_name = 'Présentations';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'quiet-zone'})
SET en.display_name = 'Zone de Silence';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'table-tents'})
SET en.display_name = 'Chevalets de Table';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'team-workspaces'})
SET en.display_name = 'Espaces d''Équipe';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'timing-pattern'})
SET en.display_name = 'Motif de Synchronisation';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'transportation'})
SET en.display_name = 'Transport';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'url-shortener'})
SET en.display_name = 'Raccourcisseur d''URL';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'websites'})
SET en.display_name = 'Sites Web';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'tickets-physical'})
SET en.display_name = 'Tickets Physiques';

// QR Code spécifiques
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-ai-vs-competitors'})
SET en.display_name = 'QR Code AI vs Concurrents';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-attendance'})
SET en.display_name = 'QR Code Présence';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-bank-transfer'})
SET en.display_name = 'QR Code Virement Bancaire';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-business-card-guide'})
SET en.display_name = 'Guide QR Code Carte de Visite';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-certificate'})
SET en.display_name = 'QR Code Certificat';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-coordinates'})
SET en.display_name = 'QR Code Coordonnées GPS';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-coupon'})
SET en.display_name = 'QR Code Coupon';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-dark-mode'})
SET en.display_name = 'Mode Sombre';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-design-guide'})
SET en.display_name = 'Guide Design QR Code';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-email-signature'})
SET en.display_name = 'QR Code Signature Email';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-flyer'})
SET en.display_name = 'QR Code Flyer';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-image-gallery'})
SET en.display_name = 'QR Code Galerie d''Images';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-light-mode'})
SET en.display_name = 'Mode Clair';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-marketing-guide'})
SET en.display_name = 'Guide Marketing QR Code';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-medical-id'})
SET en.display_name = 'QR Code Identifiant Médical';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-messaging'})
SET en.display_name = 'QR Codes Messagerie';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-museum-exhibit'})
SET en.display_name = 'QR Code Exposition Musée';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-music-platform'})
SET en.display_name = 'QR Codes Plateforme Musicale';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-networking'})
SET en.display_name = 'QR Code Networking';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-pet-tag'})
SET en.display_name = 'QR Code Médaille Animal';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-poster'})
SET en.display_name = 'QR Code Affiche';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-professional'})
SET en.display_name = 'QR Codes Réseau Professionnel';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-restaurant-guide'})
SET en.display_name = 'Guide QR Code Restaurant';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-resume'})
SET en.display_name = 'QR Code CV';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-review'})
SET en.display_name = 'QR Code Avis';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-reviews'})
SET en.display_name = 'QR Code pour Avis';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-scavenger-hunt'})
SET en.display_name = 'QR Code Chasse au Trésor';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-security-guide'})
SET en.display_name = 'Guide Sécurité QR Code';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-table-tent'})
SET en.display_name = 'QR Code Chevalet de Table';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-tattoo'})
SET en.display_name = 'QR Code Tatouage';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-ticket'})
SET en.display_name = 'QR Code Ticket';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-wedding'})
SET en.display_name = 'QR Code Mariage';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-art-installation'})
SET en.display_name = 'Installation Artistique QR';

// Intégrations - garder le nom de marque, traduire "Integration"
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'google-sheets-integration'})
SET en.display_name = 'Intégration Google Sheets';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'hubspot-integration'})
SET en.display_name = 'Intégration HubSpot';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'mailchimp-integration'})
SET en.display_name = 'Intégration Mailchimp';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'make-integration'})
SET en.display_name = 'Intégration Make';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'n8n-integration'})
SET en.display_name = 'Intégration n8n';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'notion-integration'})
SET en.display_name = 'Intégration Notion';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'salesforce-integration'})
SET en.display_name = 'Intégration Salesforce';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'shopify-integration'})
SET en.display_name = 'Intégration Shopify';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'slack-integration'})
SET en.display_name = 'Intégration Slack';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'woocommerce-integration'})
SET en.display_name = 'Intégration WooCommerce';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'wordpress-integration'})
SET en.display_name = 'Intégration WordPress';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'zapier-integration'})
SET en.display_name = 'Intégration Zapier';

// API Access
MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'api'})
SET en.display_name = 'Accès API';

// ============================================================================
// SECTION 4: CORRECTIONS MINEURES (accents, grammaire)
// ============================================================================

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'business-cards'})
SET en.display_name = 'Cartes de Visite';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'posters-billboards'})
SET en.display_name = 'Affiches et Panneaux';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'scan-limit'})
SET en.display_name = 'Limite de Scans';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'stickers-labels'})
SET en.display_name = 'Autocollants et Étiquettes';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'how-to-create-qr-code'})
SET en.display_name = 'Comment Créer un QR Code';

MATCH (en:EntityNative {locale_key: 'fr-FR', entity_key: 'qr-code-calendar'})
SET en.display_name = 'QR Code Événement Calendrier';

// ============================================================================
// SECTION 5: VERIFICATION - NE PAS MODIFIER (marques et standards)
// ============================================================================
// Ces entrées sont CORRECTES et ne doivent PAS être modifiées:
//
// MARQUES (invariants):
// - Instagram, Facebook, LinkedIn, Twitter, TikTok, Snapchat
// - Google, Apple, YouTube, Spotify, SoundCloud, Pinterest
// - PayPal, Venmo, Shopify, WooCommerce, HubSpot, Mailchimp
// - Salesforce, Zapier, Make, n8n, Notion, Slack
// - WhatsApp, Telegram, Waze
//
// STANDARDS TECHNIQUES (invariants):
// - QR Code, EAN-13, EAN-8, Code 128, Code 39
// - ITF-14, UPC-A, UPC-E, GS1-128, GS1 DataMatrix
// - Data Matrix, Aztec Code, MaxiCode, PDF417, Codabar
// - MSI Plessey, MeCard, vCard, PIX, UPI
// - WiFi, NFC, API, URL, UTM, SMS
//
// TERMES DEJA CORRECTS:
// - QR Code, QR Code Dynamique, QR Code Statique
// - Page de Destination, Lien Intelligent, Lien Court
// - Scanner QR Code, Télécharger QR Code, Partager QR Code
// - Créer QR Code, Personnaliser QR Code
// ============================================================================

// ============================================================================
// VERIFICATION QUERIES
// ============================================================================
// Exécuter pour vérifier les corrections:
//
// -- Compter les display_name en anglais restants
// MATCH (en:EntityNative {locale_key: 'fr-FR'})
// WHERE NOT en.display_name CONTAINS 'é'
//   AND NOT en.display_name CONTAINS 'è'
//   AND NOT en.display_name CONTAINS 'à'
//   AND NOT en.display_name CONTAINS 'ç'
//   AND NOT en.display_name CONTAINS 'ô'
//   AND NOT en.display_name CONTAINS 'û'
//   AND NOT en.display_name CONTAINS 'ê'
//   AND NOT en.display_name CONTAINS 'î'
//   AND NOT en.display_name CONTAINS '''
// RETURN count(en) as still_english;
//
// -- Vérifier les corrections appliquées
// MATCH (en:EntityNative {locale_key: 'fr-FR'})
// WHERE en.entity_key IN ['qr-code-generator', 'add-logo', 'agencies', 'click-tracking']
// RETURN en.entity_key, en.display_name;
//
