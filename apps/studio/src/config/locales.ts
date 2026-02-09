/**
 * Supported Locales Registry
 * 200 BCP 47 locale codes from novanet-core
 * Source: novanet-core/config/locales.yaml
 */

export interface LocaleInfo {
  code: string;
  language: string;
  country: string;
  family?: string;
}

/**
 * Primary locales (major markets) - shown first in dropdown
 */
export const PRIMARY_LOCALES: string[] = [
  'en-US', 'fr-FR', 'de-DE', 'es-ES', 'it-IT', 'pt-BR',
  'ja-JP', 'ko-KR', 'zh-CN', 'zh-TW', 'ar-SA', 'ru-RU',
  'nl-NL', 'pl-PL', 'tr-TR', 'vi-VN', 'th-TH', 'id-ID', 'hi-IN',
];

/**
 * All 200 supported locales (BCP 47 format)
 */
export const ALL_LOCALES: string[] = [
  // Afrikaans
  'af-ZA',
  // Arabic (14 variants)
  'ar-AE', 'ar-BH', 'ar-DZ', 'ar-EG', 'ar-IQ', 'ar-JO', 'ar-KW',
  'ar-LB', 'ar-LY', 'ar-MA', 'ar-OM', 'ar-QA', 'ar-SA', 'ar-TN',
  // South Asian
  'as-IN', 'bn-BD', 'bn-IN', 'gu-IN', 'hi-IN', 'kn-IN', 'ml-IN',
  'mr-IN', 'ne-NP', 'pa-IN', 'pa-PK', 'sd-PK', 'si-LK', 'ta-IN',
  'ta-LK', 'te-IN', 'ur-PK',
  // Turkic
  'az-AZ', 'kk-KZ', 'ky-KG', 'tk-TM', 'tr-TR', 'uz-UZ',
  // Slavic
  'be-BY', 'bg-BG', 'bs-BA', 'cs-CZ', 'hr-HR', 'mk-MK', 'pl-PL',
  'ru-BY', 'ru-IL', 'ru-KG', 'ru-KZ', 'ru-MD', 'ru-RU',
  'sk-SK', 'sl-SI', 'sr-RS', 'uk-UA',
  // Romance
  'ca-AD', 'ca-ES',
  'es-AR', 'es-BO', 'es-CL', 'es-CO', 'es-CR', 'es-CU', 'es-DO',
  'es-EC', 'es-ES', 'es-GT', 'es-HN', 'es-MX', 'es-NI', 'es-PA',
  'es-PE', 'es-PR', 'es-PY', 'es-SV', 'es-UY', 'es-VE',
  'fr-BE', 'fr-BF', 'fr-CA', 'fr-CD', 'fr-CH', 'fr-CI', 'fr-CM',
  'fr-DZ', 'fr-FR', 'fr-LU', 'fr-MA', 'fr-MG', 'fr-RW', 'fr-SN', 'fr-TN',
  'gl-ES', 'it-CH', 'it-IT',
  'pt-AO', 'pt-BR', 'pt-CH', 'pt-MZ', 'pt-PT',
  'ro-MD', 'ro-RO',
  // Germanic
  'da-DK',
  'de-AT', 'de-CH', 'de-DE', 'de-LU',
  'en-AE', 'en-AU', 'en-BB', 'en-BW', 'en-CA', 'en-CY', 'en-FJ',
  'en-GB', 'en-GH', 'en-HK', 'en-IE', 'en-IN', 'en-JM', 'en-KE',
  'en-KY', 'en-MU', 'en-MY', 'en-NG', 'en-NZ', 'en-PH', 'en-PK',
  'en-SA', 'en-SG', 'en-TT', 'en-TZ', 'en-UG', 'en-US', 'en-VN',
  'en-ZA', 'en-ZM', 'en-ZW',
  'is-IS', 'nl-BE', 'nl-NL', 'no-NO', 'sv-SE',
  // Celtic
  'cy-GB', 'ga-IE',
  // Greek
  'el-CY', 'el-GR',
  // Baltic
  'lt-LT', 'lv-LV',
  // Uralic
  'et-EE', 'fi-FI', 'hu-HU',
  // Basque
  'eu-ES',
  // Iranian
  'fa-IR', 'ku-TR', 'ps-AF', 'tg-TJ',
  // Armenian
  'hy-AM',
  // Georgian
  'ka-GE',
  // Semitic
  'he-IL',
  // East Asian
  'ja-JP', 'ko-KR', 'mn-MN', 'zh-CN', 'zh-HK', 'zh-SG', 'zh-TH', 'zh-TW',
  // Southeast Asian
  'ceb-PH', 'id-ID', 'jv-ID', 'km-KH', 'ms-BN', 'ms-MY', 'ms-SG',
  'my-MM', 'su-ID', 'th-TH', 'tl-PH', 'vi-VN',
  // African
  'ha-NG', 'ig-NG', 'ln-CD', 'mg-MG', 'ny-MW', 'rw-RW', 'sn-ZW',
  'so-SO', 'sw-KE', 'sw-TZ', 'wo-SN', 'xh-ZA', 'yo-NG', 'zu-ZA',
  // Austronesian
  'mi-NZ',
  // Creole
  'ht-HT',
  // Native American
  'gn-PY', 'qu-PE',
  // Albanian
  'sq-AL',
  // Maltese
  'mt-MT',
];

/**
 * Locale metadata for display
 */
export const LOCALE_METADATA: Record<string, { name: string; flag: string }> = {
  'af-ZA': { name: 'Afrikaans', flag: '🇿🇦' },
  'ar-AE': { name: 'Arabic (UAE)', flag: '🇦🇪' },
  'ar-BH': { name: 'Arabic (Bahrain)', flag: '🇧🇭' },
  'ar-DZ': { name: 'Arabic (Algeria)', flag: '🇩🇿' },
  'ar-EG': { name: 'Arabic (Egypt)', flag: '🇪🇬' },
  'ar-IQ': { name: 'Arabic (Iraq)', flag: '🇮🇶' },
  'ar-JO': { name: 'Arabic (Jordan)', flag: '🇯🇴' },
  'ar-KW': { name: 'Arabic (Kuwait)', flag: '🇰🇼' },
  'ar-LB': { name: 'Arabic (Lebanon)', flag: '🇱🇧' },
  'ar-LY': { name: 'Arabic (Libya)', flag: '🇱🇾' },
  'ar-MA': { name: 'Arabic (Morocco)', flag: '🇲🇦' },
  'ar-OM': { name: 'Arabic (Oman)', flag: '🇴🇲' },
  'ar-QA': { name: 'Arabic (Qatar)', flag: '🇶🇦' },
  'ar-SA': { name: 'Arabic (Saudi Arabia)', flag: '🇸🇦' },
  'ar-TN': { name: 'Arabic (Tunisia)', flag: '🇹🇳' },
  'as-IN': { name: 'Assamese', flag: '🇮🇳' },
  'az-AZ': { name: 'Azerbaijani', flag: '🇦🇿' },
  'be-BY': { name: 'Belarusian', flag: '🇧🇾' },
  'bg-BG': { name: 'Bulgarian', flag: '🇧🇬' },
  'bn-BD': { name: 'Bengali (Bangladesh)', flag: '🇧🇩' },
  'bn-IN': { name: 'Bengali (India)', flag: '🇮🇳' },
  'bs-BA': { name: 'Bosnian', flag: '🇧🇦' },
  'ca-AD': { name: 'Catalan (Andorra)', flag: '🇦🇩' },
  'ca-ES': { name: 'Catalan (Spain)', flag: '🇪🇸' },
  'ceb-PH': { name: 'Cebuano', flag: '🇵🇭' },
  'cs-CZ': { name: 'Czech', flag: '🇨🇿' },
  'cy-GB': { name: 'Welsh', flag: '🏴󠁧󠁢󠁷󠁬󠁳󠁿' },
  'da-DK': { name: 'Danish', flag: '🇩🇰' },
  'de-AT': { name: 'German (Austria)', flag: '🇦🇹' },
  'de-CH': { name: 'German (Switzerland)', flag: '🇨🇭' },
  'de-DE': { name: 'German', flag: '🇩🇪' },
  'de-LU': { name: 'German (Luxembourg)', flag: '🇱🇺' },
  'el-CY': { name: 'Greek (Cyprus)', flag: '🇨🇾' },
  'el-GR': { name: 'Greek', flag: '🇬🇷' },
  'en-AE': { name: 'English (UAE)', flag: '🇦🇪' },
  'en-AU': { name: 'English (Australia)', flag: '🇦🇺' },
  'en-BB': { name: 'English (Barbados)', flag: '🇧🇧' },
  'en-BW': { name: 'English (Botswana)', flag: '🇧🇼' },
  'en-CA': { name: 'English (Canada)', flag: '🇨🇦' },
  'en-CY': { name: 'English (Cyprus)', flag: '🇨🇾' },
  'en-FJ': { name: 'English (Fiji)', flag: '🇫🇯' },
  'en-GB': { name: 'English (UK)', flag: '🇬🇧' },
  'en-GH': { name: 'English (Ghana)', flag: '🇬🇭' },
  'en-HK': { name: 'English (Hong Kong)', flag: '🇭🇰' },
  'en-IE': { name: 'English (Ireland)', flag: '🇮🇪' },
  'en-IN': { name: 'English (India)', flag: '🇮🇳' },
  'en-JM': { name: 'English (Jamaica)', flag: '🇯🇲' },
  'en-KE': { name: 'English (Kenya)', flag: '🇰🇪' },
  'en-KY': { name: 'English (Cayman)', flag: '🇰🇾' },
  'en-MU': { name: 'English (Mauritius)', flag: '🇲🇺' },
  'en-MY': { name: 'English (Malaysia)', flag: '🇲🇾' },
  'en-NG': { name: 'English (Nigeria)', flag: '🇳🇬' },
  'en-NZ': { name: 'English (New Zealand)', flag: '🇳🇿' },
  'en-PH': { name: 'English (Philippines)', flag: '🇵🇭' },
  'en-PK': { name: 'English (Pakistan)', flag: '🇵🇰' },
  'en-SA': { name: 'English (Saudi Arabia)', flag: '🇸🇦' },
  'en-SG': { name: 'English (Singapore)', flag: '🇸🇬' },
  'en-TT': { name: 'English (Trinidad)', flag: '🇹🇹' },
  'en-TZ': { name: 'English (Tanzania)', flag: '🇹🇿' },
  'en-UG': { name: 'English (Uganda)', flag: '🇺🇬' },
  'en-US': { name: 'English (US)', flag: '🇺🇸' },
  'en-VN': { name: 'English (Vietnam)', flag: '🇻🇳' },
  'en-ZA': { name: 'English (South Africa)', flag: '🇿🇦' },
  'en-ZM': { name: 'English (Zambia)', flag: '🇿🇲' },
  'en-ZW': { name: 'English (Zimbabwe)', flag: '🇿🇼' },
  'es-AR': { name: 'Spanish (Argentina)', flag: '🇦🇷' },
  'es-BO': { name: 'Spanish (Bolivia)', flag: '🇧🇴' },
  'es-CL': { name: 'Spanish (Chile)', flag: '🇨🇱' },
  'es-CO': { name: 'Spanish (Colombia)', flag: '🇨🇴' },
  'es-CR': { name: 'Spanish (Costa Rica)', flag: '🇨🇷' },
  'es-CU': { name: 'Spanish (Cuba)', flag: '🇨🇺' },
  'es-DO': { name: 'Spanish (Dominican Rep)', flag: '🇩🇴' },
  'es-EC': { name: 'Spanish (Ecuador)', flag: '🇪🇨' },
  'es-ES': { name: 'Spanish (Spain)', flag: '🇪🇸' },
  'es-GT': { name: 'Spanish (Guatemala)', flag: '🇬🇹' },
  'es-HN': { name: 'Spanish (Honduras)', flag: '🇭🇳' },
  'es-MX': { name: 'Spanish (Mexico)', flag: '🇲🇽' },
  'es-NI': { name: 'Spanish (Nicaragua)', flag: '🇳🇮' },
  'es-PA': { name: 'Spanish (Panama)', flag: '🇵🇦' },
  'es-PE': { name: 'Spanish (Peru)', flag: '🇵🇪' },
  'es-PR': { name: 'Spanish (Puerto Rico)', flag: '🇵🇷' },
  'es-PY': { name: 'Spanish (Paraguay)', flag: '🇵🇾' },
  'es-SV': { name: 'Spanish (El Salvador)', flag: '🇸🇻' },
  'es-UY': { name: 'Spanish (Uruguay)', flag: '🇺🇾' },
  'es-VE': { name: 'Spanish (Venezuela)', flag: '🇻🇪' },
  'et-EE': { name: 'Estonian', flag: '🇪🇪' },
  'eu-ES': { name: 'Basque', flag: '🇪🇸' },
  'fa-IR': { name: 'Persian', flag: '🇮🇷' },
  'fi-FI': { name: 'Finnish', flag: '🇫🇮' },
  'fr-BE': { name: 'French (Belgium)', flag: '🇧🇪' },
  'fr-BF': { name: 'French (Burkina Faso)', flag: '🇧🇫' },
  'fr-CA': { name: 'French (Canada)', flag: '🇨🇦' },
  'fr-CD': { name: 'French (Congo DRC)', flag: '🇨🇩' },
  'fr-CH': { name: 'French (Switzerland)', flag: '🇨🇭' },
  'fr-CI': { name: 'French (Ivory Coast)', flag: '🇨🇮' },
  'fr-CM': { name: 'French (Cameroon)', flag: '🇨🇲' },
  'fr-DZ': { name: 'French (Algeria)', flag: '🇩🇿' },
  'fr-FR': { name: 'French', flag: '🇫🇷' },
  'fr-LU': { name: 'French (Luxembourg)', flag: '🇱🇺' },
  'fr-MA': { name: 'French (Morocco)', flag: '🇲🇦' },
  'fr-MG': { name: 'French (Madagascar)', flag: '🇲🇬' },
  'fr-RW': { name: 'French (Rwanda)', flag: '🇷🇼' },
  'fr-SN': { name: 'French (Senegal)', flag: '🇸🇳' },
  'fr-TN': { name: 'French (Tunisia)', flag: '🇹🇳' },
  'ga-IE': { name: 'Irish', flag: '🇮🇪' },
  'gl-ES': { name: 'Galician', flag: '🇪🇸' },
  'gn-PY': { name: 'Guarani', flag: '🇵🇾' },
  'gu-IN': { name: 'Gujarati', flag: '🇮🇳' },
  'ha-NG': { name: 'Hausa', flag: '🇳🇬' },
  'he-IL': { name: 'Hebrew', flag: '🇮🇱' },
  'hi-IN': { name: 'Hindi', flag: '🇮🇳' },
  'hr-HR': { name: 'Croatian', flag: '🇭🇷' },
  'ht-HT': { name: 'Haitian Creole', flag: '🇭🇹' },
  'hu-HU': { name: 'Hungarian', flag: '🇭🇺' },
  'hy-AM': { name: 'Armenian', flag: '🇦🇲' },
  'id-ID': { name: 'Indonesian', flag: '🇮🇩' },
  'ig-NG': { name: 'Igbo', flag: '🇳🇬' },
  'is-IS': { name: 'Icelandic', flag: '🇮🇸' },
  'it-CH': { name: 'Italian (Switzerland)', flag: '🇨🇭' },
  'it-IT': { name: 'Italian', flag: '🇮🇹' },
  'ja-JP': { name: 'Japanese', flag: '🇯🇵' },
  'jv-ID': { name: 'Javanese', flag: '🇮🇩' },
  'ka-GE': { name: 'Georgian', flag: '🇬🇪' },
  'kk-KZ': { name: 'Kazakh', flag: '🇰🇿' },
  'km-KH': { name: 'Khmer', flag: '🇰🇭' },
  'kn-IN': { name: 'Kannada', flag: '🇮🇳' },
  'ko-KR': { name: 'Korean', flag: '🇰🇷' },
  'ku-TR': { name: 'Kurdish', flag: '🇹🇷' },
  'ky-KG': { name: 'Kyrgyz', flag: '🇰🇬' },
  'ln-CD': { name: 'Lingala', flag: '🇨🇩' },
  'lt-LT': { name: 'Lithuanian', flag: '🇱🇹' },
  'lv-LV': { name: 'Latvian', flag: '🇱🇻' },
  'mg-MG': { name: 'Malagasy', flag: '🇲🇬' },
  'mi-NZ': { name: 'Maori', flag: '🇳🇿' },
  'mk-MK': { name: 'Macedonian', flag: '🇲🇰' },
  'ml-IN': { name: 'Malayalam', flag: '🇮🇳' },
  'mn-MN': { name: 'Mongolian', flag: '🇲🇳' },
  'mr-IN': { name: 'Marathi', flag: '🇮🇳' },
  'ms-BN': { name: 'Malay (Brunei)', flag: '🇧🇳' },
  'ms-MY': { name: 'Malay (Malaysia)', flag: '🇲🇾' },
  'ms-SG': { name: 'Malay (Singapore)', flag: '🇸🇬' },
  'mt-MT': { name: 'Maltese', flag: '🇲🇹' },
  'my-MM': { name: 'Burmese', flag: '🇲🇲' },
  'ne-NP': { name: 'Nepali', flag: '🇳🇵' },
  'nl-BE': { name: 'Dutch (Belgium)', flag: '🇧🇪' },
  'nl-NL': { name: 'Dutch', flag: '🇳🇱' },
  'no-NO': { name: 'Norwegian', flag: '🇳🇴' },
  'ny-MW': { name: 'Chichewa', flag: '🇲🇼' },
  'pa-IN': { name: 'Punjabi (India)', flag: '🇮🇳' },
  'pa-PK': { name: 'Punjabi (Pakistan)', flag: '🇵🇰' },
  'pl-PL': { name: 'Polish', flag: '🇵🇱' },
  'ps-AF': { name: 'Pashto', flag: '🇦🇫' },
  'pt-AO': { name: 'Portuguese (Angola)', flag: '🇦🇴' },
  'pt-BR': { name: 'Portuguese (Brazil)', flag: '🇧🇷' },
  'pt-CH': { name: 'Portuguese (Switzerland)', flag: '🇨🇭' },
  'pt-MZ': { name: 'Portuguese (Mozambique)', flag: '🇲🇿' },
  'pt-PT': { name: 'Portuguese', flag: '🇵🇹' },
  'qu-PE': { name: 'Quechua', flag: '🇵🇪' },
  'ro-MD': { name: 'Romanian (Moldova)', flag: '🇲🇩' },
  'ro-RO': { name: 'Romanian', flag: '🇷🇴' },
  'ru-BY': { name: 'Russian (Belarus)', flag: '🇧🇾' },
  'ru-IL': { name: 'Russian (Israel)', flag: '🇮🇱' },
  'ru-KG': { name: 'Russian (Kyrgyzstan)', flag: '🇰🇬' },
  'ru-KZ': { name: 'Russian (Kazakhstan)', flag: '🇰🇿' },
  'ru-MD': { name: 'Russian (Moldova)', flag: '🇲🇩' },
  'ru-RU': { name: 'Russian', flag: '🇷🇺' },
  'rw-RW': { name: 'Kinyarwanda', flag: '🇷🇼' },
  'sd-PK': { name: 'Sindhi', flag: '🇵🇰' },
  'si-LK': { name: 'Sinhala', flag: '🇱🇰' },
  'sk-SK': { name: 'Slovak', flag: '🇸🇰' },
  'sl-SI': { name: 'Slovenian', flag: '🇸🇮' },
  'sn-ZW': { name: 'Shona', flag: '🇿🇼' },
  'so-SO': { name: 'Somali', flag: '🇸🇴' },
  'sq-AL': { name: 'Albanian', flag: '🇦🇱' },
  'sr-RS': { name: 'Serbian', flag: '🇷🇸' },
  'su-ID': { name: 'Sundanese', flag: '🇮🇩' },
  'sv-SE': { name: 'Swedish', flag: '🇸🇪' },
  'sw-KE': { name: 'Swahili (Kenya)', flag: '🇰🇪' },
  'sw-TZ': { name: 'Swahili (Tanzania)', flag: '🇹🇿' },
  'ta-IN': { name: 'Tamil (India)', flag: '🇮🇳' },
  'ta-LK': { name: 'Tamil (Sri Lanka)', flag: '🇱🇰' },
  'te-IN': { name: 'Telugu', flag: '🇮🇳' },
  'tg-TJ': { name: 'Tajik', flag: '🇹🇯' },
  'th-TH': { name: 'Thai', flag: '🇹🇭' },
  'tk-TM': { name: 'Turkmen', flag: '🇹🇲' },
  'tl-PH': { name: 'Tagalog', flag: '🇵🇭' },
  'tr-TR': { name: 'Turkish', flag: '🇹🇷' },
  'uk-UA': { name: 'Ukrainian', flag: '🇺🇦' },
  'ur-PK': { name: 'Urdu', flag: '🇵🇰' },
  'uz-UZ': { name: 'Uzbek', flag: '🇺🇿' },
  'vi-VN': { name: 'Vietnamese', flag: '🇻🇳' },
  'wo-SN': { name: 'Wolof', flag: '🇸🇳' },
  'xh-ZA': { name: 'Xhosa', flag: '🇿🇦' },
  'yo-NG': { name: 'Yoruba', flag: '🇳🇬' },
  'zh-CN': { name: 'Chinese (Simplified)', flag: '🇨🇳' },
  'zh-HK': { name: 'Chinese (Hong Kong)', flag: '🇭🇰' },
  'zh-SG': { name: 'Chinese (Singapore)', flag: '🇸🇬' },
  'zh-TH': { name: 'Chinese (Thailand)', flag: '🇹🇭' },
  'zh-TW': { name: 'Chinese (Traditional)', flag: '🇹🇼' },
  'zu-ZA': { name: 'Zulu', flag: '🇿🇦' },
};

/**
 * Language families for grouping
 */
export type LanguageFamily =
  | 'germanic'
  | 'romance'
  | 'slavic'
  | 'semitic'
  | 'sino-tibetan'
  | 'japonic'
  | 'koreanic'
  | 'turkic'
  | 'indo-aryan'
  | 'dravidian'
  | 'tai-kadai'
  | 'austroasiatic'
  | 'austronesian'
  | 'uralic'
  | 'iranian'
  | 'baltic'
  | 'celtic'
  | 'hellenic'
  | 'albanian'
  | 'armenian'
  | 'kartvelian'
  | 'niger-congo'
  | 'afroasiatic'
  | 'isolate'
  | 'creole'
  | 'other';

/**
 * Language family mapping for each locale
 */
export const LOCALE_FAMILIES: Record<string, LanguageFamily> = {
  // Germanic
  'af-ZA': 'germanic', 'da-DK': 'germanic', 'de-AT': 'germanic', 'de-CH': 'germanic',
  'de-DE': 'germanic', 'de-LU': 'germanic', 'en-AE': 'germanic', 'en-AU': 'germanic',
  'en-BB': 'germanic', 'en-BW': 'germanic', 'en-CA': 'germanic', 'en-CY': 'germanic',
  'en-FJ': 'germanic', 'en-GB': 'germanic', 'en-GH': 'germanic', 'en-HK': 'germanic',
  'en-IE': 'germanic', 'en-IN': 'germanic', 'en-JM': 'germanic', 'en-KE': 'germanic',
  'en-KY': 'germanic', 'en-MU': 'germanic', 'en-MY': 'germanic', 'en-NG': 'germanic',
  'en-NZ': 'germanic', 'en-PH': 'germanic', 'en-PK': 'germanic', 'en-SA': 'germanic',
  'en-SG': 'germanic', 'en-TT': 'germanic', 'en-TZ': 'germanic', 'en-UG': 'germanic',
  'en-US': 'germanic', 'en-VN': 'germanic', 'en-ZA': 'germanic', 'en-ZM': 'germanic',
  'en-ZW': 'germanic', 'is-IS': 'germanic', 'nl-BE': 'germanic', 'nl-NL': 'germanic',
  'no-NO': 'germanic', 'sv-SE': 'germanic',
  // Romance
  'ca-AD': 'romance', 'ca-ES': 'romance', 'es-AR': 'romance', 'es-BO': 'romance',
  'es-CL': 'romance', 'es-CO': 'romance', 'es-CR': 'romance', 'es-CU': 'romance',
  'es-DO': 'romance', 'es-EC': 'romance', 'es-ES': 'romance', 'es-GT': 'romance',
  'es-HN': 'romance', 'es-MX': 'romance', 'es-NI': 'romance', 'es-PA': 'romance',
  'es-PE': 'romance', 'es-PR': 'romance', 'es-PY': 'romance', 'es-SV': 'romance',
  'es-UY': 'romance', 'es-VE': 'romance', 'fr-BE': 'romance', 'fr-BF': 'romance',
  'fr-CA': 'romance', 'fr-CD': 'romance', 'fr-CH': 'romance', 'fr-CI': 'romance',
  'fr-CM': 'romance', 'fr-DZ': 'romance', 'fr-FR': 'romance', 'fr-LU': 'romance',
  'fr-MA': 'romance', 'fr-MG': 'romance', 'fr-RW': 'romance', 'fr-SN': 'romance',
  'fr-TN': 'romance', 'gl-ES': 'romance', 'it-CH': 'romance', 'it-IT': 'romance',
  'pt-AO': 'romance', 'pt-BR': 'romance', 'pt-CH': 'romance', 'pt-MZ': 'romance',
  'pt-PT': 'romance', 'ro-MD': 'romance', 'ro-RO': 'romance',
  // Slavic
  'be-BY': 'slavic', 'bg-BG': 'slavic', 'bs-BA': 'slavic', 'cs-CZ': 'slavic',
  'hr-HR': 'slavic', 'mk-MK': 'slavic', 'pl-PL': 'slavic', 'ru-BY': 'slavic',
  'ru-IL': 'slavic', 'ru-KG': 'slavic', 'ru-KZ': 'slavic', 'ru-MD': 'slavic',
  'ru-RU': 'slavic', 'sk-SK': 'slavic', 'sl-SI': 'slavic', 'sr-RS': 'slavic',
  'uk-UA': 'slavic',
  // Semitic
  'ar-AE': 'semitic', 'ar-BH': 'semitic', 'ar-DZ': 'semitic', 'ar-EG': 'semitic',
  'ar-IQ': 'semitic', 'ar-JO': 'semitic', 'ar-KW': 'semitic', 'ar-LB': 'semitic',
  'ar-LY': 'semitic', 'ar-MA': 'semitic', 'ar-OM': 'semitic', 'ar-QA': 'semitic',
  'ar-SA': 'semitic', 'ar-TN': 'semitic', 'he-IL': 'semitic', 'mt-MT': 'semitic',
  // Sino-Tibetan
  'zh-CN': 'sino-tibetan', 'zh-HK': 'sino-tibetan', 'zh-SG': 'sino-tibetan',
  'zh-TH': 'sino-tibetan', 'zh-TW': 'sino-tibetan', 'my-MM': 'sino-tibetan',
  // Japonic
  'ja-JP': 'japonic',
  // Koreanic
  'ko-KR': 'koreanic',
  // Turkic
  'az-AZ': 'turkic', 'kk-KZ': 'turkic', 'ky-KG': 'turkic', 'tk-TM': 'turkic',
  'tr-TR': 'turkic', 'uz-UZ': 'turkic',
  // Indo-Aryan
  'as-IN': 'indo-aryan', 'bn-BD': 'indo-aryan', 'bn-IN': 'indo-aryan',
  'gu-IN': 'indo-aryan', 'hi-IN': 'indo-aryan', 'mr-IN': 'indo-aryan',
  'ne-NP': 'indo-aryan', 'pa-IN': 'indo-aryan', 'pa-PK': 'indo-aryan',
  'sd-PK': 'indo-aryan', 'si-LK': 'indo-aryan', 'ur-PK': 'indo-aryan',
  // Dravidian
  'kn-IN': 'dravidian', 'ml-IN': 'dravidian', 'ta-IN': 'dravidian',
  'ta-LK': 'dravidian', 'te-IN': 'dravidian',
  // Tai-Kadai
  'th-TH': 'tai-kadai',
  // Austroasiatic
  'km-KH': 'austroasiatic', 'vi-VN': 'austroasiatic',
  // Austronesian
  'ceb-PH': 'austronesian', 'id-ID': 'austronesian', 'jv-ID': 'austronesian',
  'mg-MG': 'austronesian', 'mi-NZ': 'austronesian', 'ms-BN': 'austronesian',
  'ms-MY': 'austronesian', 'ms-SG': 'austronesian', 'su-ID': 'austronesian',
  'tl-PH': 'austronesian',
  // Uralic
  'et-EE': 'uralic', 'fi-FI': 'uralic', 'hu-HU': 'uralic',
  // Iranian
  'fa-IR': 'iranian', 'ku-TR': 'iranian', 'ps-AF': 'iranian', 'tg-TJ': 'iranian',
  // Baltic
  'lt-LT': 'baltic', 'lv-LV': 'baltic',
  // Celtic
  'cy-GB': 'celtic', 'ga-IE': 'celtic',
  // Hellenic
  'el-CY': 'hellenic', 'el-GR': 'hellenic',
  // Albanian
  'sq-AL': 'albanian',
  // Armenian
  'hy-AM': 'armenian',
  // Kartvelian
  'ka-GE': 'kartvelian',
  // Niger-Congo
  'ha-NG': 'niger-congo', 'ig-NG': 'niger-congo', 'ln-CD': 'niger-congo',
  'ny-MW': 'niger-congo', 'rw-RW': 'niger-congo', 'sn-ZW': 'niger-congo',
  'sw-KE': 'niger-congo', 'sw-TZ': 'niger-congo', 'wo-SN': 'niger-congo',
  'xh-ZA': 'niger-congo', 'yo-NG': 'niger-congo', 'zu-ZA': 'niger-congo',
  // Afroasiatic
  'so-SO': 'afroasiatic',
  // Isolate
  'eu-ES': 'isolate', 'mn-MN': 'isolate',
  // Creole
  'ht-HT': 'creole',
  // Other
  'gn-PY': 'other', 'qu-PE': 'other',
};

/**
 * Language family display names
 */
export const FAMILY_NAMES: Record<LanguageFamily, string> = {
  'germanic': 'Germanic',
  'romance': 'Romance',
  'slavic': 'Slavic',
  'semitic': 'Semitic',
  'sino-tibetan': 'Sino-Tibetan',
  'japonic': 'Japanese',
  'koreanic': 'Korean',
  'turkic': 'Turkic',
  'indo-aryan': 'Indo-Aryan',
  'dravidian': 'Dravidian',
  'tai-kadai': 'Tai-Kadai',
  'austroasiatic': 'Austroasiatic',
  'austronesian': 'Austronesian',
  'uralic': 'Uralic',
  'iranian': 'Iranian',
  'baltic': 'Baltic',
  'celtic': 'Celtic',
  'hellenic': 'Greek',
  'albanian': 'Albanian',
  'armenian': 'Armenian',
  'kartvelian': 'Kartvelian',
  'niger-congo': 'Niger-Congo',
  'afroasiatic': 'Afroasiatic',
  'isolate': 'Isolate',
  'creole': 'Creole',
  'other': 'Other',
};

/**
 * Region groupings
 */
export type Region = 'europe' | 'americas' | 'asia' | 'africa' | 'oceania' | 'middle-east';

export const LOCALE_REGIONS: Record<string, Region> = {
  // Europe
  'af-ZA': 'africa', 'be-BY': 'europe', 'bg-BG': 'europe', 'bs-BA': 'europe',
  'ca-AD': 'europe', 'ca-ES': 'europe', 'cs-CZ': 'europe', 'cy-GB': 'europe',
  'da-DK': 'europe', 'de-AT': 'europe', 'de-CH': 'europe', 'de-DE': 'europe',
  'de-LU': 'europe', 'el-CY': 'europe', 'el-GR': 'europe', 'en-CY': 'europe',
  'en-GB': 'europe', 'en-IE': 'europe', 'es-ES': 'europe', 'et-EE': 'europe',
  'eu-ES': 'europe', 'fi-FI': 'europe', 'fr-BE': 'europe', 'fr-CH': 'europe',
  'fr-FR': 'europe', 'fr-LU': 'europe', 'ga-IE': 'europe', 'gl-ES': 'europe',
  'hr-HR': 'europe', 'hu-HU': 'europe', 'is-IS': 'europe', 'it-CH': 'europe',
  'it-IT': 'europe', 'lt-LT': 'europe', 'lv-LV': 'europe', 'mk-MK': 'europe',
  'mt-MT': 'europe', 'nl-BE': 'europe', 'nl-NL': 'europe', 'no-NO': 'europe',
  'pl-PL': 'europe', 'pt-CH': 'europe', 'pt-PT': 'europe', 'ro-MD': 'europe',
  'ro-RO': 'europe', 'ru-BY': 'europe', 'ru-MD': 'europe', 'ru-RU': 'europe',
  'sk-SK': 'europe', 'sl-SI': 'europe', 'sq-AL': 'europe', 'sr-RS': 'europe',
  'sv-SE': 'europe', 'uk-UA': 'europe',
  // Americas
  'en-BB': 'americas', 'en-CA': 'americas', 'en-JM': 'americas', 'en-KY': 'americas',
  'en-TT': 'americas', 'en-US': 'americas', 'es-AR': 'americas', 'es-BO': 'americas',
  'es-CL': 'americas', 'es-CO': 'americas', 'es-CR': 'americas', 'es-CU': 'americas',
  'es-DO': 'americas', 'es-EC': 'americas', 'es-GT': 'americas', 'es-HN': 'americas',
  'es-MX': 'americas', 'es-NI': 'americas', 'es-PA': 'americas', 'es-PE': 'americas',
  'es-PR': 'americas', 'es-PY': 'americas', 'es-SV': 'americas', 'es-UY': 'americas',
  'es-VE': 'americas', 'fr-CA': 'americas', 'gn-PY': 'americas', 'ht-HT': 'americas',
  'pt-BR': 'americas', 'qu-PE': 'americas',
  // Asia
  'as-IN': 'asia', 'az-AZ': 'asia', 'bn-BD': 'asia', 'bn-IN': 'asia',
  'ceb-PH': 'asia', 'en-HK': 'asia', 'en-IN': 'asia', 'en-MY': 'asia',
  'en-PH': 'asia', 'en-PK': 'asia', 'en-SG': 'asia', 'en-VN': 'asia',
  'gu-IN': 'asia', 'hi-IN': 'asia', 'hy-AM': 'asia', 'id-ID': 'asia',
  'ja-JP': 'asia', 'jv-ID': 'asia', 'ka-GE': 'asia', 'kk-KZ': 'asia',
  'km-KH': 'asia', 'kn-IN': 'asia', 'ko-KR': 'asia', 'ky-KG': 'asia',
  'ml-IN': 'asia', 'mn-MN': 'asia', 'mr-IN': 'asia', 'ms-BN': 'asia',
  'ms-MY': 'asia', 'ms-SG': 'asia', 'my-MM': 'asia', 'ne-NP': 'asia',
  'pa-IN': 'asia', 'pa-PK': 'asia', 'ru-KG': 'asia', 'ru-KZ': 'asia',
  'sd-PK': 'asia', 'si-LK': 'asia', 'su-ID': 'asia', 'ta-IN': 'asia',
  'ta-LK': 'asia', 'te-IN': 'asia', 'tg-TJ': 'asia', 'th-TH': 'asia',
  'tk-TM': 'asia', 'tl-PH': 'asia', 'ur-PK': 'asia', 'uz-UZ': 'asia',
  'vi-VN': 'asia', 'zh-CN': 'asia', 'zh-HK': 'asia', 'zh-SG': 'asia',
  'zh-TH': 'asia', 'zh-TW': 'asia',
  // Africa
  'en-BW': 'africa', 'en-GH': 'africa', 'en-KE': 'africa', 'en-MU': 'africa',
  'en-NG': 'africa', 'en-TZ': 'africa', 'en-UG': 'africa', 'en-ZA': 'africa',
  'en-ZM': 'africa', 'en-ZW': 'africa', 'fr-BF': 'africa', 'fr-CD': 'africa',
  'fr-CI': 'africa', 'fr-CM': 'africa', 'fr-DZ': 'africa', 'fr-MA': 'africa',
  'fr-MG': 'africa', 'fr-RW': 'africa', 'fr-SN': 'africa', 'fr-TN': 'africa',
  'ha-NG': 'africa', 'ig-NG': 'africa', 'ln-CD': 'africa', 'mg-MG': 'africa',
  'ny-MW': 'africa', 'pt-AO': 'africa', 'pt-MZ': 'africa', 'rw-RW': 'africa',
  'sn-ZW': 'africa', 'so-SO': 'africa', 'sw-KE': 'africa', 'sw-TZ': 'africa',
  'wo-SN': 'africa', 'xh-ZA': 'africa', 'yo-NG': 'africa', 'zu-ZA': 'africa',
  // Oceania
  'en-AU': 'oceania', 'en-FJ': 'oceania', 'en-NZ': 'oceania', 'mi-NZ': 'oceania',
  // Middle East
  'ar-AE': 'middle-east', 'ar-BH': 'middle-east', 'ar-DZ': 'middle-east',
  'ar-EG': 'middle-east', 'ar-IQ': 'middle-east', 'ar-JO': 'middle-east',
  'ar-KW': 'middle-east', 'ar-LB': 'middle-east', 'ar-LY': 'middle-east',
  'ar-MA': 'middle-east', 'ar-OM': 'middle-east', 'ar-QA': 'middle-east',
  'ar-SA': 'middle-east', 'ar-TN': 'middle-east', 'en-AE': 'middle-east',
  'en-SA': 'middle-east', 'fa-IR': 'middle-east', 'he-IL': 'middle-east',
  'ku-TR': 'middle-east', 'ps-AF': 'middle-east', 'ru-IL': 'middle-east',
  'tr-TR': 'middle-east',
};

export const REGION_NAMES: Record<Region, string> = {
  'europe': 'Europe',
  'americas': 'Americas',
  'asia': 'Asia',
  'africa': 'Africa',
  'oceania': 'Oceania',
  'middle-east': 'Middle East',
};

/**
 * Get locale display info
 */
export function getLocaleInfo(code: string): { name: string; flag: string } {
  return LOCALE_METADATA[code] || { name: code, flag: '🌐' };
}

