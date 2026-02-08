#!/usr/bin/env python3
"""Fix template descriptions in EntityContent fr-FR seed file.

Replaces lazy "La X pour vos projets QR Code" with proper French descriptions.
"""

import re
from pathlib import Path

# Category-based description templates
DESCRIPTIONS = {
    # Visual modes
    "qr-code-light-mode": "Mode d'affichage clair avec QR Code sombre sur fond blanc, idéal pour l'impression et la lisibilité optimale.",
    "qr-code-dark-mode": "Mode d'affichage sombre avec QR Code clair sur fond noir, parfait pour les designs modernes et les écrans.",
    
    # Print placements
    "qr-code-business-card": "QR Code optimisé pour carte de visite, permettant de partager instantanément vos coordonnées professionnelles.",
    "qr-code-email-signature": "QR Code compact pour signature email, offrant un accès rapide à votre profil ou site web.",
    "qr-code-flyer": "QR Code haute résolution pour flyers et dépliants, connectant le print au digital.",
    "qr-code-poster": "QR Code grand format pour affiches, scannable à distance pour campagnes publicitaires.",
    "qr-code-table-tent": "QR Code pour chevalet de table, idéal pour restaurants et événements.",
    "qr-code-packaging-label": "QR Code pour étiquettes produit, donnant accès aux informations détaillées.",
    "qr-code-sticker": "QR Code adhésif pour surfaces diverses, solution flexible et polyvalente.",
    "qr-code-banner": "QR Code pour bannières et kakémonos, visible de loin lors d'événements.",
    "qr-code-brochure": "QR Code pour brochures et catalogues, enrichissant la documentation imprimée.",
    "qr-code-window-decal": "QR Code pour vitrine, attirant les passants vers votre contenu digital.",
    "qr-code-door-hanger": "QR Code pour accroche-porte, parfait pour hôtels et promotions locales.",
    "qr-code-vehicle-wrap": "QR Code pour covering véhicule, transformant votre flotte en support publicitaire.",
    "qr-code-receipt": "QR Code pour tickets et reçus, facilitant les retours d'expérience et fidélisation.",
    "qr-code-product-insert": "QR Code pour notice produit, donnant accès aux guides et tutoriels.",
    
    # Landing page types
    "link-in-bio": "Page bio centralisée regroupant tous vos liens essentiels pour réseaux sociaux.",
    "menu-restaurant": "Menu digital interactif avec photos, descriptions et prix actualisables.",
    "forms": "Formulaire en ligne personnalisable pour collecter données et retours clients.",
    "announcement": "Page d'annonce pour communiquer actualités et événements importants.",
    "event-rsvp": "Page d'inscription événement avec gestion des confirmations de présence.",
    "booking-appointment": "Système de réservation en ligne pour rendez-vous et créneaux horaires.",
    "feedback-survey": "Questionnaire de satisfaction pour recueillir les avis clients.",
    "coupon-offer": "Page promotionnelle avec code promo et offres spéciales.",
    "download-file": "Page de téléchargement sécurisé pour documents et fichiers.",
    "vcard-contact": "Fiche contact numérique au format vCard pour carnet d'adresses.",
    
    # Content types - Social
    "qr-code-mecard": "QR Code MeCard pour partager vos coordonnées compatibles avec tous les lecteurs.",
    "qr-code-image-gallery": "Galerie d'images accessible par QR Code pour portfolios et catalogues visuels.",
    "qr-code-social": "Hub réseaux sociaux regroupant tous vos profils en un seul QR Code.",
    "qr-code-instagram": "Lien direct vers votre profil Instagram pour augmenter vos abonnés.",
    "qr-code-linkedin": "Accès rapide à votre profil LinkedIn pour networking professionnel.",
    "qr-code-facebook": "Lien vers votre page Facebook pour développer votre communauté.",
    "qr-code-twitter": "Connexion directe à votre compte Twitter/X pour engagement instantané.",
    "qr-code-youtube": "Lien vers votre chaîne YouTube pour maximiser vos vues.",
    "qr-code-tiktok": "Accès à votre profil TikTok pour toucher la génération Z.",
    "qr-code-snapchat": "Snapcode pour ajouter facilement vos contacts Snapchat.",
    "qr-code-whatsapp": "Lien WhatsApp avec message pré-rempli pour contact instantané.",
    "qr-code-telegram": "Accès direct à votre canal ou groupe Telegram.",
    "qr-code-pinterest": "Lien vers vos tableaux Pinterest pour inspiration visuelle.",
    "qr-code-spotify": "Accès à votre profil ou playlist Spotify pour partage musical.",
    "qr-code-apple-music": "Lien vers votre musique sur Apple Music.",
    "qr-code-soundcloud": "Accès à vos morceaux SoundCloud pour artistes indépendants.",
    
    # Payment
    "qr-code-payment": "QR Code de paiement pour transactions sécurisées et sans contact.",
    "qr-code-pix": "Paiement instantané via PIX, système bancaire brésilien.",
    "qr-code-upi": "Paiement mobile UPI pour le marché indien.",
    "qr-code-paypal": "Paiement sécurisé via PayPal avec QR Code.",
    "qr-code-venmo": "Transfert d'argent Venmo simplifié par QR Code.",
    "qr-code-bitcoin": "Adresse Bitcoin pour recevoir des paiements crypto.",
    "qr-code-ethereum": "Adresse Ethereum pour transactions blockchain.",
    "qr-code-bank-transfer": "Coordonnées bancaires pour virement SEPA ou international.",
    
    # Location
    "qr-code-location": "Coordonnées GPS pour localiser facilement un lieu.",
    "qr-code-google-maps": "Itinéraire Google Maps vers votre adresse.",
    "qr-code-apple-maps": "Navigation Apple Maps pour utilisateurs iOS.",
    "qr-code-waze": "Guidage Waze avec trafic en temps réel.",
    
    # Barcode types
    "ean-13": "Code-barres EAN-13 standard européen pour produits de grande distribution.",
    "ean-8": "Code-barres EAN-8 compact pour petits produits.",
    "upc-a": "Code-barres UPC-A standard nord-américain pour commerce de détail.",
    "upc-e": "Code-barres UPC-E condensé pour petits emballages.",
    "code-128": "Code-barres Code 128 haute densité pour logistique et transport.",
    "code-39": "Code-barres Code 39 alphanumérique pour applications industrielles.",
    "code-93": "Code-barres Code 93 compact pour identification interne.",
    "codabar": "Code-barres Codabar pour bibliothèques et banques de sang.",
    "itf-14": "Code-barres ITF-14 pour cartons et palettes en logistique.",
    "gs1-128": "Code-barres GS1-128 avec données avancées pour chaîne d'approvisionnement.",
    "data-matrix": "Code 2D Data Matrix compact pour marquage industriel et santé.",
    "aztec-code": "Code 2D Aztec pour billets de transport et cartes d'embarquement.",
    "pdf417": "Code 2D PDF417 haute capacité pour documents d'identité.",
    "maxicode": "Code 2D MaxiCode pour tri postal automatisé.",
    "qr-code-micro": "Micro QR Code pour espaces très restreints.",
    
    # Features
    "qr-code-analytics": "Tableau de bord analytique avec statistiques de scans en temps réel.",
    "scan-counting": "Compteur de scans précis pour mesurer l'engagement.",
    "device-detection": "Détection automatique du type d'appareil pour redirection adaptée.",
    "location-tracking": "Géolocalisation des scans pour analyse géographique.",
    "time-based-redirect": "Redirection programmée selon l'heure pour campagnes ciblées.",
    "password-protection": "Protection par mot de passe pour contenu confidentiel.",
    "expiration-date": "Date d'expiration automatique pour offres limitées.",
    "utm-builder": "Générateur de paramètres UTM pour tracking marketing.",
    "url-shortener": "Raccourcisseur d'URL avec statistiques de clics.",
    "bulk-creation": "Création en masse de QR Codes à partir d'un fichier.",
    "api-access": "Accès API pour intégration dans vos applications.",
    "white-label": "Solution marque blanche personnalisable.",
    "team-collaboration": "Espace de travail collaboratif pour équipes.",
    "folder-organization": "Organisation par dossiers pour gérer vos QR Codes.",
    "custom-domain": "Domaine personnalisé pour URLs de marque.",
    "retargeting-pixels": "Pixels de retargeting pour campagnes publicitaires.",
    
    # Industries
    "restaurants": "Solutions QR Code pour restaurants : menus, réservations, avis clients.",
    "retail": "QR Codes pour commerce de détail : produits, promotions, fidélisation.",
    "healthcare": "QR Codes santé : dossiers patients, ordonnances, informations médicales.",
    "education": "QR Codes éducation : supports de cours, exercices interactifs, inscriptions.",
    "hospitality": "QR Codes hôtellerie : check-in, services, informations touristiques.",
    "real-estate": "QR Codes immobilier : visites virtuelles, fiches biens, contact agents.",
    "events": "QR Codes événementiel : billets, programmes, networking.",
    "manufacturing": "QR Codes industrie : traçabilité, maintenance, documentation technique.",
    "logistics": "QR Codes logistique : suivi colis, inventaire, expédition.",
    "marketing-agencies": "Solutions QR Code pour agences : campagnes clients, reporting, analytics.",
    "nonprofits": "QR Codes associations : dons, bénévolat, sensibilisation.",
    "government": "QR Codes secteur public : services citoyens, documents officiels.",
    "transportation": "QR Codes transport : billets, horaires, informations voyageurs.",
    "entertainment": "QR Codes divertissement : billetterie, contenus exclusifs, expériences.",
    "sports": "QR Codes sport : billets, merchandising, fan engagement.",
    "beauty": "QR Codes beauté : tutoriels produits, rendez-vous, avis.",
    "fitness": "QR Codes fitness : programmes, réservations cours, suivi performance.",
    "automotive": "QR Codes automobile : documentation véhicule, entretien, rappels.",
    "food-beverage": "QR Codes alimentaire : traçabilité, recettes, allergènes.",
    "finance": "QR Codes finance : paiements, authentification, documents.",
    
    # Mediums
    "catalogs": "QR Codes catalogues : accès produits, commandes, informations détaillées.",
    "magazines": "QR Codes magazines : contenus enrichis, abonnements, publicités interactives.",
    "newspapers": "QR Codes presse : articles complets, archives, abonnements.",
    "direct-mail": "QR Codes publipostage : offres personnalisées, tracking campagnes.",
    "packaging": "QR Codes emballage : authenticité, origine, mode d'emploi.",
    "signage": "QR Codes signalétique : informations lieu, navigation, services.",
    "outdoor-advertising": "QR Codes affichage extérieur : campagnes urbaines, événements.",
    "trade-shows": "QR Codes salons professionnels : stands, networking, documentation.",
    "billboards": "QR Codes panneaux publicitaires : campagnes grand format.",
    "transit-advertising": "QR Codes publicité transport : métro, bus, gares.",
    
    # Actions/Use Cases
    "create-qr-code": "Créer un QR Code personnalisé en quelques clics avec notre générateur gratuit.",
    "scan-qr-code": "Scanner un QR Code avec votre smartphone pour accéder instantanément au contenu.",
    "download-qr-code": "Télécharger votre QR Code en haute résolution (PNG, SVG, PDF, EPS).",
    "print-qr-code": "Imprimer votre QR Code avec les recommandations de taille et résolution.",
    "share-qr-code": "Partager votre QR Code sur réseaux sociaux, email ou messagerie.",
    "track-qr-code": "Suivre les performances de votre QR Code avec analytics détaillés.",
    "edit-qr-code": "Modifier la destination de votre QR Code dynamique sans le réimprimer.",
    "design-qr-code": "Personnaliser le design de votre QR Code : couleurs, formes, logo.",
    
    # Guides & Comparisons
    "qr-code-size-guide": "Guide des tailles optimales de QR Code selon le support d'impression.",
    "qr-code-design-guide": "Bonnes pratiques de design pour QR Codes lisibles et esthétiques.",
    "qr-code-print-guide": "Conseils d'impression pour QR Codes haute qualité.",
    "qr-code-marketing-guide": "Stratégies marketing avec QR Codes pour maximiser l'engagement.",
    "dynamic-vs-static": "Comparaison QR Code dynamique vs statique : avantages et cas d'usage.",
    "qr-code-vs-barcode": "Différences entre QR Code et code-barres : capacité, usage, lisibilité.",
    "short-link-vs-qr-code": "Quand utiliser un lien court vs un QR Code pour vos campagnes.",
    "free-vs-paid": "QR Code gratuit vs payant : fonctionnalités et limites comparées.",
    
    # Fun/Creative
    "qr-code-tattoo": "QR Code tatouage : considérations artistiques et techniques.",
    "qr-code-art": "QR Code artistique : quand technologie rencontre créativité.",
    "funny-qr-codes": "QR Codes créatifs et humoristiques pour campagnes virales.",
    "qr-code-wedding": "QR Codes mariage : invitations, RSVP, galerie photos.",
    "qr-code-memorial": "QR Codes mémoriaux : hommages et souvenirs numériques.",
    "qr-code-games": "QR Codes jeux : chasses au trésor, escape games, gamification.",
    "qr-code-puzzles": "Puzzles et énigmes avec QR Codes pour engagement ludique.",
}

def fix_descriptions(seed_path: Path) -> tuple:
    """Fix template descriptions in seed file."""
    with open(seed_path, "r", encoding="utf-8") as f:
        content = f.read()
    
    original = content
    fixed_count = 0
    
    for entity_key, description in DESCRIPTIONS.items():
        # Find pattern: el.description = 'La/Le/L' X pour vos projets QR Code.'
        pattern = rf"(MERGE \(el:EntityContent \{{key: 'entity:{entity_key}@fr-FR'\}}\).*?el\.description = ')[^']*pour vos projets QR Code\.'"
        
        replacement = rf"\1{description}'"
        new_content, count = re.subn(pattern, replacement, content, flags=re.DOTALL)
        if count > 0:
            content = new_content
            fixed_count += count
    
    if fixed_count > 0:
        with open(seed_path, "w", encoding="utf-8") as f:
            f.write(content)
    
    return fixed_count, len(DESCRIPTIONS)

if __name__ == "__main__":
    seed_path = Path("packages/db/seed/11-entity-content-fr-fr.cypher")
    fixed, total = fix_descriptions(seed_path)
    print(f"Fixed {fixed} descriptions out of {total} defined")
