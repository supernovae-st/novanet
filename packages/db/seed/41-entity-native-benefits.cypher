// packages/db/seed/41-entity-native-benefits.cypher
// v0.13.0 - EntityNative.benefits: Value propositions per locale
//
// ADR-029: *Native Pattern - EntityNative is human-authored native content
//
// benefits[] describes value propositions in the target locale.
// These are NOT translated - they are authored natively per locale.

// ============================================================================
// 1. QR CODE GENERATOR - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code-generator', locale_key: 'fr-FR'})
SET en.benefits = [
  "Création gratuite et illimitée",
  "Personnalisation complète (couleurs, logo, cadres)",
  "QR Codes dynamiques avec suivi statistique",
  "Téléchargement HD (PNG, SVG, PDF)",
  "Sans inscription requise"
];

// ============================================================================
// 2. QR CODE - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code', locale_key: 'fr-FR'})
SET en.benefits = [
  "Accès instantané aux informations",
  "Fonctionnement universel (tous smartphones)",
  "Solution sans contact idéale",
  "Économique et écologique (pas de papier)"
];

// ============================================================================
// 3. DYNAMIC QR CODE - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'dynamic-qr-code', locale_key: 'fr-FR'})
SET en.benefits = [
  "Modification de destination sans réimprimer",
  "Statistiques de scan en temps réel",
  "Programmation par date et heure",
  "Protection par mot de passe",
  "A/B testing intégré"
];

// ============================================================================
// 4. STATIC QR CODE - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'static-qr-code', locale_key: 'fr-FR'})
SET en.benefits = [
  "Gratuit à vie, sans abonnement",
  "Fonctionne hors ligne",
  "Stockage direct dans le code",
  "Idéal pour données fixes (WiFi, vCard)"
];

// ============================================================================
// 5. QR CODE WIFI - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code-wifi', locale_key: 'fr-FR'})
SET en.benefits = [
  "Connexion instantanée sans taper le mot de passe",
  "Idéal pour hôtels, restaurants, Airbnb",
  "Compatible tous smartphones",
  "Sécurisé (le mot de passe reste crypté)"
];

// ============================================================================
// 6. QR CODE VCARD - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code-vcard', locale_key: 'fr-FR'})
SET en.benefits = [
  "Partage de coordonnées en un scan",
  "Ajout direct au carnet de contacts",
  "Carte de visite digitale moderne",
  "Moins d erreurs de saisie"
];

// ============================================================================
// 7. QR CODE MENU - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code-menu', locale_key: 'fr-FR'})
SET en.benefits = [
  "Menu digital sans contact",
  "Mise à jour instantanée des prix",
  "Économie d impression",
  "Conformité réglementations sanitaires"
];

// ============================================================================
// 8. SMART LINK - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'smart-link', locale_key: 'fr-FR'})
SET en.benefits = [
  "Redirection intelligente par appareil",
  "Analytiques détaillées",
  "Personnalisation de l aperçu social",
  "Retargeting intégré"
];

// ============================================================================
// 9. BARCODE - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'barcode', locale_key: 'fr-FR'})
SET en.benefits = [
  "Standard universel de commerce",
  "Lecture ultra-rapide",
  "Intégration POS/ERP native",
  "Gestion d inventaire simplifiée"
];

// ============================================================================
// 10. LANDING PAGE - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'landing-page', locale_key: 'fr-FR'})
SET en.benefits = [
  "Création sans code en quelques minutes",
  "Templates optimisés conversion",
  "Hébergement inclus",
  "Domaine personnalisé disponible"
];

// ============================================================================
// 11. QR CODE INSTAGRAM - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code-instagram', locale_key: 'fr-FR'})
SET en.benefits = [
  "Redirection directe vers votre profil",
  "Augmentation des abonnés",
  "Idéal pour cartes de visite et flyers",
  "Statistiques de scan disponibles"
];

// ============================================================================
// 12. QR CODE FACEBOOK - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code-facebook', locale_key: 'fr-FR'})
SET en.benefits = [
  "Lien direct vers page ou profil",
  "Booste les likes et followers",
  "Simple à partager offline",
  "Mesure du ROI de vos supports print"
];

// ============================================================================
// 13. QR CODE YOUTUBE - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code-youtube', locale_key: 'fr-FR'})
SET en.benefits = [
  "Accès direct à votre vidéo ou chaîne",
  "Augmentation des vues et abonnés",
  "Parfait pour packaging produit",
  "Analytics de scan intégrés"
];

// ============================================================================
// 14. QR CODE PDF - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code-pdf', locale_key: 'fr-FR'})
SET en.benefits = [
  "Partage de documents sans email",
  "Mise à jour du PDF sans changer le QR",
  "Idéal pour catalogues et manuels",
  "Téléchargement direct sur smartphone"
];

// ============================================================================
// 15. QR CODE PAYMENT - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'qr-code-payment', locale_key: 'fr-FR'})
SET en.benefits = [
  "Paiement sans contact instantané",
  "Compatible PayPal, Stripe, crypto",
  "Idéal pour commerces et restaurants",
  "Réduction des frais de transaction"
];

// ============================================================================
// 16. CREATE QR CODE (action) - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'create-qr-code', locale_key: 'fr-FR'})
SET en.benefits = [
  "Gratuit et sans inscription",
  "Interface intuitive",
  "Résultat en moins de 30 secondes",
  "Formats haute résolution inclus"
];

// ============================================================================
// 17. SCAN QR CODE (action) - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'scan-qr-code', locale_key: 'fr-FR'})
SET en.benefits = [
  "Caméra smartphone native suffit",
  "Détection instantanée",
  "Historique de scans disponible",
  "Fonctionne même hors ligne (QR statiques)"
];

// ============================================================================
// 18. CUSTOM QR CODE - FR-FR
// ============================================================================

MATCH (en:EntityNative {entity_key: 'custom-qr-code', locale_key: 'fr-FR'})
SET en.benefits = [
  "Intégration de votre logo",
  "Couleurs de marque personnalisables",
  "Formes et styles variés",
  "Maintient la scannabilité garantie"
];

// ============================================================================
// VERIFICATION QUERY
// ============================================================================
// Run this to verify benefits were set:
//
// MATCH (en:EntityNative) WHERE en.benefits IS NOT NULL
// RETURN en.entity_key, size(en.benefits) AS benefit_count
// ORDER BY benefit_count DESC;
//
