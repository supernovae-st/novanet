#!/usr/bin/env python3
"""Generate EntityNative fr-FR content from Entity definitions.

This script creates French localized content for each Entity.
For production, this would use an LLM API. For now, uses rule-based translation.
"""

import json
import re
from pathlib import Path

# French translations for common terms
TRANSLATIONS = {
    "QR Code": "QR Code",
    "QR code": "QR Code",
    "qr code": "QR Code",
    "Create": "Créer",
    "create": "créer",
    "Scan": "Scanner",
    "scan": "scanner",
    "Generator": "Générateur",
    "generator": "générateur",
    "Scanner": "Scanner",
    "Dynamic": "Dynamique",
    "dynamic": "dynamique",
    "Static": "Statique",
    "static": "statique",
    "Custom": "Personnalisé",
    "custom": "personnalisé",
    "with Logo": "avec Logo",
    "with Text": "avec Texte",
    "with Image": "avec Image",
    "Background": "Arrière-plan",
    "Color": "Couleur",
    "Colors": "Couleurs",
    "Shapes": "Formes",
    "Transparent": "Transparent",
    "Business Card": "Carte de Visite",
    "Landing Page": "Page d'Atterrissage",
    "Smart Link": "Lien Intelligent",
    "Short Link": "Lien Court",
    "Barcode": "Code-barres",
    "WiFi": "WiFi",
    "vCard": "vCard",
    "URL": "URL",
    "SMS": "SMS",
    "Email": "Email",
    "PDF": "PDF",
    "Menu": "Menu",
    "Restaurant": "Restaurant",
    "Analytics": "Analytiques",
    "Tracking": "Suivi",
    "Download": "Télécharger",
    "Print": "Imprimer",
    "Share": "Partager",
    "Design": "Design",
    "Guide": "Guide",
    "How to": "Comment",
    "vs": "vs",
    "Free": "Gratuit",
    "Paid": "Payant",
}

def translate_display_name(en_name: str) -> str:
    """Translate display name to French."""
    result = en_name
    for en, fr in sorted(TRANSLATIONS.items(), key=lambda x: -len(x[0])):
        result = result.replace(en, fr)
    return result

def key_to_slug(key: str) -> str:
    """Convert entity key to French slug."""
    # Replace common English terms with French
    slug = key
    slug_replacements = {
        "create-": "creer-",
        "scan-": "scanner-",
        "download-": "telecharger-",
        "print-": "imprimer-",
        "share-": "partager-",
        "how-to-": "comment-",
        "-guide": "-guide",
        "-vs-": "-vs-",
        "dynamic-": "dynamique-",
        "static-": "statique-",
        "custom-": "personnalise-",
        "business-card": "carte-visite",
        "landing-page": "page-atterrissage",
        "smart-link": "lien-intelligent",
        "short-link": "lien-court",
    }
    for en, fr in slug_replacements.items():
        slug = slug.replace(en, fr)
    return slug

def generate_french_description(en_desc: str, entity_type: str) -> str:
    """Generate French description."""
    # For now, keep English with a note - in production would use LLM
    return en_desc  # TODO: LLM translation

def generate_french_llm_context(en_context: str) -> str:
    """Generate French llm_context."""
    # Keep structure, translate keywords
    result = en_context
    result = result.replace("USE:", "UTILISER:")
    result = result.replace("TRIGGERS:", "DECLENCHEURS:")
    result = result.replace("NOT:", "PAS:")
    return result

def generate_entitycontent(entities: list) -> list:
    """Generate EntityNative fr-FR for all entities."""
    entitycontent_list = []

    for entity in entities:
        entitycontent = {
            "entity_key": entity["key"],
            "locale_key": "fr-FR",
            "slug": key_to_slug(entity["key"]),
            "display_name": translate_display_name(entity["display_name"]),
            "description": generate_french_description(entity["description"], entity["type"]),
            "llm_context": generate_french_llm_context(entity.get("llm_context", "")),
            "version": 1
        }
        entitycontent_list.append(entitycontent)

    return entitycontent_list

if __name__ == "__main__":
    # Load entities
    with open("scripts/seo-import/output/entities.json", "r") as f:
        entities = json.load(f)

    # Generate French versions
    entitycontent_list = generate_entitycontent(entities)

    # Save output
    output_path = Path("scripts/seo-import/output/entitycontent_fr.json")
    with open(output_path, "w") as f:
        json.dump(entitycontent_list, f, indent=2, ensure_ascii=False)

    print(f"Generated {len(entitycontent_list)} EntityNative fr-FR to {output_path}")
