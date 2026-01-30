// novanet-core/scripts/migrate-locale-v710.ts
/**
 * Migrate Locale nodes to v7.1.0 standard properties
 *
 * Updates existing Locale nodes to have:
 * - display_name (from language + country names)
 * - icon (flag emoji from country code)
 * - description
 * - llm_context
 * - priority
 * - freshness
 * - language_code (renamed from language)
 * - country_code (renamed from region)
 *
 * Usage: npm run migrate:locale-v710
 */

import { existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';
import neo4j, { Driver, Session } from 'neo4j-driver';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

function findProjectRoot(startDir: string): string {
  let dir = startDir;
  while (!existsSync(join(dir, 'package.json'))) {
    const parent = dirname(dir);
    if (parent === dir) throw new Error('Could not find project root');
    dir = parent;
  }
  return dir;
}

// Validate we're in correct project
findProjectRoot(__dirname);

// Neo4j configuration
const NEO4J_URI = process.env.NEO4J_URI || 'bolt://localhost:7687';
const NEO4J_USER = process.env.NEO4J_USER || 'neo4j';
const NEO4J_PASSWORD = process.env.NEO4J_PASSWORD || 'novanetpassword';

// Language code to name mapping (ISO 639-1)
const LANGUAGE_NAMES: Record<string, string> = {
  'af': 'Afrikaans', 'ar': 'Arabic', 'az': 'Azerbaijani', 'be': 'Belarusian',
  'bg': 'Bulgarian', 'bn': 'Bengali', 'bs': 'Bosnian', 'ca': 'Catalan',
  'cs': 'Czech', 'cy': 'Welsh', 'da': 'Danish', 'de': 'German',
  'el': 'Greek', 'en': 'English', 'es': 'Spanish', 'et': 'Estonian',
  'eu': 'Basque', 'fa': 'Persian', 'fi': 'Finnish', 'fil': 'Filipino',
  'fr': 'French', 'ga': 'Irish', 'gl': 'Galician', 'gu': 'Gujarati',
  'he': 'Hebrew', 'hi': 'Hindi', 'hr': 'Croatian', 'hu': 'Hungarian',
  'hy': 'Armenian', 'id': 'Indonesian', 'is': 'Icelandic', 'it': 'Italian',
  'ja': 'Japanese', 'ka': 'Georgian', 'kk': 'Kazakh', 'km': 'Khmer',
  'kn': 'Kannada', 'ko': 'Korean', 'ky': 'Kyrgyz', 'lo': 'Lao',
  'lt': 'Lithuanian', 'lv': 'Latvian', 'mk': 'Macedonian', 'ml': 'Malayalam',
  'mn': 'Mongolian', 'mr': 'Marathi', 'ms': 'Malay', 'mt': 'Maltese',
  'my': 'Burmese', 'ne': 'Nepali', 'nl': 'Dutch', 'no': 'Norwegian',
  'pa': 'Punjabi', 'pl': 'Polish', 'pt': 'Portuguese', 'ro': 'Romanian',
  'ru': 'Russian', 'si': 'Sinhala', 'sk': 'Slovak', 'sl': 'Slovenian',
  'sq': 'Albanian', 'sr': 'Serbian', 'sv': 'Swedish', 'sw': 'Swahili',
  'ta': 'Tamil', 'te': 'Telugu', 'th': 'Thai', 'tl': 'Tagalog',
  'tr': 'Turkish', 'uk': 'Ukrainian', 'ur': 'Urdu', 'uz': 'Uzbek',
  'vi': 'Vietnamese', 'zh': 'Chinese', 'zu': 'Zulu'
};

// Country code to name mapping (ISO 3166-1 alpha-2)
const COUNTRY_NAMES: Record<string, string> = {
  'AE': 'United Arab Emirates', 'AF': 'Afghanistan', 'AL': 'Albania', 'AM': 'Armenia',
  'AR': 'Argentina', 'AT': 'Austria', 'AU': 'Australia', 'AZ': 'Azerbaijan',
  'BA': 'Bosnia', 'BD': 'Bangladesh', 'BE': 'Belgium', 'BG': 'Bulgaria',
  'BH': 'Bahrain', 'BN': 'Brunei', 'BO': 'Bolivia', 'BR': 'Brazil',
  'BY': 'Belarus', 'CA': 'Canada', 'CH': 'Switzerland', 'CL': 'Chile',
  'CN': 'China', 'CO': 'Colombia', 'CR': 'Costa Rica', 'CY': 'Cyprus',
  'CZ': 'Czech Republic', 'DE': 'Germany', 'DK': 'Denmark', 'DO': 'Dominican Republic',
  'DZ': 'Algeria', 'EC': 'Ecuador', 'EE': 'Estonia', 'EG': 'Egypt',
  'ES': 'Spain', 'FI': 'Finland', 'FR': 'France', 'GB': 'United Kingdom',
  'GE': 'Georgia', 'GH': 'Ghana', 'GR': 'Greece', 'GT': 'Guatemala',
  'HK': 'Hong Kong', 'HN': 'Honduras', 'HR': 'Croatia', 'HU': 'Hungary',
  'ID': 'Indonesia', 'IE': 'Ireland', 'IL': 'Israel', 'IN': 'India',
  'IQ': 'Iraq', 'IR': 'Iran', 'IS': 'Iceland', 'IT': 'Italy',
  'JO': 'Jordan', 'JP': 'Japan', 'KE': 'Kenya', 'KG': 'Kyrgyzstan',
  'KH': 'Cambodia', 'KR': 'South Korea', 'KW': 'Kuwait', 'KZ': 'Kazakhstan',
  'LA': 'Laos', 'LB': 'Lebanon', 'LK': 'Sri Lanka', 'LT': 'Lithuania',
  'LU': 'Luxembourg', 'LV': 'Latvia', 'LY': 'Libya', 'MA': 'Morocco',
  'ME': 'Montenegro', 'MK': 'North Macedonia', 'MM': 'Myanmar', 'MN': 'Mongolia',
  'MO': 'Macau', 'MT': 'Malta', 'MX': 'Mexico', 'MY': 'Malaysia',
  'NG': 'Nigeria', 'NI': 'Nicaragua', 'NL': 'Netherlands', 'NO': 'Norway',
  'NP': 'Nepal', 'NZ': 'New Zealand', 'OM': 'Oman', 'PA': 'Panama',
  'PE': 'Peru', 'PH': 'Philippines', 'PK': 'Pakistan', 'PL': 'Poland',
  'PR': 'Puerto Rico', 'PT': 'Portugal', 'PY': 'Paraguay', 'QA': 'Qatar',
  'RO': 'Romania', 'RS': 'Serbia', 'RU': 'Russia', 'SA': 'Saudi Arabia',
  'SD': 'Sudan', 'SE': 'Sweden', 'SG': 'Singapore', 'SI': 'Slovenia',
  'SK': 'Slovakia', 'SN': 'Senegal', 'SV': 'El Salvador', 'SY': 'Syria',
  'TH': 'Thailand', 'TJ': 'Tajikistan', 'TM': 'Turkmenistan', 'TN': 'Tunisia',
  'TR': 'Turkey', 'TW': 'Taiwan', 'TZ': 'Tanzania', 'UA': 'Ukraine',
  'UG': 'Uganda', 'US': 'United States', 'UY': 'Uruguay', 'UZ': 'Uzbekistan',
  'VE': 'Venezuela', 'VN': 'Vietnam', 'YE': 'Yemen', 'ZA': 'South Africa',
  'ZW': 'Zimbabwe'
};

/**
 * Convert country code to flag emoji
 */
function countryCodeToFlag(countryCode: string): string {
  const codePoints = countryCode
    .toUpperCase()
    .split('')
    .map(char => 127397 + char.charCodeAt(0));
  return String.fromCodePoint(...codePoints);
}

/**
 * Main migration function
 */
async function main(): Promise<void> {
  console.log('=== Locale v7.1.0 Migration ===');
  console.log(`Neo4j URI: ${NEO4J_URI}`);
  console.log('');

  const driver: Driver = neo4j.driver(
    NEO4J_URI,
    neo4j.auth.basic(NEO4J_USER, NEO4J_PASSWORD)
  );

  const session: Session = driver.session();

  try {
    // Get all Locale nodes that need migration
    const result = await session.run(`
      MATCH (l:Locale)
      WHERE l.display_name IS NULL
      RETURN l.key AS key, l.language AS language, l.region AS region
    `);

    const locales = result.records.map(r => ({
      key: r.get('key') as string,
      language: r.get('language') as string,
      region: r.get('region') as string,
    }));

    console.log(`Found ${locales.length} locale(s) to migrate\n`);

    if (locales.length === 0) {
      console.log('No migration needed.');
      return;
    }

    let migrated = 0;

    for (const locale of locales) {
      const languageName = LANGUAGE_NAMES[locale.language] || locale.language;
      const countryName = COUNTRY_NAMES[locale.region] || locale.region;
      const displayName = `${languageName} (${countryName})`;
      const icon = countryCodeToFlag(locale.region);
      const description = `${languageName} locale for ${countryName} market`;
      const llmContext = `USE: ${languageName} content for ${countryName}. TRIGGERS: ${locale.key}, ${languageName.toLowerCase()}. NOT: other ${languageName} variants.`;

      await session.run(`
        MATCH (l:Locale {key: $key})
        SET l.display_name = $displayName,
            l.icon = $icon,
            l.description = $description,
            l.llm_context = $llmContext,
            l.priority = 'medium',
            l.freshness = 'static',
            l.language_code = l.language,
            l.country_code = l.region,
            l.updated_at = datetime()
        REMOVE l.language, l.region, l.name
      `, {
        key: locale.key,
        displayName,
        icon,
        description,
        llmContext,
      });

      migrated++;
      if (migrated % 20 === 0) {
        console.log(`  Migrated ${migrated}/${locales.length}...`);
      }
    }

    console.log(`\n=== Migration Complete ===`);
    console.log(`Locales migrated: ${migrated}`);

  } catch (error) {
    console.error('Migration failed:', error);
    throw error;
  } finally {
    await session.close();
    await driver.close();
  }
}

main().catch(error => {
  console.error('Fatal error:', error);
  process.exit(1);
});
