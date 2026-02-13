// =============================================================================
// COUNTRY SEED - NovaNet v0.12.3
// =============================================================================
// Creates all 249 ISO 3166-1 countries/territories with UN M49 classification.
//
// Sources:
// - ISO 3166-1: Country codes (alpha-2, alpha-3)
// - UN M49: Geographic classification and numeric codes
// - lukes/ISO-3166-Countries-with-Regional-Codes (GitHub)
//
// Hierarchy: Continent → GeoRegion → Country → Locale
// =============================================================================

// Create Country nodes

MERGE (c:Country {
  key: 'AF',
  name: 'Afghanistan',
  alpha3: 'AFG',
  m49_code: 4,
  region: 'Asia',
  sub_region: 'Southern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Afghanistan market. TRIGGERS: afghanistan.'
});

MERGE (c:Country {
  key: 'AX',
  name: 'Åland Islands',
  alpha3: 'ALA',
  m49_code: 248,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Åland Islands market. TRIGGERS: åland islands, åland.'
});

MERGE (c:Country {
  key: 'AL',
  name: 'Albania',
  alpha3: 'ALB',
  m49_code: 8,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Albania market. TRIGGERS: albania.'
});

MERGE (c:Country {
  key: 'DZ',
  name: 'Algeria',
  alpha3: 'DZA',
  m49_code: 12,
  region: 'Africa',
  sub_region: 'Northern Africa',
  intermediate_region: '',
  llm_context: 'USE: for Algeria market. TRIGGERS: algeria.'
});

MERGE (c:Country {
  key: 'AS',
  name: 'American Samoa',
  alpha3: 'ASM',
  m49_code: 16,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for American Samoa market. TRIGGERS: american samoa, american.'
});

MERGE (c:Country {
  key: 'AD',
  name: 'Andorra',
  alpha3: 'AND',
  m49_code: 20,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Andorra market. TRIGGERS: andorra.'
});

MERGE (c:Country {
  key: 'AO',
  name: 'Angola',
  alpha3: 'AGO',
  m49_code: 24,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Middle Africa',
  llm_context: 'USE: for Angola market. TRIGGERS: angola.'
});

MERGE (c:Country {
  key: 'AI',
  name: 'Anguilla',
  alpha3: 'AIA',
  m49_code: 660,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Anguilla market. TRIGGERS: anguilla.'
});

MERGE (c:Country {
  key: 'AQ',
  name: 'Antarctica',
  alpha3: 'ATA',
  m49_code: 10,
  region: '',
  sub_region: '',
  intermediate_region: '',
  llm_context: 'USE: for Antarctica market. TRIGGERS: antarctica.'
});

MERGE (c:Country {
  key: 'AG',
  name: 'Antigua and Barbuda',
  alpha3: 'ATG',
  m49_code: 28,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Antigua and Barbuda market. TRIGGERS: antigua and barbuda, antigua.'
});

MERGE (c:Country {
  key: 'AR',
  name: 'Argentina',
  alpha3: 'ARG',
  m49_code: 32,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Argentina market. TRIGGERS: argentina.'
});

MERGE (c:Country {
  key: 'AM',
  name: 'Armenia',
  alpha3: 'ARM',
  m49_code: 51,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Armenia market. TRIGGERS: armenia.'
});

MERGE (c:Country {
  key: 'AW',
  name: 'Aruba',
  alpha3: 'ABW',
  m49_code: 533,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Aruba market. TRIGGERS: aruba.'
});

MERGE (c:Country {
  key: 'AU',
  name: 'Australia',
  alpha3: 'AUS',
  m49_code: 36,
  region: 'Oceania',
  sub_region: 'Australia and New Zealand',
  intermediate_region: '',
  llm_context: 'USE: for Australia market. TRIGGERS: australia.'
});

MERGE (c:Country {
  key: 'AT',
  name: 'Austria',
  alpha3: 'AUT',
  m49_code: 40,
  region: 'Europe',
  sub_region: 'Western Europe',
  intermediate_region: '',
  llm_context: 'USE: for Austria market. TRIGGERS: austria.'
});

MERGE (c:Country {
  key: 'AZ',
  name: 'Azerbaijan',
  alpha3: 'AZE',
  m49_code: 31,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Azerbaijan market. TRIGGERS: azerbaijan.'
});

MERGE (c:Country {
  key: 'BS',
  name: 'Bahamas',
  alpha3: 'BHS',
  m49_code: 44,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Bahamas market. TRIGGERS: bahamas.'
});

MERGE (c:Country {
  key: 'BH',
  name: 'Bahrain',
  alpha3: 'BHR',
  m49_code: 48,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Bahrain market. TRIGGERS: bahrain.'
});

MERGE (c:Country {
  key: 'BD',
  name: 'Bangladesh',
  alpha3: 'BGD',
  m49_code: 50,
  region: 'Asia',
  sub_region: 'Southern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Bangladesh market. TRIGGERS: bangladesh.'
});

MERGE (c:Country {
  key: 'BB',
  name: 'Barbados',
  alpha3: 'BRB',
  m49_code: 52,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Barbados market. TRIGGERS: barbados.'
});

MERGE (c:Country {
  key: 'BY',
  name: 'Belarus',
  alpha3: 'BLR',
  m49_code: 112,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Belarus market. TRIGGERS: belarus.'
});

MERGE (c:Country {
  key: 'BE',
  name: 'Belgium',
  alpha3: 'BEL',
  m49_code: 56,
  region: 'Europe',
  sub_region: 'Western Europe',
  intermediate_region: '',
  llm_context: 'USE: for Belgium market. TRIGGERS: belgium.'
});

MERGE (c:Country {
  key: 'BZ',
  name: 'Belize',
  alpha3: 'BLZ',
  m49_code: 84,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Central America',
  llm_context: 'USE: for Belize market. TRIGGERS: belize.'
});

MERGE (c:Country {
  key: 'BJ',
  name: 'Benin',
  alpha3: 'BEN',
  m49_code: 204,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Benin market. TRIGGERS: benin.'
});

MERGE (c:Country {
  key: 'BM',
  name: 'Bermuda',
  alpha3: 'BMU',
  m49_code: 60,
  region: 'Americas',
  sub_region: 'Northern America',
  intermediate_region: '',
  llm_context: 'USE: for Bermuda market. TRIGGERS: bermuda.'
});

MERGE (c:Country {
  key: 'BT',
  name: 'Bhutan',
  alpha3: 'BTN',
  m49_code: 64,
  region: 'Asia',
  sub_region: 'Southern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Bhutan market. TRIGGERS: bhutan.'
});

MERGE (c:Country {
  key: 'BO',
  name: 'Bolivia, Plurinational State of',
  alpha3: 'BOL',
  m49_code: 68,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Bolivia, Plurinational State of market. TRIGGERS: bolivia, plurinational state of, bolivia,.'
});

MERGE (c:Country {
  key: 'BQ',
  name: 'Bonaire, Sint Eustatius and Saba',
  alpha3: 'BES',
  m49_code: 535,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Bonaire, Sint Eustatius and Saba market. TRIGGERS: bonaire, sint eustatius and saba, bonaire,.'
});

MERGE (c:Country {
  key: 'BA',
  name: 'Bosnia and Herzegovina',
  alpha3: 'BIH',
  m49_code: 70,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Bosnia and Herzegovina market. TRIGGERS: bosnia and herzegovina, bosnia.'
});

MERGE (c:Country {
  key: 'BW',
  name: 'Botswana',
  alpha3: 'BWA',
  m49_code: 72,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Southern Africa',
  llm_context: 'USE: for Botswana market. TRIGGERS: botswana.'
});

MERGE (c:Country {
  key: 'BV',
  name: 'Bouvet Island',
  alpha3: 'BVT',
  m49_code: 74,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Bouvet Island market. TRIGGERS: bouvet island, bouvet.'
});

MERGE (c:Country {
  key: 'BR',
  name: 'Brazil',
  alpha3: 'BRA',
  m49_code: 76,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Brazil market. TRIGGERS: brazil.'
});

MERGE (c:Country {
  key: 'IO',
  name: 'British Indian Ocean Territory',
  alpha3: 'IOT',
  m49_code: 86,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for British Indian Ocean Territory market. TRIGGERS: british indian ocean territory, british.'
});

MERGE (c:Country {
  key: 'BN',
  name: 'Brunei Darussalam',
  alpha3: 'BRN',
  m49_code: 96,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Brunei Darussalam market. TRIGGERS: brunei darussalam, brunei.'
});

MERGE (c:Country {
  key: 'BG',
  name: 'Bulgaria',
  alpha3: 'BGR',
  m49_code: 100,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Bulgaria market. TRIGGERS: bulgaria.'
});

MERGE (c:Country {
  key: 'BF',
  name: 'Burkina Faso',
  alpha3: 'BFA',
  m49_code: 854,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Burkina Faso market. TRIGGERS: burkina faso, burkina.'
});

MERGE (c:Country {
  key: 'BI',
  name: 'Burundi',
  alpha3: 'BDI',
  m49_code: 108,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Burundi market. TRIGGERS: burundi.'
});

MERGE (c:Country {
  key: 'CV',
  name: 'Cabo Verde',
  alpha3: 'CPV',
  m49_code: 132,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Cabo Verde market. TRIGGERS: cabo verde, cabo.'
});

MERGE (c:Country {
  key: 'KH',
  name: 'Cambodia',
  alpha3: 'KHM',
  m49_code: 116,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Cambodia market. TRIGGERS: cambodia.'
});

MERGE (c:Country {
  key: 'CM',
  name: 'Cameroon',
  alpha3: 'CMR',
  m49_code: 120,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Middle Africa',
  llm_context: 'USE: for Cameroon market. TRIGGERS: cameroon.'
});

MERGE (c:Country {
  key: 'CA',
  name: 'Canada',
  alpha3: 'CAN',
  m49_code: 124,
  region: 'Americas',
  sub_region: 'Northern America',
  intermediate_region: '',
  llm_context: 'USE: for Canada market. TRIGGERS: canada.'
});

MERGE (c:Country {
  key: 'KY',
  name: 'Cayman Islands',
  alpha3: 'CYM',
  m49_code: 136,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Cayman Islands market. TRIGGERS: cayman islands, cayman.'
});

MERGE (c:Country {
  key: 'CF',
  name: 'Central African Republic',
  alpha3: 'CAF',
  m49_code: 140,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Middle Africa',
  llm_context: 'USE: for Central African Republic market. TRIGGERS: central african republic, central.'
});

MERGE (c:Country {
  key: 'TD',
  name: 'Chad',
  alpha3: 'TCD',
  m49_code: 148,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Middle Africa',
  llm_context: 'USE: for Chad market. TRIGGERS: chad.'
});

MERGE (c:Country {
  key: 'CL',
  name: 'Chile',
  alpha3: 'CHL',
  m49_code: 152,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Chile market. TRIGGERS: chile.'
});

MERGE (c:Country {
  key: 'CN',
  name: 'China',
  alpha3: 'CHN',
  m49_code: 156,
  region: 'Asia',
  sub_region: 'Eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for China market. TRIGGERS: china.'
});

MERGE (c:Country {
  key: 'CX',
  name: 'Christmas Island',
  alpha3: 'CXR',
  m49_code: 162,
  region: 'Oceania',
  sub_region: 'Australia and New Zealand',
  intermediate_region: '',
  llm_context: 'USE: for Christmas Island market. TRIGGERS: christmas island, christmas.'
});

MERGE (c:Country {
  key: 'CC',
  name: 'Cocos (Keeling) Islands',
  alpha3: 'CCK',
  m49_code: 166,
  region: 'Oceania',
  sub_region: 'Australia and New Zealand',
  intermediate_region: '',
  llm_context: 'USE: for Cocos (Keeling) Islands market. TRIGGERS: cocos (keeling) islands, cocos.'
});

MERGE (c:Country {
  key: 'CO',
  name: 'Colombia',
  alpha3: 'COL',
  m49_code: 170,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Colombia market. TRIGGERS: colombia.'
});

MERGE (c:Country {
  key: 'KM',
  name: 'Comoros',
  alpha3: 'COM',
  m49_code: 174,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Comoros market. TRIGGERS: comoros.'
});

MERGE (c:Country {
  key: 'CG',
  name: 'Congo',
  alpha3: 'COG',
  m49_code: 178,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Middle Africa',
  llm_context: 'USE: for Congo market. TRIGGERS: congo.'
});

MERGE (c:Country {
  key: 'CD',
  name: 'Congo, Democratic Republic of the',
  alpha3: 'COD',
  m49_code: 180,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Middle Africa',
  llm_context: 'USE: for Congo, Democratic Republic of the market. TRIGGERS: congo, democratic republic of the, congo,.'
});

MERGE (c:Country {
  key: 'CK',
  name: 'Cook Islands',
  alpha3: 'COK',
  m49_code: 184,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for Cook Islands market. TRIGGERS: cook islands, cook.'
});

MERGE (c:Country {
  key: 'CR',
  name: 'Costa Rica',
  alpha3: 'CRI',
  m49_code: 188,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Central America',
  llm_context: 'USE: for Costa Rica market. TRIGGERS: costa rica, costa.'
});

MERGE (c:Country {
  key: 'CI',
  name: 'Côte d\'Ivoire',
  alpha3: 'CIV',
  m49_code: 384,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Côte d\'Ivoire market. TRIGGERS: côte d\'ivoire, côte.'
});

MERGE (c:Country {
  key: 'HR',
  name: 'Croatia',
  alpha3: 'HRV',
  m49_code: 191,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Croatia market. TRIGGERS: croatia.'
});

MERGE (c:Country {
  key: 'CU',
  name: 'Cuba',
  alpha3: 'CUB',
  m49_code: 192,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Cuba market. TRIGGERS: cuba.'
});

MERGE (c:Country {
  key: 'CW',
  name: 'Curaçao',
  alpha3: 'CUW',
  m49_code: 531,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Curaçao market. TRIGGERS: curaçao.'
});

MERGE (c:Country {
  key: 'CY',
  name: 'Cyprus',
  alpha3: 'CYP',
  m49_code: 196,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Cyprus market. TRIGGERS: cyprus.'
});

MERGE (c:Country {
  key: 'CZ',
  name: 'Czechia',
  alpha3: 'CZE',
  m49_code: 203,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Czechia market. TRIGGERS: czechia.'
});

MERGE (c:Country {
  key: 'DK',
  name: 'Denmark',
  alpha3: 'DNK',
  m49_code: 208,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Denmark market. TRIGGERS: denmark.'
});

MERGE (c:Country {
  key: 'DJ',
  name: 'Djibouti',
  alpha3: 'DJI',
  m49_code: 262,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Djibouti market. TRIGGERS: djibouti.'
});

MERGE (c:Country {
  key: 'DM',
  name: 'Dominica',
  alpha3: 'DMA',
  m49_code: 212,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Dominica market. TRIGGERS: dominica.'
});

MERGE (c:Country {
  key: 'DO',
  name: 'Dominican Republic',
  alpha3: 'DOM',
  m49_code: 214,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Dominican Republic market. TRIGGERS: dominican republic, dominican.'
});

MERGE (c:Country {
  key: 'EC',
  name: 'Ecuador',
  alpha3: 'ECU',
  m49_code: 218,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Ecuador market. TRIGGERS: ecuador.'
});

MERGE (c:Country {
  key: 'EG',
  name: 'Egypt',
  alpha3: 'EGY',
  m49_code: 818,
  region: 'Africa',
  sub_region: 'Northern Africa',
  intermediate_region: '',
  llm_context: 'USE: for Egypt market. TRIGGERS: egypt.'
});

MERGE (c:Country {
  key: 'SV',
  name: 'El Salvador',
  alpha3: 'SLV',
  m49_code: 222,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Central America',
  llm_context: 'USE: for El Salvador market. TRIGGERS: el salvador, el.'
});

MERGE (c:Country {
  key: 'GQ',
  name: 'Equatorial Guinea',
  alpha3: 'GNQ',
  m49_code: 226,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Middle Africa',
  llm_context: 'USE: for Equatorial Guinea market. TRIGGERS: equatorial guinea, equatorial.'
});

MERGE (c:Country {
  key: 'ER',
  name: 'Eritrea',
  alpha3: 'ERI',
  m49_code: 232,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Eritrea market. TRIGGERS: eritrea.'
});

MERGE (c:Country {
  key: 'EE',
  name: 'Estonia',
  alpha3: 'EST',
  m49_code: 233,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Estonia market. TRIGGERS: estonia.'
});

MERGE (c:Country {
  key: 'SZ',
  name: 'Eswatini',
  alpha3: 'SWZ',
  m49_code: 748,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Southern Africa',
  llm_context: 'USE: for Eswatini market. TRIGGERS: eswatini.'
});

MERGE (c:Country {
  key: 'ET',
  name: 'Ethiopia',
  alpha3: 'ETH',
  m49_code: 231,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Ethiopia market. TRIGGERS: ethiopia.'
});

MERGE (c:Country {
  key: 'FK',
  name: 'Falkland Islands (Malvinas)',
  alpha3: 'FLK',
  m49_code: 238,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Falkland Islands (Malvinas) market. TRIGGERS: falkland islands (malvinas), falkland.'
});

MERGE (c:Country {
  key: 'FO',
  name: 'Faroe Islands',
  alpha3: 'FRO',
  m49_code: 234,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Faroe Islands market. TRIGGERS: faroe islands, faroe.'
});

MERGE (c:Country {
  key: 'FJ',
  name: 'Fiji',
  alpha3: 'FJI',
  m49_code: 242,
  region: 'Oceania',
  sub_region: 'Melanesia',
  intermediate_region: '',
  llm_context: 'USE: for Fiji market. TRIGGERS: fiji.'
});

MERGE (c:Country {
  key: 'FI',
  name: 'Finland',
  alpha3: 'FIN',
  m49_code: 246,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Finland market. TRIGGERS: finland.'
});

MERGE (c:Country {
  key: 'FR',
  name: 'France',
  alpha3: 'FRA',
  m49_code: 250,
  region: 'Europe',
  sub_region: 'Western Europe',
  intermediate_region: '',
  llm_context: 'USE: for France market. TRIGGERS: france.'
});

MERGE (c:Country {
  key: 'GF',
  name: 'French Guiana',
  alpha3: 'GUF',
  m49_code: 254,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for French Guiana market. TRIGGERS: french guiana, french.'
});

MERGE (c:Country {
  key: 'PF',
  name: 'French Polynesia',
  alpha3: 'PYF',
  m49_code: 258,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for French Polynesia market. TRIGGERS: french polynesia, french.'
});

MERGE (c:Country {
  key: 'TF',
  name: 'French Southern Territories',
  alpha3: 'ATF',
  m49_code: 260,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for French Southern Territories market. TRIGGERS: french southern territories, french.'
});

MERGE (c:Country {
  key: 'GA',
  name: 'Gabon',
  alpha3: 'GAB',
  m49_code: 266,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Middle Africa',
  llm_context: 'USE: for Gabon market. TRIGGERS: gabon.'
});

MERGE (c:Country {
  key: 'GM',
  name: 'Gambia',
  alpha3: 'GMB',
  m49_code: 270,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Gambia market. TRIGGERS: gambia.'
});

MERGE (c:Country {
  key: 'GE',
  name: 'Georgia',
  alpha3: 'GEO',
  m49_code: 268,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Georgia market. TRIGGERS: georgia.'
});

MERGE (c:Country {
  key: 'DE',
  name: 'Germany',
  alpha3: 'DEU',
  m49_code: 276,
  region: 'Europe',
  sub_region: 'Western Europe',
  intermediate_region: '',
  llm_context: 'USE: for Germany market. TRIGGERS: germany.'
});

MERGE (c:Country {
  key: 'GH',
  name: 'Ghana',
  alpha3: 'GHA',
  m49_code: 288,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Ghana market. TRIGGERS: ghana.'
});

MERGE (c:Country {
  key: 'GI',
  name: 'Gibraltar',
  alpha3: 'GIB',
  m49_code: 292,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Gibraltar market. TRIGGERS: gibraltar.'
});

MERGE (c:Country {
  key: 'GR',
  name: 'Greece',
  alpha3: 'GRC',
  m49_code: 300,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Greece market. TRIGGERS: greece.'
});

MERGE (c:Country {
  key: 'GL',
  name: 'Greenland',
  alpha3: 'GRL',
  m49_code: 304,
  region: 'Americas',
  sub_region: 'Northern America',
  intermediate_region: '',
  llm_context: 'USE: for Greenland market. TRIGGERS: greenland.'
});

MERGE (c:Country {
  key: 'GD',
  name: 'Grenada',
  alpha3: 'GRD',
  m49_code: 308,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Grenada market. TRIGGERS: grenada.'
});

MERGE (c:Country {
  key: 'GP',
  name: 'Guadeloupe',
  alpha3: 'GLP',
  m49_code: 312,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Guadeloupe market. TRIGGERS: guadeloupe.'
});

MERGE (c:Country {
  key: 'GU',
  name: 'Guam',
  alpha3: 'GUM',
  m49_code: 316,
  region: 'Oceania',
  sub_region: 'Micronesia',
  intermediate_region: '',
  llm_context: 'USE: for Guam market. TRIGGERS: guam.'
});

MERGE (c:Country {
  key: 'GT',
  name: 'Guatemala',
  alpha3: 'GTM',
  m49_code: 320,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Central America',
  llm_context: 'USE: for Guatemala market. TRIGGERS: guatemala.'
});

MERGE (c:Country {
  key: 'GG',
  name: 'Guernsey',
  alpha3: 'GGY',
  m49_code: 831,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Guernsey market. TRIGGERS: guernsey.'
});

MERGE (c:Country {
  key: 'GN',
  name: 'Guinea',
  alpha3: 'GIN',
  m49_code: 324,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Guinea market. TRIGGERS: guinea.'
});

MERGE (c:Country {
  key: 'GW',
  name: 'Guinea-Bissau',
  alpha3: 'GNB',
  m49_code: 624,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Guinea-Bissau market. TRIGGERS: guinea-bissau.'
});

MERGE (c:Country {
  key: 'GY',
  name: 'Guyana',
  alpha3: 'GUY',
  m49_code: 328,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Guyana market. TRIGGERS: guyana.'
});

MERGE (c:Country {
  key: 'HT',
  name: 'Haiti',
  alpha3: 'HTI',
  m49_code: 332,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Haiti market. TRIGGERS: haiti.'
});

MERGE (c:Country {
  key: 'HM',
  name: 'Heard Island and McDonald Islands',
  alpha3: 'HMD',
  m49_code: 334,
  region: 'Oceania',
  sub_region: 'Australia and New Zealand',
  intermediate_region: '',
  llm_context: 'USE: for Heard Island and McDonald Islands market. TRIGGERS: heard island and mcdonald islands, heard.'
});

MERGE (c:Country {
  key: 'VA',
  name: 'Holy See',
  alpha3: 'VAT',
  m49_code: 336,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Holy See market. TRIGGERS: holy see, holy.'
});

MERGE (c:Country {
  key: 'HN',
  name: 'Honduras',
  alpha3: 'HND',
  m49_code: 340,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Central America',
  llm_context: 'USE: for Honduras market. TRIGGERS: honduras.'
});

MERGE (c:Country {
  key: 'HK',
  name: 'Hong Kong',
  alpha3: 'HKG',
  m49_code: 344,
  region: 'Asia',
  sub_region: 'Eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Hong Kong market. TRIGGERS: hong kong, hong.'
});

MERGE (c:Country {
  key: 'HU',
  name: 'Hungary',
  alpha3: 'HUN',
  m49_code: 348,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Hungary market. TRIGGERS: hungary.'
});

MERGE (c:Country {
  key: 'IS',
  name: 'Iceland',
  alpha3: 'ISL',
  m49_code: 352,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Iceland market. TRIGGERS: iceland.'
});

MERGE (c:Country {
  key: 'IN',
  name: 'India',
  alpha3: 'IND',
  m49_code: 356,
  region: 'Asia',
  sub_region: 'Southern Asia',
  intermediate_region: '',
  llm_context: 'USE: for India market. TRIGGERS: india.'
});

MERGE (c:Country {
  key: 'ID',
  name: 'Indonesia',
  alpha3: 'IDN',
  m49_code: 360,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Indonesia market. TRIGGERS: indonesia.'
});

MERGE (c:Country {
  key: 'IR',
  name: 'Iran, Islamic Republic of',
  alpha3: 'IRN',
  m49_code: 364,
  region: 'Asia',
  sub_region: 'Southern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Iran, Islamic Republic of market. TRIGGERS: iran, islamic republic of, iran,.'
});

MERGE (c:Country {
  key: 'IQ',
  name: 'Iraq',
  alpha3: 'IRQ',
  m49_code: 368,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Iraq market. TRIGGERS: iraq.'
});

MERGE (c:Country {
  key: 'IE',
  name: 'Ireland',
  alpha3: 'IRL',
  m49_code: 372,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Ireland market. TRIGGERS: ireland.'
});

MERGE (c:Country {
  key: 'IM',
  name: 'Isle of Man',
  alpha3: 'IMN',
  m49_code: 833,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Isle of Man market. TRIGGERS: isle of man, isle.'
});

MERGE (c:Country {
  key: 'IL',
  name: 'Israel',
  alpha3: 'ISR',
  m49_code: 376,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Israel market. TRIGGERS: israel.'
});

MERGE (c:Country {
  key: 'IT',
  name: 'Italy',
  alpha3: 'ITA',
  m49_code: 380,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Italy market. TRIGGERS: italy.'
});

MERGE (c:Country {
  key: 'JM',
  name: 'Jamaica',
  alpha3: 'JAM',
  m49_code: 388,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Jamaica market. TRIGGERS: jamaica.'
});

MERGE (c:Country {
  key: 'JP',
  name: 'Japan',
  alpha3: 'JPN',
  m49_code: 392,
  region: 'Asia',
  sub_region: 'Eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Japan market. TRIGGERS: japan.'
});

MERGE (c:Country {
  key: 'JE',
  name: 'Jersey',
  alpha3: 'JEY',
  m49_code: 832,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Jersey market. TRIGGERS: jersey.'
});

MERGE (c:Country {
  key: 'JO',
  name: 'Jordan',
  alpha3: 'JOR',
  m49_code: 400,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Jordan market. TRIGGERS: jordan.'
});

MERGE (c:Country {
  key: 'KZ',
  name: 'Kazakhstan',
  alpha3: 'KAZ',
  m49_code: 398,
  region: 'Asia',
  sub_region: 'Central Asia',
  intermediate_region: '',
  llm_context: 'USE: for Kazakhstan market. TRIGGERS: kazakhstan.'
});

MERGE (c:Country {
  key: 'KE',
  name: 'Kenya',
  alpha3: 'KEN',
  m49_code: 404,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Kenya market. TRIGGERS: kenya.'
});

MERGE (c:Country {
  key: 'KI',
  name: 'Kiribati',
  alpha3: 'KIR',
  m49_code: 296,
  region: 'Oceania',
  sub_region: 'Micronesia',
  intermediate_region: '',
  llm_context: 'USE: for Kiribati market. TRIGGERS: kiribati.'
});

MERGE (c:Country {
  key: 'KP',
  name: 'Korea, Democratic People\'s Republic of',
  alpha3: 'PRK',
  m49_code: 408,
  region: 'Asia',
  sub_region: 'Eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Korea, Democratic People\'s Republic of market. TRIGGERS: korea, democratic people\'s republic of, korea,.'
});

MERGE (c:Country {
  key: 'KR',
  name: 'Korea, Republic of',
  alpha3: 'KOR',
  m49_code: 410,
  region: 'Asia',
  sub_region: 'Eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Korea, Republic of market. TRIGGERS: korea, republic of, korea,.'
});

MERGE (c:Country {
  key: 'KW',
  name: 'Kuwait',
  alpha3: 'KWT',
  m49_code: 414,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Kuwait market. TRIGGERS: kuwait.'
});

MERGE (c:Country {
  key: 'KG',
  name: 'Kyrgyzstan',
  alpha3: 'KGZ',
  m49_code: 417,
  region: 'Asia',
  sub_region: 'Central Asia',
  intermediate_region: '',
  llm_context: 'USE: for Kyrgyzstan market. TRIGGERS: kyrgyzstan.'
});

MERGE (c:Country {
  key: 'LA',
  name: 'Lao People\'s Democratic Republic',
  alpha3: 'LAO',
  m49_code: 418,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Lao People\'s Democratic Republic market. TRIGGERS: lao people\'s democratic republic, lao.'
});

MERGE (c:Country {
  key: 'LV',
  name: 'Latvia',
  alpha3: 'LVA',
  m49_code: 428,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Latvia market. TRIGGERS: latvia.'
});

MERGE (c:Country {
  key: 'LB',
  name: 'Lebanon',
  alpha3: 'LBN',
  m49_code: 422,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Lebanon market. TRIGGERS: lebanon.'
});

MERGE (c:Country {
  key: 'LS',
  name: 'Lesotho',
  alpha3: 'LSO',
  m49_code: 426,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Southern Africa',
  llm_context: 'USE: for Lesotho market. TRIGGERS: lesotho.'
});

MERGE (c:Country {
  key: 'LR',
  name: 'Liberia',
  alpha3: 'LBR',
  m49_code: 430,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Liberia market. TRIGGERS: liberia.'
});

MERGE (c:Country {
  key: 'LY',
  name: 'Libya',
  alpha3: 'LBY',
  m49_code: 434,
  region: 'Africa',
  sub_region: 'Northern Africa',
  intermediate_region: '',
  llm_context: 'USE: for Libya market. TRIGGERS: libya.'
});

MERGE (c:Country {
  key: 'LI',
  name: 'Liechtenstein',
  alpha3: 'LIE',
  m49_code: 438,
  region: 'Europe',
  sub_region: 'Western Europe',
  intermediate_region: '',
  llm_context: 'USE: for Liechtenstein market. TRIGGERS: liechtenstein.'
});

MERGE (c:Country {
  key: 'LT',
  name: 'Lithuania',
  alpha3: 'LTU',
  m49_code: 440,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Lithuania market. TRIGGERS: lithuania.'
});

MERGE (c:Country {
  key: 'LU',
  name: 'Luxembourg',
  alpha3: 'LUX',
  m49_code: 442,
  region: 'Europe',
  sub_region: 'Western Europe',
  intermediate_region: '',
  llm_context: 'USE: for Luxembourg market. TRIGGERS: luxembourg.'
});

MERGE (c:Country {
  key: 'MO',
  name: 'Macao',
  alpha3: 'MAC',
  m49_code: 446,
  region: 'Asia',
  sub_region: 'Eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Macao market. TRIGGERS: macao.'
});

MERGE (c:Country {
  key: 'MG',
  name: 'Madagascar',
  alpha3: 'MDG',
  m49_code: 450,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Madagascar market. TRIGGERS: madagascar.'
});

MERGE (c:Country {
  key: 'MW',
  name: 'Malawi',
  alpha3: 'MWI',
  m49_code: 454,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Malawi market. TRIGGERS: malawi.'
});

MERGE (c:Country {
  key: 'MY',
  name: 'Malaysia',
  alpha3: 'MYS',
  m49_code: 458,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Malaysia market. TRIGGERS: malaysia.'
});

MERGE (c:Country {
  key: 'MV',
  name: 'Maldives',
  alpha3: 'MDV',
  m49_code: 462,
  region: 'Asia',
  sub_region: 'Southern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Maldives market. TRIGGERS: maldives.'
});

MERGE (c:Country {
  key: 'ML',
  name: 'Mali',
  alpha3: 'MLI',
  m49_code: 466,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Mali market. TRIGGERS: mali.'
});

MERGE (c:Country {
  key: 'MT',
  name: 'Malta',
  alpha3: 'MLT',
  m49_code: 470,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Malta market. TRIGGERS: malta.'
});

MERGE (c:Country {
  key: 'MH',
  name: 'Marshall Islands',
  alpha3: 'MHL',
  m49_code: 584,
  region: 'Oceania',
  sub_region: 'Micronesia',
  intermediate_region: '',
  llm_context: 'USE: for Marshall Islands market. TRIGGERS: marshall islands, marshall.'
});

MERGE (c:Country {
  key: 'MQ',
  name: 'Martinique',
  alpha3: 'MTQ',
  m49_code: 474,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Martinique market. TRIGGERS: martinique.'
});

MERGE (c:Country {
  key: 'MR',
  name: 'Mauritania',
  alpha3: 'MRT',
  m49_code: 478,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Mauritania market. TRIGGERS: mauritania.'
});

MERGE (c:Country {
  key: 'MU',
  name: 'Mauritius',
  alpha3: 'MUS',
  m49_code: 480,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Mauritius market. TRIGGERS: mauritius.'
});

MERGE (c:Country {
  key: 'YT',
  name: 'Mayotte',
  alpha3: 'MYT',
  m49_code: 175,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Mayotte market. TRIGGERS: mayotte.'
});

MERGE (c:Country {
  key: 'MX',
  name: 'Mexico',
  alpha3: 'MEX',
  m49_code: 484,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Central America',
  llm_context: 'USE: for Mexico market. TRIGGERS: mexico.'
});

MERGE (c:Country {
  key: 'FM',
  name: 'Micronesia, Federated States of',
  alpha3: 'FSM',
  m49_code: 583,
  region: 'Oceania',
  sub_region: 'Micronesia',
  intermediate_region: '',
  llm_context: 'USE: for Micronesia, Federated States of market. TRIGGERS: micronesia, federated states of, micronesia,.'
});

MERGE (c:Country {
  key: 'MD',
  name: 'Moldova, Republic of',
  alpha3: 'MDA',
  m49_code: 498,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Moldova, Republic of market. TRIGGERS: moldova, republic of, moldova,.'
});

MERGE (c:Country {
  key: 'MC',
  name: 'Monaco',
  alpha3: 'MCO',
  m49_code: 492,
  region: 'Europe',
  sub_region: 'Western Europe',
  intermediate_region: '',
  llm_context: 'USE: for Monaco market. TRIGGERS: monaco.'
});

MERGE (c:Country {
  key: 'MN',
  name: 'Mongolia',
  alpha3: 'MNG',
  m49_code: 496,
  region: 'Asia',
  sub_region: 'Eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Mongolia market. TRIGGERS: mongolia.'
});

MERGE (c:Country {
  key: 'ME',
  name: 'Montenegro',
  alpha3: 'MNE',
  m49_code: 499,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Montenegro market. TRIGGERS: montenegro.'
});

MERGE (c:Country {
  key: 'MS',
  name: 'Montserrat',
  alpha3: 'MSR',
  m49_code: 500,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Montserrat market. TRIGGERS: montserrat.'
});

MERGE (c:Country {
  key: 'MA',
  name: 'Morocco',
  alpha3: 'MAR',
  m49_code: 504,
  region: 'Africa',
  sub_region: 'Northern Africa',
  intermediate_region: '',
  llm_context: 'USE: for Morocco market. TRIGGERS: morocco.'
});

MERGE (c:Country {
  key: 'MZ',
  name: 'Mozambique',
  alpha3: 'MOZ',
  m49_code: 508,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Mozambique market. TRIGGERS: mozambique.'
});

MERGE (c:Country {
  key: 'MM',
  name: 'Myanmar',
  alpha3: 'MMR',
  m49_code: 104,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Myanmar market. TRIGGERS: myanmar.'
});

MERGE (c:Country {
  key: 'NA',
  name: 'Namibia',
  alpha3: 'NAM',
  m49_code: 516,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Southern Africa',
  llm_context: 'USE: for Namibia market. TRIGGERS: namibia.'
});

MERGE (c:Country {
  key: 'NR',
  name: 'Nauru',
  alpha3: 'NRU',
  m49_code: 520,
  region: 'Oceania',
  sub_region: 'Micronesia',
  intermediate_region: '',
  llm_context: 'USE: for Nauru market. TRIGGERS: nauru.'
});

MERGE (c:Country {
  key: 'NP',
  name: 'Nepal',
  alpha3: 'NPL',
  m49_code: 524,
  region: 'Asia',
  sub_region: 'Southern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Nepal market. TRIGGERS: nepal.'
});

MERGE (c:Country {
  key: 'NL',
  name: 'Netherlands, Kingdom of the',
  alpha3: 'NLD',
  m49_code: 528,
  region: 'Europe',
  sub_region: 'Western Europe',
  intermediate_region: '',
  llm_context: 'USE: for Netherlands, Kingdom of the market. TRIGGERS: netherlands, kingdom of the, netherlands,.'
});

MERGE (c:Country {
  key: 'NC',
  name: 'New Caledonia',
  alpha3: 'NCL',
  m49_code: 540,
  region: 'Oceania',
  sub_region: 'Melanesia',
  intermediate_region: '',
  llm_context: 'USE: for New Caledonia market. TRIGGERS: new caledonia, new.'
});

MERGE (c:Country {
  key: 'NZ',
  name: 'New Zealand',
  alpha3: 'NZL',
  m49_code: 554,
  region: 'Oceania',
  sub_region: 'Australia and New Zealand',
  intermediate_region: '',
  llm_context: 'USE: for New Zealand market. TRIGGERS: new zealand, new.'
});

MERGE (c:Country {
  key: 'NI',
  name: 'Nicaragua',
  alpha3: 'NIC',
  m49_code: 558,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Central America',
  llm_context: 'USE: for Nicaragua market. TRIGGERS: nicaragua.'
});

MERGE (c:Country {
  key: 'NE',
  name: 'Niger',
  alpha3: 'NER',
  m49_code: 562,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Niger market. TRIGGERS: niger.'
});

MERGE (c:Country {
  key: 'NG',
  name: 'Nigeria',
  alpha3: 'NGA',
  m49_code: 566,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Nigeria market. TRIGGERS: nigeria.'
});

MERGE (c:Country {
  key: 'NU',
  name: 'Niue',
  alpha3: 'NIU',
  m49_code: 570,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for Niue market. TRIGGERS: niue.'
});

MERGE (c:Country {
  key: 'NF',
  name: 'Norfolk Island',
  alpha3: 'NFK',
  m49_code: 574,
  region: 'Oceania',
  sub_region: 'Australia and New Zealand',
  intermediate_region: '',
  llm_context: 'USE: for Norfolk Island market. TRIGGERS: norfolk island, norfolk.'
});

MERGE (c:Country {
  key: 'MK',
  name: 'North Macedonia',
  alpha3: 'MKD',
  m49_code: 807,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for North Macedonia market. TRIGGERS: north macedonia, north.'
});

MERGE (c:Country {
  key: 'MP',
  name: 'Northern Mariana Islands',
  alpha3: 'MNP',
  m49_code: 580,
  region: 'Oceania',
  sub_region: 'Micronesia',
  intermediate_region: '',
  llm_context: 'USE: for Northern Mariana Islands market. TRIGGERS: northern mariana islands, northern.'
});

MERGE (c:Country {
  key: 'NO',
  name: 'Norway',
  alpha3: 'NOR',
  m49_code: 578,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Norway market. TRIGGERS: norway.'
});

MERGE (c:Country {
  key: 'OM',
  name: 'Oman',
  alpha3: 'OMN',
  m49_code: 512,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Oman market. TRIGGERS: oman.'
});

MERGE (c:Country {
  key: 'PK',
  name: 'Pakistan',
  alpha3: 'PAK',
  m49_code: 586,
  region: 'Asia',
  sub_region: 'Southern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Pakistan market. TRIGGERS: pakistan.'
});

MERGE (c:Country {
  key: 'PW',
  name: 'Palau',
  alpha3: 'PLW',
  m49_code: 585,
  region: 'Oceania',
  sub_region: 'Micronesia',
  intermediate_region: '',
  llm_context: 'USE: for Palau market. TRIGGERS: palau.'
});

MERGE (c:Country {
  key: 'PS',
  name: 'Palestine, State of',
  alpha3: 'PSE',
  m49_code: 275,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Palestine, State of market. TRIGGERS: palestine, state of, palestine,.'
});

MERGE (c:Country {
  key: 'PA',
  name: 'Panama',
  alpha3: 'PAN',
  m49_code: 591,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Central America',
  llm_context: 'USE: for Panama market. TRIGGERS: panama.'
});

MERGE (c:Country {
  key: 'PG',
  name: 'Papua New Guinea',
  alpha3: 'PNG',
  m49_code: 598,
  region: 'Oceania',
  sub_region: 'Melanesia',
  intermediate_region: '',
  llm_context: 'USE: for Papua New Guinea market. TRIGGERS: papua new guinea, papua.'
});

MERGE (c:Country {
  key: 'PY',
  name: 'Paraguay',
  alpha3: 'PRY',
  m49_code: 600,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Paraguay market. TRIGGERS: paraguay.'
});

MERGE (c:Country {
  key: 'PE',
  name: 'Peru',
  alpha3: 'PER',
  m49_code: 604,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Peru market. TRIGGERS: peru.'
});

MERGE (c:Country {
  key: 'PH',
  name: 'Philippines',
  alpha3: 'PHL',
  m49_code: 608,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Philippines market. TRIGGERS: philippines.'
});

MERGE (c:Country {
  key: 'PN',
  name: 'Pitcairn',
  alpha3: 'PCN',
  m49_code: 612,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for Pitcairn market. TRIGGERS: pitcairn.'
});

MERGE (c:Country {
  key: 'PL',
  name: 'Poland',
  alpha3: 'POL',
  m49_code: 616,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Poland market. TRIGGERS: poland.'
});

MERGE (c:Country {
  key: 'PT',
  name: 'Portugal',
  alpha3: 'PRT',
  m49_code: 620,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Portugal market. TRIGGERS: portugal.'
});

MERGE (c:Country {
  key: 'PR',
  name: 'Puerto Rico',
  alpha3: 'PRI',
  m49_code: 630,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Puerto Rico market. TRIGGERS: puerto rico, puerto.'
});

MERGE (c:Country {
  key: 'QA',
  name: 'Qatar',
  alpha3: 'QAT',
  m49_code: 634,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Qatar market. TRIGGERS: qatar.'
});

MERGE (c:Country {
  key: 'RE',
  name: 'Réunion',
  alpha3: 'REU',
  m49_code: 638,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Réunion market. TRIGGERS: réunion.'
});

MERGE (c:Country {
  key: 'RO',
  name: 'Romania',
  alpha3: 'ROU',
  m49_code: 642,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Romania market. TRIGGERS: romania.'
});

MERGE (c:Country {
  key: 'RU',
  name: 'Russian Federation',
  alpha3: 'RUS',
  m49_code: 643,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Russian Federation market. TRIGGERS: russian federation, russian, russia, russian.'
});

MERGE (c:Country {
  key: 'RW',
  name: 'Rwanda',
  alpha3: 'RWA',
  m49_code: 646,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Rwanda market. TRIGGERS: rwanda.'
});

MERGE (c:Country {
  key: 'BL',
  name: 'Saint Barthélemy',
  alpha3: 'BLM',
  m49_code: 652,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Saint Barthélemy market. TRIGGERS: saint barthélemy, saint.'
});

MERGE (c:Country {
  key: 'SH',
  name: 'Saint Helena, Ascension and Tristan da Cunha',
  alpha3: 'SHN',
  m49_code: 654,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Saint Helena, Ascension and Tristan da Cunha market. TRIGGERS: saint helena, ascension and tristan da cunha, saint.'
});

MERGE (c:Country {
  key: 'KN',
  name: 'Saint Kitts and Nevis',
  alpha3: 'KNA',
  m49_code: 659,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Saint Kitts and Nevis market. TRIGGERS: saint kitts and nevis, saint.'
});

MERGE (c:Country {
  key: 'LC',
  name: 'Saint Lucia',
  alpha3: 'LCA',
  m49_code: 662,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Saint Lucia market. TRIGGERS: saint lucia, saint.'
});

MERGE (c:Country {
  key: 'MF',
  name: 'Saint Martin (French part)',
  alpha3: 'MAF',
  m49_code: 663,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Saint Martin (French part) market. TRIGGERS: saint martin (french part), saint.'
});

MERGE (c:Country {
  key: 'PM',
  name: 'Saint Pierre and Miquelon',
  alpha3: 'SPM',
  m49_code: 666,
  region: 'Americas',
  sub_region: 'Northern America',
  intermediate_region: '',
  llm_context: 'USE: for Saint Pierre and Miquelon market. TRIGGERS: saint pierre and miquelon, saint.'
});

MERGE (c:Country {
  key: 'VC',
  name: 'Saint Vincent and the Grenadines',
  alpha3: 'VCT',
  m49_code: 670,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Saint Vincent and the Grenadines market. TRIGGERS: saint vincent and the grenadines, saint.'
});

MERGE (c:Country {
  key: 'WS',
  name: 'Samoa',
  alpha3: 'WSM',
  m49_code: 882,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for Samoa market. TRIGGERS: samoa.'
});

MERGE (c:Country {
  key: 'SM',
  name: 'San Marino',
  alpha3: 'SMR',
  m49_code: 674,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for San Marino market. TRIGGERS: san marino, san.'
});

MERGE (c:Country {
  key: 'ST',
  name: 'Sao Tome and Principe',
  alpha3: 'STP',
  m49_code: 678,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Middle Africa',
  llm_context: 'USE: for Sao Tome and Principe market. TRIGGERS: sao tome and principe, sao.'
});

MERGE (c:Country {
  key: 'SA',
  name: 'Saudi Arabia',
  alpha3: 'SAU',
  m49_code: 682,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Saudi Arabia market. TRIGGERS: saudi arabia, saudi.'
});

MERGE (c:Country {
  key: 'SN',
  name: 'Senegal',
  alpha3: 'SEN',
  m49_code: 686,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Senegal market. TRIGGERS: senegal.'
});

MERGE (c:Country {
  key: 'RS',
  name: 'Serbia',
  alpha3: 'SRB',
  m49_code: 688,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Serbia market. TRIGGERS: serbia.'
});

MERGE (c:Country {
  key: 'SC',
  name: 'Seychelles',
  alpha3: 'SYC',
  m49_code: 690,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Seychelles market. TRIGGERS: seychelles.'
});

MERGE (c:Country {
  key: 'SL',
  name: 'Sierra Leone',
  alpha3: 'SLE',
  m49_code: 694,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Sierra Leone market. TRIGGERS: sierra leone, sierra.'
});

MERGE (c:Country {
  key: 'SG',
  name: 'Singapore',
  alpha3: 'SGP',
  m49_code: 702,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Singapore market. TRIGGERS: singapore.'
});

MERGE (c:Country {
  key: 'SX',
  name: 'Sint Maarten (Dutch part)',
  alpha3: 'SXM',
  m49_code: 534,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Sint Maarten (Dutch part) market. TRIGGERS: sint maarten (dutch part), sint.'
});

MERGE (c:Country {
  key: 'SK',
  name: 'Slovakia',
  alpha3: 'SVK',
  m49_code: 703,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Slovakia market. TRIGGERS: slovakia.'
});

MERGE (c:Country {
  key: 'SI',
  name: 'Slovenia',
  alpha3: 'SVN',
  m49_code: 705,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Slovenia market. TRIGGERS: slovenia.'
});

MERGE (c:Country {
  key: 'SB',
  name: 'Solomon Islands',
  alpha3: 'SLB',
  m49_code: 90,
  region: 'Oceania',
  sub_region: 'Melanesia',
  intermediate_region: '',
  llm_context: 'USE: for Solomon Islands market. TRIGGERS: solomon islands, solomon.'
});

MERGE (c:Country {
  key: 'SO',
  name: 'Somalia',
  alpha3: 'SOM',
  m49_code: 706,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Somalia market. TRIGGERS: somalia.'
});

MERGE (c:Country {
  key: 'ZA',
  name: 'South Africa',
  alpha3: 'ZAF',
  m49_code: 710,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Southern Africa',
  llm_context: 'USE: for South Africa market. TRIGGERS: south africa, south.'
});

MERGE (c:Country {
  key: 'GS',
  name: 'South Georgia and the South Sandwich Islands',
  alpha3: 'SGS',
  m49_code: 239,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for South Georgia and the South Sandwich Islands market. TRIGGERS: south georgia and the south sandwich islands, south.'
});

MERGE (c:Country {
  key: 'SS',
  name: 'South Sudan',
  alpha3: 'SSD',
  m49_code: 728,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for South Sudan market. TRIGGERS: south sudan, south.'
});

MERGE (c:Country {
  key: 'ES',
  name: 'Spain',
  alpha3: 'ESP',
  m49_code: 724,
  region: 'Europe',
  sub_region: 'Southern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Spain market. TRIGGERS: spain.'
});

MERGE (c:Country {
  key: 'LK',
  name: 'Sri Lanka',
  alpha3: 'LKA',
  m49_code: 144,
  region: 'Asia',
  sub_region: 'Southern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Sri Lanka market. TRIGGERS: sri lanka, sri.'
});

MERGE (c:Country {
  key: 'SD',
  name: 'Sudan',
  alpha3: 'SDN',
  m49_code: 729,
  region: 'Africa',
  sub_region: 'Northern Africa',
  intermediate_region: '',
  llm_context: 'USE: for Sudan market. TRIGGERS: sudan.'
});

MERGE (c:Country {
  key: 'SR',
  name: 'Suriname',
  alpha3: 'SUR',
  m49_code: 740,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Suriname market. TRIGGERS: suriname.'
});

MERGE (c:Country {
  key: 'SJ',
  name: 'Svalbard and Jan Mayen',
  alpha3: 'SJM',
  m49_code: 744,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Svalbard and Jan Mayen market. TRIGGERS: svalbard and jan mayen, svalbard.'
});

MERGE (c:Country {
  key: 'SE',
  name: 'Sweden',
  alpha3: 'SWE',
  m49_code: 752,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Sweden market. TRIGGERS: sweden.'
});

MERGE (c:Country {
  key: 'CH',
  name: 'Switzerland',
  alpha3: 'CHE',
  m49_code: 756,
  region: 'Europe',
  sub_region: 'Western Europe',
  intermediate_region: '',
  llm_context: 'USE: for Switzerland market. TRIGGERS: switzerland.'
});

MERGE (c:Country {
  key: 'SY',
  name: 'Syrian Arab Republic',
  alpha3: 'SYR',
  m49_code: 760,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Syrian Arab Republic market. TRIGGERS: syrian arab republic, syrian.'
});

MERGE (c:Country {
  key: 'TW',
  name: 'Taiwan, Province of China',
  alpha3: 'TWN',
  m49_code: 158,
  region: '',
  sub_region: '',
  intermediate_region: '',
  llm_context: 'USE: for Taiwan, Province of China market. TRIGGERS: taiwan, province of china, taiwan,.'
});

MERGE (c:Country {
  key: 'TJ',
  name: 'Tajikistan',
  alpha3: 'TJK',
  m49_code: 762,
  region: 'Asia',
  sub_region: 'Central Asia',
  intermediate_region: '',
  llm_context: 'USE: for Tajikistan market. TRIGGERS: tajikistan.'
});

MERGE (c:Country {
  key: 'TZ',
  name: 'Tanzania, United Republic of',
  alpha3: 'TZA',
  m49_code: 834,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Tanzania, United Republic of market. TRIGGERS: tanzania, united republic of, tanzania,.'
});

MERGE (c:Country {
  key: 'TH',
  name: 'Thailand',
  alpha3: 'THA',
  m49_code: 764,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Thailand market. TRIGGERS: thailand.'
});

MERGE (c:Country {
  key: 'TL',
  name: 'Timor-Leste',
  alpha3: 'TLS',
  m49_code: 626,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Timor-Leste market. TRIGGERS: timor-leste.'
});

MERGE (c:Country {
  key: 'TG',
  name: 'Togo',
  alpha3: 'TGO',
  m49_code: 768,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Western Africa',
  llm_context: 'USE: for Togo market. TRIGGERS: togo.'
});

MERGE (c:Country {
  key: 'TK',
  name: 'Tokelau',
  alpha3: 'TKL',
  m49_code: 772,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for Tokelau market. TRIGGERS: tokelau.'
});

MERGE (c:Country {
  key: 'TO',
  name: 'Tonga',
  alpha3: 'TON',
  m49_code: 776,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for Tonga market. TRIGGERS: tonga.'
});

MERGE (c:Country {
  key: 'TT',
  name: 'Trinidad and Tobago',
  alpha3: 'TTO',
  m49_code: 780,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Trinidad and Tobago market. TRIGGERS: trinidad and tobago, trinidad.'
});

MERGE (c:Country {
  key: 'TN',
  name: 'Tunisia',
  alpha3: 'TUN',
  m49_code: 788,
  region: 'Africa',
  sub_region: 'Northern Africa',
  intermediate_region: '',
  llm_context: 'USE: for Tunisia market. TRIGGERS: tunisia.'
});

MERGE (c:Country {
  key: 'TR',
  name: 'Türkiye',
  alpha3: 'TUR',
  m49_code: 792,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Türkiye market. TRIGGERS: türkiye.'
});

MERGE (c:Country {
  key: 'TM',
  name: 'Turkmenistan',
  alpha3: 'TKM',
  m49_code: 795,
  region: 'Asia',
  sub_region: 'Central Asia',
  intermediate_region: '',
  llm_context: 'USE: for Turkmenistan market. TRIGGERS: turkmenistan.'
});

MERGE (c:Country {
  key: 'TC',
  name: 'Turks and Caicos Islands',
  alpha3: 'TCA',
  m49_code: 796,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Turks and Caicos Islands market. TRIGGERS: turks and caicos islands, turks.'
});

MERGE (c:Country {
  key: 'TV',
  name: 'Tuvalu',
  alpha3: 'TUV',
  m49_code: 798,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for Tuvalu market. TRIGGERS: tuvalu.'
});

MERGE (c:Country {
  key: 'UG',
  name: 'Uganda',
  alpha3: 'UGA',
  m49_code: 800,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Uganda market. TRIGGERS: uganda.'
});

MERGE (c:Country {
  key: 'UA',
  name: 'Ukraine',
  alpha3: 'UKR',
  m49_code: 804,
  region: 'Europe',
  sub_region: 'Eastern Europe',
  intermediate_region: '',
  llm_context: 'USE: for Ukraine market. TRIGGERS: ukraine.'
});

MERGE (c:Country {
  key: 'AE',
  name: 'United Arab Emirates',
  alpha3: 'ARE',
  m49_code: 784,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for United Arab Emirates market. TRIGGERS: united arab emirates, united.'
});

MERGE (c:Country {
  key: 'GB',
  name: 'United Kingdom of Great Britain and Northern Ireland',
  alpha3: 'GBR',
  m49_code: 826,
  region: 'Europe',
  sub_region: 'Northern Europe',
  intermediate_region: '',
  llm_context: 'USE: for United Kingdom of Great Britain and Northern Ireland market. TRIGGERS: united kingdom of great britain and northern ireland, united.'
});

MERGE (c:Country {
  key: 'US',
  name: 'United States of America',
  alpha3: 'USA',
  m49_code: 840,
  region: 'Americas',
  sub_region: 'Northern America',
  intermediate_region: '',
  llm_context: 'USE: for United States of America market. TRIGGERS: united states of america, united, usa, us.'
});

MERGE (c:Country {
  key: 'UM',
  name: 'United States Minor Outlying Islands',
  alpha3: 'UMI',
  m49_code: 581,
  region: 'Oceania',
  sub_region: 'Micronesia',
  intermediate_region: '',
  llm_context: 'USE: for United States Minor Outlying Islands market. TRIGGERS: united states minor outlying islands, united.'
});

MERGE (c:Country {
  key: 'UY',
  name: 'Uruguay',
  alpha3: 'URY',
  m49_code: 858,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Uruguay market. TRIGGERS: uruguay.'
});

MERGE (c:Country {
  key: 'UZ',
  name: 'Uzbekistan',
  alpha3: 'UZB',
  m49_code: 860,
  region: 'Asia',
  sub_region: 'Central Asia',
  intermediate_region: '',
  llm_context: 'USE: for Uzbekistan market. TRIGGERS: uzbekistan.'
});

MERGE (c:Country {
  key: 'VU',
  name: 'Vanuatu',
  alpha3: 'VUT',
  m49_code: 548,
  region: 'Oceania',
  sub_region: 'Melanesia',
  intermediate_region: '',
  llm_context: 'USE: for Vanuatu market. TRIGGERS: vanuatu.'
});

MERGE (c:Country {
  key: 'VE',
  name: 'Venezuela, Bolivarian Republic of',
  alpha3: 'VEN',
  m49_code: 862,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'South America',
  llm_context: 'USE: for Venezuela, Bolivarian Republic of market. TRIGGERS: venezuela, bolivarian republic of, venezuela,.'
});

MERGE (c:Country {
  key: 'VN',
  name: 'Viet Nam',
  alpha3: 'VNM',
  m49_code: 704,
  region: 'Asia',
  sub_region: 'South-eastern Asia',
  intermediate_region: '',
  llm_context: 'USE: for Viet Nam market. TRIGGERS: viet nam, viet.'
});

MERGE (c:Country {
  key: 'VG',
  name: 'Virgin Islands (British)',
  alpha3: 'VGB',
  m49_code: 92,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Virgin Islands (British) market. TRIGGERS: virgin islands (british), virgin.'
});

MERGE (c:Country {
  key: 'VI',
  name: 'Virgin Islands (U.S.)',
  alpha3: 'VIR',
  m49_code: 850,
  region: 'Americas',
  sub_region: 'Latin America and the Caribbean',
  intermediate_region: 'Caribbean',
  llm_context: 'USE: for Virgin Islands (U.S.) market. TRIGGERS: virgin islands (u.s.), virgin.'
});

MERGE (c:Country {
  key: 'WF',
  name: 'Wallis and Futuna',
  alpha3: 'WLF',
  m49_code: 876,
  region: 'Oceania',
  sub_region: 'Polynesia',
  intermediate_region: '',
  llm_context: 'USE: for Wallis and Futuna market. TRIGGERS: wallis and futuna, wallis.'
});

MERGE (c:Country {
  key: 'EH',
  name: 'Western Sahara',
  alpha3: 'ESH',
  m49_code: 732,
  region: 'Africa',
  sub_region: 'Northern Africa',
  intermediate_region: '',
  llm_context: 'USE: for Western Sahara market. TRIGGERS: western sahara, western.'
});

MERGE (c:Country {
  key: 'YE',
  name: 'Yemen',
  alpha3: 'YEM',
  m49_code: 887,
  region: 'Asia',
  sub_region: 'Western Asia',
  intermediate_region: '',
  llm_context: 'USE: for Yemen market. TRIGGERS: yemen.'
});

MERGE (c:Country {
  key: 'ZM',
  name: 'Zambia',
  alpha3: 'ZMB',
  m49_code: 894,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Zambia market. TRIGGERS: zambia.'
});

MERGE (c:Country {
  key: 'ZW',
  name: 'Zimbabwe',
  alpha3: 'ZWE',
  m49_code: 716,
  region: 'Africa',
  sub_region: 'Sub-Saharan Africa',
  intermediate_region: 'Eastern Africa',
  llm_context: 'USE: for Zimbabwe market. TRIGGERS: zimbabwe.'
});


// =============================================================================
// COUNTRY → GEOREGION RELATIONSHIPS
// =============================================================================

MATCH (c:Country {key: 'AF'}), (r:GeoRegion {key: 'southern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AF'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AX'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AX'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AL'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AL'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'DZ'}), (r:GeoRegion {key: 'northern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'DZ'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AS'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AS'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AD'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AD'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AO'}), (r:GeoRegion {key: 'middle-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AO'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AI'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AI'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AG'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AG'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AR'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AR'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AM'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AM'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AW'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AW'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AU'}), (r:GeoRegion {key: 'australia-new-zealand'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AU'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AT'}), (r:GeoRegion {key: 'western-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AT'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AZ'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AZ'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BS'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BS'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BH'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BH'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BD'}), (r:GeoRegion {key: 'southern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BD'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BB'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BB'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BY'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BY'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BE'}), (r:GeoRegion {key: 'western-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BE'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BZ'}), (r:GeoRegion {key: 'central-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BZ'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BJ'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BJ'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BM'}), (r:GeoRegion {key: 'northern-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BM'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BT'}), (r:GeoRegion {key: 'southern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BT'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BO'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BO'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BQ'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BQ'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BA'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BA'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BW'}), (r:GeoRegion {key: 'southern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BW'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BV'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BV'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BR'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BR'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'IO'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'IO'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BN'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BN'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BG'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BG'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BF'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BF'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BI'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BI'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CV'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CV'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KH'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KH'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CM'}), (r:GeoRegion {key: 'middle-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CM'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CA'}), (r:GeoRegion {key: 'northern-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CA'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KY'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KY'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CF'}), (r:GeoRegion {key: 'middle-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CF'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TD'}), (r:GeoRegion {key: 'middle-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TD'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CL'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CL'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CN'}), (r:GeoRegion {key: 'eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CN'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CX'}), (r:GeoRegion {key: 'australia-new-zealand'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CX'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CC'}), (r:GeoRegion {key: 'australia-new-zealand'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CC'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CO'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CO'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KM'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KM'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CG'}), (r:GeoRegion {key: 'middle-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CG'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CD'}), (r:GeoRegion {key: 'middle-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CD'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CK'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CK'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CR'}), (r:GeoRegion {key: 'central-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CR'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CI'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CI'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'HR'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'HR'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CU'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CU'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CW'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CW'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CY'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CY'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CZ'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CZ'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'DK'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'DK'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'DJ'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'DJ'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'DM'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'DM'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'DO'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'DO'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'EC'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'EC'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'EG'}), (r:GeoRegion {key: 'northern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'EG'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SV'}), (r:GeoRegion {key: 'central-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SV'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GQ'}), (r:GeoRegion {key: 'middle-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GQ'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ER'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ER'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'EE'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'EE'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SZ'}), (r:GeoRegion {key: 'southern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SZ'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ET'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ET'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'FK'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'FK'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'FO'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'FO'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'FJ'}), (r:GeoRegion {key: 'melanesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'FJ'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'FI'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'FI'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'FR'}), (r:GeoRegion {key: 'western-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'FR'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GF'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GF'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PF'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PF'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TF'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TF'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GA'}), (r:GeoRegion {key: 'middle-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GA'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GM'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GM'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GE'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GE'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'DE'}), (r:GeoRegion {key: 'western-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'DE'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GH'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GH'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GI'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GI'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GR'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GR'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GL'}), (r:GeoRegion {key: 'northern-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GL'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GD'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GD'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GP'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GP'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GU'}), (r:GeoRegion {key: 'micronesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GU'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GT'}), (r:GeoRegion {key: 'central-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GT'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GG'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GG'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GN'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GN'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GW'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GW'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GY'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GY'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'HT'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'HT'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'HM'}), (r:GeoRegion {key: 'australia-new-zealand'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'HM'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'VA'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'VA'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'HN'}), (r:GeoRegion {key: 'central-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'HN'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'HK'}), (r:GeoRegion {key: 'eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'HK'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'HU'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'HU'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'IS'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'IS'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'IN'}), (r:GeoRegion {key: 'southern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'IN'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ID'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ID'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'IR'}), (r:GeoRegion {key: 'southern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'IR'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'IQ'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'IQ'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'IE'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'IE'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'IM'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'IM'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'IL'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'IL'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'IT'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'IT'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'JM'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'JM'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'JP'}), (r:GeoRegion {key: 'eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'JP'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'JE'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'JE'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'JO'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'JO'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KZ'}), (r:GeoRegion {key: 'central-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KZ'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KE'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KE'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KI'}), (r:GeoRegion {key: 'micronesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KI'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KP'}), (r:GeoRegion {key: 'eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KP'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KR'}), (r:GeoRegion {key: 'eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KR'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KW'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KW'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KG'}), (r:GeoRegion {key: 'central-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KG'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LA'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LA'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LV'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LV'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LB'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LB'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LS'}), (r:GeoRegion {key: 'southern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LS'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LR'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LR'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LY'}), (r:GeoRegion {key: 'northern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LY'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LI'}), (r:GeoRegion {key: 'western-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LI'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LT'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LT'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LU'}), (r:GeoRegion {key: 'western-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LU'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MO'}), (r:GeoRegion {key: 'eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MO'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MG'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MG'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MW'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MW'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MY'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MY'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MV'}), (r:GeoRegion {key: 'southern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MV'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ML'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ML'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MT'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MT'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MH'}), (r:GeoRegion {key: 'micronesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MH'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MQ'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MQ'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MR'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MR'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MU'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MU'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'YT'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'YT'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MX'}), (r:GeoRegion {key: 'central-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MX'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'FM'}), (r:GeoRegion {key: 'micronesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'FM'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MD'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MD'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MC'}), (r:GeoRegion {key: 'western-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MC'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MN'}), (r:GeoRegion {key: 'eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MN'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ME'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ME'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MS'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MS'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MA'}), (r:GeoRegion {key: 'northern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MA'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MZ'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MZ'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MM'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MM'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NA'}), (r:GeoRegion {key: 'southern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NA'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NR'}), (r:GeoRegion {key: 'micronesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NR'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NP'}), (r:GeoRegion {key: 'southern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NP'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NL'}), (r:GeoRegion {key: 'western-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NL'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NC'}), (r:GeoRegion {key: 'melanesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NC'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NZ'}), (r:GeoRegion {key: 'australia-new-zealand'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NZ'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NI'}), (r:GeoRegion {key: 'central-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NI'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NE'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NE'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NG'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NG'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NU'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NU'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NF'}), (r:GeoRegion {key: 'australia-new-zealand'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NF'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MK'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MK'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MP'}), (r:GeoRegion {key: 'micronesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MP'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'NO'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'NO'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'OM'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'OM'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PK'}), (r:GeoRegion {key: 'southern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PK'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PW'}), (r:GeoRegion {key: 'micronesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PW'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PS'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PS'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PA'}), (r:GeoRegion {key: 'central-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PA'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PG'}), (r:GeoRegion {key: 'melanesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PG'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PY'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PY'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PE'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PE'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PH'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PH'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PN'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PN'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PL'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PL'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PT'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PT'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PR'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PR'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'QA'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'QA'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'RE'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'RE'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'RO'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'RO'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'RU'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'RU'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'RW'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'RW'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'BL'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'BL'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SH'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SH'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'KN'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'KN'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LC'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LC'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'MF'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'MF'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'PM'}), (r:GeoRegion {key: 'northern-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'PM'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'VC'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'VC'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'WS'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'WS'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SM'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SM'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ST'}), (r:GeoRegion {key: 'middle-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ST'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SA'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SA'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SN'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SN'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'RS'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'RS'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SC'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SC'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SL'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SL'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SG'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SG'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SX'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SX'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SK'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SK'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SI'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SI'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SB'}), (r:GeoRegion {key: 'melanesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SB'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SO'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SO'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ZA'}), (r:GeoRegion {key: 'southern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ZA'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GS'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GS'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SS'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SS'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ES'}), (r:GeoRegion {key: 'southern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ES'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'LK'}), (r:GeoRegion {key: 'southern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'LK'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SD'}), (r:GeoRegion {key: 'northern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SD'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SR'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SR'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SJ'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SJ'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SE'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SE'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'CH'}), (r:GeoRegion {key: 'western-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'CH'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'SY'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'SY'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TJ'}), (r:GeoRegion {key: 'central-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TJ'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TZ'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TZ'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TH'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TH'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TL'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TL'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TG'}), (r:GeoRegion {key: 'western-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TG'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TK'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TK'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TO'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TO'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TT'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TT'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TN'}), (r:GeoRegion {key: 'northern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TN'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TR'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TR'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TM'}), (r:GeoRegion {key: 'central-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TM'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TC'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TC'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'TV'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'TV'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'UG'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'UG'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'UA'}), (r:GeoRegion {key: 'eastern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'UA'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'AE'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'AE'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'GB'}), (r:GeoRegion {key: 'northern-europe'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'GB'}), (cont:Continent {key: 'europe'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'US'}), (r:GeoRegion {key: 'northern-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'US'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'UM'}), (r:GeoRegion {key: 'micronesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'UM'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'UY'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'UY'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'UZ'}), (r:GeoRegion {key: 'central-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'UZ'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'VU'}), (r:GeoRegion {key: 'melanesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'VU'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'VE'}), (r:GeoRegion {key: 'south-america'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'VE'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'VN'}), (r:GeoRegion {key: 'south-eastern-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'VN'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'VG'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'VG'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'VI'}), (r:GeoRegion {key: 'caribbean'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'VI'}), (cont:Continent {key: 'americas'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'WF'}), (r:GeoRegion {key: 'polynesia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'WF'}), (cont:Continent {key: 'oceania'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'EH'}), (r:GeoRegion {key: 'northern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'EH'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'YE'}), (r:GeoRegion {key: 'western-asia'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'YE'}), (cont:Continent {key: 'asia'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ZM'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ZM'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
MATCH (c:Country {key: 'ZW'}), (r:GeoRegion {key: 'eastern-africa'}) MERGE (c)-[:IN_REGION]->(r);
MATCH (c:Country {key: 'ZW'}), (cont:Continent {key: 'africa'}) MERGE (c)-[:IN_CONTINENT]->(cont);
