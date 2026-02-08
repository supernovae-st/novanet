#!/usr/bin/env python3
"""Fix ALL template descriptions in EntityContent fr-FR seed file (v3).

Comprehensive fix for all 160+ entity keys with template patterns.
"""

import re
from pathlib import Path

# Entity key -> proper French description (SEO-optimized, concise)
DESCRIPTIONS = {
    # === Core Product ===
    "qr-code": "Code matriciel 2D encodant données textuelles accessibles par scan smartphone.",
    "qr-code-generator": "Générateur de QR Codes personnalisables avec suivi statistique.",
    "barcode-generator": "Générateur de codes-barres 1D et 2D pour produits et inventaire.",
    "batch-qr-generator": "Création en masse de QR Codes à partir de fichiers CSV.",
    
    # === Actions ===
    "create-qr-code": "Créer un QR Code personnalisé en quelques clics.",
    "create-barcode": "Créer un code-barres pour vos produits et emballages.",
    "create-landing-page": "Créer une page de destination mobile-first.",
    "create-smart-link": "Créer un lien intelligent avec redirection conditionnelle.",
    "shorten-url": "Raccourcir une URL avec statistiques de clics.",
    "edit-destination": "Modifier la destination d'un QR Code dynamique sans réimpression.",
    "share-qr-code": "Partager votre QR Code sur réseaux sociaux et messageries.",
    "scan-barcode": "Scanner un code-barres pour obtenir les informations produit.",
    
    # === Content Types - Social ===
    "qr-code-instagram": "Lien direct vers profil Instagram pour gagner des abonnés.",
    "qr-code-facebook": "Lien vers page ou profil Facebook.",
    "qr-code-twitter": "Accès direct au compte Twitter/X.",
    "qr-code-tiktok": "Lien vers profil TikTok pour audience jeune.",
    "qr-code-linkedin": "Accès rapide au profil LinkedIn professionnel.",
    "qr-code-youtube": "Lien vers chaîne ou vidéo YouTube.",
    "qr-code-pinterest": "Lien vers tableaux Pinterest.",
    "qr-code-snapchat": "Snapcode pour ajout Snapchat instantané.",
    "qr-code-spotify": "Accès à profil ou playlist Spotify.",
    "qr-code-apple-music": "Lien vers musique Apple Music.",
    "qr-code-whatsapp": "Lien WhatsApp avec message pré-rempli.",
    "qr-code-telegram": "Accès direct à canal Telegram.",
    "qr-code-messaging": "QR Code pour messageries instantanées.",
    "qr-code-networking": "Hub professionnel pour networking.",
    "instagram": "Solutions QR Code pour Instagram.",
    
    # === Content Types - Payment ===
    "qr-code-payment": "QR Code de paiement sans contact sécurisé.",
    "qr-code-pix": "Paiement instantané via PIX brésilien.",
    "qr-code-upi": "Paiement mobile UPI pour le marché indien.",
    "qr-code-bitcoin": "Adresse Bitcoin pour paiements crypto.",
    "qr-code-venmo": "Transfert d'argent Venmo simplifié.",
    "qr-code-contactless-payment": "Paiement sans contact par QR Code.",
    "paypal": "Solutions QR Code pour paiements PayPal.",
    
    # === Content Types - Location ===
    "qr-code-location": "Coordonnées GPS pour localisation précise.",
    "qr-code-google-maps": "Itinéraire Google Maps vers votre adresse.",
    "qr-code-apple-maps": "Navigation Apple Maps pour iOS.",
    "qr-code-coordinates": "Coordonnées géographiques pour navigation.",
    "waze": "Guidage Waze avec trafic temps réel.",
    
    # === Content Types - Business ===
    "qr-code-business-card": "QR Code pour carte de visite numérique.",
    "qr-code-email-signature": "QR Code compact pour signature email.",
    "qr-code-calendar": "Événement calendrier ajouté en un scan.",
    "qr-code-pdf": "Accès direct à document PDF.",
    "qr-code-url": "Redirection vers URL de votre choix.",
    "qr-code-file": "Téléchargement de fichier via QR Code.",
    "documents": "Partage de documents via QR Code.",
    
    # === Print Placements ===
    "qr-code-poster": "QR Code grand format pour affiches publicitaires.",
    "qr-code-packaging-label": "QR Code pour étiquettes et emballages.",
    "business-cards": "QR Code optimisé pour cartes de visite.",
    "flyers": "QR Code haute résolution pour flyers.",
    "banners": "QR Code pour bannières événementielles.",
    "stickers-labels": "QR Code adhésif pour surfaces diverses.",
    "table-tents": "QR Code pour chevalets de table restaurant.",
    "posters-billboards": "QR Code pour affiches grand format.",
    "menus-printed": "QR Code pour menus imprimés.",
    "product-labels": "QR Code pour étiquettes produit.",
    "product-packaging": "QR Code pour emballages produit.",
    "receipts": "QR Code pour tickets et reçus.",
    
    # === Landing Pages ===
    "landing-page-builder": "Constructeur de pages de destination optimisées mobile.",
    "menu-builder": "Constructeur de menus digitaux interactifs.",
    "menu-restaurant": "Menu digital avec photos et prix actualisables.",
    "vcard-generator": "Générateur de cartes de visite numériques vCard.",
    "wifi-qr-generator": "Générateur de QR Codes WiFi pour connexion automatique.",
    "forms": "Formulaires en ligne personnalisables.",
    "announcement": "Page d'annonce pour actualités et événements.",
    "booking-appointment": "Système de réservation en ligne.",
    "qr-code-survey": "Questionnaire de satisfaction client.",
    "qr-code-review": "Collecte d'avis clients via QR Code.",
    "qr-code-reviews": "Gestion des avis clients par QR Code.",
    
    # === App Downloads ===
    "qr-code-app": "Lien vers téléchargement d'application.",
    "qr-code-app-download": "Téléchargement d'app mobile via QR Code.",
    
    # === Events ===
    "qr-code-attendance": "QR Code de présence pour émargement.",
    "qr-code-ticket": "QR Code pour billets événementiels.",
    "event-management": "Solutions QR Code pour gestion d'événements.",
    "qr-code-scavenger-hunt": "Chasse au trésor interactive avec QR Codes.",
    
    # === Barcode Types ===
    "data-matrix": "Code 2D Data Matrix compact haute densité.",
    "aztec-code": "Code 2D Aztec pour billets de transport.",
    "pdf417": "Code 2D PDF417 haute capacité données.",
    "gs1-128": "Code-barres GS1-128 pour chaîne d'approvisionnement.",
    "gs1-datamatrix": "Code GS1 DataMatrix pour traçabilité santé.",
    "itf-14": "Code-barres ITF-14 pour cartons et palettes.",
    "code-39": "Code-barres Code 39 alphanumérique industriel.",
    "msi-plessey": "Code-barres MSI Plessey pour inventaire.",
    
    # === Features ===
    "analytics": "Tableau de bord analytique des scans en temps réel.",
    "click-tracking": "Suivi des clics pour analyse de performance.",
    "scan-counting": "Compteur de scans précis et fiable.",
    "device-detection": "Détection automatique du type d'appareil.",
    "geo-tracking": "Géolocalisation des scans pour analyse territoriale.",
    "time-series": "Données temporelles pour analyse de tendances.",
    "contextual-routing": "Redirection selon contexte (heure, lieu, appareil).",
    "scan-limit": "Limite de scans configurable.",
    "custom-domain-name": "Domaine personnalisé pour URLs de marque.",
    "custom-link-preview": "Aperçu de lien personnalisable pour partage.",
    "utm-builder": "Générateur de paramètres UTM pour tracking marketing.",
    "url-shortener": "Raccourcisseur d'URL avec statistiques détaillées.",
    "white-label": "Solution marque blanche personnalisable.",
    "api": "API REST pour intégration dans vos applications.",
    
    # === Technical Concepts ===
    "encoding-mode": "Modes d'encodage QR Code (numérique, alphanumérique, binaire).",
    "error-correction": "Niveaux de correction d'erreur QR Code (L, M, Q, H).",
    "qr-code-version": "Versions QR Code (1-40) et capacité de données.",
    "finder-pattern": "Motifs de repérage pour lecture QR Code.",
    "timing-pattern": "Motifs de synchronisation QR Code.",
    "quiet-zone": "Zone de silence autour du QR Code pour lisibilité.",
    
    # === Industries ===
    "restaurants": "Solutions QR Code pour restaurants : menus, avis, paiement.",
    "manufacturing": "QR Codes pour industrie : traçabilité, documentation.",
    "event-management": "QR Codes pour événements : billets, accès, networking.",
    "nonprofits": "QR Codes pour associations : dons, bénévolat.",
    "enterprise": "Solutions QR Code entreprise avec sécurité avancée.",
    
    # === Mediums ===
    "catalogs": "QR Codes pour catalogues produits interactifs.",
    "magazines": "QR Codes pour magazines avec contenus enrichis.",
    "newspapers": "QR Codes pour presse avec articles complets.",
    "direct-mail": "QR Codes pour publipostage personnalisé.",
    "emails": "QR Codes pour campagnes email marketing.",
    "websites": "QR Codes pour sites web avec analytics.",
    "presentations": "QR Codes pour présentations interactives.",
    
    # === User Types ===
    "developers": "Outils QR Code pour développeurs (API, SDK).",
    "creative-agencies": "Solutions pour agences créatives.",
    "freelancers": "Outils QR Code pour freelances et indépendants.",
    "small-business": "QR Codes pour petites entreprises et commerces.",
    
    # === Integrations ===
    "shopify": "Intégration Shopify pour e-commerce.",
    "shopify-integration": "Connecteur Shopify natif pour boutiques en ligne.",
    "woocommerce": "Intégration WooCommerce pour WordPress.",
    "woocommerce-integration": "Plugin WooCommerce pour QR Codes produits.",
    "wordpress-integration": "Plugin WordPress pour QR Codes.",
    "hubspot": "Intégration HubSpot pour marketing automation.",
    "hubspot-integration": "Connecteur HubSpot natif pour CRM.",
    "mailchimp": "Intégration Mailchimp pour campagnes email.",
    "mailchimp-integration": "Connecteur Mailchimp natif.",
    "google-sheets-integration": "Synchronisation Google Sheets pour données.",
    "zapier-integration": "Automatisation via Zapier (5000+ apps).",
    "slack-integration": "Notifications Slack pour scans.",
    "notion-integration": "Intégration Notion pour documentation.",
    "n8n": "Workflow automation avec n8n.",
    "n8n-integration": "Connecteur n8n pour automatisation.",
    "make": "Intégration Make (ex-Integromat).",
    "make-integration": "Automatisation via Make.",
    
    # === Guides & Comparisons ===
    "how-to-create-qr-code": "Guide complet pour créer un QR Code.",
    "qr-code-marketing-guide": "Stratégies marketing avec QR Codes.",
    "qr-code-print-guide": "Guide d'impression QR Code haute qualité.",
    "qr-code-business-card-guide": "Guide QR Code pour cartes de visite.",
    "qr-code-restaurant-guide": "Guide QR Code pour restaurants.",
    "qr-code-analytics-guide": "Guide d'analyse des performances QR Code.",
    "qr-code-api-guide": "Documentation API QR Code.",
    "dynamic-vs-static-guide": "Comparaison QR Code dynamique vs statique.",
    "qr-code-vs-barcode": "Différences QR Code et code-barres.",
    "short-link-vs-qr-code": "Lien court vs QR Code : que choisir ?",
    "free-vs-paid-qr-generator": "Générateur QR Code gratuit vs payant.",
    "qr-code-ai-vs-competitors": "Comparatif QR Code AI vs concurrents.",
    "spotify-code-vs-qr-code": "Spotify Code vs QR Code musical.",
    
    # === Creative ===
    "qr-code-tattoo": "QR Code tatouage : art et technologie.",
    "qr-code-wedding": "QR Codes mariage : invitations, RSVP, photos.",
    "qr-code-art-installation": "QR Codes pour installations artistiques.",
    "funny-qr-codes": "QR Codes créatifs et humoristiques.",
    "qr-code-dark-mode": "QR Code mode sombre pour écrans.",
    
    # === Use Cases ===
    "qr-code-medical-id": "QR Code identification médicale d'urgence.",
    "qr-code-certificate": "QR Code pour certificats et diplômes.",
    "qr-code-loyalty-program": "Programme de fidélité par QR Code.",
    "qr-code-museum-exhibit": "QR Codes pour expositions muséales.",
    "qr-code-music-platform": "QR Code pour plateformes musicales.",
    "qr-code-video-platform": "QR Code pour plateformes vidéo.",
    "qr-code-product-authentication": "Authentification produit anti-contrefaçon.",
    "qr-code-professional": "Solutions QR Code pour professionnels.",

    # === Missing from first pass (34 keys) ===
    "qr-code-flyer": "QR Code haute résolution pour flyers et dépliants.",
    "qr-code-table-tent": "QR Code pour chevalet de table restaurant.",
    "link-in-bio": "Page bio centralisée regroupant tous vos liens.",
    "event-rsvp": "Page d'inscription événement avec confirmation.",
    "qr-code-social": "Hub réseaux sociaux en un seul QR Code.",
    "qr-code-soundcloud": "Accès à morceaux SoundCloud.",
    "qr-code-paypal": "Paiement sécurisé via PayPal.",
    "qr-code-bank-transfer": "Coordonnées bancaires pour virement.",
    "qr-code-waze": "Guidage Waze avec trafic temps réel.",
    "qr-code-app-store": "Lien direct vers App Store iOS.",
    "qr-code-play-store": "Lien direct vers Google Play Android.",
    "qr-code-pet-tag": "QR Code pour médaille d'identification animaux.",
    "ean-8": "Code-barres EAN-8 compact pour petits produits.",
    "upc-a": "Code-barres UPC-A standard nord-américain.",
    "upc-e": "Code-barres UPC-E condensé pour petits emballages.",
    "code-128": "Code-barres Code 128 haute densité logistique.",
    "codabar": "Code-barres Codabar pour bibliothèques.",
    "maxicode": "Code 2D MaxiCode pour tri postal automatisé.",
    "bulk-creation": "Création en masse de QR Codes depuis fichier.",
    "team-workspaces": "Espaces de travail collaboratifs pour équipes.",
    "webhooks": "Webhooks pour notifications automatisées.",
    "link-in-bio-builder": "Constructeur de pages bio pour réseaux sociaux.",
    "brochures": "QR Codes pour brochures et catalogues.",
    "tickets-physical": "QR Codes pour billetterie physique.",
    "change-colors": "Personnalisation des couleurs du QR Code.",
    "consulting": "Services de conseil QR Code pour entreprises.",
    "agencies": "Solutions QR Code pour agences marketing.",
    "soundcloud": "Intégration SoundCloud pour artistes.",
    "salesforce": "Intégration Salesforce pour CRM.",
    "salesforce-integration": "Connecteur Salesforce natif pour automatisation.",
    "data-capacity": "Capacité de données des différents formats QR.",
    "module": "Modules QR Code pour développeurs.",
    "qr-code-event-checkin": "QR Code d'enregistrement pour événements.",
    "qr-code-security-guide": "Guide de sécurité pour QR Codes.",
}

def fix_file(path: Path) -> int:
    """Fix template descriptions in seed file."""
    print(f"[DEBUG] Reading {path}")
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    print(f"[DEBUG] File size: {len(content)} bytes")
    print(f"[DEBUG] Template count before: {content.count('pour vos projets QR Code')}")
    print(f"[DEBUG] DESCRIPTIONS count: {len(DESCRIPTIONS)}")

    fixed = 0
    for entity_key, desc in DESCRIPTIONS.items():
        # Pattern: Match the MERGE block and its description with template text
        # Handle escaped quotes: match non-quote/non-backslash chars OR any escaped char
        pattern = rf"(entity_key = '{re.escape(entity_key)}',.*?el\.description = ')(?:[^'\\]|\\.)*pour vos projets QR Code\.'"
        
        escaped_desc = desc.replace("'", "\\'")
        replacement = rf"\g<1>{escaped_desc}'"
        
        new_content, count = re.subn(pattern, replacement, content, flags=re.DOTALL)
        if count > 0:
            content = new_content
            fixed += count

    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

    return fixed

if __name__ == "__main__":
    path = Path("packages/db/seed/11-entity-content-fr-fr.cypher")
    count = fix_file(path)
    print(f"Fixed {count} descriptions")
    
    # Check remaining templates
    with open(path, "r") as f:
        content = f.read()
    remaining = content.count("pour vos projets QR Code")
    print(f"Remaining template patterns: {remaining}")
    
    if remaining > 0:
        # Show which ones remain
        import re as re_mod
        matches = re_mod.findall(r"entity_key = '([^']+)',.*?pour vos projets QR Code\.'", content, flags=re_mod.DOTALL)
        print(f"\nRemaining entity keys ({len(matches)}):")
        for m in sorted(set(matches))[:30]:
            print(f"  {m}")
