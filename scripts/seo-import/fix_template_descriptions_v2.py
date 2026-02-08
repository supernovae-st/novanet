#!/usr/bin/env python3
"""Fix template descriptions in EntityContent fr-FR seed file (v2).

Finds entity_key patterns and replaces their description field.
"""

import re
from pathlib import Path

# Entity key -> proper French description
DESCRIPTIONS = {
    # Visual modes
    "qr-code-light-mode": "Mode clair avec QR Code sombre sur fond blanc, idéal pour l'impression.",
    "qr-code-dark-mode": "Mode sombre avec QR Code clair sur fond noir, parfait pour écrans.",
    
    # Print placements
    "qr-code-business-card": "QR Code optimisé pour carte de visite professionnelle.",
    "qr-code-email-signature": "QR Code compact pour signature email.",
    "qr-code-flyer": "QR Code haute résolution pour flyers et dépliants.",
    "qr-code-poster": "QR Code grand format pour affiches publicitaires.",
    "qr-code-table-tent": "QR Code pour chevalet de table restaurant.",
    "qr-code-packaging-label": "QR Code pour étiquettes et emballages produits.",
    "qr-code-sticker": "QR Code adhésif polyvalent.",
    "qr-code-banner": "QR Code pour bannières événementielles.",
    "qr-code-brochure": "QR Code pour brochures et catalogues.",
    "qr-code-window-decal": "QR Code pour vitrine commerciale.",
    "qr-code-door-hanger": "QR Code pour accroche-porte hôtel.",
    "qr-code-vehicle-wrap": "QR Code pour covering véhicule.",
    "qr-code-receipt": "QR Code pour tickets et reçus.",
    "qr-code-product-insert": "QR Code pour notices et inserts produit.",
    
    # Landing page types
    "link-in-bio": "Page bio centralisée regroupant tous vos liens.",
    "menu-restaurant": "Menu digital interactif avec photos et prix.",
    "forms": "Formulaire en ligne personnalisable.",
    "announcement": "Page d'annonce pour actualités et événements.",
    "event-rsvp": "Page d'inscription événement avec confirmations.",
    "booking-appointment": "Système de réservation en ligne.",
    "feedback-survey": "Questionnaire de satisfaction client.",
    "coupon-offer": "Page promotionnelle avec code promo.",
    "download-file": "Page de téléchargement de fichiers.",
    "vcard-contact": "Fiche contact numérique vCard.",
    
    # Social platforms
    "qr-code-mecard": "QR Code MeCard pour partage de coordonnées.",
    "qr-code-image-gallery": "Galerie d'images accessible par QR Code.",
    "qr-code-social": "Hub réseaux sociaux en un QR Code.",
    "qr-code-instagram": "Lien direct vers profil Instagram.",
    "qr-code-linkedin": "Accès rapide à profil LinkedIn.",
    "qr-code-facebook": "Lien vers page Facebook.",
    "qr-code-twitter": "Connexion directe à compte Twitter/X.",
    "qr-code-youtube": "Lien vers chaîne YouTube.",
    "qr-code-tiktok": "Accès à profil TikTok.",
    "qr-code-snapchat": "Snapcode pour ajout Snapchat.",
    "qr-code-whatsapp": "Lien WhatsApp avec message pré-rempli.",
    "qr-code-telegram": "Accès direct à canal Telegram.",
    "qr-code-pinterest": "Lien vers tableaux Pinterest.",
    "qr-code-spotify": "Accès à profil ou playlist Spotify.",
    "qr-code-apple-music": "Lien vers musique Apple Music.",
    "qr-code-soundcloud": "Accès à morceaux SoundCloud.",
    
    # Payment
    "qr-code-payment": "QR Code de paiement sans contact.",
    "qr-code-pix": "Paiement instantané via PIX brésilien.",
    "qr-code-upi": "Paiement mobile UPI pour l'Inde.",
    "qr-code-paypal": "Paiement sécurisé via PayPal.",
    "qr-code-venmo": "Transfert d'argent Venmo.",
    "qr-code-bitcoin": "Adresse Bitcoin pour paiements crypto.",
    "qr-code-ethereum": "Adresse Ethereum pour transactions blockchain.",
    "qr-code-bank-transfer": "Coordonnées bancaires pour virement.",
    
    # Location
    "qr-code-location": "Coordonnées GPS pour localisation.",
    "qr-code-google-maps": "Itinéraire Google Maps.",
    "qr-code-apple-maps": "Navigation Apple Maps.",
    "qr-code-waze": "Guidage Waze avec trafic temps réel.",
    
    # App downloads
    "qr-code-app-download": "Lien de téléchargement d'application mobile.",
    "qr-code-app-store": "Lien direct vers App Store iOS.",
    "qr-code-google-play": "Lien direct vers Google Play Android.",
    "qr-code-amazon-app-store": "Lien vers Amazon Appstore.",
    
    # Event & check-in
    "qr-code-event": "QR Code événementiel pour billets et accès.",
    "qr-code-attendance": "QR Code de présence pour émargement.",
    "qr-code-check-in": "QR Code d'enregistrement pour accueil.",
    
    # Barcodes
    "ean-13": "Code-barres EAN-13 européen pour produits.",
    "ean-8": "Code-barres EAN-8 compact.",
    "upc-a": "Code-barres UPC-A nord-américain.",
    "upc-e": "Code-barres UPC-E condensé.",
    "code-128": "Code-barres Code 128 haute densité.",
    "code-39": "Code-barres Code 39 alphanumérique.",
    "code-93": "Code-barres Code 93 compact.",
    "codabar": "Code-barres Codabar pour bibliothèques.",
    "itf-14": "Code-barres ITF-14 pour cartons.",
    "gs1-128": "Code-barres GS1-128 avec données avancées.",
    "data-matrix": "Code 2D Data Matrix compact.",
    "aztec-code": "Code 2D Aztec pour billets transport.",
    "pdf417": "Code 2D PDF417 haute capacité.",
    "maxicode": "Code 2D MaxiCode pour tri postal.",
    "qr-code-micro": "Micro QR Code pour espaces restreints.",
    
    # Features & tools
    "qr-code-analytics": "Tableau de bord analytique des scans.",
    "scan-counting": "Compteur de scans en temps réel.",
    "device-detection": "Détection automatique du type d'appareil.",
    "location-tracking": "Géolocalisation des scans.",
    "time-based-redirect": "Redirection programmée selon l'heure.",
    "password-protection": "Protection par mot de passe.",
    "expiration-date": "Date d'expiration automatique.",
    "utm-builder": "Générateur de paramètres UTM marketing.",
    "url-shortener": "Raccourcisseur d'URL avec statistiques.",
    "bulk-creation": "Création en masse de QR Codes.",
    "api-access": "Accès API pour intégration.",
    "qr-code-api": "API REST pour génération de QR Codes.",
    "white-label": "Solution marque blanche personnalisable.",
    "team-collaboration": "Espace de travail collaboratif.",
    "folder-organization": "Organisation par dossiers.",
    "custom-domain": "Domaine personnalisé pour URLs.",
    "custom-link-preview": "Aperçu de lien personnalisable.",
    "retargeting-pixels": "Pixels de retargeting publicitaire.",
    "landing-page-builder": "Constructeur de pages de destination.",
    "menu-builder": "Constructeur de menus digitaux.",
    "vcard-generator": "Générateur de cartes vCard.",
    "wifi-qr-generator": "Générateur de QR Codes WiFi.",
    
    # Industries
    "restaurants": "Solutions QR Code pour restaurants.",
    "retail": "QR Codes pour commerce de détail.",
    "healthcare": "QR Codes pour le secteur santé.",
    "education": "QR Codes pour l'éducation.",
    "hospitality": "QR Codes pour l'hôtellerie.",
    "real-estate": "QR Codes pour l'immobilier.",
    "events": "QR Codes pour l'événementiel.",
    "manufacturing": "QR Codes pour l'industrie.",
    "logistics": "QR Codes pour la logistique.",
    "marketing-agencies": "Solutions pour agences marketing.",
    "nonprofits": "QR Codes pour associations.",
    "government": "QR Codes pour secteur public.",
    "transportation": "QR Codes pour les transports.",
    "entertainment": "QR Codes pour le divertissement.",
    "sports": "QR Codes pour le sport.",
    "beauty": "QR Codes pour la beauté.",
    "fitness": "QR Codes pour le fitness.",
    "automotive": "QR Codes pour l'automobile.",
    "food-beverage": "QR Codes pour l'alimentaire.",
    "finance": "QR Codes pour la finance.",
    
    # Mediums
    "catalogs": "QR Codes pour catalogues.",
    "magazines": "QR Codes pour magazines.",
    "newspapers": "QR Codes pour presse écrite.",
    "direct-mail": "QR Codes pour publipostage.",
    "packaging": "QR Codes pour emballages.",
    "signage": "QR Codes pour signalétique.",
    "outdoor-advertising": "QR Codes pour affichage extérieur.",
    "trade-shows": "QR Codes pour salons professionnels.",
    "billboards": "QR Codes pour panneaux publicitaires.",
    "transit-advertising": "QR Codes pour publicité transport.",
    
    # Actions
    "create-qr-code": "Créer un QR Code personnalisé en quelques clics.",
    "scan-qr-code": "Scanner un QR Code avec votre smartphone.",
    "download-qr-code": "Télécharger votre QR Code en haute résolution.",
    "print-qr-code": "Imprimer votre QR Code avec les bonnes dimensions.",
    "share-qr-code": "Partager votre QR Code sur réseaux sociaux.",
    "track-qr-code": "Suivre les performances de votre QR Code.",
    "edit-qr-code": "Modifier la destination de votre QR Code dynamique.",
    "design-qr-code": "Personnaliser le design de votre QR Code.",
    
    # Guides
    "qr-code-size-guide": "Guide des tailles optimales de QR Code.",
    "qr-code-design-guide": "Bonnes pratiques de design QR Code.",
    "qr-code-print-guide": "Conseils d'impression pour QR Codes.",
    "qr-code-marketing-guide": "Stratégies marketing avec QR Codes.",
    "dynamic-vs-static": "Comparaison QR Code dynamique vs statique.",
    "qr-code-vs-barcode": "Différences entre QR Code et code-barres.",
    "short-link-vs-qr-code": "Lien court vs QR Code : que choisir ?",
    "free-vs-paid": "QR Code gratuit vs payant : comparatif.",
    
    # Creative
    "qr-code-tattoo": "QR Code tatouage : art et technologie.",
    "qr-code-art": "QR Code artistique créatif.",
    "funny-qr-codes": "QR Codes créatifs et humoristiques.",
    "qr-code-wedding": "QR Codes pour mariages.",
    "qr-code-memorial": "QR Codes mémoriaux.",
    "qr-code-games": "QR Codes pour jeux et chasses au trésor.",
    "qr-code-puzzles": "Puzzles et énigmes avec QR Codes.",
}

def fix_file(path: Path) -> int:
    """Fix template descriptions in seed file."""
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    
    fixed = 0
    for entity_key, desc in DESCRIPTIONS.items():
        # Pattern: el.entity_key = 'xxx', ... el.description = 'La/Le/L' Y pour vos projets QR Code.'
        # We need to find the description line that follows the entity_key line
        
        # Find blocks for this entity_key - allow escaped quotes in description
        # Pattern: match escaped quotes (\') and non-quotes, ending with template pattern
        pattern = rf"(entity_key = '{re.escape(entity_key)}',.*?el\.description = ')([^']*\\'[^']*|[^']*)pour vos projets QR Code\\.'"
        
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
