#!/usr/bin/env python3
"""Generate EntityL10n fr-FR content from Entity definitions.

This script creates French localized content for each Entity.
CRITICAL: This is GENERATION, not translation.
- Use "QR Code" (not "Code QR") in French
- Generate content natively in French
- Formal but accessible tone for professional audience
"""

import json
import re
from pathlib import Path
from typing import Dict, Tuple

# =============================================================================
# FRENCH DESCRIPTION TEMPLATES BY TYPE
# =============================================================================

# These are native French descriptions, not translations
THING_DESCRIPTIONS = {
    "qr-code": "Code matriciel 2D permettant d'encoder des données accessibles par scan mobile.",
    "smart-link": "URL intelligente avec règles de routage pour rediriger selon l'appareil ou la localisation.",
    "barcode": "Code-barres linéaire 1D pour l'identification des produits (EAN, UPC, Code 128).",
    "landing-page": "Page web de destination créée via un constructeur no-code intégré.",
    "short-link": "URL raccourcie avec suivi des clics et statistiques de performance.",
    "qr-code-style": "Catégorie de style visuel pour personnaliser l'apparence des QR Codes.",
    "qr-code-content": "Catégorie de contenu définissant le type de données encodées dans un QR Code.",
    "qr-code-frame": "Modèle de placement physique optimisé pour l'impression des QR Codes.",
    "barcode-format": "Format technique de code-barres définissant la structure d'encodage.",
    "landing-page-type": "Type de page de destination adapté à un cas d'usage spécifique.",
    "custom-qr-code": "QR Code entièrement personnalisable avec couleurs, formes et logo.",
    "qr-code-image": "QR Code avec image ou photo en arrière-plan.",
    "qr-code-art": "QR Code artistique généré par intelligence artificielle.",
    "qr-code-photo": "QR Code intégrant une photographie en fond.",
    "qr-code-with-logo": "QR Code affichant un logo au centre pour renforcer l'identité de marque.",
    "qr-code-with-text": "QR Code accompagné d'un texte d'appel à l'action.",
    "qr-code-color": "Personnalisation des couleurs de premier plan et d'arrière-plan du QR Code.",
    "qr-code-shapes": "Personnalisation des formes des modules et des yeux du QR Code.",
    "qr-code-transparent-background": "QR Code avec fond transparent pour superposition sur d'autres visuels.",
    "qr-code-background": "Options de personnalisation de l'arrière-plan des QR Codes.",
    "qr-code-background-color": "Couleur d'arrière-plan unie pour les QR Codes.",
    "qr-code-background-gradient": "Dégradé de couleurs en arrière-plan des QR Codes.",
    "qr-code-background-image": "Image ou motif en arrière-plan des QR Codes.",
}

CONCEPT_DESCRIPTIONS = {
    "dynamic-qr-code": "QR Code modifiable après impression dont la destination peut être mise à jour sans régénérer le code.",
    "static-qr-code": "QR Code fixe avec données encodées directement, gratuit et permanent.",
    "qr-code-analytics": "Statistiques détaillées sur les scans : localisation, appareil, horaire.",
    "qr-code-tracking": "Suivi en temps réel des interactions avec vos QR Codes.",
}

ACTION_DESCRIPTIONS = {
    "create-qr-code": "Générer un QR Code personnalisé en quelques clics.",
    "scan-qr-code": "Lire un QR Code avec la caméra de votre smartphone.",
    "download-qr-code": "Télécharger votre QR Code en PNG, SVG ou PDF haute résolution.",
    "print-qr-code": "Imprimer votre QR Code avec les paramètres optimaux.",
    "share-qr-code": "Partager votre QR Code par email, réseaux sociaux ou lien direct.",
    "edit-qr-code": "Modifier le contenu ou le design de votre QR Code dynamique.",
    "track-qr-code": "Suivre les statistiques de scan de votre QR Code.",
    "customize-qr-code": "Personnaliser l'apparence de votre QR Code avec vos couleurs et logo.",
}

CONTENT_TYPE_DESCRIPTIONS = {
    "qr-code-url": "QR Code redirigeant vers une adresse web.",
    "qr-code-wifi": "QR Code pour connexion WiFi automatique.",
    "qr-code-vcard": "QR Code contenant une carte de visite électronique.",
    "qr-code-sms": "QR Code pré-remplissant un SMS.",
    "qr-code-email": "QR Code ouvrant un email pré-rédigé.",
    "qr-code-phone": "QR Code pour appel téléphonique direct.",
    "qr-code-pdf": "QR Code donnant accès à un document PDF.",
    "qr-code-video": "QR Code redirigeant vers une vidéo.",
    "qr-code-audio": "QR Code donnant accès à un fichier audio.",
    "qr-code-location": "QR Code affichant une localisation sur une carte.",
    "qr-code-event": "QR Code ajoutant un événement au calendrier.",
    "qr-code-calendar": "QR Code pour événement calendrier.",
    "qr-code-text": "QR Code encodant du texte brut.",
    "qr-code-app": "QR Code redirigeant vers une application mobile.",
    "qr-code-social": "QR Code vers un profil de réseau social.",
    "qr-code-payment": "QR Code pour paiement mobile.",
    "qr-code-coupon": "QR Code offrant une réduction ou un bon d'achat.",
    "qr-code-menu": "QR Code affichant un menu de restaurant.",
    "qr-code-feedback": "QR Code pour recueillir des avis clients.",
    "qr-code-survey": "QR Code vers un formulaire de sondage.",
    "qr-code-bitcoin": "QR Code pour paiement en Bitcoin.",
    "qr-code-crypto": "QR Code pour transactions en cryptomonnaie.",
}

TOOL_DESCRIPTIONS = {
    "qr-code-generator": "Outil en ligne pour créer des QR Codes personnalisés.",
    "qr-code-scanner": "Application pour lire les QR Codes.",
    "qr-code-maker": "Créateur de QR Codes professionnel.",
    "barcode-generator": "Générateur de codes-barres 1D en ligne.",
    "link-shortener": "Outil de raccourcissement d'URL avec statistiques.",
    "page-builder": "Constructeur de pages de destination sans code.",
}

FEATURE_DESCRIPTIONS = {
    "bulk-qr-codes": "Génération de QR Codes en masse à partir d'un fichier.",
    "qr-code-api": "API REST pour intégrer la génération de QR Codes.",
    "white-label": "Solution en marque blanche personnalisable.",
    "custom-domain": "Domaine personnalisé pour vos liens courts.",
    "password-protection": "Protection par mot de passe des QR Codes.",
    "expiration-date": "Date d'expiration configurable pour vos QR Codes.",
    "scan-limit": "Limitation du nombre de scans autorisés.",
    "geo-targeting": "Redirection selon la localisation géographique.",
    "device-targeting": "Redirection selon le type d'appareil.",
    "a-b-testing": "Test A/B pour optimiser vos campagnes QR Code.",
    "retargeting": "Reciblage des visiteurs ayant scanné vos QR Codes.",
    "utm-tracking": "Paramètres UTM pour le suivi marketing.",
}

MEDIUM_DESCRIPTIONS = {
    "business-card-qr": "QR Code optimisé pour cartes de visite professionnelles.",
    "poster-qr": "QR Code grande taille pour affiches et panneaux.",
    "flyer-qr": "QR Code adapté aux flyers et dépliants.",
    "sticker-qr": "QR Code au format autocollant.",
    "table-qr": "QR Code pour table de restaurant ou événement.",
    "window-qr": "QR Code pour vitrine de magasin.",
    "product-qr": "QR Code pour étiquette produit.",
    "packaging-qr": "QR Code pour emballage.",
    "badge-qr": "QR Code pour badge d'identification.",
    "receipt-qr": "QR Code pour ticket de caisse.",
    "screen-qr": "QR Code pour affichage sur écran.",
    "banner-qr": "QR Code pour bannière publicitaire.",
    "brochure-qr": "QR Code pour brochure commerciale.",
    "catalog-qr": "QR Code pour catalogue produits.",
    "magazine-qr": "QR Code pour publicité magazine.",
    "newspaper-qr": "QR Code pour annonce presse.",
    "billboard-qr": "QR Code pour panneau d'affichage.",
    "vehicle-qr": "QR Code pour habillage véhicule.",
    "uniform-qr": "QR Code pour uniforme ou vêtement de travail.",
    "exhibition-qr": "QR Code pour stand d'exposition.",
}

INDUSTRY_DESCRIPTIONS = {
    "restaurants": "Solutions QR Code pour la restauration : menus, paiements, avis.",
    "retail": "QR Codes pour le commerce de détail : produits, promotions, fidélité.",
    "healthcare": "QR Codes pour la santé : informations patient, médicaments, rendez-vous.",
    "education": "QR Codes pour l'éducation : ressources, cours, examens.",
    "hospitality": "QR Codes pour l'hôtellerie : check-in, services, informations.",
    "real-estate": "QR Codes pour l'immobilier : visites virtuelles, annonces, contacts.",
    "manufacturing": "QR Codes pour l'industrie : traçabilité, maintenance, inventaire.",
    "logistics": "QR Codes pour la logistique : suivi colis, inventaire, livraison.",
    "marketing": "QR Codes pour le marketing : campagnes, analytics, engagement.",
    "events": "QR Codes pour événements : billetterie, accès, programme.",
    "tourism": "QR Codes pour le tourisme : attractions, guides, réservations.",
    "automotive": "QR Codes pour l'automobile : manuels, entretien, historique.",
    "fashion": "QR Codes pour la mode : authentification, lookbook, achats.",
    "food-beverage": "QR Codes pour l'agroalimentaire : traçabilité, recettes, allergènes.",
    "entertainment": "QR Codes pour le divertissement : billets, contenus exclusifs.",
    "sports": "QR Codes pour le sport : billets, statistiques, merchandising.",
    "beauty": "QR Codes pour la beauté : tutoriels, ingrédients, achats.",
    "fitness": "QR Codes pour le fitness : programmes, équipements, cours.",
    "finance": "QR Codes pour la finance : paiements, documents, authentification.",
    "nonprofit": "QR Codes pour associations : dons, informations, événements.",
    "government": "QR Codes pour services publics : formulaires, informations, accès.",
    "construction": "QR Codes pour le BTP : plans, sécurité, documentation.",
    "agriculture": "QR Codes pour l'agriculture : traçabilité, certifications, vente directe.",
    "energy": "QR Codes pour l'énergie : relevés, informations, maintenance.",
    "telecommunications": "QR Codes pour les télécoms : activation, support, produits.",
}

BRAND_DESCRIPTIONS = {
    "instagram": "QR Code vers votre profil Instagram.",
    "facebook": "QR Code vers votre page Facebook.",
    "whatsapp": "QR Code pour démarrer une conversation WhatsApp.",
    "linkedin": "QR Code vers votre profil LinkedIn.",
    "tiktok": "QR Code vers votre compte TikTok.",
    "youtube": "QR Code vers votre chaîne YouTube.",
    "twitter": "QR Code vers votre compte Twitter/X.",
    "spotify": "QR Code vers votre musique Spotify.",
    "snapchat": "QR Code vers votre profil Snapchat.",
    "pinterest": "QR Code vers vos tableaux Pinterest.",
    "telegram": "QR Code vers votre canal Telegram.",
    "discord": "QR Code vers votre serveur Discord.",
    "twitch": "QR Code vers votre chaîne Twitch.",
    "reddit": "QR Code vers votre profil Reddit.",
    "github": "QR Code vers votre profil GitHub.",
    "dribbble": "QR Code vers votre portfolio Dribbble.",
    "behance": "QR Code vers votre portfolio Behance.",
    "medium": "QR Code vers votre blog Medium.",
    "patreon": "QR Code vers votre page Patreon.",
    "paypal": "QR Code pour paiement PayPal.",
    "venmo": "QR Code pour paiement Venmo.",
    "cash-app": "QR Code pour paiement Cash App.",
    "google-maps": "QR Code vers une localisation Google Maps.",
    "apple-maps": "QR Code vers une localisation Apple Maps.",
    "waze": "QR Code vers une navigation Waze.",
}

INTEGRATION_DESCRIPTIONS = {
    "shopify": "Intégration Shopify pour QR Codes e-commerce.",
    "wordpress": "Plugin WordPress pour génération de QR Codes.",
    "zapier": "Automatisation Zapier pour workflows QR Code.",
    "hubspot": "Intégration HubSpot pour marketing automation.",
    "salesforce": "Connecteur Salesforce pour gestion de campagnes.",
    "mailchimp": "Intégration Mailchimp pour email marketing.",
    "google-analytics": "Suivi Google Analytics pour QR Codes.",
    "google-sheets": "Export vers Google Sheets automatisé.",
    "slack": "Notifications Slack pour scans QR Code.",
    "microsoft-teams": "Intégration Microsoft Teams.",
    "canva": "Création de QR Codes dans Canva.",
    "figma": "Plugin Figma pour design avec QR Codes.",
}

USE_CASE_DESCRIPTIONS = {
    "contactless-menu": "Menu sans contact pour restaurants et bars.",
    "digital-business-card": "Carte de visite numérique interactive.",
    "product-authentication": "Authentification et traçabilité des produits.",
    "event-registration": "Inscription et billetterie événementielle.",
    "asset-tracking": "Suivi et gestion des actifs de l'entreprise.",
    "employee-badge": "Badge d'identification employé numérique.",
    "customer-feedback": "Collecte d'avis clients simplifiée.",
    "loyalty-program": "Programme de fidélité client.",
    "lead-generation": "Génération de leads qualifiés.",
    "emergency-information": "Informations d'urgence accessibles rapidement.",
    "wifi-sharing": "Partage de connexion WiFi simplifié.",
    "document-sharing": "Partage de documents sécurisé.",
}

GUIDE_DESCRIPTIONS = {
    "how-to-create-qr-code": "Guide complet pour créer votre premier QR Code.",
    "how-to-scan-qr-code": "Comment scanner un QR Code avec votre smartphone.",
    "how-to-customize-qr-code": "Personnaliser l'apparence de vos QR Codes.",
    "how-to-track-qr-code": "Mesurer les performances de vos QR Codes.",
    "how-to-print-qr-code": "Bonnes pratiques pour l'impression des QR Codes.",
    "qr-code-best-practices": "Meilleures pratiques pour des QR Codes efficaces.",
    "qr-code-size-guide": "Guide des tailles optimales pour vos QR Codes.",
    "qr-code-design-tips": "Conseils de design pour QR Codes attractifs.",
    "dynamic-vs-static": "Choisir entre QR Code dynamique et statique.",
    "qr-code-security": "Sécuriser vos QR Codes contre la fraude.",
}

COMPARISON_DESCRIPTIONS = {
    "dynamic-vs-static-qr-code": "Comparatif entre QR Codes dynamiques et statiques.",
    "qr-code-vs-barcode": "Différences entre QR Codes et codes-barres traditionnels.",
    "qr-code-vs-nfc": "QR Code vs NFC : quelle technologie choisir ?",
    "free-vs-paid-qr-code": "QR Codes gratuits vs premium : que choisir ?",
}

# =============================================================================
# FRENCH TERMINOLOGY MAPPINGS
# =============================================================================

DISPLAY_NAME_TRANSLATIONS = {
    # Keep QR Code as-is (NOT "Code QR")
    "QR Code": "QR Code",
    "QR code": "QR Code",

    # Multi-word terms with QR Code - adjectives AFTER in French
    "Dynamic QR Code": "QR Code Dynamique",
    "Static QR Code": "QR Code Statique",
    "Custom QR Code": "QR Code Personnalisé",
    "WiFi QR Code": "QR Code WiFi",
    "URL QR Code": "QR Code URL",

    # Multi-word terms (order matters - longer first)
    "Business Card": "Carte de Visite",
    "Landing Page": "Page de Destination",
    "Smart Link": "Lien Intelligent",
    "Short Link": "Lien Court",
    "Link in Bio": "Lien en Bio",
    "Digital Menu": "Menu Digital",
    "App Store": "App Store",
    "Google Play": "Google Play",
    "Data Matrix": "Data Matrix",
    "Background Color": "Couleur de Fond",
    "Background Gradient": "Fond en Dégradé",
    "Background Image": "Image de Fond",
    "Transparent Background": "Fond Transparent",
    "with Logo": "avec Logo",
    "with Text": "avec Texte",
    "with Image": "avec Image",
    "How to": "Comment",
    "Best Practices": "Bonnes Pratiques",
    "Size Guide": "Guide des Tailles",
    "Design Tips": "Conseils Design",

    # Single words - Features
    "Generator": "Générateur",
    "Scanner": "Scanner",
    "Create": "Créer",
    "Scan": "Scanner",
    "Download": "Télécharger",
    "Print": "Imprimer",
    "Share": "Partager",
    "Edit": "Modifier",
    "Track": "Suivre",
    "Customize": "Personnaliser",
    "Dynamic": "Dynamique",
    "Static": "Statique",
    "Custom": "Personnalisé",
    "Colors": "Couleurs",
    "Color": "Couleur",
    "Shapes": "Formes",
    "Analytics": "Analytiques",
    "Tracking": "Suivi",
    "Design": "Design",
    "Guide": "Guide",
    "Free": "Gratuit",
    "Paid": "Premium",
    "Bulk": "En Masse",
    "API": "API",

    # Content types
    "URL": "URL",
    "WiFi": "WiFi",
    "vCard": "vCard",
    "SMS": "SMS",
    "Email": "Email",
    "Phone": "Téléphone",
    "PDF": "PDF",
    "Video": "Vidéo",
    "Audio": "Audio",
    "Location": "Localisation",
    "Event": "Événement",
    "Calendar": "Calendrier",
    "Text": "Texte",
    "App": "Application",
    "Social": "Réseaux Sociaux",
    "Payment": "Paiement",
    "Coupon": "Coupon",
    "Menu": "Menu",
    "Feedback": "Avis",
    "Survey": "Sondage",
    "Bitcoin": "Bitcoin",
    "Crypto": "Crypto",

    # Things
    "Barcode": "Code-barres",
    "Format": "Format",
    "Art": "Art",
    "Photo": "Photo",
    "Content": "Contenu",
    "Frame": "Cadre",
    "Style": "Style",
    "Type": "Type",
    "Template": "Modèle",
    "Background": "Arrière-plan",

    # Mediums
    "Poster": "Affiche",
    "Flyer": "Flyer",
    "Sticker": "Autocollant",
    "Table": "Table",
    "Window": "Vitrine",
    "Product": "Produit",
    "Packaging": "Emballage",
    "Badge": "Badge",
    "Receipt": "Ticket",
    "Screen": "Écran",
    "Banner": "Bannière",
    "Brochure": "Brochure",
    "Catalog": "Catalogue",
    "Magazine": "Magazine",
    "Newspaper": "Journal",
    "Billboard": "Panneau",
    "Vehicle": "Véhicule",
    "Uniform": "Uniforme",
    "Exhibition": "Exposition",

    # Industries
    "Retail": "Commerce de Détail",
    "Healthcare": "Santé",
    "Education": "Éducation",
    "Hospitality": "Hôtellerie",
    "Manufacturing": "Industrie",
    "Logistics": "Logistique",
    "Marketing": "Marketing",
    "Restaurants": "Restaurants",
    "Events": "Événements",
    "Real Estate": "Immobilier",
    "Tourism": "Tourisme",
    "Automotive": "Automobile",
    "Fashion": "Mode",
    "Food & Beverage": "Agroalimentaire",
    "Entertainment": "Divertissement",
    "Sports": "Sport",
    "Beauty": "Beauté",
    "Fitness": "Fitness",
    "Finance": "Finance",
    "Nonprofit": "Associations",
    "Government": "Services Publics",
    "Construction": "BTP",
    "Agriculture": "Agriculture",
    "Energy": "Énergie",
    "Telecommunications": "Télécoms",

    # Features
    "White Label": "Marque Blanche",
    "Custom Domain": "Domaine Personnalisé",
    "Password Protection": "Protection par Mot de Passe",
    "Expiration Date": "Date d'Expiration",
    "Scan Limit": "Limite de Scans",
    "Geo-Targeting": "Géo-Ciblage",
    "Device Targeting": "Ciblage par Appareil",
    "A/B Testing": "Test A/B",
    "Retargeting": "Reciblage",
    "UTM Tracking": "Suivi UTM",

    # Use cases
    "Contactless Menu": "Menu Sans Contact",
    "Digital Business Card": "Carte de Visite Numérique",
    "Product Authentication": "Authentification Produit",
    "Event Registration": "Inscription Événement",
    "Asset Tracking": "Suivi des Actifs",
    "Employee Badge": "Badge Employé",
    "Customer Feedback": "Avis Clients",
    "Loyalty Program": "Programme de Fidélité",
    "Lead Generation": "Génération de Leads",
    "Emergency Information": "Informations d'Urgence",
    "WiFi Sharing": "Partage WiFi",
    "Document Sharing": "Partage de Documents",

    # Comparisons
    "vs": "vs",
    "NFC": "NFC",

    # Keep brand names unchanged
    "Instagram": "Instagram",
    "Facebook": "Facebook",
    "WhatsApp": "WhatsApp",
    "LinkedIn": "LinkedIn",
    "TikTok": "TikTok",
    "YouTube": "YouTube",
    "Twitter": "Twitter",
    "Spotify": "Spotify",
    "Snapchat": "Snapchat",
    "Pinterest": "Pinterest",
    "Telegram": "Telegram",
    "Discord": "Discord",
    "Twitch": "Twitch",
    "Reddit": "Reddit",
    "GitHub": "GitHub",
    "Dribbble": "Dribbble",
    "Behance": "Behance",
    "Medium": "Medium",
    "Patreon": "Patreon",
    "PayPal": "PayPal",
    "Venmo": "Venmo",
    "Cash App": "Cash App",
    "Google Maps": "Google Maps",
    "Apple Maps": "Apple Maps",
    "Waze": "Waze",
    "Shopify": "Shopify",
    "WordPress": "WordPress",
    "Zapier": "Zapier",
    "HubSpot": "HubSpot",
    "Salesforce": "Salesforce",
    "Mailchimp": "Mailchimp",
    "Google Analytics": "Google Analytics",
    "Google Sheets": "Google Sheets",
    "Slack": "Slack",
    "Microsoft Teams": "Microsoft Teams",
    "Canva": "Canva",
    "Figma": "Figma",
    "EAN-13": "EAN-13",
    "EAN-8": "EAN-8",
    "UPC-A": "UPC-A",
    "UPC-E": "UPC-E",
    "Code 128": "Code 128",
    "Code 39": "Code 39",
    "ITF-14": "ITF-14",
}

# Slug translations for URL-safe French
SLUG_TRANSLATIONS = {
    "create-": "creer-",
    "scan-": "scanner-",
    "download-": "telecharger-",
    "print-": "imprimer-",
    "share-": "partager-",
    "edit-": "modifier-",
    "track-": "suivre-",
    "customize-": "personnaliser-",
    "how-to-": "comment-",
    "-guide": "-guide",
    "-vs-": "-vs-",
    "dynamic-": "dynamique-",
    "static-": "statique-",
    "custom-": "personnalise-",
    "business-card": "carte-visite",
    "landing-page": "page-destination",
    "smart-link": "lien-intelligent",
    "short-link": "lien-court",
    "link-in-bio": "lien-en-bio",
    "digital-menu": "menu-digital",
    "transparent-background": "fond-transparent",
    "background-color": "couleur-fond",
    "background-gradient": "fond-degrade",
    "background-image": "image-fond",
    "with-logo": "avec-logo",
    "with-text": "avec-texte",
    "with-image": "avec-image",
    "generator": "generateur",
    "scanner": "scanner",
    "colors": "couleurs",
    "shapes": "formes",
    "analytics": "analytiques",
    "tracking": "suivi",
    "content": "contenu",
    "frame": "cadre",
    "style": "style",
    "art": "art",
    "photo": "photo",
    "barcode": "code-barres",
    "format": "format",
    "best-practices": "bonnes-pratiques",
    "size-guide": "guide-tailles",
    "design-tips": "conseils-design",
    "white-label": "marque-blanche",
    "custom-domain": "domaine-personnalise",
    "password-protection": "protection-mot-de-passe",
    "expiration-date": "date-expiration",
    "scan-limit": "limite-scans",
    "geo-targeting": "geo-ciblage",
    "device-targeting": "ciblage-appareil",
    "a-b-testing": "test-ab",
    "retargeting": "reciblage",
    "utm-tracking": "suivi-utm",
    "bulk-": "masse-",
    "contactless-menu": "menu-sans-contact",
    "digital-business-card": "carte-visite-numerique",
    "product-authentication": "authentification-produit",
    "event-registration": "inscription-evenement",
    "asset-tracking": "suivi-actifs",
    "employee-badge": "badge-employe",
    "customer-feedback": "avis-clients",
    "loyalty-program": "programme-fidelite",
    "lead-generation": "generation-leads",
    "emergency-information": "informations-urgence",
    "wifi-sharing": "partage-wifi",
    "document-sharing": "partage-documents",
    # Industries
    "restaurants": "restaurants",
    "retail": "commerce-detail",
    "healthcare": "sante",
    "education": "education",
    "hospitality": "hotellerie",
    "real-estate": "immobilier",
    "manufacturing": "industrie",
    "logistics": "logistique",
    "marketing": "marketing",
    "events": "evenements",
    "tourism": "tourisme",
    "automotive": "automobile",
    "fashion": "mode",
    "food-beverage": "agroalimentaire",
    "entertainment": "divertissement",
    "sports": "sport",
    "beauty": "beaute",
    "fitness": "fitness",
    "finance": "finance",
    "nonprofit": "associations",
    "government": "services-publics",
    "construction": "btp",
    "agriculture": "agriculture",
    "energy": "energie",
    "telecommunications": "telecoms",
}

# =============================================================================
# HELPER FUNCTIONS
# =============================================================================

def translate_display_name(en_name: str) -> str:
    """Translate display name to French."""
    result = en_name
    # Sort by length (longest first) to avoid partial replacements
    for en, fr in sorted(DISPLAY_NAME_TRANSLATIONS.items(), key=lambda x: -len(x[0])):
        result = result.replace(en, fr)
    return result


def key_to_slug(key: str) -> str:
    """Convert entity key to French URL-safe slug."""
    slug = key.lower()
    # Apply slug translations (longer patterns first)
    for en, fr in sorted(SLUG_TRANSLATIONS.items(), key=lambda x: -len(x[0])):
        slug = slug.replace(en, fr)
    # Remove any non-URL-safe characters
    slug = re.sub(r'[^a-z0-9-]', '', slug)
    # Clean up multiple hyphens
    slug = re.sub(r'-+', '-', slug)
    return slug.strip('-')


def get_french_article(word: str, lowercase: bool = False) -> str:
    """Get the appropriate French definite article for a word."""
    word_lower = word.lower().strip()
    vowels = ('a', 'e', 'i', 'o', 'u', 'h')

    # Plural words
    if word_lower.endswith('s') and not word_lower.endswith(('ss', 'us', 'is', 'ès')):
        return "les " if lowercase else "Les "

    # Feminine words (simplified heuristics)
    feminine_endings = ('tion', 'sion', 'té', 'ure', 'ade', 'ude', 'ée', 'ie', 'lle', 'sse')
    feminine_words = ('page', 'couleur', 'image', 'carte', 'table', 'affiche', 'vitrine',
                      'localisation', 'vidéo', 'application', 'brochure', 'bannière', 'mode')

    is_feminine = (
        any(word_lower.endswith(end) for end in feminine_endings) or
        any(word_lower.startswith(fw) or fw in word_lower for fw in feminine_words)
    )

    if is_feminine:
        if word_lower.startswith(vowels):
            return "l'" if lowercase else "L'"
        return "la " if lowercase else "La "

    # Default masculine
    if word_lower.startswith(vowels):
        return "l'" if lowercase else "L'"
    return "le " if lowercase else "Le "


def get_description_from_dict(key: str, desc_dict: Dict[str, str], entity: Dict) -> str:
    """Get description from dictionary or generate a default."""
    if key in desc_dict:
        return desc_dict[key]
    # Fallback to translated English description
    fr_display = translate_display_name(entity["display_name"])
    return f"{get_french_article(fr_display)}{fr_display} pour vos projets QR Code."


def generate_llm_context_fr(en_context: str) -> str:
    """Generate French llm_context from English."""
    if not en_context:
        return ""

    result = en_context
    # Translate structure markers
    result = result.replace("USE:", "UTILISER:")
    result = result.replace("TRIGGERS:", "DECLENCHEURS:")
    result = result.replace("NOT:", "EXCLURE:")

    # Translate common terms in context
    context_translations = {
        "when discussing": "pour discuter de",
        "when user": "quand l'utilisateur",
        "redirect to": "rediriger vers",
        "use this for": "utiliser pour",
        "instead": "plutôt",
        "use": "utiliser",
    }
    for en, fr in context_translations.items():
        result = result.replace(en, fr)

    return result


# =============================================================================
# TYPE-SPECIFIC CONTENT GENERATORS
# =============================================================================

def generate_thing_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for THING entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, THING_DESCRIPTIONS, entity)
    definition = f"Solution QR Code AI pour la création et gestion de {fr_display}."
    purpose = f"Optimisez vos campagnes marketing avec {fr_display} personnalisés."

    return description, definition, purpose, fr_display


def generate_action_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for ACTION entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, ACTION_DESCRIPTIONS, entity)
    definition = f"Fonctionnalité QR Code AI permettant de {fr_display.lower().replace('créer', 'créer').replace('scanner', 'scanner').replace('télécharger', 'télécharger').replace('imprimer', 'imprimer').replace('partager', 'partager')} facilement."
    purpose = f"Simplifiez votre workflow en utilisant notre outil pour {fr_display.lower()}."

    return description, definition, purpose, fr_display


def generate_content_type_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for CONTENT_TYPE entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, CONTENT_TYPE_DESCRIPTIONS, entity)
    definition = f"Format de données {fr_display} compatible avec les QR Codes."
    purpose = f"Créez des QR Codes {fr_display} pour un accès instantané à vos contenus."

    return description, definition, purpose, fr_display


def generate_concept_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for CONCEPT entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, CONCEPT_DESCRIPTIONS, entity)
    definition = f"Concept clé pour comprendre et maîtriser les {fr_display}."
    purpose = f"Exploitez les avantages des {fr_display} pour vos campagnes marketing."

    return description, definition, purpose, fr_display


def generate_tool_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for TOOL entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, TOOL_DESCRIPTIONS, entity)
    definition = f"Outil professionnel {fr_display} intégré à QR Code AI."
    purpose = f"Utilisez notre {fr_display} pour créer des QR Codes professionnels."

    return description, definition, purpose, fr_display


def generate_feature_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for FEATURE entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, FEATURE_DESCRIPTIONS, entity)
    definition = f"Fonctionnalité avancée {fr_display} de QR Code AI."
    purpose = f"Améliorez vos QR Codes avec la fonctionnalité {fr_display}."

    return description, definition, purpose, fr_display


def generate_medium_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for MEDIUM entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, MEDIUM_DESCRIPTIONS, entity)
    definition = f"Support physique {fr_display} optimisé pour l'affichage de QR Codes."
    purpose = f"Intégrez vos QR Codes sur {fr_display} pour maximiser l'engagement."

    return description, definition, purpose, fr_display


def generate_industry_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for INDUSTRY entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, INDUSTRY_DESCRIPTIONS, entity)
    definition = f"Solutions QR Code spécialisées pour le secteur {fr_display}."
    purpose = f"Transformez votre activité {fr_display} avec des QR Codes sur mesure."

    return description, definition, purpose, fr_display


def generate_brand_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for BRAND entities."""
    key = entity["key"]
    fr_display = entity["display_name"]  # Keep brand names unchanged

    description = get_description_from_dict(key, BRAND_DESCRIPTIONS, entity)
    definition = f"Intégration QR Code pour la plateforme {fr_display}."
    purpose = f"Connecter votre audience à {fr_display} via un QR Code personnalisé."

    return description, definition, purpose, fr_display


def generate_integration_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for INTEGRATION entities."""
    key = entity["key"]
    fr_display = entity["display_name"]  # Keep integration names unchanged

    description = get_description_from_dict(key, INTEGRATION_DESCRIPTIONS, entity)
    definition = f"Connexion API entre QR Code AI et {fr_display}."
    purpose = f"Automatiser la création et la gestion de QR Codes avec {fr_display}."

    return description, definition, purpose, fr_display


def generate_use_case_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for USE_CASE entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, USE_CASE_DESCRIPTIONS, entity)
    definition = f"Cas d'usage professionnel {fr_display} avec QR Codes."
    purpose = f"Implémentez {fr_display} dans votre organisation grâce aux QR Codes."

    return description, definition, purpose, fr_display


def generate_guide_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for GUIDE entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, GUIDE_DESCRIPTIONS, entity)
    definition = f"Guide pratique et tutoriel : {fr_display}."
    purpose = f"Apprenez à maîtriser les QR Codes avec ce guide {fr_display}."

    return description, definition, purpose, fr_display


def generate_comparison_content(entity: Dict) -> Tuple[str, str, str, str]:
    """Generate French content for COMPARISON entities."""
    key = entity["key"]
    fr_display = translate_display_name(entity["display_name"])

    description = get_description_from_dict(key, COMPARISON_DESCRIPTIONS, entity)
    definition = f"Comparatif détaillé : {fr_display}."
    purpose = f"Faites le bon choix grâce à notre analyse {fr_display}."

    return description, definition, purpose, fr_display


# =============================================================================
# MAIN GENERATOR
# =============================================================================

TYPE_GENERATORS = {
    "THING": generate_thing_content,
    "ACTION": generate_action_content,
    "CONTENT_TYPE": generate_content_type_content,
    "CONCEPT": generate_concept_content,
    "TOOL": generate_tool_content,
    "FEATURE": generate_feature_content,
    "MEDIUM": generate_medium_content,
    "INDUSTRY": generate_industry_content,
    "BRAND": generate_brand_content,
    "INTEGRATION": generate_integration_content,
    "USE_CASE": generate_use_case_content,
    "GUIDE": generate_guide_content,
    "COMPARISON": generate_comparison_content,
}


def generate_entityl10n(entities: list) -> list:
    """Generate EntityL10n fr-FR for all entities."""
    entityl10n_list = []

    for entity in entities:
        entity_type = entity.get("type", "THING")
        generator = TYPE_GENERATORS.get(entity_type, generate_thing_content)

        # Generate type-specific content
        description, definition, purpose, fr_display = generator(entity)

        # Generate slug
        slug = key_to_slug(entity["key"])

        # Generate French llm_context
        llm_context = generate_llm_context_fr(entity.get("llm_context", ""))

        entityl10n = {
            "entity_key": entity["key"],
            "locale_key": "fr-FR",
            "slug": slug,
            "display_name": fr_display,
            "description": description,
            "definition": definition,
            "purpose": purpose,
            "llm_context": llm_context,
            "version": 1
        }
        entityl10n_list.append(entityl10n)

    return entityl10n_list


def main():
    """Main entry point."""
    # Load entities
    input_path = Path("scripts/seo-import/output/entities.json")
    with open(input_path, "r", encoding="utf-8") as f:
        entities = json.load(f)

    print(f"Loaded {len(entities)} entities from {input_path}")

    # Generate French versions
    entityl10n_list = generate_entityl10n(entities)

    # Save output
    output_path = Path("scripts/seo-import/output/entityl10n_fr.json")
    output_path.parent.mkdir(parents=True, exist_ok=True)

    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(entityl10n_list, f, indent=2, ensure_ascii=False)

    print(f"Generated {len(entityl10n_list)} EntityL10n fr-FR to {output_path}")

    # Print sample output
    print("\n" + "="*70)
    print("SAMPLE OUTPUT (first 5 entities):")
    print("="*70)
    for el in entityl10n_list[:5]:
        print(f"\n--- {el['entity_key']} ---")
        print(f"  display_name: {el['display_name']}")
        print(f"  slug: {el['slug']}")
        print(f"  description: {el['description']}")
        print(f"  definition: {el['definition']}")
        print(f"  purpose: {el['purpose']}")

    # Print stats by type
    print("\n" + "="*70)
    print("STATS BY TYPE:")
    print("="*70)
    from collections import Counter
    type_counts = Counter()
    for entity in entities:
        type_counts[entity.get("type", "THING")] += 1
    for t, count in sorted(type_counts.items()):
        print(f"  {t}: {count}")


if __name__ == "__main__":
    main()
