#!/usr/bin/env node
/**
 * Extend EntityNatives to fr-FR and hi-IN
 * Task D: Add 2 more top market locales (total 10)
 */

const fs = require('fs');
const path = require('path');

const OUTPUT_FILE = path.join(__dirname, '..', 'seed', '26.1-entity-natives-fr-hi.cypher');

const LOCALES_TO_ADD = ['fr-FR', 'hi-IN'];

const ENTITIES = {
  'qr-code': {
    'fr-FR': { text: 'code qr', title: 'Code QR', abbrev: 'QR', url: 'code-qr', desc: 'Un code-barres bidimensionnel qui stocke des données sous forme de motif carré, scannable par smartphone.' },
    'hi-IN': { text: 'क्यूआर कोड', title: 'क्यूआर कोड', abbrev: 'QR', url: 'qr-code', desc: 'एक द्विआयामी बारकोड जो डेटा को वर्गाकार पैटर्न में संग्रहीत करता है, स्मार्टफोन से स्कैन किया जा सकता है।' },
  },
  'dynamic-qr-code': {
    'fr-FR': { text: 'code qr dynamique', title: 'Code QR Dynamique', abbrev: 'QR dynamique', url: 'code-qr-dynamique', desc: 'Un code QR modifiable permettant de changer la destination sans réimpression. Inclut le suivi analytique.' },
    'hi-IN': { text: 'डायनामिक क्यूआर कोड', title: 'डायनामिक क्यूआर कोड', abbrev: 'डायनामिक', url: 'dynamic-qr-code', desc: 'एक संपादन योग्य QR कोड जो बिना पुनर्मुद्रण के गंतव्य बदलने की अनुमति देता है। एनालिटिक्स ट्रैकिंग शामिल।' },
  },
  'static-qr-code': {
    'fr-FR': { text: 'code qr statique', title: 'Code QR Statique', abbrev: 'QR statique', url: 'code-qr-statique', desc: 'Un code QR fixe avec des données encodées directement. Non modifiable après création.' },
    'hi-IN': { text: 'स्टैटिक क्यूआर कोड', title: 'स्टैटिक क्यूआर कोड', abbrev: 'स्टैटिक', url: 'static-qr-code', desc: 'सीधे एन्कोड किए गए डेटा के साथ एक निश्चित QR कोड। निर्माण के बाद संपादन योग्य नहीं।' },
  },
  'custom-qr-code': {
    'fr-FR': { text: 'code qr personnalisé', title: 'Code QR Personnalisé', abbrev: 'QR personnalisé', url: 'code-qr-personnalise', desc: 'Un code QR avec éléments visuels configurables : couleurs, logos, formes et cadres.' },
    'hi-IN': { text: 'कस्टम क्यूआर कोड', title: 'कस्टम क्यूआर कोड', abbrev: 'कस्टम', url: 'custom-qr-code', desc: 'कॉन्फ़िगर करने योग्य विज़ुअल तत्वों वाला QR कोड: रंग, लोगो, आकार और फ्रेम।' },
  },
  'qr-code-art': {
    'fr-FR': { text: 'art qr', title: 'Art QR', abbrev: 'Art QR', url: 'art-qr', desc: 'Un code QR artistique généré par IA où le motif scannable fusionne avec de l\'art.' },
    'hi-IN': { text: 'क्यूआर कोड आर्ट', title: 'क्यूआर कोड आर्ट', abbrev: 'QR आर्ट', url: 'qr-code-art', desc: 'AI-जनित कलात्मक QR कोड जहां स्कैन करने योग्य पैटर्न कला के साथ विलीन हो जाता है।' },
  },
  'qr-code-generator': {
    'fr-FR': { text: 'générateur de codes qr', title: 'Générateur de Codes QR', abbrev: 'Générateur QR', url: 'generateur-codes-qr', desc: 'Outil principal de création de codes QR avec tous les formats et options d\'export.' },
    'hi-IN': { text: 'क्यूआर कोड जनरेटर', title: 'क्यूआर कोड जनरेटर', abbrev: 'QR जनरेटर', url: 'qr-code-generator', desc: 'सभी फॉर्मेट और एक्सपोर्ट विकल्पों के साथ प्राथमिक QR कोड निर्माण उपकरण।' },
  },
  'barcode': {
    'fr-FR': { text: 'code-barres', title: 'Code-barres', abbrev: 'Barres', url: 'code-barres', desc: 'Formats de codes-barres linéaires unidimensionnels incluant EAN-13, UPC-A, Code 128.' },
    'hi-IN': { text: 'बारकोड', title: 'बारकोड', abbrev: 'बारकोड', url: 'barcode', desc: 'EAN-13, UPC-A, Code 128 सहित एक-आयामी रैखिक बारकोड प्रारूप।' },
  },
  'smart-link': {
    'fr-FR': { text: 'lien intelligent', title: 'Lien Intelligent', abbrev: 'Smart Link', url: 'lien-intelligent', desc: 'Une URL raccourcie intelligente avec règles de routage conditionnel et analytics.' },
    'hi-IN': { text: 'स्मार्ट लिंक', title: 'स्मार्ट लिंक', abbrev: 'स्मार्ट लिंक', url: 'smart-link', desc: 'सशर्त रूटिंग नियमों और एनालिटिक्स के साथ एक बुद्धिमान शॉर्ट URL।' },
  },
  'landing-page': {
    'fr-FR': { text: 'page de destination', title: 'Page de Destination', abbrev: 'Landing', url: 'page-de-destination', desc: 'Une page de destination optimisée mobile créée avec un constructeur drag-and-drop.' },
    'hi-IN': { text: 'लैंडिंग पेज', title: 'लैंडिंग पेज', abbrev: 'लैंडिंग', url: 'landing-page', desc: 'ड्रैग-एंड-ड्रॉप बिल्डर के साथ बनाया गया मोबाइल-ऑप्टिमाइज़्ड लैंडिंग पेज।' },
  },
};

function escapeCypher(text) {
  if (!text) return '';
  return text
    .replace(/\\/g, '\\\\')
    .replace(/'/g, "\\'")
    .replace(/\n/g, ' ');
}

function generateDenominationForms(data) {
  return JSON.stringify([
    { type: 'text', value: data.text, priority: 1 },
    { type: 'title', value: data.title, priority: 1 },
    { type: 'abbrev', value: data.abbrev, priority: 1 },
    { type: 'url', value: data.url, priority: 1 },
  ]).replace(/'/g, "\\'");
}

console.log('🔄 Extending EntityNatives to fr-FR and hi-IN...\n');

const lines = [
  '// ============================================================================',
  '// Seed 26.1: EntityNatives Extended (fr-FR, hi-IN)',
  '// ============================================================================',
  '// Generated by: extend-entity-natives.js',
  `// Generated at: ${new Date().toISOString()}`,
  '// Locales: fr-FR, hi-IN (2 locales × 9 entities = 18 nodes)',
  '// ============================================================================',
  '',
];

let count = 0;

for (const locale of LOCALES_TO_ADD) {
  lines.push(`// ----------------------------------------------------------------------------`);
  lines.push(`// ${locale} EntityNatives`);
  lines.push(`// ----------------------------------------------------------------------------`);
  lines.push('');

  for (const [entitySlug, translations] of Object.entries(ENTITIES)) {
    const data = translations[locale];
    if (!data) continue;

    const key = `${entitySlug}@${locale}`;
    const entityKey = `entity:${entitySlug}`;
    const denomForms = generateDenominationForms(data);

    lines.push(`// ${data.title}`);
    lines.push(`MERGE (en:EntityNative {key: '${key}'})`);
    lines.push(`SET en.display_name = '${escapeCypher(data.title)}',`);
    lines.push(`    en.locale = '${locale}',`);
    lines.push(`    en.description = '${escapeCypher(data.desc)}',`);
    lines.push(`    en.denomination_forms = '${denomForms}',`);
    lines.push(`    en.provenance = 'authored',`);
    lines.push(`    en.created_at = datetime(),`);
    lines.push(`    en.updated_at = datetime();`);
    lines.push('');

    // Link to Entity
    lines.push(`MATCH (e:Entity {key: '${entityKey}'})`);
    lines.push(`MATCH (en:EntityNative {key: '${key}'})`);
    lines.push(`MERGE (e)-[:HAS_NATIVE]->(en);`);
    lines.push('');

    // Link to Locale
    lines.push(`MATCH (l:Locale {key: '${locale}'})`);
    lines.push(`MATCH (en:EntityNative {key: '${key}'})`);
    lines.push(`MERGE (en)-[:FOR_LOCALE]->(l);`);
    lines.push('');

    count++;
  }
}

fs.writeFileSync(OUTPUT_FILE, lines.join('\n'));

console.log(`
═══════════════════════════════════════════════════════════════════
📊 ENTITYNATIVE EXTENSION COMPLETE
═══════════════════════════════════════════════════════════════════
   Locales:         ${LOCALES_TO_ADD.length} (fr-FR, hi-IN)
   Entities:        ${Object.keys(ENTITIES).length}
   EntityNatives:   ${count}

📁 Output: packages/db/seed/26.1-entity-natives-fr-hi.cypher
`);
