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

MERGE (c:Country {key: 'AF'})
SET c.display_name = 'Afghanistan',
    c.alpha3 = 'AFG',
    c.m49_code = 4,
    c.region = 'Asia',
    c.sub_region = 'Southern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Afghanistan market. TRIGGERS: afghanistan.',
    c.description = 'Country node for Afghanistan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AX'})
SET c.display_name = 'Åland Islands',
    c.alpha3 = 'ALA',
    c.m49_code = 248,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Åland Islands market. TRIGGERS: åland islands, åland.',
    c.description = 'Country node for Åland Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AL'})
SET c.display_name = 'Albania',
    c.alpha3 = 'ALB',
    c.m49_code = 8,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Albania market. TRIGGERS: albania.',
    c.description = 'Country node for Albania',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'DZ'})
SET c.display_name = 'Algeria',
    c.alpha3 = 'DZA',
    c.m49_code = 12,
    c.region = 'Africa',
    c.sub_region = 'Northern Africa',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Algeria market. TRIGGERS: algeria.',
    c.description = 'Country node for Algeria',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AS'})
SET c.display_name = 'American Samoa',
    c.alpha3 = 'ASM',
    c.m49_code = 16,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for American Samoa market. TRIGGERS: american samoa, american.',
    c.description = 'Country node for American Samoa',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AD'})
SET c.display_name = 'Andorra',
    c.alpha3 = 'AND',
    c.m49_code = 20,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Andorra market. TRIGGERS: andorra.',
    c.description = 'Country node for Andorra',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AO'})
SET c.display_name = 'Angola',
    c.alpha3 = 'AGO',
    c.m49_code = 24,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Middle Africa',
    c.llm_context = 'USE: for Angola market. TRIGGERS: angola.',
    c.description = 'Country node for Angola',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AI'})
SET c.display_name = 'Anguilla',
    c.alpha3 = 'AIA',
    c.m49_code = 660,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Anguilla market. TRIGGERS: anguilla.',
    c.description = 'Country node for Anguilla',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AQ'})
SET c.display_name = 'Antarctica',
    c.alpha3 = 'ATA',
    c.m49_code = 10,
    c.region = '',
    c.sub_region = '',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Antarctica market. TRIGGERS: antarctica.',
    c.description = 'Country node for Antarctica',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AG'})
SET c.display_name = 'Antigua and Barbuda',
    c.alpha3 = 'ATG',
    c.m49_code = 28,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Antigua and Barbuda market. TRIGGERS: antigua and barbuda, antigua.',
    c.description = 'Country node for Antigua and Barbuda',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AR'})
SET c.display_name = 'Argentina',
    c.alpha3 = 'ARG',
    c.m49_code = 32,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Argentina market. TRIGGERS: argentina.',
    c.description = 'Country node for Argentina',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AM'})
SET c.display_name = 'Armenia',
    c.alpha3 = 'ARM',
    c.m49_code = 51,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Armenia market. TRIGGERS: armenia.',
    c.description = 'Country node for Armenia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AW'})
SET c.display_name = 'Aruba',
    c.alpha3 = 'ABW',
    c.m49_code = 533,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Aruba market. TRIGGERS: aruba.',
    c.description = 'Country node for Aruba',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AU'})
SET c.display_name = 'Australia',
    c.alpha3 = 'AUS',
    c.m49_code = 36,
    c.region = 'Oceania',
    c.sub_region = 'Australia and New Zealand',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Australia market. TRIGGERS: australia.',
    c.description = 'Country node for Australia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AT'})
SET c.display_name = 'Austria',
    c.alpha3 = 'AUT',
    c.m49_code = 40,
    c.region = 'Europe',
    c.sub_region = 'Western Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Austria market. TRIGGERS: austria.',
    c.description = 'Country node for Austria',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AZ'})
SET c.display_name = 'Azerbaijan',
    c.alpha3 = 'AZE',
    c.m49_code = 31,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Azerbaijan market. TRIGGERS: azerbaijan.',
    c.description = 'Country node for Azerbaijan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BS'})
SET c.display_name = 'Bahamas',
    c.alpha3 = 'BHS',
    c.m49_code = 44,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Bahamas market. TRIGGERS: bahamas.',
    c.description = 'Country node for Bahamas',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BH'})
SET c.display_name = 'Bahrain',
    c.alpha3 = 'BHR',
    c.m49_code = 48,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Bahrain market. TRIGGERS: bahrain.',
    c.description = 'Country node for Bahrain',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BD'})
SET c.display_name = 'Bangladesh',
    c.alpha3 = 'BGD',
    c.m49_code = 50,
    c.region = 'Asia',
    c.sub_region = 'Southern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Bangladesh market. TRIGGERS: bangladesh.',
    c.description = 'Country node for Bangladesh',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BB'})
SET c.display_name = 'Barbados',
    c.alpha3 = 'BRB',
    c.m49_code = 52,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Barbados market. TRIGGERS: barbados.',
    c.description = 'Country node for Barbados',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BY'})
SET c.display_name = 'Belarus',
    c.alpha3 = 'BLR',
    c.m49_code = 112,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Belarus market. TRIGGERS: belarus.',
    c.description = 'Country node for Belarus',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BE'})
SET c.display_name = 'Belgium',
    c.alpha3 = 'BEL',
    c.m49_code = 56,
    c.region = 'Europe',
    c.sub_region = 'Western Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Belgium market. TRIGGERS: belgium.',
    c.description = 'Country node for Belgium',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BZ'})
SET c.display_name = 'Belize',
    c.alpha3 = 'BLZ',
    c.m49_code = 84,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Central America',
    c.llm_context = 'USE: for Belize market. TRIGGERS: belize.',
    c.description = 'Country node for Belize',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BJ'})
SET c.display_name = 'Benin',
    c.alpha3 = 'BEN',
    c.m49_code = 204,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Benin market. TRIGGERS: benin.',
    c.description = 'Country node for Benin',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BM'})
SET c.display_name = 'Bermuda',
    c.alpha3 = 'BMU',
    c.m49_code = 60,
    c.region = 'Americas',
    c.sub_region = 'Northern America',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Bermuda market. TRIGGERS: bermuda.',
    c.description = 'Country node for Bermuda',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BT'})
SET c.display_name = 'Bhutan',
    c.alpha3 = 'BTN',
    c.m49_code = 64,
    c.region = 'Asia',
    c.sub_region = 'Southern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Bhutan market. TRIGGERS: bhutan.',
    c.description = 'Country node for Bhutan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BO'})
SET c.display_name = 'Bolivia, Plurinational State of',
    c.alpha3 = 'BOL',
    c.m49_code = 68,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Bolivia, Plurinational State of market. TRIGGERS: bolivia, plurinational state of, bolivia,.',
    c.description = 'Country node for Bolivia, Plurinational State of',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BQ'})
SET c.display_name = 'Bonaire, Sint Eustatius and Saba',
    c.alpha3 = 'BES',
    c.m49_code = 535,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Bonaire, Sint Eustatius and Saba market. TRIGGERS: bonaire, sint eustatius and saba, bonaire,.',
    c.description = 'Country node for Bonaire, Sint Eustatius and Saba',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BA'})
SET c.display_name = 'Bosnia and Herzegovina',
    c.alpha3 = 'BIH',
    c.m49_code = 70,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Bosnia and Herzegovina market. TRIGGERS: bosnia and herzegovina, bosnia.',
    c.description = 'Country node for Bosnia and Herzegovina',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BW'})
SET c.display_name = 'Botswana',
    c.alpha3 = 'BWA',
    c.m49_code = 72,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Southern Africa',
    c.llm_context = 'USE: for Botswana market. TRIGGERS: botswana.',
    c.description = 'Country node for Botswana',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BV'})
SET c.display_name = 'Bouvet Island',
    c.alpha3 = 'BVT',
    c.m49_code = 74,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Bouvet Island market. TRIGGERS: bouvet island, bouvet.',
    c.description = 'Country node for Bouvet Island',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BR'})
SET c.display_name = 'Brazil',
    c.alpha3 = 'BRA',
    c.m49_code = 76,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Brazil market. TRIGGERS: brazil.',
    c.description = 'Country node for Brazil',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'IO'})
SET c.display_name = 'British Indian Ocean Territory',
    c.alpha3 = 'IOT',
    c.m49_code = 86,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for British Indian Ocean Territory market. TRIGGERS: british indian ocean territory, british.',
    c.description = 'Country node for British Indian Ocean Territory',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BN'})
SET c.display_name = 'Brunei Darussalam',
    c.alpha3 = 'BRN',
    c.m49_code = 96,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Brunei Darussalam market. TRIGGERS: brunei darussalam, brunei.',
    c.description = 'Country node for Brunei Darussalam',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BG'})
SET c.display_name = 'Bulgaria',
    c.alpha3 = 'BGR',
    c.m49_code = 100,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Bulgaria market. TRIGGERS: bulgaria.',
    c.description = 'Country node for Bulgaria',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BF'})
SET c.display_name = 'Burkina Faso',
    c.alpha3 = 'BFA',
    c.m49_code = 854,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Burkina Faso market. TRIGGERS: burkina faso, burkina.',
    c.description = 'Country node for Burkina Faso',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BI'})
SET c.display_name = 'Burundi',
    c.alpha3 = 'BDI',
    c.m49_code = 108,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Burundi market. TRIGGERS: burundi.',
    c.description = 'Country node for Burundi',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CV'})
SET c.display_name = 'Cabo Verde',
    c.alpha3 = 'CPV',
    c.m49_code = 132,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Cabo Verde market. TRIGGERS: cabo verde, cabo.',
    c.description = 'Country node for Cabo Verde',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KH'})
SET c.display_name = 'Cambodia',
    c.alpha3 = 'KHM',
    c.m49_code = 116,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Cambodia market. TRIGGERS: cambodia.',
    c.description = 'Country node for Cambodia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CM'})
SET c.display_name = 'Cameroon',
    c.alpha3 = 'CMR',
    c.m49_code = 120,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Middle Africa',
    c.llm_context = 'USE: for Cameroon market. TRIGGERS: cameroon.',
    c.description = 'Country node for Cameroon',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CA'})
SET c.display_name = 'Canada',
    c.alpha3 = 'CAN',
    c.m49_code = 124,
    c.region = 'Americas',
    c.sub_region = 'Northern America',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Canada market. TRIGGERS: canada.',
    c.description = 'Country node for Canada',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KY'})
SET c.display_name = 'Cayman Islands',
    c.alpha3 = 'CYM',
    c.m49_code = 136,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Cayman Islands market. TRIGGERS: cayman islands, cayman.',
    c.description = 'Country node for Cayman Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CF'})
SET c.display_name = 'Central African Republic',
    c.alpha3 = 'CAF',
    c.m49_code = 140,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Middle Africa',
    c.llm_context = 'USE: for Central African Republic market. TRIGGERS: central african republic, central.',
    c.description = 'Country node for Central African Republic',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TD'})
SET c.display_name = 'Chad',
    c.alpha3 = 'TCD',
    c.m49_code = 148,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Middle Africa',
    c.llm_context = 'USE: for Chad market. TRIGGERS: chad.',
    c.description = 'Country node for Chad',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CL'})
SET c.display_name = 'Chile',
    c.alpha3 = 'CHL',
    c.m49_code = 152,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Chile market. TRIGGERS: chile.',
    c.description = 'Country node for Chile',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CN'})
SET c.display_name = 'China',
    c.alpha3 = 'CHN',
    c.m49_code = 156,
    c.region = 'Asia',
    c.sub_region = 'Eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for China market. TRIGGERS: china.',
    c.description = 'Country node for China',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CX'})
SET c.display_name = 'Christmas Island',
    c.alpha3 = 'CXR',
    c.m49_code = 162,
    c.region = 'Oceania',
    c.sub_region = 'Australia and New Zealand',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Christmas Island market. TRIGGERS: christmas island, christmas.',
    c.description = 'Country node for Christmas Island',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CC'})
SET c.display_name = 'Cocos (Keeling) Islands',
    c.alpha3 = 'CCK',
    c.m49_code = 166,
    c.region = 'Oceania',
    c.sub_region = 'Australia and New Zealand',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Cocos (Keeling) Islands market. TRIGGERS: cocos (keeling) islands, cocos.',
    c.description = 'Country node for Cocos (Keeling) Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CO'})
SET c.display_name = 'Colombia',
    c.alpha3 = 'COL',
    c.m49_code = 170,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Colombia market. TRIGGERS: colombia.',
    c.description = 'Country node for Colombia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KM'})
SET c.display_name = 'Comoros',
    c.alpha3 = 'COM',
    c.m49_code = 174,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Comoros market. TRIGGERS: comoros.',
    c.description = 'Country node for Comoros',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CG'})
SET c.display_name = 'Congo',
    c.alpha3 = 'COG',
    c.m49_code = 178,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Middle Africa',
    c.llm_context = 'USE: for Congo market. TRIGGERS: congo.',
    c.description = 'Country node for Congo',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CD'})
SET c.display_name = 'Congo, Democratic Republic of the',
    c.alpha3 = 'COD',
    c.m49_code = 180,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Middle Africa',
    c.llm_context = 'USE: for Congo, Democratic Republic of the market. TRIGGERS: congo, democratic republic of the, congo,.',
    c.description = 'Country node for Congo, Democratic Republic of the',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CK'})
SET c.display_name = 'Cook Islands',
    c.alpha3 = 'COK',
    c.m49_code = 184,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Cook Islands market. TRIGGERS: cook islands, cook.',
    c.description = 'Country node for Cook Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CR'})
SET c.display_name = 'Costa Rica',
    c.alpha3 = 'CRI',
    c.m49_code = 188,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Central America',
    c.llm_context = 'USE: for Costa Rica market. TRIGGERS: costa rica, costa.',
    c.description = 'Country node for Costa Rica',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CI'})
SET c.display_name = 'Côte d\'Ivoire',
    c.alpha3 = 'CIV',
    c.m49_code = 384,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Côte d\'Ivoire market. TRIGGERS: côte d\'ivoire, côte.',
    c.description = 'Country node for Côte d\'Ivoire',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'HR'})
SET c.display_name = 'Croatia',
    c.alpha3 = 'HRV',
    c.m49_code = 191,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Croatia market. TRIGGERS: croatia.',
    c.description = 'Country node for Croatia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CU'})
SET c.display_name = 'Cuba',
    c.alpha3 = 'CUB',
    c.m49_code = 192,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Cuba market. TRIGGERS: cuba.',
    c.description = 'Country node for Cuba',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CW'})
SET c.display_name = 'Curaçao',
    c.alpha3 = 'CUW',
    c.m49_code = 531,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Curaçao market. TRIGGERS: curaçao.',
    c.description = 'Country node for Curaçao',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CY'})
SET c.display_name = 'Cyprus',
    c.alpha3 = 'CYP',
    c.m49_code = 196,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Cyprus market. TRIGGERS: cyprus.',
    c.description = 'Country node for Cyprus',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CZ'})
SET c.display_name = 'Czechia',
    c.alpha3 = 'CZE',
    c.m49_code = 203,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Czechia market. TRIGGERS: czechia.',
    c.description = 'Country node for Czechia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'DK'})
SET c.display_name = 'Denmark',
    c.alpha3 = 'DNK',
    c.m49_code = 208,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Denmark market. TRIGGERS: denmark.',
    c.description = 'Country node for Denmark',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'DJ'})
SET c.display_name = 'Djibouti',
    c.alpha3 = 'DJI',
    c.m49_code = 262,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Djibouti market. TRIGGERS: djibouti.',
    c.description = 'Country node for Djibouti',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'DM'})
SET c.display_name = 'Dominica',
    c.alpha3 = 'DMA',
    c.m49_code = 212,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Dominica market. TRIGGERS: dominica.',
    c.description = 'Country node for Dominica',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'DO'})
SET c.display_name = 'Dominican Republic',
    c.alpha3 = 'DOM',
    c.m49_code = 214,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Dominican Republic market. TRIGGERS: dominican republic, dominican.',
    c.description = 'Country node for Dominican Republic',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'EC'})
SET c.display_name = 'Ecuador',
    c.alpha3 = 'ECU',
    c.m49_code = 218,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Ecuador market. TRIGGERS: ecuador.',
    c.description = 'Country node for Ecuador',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'EG'})
SET c.display_name = 'Egypt',
    c.alpha3 = 'EGY',
    c.m49_code = 818,
    c.region = 'Africa',
    c.sub_region = 'Northern Africa',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Egypt market. TRIGGERS: egypt.',
    c.description = 'Country node for Egypt',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SV'})
SET c.display_name = 'El Salvador',
    c.alpha3 = 'SLV',
    c.m49_code = 222,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Central America',
    c.llm_context = 'USE: for El Salvador market. TRIGGERS: el salvador, el.',
    c.description = 'Country node for El Salvador',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GQ'})
SET c.display_name = 'Equatorial Guinea',
    c.alpha3 = 'GNQ',
    c.m49_code = 226,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Middle Africa',
    c.llm_context = 'USE: for Equatorial Guinea market. TRIGGERS: equatorial guinea, equatorial.',
    c.description = 'Country node for Equatorial Guinea',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ER'})
SET c.display_name = 'Eritrea',
    c.alpha3 = 'ERI',
    c.m49_code = 232,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Eritrea market. TRIGGERS: eritrea.',
    c.description = 'Country node for Eritrea',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'EE'})
SET c.display_name = 'Estonia',
    c.alpha3 = 'EST',
    c.m49_code = 233,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Estonia market. TRIGGERS: estonia.',
    c.description = 'Country node for Estonia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SZ'})
SET c.display_name = 'Eswatini',
    c.alpha3 = 'SWZ',
    c.m49_code = 748,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Southern Africa',
    c.llm_context = 'USE: for Eswatini market. TRIGGERS: eswatini.',
    c.description = 'Country node for Eswatini',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ET'})
SET c.display_name = 'Ethiopia',
    c.alpha3 = 'ETH',
    c.m49_code = 231,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Ethiopia market. TRIGGERS: ethiopia.',
    c.description = 'Country node for Ethiopia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'FK'})
SET c.display_name = 'Falkland Islands (Malvinas)',
    c.alpha3 = 'FLK',
    c.m49_code = 238,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Falkland Islands (Malvinas) market. TRIGGERS: falkland islands (malvinas), falkland.',
    c.description = 'Country node for Falkland Islands (Malvinas)',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'FO'})
SET c.display_name = 'Faroe Islands',
    c.alpha3 = 'FRO',
    c.m49_code = 234,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Faroe Islands market. TRIGGERS: faroe islands, faroe.',
    c.description = 'Country node for Faroe Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'FJ'})
SET c.display_name = 'Fiji',
    c.alpha3 = 'FJI',
    c.m49_code = 242,
    c.region = 'Oceania',
    c.sub_region = 'Melanesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Fiji market. TRIGGERS: fiji.',
    c.description = 'Country node for Fiji',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'FI'})
SET c.display_name = 'Finland',
    c.alpha3 = 'FIN',
    c.m49_code = 246,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Finland market. TRIGGERS: finland.',
    c.description = 'Country node for Finland',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'FR'})
SET c.display_name = 'France',
    c.alpha3 = 'FRA',
    c.m49_code = 250,
    c.region = 'Europe',
    c.sub_region = 'Western Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for France market. TRIGGERS: france.',
    c.description = 'Country node for France',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GF'})
SET c.display_name = 'French Guiana',
    c.alpha3 = 'GUF',
    c.m49_code = 254,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for French Guiana market. TRIGGERS: french guiana, french.',
    c.description = 'Country node for French Guiana',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PF'})
SET c.display_name = 'French Polynesia',
    c.alpha3 = 'PYF',
    c.m49_code = 258,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for French Polynesia market. TRIGGERS: french polynesia, french.',
    c.description = 'Country node for French Polynesia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TF'})
SET c.display_name = 'French Southern Territories',
    c.alpha3 = 'ATF',
    c.m49_code = 260,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for French Southern Territories market. TRIGGERS: french southern territories, french.',
    c.description = 'Country node for French Southern Territories',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GA'})
SET c.display_name = 'Gabon',
    c.alpha3 = 'GAB',
    c.m49_code = 266,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Middle Africa',
    c.llm_context = 'USE: for Gabon market. TRIGGERS: gabon.',
    c.description = 'Country node for Gabon',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GM'})
SET c.display_name = 'Gambia',
    c.alpha3 = 'GMB',
    c.m49_code = 270,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Gambia market. TRIGGERS: gambia.',
    c.description = 'Country node for Gambia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GE'})
SET c.display_name = 'Georgia',
    c.alpha3 = 'GEO',
    c.m49_code = 268,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Georgia market. TRIGGERS: georgia.',
    c.description = 'Country node for Georgia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'DE'})
SET c.display_name = 'Germany',
    c.alpha3 = 'DEU',
    c.m49_code = 276,
    c.region = 'Europe',
    c.sub_region = 'Western Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Germany market. TRIGGERS: germany.',
    c.description = 'Country node for Germany',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GH'})
SET c.display_name = 'Ghana',
    c.alpha3 = 'GHA',
    c.m49_code = 288,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Ghana market. TRIGGERS: ghana.',
    c.description = 'Country node for Ghana',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GI'})
SET c.display_name = 'Gibraltar',
    c.alpha3 = 'GIB',
    c.m49_code = 292,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Gibraltar market. TRIGGERS: gibraltar.',
    c.description = 'Country node for Gibraltar',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GR'})
SET c.display_name = 'Greece',
    c.alpha3 = 'GRC',
    c.m49_code = 300,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Greece market. TRIGGERS: greece.',
    c.description = 'Country node for Greece',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GL'})
SET c.display_name = 'Greenland',
    c.alpha3 = 'GRL',
    c.m49_code = 304,
    c.region = 'Americas',
    c.sub_region = 'Northern America',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Greenland market. TRIGGERS: greenland.',
    c.description = 'Country node for Greenland',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GD'})
SET c.display_name = 'Grenada',
    c.alpha3 = 'GRD',
    c.m49_code = 308,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Grenada market. TRIGGERS: grenada.',
    c.description = 'Country node for Grenada',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GP'})
SET c.display_name = 'Guadeloupe',
    c.alpha3 = 'GLP',
    c.m49_code = 312,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Guadeloupe market. TRIGGERS: guadeloupe.',
    c.description = 'Country node for Guadeloupe',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GU'})
SET c.display_name = 'Guam',
    c.alpha3 = 'GUM',
    c.m49_code = 316,
    c.region = 'Oceania',
    c.sub_region = 'Micronesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Guam market. TRIGGERS: guam.',
    c.description = 'Country node for Guam',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GT'})
SET c.display_name = 'Guatemala',
    c.alpha3 = 'GTM',
    c.m49_code = 320,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Central America',
    c.llm_context = 'USE: for Guatemala market. TRIGGERS: guatemala.',
    c.description = 'Country node for Guatemala',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GG'})
SET c.display_name = 'Guernsey',
    c.alpha3 = 'GGY',
    c.m49_code = 831,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Guernsey market. TRIGGERS: guernsey.',
    c.description = 'Country node for Guernsey',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GN'})
SET c.display_name = 'Guinea',
    c.alpha3 = 'GIN',
    c.m49_code = 324,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Guinea market. TRIGGERS: guinea.',
    c.description = 'Country node for Guinea',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GW'})
SET c.display_name = 'Guinea-Bissau',
    c.alpha3 = 'GNB',
    c.m49_code = 624,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Guinea-Bissau market. TRIGGERS: guinea-bissau.',
    c.description = 'Country node for Guinea-Bissau',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GY'})
SET c.display_name = 'Guyana',
    c.alpha3 = 'GUY',
    c.m49_code = 328,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Guyana market. TRIGGERS: guyana.',
    c.description = 'Country node for Guyana',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'HT'})
SET c.display_name = 'Haiti',
    c.alpha3 = 'HTI',
    c.m49_code = 332,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Haiti market. TRIGGERS: haiti.',
    c.description = 'Country node for Haiti',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'HM'})
SET c.display_name = 'Heard Island and McDonald Islands',
    c.alpha3 = 'HMD',
    c.m49_code = 334,
    c.region = 'Oceania',
    c.sub_region = 'Australia and New Zealand',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Heard Island and McDonald Islands market. TRIGGERS: heard island and mcdonald islands, heard.',
    c.description = 'Country node for Heard Island and McDonald Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'VA'})
SET c.display_name = 'Holy See',
    c.alpha3 = 'VAT',
    c.m49_code = 336,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Holy See market. TRIGGERS: holy see, holy.',
    c.description = 'Country node for Holy See',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'HN'})
SET c.display_name = 'Honduras',
    c.alpha3 = 'HND',
    c.m49_code = 340,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Central America',
    c.llm_context = 'USE: for Honduras market. TRIGGERS: honduras.',
    c.description = 'Country node for Honduras',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'HK'})
SET c.display_name = 'Hong Kong',
    c.alpha3 = 'HKG',
    c.m49_code = 344,
    c.region = 'Asia',
    c.sub_region = 'Eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Hong Kong market. TRIGGERS: hong kong, hong.',
    c.description = 'Country node for Hong Kong',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'HU'})
SET c.display_name = 'Hungary',
    c.alpha3 = 'HUN',
    c.m49_code = 348,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Hungary market. TRIGGERS: hungary.',
    c.description = 'Country node for Hungary',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'IS'})
SET c.display_name = 'Iceland',
    c.alpha3 = 'ISL',
    c.m49_code = 352,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Iceland market. TRIGGERS: iceland.',
    c.description = 'Country node for Iceland',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'IN'})
SET c.display_name = 'India',
    c.alpha3 = 'IND',
    c.m49_code = 356,
    c.region = 'Asia',
    c.sub_region = 'Southern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for India market. TRIGGERS: india.',
    c.description = 'Country node for India',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ID'})
SET c.display_name = 'Indonesia',
    c.alpha3 = 'IDN',
    c.m49_code = 360,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Indonesia market. TRIGGERS: indonesia.',
    c.description = 'Country node for Indonesia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'IR'})
SET c.display_name = 'Iran, Islamic Republic of',
    c.alpha3 = 'IRN',
    c.m49_code = 364,
    c.region = 'Asia',
    c.sub_region = 'Southern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Iran, Islamic Republic of market. TRIGGERS: iran, islamic republic of, iran,.',
    c.description = 'Country node for Iran, Islamic Republic of',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'IQ'})
SET c.display_name = 'Iraq',
    c.alpha3 = 'IRQ',
    c.m49_code = 368,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Iraq market. TRIGGERS: iraq.',
    c.description = 'Country node for Iraq',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'IE'})
SET c.display_name = 'Ireland',
    c.alpha3 = 'IRL',
    c.m49_code = 372,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Ireland market. TRIGGERS: ireland.',
    c.description = 'Country node for Ireland',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'IM'})
SET c.display_name = 'Isle of Man',
    c.alpha3 = 'IMN',
    c.m49_code = 833,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Isle of Man market. TRIGGERS: isle of man, isle.',
    c.description = 'Country node for Isle of Man',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'IL'})
SET c.display_name = 'Israel',
    c.alpha3 = 'ISR',
    c.m49_code = 376,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Israel market. TRIGGERS: israel.',
    c.description = 'Country node for Israel',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'IT'})
SET c.display_name = 'Italy',
    c.alpha3 = 'ITA',
    c.m49_code = 380,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Italy market. TRIGGERS: italy.',
    c.description = 'Country node for Italy',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'JM'})
SET c.display_name = 'Jamaica',
    c.alpha3 = 'JAM',
    c.m49_code = 388,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Jamaica market. TRIGGERS: jamaica.',
    c.description = 'Country node for Jamaica',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'JP'})
SET c.display_name = 'Japan',
    c.alpha3 = 'JPN',
    c.m49_code = 392,
    c.region = 'Asia',
    c.sub_region = 'Eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Japan market. TRIGGERS: japan.',
    c.description = 'Country node for Japan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'JE'})
SET c.display_name = 'Jersey',
    c.alpha3 = 'JEY',
    c.m49_code = 832,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Jersey market. TRIGGERS: jersey.',
    c.description = 'Country node for Jersey',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'JO'})
SET c.display_name = 'Jordan',
    c.alpha3 = 'JOR',
    c.m49_code = 400,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Jordan market. TRIGGERS: jordan.',
    c.description = 'Country node for Jordan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KZ'})
SET c.display_name = 'Kazakhstan',
    c.alpha3 = 'KAZ',
    c.m49_code = 398,
    c.region = 'Asia',
    c.sub_region = 'Central Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Kazakhstan market. TRIGGERS: kazakhstan.',
    c.description = 'Country node for Kazakhstan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KE'})
SET c.display_name = 'Kenya',
    c.alpha3 = 'KEN',
    c.m49_code = 404,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Kenya market. TRIGGERS: kenya.',
    c.description = 'Country node for Kenya',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KI'})
SET c.display_name = 'Kiribati',
    c.alpha3 = 'KIR',
    c.m49_code = 296,
    c.region = 'Oceania',
    c.sub_region = 'Micronesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Kiribati market. TRIGGERS: kiribati.',
    c.description = 'Country node for Kiribati',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KP'})
SET c.display_name = 'Korea, Democratic People\'s Republic of',
    c.alpha3 = 'PRK',
    c.m49_code = 408,
    c.region = 'Asia',
    c.sub_region = 'Eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Korea, Democratic People\'s Republic of market. TRIGGERS: korea, democratic people\'s republic of, korea,.',
    c.description = 'Country node for Korea, Democratic People\'s Republic of',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KR'})
SET c.display_name = 'Korea, Republic of',
    c.alpha3 = 'KOR',
    c.m49_code = 410,
    c.region = 'Asia',
    c.sub_region = 'Eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Korea, Republic of market. TRIGGERS: korea, republic of, korea,.',
    c.description = 'Country node for Korea, Republic of',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KW'})
SET c.display_name = 'Kuwait',
    c.alpha3 = 'KWT',
    c.m49_code = 414,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Kuwait market. TRIGGERS: kuwait.',
    c.description = 'Country node for Kuwait',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KG'})
SET c.display_name = 'Kyrgyzstan',
    c.alpha3 = 'KGZ',
    c.m49_code = 417,
    c.region = 'Asia',
    c.sub_region = 'Central Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Kyrgyzstan market. TRIGGERS: kyrgyzstan.',
    c.description = 'Country node for Kyrgyzstan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LA'})
SET c.display_name = 'Lao People\'s Democratic Republic',
    c.alpha3 = 'LAO',
    c.m49_code = 418,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Lao People\'s Democratic Republic market. TRIGGERS: lao people\'s democratic republic, lao.',
    c.description = 'Country node for Lao People\'s Democratic Republic',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LV'})
SET c.display_name = 'Latvia',
    c.alpha3 = 'LVA',
    c.m49_code = 428,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Latvia market. TRIGGERS: latvia.',
    c.description = 'Country node for Latvia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LB'})
SET c.display_name = 'Lebanon',
    c.alpha3 = 'LBN',
    c.m49_code = 422,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Lebanon market. TRIGGERS: lebanon.',
    c.description = 'Country node for Lebanon',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LS'})
SET c.display_name = 'Lesotho',
    c.alpha3 = 'LSO',
    c.m49_code = 426,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Southern Africa',
    c.llm_context = 'USE: for Lesotho market. TRIGGERS: lesotho.',
    c.description = 'Country node for Lesotho',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LR'})
SET c.display_name = 'Liberia',
    c.alpha3 = 'LBR',
    c.m49_code = 430,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Liberia market. TRIGGERS: liberia.',
    c.description = 'Country node for Liberia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LY'})
SET c.display_name = 'Libya',
    c.alpha3 = 'LBY',
    c.m49_code = 434,
    c.region = 'Africa',
    c.sub_region = 'Northern Africa',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Libya market. TRIGGERS: libya.',
    c.description = 'Country node for Libya',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LI'})
SET c.display_name = 'Liechtenstein',
    c.alpha3 = 'LIE',
    c.m49_code = 438,
    c.region = 'Europe',
    c.sub_region = 'Western Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Liechtenstein market. TRIGGERS: liechtenstein.',
    c.description = 'Country node for Liechtenstein',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LT'})
SET c.display_name = 'Lithuania',
    c.alpha3 = 'LTU',
    c.m49_code = 440,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Lithuania market. TRIGGERS: lithuania.',
    c.description = 'Country node for Lithuania',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LU'})
SET c.display_name = 'Luxembourg',
    c.alpha3 = 'LUX',
    c.m49_code = 442,
    c.region = 'Europe',
    c.sub_region = 'Western Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Luxembourg market. TRIGGERS: luxembourg.',
    c.description = 'Country node for Luxembourg',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MO'})
SET c.display_name = 'Macao',
    c.alpha3 = 'MAC',
    c.m49_code = 446,
    c.region = 'Asia',
    c.sub_region = 'Eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Macao market. TRIGGERS: macao.',
    c.description = 'Country node for Macao',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MG'})
SET c.display_name = 'Madagascar',
    c.alpha3 = 'MDG',
    c.m49_code = 450,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Madagascar market. TRIGGERS: madagascar.',
    c.description = 'Country node for Madagascar',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MW'})
SET c.display_name = 'Malawi',
    c.alpha3 = 'MWI',
    c.m49_code = 454,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Malawi market. TRIGGERS: malawi.',
    c.description = 'Country node for Malawi',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MY'})
SET c.display_name = 'Malaysia',
    c.alpha3 = 'MYS',
    c.m49_code = 458,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Malaysia market. TRIGGERS: malaysia.',
    c.description = 'Country node for Malaysia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MV'})
SET c.display_name = 'Maldives',
    c.alpha3 = 'MDV',
    c.m49_code = 462,
    c.region = 'Asia',
    c.sub_region = 'Southern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Maldives market. TRIGGERS: maldives.',
    c.description = 'Country node for Maldives',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ML'})
SET c.display_name = 'Mali',
    c.alpha3 = 'MLI',
    c.m49_code = 466,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Mali market. TRIGGERS: mali.',
    c.description = 'Country node for Mali',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MT'})
SET c.display_name = 'Malta',
    c.alpha3 = 'MLT',
    c.m49_code = 470,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Malta market. TRIGGERS: malta.',
    c.description = 'Country node for Malta',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MH'})
SET c.display_name = 'Marshall Islands',
    c.alpha3 = 'MHL',
    c.m49_code = 584,
    c.region = 'Oceania',
    c.sub_region = 'Micronesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Marshall Islands market. TRIGGERS: marshall islands, marshall.',
    c.description = 'Country node for Marshall Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MQ'})
SET c.display_name = 'Martinique',
    c.alpha3 = 'MTQ',
    c.m49_code = 474,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Martinique market. TRIGGERS: martinique.',
    c.description = 'Country node for Martinique',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MR'})
SET c.display_name = 'Mauritania',
    c.alpha3 = 'MRT',
    c.m49_code = 478,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Mauritania market. TRIGGERS: mauritania.',
    c.description = 'Country node for Mauritania',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MU'})
SET c.display_name = 'Mauritius',
    c.alpha3 = 'MUS',
    c.m49_code = 480,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Mauritius market. TRIGGERS: mauritius.',
    c.description = 'Country node for Mauritius',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'YT'})
SET c.display_name = 'Mayotte',
    c.alpha3 = 'MYT',
    c.m49_code = 175,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Mayotte market. TRIGGERS: mayotte.',
    c.description = 'Country node for Mayotte',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MX'})
SET c.display_name = 'Mexico',
    c.alpha3 = 'MEX',
    c.m49_code = 484,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Central America',
    c.llm_context = 'USE: for Mexico market. TRIGGERS: mexico.',
    c.description = 'Country node for Mexico',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'FM'})
SET c.display_name = 'Micronesia, Federated States of',
    c.alpha3 = 'FSM',
    c.m49_code = 583,
    c.region = 'Oceania',
    c.sub_region = 'Micronesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Micronesia, Federated States of market. TRIGGERS: micronesia, federated states of, micronesia,.',
    c.description = 'Country node for Micronesia, Federated States of',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MD'})
SET c.display_name = 'Moldova, Republic of',
    c.alpha3 = 'MDA',
    c.m49_code = 498,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Moldova, Republic of market. TRIGGERS: moldova, republic of, moldova,.',
    c.description = 'Country node for Moldova, Republic of',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MC'})
SET c.display_name = 'Monaco',
    c.alpha3 = 'MCO',
    c.m49_code = 492,
    c.region = 'Europe',
    c.sub_region = 'Western Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Monaco market. TRIGGERS: monaco.',
    c.description = 'Country node for Monaco',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MN'})
SET c.display_name = 'Mongolia',
    c.alpha3 = 'MNG',
    c.m49_code = 496,
    c.region = 'Asia',
    c.sub_region = 'Eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Mongolia market. TRIGGERS: mongolia.',
    c.description = 'Country node for Mongolia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ME'})
SET c.display_name = 'Montenegro',
    c.alpha3 = 'MNE',
    c.m49_code = 499,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Montenegro market. TRIGGERS: montenegro.',
    c.description = 'Country node for Montenegro',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MS'})
SET c.display_name = 'Montserrat',
    c.alpha3 = 'MSR',
    c.m49_code = 500,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Montserrat market. TRIGGERS: montserrat.',
    c.description = 'Country node for Montserrat',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MA'})
SET c.display_name = 'Morocco',
    c.alpha3 = 'MAR',
    c.m49_code = 504,
    c.region = 'Africa',
    c.sub_region = 'Northern Africa',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Morocco market. TRIGGERS: morocco.',
    c.description = 'Country node for Morocco',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MZ'})
SET c.display_name = 'Mozambique',
    c.alpha3 = 'MOZ',
    c.m49_code = 508,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Mozambique market. TRIGGERS: mozambique.',
    c.description = 'Country node for Mozambique',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MM'})
SET c.display_name = 'Myanmar',
    c.alpha3 = 'MMR',
    c.m49_code = 104,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Myanmar market. TRIGGERS: myanmar.',
    c.description = 'Country node for Myanmar',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NA'})
SET c.display_name = 'Namibia',
    c.alpha3 = 'NAM',
    c.m49_code = 516,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Southern Africa',
    c.llm_context = 'USE: for Namibia market. TRIGGERS: namibia.',
    c.description = 'Country node for Namibia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NR'})
SET c.display_name = 'Nauru',
    c.alpha3 = 'NRU',
    c.m49_code = 520,
    c.region = 'Oceania',
    c.sub_region = 'Micronesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Nauru market. TRIGGERS: nauru.',
    c.description = 'Country node for Nauru',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NP'})
SET c.display_name = 'Nepal',
    c.alpha3 = 'NPL',
    c.m49_code = 524,
    c.region = 'Asia',
    c.sub_region = 'Southern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Nepal market. TRIGGERS: nepal.',
    c.description = 'Country node for Nepal',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NL'})
SET c.display_name = 'Netherlands, Kingdom of the',
    c.alpha3 = 'NLD',
    c.m49_code = 528,
    c.region = 'Europe',
    c.sub_region = 'Western Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Netherlands, Kingdom of the market. TRIGGERS: netherlands, kingdom of the, netherlands,.',
    c.description = 'Country node for Netherlands, Kingdom of the',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NC'})
SET c.display_name = 'New Caledonia',
    c.alpha3 = 'NCL',
    c.m49_code = 540,
    c.region = 'Oceania',
    c.sub_region = 'Melanesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for New Caledonia market. TRIGGERS: new caledonia, new.',
    c.description = 'Country node for New Caledonia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NZ'})
SET c.display_name = 'New Zealand',
    c.alpha3 = 'NZL',
    c.m49_code = 554,
    c.region = 'Oceania',
    c.sub_region = 'Australia and New Zealand',
    c.intermediate_region = '',
    c.llm_context = 'USE: for New Zealand market. TRIGGERS: new zealand, new.',
    c.description = 'Country node for New Zealand',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NI'})
SET c.display_name = 'Nicaragua',
    c.alpha3 = 'NIC',
    c.m49_code = 558,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Central America',
    c.llm_context = 'USE: for Nicaragua market. TRIGGERS: nicaragua.',
    c.description = 'Country node for Nicaragua',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NE'})
SET c.display_name = 'Niger',
    c.alpha3 = 'NER',
    c.m49_code = 562,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Niger market. TRIGGERS: niger.',
    c.description = 'Country node for Niger',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NG'})
SET c.display_name = 'Nigeria',
    c.alpha3 = 'NGA',
    c.m49_code = 566,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Nigeria market. TRIGGERS: nigeria.',
    c.description = 'Country node for Nigeria',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NU'})
SET c.display_name = 'Niue',
    c.alpha3 = 'NIU',
    c.m49_code = 570,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Niue market. TRIGGERS: niue.',
    c.description = 'Country node for Niue',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NF'})
SET c.display_name = 'Norfolk Island',
    c.alpha3 = 'NFK',
    c.m49_code = 574,
    c.region = 'Oceania',
    c.sub_region = 'Australia and New Zealand',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Norfolk Island market. TRIGGERS: norfolk island, norfolk.',
    c.description = 'Country node for Norfolk Island',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MK'})
SET c.display_name = 'North Macedonia',
    c.alpha3 = 'MKD',
    c.m49_code = 807,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for North Macedonia market. TRIGGERS: north macedonia, north.',
    c.description = 'Country node for North Macedonia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MP'})
SET c.display_name = 'Northern Mariana Islands',
    c.alpha3 = 'MNP',
    c.m49_code = 580,
    c.region = 'Oceania',
    c.sub_region = 'Micronesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Northern Mariana Islands market. TRIGGERS: northern mariana islands, northern.',
    c.description = 'Country node for Northern Mariana Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'NO'})
SET c.display_name = 'Norway',
    c.alpha3 = 'NOR',
    c.m49_code = 578,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Norway market. TRIGGERS: norway.',
    c.description = 'Country node for Norway',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'OM'})
SET c.display_name = 'Oman',
    c.alpha3 = 'OMN',
    c.m49_code = 512,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Oman market. TRIGGERS: oman.',
    c.description = 'Country node for Oman',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PK'})
SET c.display_name = 'Pakistan',
    c.alpha3 = 'PAK',
    c.m49_code = 586,
    c.region = 'Asia',
    c.sub_region = 'Southern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Pakistan market. TRIGGERS: pakistan.',
    c.description = 'Country node for Pakistan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PW'})
SET c.display_name = 'Palau',
    c.alpha3 = 'PLW',
    c.m49_code = 585,
    c.region = 'Oceania',
    c.sub_region = 'Micronesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Palau market. TRIGGERS: palau.',
    c.description = 'Country node for Palau',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PS'})
SET c.display_name = 'Palestine, State of',
    c.alpha3 = 'PSE',
    c.m49_code = 275,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Palestine, State of market. TRIGGERS: palestine, state of, palestine,.',
    c.description = 'Country node for Palestine, State of',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PA'})
SET c.display_name = 'Panama',
    c.alpha3 = 'PAN',
    c.m49_code = 591,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Central America',
    c.llm_context = 'USE: for Panama market. TRIGGERS: panama.',
    c.description = 'Country node for Panama',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PG'})
SET c.display_name = 'Papua New Guinea',
    c.alpha3 = 'PNG',
    c.m49_code = 598,
    c.region = 'Oceania',
    c.sub_region = 'Melanesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Papua New Guinea market. TRIGGERS: papua new guinea, papua.',
    c.description = 'Country node for Papua New Guinea',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PY'})
SET c.display_name = 'Paraguay',
    c.alpha3 = 'PRY',
    c.m49_code = 600,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Paraguay market. TRIGGERS: paraguay.',
    c.description = 'Country node for Paraguay',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PE'})
SET c.display_name = 'Peru',
    c.alpha3 = 'PER',
    c.m49_code = 604,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Peru market. TRIGGERS: peru.',
    c.description = 'Country node for Peru',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PH'})
SET c.display_name = 'Philippines',
    c.alpha3 = 'PHL',
    c.m49_code = 608,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Philippines market. TRIGGERS: philippines.',
    c.description = 'Country node for Philippines',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PN'})
SET c.display_name = 'Pitcairn',
    c.alpha3 = 'PCN',
    c.m49_code = 612,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Pitcairn market. TRIGGERS: pitcairn.',
    c.description = 'Country node for Pitcairn',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PL'})
SET c.display_name = 'Poland',
    c.alpha3 = 'POL',
    c.m49_code = 616,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Poland market. TRIGGERS: poland.',
    c.description = 'Country node for Poland',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PT'})
SET c.display_name = 'Portugal',
    c.alpha3 = 'PRT',
    c.m49_code = 620,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Portugal market. TRIGGERS: portugal.',
    c.description = 'Country node for Portugal',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PR'})
SET c.display_name = 'Puerto Rico',
    c.alpha3 = 'PRI',
    c.m49_code = 630,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Puerto Rico market. TRIGGERS: puerto rico, puerto.',
    c.description = 'Country node for Puerto Rico',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'QA'})
SET c.display_name = 'Qatar',
    c.alpha3 = 'QAT',
    c.m49_code = 634,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Qatar market. TRIGGERS: qatar.',
    c.description = 'Country node for Qatar',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'RE'})
SET c.display_name = 'Réunion',
    c.alpha3 = 'REU',
    c.m49_code = 638,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Réunion market. TRIGGERS: réunion.',
    c.description = 'Country node for Réunion',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'RO'})
SET c.display_name = 'Romania',
    c.alpha3 = 'ROU',
    c.m49_code = 642,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Romania market. TRIGGERS: romania.',
    c.description = 'Country node for Romania',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'RU'})
SET c.display_name = 'Russian Federation',
    c.alpha3 = 'RUS',
    c.m49_code = 643,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Russian Federation market. TRIGGERS: russian federation, russian, russia, russian.',
    c.description = 'Country node for Russian Federation',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'RW'})
SET c.display_name = 'Rwanda',
    c.alpha3 = 'RWA',
    c.m49_code = 646,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Rwanda market. TRIGGERS: rwanda.',
    c.description = 'Country node for Rwanda',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'BL'})
SET c.display_name = 'Saint Barthélemy',
    c.alpha3 = 'BLM',
    c.m49_code = 652,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Saint Barthélemy market. TRIGGERS: saint barthélemy, saint.',
    c.description = 'Country node for Saint Barthélemy',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SH'})
SET c.display_name = 'Saint Helena, Ascension and Tristan da Cunha',
    c.alpha3 = 'SHN',
    c.m49_code = 654,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Saint Helena, Ascension and Tristan da Cunha market. TRIGGERS: saint helena, ascension and tristan da cunha, saint.',
    c.description = 'Country node for Saint Helena, Ascension and Tristan da Cunha',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'KN'})
SET c.display_name = 'Saint Kitts and Nevis',
    c.alpha3 = 'KNA',
    c.m49_code = 659,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Saint Kitts and Nevis market. TRIGGERS: saint kitts and nevis, saint.',
    c.description = 'Country node for Saint Kitts and Nevis',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LC'})
SET c.display_name = 'Saint Lucia',
    c.alpha3 = 'LCA',
    c.m49_code = 662,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Saint Lucia market. TRIGGERS: saint lucia, saint.',
    c.description = 'Country node for Saint Lucia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'MF'})
SET c.display_name = 'Saint Martin (French part)',
    c.alpha3 = 'MAF',
    c.m49_code = 663,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Saint Martin (French part) market. TRIGGERS: saint martin (french part), saint.',
    c.description = 'Country node for Saint Martin (French part)',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'PM'})
SET c.display_name = 'Saint Pierre and Miquelon',
    c.alpha3 = 'SPM',
    c.m49_code = 666,
    c.region = 'Americas',
    c.sub_region = 'Northern America',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Saint Pierre and Miquelon market. TRIGGERS: saint pierre and miquelon, saint.',
    c.description = 'Country node for Saint Pierre and Miquelon',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'VC'})
SET c.display_name = 'Saint Vincent and the Grenadines',
    c.alpha3 = 'VCT',
    c.m49_code = 670,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Saint Vincent and the Grenadines market. TRIGGERS: saint vincent and the grenadines, saint.',
    c.description = 'Country node for Saint Vincent and the Grenadines',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'WS'})
SET c.display_name = 'Samoa',
    c.alpha3 = 'WSM',
    c.m49_code = 882,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Samoa market. TRIGGERS: samoa.',
    c.description = 'Country node for Samoa',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SM'})
SET c.display_name = 'San Marino',
    c.alpha3 = 'SMR',
    c.m49_code = 674,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for San Marino market. TRIGGERS: san marino, san.',
    c.description = 'Country node for San Marino',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ST'})
SET c.display_name = 'Sao Tome and Principe',
    c.alpha3 = 'STP',
    c.m49_code = 678,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Middle Africa',
    c.llm_context = 'USE: for Sao Tome and Principe market. TRIGGERS: sao tome and principe, sao.',
    c.description = 'Country node for Sao Tome and Principe',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SA'})
SET c.display_name = 'Saudi Arabia',
    c.alpha3 = 'SAU',
    c.m49_code = 682,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Saudi Arabia market. TRIGGERS: saudi arabia, saudi.',
    c.description = 'Country node for Saudi Arabia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SN'})
SET c.display_name = 'Senegal',
    c.alpha3 = 'SEN',
    c.m49_code = 686,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Senegal market. TRIGGERS: senegal.',
    c.description = 'Country node for Senegal',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'RS'})
SET c.display_name = 'Serbia',
    c.alpha3 = 'SRB',
    c.m49_code = 688,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Serbia market. TRIGGERS: serbia.',
    c.description = 'Country node for Serbia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SC'})
SET c.display_name = 'Seychelles',
    c.alpha3 = 'SYC',
    c.m49_code = 690,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Seychelles market. TRIGGERS: seychelles.',
    c.description = 'Country node for Seychelles',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SL'})
SET c.display_name = 'Sierra Leone',
    c.alpha3 = 'SLE',
    c.m49_code = 694,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Sierra Leone market. TRIGGERS: sierra leone, sierra.',
    c.description = 'Country node for Sierra Leone',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SG'})
SET c.display_name = 'Singapore',
    c.alpha3 = 'SGP',
    c.m49_code = 702,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Singapore market. TRIGGERS: singapore.',
    c.description = 'Country node for Singapore',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SX'})
SET c.display_name = 'Sint Maarten (Dutch part)',
    c.alpha3 = 'SXM',
    c.m49_code = 534,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Sint Maarten (Dutch part) market. TRIGGERS: sint maarten (dutch part), sint.',
    c.description = 'Country node for Sint Maarten (Dutch part)',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SK'})
SET c.display_name = 'Slovakia',
    c.alpha3 = 'SVK',
    c.m49_code = 703,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Slovakia market. TRIGGERS: slovakia.',
    c.description = 'Country node for Slovakia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SI'})
SET c.display_name = 'Slovenia',
    c.alpha3 = 'SVN',
    c.m49_code = 705,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Slovenia market. TRIGGERS: slovenia.',
    c.description = 'Country node for Slovenia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SB'})
SET c.display_name = 'Solomon Islands',
    c.alpha3 = 'SLB',
    c.m49_code = 90,
    c.region = 'Oceania',
    c.sub_region = 'Melanesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Solomon Islands market. TRIGGERS: solomon islands, solomon.',
    c.description = 'Country node for Solomon Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SO'})
SET c.display_name = 'Somalia',
    c.alpha3 = 'SOM',
    c.m49_code = 706,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Somalia market. TRIGGERS: somalia.',
    c.description = 'Country node for Somalia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ZA'})
SET c.display_name = 'South Africa',
    c.alpha3 = 'ZAF',
    c.m49_code = 710,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Southern Africa',
    c.llm_context = 'USE: for South Africa market. TRIGGERS: south africa, south.',
    c.description = 'Country node for South Africa',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GS'})
SET c.display_name = 'South Georgia and the South Sandwich Islands',
    c.alpha3 = 'SGS',
    c.m49_code = 239,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for South Georgia and the South Sandwich Islands market. TRIGGERS: south georgia and the south sandwich islands, south.',
    c.description = 'Country node for South Georgia and the South Sandwich Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SS'})
SET c.display_name = 'South Sudan',
    c.alpha3 = 'SSD',
    c.m49_code = 728,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for South Sudan market. TRIGGERS: south sudan, south.',
    c.description = 'Country node for South Sudan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ES'})
SET c.display_name = 'Spain',
    c.alpha3 = 'ESP',
    c.m49_code = 724,
    c.region = 'Europe',
    c.sub_region = 'Southern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Spain market. TRIGGERS: spain.',
    c.description = 'Country node for Spain',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'LK'})
SET c.display_name = 'Sri Lanka',
    c.alpha3 = 'LKA',
    c.m49_code = 144,
    c.region = 'Asia',
    c.sub_region = 'Southern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Sri Lanka market. TRIGGERS: sri lanka, sri.',
    c.description = 'Country node for Sri Lanka',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SD'})
SET c.display_name = 'Sudan',
    c.alpha3 = 'SDN',
    c.m49_code = 729,
    c.region = 'Africa',
    c.sub_region = 'Northern Africa',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Sudan market. TRIGGERS: sudan.',
    c.description = 'Country node for Sudan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SR'})
SET c.display_name = 'Suriname',
    c.alpha3 = 'SUR',
    c.m49_code = 740,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Suriname market. TRIGGERS: suriname.',
    c.description = 'Country node for Suriname',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SJ'})
SET c.display_name = 'Svalbard and Jan Mayen',
    c.alpha3 = 'SJM',
    c.m49_code = 744,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Svalbard and Jan Mayen market. TRIGGERS: svalbard and jan mayen, svalbard.',
    c.description = 'Country node for Svalbard and Jan Mayen',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SE'})
SET c.display_name = 'Sweden',
    c.alpha3 = 'SWE',
    c.m49_code = 752,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Sweden market. TRIGGERS: sweden.',
    c.description = 'Country node for Sweden',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'CH'})
SET c.display_name = 'Switzerland',
    c.alpha3 = 'CHE',
    c.m49_code = 756,
    c.region = 'Europe',
    c.sub_region = 'Western Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Switzerland market. TRIGGERS: switzerland.',
    c.description = 'Country node for Switzerland',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'SY'})
SET c.display_name = 'Syrian Arab Republic',
    c.alpha3 = 'SYR',
    c.m49_code = 760,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Syrian Arab Republic market. TRIGGERS: syrian arab republic, syrian.',
    c.description = 'Country node for Syrian Arab Republic',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TW'})
SET c.display_name = 'Taiwan, Province of China',
    c.alpha3 = 'TWN',
    c.m49_code = 158,
    c.region = '',
    c.sub_region = '',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Taiwan, Province of China market. TRIGGERS: taiwan, province of china, taiwan,.',
    c.description = 'Country node for Taiwan, Province of China',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TJ'})
SET c.display_name = 'Tajikistan',
    c.alpha3 = 'TJK',
    c.m49_code = 762,
    c.region = 'Asia',
    c.sub_region = 'Central Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Tajikistan market. TRIGGERS: tajikistan.',
    c.description = 'Country node for Tajikistan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TZ'})
SET c.display_name = 'Tanzania, United Republic of',
    c.alpha3 = 'TZA',
    c.m49_code = 834,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Tanzania, United Republic of market. TRIGGERS: tanzania, united republic of, tanzania,.',
    c.description = 'Country node for Tanzania, United Republic of',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TH'})
SET c.display_name = 'Thailand',
    c.alpha3 = 'THA',
    c.m49_code = 764,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Thailand market. TRIGGERS: thailand.',
    c.description = 'Country node for Thailand',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TL'})
SET c.display_name = 'Timor-Leste',
    c.alpha3 = 'TLS',
    c.m49_code = 626,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Timor-Leste market. TRIGGERS: timor-leste.',
    c.description = 'Country node for Timor-Leste',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TG'})
SET c.display_name = 'Togo',
    c.alpha3 = 'TGO',
    c.m49_code = 768,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Western Africa',
    c.llm_context = 'USE: for Togo market. TRIGGERS: togo.',
    c.description = 'Country node for Togo',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TK'})
SET c.display_name = 'Tokelau',
    c.alpha3 = 'TKL',
    c.m49_code = 772,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Tokelau market. TRIGGERS: tokelau.',
    c.description = 'Country node for Tokelau',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TO'})
SET c.display_name = 'Tonga',
    c.alpha3 = 'TON',
    c.m49_code = 776,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Tonga market. TRIGGERS: tonga.',
    c.description = 'Country node for Tonga',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TT'})
SET c.display_name = 'Trinidad and Tobago',
    c.alpha3 = 'TTO',
    c.m49_code = 780,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Trinidad and Tobago market. TRIGGERS: trinidad and tobago, trinidad.',
    c.description = 'Country node for Trinidad and Tobago',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TN'})
SET c.display_name = 'Tunisia',
    c.alpha3 = 'TUN',
    c.m49_code = 788,
    c.region = 'Africa',
    c.sub_region = 'Northern Africa',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Tunisia market. TRIGGERS: tunisia.',
    c.description = 'Country node for Tunisia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TR'})
SET c.display_name = 'Türkiye',
    c.alpha3 = 'TUR',
    c.m49_code = 792,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Türkiye market. TRIGGERS: türkiye.',
    c.description = 'Country node for Türkiye',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TM'})
SET c.display_name = 'Turkmenistan',
    c.alpha3 = 'TKM',
    c.m49_code = 795,
    c.region = 'Asia',
    c.sub_region = 'Central Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Turkmenistan market. TRIGGERS: turkmenistan.',
    c.description = 'Country node for Turkmenistan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TC'})
SET c.display_name = 'Turks and Caicos Islands',
    c.alpha3 = 'TCA',
    c.m49_code = 796,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Turks and Caicos Islands market. TRIGGERS: turks and caicos islands, turks.',
    c.description = 'Country node for Turks and Caicos Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'TV'})
SET c.display_name = 'Tuvalu',
    c.alpha3 = 'TUV',
    c.m49_code = 798,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Tuvalu market. TRIGGERS: tuvalu.',
    c.description = 'Country node for Tuvalu',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'UG'})
SET c.display_name = 'Uganda',
    c.alpha3 = 'UGA',
    c.m49_code = 800,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Uganda market. TRIGGERS: uganda.',
    c.description = 'Country node for Uganda',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'UA'})
SET c.display_name = 'Ukraine',
    c.alpha3 = 'UKR',
    c.m49_code = 804,
    c.region = 'Europe',
    c.sub_region = 'Eastern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Ukraine market. TRIGGERS: ukraine.',
    c.description = 'Country node for Ukraine',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'AE'})
SET c.display_name = 'United Arab Emirates',
    c.alpha3 = 'ARE',
    c.m49_code = 784,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for United Arab Emirates market. TRIGGERS: united arab emirates, united.',
    c.description = 'Country node for United Arab Emirates',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'GB'})
SET c.display_name = 'United Kingdom of Great Britain and Northern Ireland',
    c.alpha3 = 'GBR',
    c.m49_code = 826,
    c.region = 'Europe',
    c.sub_region = 'Northern Europe',
    c.intermediate_region = '',
    c.llm_context = 'USE: for United Kingdom of Great Britain and Northern Ireland market. TRIGGERS: united kingdom of great britain and northern ireland, united.',
    c.description = 'Country node for United Kingdom of Great Britain and Northern Ireland',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'US'})
SET c.display_name = 'United States of America',
    c.alpha3 = 'USA',
    c.m49_code = 840,
    c.region = 'Americas',
    c.sub_region = 'Northern America',
    c.intermediate_region = '',
    c.llm_context = 'USE: for United States of America market. TRIGGERS: united states of america, united, usa, us.',
    c.description = 'Country node for United States of America',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'UM'})
SET c.display_name = 'United States Minor Outlying Islands',
    c.alpha3 = 'UMI',
    c.m49_code = 581,
    c.region = 'Oceania',
    c.sub_region = 'Micronesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for United States Minor Outlying Islands market. TRIGGERS: united states minor outlying islands, united.',
    c.description = 'Country node for United States Minor Outlying Islands',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'UY'})
SET c.display_name = 'Uruguay',
    c.alpha3 = 'URY',
    c.m49_code = 858,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Uruguay market. TRIGGERS: uruguay.',
    c.description = 'Country node for Uruguay',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'UZ'})
SET c.display_name = 'Uzbekistan',
    c.alpha3 = 'UZB',
    c.m49_code = 860,
    c.region = 'Asia',
    c.sub_region = 'Central Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Uzbekistan market. TRIGGERS: uzbekistan.',
    c.description = 'Country node for Uzbekistan',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'VU'})
SET c.display_name = 'Vanuatu',
    c.alpha3 = 'VUT',
    c.m49_code = 548,
    c.region = 'Oceania',
    c.sub_region = 'Melanesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Vanuatu market. TRIGGERS: vanuatu.',
    c.description = 'Country node for Vanuatu',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'VE'})
SET c.display_name = 'Venezuela, Bolivarian Republic of',
    c.alpha3 = 'VEN',
    c.m49_code = 862,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'South America',
    c.llm_context = 'USE: for Venezuela, Bolivarian Republic of market. TRIGGERS: venezuela, bolivarian republic of, venezuela,.',
    c.description = 'Country node for Venezuela, Bolivarian Republic of',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'VN'})
SET c.display_name = 'Viet Nam',
    c.alpha3 = 'VNM',
    c.m49_code = 704,
    c.region = 'Asia',
    c.sub_region = 'South-eastern Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Viet Nam market. TRIGGERS: viet nam, viet.',
    c.description = 'Country node for Viet Nam',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'VG'})
SET c.display_name = 'Virgin Islands (British)',
    c.alpha3 = 'VGB',
    c.m49_code = 92,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Virgin Islands (British) market. TRIGGERS: virgin islands (british), virgin.',
    c.description = 'Country node for Virgin Islands (British)',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'VI'})
SET c.display_name = 'Virgin Islands (U.S.)',
    c.alpha3 = 'VIR',
    c.m49_code = 850,
    c.region = 'Americas',
    c.sub_region = 'Latin America and the Caribbean',
    c.intermediate_region = 'Caribbean',
    c.llm_context = 'USE: for Virgin Islands (U.S.) market. TRIGGERS: virgin islands (u.s.), virgin.',
    c.description = 'Country node for Virgin Islands (U.S.)',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'WF'})
SET c.display_name = 'Wallis and Futuna',
    c.alpha3 = 'WLF',
    c.m49_code = 876,
    c.region = 'Oceania',
    c.sub_region = 'Polynesia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Wallis and Futuna market. TRIGGERS: wallis and futuna, wallis.',
    c.description = 'Country node for Wallis and Futuna',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'EH'})
SET c.display_name = 'Western Sahara',
    c.alpha3 = 'ESH',
    c.m49_code = 732,
    c.region = 'Africa',
    c.sub_region = 'Northern Africa',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Western Sahara market. TRIGGERS: western sahara, western.',
    c.description = 'Country node for Western Sahara',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'YE'})
SET c.display_name = 'Yemen',
    c.alpha3 = 'YEM',
    c.m49_code = 887,
    c.region = 'Asia',
    c.sub_region = 'Western Asia',
    c.intermediate_region = '',
    c.llm_context = 'USE: for Yemen market. TRIGGERS: yemen.',
    c.description = 'Country node for Yemen',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ZM'})
SET c.display_name = 'Zambia',
    c.alpha3 = 'ZMB',
    c.m49_code = 894,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Zambia market. TRIGGERS: zambia.',
    c.description = 'Country node for Zambia',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();

MERGE (c:Country {key: 'ZW'})
SET c.display_name = 'Zimbabwe',
    c.alpha3 = 'ZWE',
    c.m49_code = 716,
    c.region = 'Africa',
    c.sub_region = 'Sub-Saharan Africa',
    c.intermediate_region = 'Eastern Africa',
    c.llm_context = 'USE: for Zimbabwe market. TRIGGERS: zimbabwe.',
    c.description = 'Country node for Zimbabwe',
    c.created_by = 'seed:immutable',
    c.created_at = datetime(),
    c.updated_at = datetime();


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
