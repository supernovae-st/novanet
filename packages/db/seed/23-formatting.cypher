// ============================================================================
// FORMATTING SEED - Generated from ATH 2-rules-formatting
// Generated: 2026-02-08 00:48:57
// Source: /Users/thibaut/Projects/traduction_ai/ath-know-l10n/outputs/localization-data/2-rules-formatting/
// Locales: 200
// ============================================================================

// ----------------------------------------------------------------------------
// PART 1: Formatting nodes (200 locales)
// ----------------------------------------------------------------------------

MERGE (f:Formatting {key: 'ceb-PH'})
SET f.display_name = 'ceb-PH Formatting',
    f.content = 'Formatting rules for ceb-PH',
    f.llm_context = 'ceb-PH: Numbers use \'.\' decimal, \',\' thousands. Dates: MM/DD/YYYY (gregorian) Time: 12-hour Currency: peso sign or P before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"MM/DD/YYYY","short_pattern":"MM/DD/YY","long_pattern":"MMMM D, YYYY","full_pattern":null,"date_separator":"/","month_names":["Enero","Pebrero","Marso","Abril","Mayo","Hunyo","Hulyo","Agosto","Septyembre","Oktubre","Nobyembre","Disyembre"],"month_abbrev":[],"day_names":["Lunes","Martes","Miyerkules","Huwebes","Biyernes","Sabado","Dominggo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PHP","symbol":"peso sign or P","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[House/Building Number] [Street Name]\n[Barangay], [City/Municipality]\n[Province] [Postal Code]\nPHILIPPINES","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Maria Santos\n123 Osmeña Boulevard\nBrgy. Capitol Site, Cebu City\nCebu 6000\nPHILIPPINES"],"postal_code_examples":["6000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"deg C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| ISO 216 standard, though US Letter also used |","The Philippines uses the metric system for official and commercial purposes","US customary units (feet, inches, pounds) are commonly used informally due to American influence"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+63\\s?\\d{3}\\s?\\d{3}\\s?\\d{4}$","time":"^\\d{1,2}:\\d{2}(:\\d{2})?\\s?(AM\\|PM)$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","phone (national)":"^0\\d{3}\\s?\\d{3}\\s?\\d{4}$","currency":"^P\\d{1,3}(,\\d{3})*(\\.\\d{2})?$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ceb-PH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'mn-MN'})
SET f.display_name = 'mn-MN Formatting',
    f.content = 'Formatting rules for mn-MN',
    f.llm_context = 'mn-MN: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY.MM.DD (gregorian) Time: 24-hour Currency: ₮ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY.MM.DD","short_pattern":"YY.MM.DD","long_pattern":"YYYY оны M сарын D","full_pattern":null,"date_separator":".","month_names":["Нэгдүгээр сар","Хоёрдугаар сар","Гуравдугаар сар","Дөрөвдүгээр сар","Тавдугаар сар","Зургадугаар сар","Долдугаар сар","Наймдугаар сар","Есдүгээр сар","Аравдугаар сар","Арван нэгдүгээр сар","Арван хоёрдугаар сар"],"month_abbrev":[],"day_names":["Даваа","Мягмар","Лхагва","Пүрэв","Баасан","Бямба","Ням"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025.01.15"},{"input":"2025-12-31","output":"2025.12.31"}],"incorrect_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"01-15-2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"MNT","symbol":"₮","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 ₮"},{"input":"1234.56","output":"1 234,56 ₮"},{"input":"0.99","output":"0,99 ₮"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Country]\n[City], [District/Province]\n[Street Name], [Building Number]-[Apartment]\n[Postal Code]","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["МОНГОЛ УЛС\nУлаанбаатар хот, Сүхбаатар дүүрэг\nЭнхтайваны өргөн чөлөө 1\n14200"],"postal_code_examples":["14200"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["| Standard international |","Mongolia uses the metric system exclusively","Temperature always in Celsius (important for extreme cold climate: -30°C to -40°C in winter)","Road distances in kilometers, shorter distances in meters"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","date":"^\\d{4}\\.\\d{2}\\.\\d{2}$","phone (international)":"^\\+976 \\d{2} \\d{2} \\d{2} \\d{2}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? ₮$","phone (national)":"^\\d{2} \\d{2} \\d{2} \\d{2}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/mn-MN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-MY'})
SET f.display_name = 'en-MY Formatting',
    f.content = 'Formatting rules for en-MY',
    f.llm_context = 'en-MY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (day/month/year) (gregorian) Time: 12-hour (primary), 24-hour (also commonly used) Currency: RM (Ringgit Malaysia) before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (day/month/year)","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY (e.g., 15 January 2025)","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour (primary), 24-hour (also commonly used)","pattern":"h:mm a (12-hour format)","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"23:59:59","output":"11:59:59 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"MYR (ISO 4217)","symbol":"RM (Ringgit Malaysia)","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"RM10.50"},{"input":"1234.56","output":"RM1,234.56"},{"input":"0.99","output":"RM0.99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Unit/Building Number], [Street Name]\n[Taman/Area Name]\n[Postcode] [City/Town]\n[State]\nMALAYSIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Ahmad bin Ismail\nNo. 12, Jalan Bukit Bintang\nBukit Bintang\n55100 Kuala Lumpur\nWilayah Persekutuan\nMALAYSIA"],"postal_code_examples":["55100"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard for documents |","Malaysia uses the metric system exclusively","Imperial units rarely encountered except in legacy contexts or international references"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^RM\\d{1,3}(,\\d{3})*(\\.\\d{2})?$","phone (international)":"^\\+60\\s?\\d{1,2}-\\d{3,4}\\s?\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time (24h)":"^\\d{2}:\\d{2}$","time (12h)":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM)$","phone (national)":"^0\\d{1,2}-\\d{3,4}\\s?\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-MY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'bn-BD'})
SET f.display_name = 'bn-BD Formatting',
    f.content = 'Formatting rules for bn-BD',
    f.llm_context = 'bn-BD: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ৳ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["জানুয়ারি","ফেব্রুয়ারি","মার্চ","এপ্রিল","মে","জুন","জুলাই","আগস্ট","সেপ্টেম্বর","অক্টোবর","নভেম্বর","ডিসেম্বর"],"month_abbrev":[],"day_names":["সোমবার","মঙ্গলবার","বুধবার","বৃহস্পতিবার","শুক্রবার","শনিবার","রবিবার"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"পূর্বাহ্ন","pm_indicator":"অপরাহ্ন","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"BDT","symbol":"৳","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[House/Holding Number], [Road/Street Name]\n[Area/Mohalla/Block]\n[Thana/Upazila], [District] - [Postal Code]\nBANGLADESH","postal_code_pattern":"NNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["মোহাম্মদ করিম\nবাড়ি নং ১২, রোড নং ৫\nধানমন্ডি আবাসিক এলাকা\nধানমন্ডি, ঢাকা - ১২০৫\nBANGLADESH"],"postal_code_examples":["1205"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Bangladesh uses metric system exclusively for official and commercial purposes","Traditional measurements still used in land (শতক/shotok, কাঠা/katha, বিঘা/bigha) and gold (ভরি/bhori, আনা/ana)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^৳[\\d০-৯]{1,3}(,[\\d০-৯]{2})*,?[\\d০-৯]{3}(\\.[\\d০-৯]{2})?$","phone (international)":"^\\+880\\s\\d{4}\\s\\d{6}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^0\\d{4}-\\d{6}$","number":"^-?[\\d০-৯]{1,3}(,[\\d০-৯]{2})*,?[\\d০-৯]{3}(\\.[\\d০-৯]+)?$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM\\|পূর্বাহ্ন\\|অপরাহ্ন)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/bn-BD.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-TZ'})
SET f.display_name = 'en-TZ Formatting',
    f.content = 'Formatting rules for en-TZ',
    f.llm_context = 'en-TZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: TSh before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"TZS","symbol":"TSh","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building/Plot Number] [Street Name]\n[Area/Neighbourhood]\n[City/Town]\n[Postal Code]\nTANZANIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Plot 45 Ali Hassan Mwinyi Road\nMasaki\nDar es Salaam\n14101\nTANZANIA","Tanzania National Parks Authority (TANAPA)\nP.O. Box 3134\nArusha\n23101\nTANZANIA","House 12, Block B\nMikocheni B\nDar es Salaam\n14101\nTANZANIA","Hurumzi Street\nStone Town\nZanzibar\n71101\nTANZANIA","Parliament Building\nDodoma\n41101\nTANZANIA"],"postal_code_examples":["14101"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null},{"category":"Mfuko","unit":"Varies","symbol":"Bag; context-dependent (cement = 50kg)","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","tin (tax id)":"^[0-9]{9}$","passport":"^TZ[0-9]{8}$","phone (international)":"^\\+255 [2-7][0-9]{2} [0-9]{3} [0-9]{3}$","currency":"^TSh [0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","phone (national)":"^0[2-7][0-9]{2} [0-9]{3} [0-9]{3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-TZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-SG'})
SET f.display_name = 'en-SG Formatting',
    f.content = 'Formatting rules for en-SG',
    f.llm_context = 'en-SG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"am","pm_indicator":"pm","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"SGD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Block/House Number + Street Name\n#Floor-Unit (for HDB/condos)\nSingapore + 6-digit Postal Code\nSINGAPORE","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Blk 456 Tampines Street 42\n#12-345\nSingapore 520456\nSINGAPORE"],"postal_code_examples":["049145"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Singapore uses metric system exclusively for all official purposes","Road signs display distances in kilometres (e.g., \"PIE 5 km\")","Fuel sold in litres at petrol stations, fuel economy in km/L","Body weight in kilograms (never stone/pounds)","Food portions often in grams (e.g., \"500g chicken rice\")","Property size in square feet (sqft) OR square metres (sqm) - both accepted","Swimming pool lengths in metres"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(am\\|pm)$","phone (national)":"^[689]\\d{3}\\s\\d{4}$","nric (citizen)":"^(S\\|T)\\d{7}[A-Z]$","uen (business)":"^[\\dA-Z]{9,10}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","fin (foreigner)":"^(F\\|G\\|M)\\d{7}[A-Z]$","currency":"^S?\\$[\\d,]+(\\.\\d{2})?$","time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","number":"^-?[\\d,]+(\\.\\d+)?$","phone (international)":"^\\+65\\s[689]\\d{3}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-SG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ta-LK'})
SET f.display_name = 'ta-LK Formatting',
    f.content = 'Formatting rules for ta-LK',
    f.llm_context = 'ta-LK: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ரூ or Rs before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM, YYYY","full_pattern":null,"date_separator":"/","month_names":["ஜனவரி","பிப்ரவரி","மார்ச்","ஏப்ரல்","மே","ஜூன்","ஜூலை","ஆகஸ்ட்","செப்டம்பர்","அக்டோபர்","நவம்பர்","டிசம்பர்"],"month_abbrev":[],"day_names":["திங்கள்","செவ்வாய்","புதன்","வியாழன்","வெள்ளி","சனி","ஞாயிறு"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"முற்பகல்","pm_indicator":"பிற்பகல்","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 பிற்பகல்"},{"input":"09:00","output":"9:00 முற்பகல்"},{"input":"23:59:59","output":"11:59:59 பிற்பகல்"},{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"LKR","symbol":"ரூ or Rs","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"ரூ 10.50"},{"input":"1234.56","output":"ரூ 1,234.56"},{"input":"0.99","output":"ரூ 0.99"},{"input":"10.50","output":"Rs. 10.50"},{"input":"1234.56","output":"Rs. 1,234.56"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building Number], [Street Name]\n[Area/Suburb]\n[City]\n[Postal Code]\nSRI LANKA","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["திரு. கே. சிவராஜா\n45, காலி வீதி\nகொழும்பு 04\n00400\nSRI LANKA","திருமதி எஸ். செல்வராணி\n12, சுதந்திர வீதி\nநல்லூர்\nயாழ்ப்பாணம்\n40000\nSRI LANKA"],"postal_code_examples":["00100"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Sri Lanka uses metric system exclusively","Exception: Some traditional measurements used in property (perches for land area)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^\\d{1,2}:\\d{2}\\s?(முற்பகல்\\|பிற்பகல்)$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","phone (national)":"^0\\d{2}\\s\\d{3}\\s\\d{4}$","phone (international)":"^\\+94\\s\\d{2}\\s\\d{3}\\s\\d{4}$","currency":"^(ரூ\\|Rs\\.)\\s\\d{1,3}(,\\d{3})*(\\.\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ta-LK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-ZA'})
SET f.display_name = 'en-ZA Formatting',
    f.content = 'Formatting rules for en-ZA',
    f.llm_context = 'en-ZA: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY/MM/DD (official), DD/MM/YYYY (common) (gregorian) Time: 24-hour (official/formal), 12-hour (informal) Currency: R before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY/MM/DD (official), DD/MM/YYYY (common)","short_pattern":"YYYY/MM/DD or DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour (official/formal), 12-hour (informal)","pattern":"HH:mm (24-hour) or h:mm a (12-hour)","pattern_with_seconds":"HH:mm:ss or h:mm:ss a","time_separator":":","am_indicator":"am","pm_indicator":"pm","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"ZAR","symbol":"R","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Number] [Street Name]\n[Suburb/Township]\n[City]\n[Postal Code]\nSOUTH AFRICA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123 Church Street\nHatfield\nPretoria\n0083\nSOUTH AFRICA","Unit 5, 456 Long Street\nStellenbosch\n7600\nSOUTH AFRICA","789 Adderley Street, Cape Town CBD, Cape Town, 8001\nSOUTH AFRICA"],"postal_code_examples":["0001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (ISO 216 standard) |","South Africa uses the metric system exclusively","Distance on road signs: kilometres","Speed limits: km/h (commonly 60, 80, 100, 120)","Body weight: kilograms","Height: centimetres or metres (e.g., 1,75 m)","Exception: aviation uses feet and nautical miles worldwide","Cooking: millilitres, grams, litres (metric only)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","0,5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","0°C","37°C","100°C","-5°C"]}',
    f.validation_patterns = '{"date":"^\\d{4}\\/(0[1-9]\\|1[0-2])\\/(0[1-9]\\|[12][0-9]\\|3[01])$","number":"^-?[0-9]{1,3}( [0-9]{3})*(,[0-9]+)?$","phone (national)":"^0[0-9]{2} [0-9]{3} [0-9]{4}$","phone (international)":"^\\+27 [0-9]{2} [0-9]{3} [0-9]{4}$","currency":"^R[0-9]{1,3}( [0-9]{3})*,[0-9]{2}$","time":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-ZA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-BB'})
SET f.display_name = 'en-BB Formatting',
    f.content = 'Formatting rules for en-BB',
    f.llm_context = 'en-BB: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (British heritage) (gregorian) Time: 12-hour (primary), 24-hour (transport, official) Currency: $ or Bds$ (when distinguishing from USD) before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","Barbados National Standards Institution"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (British heritage)","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour (primary), 24-hour (transport, official)","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"BBD (Barbados Dollar)","symbol":"$ or Bds$ (when distinguishing from USD)","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Number] [Street Name]\n[District/Village] (optional)\n[Parish]\n[Postal Code] (optional)\nBARBADOS","postal_code_pattern":"BB + 5 digits (e.g., BB11000)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["1 Warrens\nWarrens\nSt. Michael\nBB22026\nBARBADOS","45 First Street\nHoletown\nSt. James\nBB24016\nBARBADOS","12 Church Street\nSpeightstown\nSt. Peter\nBB25000\nBARBADOS","Cherry Tree Hill\nSt. Andrew\nBARBADOS"],"postal_code_examples":["BB11000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"Letter","notes":["Barbados officially uses metric system (SI units)","British colonial heritage means some imperial usage persists in conversation","Paper: US Letter standard (8.5\" x 11\") due to US business influence - NOT A4","Height: Often feet/inches colloquially (\"5 foot 10\")","Land: Acres commonly used (plantation heritage)","Driving: Left-hand traffic (British system)","Exception: Aviation uses feet and nautical miles (international standard)","Cooking: Mix of metric and US cup measures due to US/UK media"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","15","8.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","phone (international)":"^\\+1 246 [0-9]{3} [0-9]{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","currency":"^(Bds)?\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","trn":"^[A-Z0-9]{11}$","passport":"^BRB[0-9]+$","phone (national)":"^\\(246\\) [0-9]{3}-[0-9]{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-BB.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'si-LK'})
SET f.display_name = 'si-LK Formatting',
    f.content = 'Formatting rules for si-LK',
    f.llm_context = 'si-LK: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY-MM-DD (gregorian) Time: 24-hour Currency: රු or Rs before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY-MM-DD","short_pattern":"YYYY/MM/DD","long_pattern":"YYYY MMMM D","full_pattern":null,"date_separator":"-","month_names":["ජනවාරි","පෙබරවාරි","මාර්තු","අප්‍රේල්","මැයි","ජූනි","ජූලි","අගෝස්තු","සැප්තැම්බර්","ඔක්තෝබර්","නොවැම්බර්","දෙසැම්බර්"],"month_abbrev":[],"day_names":["සඳුදා","අඟහරුවාදා","බදාදා","බ්‍රහස්පතින්දා","සිකුරාදා","සෙනසුරාදා","ඉරිදා"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025-01-15"},{"input":"2025-12-31","output":"2025-12-31"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH.mm","pattern_with_seconds":"HH.mm.ss","time_separator":".","am_indicator":"පෙ.ව.","pm_indicator":"ප.ව.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14.30"},{"input":"09:00","output":"09.00"},{"input":"23:59:59","output":"23.59.59"},{"input":"14:30","output":"2.30 ප.ව."},{"input":"09:00","output":"9.00 පෙ.ව."}],"incorrect_examples":[]}',
    f.currency = '{"code":"LKR","symbol":"රු or Rs","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"රු 10.50"},{"input":"1234.56","output":"රු 1,234.56"},{"input":"0.99","output":"රු 0.99"},{"input":"10.50","output":"Rs. 10.50"},{"input":"1234.56","output":"Rs. 1,234.56"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building Number], [Street Name]\n[Area/Suburb]\n[City]\n[Postal Code]\nSRI LANKA","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ආර්. පෙරේරා මහතා\n45, ගාලු පාර\nකොල්ලුපිටිය\nකොළඹ 03\n00300\nSRI LANKA","Mr. R. Perera\n45, Galle Road\nKollupitiya\nColombo 03\n00300\nSRI LANKA"],"postal_code_examples":["00100"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Sri Lanka uses metric system exclusively","Exception: Some traditional measurements used in property (perches for land area)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+94\\s\\d{2}\\s\\d{3}\\s\\d{4}$","currency":"^(රු\\|Rs\\.)\\s\\d{1,3}(,\\d{3})*(\\.\\d{2})?$","time":"^\\d{2}\\.\\d{2}(\\.\\d{2})?$","date":"^\\d{4}-\\d{2}-\\d{2}$","phone (national)":"^0\\d{2}\\s\\d{3}\\s\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/si-LK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'bs-BA'})
SET f.display_name = 'bs-BA Formatting',
    f.content = 'Formatting rules for bs-BA',
    f.llm_context = 'bs-BA: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (period separator) (gregorian) Time: 24-hour Currency: KM after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (period separator)","short_pattern":"DD.MM.YY","long_pattern":"D. MMMM YYYY (e.g., \"15. januar 2025\")","full_pattern":null,"date_separator":".","month_names":["januar","februar","mart","april","maj","juni","juli","august","septembar","oktobar","novembar","decembar"],"month_abbrev":[],"day_names":["ponedjeljak","utorak","srijeda","cetvrtak","petak","subota","nedjelja"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"BAM (Bosna i Hercegovina konvertibilna marka)","symbol":"KM","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 KM"},{"input":"1234.56","output":"1.234,56 KM"},{"input":"0.99","output":"0,99 KM"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nBOSNIA AND HERZEGOVINA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Marsala Tita 25\n71000 Sarajevo\nBOSNIA AND HERZEGOVINA"],"postal_code_examples":["71000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210x297mm) |","Bosnia and Herzegovina uses metric system exclusively","Imperial units not used in everyday contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^0\\d{2} \\d{3} \\d{3}$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} KM$","phone (international)":"^\\+387 \\d{2} \\d{3} \\d{3}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/bs-BA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'it-CH'})
SET f.display_name = 'it-CH Formatting',
    f.content = 'Formatting rules for it-CH',
    f.llm_context = 'it-CH: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day, month, year with period separators) (gregorian) Time: 24-hour Currency: CHF or fr. before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day, month, year with period separators)","short_pattern":"DD.MM.YY (two-digit year)","long_pattern":"D MMMM YYYY (e.g., \"15 gennaio 2025\")","full_pattern":null,"date_separator":".","month_names":["gennaio","febbraio","marzo","aprile","maggio","giugno","luglio","agosto","settembre","ottobre","novembre","dicembre"],"month_abbrev":[],"day_names":["lunedi","martedi","mercoledi","giovedi","venerdi","sabato","domenica"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format with leading zeros)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"CHF (ISO 4217)","symbol":"CHF or fr.","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"CHF 10.50"},{"input":"1234.56","output":"CHF 1\'234.55"},{"input":"0.99","output":"CHF 1.00"},{"input":"19.95","output":"CHF 19.95"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[First Name] [Last Name]\n[Street Name] [House Number]\n[Postal Code] [City]\nSVIZZERA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Mario Bianchi\nVia Nassa 15\n6900 Lugano\nSVIZZERA"],"postal_code_examples":["6900"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L or l","notes":null}],"paper_size":"A4","notes":["| Standard: 210 x 297 mm |","Metric system used exclusively for all measurements","No imperial units in common use","Fuel consumption expressed as liters per 100 km (L/100 km)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C","36.5°C"]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\'\\d{3})*(\\.\\d+)?$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","currency":"^CHF\\s\\d{1,3}(\'\\d{3})*(\\.\\d{2})?$","phone (international)":"^\\+41\\s\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","phone (national)":"^0\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/it-CH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'tr-TR'})
SET f.display_name = 'tr-TR Formatting',
    f.content = 'Formatting rules for tr-TR',
    f.llm_context = 'tr-TR: Numbers use \'.\' decimal, \',\' thousands. Dates: D.MM.YYYY (day.month.year) (gregorian) Time: 24-hour Currency: ₺ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D.MM.YYYY (day.month.year)","short_pattern":"D.MM.YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":".","month_names":["Ocak","Şubat","Mart","Nisan","Mayıs","Haziran","Temmuz","Ağustos","Eylül","Ekim","Kasım","Aralık"],"month_abbrev":[],"day_names":["Pazartesi","Salı","Çarşamba","Perşembe","Cuma","Cumartesi","Pazar"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"TRY (Turkish Lira)","symbol":"₺","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 ₺"},{"input":"1234.56","output":"1.234,56 ₺"},{"input":"0.99","output":"0,99 ₺"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] No: [Number], [Neighborhood/Quarter]\n[Postal Code] [District]/[City]\nTURKEY","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Atatürk Bulvarı No: 125, Çankaya\n06100 Çankaya/Ankara\nTURKEY"],"postal_code_examples":["34000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard international size (210×297mm) |","Turkey uses the metric system exclusively","International contexts may require dual units (metric first, imperial in parentheses)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^0[2-5]\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","date":"^\\d{1,2}\\.\\d{2}\\.\\d{4}$","phone (international)":"^\\+90\\s\\d{3}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$","currency":"^\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\s₺$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/tr-TR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-MX'})
SET f.display_name = 'es-MX Formatting',
    f.content = 'Formatting rules for es-MX',
    f.llm_context = 'es-MX: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: Hybrid (12-hour common in daily speech, 24-hour in formal/transport) Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"Hybrid (12-hour common in daily speech, 24-hour in formal/transport)","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p.m."},{"input":"09:00","output":"9:00 a.m."},{"input":"23:59:59","output":"11:59:59 p.m."},{"input":"14:30","output":"14:30"},{"input":"12:00","output":"12:00 p.m."},{"input":"00:00","output":"12:00 a.m."}],"incorrect_examples":[]}',
    f.currency = '{"code":"MXN","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"$10.50"},{"input":"1234.56","output":"$1,234.56"},{"input":"0.99","output":"$0.99"},{"input":"1500.00","output":"$1,500.00"},{"input":"999999.99","output":"$999,999.99"},{"input":"49.00","output":"$49.00"},{"input":"15750.00","output":"$15,750.00"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name] [Number] [Interior/Apartment]\n[Colonia]\n[Postal Code] [City/Delegacion], [State]\nMEXICO","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Av. Paseo de la Reforma 505 Piso 23\nCol. Cuauhtemoc\n06500 Ciudad de Mexico, CDMX\nMEXICO","Calle Hidalgo 123\nCol. Centro\n64000 Monterrey, Nuevo Leon\nMEXICO"],"postal_code_examples":["01000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US Letter (8.5\" x 11\") more common than A4 |","Metric system is official and standard throughout Mexico","Exception: Paper size commonly uses US Letter format (carta) due to US proximity","US border cities may show some imperial influence (miles, gallons) informally","Gas stations sell fuel by liter, not gallon","Speed limits shown in km/h, not mph","Height often expressed in meters (1.75 m, not 5\'9\")"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^\\(\\d{2,3}\\)\\s\\d{3,4}-\\d{4}$","phone (international)":"^\\+52\\s\\d{2,3}\\s\\d{4}\\s\\d{4}$","time (12h)":"^\\d{1,2}:\\d{2}(:\\d{2})?\\s[ap]\\.m\\.$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","currency":"^\\$\\d{1,3}(,\\d{3})*\\.\\d{2}$","time (24h)":"^\\d{2}:\\d{2}(:\\d{2})?$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-MX.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-MA'})
SET f.display_name = 'ar-MA Formatting',
    f.content = 'Formatting rules for ar-MA',
    f.llm_context = 'ar-MA: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 24-hour Currency: د.م. after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["يناير","فبراير","مارس","أبريل","ماي","يونيو","يوليوز","غشت","شتنبر","أكتوبر","نونبر","دجنبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"MAD","symbol":"د.م.","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Number] [Street Name]\n[Neighborhood/District]\n[Postal Code] [City]\nالمغرب","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["45 شارع محمد الخامس\nحي المعاريف\n20250 الدار البيضاء\nالمغرب"],"postal_code_examples":["20000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","Morocco uses the metric system exclusively (French colonial legacy)","Temperature always in Celsius (°C) for weather, medical, cooking","Distance in kilometers (km) for road signs, maps","Local traditional units exist but are rarely used: Quintal (قنطار) = 100 kg for agricultural products"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50","12.5","12,5"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","22°C","-5°C","-5°C"]}',
    f.validation_patterns = '{"number":"^-?[0-9]{1,3}(\\.[0-9]{3})*(,[0-9]+)?$","phone (national)":"^0[567]\\d{2}-\\d{6}$","currency":"^[0-9]{1,3}(\\.[0-9]{3})*(,[0-9]{2})?\\sد\\.م\\.$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (international)":"^\\+212\\s[567]\\d{2}-\\d{6}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-MA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-LY'})
SET f.display_name = 'ar-LY Formatting',
    f.content = 'Formatting rules for ar-LY',
    f.llm_context = 'ar-LY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: د.ل after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"arabic-indic","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["يناير","فبراير","مارس","أبريل","مايو","يونيو","يوليو","أغسطس","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"١٥/٠١/٢٠٢٥"},{"input":"2025-12-31","output":"٣١/١٢/٢٠٢٥"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"٢:٣٠ م"},{"input":"09:00","output":"٩:٠٠ ص"},{"input":"23:59:59","output":"١١:٥٩:٥٩ م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"LYD","symbol":"د.ل","symbol_position":"after","space_between":true,"decimal_places":33,"subunit":"Dirham (درهم) - 1000 dirhams = 1 dinar (NOT fils like Gulf states)","correct_examples":[{"input":"10.500","output":"١٠٫٥٠٠ د.ل"},{"input":"1234.567","output":"١٬٢٣٤٫٥٦٧ د.ل"},{"input":"0.999","output":"٠٫٩٩٩ د.ل"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building/House Number] [Street Name]\n[District/Neighborhood]\n[City]\nليبيا","postal_code_pattern":"None","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["٤٥ شارع عمر المختار\nحي الأندلس\nطرابلس\nليبيا","١٢ شارع جمال عبد الناصر\nمنطقة الفويهات\nبنغازي\nليبيا","بجوار مسجد الكبير\nشارع الاستقلال\nمصراتة\nليبيا"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°م","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"كم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"كجم","notes":null},{"category":"Volume","unit":"Liters","symbol":"ل","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","Libya uses the metric system exclusively","Temperature always in Celsius (°م) for weather, medical, cooking","Distance in kilometers (كم) for road signs, maps","Road distances often measured from Tripoli or Benghazi as reference points","National Oil Corporation, Brega, Ras Lanuf terminals) uses imperial units in technical contexts:","Production: barrels per day (bpd / برميل يوميا)","Drilling: feet for depth measurements","Pipelines: inches for diameter"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","٥٠","12.5"]}',
    f.temperature = '{"format":"{number}°م","default_unit":"celsius","examples":["22°C","٢٢°C","-5°C","45°C","٤٥°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+٢١٨\\s\\d{2}\\s\\d{3}\\s\\d{4}$","number":"^[؜\\-+]?[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]+)?$","time":"^[١-٩٠]{1,2}:[٠-٥٩]{2}\\s[صم]$","phone (national)":"^٠[٩٢][١٢٤]\\s\\d{3}\\s\\d{4}$","currency":"^[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]{3})?\\sد\\.ل$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-LY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-MG'})
SET f.display_name = 'fr-MG Formatting',
    f.content = 'Formatting rules for fr-MG',
    f.llm_context = 'fr-MG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: Ar (Ariary) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["janvier","fevrier","mars","avril","mai","juin","juillet","aout","septembre","octobre","novembre","decembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"MGA (ISO 4217)","symbol":"Ar (Ariary)","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Type] [Street Name]\n[Neighborhood/Fokontany]\n[Postal Code] [City]\nMADAGASCAR","postal_code_pattern":"NNN (3 digits - unique to Madagascar!)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["12 rue Rainandriamampandry\nAnalakely\n101 Antananarivo\nMADAGASCAR","Lot IVG 45 Ambohipo\nFokontany Ambohipo Sud\n101 Antananarivo\nMADAGASCAR","Villa Baobab, Route de l\'Universite\nAmbondrona\n501 Toliara\nMADAGASCAR","Fokontany Ambatolampy\nCommune Ankazobe\n105 Ankazobe\nMADAGASCAR"],"postal_code_examples":["101"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogrammes","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null},{"category":"Hectares","unit":"ha","symbol":"Common for agricultural land","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","Madagascar uses the metric system exclusively for all official measurements","Imperial units (miles, pounds, Fahrenheit) are NOT used","Land is commonly measured in \"ares\" (a) or \"hectares\" (ha) for agriculture"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"passport":"^[A-Z]\\d{7}$","stat (company)":"^\\d{13}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","currency":"^\\d{1,3}( \\d{3})* Ar$","nif (tax id)":"^\\d{7,10}$","rcs (commerce)":"^\\d{4} [A-Z] \\d{5}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","cin (national id)":"^\\d{3}( \\d{3}){3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-MG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-EG'})
SET f.display_name = 'ar-EG Formatting',
    f.content = 'Formatting rules for ar-EG',
    f.llm_context = 'ar-EG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: ج.م after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YYYY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["يناير","فبراير","مارس","أبريل","مايو","يونيو","يوليو","أغسطس","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"15/1/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 م"},{"input":"09:00","output":"9:00 ص"},{"input":"23:30","output":"11:30 م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EGP","symbol":"ج.م","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"150","output":"150 ج.م"},{"input":"2500","output":"2,500 ج.م"},{"input":"25.50","output":"25.50 ج.م"},{"input":"150000","output":"150,000 ج.م"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Building Number], [Street Name]\n[Floor/Apartment details]\n[District], [City]\n[Governorate] [Postal Code]\nمصر","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["محمد أحمد السيد\n٤٥ شارع شريف\nالدور الثالث، شقة ٧\nوسط البلد، القاهرة\nمحافظة القاهرة ١١١١١\nمصر","شركة الأسكندرية للتجارة\n١٢٣ طريق الحرية\nسموحة\nالإسكندرية ٢١٥١١\nمصر","أحمد محمود\n٧٨ شارع فيصل\nبجوار مسجد الرحمن\nفيصل، الجيزة ١٢٥١١\nمصر"],"postal_code_examples":["11111"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Egypt uses the metric system exclusively for official purposes","Temperature: Celsius (°C) - summer 35-45°C","Distance: Kilometers for roads and maps","Weight: Kilograms, grams","قنطار (qintar): ~44.93 kg (cotton, agricultural goods)","فدان (feddan): ~4,200 m² (land area, real estate)","أردب (ardeb): ~198 liters (grain volume)","قيراط (qirat): ~175 m² (urban land subdivision)","رطل (rotl): ~449 grams (traditional markets)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["14","14","25","25","7.5","7.5"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["35°C","35°C","42°C","42°C","15°C","15°C"]}',
    f.validation_patterns = '{"time":"^\\d{1,2}:\\d{2}\\s[صم]$","phone (international)":"^\\+20\\s1[0125]\\s\\d{4}\\s\\d{4}$","date":"^\\d{1,2}/\\d{1,2}/\\d{4}$","currency":"^[\\d,]+(\\.\\d{2})?\\sج\\.م$","phone (mobile)":"^01[0125]\\s\\d{4}\\s\\d{4}$","number":"^-?[\\d,]+(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-EG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'my-MM'})
SET f.display_name = 'my-MM Formatting',
    f.content = 'Formatting rules for my-MM',
    f.llm_context = 'my-MM: Numbers use \'.\' decimal, \',\' thousands. Dates: d/M/yyyy (gregorian) Time: 24-hour Currency: K or Ks after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d/M/yyyy","short_pattern":"d/M/yy","long_pattern":"yyyy၊ MMMM d","full_pattern":null,"date_separator":"/","month_names":["ဇန်နဝါရီ","ဖေဖော်ဝါရီ","မတ်","ဧပြီ","မေ","ဇွန်","ဇူလိုင်","ဩဂုတ်","စက်တင်ဘာ","အောက်တိုဘာ","နိုဝင်ဘာ","ဒီဇင်ဘာ"],"month_abbrev":[],"day_names":["တနင်္လာ","အင်္ဂါ","ဗုဒ္ဓဟူး","ကြာသပတေး","သောကြာ","စနေ","တနင်္ဂနွေ"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15/1/2026"},{"input":"2026-12-31","output":"31/12/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"15.01.2026"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"နံနက်","pm_indicator":"ညနေ","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"MMK","symbol":"K or Ks","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"1050","output":"1,050 K"},{"input":"123456","output":"123,456 K"},{"input":"500","output":"500 K"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building/House Number], [Street Name]\n[Ward (ရပ်ကွက်)], [Township (မြို့နယ်)]\n[City/Region]\n[Postal Code]\nMYANMAR","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123, Anawrahta Road\nလသာရပ်ကွက်, လသာမြို့နယ်\nရန်ကုန်\n11141\nMYANMAR","123, Anawrahta Road\nLatha Ward, Latha Township\nYangon\n11141\nMYANMAR"],"postal_code_examples":["11141"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard paper size |","Myanmar officially uses the metric system for all government and commercial purposes","Traditional Myanmar units (tical/ပဲ for gold weight, viss/ပိဿာ for general weight) are still used in specific markets and traditional contexts"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\d{1,3}(,\\d{3})*\\s(K|Ks|ကျပ်)$","phone (national)":"^0\\d{1,2}-\\d{3}-\\d{3,4}(-\\d{3})?$","phone (international)":"^\\+95\\s\\d{1,2}\\s\\d{3}\\s\\d{3,4}(\\s\\d{3})?$","date":"^\\d{1,2}/\\d{1,2}/\\d{4}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/my-MM.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ta-IN'})
SET f.display_name = 'ta-IN Formatting',
    f.content = 'Formatting rules for ta-IN',
    f.llm_context = 'ta-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM, YYYY","full_pattern":null,"date_separator":"/","month_names":["ஜனவரி","பிப்ரவரி","மார்ச்","ஏப்ரல்","மே","ஜூன்","ஜூலை","ஆகஸ்ட்","செப்டம்பர்","அக்டோபர்","நவம்பர்","டிசம்பர்"],"month_abbrev":[],"day_names":["திங்கள்","செவ்வாய்","புதன்","வியாழன்","வெள்ளி","சனி","ஞாயிறு"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"முற்பகல்","pm_indicator":"பிற்பகல்","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["திரு. முருகன் செல்வம்\n123, அண்ணா சாலை\nதியாகராய நகர்\nசென்னை - 600017\nதமிழ்நாடு\nINDIA"],"postal_code_examples":["600001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","India uses metric system exclusively","Traditional Tamil measurements still used locally (e.g., நாழிகை for time, பரணி for volume)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","phone (national)":"^\\d{5}\\s\\d{5}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM\\|முற்பகல்\\|பிற்பகல்)$","phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ta-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-CD'})
SET f.display_name = 'fr-CD Formatting',
    f.content = 'Formatting rules for fr-CD',
    f.llm_context = 'fr-CD: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: FC after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["janvier","fevrier","mars","avril","mai","juin","juillet","aout","septembre","octobre","novembre","decembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"CDF","symbol":"FC","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[Commune/Quartier]\n[City]\nREPUBLIQUE DEMOCRATIQUE DU CONGO","postal_code_pattern":"N/A (no standardized postal code system)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["45 Avenue du Commerce\nCommune de la Gombe\nKinshasa\nREPUBLIQUE DEMOCRATIQUE DU CONGO"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","DRC uses the metric system for all official measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\d{1,3}( \\d{3})*,\\d{2} FC$","number":"^-?\\d{1,3}( \\d{3})*,\\d+$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (international)":"^\\+243 \\d{3}( \\d{3}){2}$","phone (national)":"^0\\d{2,3}( \\d{3}){2}$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-CD.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'as-IN'})
SET f.display_name = 'as-IN Formatting',
    f.content = 'Formatting rules for as-IN',
    f.llm_context = 'as-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD-MM-YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD-MM-YYYY","short_pattern":"DD-MM-YY","long_pattern":"D MMMM, YYYY","full_pattern":null,"date_separator":"-","month_names":["জানুৱাৰী","ফেব্ৰুৱাৰী","মাৰ্চ","এপ্ৰিল","মে","জুন","জুলাই","আগষ্ট","ছেপ্টেম্বৰ","অক্টোবৰ","নৱেম্বৰ","ডিচেম্বৰ"],"month_abbrev":[],"day_names":["সোমবাৰ","মঙ্গলবাৰ","বুধবাৰ","বৃহস্পতিবাৰ","শুক্ৰবাৰ","শনিবাৰ","দেওবাৰ"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15-01-2025"},{"input":"2025-12-31","output":"31-12-2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01-15-2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"পূৰ্বাহ্ন","pm_indicator":"অপৰাহ্ন","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"23:59:59","output":"11:59:59 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"₹10.50"},{"input":"1234.56","output":"₹1,234.56"},{"input":"0.99","output":"₹0.99"},{"input":"100000","output":"₹1,00,000"},{"input":"10000000","output":"₹1,00,00,000"},{"input":"750000.50","output":"₹7,50,000.50"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["শ্ৰী ৰাজীৱ বৰা\n123, এম জি ৰোড\nপান বজাৰ\nগুৱাহাটী - 781001\nঅসম\nINDIA"],"postal_code_examples":["781001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","India uses metric system exclusively","Exceptions: Some traditional measurements still used in agriculture (e.g., bigha/বিঘা for land in Assam)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","phone (national)":"^\\d{5}\\s\\d{5}$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","date":"^\\d{2}-\\d{2}-\\d{4}$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM\\|পূৰ্বাহ্ন\\|অপৰাহ্ন)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/as-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'lt-LT'})
SET f.display_name = 'lt-LT Formatting',
    f.content = 'Formatting rules for lt-LT',
    f.llm_context = 'lt-LT: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY-MM-DD (ISO format, commonly used in Lithuania) (gregorian) Time: 24-hour Currency: EUR or euro (the euro symbol is less commonly used in Lithuanian text) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY-MM-DD (ISO format, commonly used in Lithuania)","short_pattern":"YYYY-MM-DD or YY-MM-DD","long_pattern":"YYYY m. MMMM D d. (e.g., \"2025 m. sausio 15 d.\")","full_pattern":null,"date_separator":"-","month_names":["sausis","vasaris","kovas","balandis","gegužė","birželis","liepa","rugpjūtis","rugsėjis","spalis","lapkritis","gruodis"],"month_abbrev":[],"day_names":["pirmadienis","antradienis","trečiadienis","ketvirtadienis","penktadienis","šeštadienis","sekmadienis"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025-01-15"},{"input":"2025-12-31","output":"2025-12-31"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (Euro)","symbol":"EUR or euro (the euro symbol is less commonly used in Lithuanian text)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 EUR"},{"input":"1234.56","output":"1 234,56 EUR"},{"input":"0.99","output":"0,99 EUR"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]-[Apartment]\nLT-[Postal Code] [City]\nLITHUANIA","postal_code_pattern":"LT-NNNNN (LT prefix, hyphen, 5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Gedimino pr. 53-1\nLT-01109 Vilnius\nLITHUANIA"],"postal_code_examples":["LT-01109"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210x297mm) |","Lithuania uses metric system exclusively (EU member since 2004)","Imperial units not used except in specialized contexts (aviation, some technical fields)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+370 \\d{1,3} ?\\d{3} ?\\d{4,5}$","number":"^-?\\d{1,3}( \\d{3})*,\\d+$","currency":"^\\d{1,3}( \\d{3})*,\\d{2} EUR$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","date":"^\\d{4}-\\d{2}-\\d{2}$","phone (national)":"^8-?\\d{3} ?\\d{5}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/lt-LT.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'sw-KE'})
SET f.display_name = 'sw-KE Formatting',
    f.content = 'Formatting rules for sw-KE',
    f.llm_context = 'sw-KE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: Ksh before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januari","Februari","Machi","Aprili","Mei","Juni","Julai","Agosti","Septemba","Oktoba","Novemba","Desemba"],"month_abbrev":[],"day_names":["Jumatatu","Jumanne","Jumatano","Alhamisi","Ijumaa","Jumamosi","Jumapili"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"KES","symbol":"Ksh","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Nambari] [Jina la Barabara]\n[Mtaa/Jirani]\n[Mji/Eneo]\n[Msimbo wa Posta]-[Jina la Posta]\nKENYA","postal_code_pattern":"NNNNN (tarakimu 5)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Westlands Business Park\n14 Waiyaki Way\nWestlands\nNairobi\n00100-GPO\nKENYA","S.L.P. 12345-00100\nNairobi\nKENYA","Nyumba 25\nMtaa wa Lavington\nNairobi\n00100-Nairobi\nKENYA","45 Moi Avenue\nMombasa\n80100-GPO\nKENYA"],"postal_code_examples":["00100"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Selsiasi","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilomita","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogramu","symbol":"kg","notes":null},{"category":"Volume","unit":"Lita","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (kiwango cha kimataifa) |","Kenya inatumia mfumo wa metriki kama kiwango rasmi cha vipimo","Vipimo vya kifalme vya Uingereza vinaweza kuonekana mara kwa mara katika muktadha usio rasmi kutokana na historia ya ukoloni","Ubaguzi: anga inatumia futi na maili za baharini duniani kote","Eneo la ardhi: mara nyingi hupimwa kwa ekari (urithi wa ukoloni) pamoja na hekta","Mazao ya kilimo: kawaida huuzwa kwa kilogramu; masoko yasiyo rasmi yanaweza kutumia vipimo vya kienyeji","Urefu: wakati mwingine huonyeshwa kwa futi/inchi bila rasmi, lakini mita/sentimita rasmi","Mafuta: huuzwa kwa lita; ufanisi wa mafuta huonyeshwa kwa km/L"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["25°C","18°C","32°C","37°C"]}',
    f.validation_patterns = '{"time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","phone (national)":"^0[2-7][0-9]{2} [0-9]{3} [0-9]{3}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","phone (international)":"^\\+254 [2-7][0-9]{2} [0-9]{3} [0-9]{3}$","currency":"^Ksh [0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/sw-KE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'hy-AM'})
SET f.display_name = 'hy-AM Formatting',
    f.content = 'Formatting rules for hy-AM',
    f.llm_context = 'hy-AM: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: dram after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM, YYYY t.","full_pattern":null,"date_separator":".","month_names":["hunvar","petarvar","mart","april","mayis","hunis","hulis","ogostos","september","hoktember","noyember","dektember"],"month_abbrev":[],"day_names":["yerkushabti","yerekushabti","chorekhshabti","hingshabti","urbat","shabat","kiraki"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"AMD","symbol":"dram","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Building], [Apartment]\n[City], [Postal Code]\n[Region/Marz]\nARMENIA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Abovyan poxoc 10, bn. 5\nYerevan, 0001\nYerevan marz\nHAYASTAN"],"postal_code_examples":["0001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"l","notes":null}],"paper_size":"A4","notes":["| Standard European |","Armenia uses metric system exclusively for all measurements","Temperature always in Celsius","Road distances in kilometers, short distances in meters"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","time":"^([01]\\d|2[0-3]):[0-5]\\d$","phone (national)":"^\\(0\\d{2}\\) \\d{2}-\\d{2}-\\d{2}$","currency":"^\\d{1,3}( \\d{3})* dram$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","phone (international)":"^\\+374 \\d{2} \\d{3} \\d{3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/hy-AM.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'pt-MZ'})
SET f.display_name = 'pt-MZ Formatting',
    f.content = 'Formatting rules for pt-MZ',
    f.llm_context = 'pt-MZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour (primary) Currency: MT or MTn after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D \'de\' MMMM \'de\' YYYY","full_pattern":null,"date_separator":"/","month_names":["Janeiro","Fevereiro","Março","Abril","Maio","Junho","Julho","Agosto","Setembro","Outubro","Novembro","Dezembro"],"month_abbrev":[],"day_names":["segunda-feira","terça-feira","quarta-feira","quinta-feira","sexta-feira","sábado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour (primary)","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"(not used in formal contexts)","pm_indicator":"(not used in formal contexts)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"MZN (ISO 4217)","symbol":"MT or MTn","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 MT"},{"input":"1234.56","output":"1 234,56 MT"},{"input":"0.99","output":"0,99 MT"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name], [Number]\n[Neighborhood/Bairro]\n[City], [Province]\n[Postal Code] (if applicable)\nMOÇAMBIQUE","postal_code_pattern":"NNNN (4 digits, when used)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Avenida Julius Nyerere, 876\nBairro da Sommerschield\nMaputo, Maputo Cidade\nMOÇAMBIQUE"],"postal_code_examples":["1100"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Mozambique uses the metric system exclusively for official purposes","Some informal markets may use local units, but official documentation always uses metric"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2}) MT$","phone (national)":"^\\d{2} \\d{3} \\d{4}$","phone (international)":"^\\+258 \\d{2} \\d{3} \\d{4}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/pt-MZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'nl-BE'})
SET f.display_name = 'nl-BE Formatting',
    f.content = 'Formatting rules for nl-BE',
    f.llm_context = 'nl-BE: Numbers use \'.\' decimal, \',\' thousands. Dates: D/MM/YYYY (gregorian) Time: 24-hour Currency: € before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D/MM/YYYY","short_pattern":"D/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["januari","februari","maart","april","mei","juni","juli","augustus","september","oktober","november","december"],"month_abbrev":[],"day_names":["maandag","dinsdag","woensdag","donderdag","vrijdag","zaterdag","zondag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15/01/2026"},{"input":"2026-12-31","output":"31/12/2026"},{"input":"2026-07-21","output":"21/07/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"15-01-2026"},{"input":"2026-01-15","output":"15.01.2026"},{"input":"2026-01-15","output":"Januari 15, 2026"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":null,"pm_indicator":null,"prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"€","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"€ 10,50"},{"input":"1234.56","output":"€ 1.234,56"},{"input":"0.99","output":"€ 0,99"},{"input":"1000000","output":"€ 1.000.000,00"},{"input":"8.00","output":"€ 8,00"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [House Number][Bus Number]\n[Postal Code] [City]\nBELGIUM","postal_code_pattern":"NNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Grote Markt 1\n1000 Brussel\nBELGIUM","Meir 48 bus 3\n2000 Antwerpen\nBELGIUM","Korenmarkt 15\n9000 Gent\nBELGIUM","Markt 7\n8000 Brugge\nBELGIUM"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm (ISO 216 standard) |","Metric system is universal and legally required in Belgium","Imperial units (pounds, miles, Fahrenheit) never used except rare international contexts","Aviation uses feet and nautical miles (international ICAO standard)","Fuel economy: liters per 100 km (L/100km), NOT miles per gallon","Body weight: kilograms (NOT stones like UK)","Height: meters and centimeters (1,75 m), NOT feet/inches"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50","21","21","6","6","100","100","50"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","22°C","-5°C","-5°C"]}',
    f.validation_patterns = '{"date":"^\\d{1,2}/\\d{2}/\\d{4}$","phone (international)":"^\\+32\\s?\\d{1,3}\\s?\\d{2}\\s?\\d{2}\\s?\\d{2}$","rijksregisternummer":"^\\d{2}\\.\\d{2}\\.\\d{2}-\\d{3}\\.\\d{2}$","percentage":"^\\d+(\\,\\d+)?\\s%$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","currency":"^€\\s\\d{1,3}(\\.\\d{3})*(,\\d{2})?$","phone (mobile)":"^04\\d{2}\\s?\\d{2}\\s?\\d{2}\\s?\\d{2}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (landline)":"^0[1-9]\\d?\\s?\\d{2,3}\\s?\\d{2}\\s?\\d{2}$","ondernemingsnummer":"^\\d{4}\\.\\d{3}\\.\\d{3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/nl-BE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'te-IN'})
SET f.display_name = 'te-IN Formatting',
    f.content = 'Formatting rules for te-IN',
    f.llm_context = 'te-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD-MM-YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD-MM-YYYY","short_pattern":"DD-MM-YY","long_pattern":"D MMMM, YYYY","full_pattern":null,"date_separator":"-","month_names":["జనవరి","ఫిబ్రవరి","మార్చి","ఏప్రిల్","మే","జూన్","జులై","ఆగస్టు","సెప్టెంబర్","అక్టోబర్","నవంబర్","డిసెంబర్"],"month_abbrev":[],"day_names":["సోమవారం","మంగళవారం","బుధవారం","గురువారం","శుక్రవారం","శనివారం","ఆదివారం"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["శ్రీ రామకృష్ణ\n123, గాంధీ నగర్\nమియాపూర్\nహైదరాబాద్ - 500049\nతెలంగాణ\nINDIA"],"postal_code_examples":["500001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","India uses metric system exclusively","Traditional measurements still used in some contexts: గజం (gajam) for cloth length, ఎకరం (acre) for land area","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM)$","date":"^\\d{2}-\\d{2}-\\d{4}$","phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","phone (national)":"^\\d{5}\\s\\d{5}$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/te-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ml-IN'})
SET f.display_name = 'ml-IN Formatting',
    f.content = 'Formatting rules for ml-IN',
    f.llm_context = 'ml-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["ജനുവരി","ഫെബ്രുവരി","മാര്‍ച്ച്","ഏപ്രില്‍","മേയ്","ജൂണ്‍","ജൂലൈ","ആഗസ്റ്റ്","സെപ്റ്റംബര്‍","ഒക്‌ടോബര്‍","നവംബര്‍","ഡിസംബര്‍"],"month_abbrev":[],"day_names":["തിങ്കളാഴ്ച","ചൊവ്വാഴ്ച","ബുധനാഴ്ച","വ്യാഴാഴ്ച","വെള്ളിയാഴ്ച","ശനിയാഴ്ച","ഞായറാഴ്ച"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"23:59:59","output":"11:59:59 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"₹10.50"},{"input":"1234.56","output":"₹1,234.56"},{"input":"0.99","output":"₹0.99"},{"input":"100000","output":"₹1,00,000"},{"input":"10000000","output":"₹1,00,00,000"},{"input":"750000.50","output":"₹7,50,000.50"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ശ്രീ രാജേഷ് കുമാര്‍\n123, എം.ജി. റോഡ്\nകോവളം\nതിരുവനന്തപുരം - 695527\nകേരളം\nINDIA"],"postal_code_examples":["695001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","India uses metric system exclusively","Exceptions: Some traditional measurements still used in Kerala (e.g., acre, cent for land)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^\\d{5}\\s\\d{5}$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ml-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'vi-VN'})
SET f.display_name = 'vi-VN Formatting',
    f.content = 'Formatting rules for vi-VN',
    f.llm_context = 'vi-VN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: ₫ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"Ngày DD tháng MM năm YYYY","full_pattern":null,"date_separator":"/","month_names":["Tháng Một","Tháng Hai","Tháng Ba","Tháng Tư","Tháng Năm","Tháng Sáu","Tháng Bảy","Tháng Tám","Tháng Chín","Tháng Mười","Tháng Mười Một","Tháng Mười Hai"],"month_abbrev":[],"day_names":["Thứ Hai","Thứ Ba","Thứ Tư","Thứ Năm","Thứ Sáu","Thứ Bảy","Chủ Nhật"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"VND (Vietnamese Dong)","symbol":"₫","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10000","output":"10.000 ₫"},{"input":"1234567","output":"1.234.567 ₫"},{"input":"50000","output":"50.000 ₫"},{"input":"1000000","output":"1.000.000 ₫"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name], [Ward/Commune]\n[District], [City/Province]\nVIETNAM","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123 Đường Nguyễn Huệ, Phường Bến Nghé\nQuận 1, Thành phố Hồ Chí Minh\n700000\nVIETNAM"],"postal_code_examples":["700000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| International standard |","Vietnam uses metric system exclusively","No imperial conversions needed in domestic context","Exception: Aviation and some technical fields use international standards"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\d{1,3}(\\.\\d{3})*[\\s]₫$","phone (international)":"^\\+84[\\s]\\d{1,3}[\\s]\\d{3,4}[\\s]\\d{3,4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (national)":"^0\\d{1,3}[\\s]\\d{3,4}[\\s]\\d{3,4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/vi-VN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-BE'})
SET f.display_name = 'fr-BE Formatting',
    f.content = 'Formatting rules for fr-BE',
    f.llm_context = 'fr-BE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (e.g., 15/01/2026) (gregorian) Time: 24-hour (standard in Belgium) Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","SPF Economie (Belgian federal statistics"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (e.g., 15/01/2026)","short_pattern":"DD/MM/YY (e.g., 15/01/26)","long_pattern":"D MMMM YYYY (e.g., 15 janvier 2026)","full_pattern":"EEEE D MMMM YYYY (e.g., mercredi 15 janvier 2026)","date_separator":"/","month_names":["janvier","février","mars","avril","mai","juin","juillet","août","septembre","octobre","novembre","décembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15/01/2026"},{"input":"2026-12-31","output":"31/12/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"2026-01-15"}]}',
    f.time = '{"system":"24-hour (standard in Belgium)","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss (e.g., 14:30:45)","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (Euro)","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":"centime (1 EUR = 100 centimes)","correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1 234,56 €"},{"input":"0.99","output":"0,99 €"},{"input":"15000.00","output":"15 000,00 €"},{"input":"99.00","output":"99,00 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Nom du destinataire\nNom de rue Numero (bte X)\nCode postal VILLE\nBELGIQUE","postal_code_pattern":"NNNN (4 digits, no prefix)","postal_code_position":"before_city","city_format":"UPPERCASE","street_types":null,"po_box_format":null,"example_addresses":["M. Jean DUPONT\nAvenue Louise 123\n1050 IXELLES\nBELGIQUE","Mme Marie LAMBERT\nRue de la Station 45\n4000 LIEGE\nBELGIQUE","Commission europeenne\nRue de la Loi 200\n1049 BRUXELLES\nBELGIQUE","M. Pierre MARTIN\nBoulevard du Souverain 280 bte 15\n1160 AUDERGHEM\nBELGIQUE"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogrammes","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null},{"category":"Fuel","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Belgium uses metric system exclusively (EU standard)","Imperial units never used in official contexts","Exception: aviation uses feet for altitude (international standard)","Exception: nautical uses knots and nautical miles"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","22°C","-5°C","0°C"]}',
    f.validation_patterns = '{"passeport":"^E[A-Z0-9]{7}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? €$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","phone (mobile)":"^0[4][0-9]{2}( \\d{2}){3}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (landline bxl)":"^02 \\d{3}( \\d{2}){2}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (international)":"^\\+32 [1-9]\\d{1,2}( \\d{2}){3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-BE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ru-IL'})
SET f.display_name = 'ru-IL Formatting',
    f.content = 'Formatting rules for ru-IL',
    f.llm_context = 'ru-IL: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: ₪ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY г.","full_pattern":null,"date_separator":".","month_names":["января","февраля","марта","апреля","мая","июня","июля","августа","сентября","октября","ноября","декабря"],"month_abbrev":[],"day_names":["понедельник","вторник","среда","четверг","пятница","суббота","воскресенье"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"},{"input":"2025-01-15","output":"15/01/2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"ILS (ISO 4217)","symbol":"₪","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10.50 ₪"},{"input":"1234.56","output":"1,234.56 ₪"},{"input":"0.99","output":"0.99 ₪"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Улица] [Номер дома]\n[Город] [Почтовый индекс]\n[Страна]","postal_code_pattern":"NNNNNNN (7 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["улица Ротшильда 1\nТель-Авив-Яффо 6688101\nИзраиль"],"postal_code_examples":["6688101"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["| Стандарт 210×297мм |","Израиль использует исключительно метрическую систему","Исключение: некоторые импортные товары могут показывать двойные измерения (метрическая система - основная)","Температура всегда в градусах Цельсия для погоды, медицины, кулинарии"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50","12.5","12.5","100","100","22","36.6"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C","36.6°C"]}',
    f.validation_patterns = '{"phone (national)":"^0\\d{1,2}-\\d{3}-\\d{4}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","time":"^([01]\\d|2[0-3]):([0-5]\\d)(:[0-5]\\d)?$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","currency":"^\\d{1,3}(,\\d{3})*\\.\\d{2}\\s?₪$","phone (international)":"^\\+972\\s?\\d{1,2}\\s?\\d{3}\\s?\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ru-IL.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'rw-RW'})
SET f.display_name = 'rw-RW Formatting',
    f.content = 'Formatting rules for rw-RW',
    f.llm_context = 'rw-RW: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: FRw after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Mutarama","Gashyantare","Werurwe","Mata","Gicurasi","Kamena","Nyakanga","Kanama","Nzeli","Ukwakira","Ugushyingo","Ukuboza"],"month_abbrev":[],"day_names":["Kuwa mbere","Kuwa kabiri","Kuwa gatatu","Kuwa kane","Kuwa gatanu","Kuwa gatandatu","Ku cyumweru"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"n/a","pm_indicator":"n/a","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"RWF","symbol":"FRw","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building Name/Number]\n[Street Name]\n[District]\n[City/Town]\nRWANDA","postal_code_pattern":"None (Rwanda does not use postal codes for delivery)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Kigali Business Center\nKG 7 Ave\nNyarugenge\nKigali\nRWANDA","P.O. Box 1234\nKigali\nRWANDA","House 15\nKN 5 Road\nKimihurura\nGasabo\nKigali\nRWANDA","45 Avenue de la Revolution\nMusanze\nNorthern Province\nRWANDA"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","Rwanda uses the metric system as the official measurement standard","Colonial influences (Belgian/German) established metric usage","Exception: aviation uses feet and nautical miles worldwide","Land area: measured in hectares and square metres","Agricultural produce: commonly sold by kilogram","Height: expressed in metres/centimetres officially","Fuel: sold in litres; fuel economy expressed in km/L","Traditional measurements occasionally used in rural markets"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","0,5","50"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","18°C","28°C","37°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+250 [27][0-9]{2} [0-9]{3} [0-9]{3}$","number":"^-?[0-9]{1,3}(\\.[0-9]{3})*(,[0-9]+)?$","currency":"^[0-9]{1,3}(\\.[0-9]{3})* FRw$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","time":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","phone (national)":"^0[27][0-9]{2} [0-9]{3} [0-9]{3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/rw-RW.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-KY'})
SET f.display_name = 'en-KY Formatting',
    f.content = 'Formatting rules for en-KY',
    f.llm_context = 'en-KY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"KYD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Street Number and Name\nDistrict (optional)\nGeorge Town, Grand Cayman (or other island)\nCAYMAN ISLANDS","postal_code_pattern":"KY1-NNNN (e.g., KY1-1001) - optional, primarily used for international mail","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"PO Box","example_addresses":["123 Harbour Drive\nGeorge Town\nGrand Cayman\nCAYMAN ISLANDS","PO Box 1234\nGrand Cayman KY1-1102\nCAYMAN ISLANDS","45 West Bay Road\nSeven Mile Beach\nGrand Cayman\nCAYMAN ISLANDS","Lot 52 Dennis Point\nCayman Brac KY2-2101\nCAYMAN ISLANDS"],"postal_code_examples":["KY1-1001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Fahrenheit","symbol":"F","notes":null},{"category":"Distance","unit":"Miles","symbol":"mi","notes":null},{"category":"Weight","unit":"Pounds","symbol":"lb","notes":null},{"category":"Volume","unit":"Gallons","symbol":"gal","notes":null}],"paper_size":"Letter","notes":["| US Letter (8.5\" x 11\") standard |","Cayman Islands uses a mix of imperial and metric influenced by both British heritage and US proximity","Temperature: Fahrenheit common in weather reports and daily conversation","Fuel: Sold in gallons (US) with price per gallon","Speed limits: Posted in miles per hour (mph)","Grocery: Mix of pounds and kilograms","Construction: Imperial measurements (feet, inches) predominant","Exception: Diving uses metres and bars (metric) following international diving standards"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","phone (international)":"^\\+1 345 [0-9]{3} [0-9]{4}$","phone (national)":"^\\(345\\) [0-9]{3}-[0-9]{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-KY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ky-KG'})
SET f.display_name = 'ky-KG Formatting',
    f.content = 'Formatting rules for ky-KG',
    f.llm_context = 'ky-KG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: сом after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"YYYY-ж., D-MMMM","full_pattern":null,"date_separator":".","month_names":["январь","февраль","март","апрель","май","июнь","июль","август","сентябрь","октябрь","ноябрь","декабрь"],"month_abbrev":[],"day_names":["дүйшөмбү","шейшемби","шаршемби","бейшемби","жума","ишемби","жекшемби"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"KGS","symbol":"сом","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 сом"},{"input":"1234.56","output":"1 234,56 сом"},{"input":"0.99","output":"0,99 сом"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name], [Building Number]\n[Postal Code] [City]\n[Region/Oblast]\nКЫРГЫЗСТАН","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ул. Киевская, д. 96, кв. 15\n720001 Бишкек\nЧуйская область\nКЫРГЫЗСТАН"],"postal_code_examples":["720001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["| Standard international |","Kyrgyzstan uses metric system exclusively for all measurements","Temperature always in Celsius","Road distances in kilometers, short distances in meters"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^([01]\\d|2[0-3]):[0-5]\\d$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","phone (international)":"^\\+996 \\d{3} \\d{2}-\\d{2}-\\d{2}$","phone (national)":"^\\(0\\d{3}\\) \\d{2}-\\d{2}-\\d{2}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? сом$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ky-KG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-IE'})
SET f.display_name = 'en-IE Formatting',
    f.content = 'Formatting rules for en-IE',
    f.llm_context = 'en-IE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour (primary), 12-hour (also common) Currency: EUR before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"},{"input":"2025-03-17","output":"17/03/2025"},{"input":"2025-12-25","output":"25/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-01-15","output":"January 15, 2025"}]}',
    f.time = '{"system":"24-hour (primary), 12-hour (also common)","pattern":"HH:mm (24-hour) or h:mm a (12-hour)","pattern_with_seconds":"HH:mm:ss (24-hour) or h:mm:ss a (12-hour)","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"},{"input":"17:45","output":"17:45"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"EUR","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"EUR10.50"},{"input":"1234.56","output":"EUR1,234.56"},{"input":"0.99","output":"EUR0.99"},{"input":"1000000","output":"EUR1,000,000.00"},{"input":"5.00","output":"EUR5.00"},{"input":"99.95","output":"EUR99.95"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[Town/Village]\n[City/Town]\n[County]\n[Eircode]\nIRELAND","postal_code_pattern":"ANN ANNN (Eircode: letter, digit, digit, space, letter, digit, digit, digit)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["15 O\'Connell Street\nDublin 1\nD01 X2P3\nIRELAND","Ballymore House\nBallymore\nCo. Galway\nH91 AB12\nIRELAND","Apt 4, 25 Grafton Street\nDublin 2\nD02 Y3K4\nIRELAND"],"postal_code_examples":["D01 X2P3"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 mm × 297 mm (standard) |","Ireland uses metric system as official standard (EU member)","Road signs: distances in kilometres since 2005","Speed limits: km/h","Fuel: sold in litres","Legacy imperial: pints for draught beer (legally defined as 568ml), stone/pounds sometimes for body weight","Exception: aviation uses feet and nautical miles worldwide","Land area: often in acres in rural/agricultural contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["20°C","0°C","37°C","100°C"]}',
    f.validation_patterns = '{"time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","phone (national)":"^0[1-9][0-9] [0-9]{3} [0-9]{4}$","currency":"^EUR[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","phone (international)":"^\\+353 [1-9][0-9]? [0-9]{3} [0-9]{4}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] [ap]\\.m\\.$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-IE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'su-ID'})
SET f.display_name = 'su-ID Formatting',
    f.content = 'Formatting rules for su-ID',
    f.llm_context = 'su-ID: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: Rp before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januari","Pebruari","Maret","April","Mei","Juni","Juli","Agustus","September","Oktober","Nopember","Desember"],"month_abbrev":[],"day_names":["Senen","Salasa","Rebo","Kemis","Jumaah","Saptu","Minggu"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH.mm","pattern_with_seconds":"HH.mm.ss","time_separator":".","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14.30"},{"input":"09:00","output":"09.00"},{"input":"23:59:59","output":"23.59.59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"IDR","symbol":"Rp","symbol_position":"before","space_between":true,"decimal_places":20,"subunit":null,"correct_examples":[{"input":"10.50","output":"Rp10,50"},{"input":"1234.56","output":"Rp1.234,56"},{"input":"0.99","output":"Rp0,99"},{"input":"1000","output":"Rp1.000"},{"input":"50000","output":"Rp50.000"},{"input":"1000000","output":"Rp1.000.000"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Address with Number]\n[District/Subdistrict], [City] [Postal Code]\n[Province]\nINDONESIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Asep Suryadi\nJl. Asia Afrika No. 65\nBraga, Bandung 40111\nJawa Barat\nINDONESIA"],"postal_code_examples":["40111"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Indonesia uses the metric system exclusively for official and commercial purposes","Imperial units may appear in informal contexts due to international influence, but metric is always primary"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{2}\\.\\d{2}(:\\d{2})?$","currency":"^Rp\\d{1,3}(\\.\\d{3})*(,\\d{2})?$","phone (international)":"^\\+62\\s?\\d{2,3}\\s?\\d{4}\\s?\\d{4,5}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^0\\d{2,3}\\s?\\d{4}\\s?\\d{4,5}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/su-ID.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-UG'})
SET f.display_name = 'en-UG Formatting',
    f.content = 'Formatting rules for en-UG',
    f.llm_context = 'en-UG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: USh before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"UGX","symbol":"USh","symbol_position":"before","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building/House Name/Number]\n[Plot Number] [Street Name]\n[Suburb/Trading Centre/Village]\n[City/Town]\n[District]\nUGANDA","postal_code_pattern":"NONE","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Rwenzori House\nPlot 1, Lumumba Avenue\nNakasero\nKampala\nCentral Division\nUGANDA","Uganda Revenue Authority\nP.O. Box 7279\nKampala\nUGANDA","House 15, Mbuya Hill Road\nMbuya\nKampala\nNakawa Division\nUGANDA","Plot 234, Entebbe Road\nKajjansi Trading Centre\nWakiso District\nUGANDA","Plot 45, Main Street\nJinja Town\nJinja District\nUGANDA","Near Bwindi Community Hospital\nBuhoma Trading Centre\nKanungu District\nUGANDA"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","Uganda uses the metric system as the official measurement standard","Exception: land often measured in acres (colonial legacy) alongside hectares","Exception: aviation uses feet and nautical miles internationally","Fuel sold in litres at petrol stations; fuel economy in km/L","Agricultural produce: sold by kilogram at formal markets; informal markets may use debe (18L tin), kasuku (3L), cup measures","Height: officially in metres/centimetres; informally feet/inches still used","British spelling: kilometre (not kilometer), litre (not liter)","Room temperature in Kampala: typically 22-28C year-round"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","18","6","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","phone (international)":"^\\+256 [3-7][0-9]{2} [0-9]{3} [0-9]{3}$","currency":"^USh [0-9]{1,3}(,[0-9]{3})*$","nin (national id)":"^CM[A-Z0-9]{13}$","tin (tax id)":"^\\d{10}$","time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","passport":"^A\\d{8}$","phone (national)":"^0[3-7][0-9]{2} [0-9]{3} [0-9]{3}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-UG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'sw-TZ'})
SET f.display_name = 'sw-TZ Formatting',
    f.content = 'Formatting rules for sw-TZ',
    f.llm_context = 'sw-TZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: TSh before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januari","Februari","Machi","Aprili","Mei","Juni","Julai","Agosti","Septemba","Oktoba","Novemba","Desemba"],"month_abbrev":[],"day_names":["Jumatatu","Jumanne","Jumatano","Alhamisi","Ijumaa","Jumamosi","Jumapili"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"asubuhi","pm_indicator":"jioni/usiku","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"TZS","symbol":"TSh","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Nambari ya Kiwanja/Jengo] [Jina la Barabara]\n[Eneo/Mtaa]\n[Jiji/Mji]\n[Msimbo wa Posta]\nTANZANIA","postal_code_pattern":"NNNNN (tarakimu 5)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Kiwanja 45 Barabara ya Ali Hassan Mwinyi\nMasaki\nDar es Salaam\n14101\nTANZANIA","S.L.P. 12345\nDar es Salaam\nTANZANIA","Nyumba 12\nMikocheni B\nDar es Salaam\n14101\nTANZANIA","Barabara ya Sokoine\nArusha\n23101\nTANZANIA","Barabara ya Kenyatta\nStone Town\nZanzibar\n71101\nTANZANIA","Barabara ya Nyerere\nDodoma\n41101\nTANZANIA"],"postal_code_examples":["14101"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Joto","unit":"Selsiasi","symbol":"°C","notes":null},{"category":"Umbali","unit":"Kilomita","symbol":"km","notes":null},{"category":"Uzito","unit":"Kilogramu","symbol":"kg","notes":null},{"category":"Ujazo","unit":"Lita","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (kiwango cha kimataifa) |","Tanzania inatumia mfumo wa metriki kama kiwango rasmi cha vipimo","Vipimo vya imperial vya Uingereza vinaweza kuonekana wakati mwingine kutokana na historia ya ukoloni","Isipokuwa: anga hutumia futi na maili ya bahari ulimwenguni kote","Eneo la ardhi: mara nyingi hupimwa kwa ekari (urithi wa ukoloni) pamoja na hekta","Mazao ya kilimo: kawaida huuzwa kwa kilogramu; masoko yasiyo rasmi yanaweza kutumia vipimo vya ndani (debe, gunia, pishi)","Urefu: wakati mwingine huonyeshwa kwa futi/inchi bila kukusudia, lakini mita/sentimita rasmi","Mafuta: huuzwa kwa lita; ufanisi wa mafuta huonyeshwa kwa km/L"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["30°C","25°C","33°C","37°C","20°C"]}',
    f.validation_patterns = '{"simu (kitaifa)":"^0[2-7][0-9]{2} [0-9]{3} [0-9]{3}$","sarafu":"^TSh [0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","saa":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","simu (kimataifa)":"^\\+255 [2-7][0-9]{2} [0-9]{3} [0-9]{3}$","tarehe":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","nambari":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/sw-TZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'hu-HU'})
SET f.display_name = 'hu-HU Formatting',
    f.content = 'Formatting rules for hu-HU',
    f.llm_context = 'hu-HU: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY. MM. DD. (year, month, day with dots and trailing dot) (gregorian) Time: 24-hour Currency: Ft after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY. MM. DD. (year, month, day with dots and trailing dot)","short_pattern":"YYYY. MM. DD.","long_pattern":"YYYY. MMMM D. (full month name)","full_pattern":null,"date_separator":".","month_names":["január","február","március","április","május","június","július","augusztus","szeptember","október","november","december"],"month_abbrev":[],"day_names":["hétfő","kedd","szerda","csütörtök","péntek","szombat","vasárnap"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025. 01. 15."},{"input":"2025-12-31","output":"2025. 12. 31."}],"incorrect_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour with leading zero)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"HUF (Hungarian Forint)","symbol":"Ft","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10","output":"10 Ft"},{"input":"1234","output":"1 234 Ft"},{"input":"1000","output":"1 000 Ft"},{"input":"1000","output":"1 000 Ft"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street name] [Number]\n[Postal code] [City]\nHUNGARY","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Andrássy út 60\n1062 Budapest\nHUNGARY"],"postal_code_examples":["1062"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210×297 mm) |","Hungary uses the metric system exclusively","Imperial units (miles, pounds, Fahrenheit) are not commonly used and should be avoided or explained"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","phone (national)":"^\\(\\d{1,2}\\)\\s\\d{3}-\\d{3,4}$","date":"^\\d{4}\\.\\s\\d{2}\\.\\s\\d{2}\\.$","phone (international)":"^\\+36\\s\\d{1,2}\\s\\d{3}\\s\\d{3,4}$","currency":"^\\d{1,3}(\\s\\d{3})*\\sFt$","time":"^\\d{2}:\\d{2}(:\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/hu-HU.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-EC'})
SET f.display_name = 'es-EC Formatting',
    f.content = 'Formatting rules for es-EC',
    f.llm_context = 'es-EC: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"USD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street/Avenue] [Number] y [Cross Street]\n[Sector/Neighborhood]\n[City]\n[Province]\n[Postal Code]\nECUADOR","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Av. Amazonas N24-45 y Colon\nLa Mariscal\nQuito\nPichincha\n170517\nECUADOR"],"postal_code_examples":["170517"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Liters","unit":"L","symbol":"Metric system standard","notes":null},{"category":"Gallons","unit":"gal","symbol":"US gallon (3.785 L) - Ecuador exception","notes":null},{"category":"Hectares","unit":"ha","symbol":"10,000 m2 - used for agricultural land","notes":null}],"paper_size":"Letter","notes":["Ecuador exception |","| ISO A4 standard (210 mm x 297 mm) |","used for agricultural land |","Metric system is official throughout Ecuador","**Fuel sold by gallon (galon)**: Unlike most metric countries, Ecuador sells gasoline in US gallons","**Paper size uses A4**: Different from US Letter used in some neighboring countries","**Altitude in meters**: Critical for Ecuador given Andean geography (Quito at 2,850m)","**Land measured in hectares**: Agricultural and real estate commonly use hectares (1 ha = 10,000 m2)","**Speed limits in km/h**: Urban 50 km/h, Highway 90-120 km/h"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{2}:\\d{2}(:\\d{2})?$","ruc (natural)":"^\\d{10}001$","phone (international)":"^\\+593 \\d{1,2} \\d{3} \\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","ruc (company)":"^[0-2][0-9]9\\d{7}001$","currency":"^\\$\\d{1,3}(\\.\\d{3})*,\\d{2}$","cedula":"^\\d{10}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-EC.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'mg-MG'})
SET f.display_name = 'mg-MG Formatting',
    f.content = 'Formatting rules for mg-MG',
    f.llm_context = 'mg-MG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: Ar after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Janoary","Febroary","Martsa","Aprily","Mey","Jona","Jolay","Aogositra","Septambra","Oktobra","Novambra","Desambra"],"month_abbrev":[],"day_names":["Alatsinainy","Talata","Alarobia","Alakamisy","Zoma","Asabotsy","Alahady"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"MGA","symbol":"Ar","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[Postal Code] [City]\nMADAGASCAR","postal_code_pattern":"NNN (3 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Lot IVG 234 Analakely\n101 Antananarivo\nMADAGASCAR"],"postal_code_examples":["101"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","Madagascar uses the metric system exclusively for all official measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\d{1,3}( \\d{3})* Ar$","phone (national)":"^0[23][234]( \\d{2}){2}( \\d{3})( \\d{2})$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (international)":"^\\+261 [23][234]( \\d{2}){2}( \\d{3})( \\d{2})$","number":"^-?\\d{1,3}( \\d{3})*,\\d+$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/mg-MG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ca-AD'})
SET f.display_name = 'ca-AD Formatting',
    f.content = 'Formatting rules for ca-AD',
    f.llm_context = 'ca-AD: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["gener","febrer","marc","abril","maig","juny","juliol","agost","setembre","octubre","novembre","desembre"],"month_abbrev":[],"day_names":["dilluns","dimarts","dimecres","dijous","divendres","dissabte","diumenge"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1.234,56 €"},{"input":"0.99","output":"0,99 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name], [Number]\n[Postal Code] [City]\nANDORRA","postal_code_pattern":"ADNNN (AD followed by 3 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Carrer Major, 12\nAD500 Andorra la Vella\nANDORRA"],"postal_code_examples":["AD500"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","Andorra uses the metric system exclusively for all measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","date":"^\\d{1,2}/\\d{2}/\\d{4}$","phone (international)":"^\\+376 \\d{3} \\d{3}$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} €$","phone (national)":"^\\d{3} \\d{3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ca-AD.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ko-KR'})
SET f.display_name = 'ko-KR Formatting',
    f.content = 'Formatting rules for ko-KR',
    f.llm_context = 'ko-KR: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY. M. D. (gregorian) Time: 12-hour Currency: ₩ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY. M. D.","short_pattern":"YY. M. D.","long_pattern":"YYYY년 M월 D일","full_pattern":null,"date_separator":".","month_names":["1월","2월","3월","4월","5월","6월","7월","8월","9월","10월","11월","12월"],"month_abbrev":[],"day_names":["월요일","화요일","수요일","목요일","금요일","토요일","일요일"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025. 1. 15."},{"input":"2025-12-31","output":"2025. 12. 31."}],"incorrect_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"01/15/2025"}]}',
    f.time = '{"system":"12-hour","pattern":"a h:mm","pattern_with_seconds":"a h:mm:ss","time_separator":":","am_indicator":"오전","pm_indicator":"오후","prayer_times":null,"correct_examples":[{"input":"14:30","output":"오후 2:30"},{"input":"09:00","output":"오전 9:00"},{"input":"23:59:59","output":"오후 11:59:59"},{"input":"14:30","output":"14:30"}],"incorrect_examples":[]}',
    f.currency = '{"code":"KRW","symbol":"₩","symbol_position":"before","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10.50","output":"₩11"},{"input":"1234.56","output":"₩1,235"},{"input":"0.99","output":"₩1"},{"input":"1000","output":"₩1,000"},{"input":"50000","output":"₩50,000"},{"input":"1000000","output":"₩1,000,000"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Postal Code]\n[Province/City] [District] [Street Address]\n[Building Name, Floor/Room Number]\nSOUTH KOREA","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["06234\n서울특별시 강남구 테헤란로 123\n스타타워 15층 1501호\nSOUTH KOREA"],"postal_code_examples":["06234"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard paper size (210×297mm) |","Metric system is standard in all contexts","Traditional Korean units (자, 척, 근) are rarely used in modern contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^₩\\d{1,3}(,\\d{3})*$","phone (national)":"^0\\d{1,2}-\\d{3,4}-\\d{4}$","phone (international)":"^\\+82\\s\\d{1,2}\\s\\d{3,4}\\s\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","date":"^\\d{4}\\.\\s?\\d{1,2}\\.\\s?\\d{1,2}\\.$","time":"^(오전\\|오후)\\s\\d{1,2}:\\d{2}(:\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ko-KR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ln-CD'})
SET f.display_name = 'ln-CD Formatting',
    f.content = 'Formatting rules for ln-CD',
    f.llm_context = 'ln-CD: Numbers use \'.\' decimal, \',\' thousands. Dates: D/M/YYYY (gregorian) Time: 24-hour Currency: FC after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D/M/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["sanza ya yambo","sanza ya mibale","sanza ya misato","sanza ya minei","sanza ya mitano","sanza ya motoba","sanza ya nsambo","sanza ya mwambe","sanza ya libwa","sanza ya zomi","sanza ya zomi na moko","sanza ya zomi na mibale"],"month_abbrev":[],"day_names":["mokolo ya liboso","mokolo ya mibale","mokolo ya misato","mokolo ya minei","mokolo ya mitano","mposo ya liboso","eyenga"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"CDF","symbol":"FC","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name], [Number]\n[District/Commune]\n[City]\nREPUBLIQUE DEMOCRATIQUE DU CONGO","postal_code_pattern":"Not standardized (postal codes not widely used in DRC)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Avenue du 30 Juin, 1523\nCommune de la Gombe\nKinshasa\nREPUBLIQUE DEMOCRATIQUE DU CONGO"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","DRC uses metric system exclusively for all official measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"phone (national)":"^0\\d{3} \\d{3} \\d{3}$","date":"^\\d{1,2}/\\d{1,2}/\\d{4}$","phone (international)":"^\\+243 \\d{3} \\d{3} \\d{3}$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} FC$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ln-CD.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-KE'})
SET f.display_name = 'en-KE Formatting',
    f.content = 'Formatting rules for en-KE',
    f.llm_context = 'en-KE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: Ksh before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"KES","symbol":"Ksh","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Building/House Name or Number\nStreet Name\nEstate/Neighbourhood\nTown/City\nPostal Code-Post Office Name\nCounty Name (optional for formal)\nKENYA","postal_code_pattern":"","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Lonrho House, 14th Floor\nStandard Street\nNairobi CBD\nNairobi\n00100-GPO\nNairobi County\nKENYA","P.O. Box 12345-00100\nNairobi\nKENYA","House 25, Peponi Gardens\nOff Peponi Road\nWestlands\nNairobi\n00800-Westlands\nKENYA","Nyali Centre, Suite 3A\nLinks Road\nNyali\nMombasa\n80100-GPO\nMombasa County\nKENYA","P.O. Box 456-30100\nEldoret\nUasin Gishu County\nKENYA"],"postal_code_examples":["00100"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null},{"category":"Acres","unit":"ac","symbol":"Primary for land transactions (colonial legacy)","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (ISO standard) |","Kenya officially uses the metric system (SI units)","Road distances in kilometres (Nairobi-Mombasa: 485 km via SGR)","Fuel sold in litres; fuel economy typically km/L","Body weight: kilograms officially, but feet/inches common for height informally","**Land exception**: Acres dominant over hectares (1 acre = 0.4047 hectares)","Agricultural produce: per kilogram (e.g., sukuma wiki Ksh 20/kg)","Informal markets: \"debe\" (tin), \"gunia\" (sack) as volume measures","Aviation: feet for altitude, nautical miles (international standard)","Safari distances often given in kilometres (\"Big Five viewing, 3 km from camp\")"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["16","30","10","7.5","12.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","12°C","32°C","28°C","15°C","37°C","-5°C"]}',
    f.validation_patterns = '{"time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9](:[0-5][0-9])?$","time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(AM\\|PM)$","phone (national)":"^0[17][0-9]{2}\\s?[0-9]{3}\\s?[0-9]{3}$","landline (nairobi)":"^020\\s?[0-9]{3}\\s?[0-9]{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","phone (international)":"^\\+254\\s?[17][0-9]{2}\\s?[0-9]{3}\\s?[0-9]{3}$","passport":"^[A-Z][0-9]{8}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","currency":"^Ksh\\s[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-KE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-DZ'})
SET f.display_name = 'ar-DZ Formatting',
    f.content = 'Formatting rules for ar-DZ',
    f.llm_context = 'ar-DZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 24-hour Currency: د.ج after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["جانفي (Janfi)","فيفري (Fifri)","مارس","أفريل (Afril)","ماي","جوان (Jouan)","جويلية (Juiliya)","أوت (Out)","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-07-05","output":"05/07/2025"}],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:45","output":"23:45"}],"incorrect_examples":[]}',
    f.currency = '{"code":"DZD","symbol":"د.ج","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"1500","output":"1.500 د.ج"},{"input":"45000","output":"45.000 د.ج"},{"input":"2.50","output":"2,50 د.ج"},{"input":"150000","output":"150.000 د.ج"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Number] [Street Type] [Street Name]\n[Postal Code] [City]\n[Wilaya]\nALGERIE","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["السيد كريم بن عمر\n23 شارع ديدوش مراد\n16000 الجزائر العاصمة\nولاية الجزائر\nالجزائر","مؤسسة الغرب للتجارة\n45 شارع العربي بن مهيدي\n31000 وهران\nولاية وهران\nالجزائر","Société Générale Algérie\n5 Rue Khelifa Boukhalfa\n16000 Alger\nALGERIE"],"postal_code_examples":["16000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Algeria uses the metric system exclusively (French colonial legacy)","Temperature: Celsius (°C) - Sahara can reach 50°C+","Distance: Kilometers for all road signs","Weight: Kilograms, grams, tonnes","Volume: Liters for fuel and liquids","Land: Hectares (ha) for agriculture"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["19","19","50","50","12,5","12,5"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["35°C","35°C","48°C","48°C","-2°C","-2°C"]}',
    f.validation_patterns = '{"landline":"^0[2-4][1-9]\\s\\d{2}\\s\\d{2}\\s\\d{2}$","phone (mobile)":"^0[567]\\s\\d{2}\\s\\d{2}\\s\\d{2}\\s\\d{2}$","number":"^-?[0-9]{1,3}(\\.[0-9]{3})*(,[0-9]+)?$","currency":"^[0-9]{1,3}(\\.[0-9]{3})*(,[0-9]{2})?\\sد\\.ج$","phone (international)":"^\\+213\\s[567]\\s\\d{2}\\s\\d{2}\\s\\d{2}\\s\\d{2}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-DZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-CA'})
SET f.display_name = 'fr-CA Formatting',
    f.content = 'Formatting rules for fr-CA',
    f.llm_context = 'fr-CA: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY-MM-DD (official Canadian standard per ISO 8601) (gregorian) Time: 24-hour (standard in Quebec) Currency: $ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY-MM-DD (official Canadian standard per ISO 8601)","short_pattern":"YYYY-MM-DD","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"-","month_names":["janvier","février","mars","avril","mai","juin","juillet","août","septembre","octobre","novembre","décembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025-01-15"},{"input":"2025-12-31","output":"2025-12-31"},{"input":"2025-06-24","output":"2025-06-24"},{"input":"2025-07-01","output":"2025-07-01"},{"input":"2025-10-13","output":"2025-10-13"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour (standard in Quebec)","pattern":"HH h mm (with letter h as separator) or HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":"h","am_indicator":"(not typically used - 24-hour system preferred)","pm_indicator":"(not typically used - 24-hour system preferred)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14 h 30"},{"input":"09:00","output":"9 h"},{"input":"23:59:59","output":"23:59:59"},{"input":"12:00","output":"12 h"},{"input":"00:00","output":"0 h"},{"input":"17:45","output":"17 h 45"}],"incorrect_examples":[]}',
    f.currency = '{"code":"CAD","symbol":"$","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 $"},{"input":"1234.56","output":"1 234,56 $"},{"input":"0.99","output":"0,99 $"},{"input":"1000000","output":"1 000 000,00 $"},{"input":"5.00","output":"5,00 $"},{"input":"99.95","output":"99,95 $"},{"input":"10.50","output":"10,50 $ CA"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number], [Street Type] [Street Name] [Unit] (optional)\n[City] (Québec)  [Postal Code]\nCANADA","postal_code_pattern":"ANA NAN (alternating letter-number-letter space number-letter-number)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["1234, boulevard René-Lévesque Ouest\nMontréal (Québec)  H3G 1T4\nCANADA","5678, avenue du Parc, app. 302\nMontréal (Québec)  H2V 4G7\nCANADA","150, rue Wellington\nOttawa (Ontario)  K1A 0A9\nCANADA"],"postal_code_examples":["H3G 1T4"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilomètres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogrammes","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 8,5 po × 11 po (comme aux États-Unis, pas A4) |","Le Canada utilise officiellement le système métrique depuis 1970","Les unités impériales persistent dans le langage courant (pieds, livres)","Signalisation routière: kilomètres et km/h","Poids corporel: les gens utilisent souvent les livres informellement","Taille: pieds et pouces courants (ex.: 5 pi 10 po)","Cuisine: mélange de métrique et impérial (tasses, cuillères à soupe)","Construction: mesures impériales encore courantes (bois, plomberie)","Exception: aviation utilise pieds et milles marins mondialement"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","0,5","100","50","12.5"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","0°C","37°C","100°C","-40°C","-25°C"]}',
    f.validation_patterns = '{"time (h separator)":"^\\d{1,2} h( \\d{2})?$","date":"^\\d{4}-(0[1-9]\\|1[0-2])-(0[1-9]\\|[12][0-9]\\|3[01])$","phone (national)":"^\\(?[0-9]{3}\\)?[ -]?[0-9]{3}-[0-9]{4}$","phone (international)":"^\\+1 [0-9]{3} [0-9]{3}-[0-9]{4}$","number":"^-?[0-9]{1,3}( [0-9]{3})*(,[0-9]+)?$","currency":"^[0-9]{1,3}( [0-9]{3})*,[0-9]{2} \\$$","time (colon)":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-CA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-GB'})
SET f.display_name = 'en-GB Formatting',
    f.content = 'Formatting rules for en-GB',
    f.llm_context = 'en-GB: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: £ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"am","pm_indicator":"pm","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"GBP","symbol":"£","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Flat/Unit Number, Building Name\nHouse Number Street Name\nLocality/Town\nCity\nCounty (optional)\nPostcode\nUNITED KINGDOM","postal_code_pattern":"Outward code (1-2 letters + 1-2 digits) + space + Inward code (digit + 2 letters)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Flat 4\n27 Oxford Street\nLondon\nW1D 2DW\nUNITED KINGDOM"],"postal_code_examples":["SW1A 1AA"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Miles","symbol":"mi","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Road signs and distances use miles; athletic events use metres/kilometres","Body weight often expressed in stone and pounds (e.g., \"11 stone 4 pounds\")","Fuel sold in litres but fuel economy often discussed in miles per gallon (mpg)","Pints (568 ml imperial) used for draught beer and milk; litres for most other liquids"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^£[\\d,]+(\\.\\d{2})?$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(am\\|pm)$","phone (national)":"^0[1-9][0-9]{8,10}$","phone (international)":"^\\+44\\s?[1-9][0-9]{9,10}$","number":"^-?[\\d,]+(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-GB.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-RW'})
SET f.display_name = 'fr-RW Formatting',
    f.content = 'Formatting rules for fr-RW',
    f.llm_context = 'fr-RW: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: FRw (preferred), RF (alternative) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["janvier","fevrier","mars","avril","mai","juin","juillet","aout","septembre","octobre","novembre","decembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"RWF (ISO 4217)","symbol":"FRw (preferred), RF (alternative)","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Name/Organization\nStreet Code + Number + Street Type\nCell, Sector\nDistrict, Province\nRWANDA","postal_code_pattern":"N/A (Rwanda does not use postal codes)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Banque Nationale du Rwanda\nKG 6 Avenue\nKamatamu, Kacyiru\nGasabo, Kigali\nRWANDA","Hotel des Mille Collines\nKN 2 Avenue\nMuhima, Nyarugenge\nNyarugenge, Kigali\nRWANDA","Hopital de Butare\nSecteur Ngoma\nDistrict Huye\nProvince du Sud\nRWANDA"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogrammes","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","Rwanda uses the metric system exclusively for all official measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts","Land area often expressed in hectares (ha) for agricultural/property contexts","Local context: \"imyaka\" (Kinyarwanda) used colloquially for age/years; \"imitwe\" for counting heads (livestock, people in census)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"phone (landline)":"^0252 \\d{3} \\d{3}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (international)":"^\\+250 [7][2389]\\d{1} \\d{3} \\d{3}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","tin (tax id)":"^\\d{9}$","phone (national)":"^0[7][2389]\\d{1} \\d{3} \\d{3}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","currency":"^\\d{1,3}( \\d{3})* FRw$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-RW.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ru-BY'})
SET f.display_name = 'ru-BY Formatting',
    f.content = 'Formatting rules for ru-BY',
    f.llm_context = 'ru-BY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: Br after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY г.","full_pattern":null,"date_separator":".","month_names":["января","февраля","марта","апреля","мая","июня","июля","августа","сентября","октября","ноября","декабря"],"month_abbrev":[],"day_names":["понедельник","вторник","среда","четверг","пятница","суббота","воскресенье"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"BYN (Belarusian Ruble)","symbol":"Br","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"ул./пр. Name, д. Number, кв. Number\nPostal, г. City\nOblast\nБЕЛАРУСЬ","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ул. Независимости, д. 12, кв. 34\n220030, г. Минск\nБЕЛАРУСЬ","ул. Советская, д. 45\n224000, г. Брест\nБрестская область\nБЕЛАРУСЬ","пр. Космонавтов, д. 78, оф. 15\n230000, г. Гродно\nГродненская область\nБЕЛАРУСЬ"],"postal_code_examples":["220030"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["| Standard European |","Belarus uses metric system exclusively for all measurements","Temperature always in Celsius (winter can reach -30°C in northern regions)","Road distances in kilometers, city distances in meters","No imperial units used"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["-20°C"]}',
    f.validation_patterns = '{"date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d$","phone (international)":"^\\+375 \\d{2} \\d{3}-\\d{2}-\\d{2}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? Br$","phone (national)":"^8 \\(0\\d{2,3}\\) \\d{3}-\\d{2}-\\d{2}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ru-BY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-GT'})
SET f.display_name = 'es-GT Formatting',
    f.content = 'Formatting rules for es-GT',
    f.llm_context = 'es-GT: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: Q before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p.m."},{"input":"09:00","output":"9:00 a.m."},{"input":"23:59:59","output":"11:59:59 p.m."},{"input":"14:30","output":"14:30"},{"input":"00:00","output":"12:00 a.m."}],"incorrect_examples":[]}',
    f.currency = '{"code":"GTQ","symbol":"Q","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"Q10.50"},{"input":"1234.56","output":"Q1,234.56"},{"input":"0.99","output":"Q0.99"},{"input":"1500.00","output":"Q1,500.00"},{"input":"999999.99","output":"Q999,999.99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Calle/Avenida Number] [Cross Street]-[Building Number], [Zona]\n[City/Municipality]\n[Postal Code] [Department]\nGUATEMALA","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["12 Calle 4-45, Zona 1\nGuatemala\n01001 Guatemala\nGUATEMALA","Edificio Atlantis, 7 Avenida 5-10, Zona 10\nGuatemala\n01010 Guatemala\nGUATEMALA","Residenciales Vista Hermosa, Casa 15\n14 Calle 3-51, Zona 14\nGuatemala\n01014 Guatemala\nGUATEMALA"],"postal_code_examples":["01001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Fuel","unit":"Gallons","symbol":"gal","notes":null},{"category":"Unit","unit":"Equivalent","symbol":"Usage","notes":null}],"paper_size":"Letter","notes":["| US letter size (8.5\" x 11\") standard |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"dpi":"^\\d{4} \\d{5} \\d{4}$","nit":"^\\d{8}-\\d$","phone (national)":"^\\d{4}-\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","time":"^\\d{1,2}:\\d{2}(:\\d{2})?\\s?(a\\.m\\.|p\\.m\\.)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","currency":"^Q\\d{1,3}(,\\d{3})*\\.\\d{2}$","phone (international)":"^\\+502 \\d{4} \\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-GT.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ms-SG'})
SET f.display_name = 'ms-SG Formatting',
    f.content = 'Formatting rules for ms-SG',
    f.llm_context = 'ms-SG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: $ (Dollar sign, sometimes S$ for disambiguation) before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januari","Februari","Mac","April","Mei","Jun","Julai","Ogos","September","Oktober","November","Disember"],"month_abbrev":[],"day_names":["Isnin","Selasa","Rabu","Khamis","Jumaat","Sabtu","Ahad"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"PG (Pagi)","pm_indicator":"PTG (Petang)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"SGD (ISO 4217: Singapore Dollar)","symbol":"$ (Dollar sign, sometimes S$ for disambiguation)","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Unit Number, Building Name\nBlock/House Number Street Name\n#Floor-Unit (for HDB/condos)\nSingapura Postal Code\nSINGAPURA","postal_code_pattern":"NNNNNN (6 digits, no spaces or letters)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Blk 456 Tampines Street 42\n#12-345\nSingapura 520456\nSINGAPURA"],"postal_code_examples":["520456"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometer","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogram","symbol":"kg","notes":null},{"category":"Volume","unit":"Liter","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Singapore uses metric system exclusively for all official purposes","Road signs display distances in kilometres","Fuel sold in litres, fuel economy in km/L","Body weight typically expressed in kilograms (not stone/pounds)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["32°C"]}',
    f.validation_patterns = '{"number":"^-?[\\d,]+(\\.\\d+)?$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","phone (international)":"^\\+65\\s[689]\\d{3}\\s\\d{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s(PG\\|PTG)$","phone (national)":"^[689]\\d{3}\\s\\d{4}$","currency":"^S?\\$[\\d,]+(\\.\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ms-SG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ht-HT'})
SET f.display_name = 'ht-HT Formatting',
    f.content = 'Formatting rules for ht-HT',
    f.llm_context = 'ht-HT: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour (common in daily use) / 24-hour (formal contexts) Currency: G (or HTG) before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["janvye","fevriye","mas","avril","me","jen","jiyè","out","septanm","oktòb","novanm","desanm"],"month_abbrev":[],"day_names":["lendi","madi","mèkredi","jedi","vandredi","samdi","dimanch"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"12-hour (common in daily use) / 24-hour (formal contexts)","pattern":"h:mm a (12-hour) or HH:mm (24-hour)","pattern_with_seconds":"h:mm:ss a or HH:mm:ss","time_separator":":","am_indicator":"AM (or dimaten)","pm_indicator":"PM (or apremidi/aswè)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"23:59:59","output":"11:59:59 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"HTG","symbol":"G (or HTG)","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"G 10,50"},{"input":"1234.56","output":"G 1 234,56"},{"input":"0.99","output":"G 0,99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[Locality/Neighborhood]\n[City] HT [Postal Code]\nHAITI","postal_code_pattern":"HT NNNN (HT prefix + 4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["23 Ri Grègwa\nPetyonvil\nPòtoprens HT 6140\nHAITI"],"postal_code_examples":["HT 6110"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US Letter (8.5 x 11 in) more common due to US influence |","Haiti officially uses the metric system","US influence means some imperial units are used informally (pounds for weight, sometimes miles)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50","12,5"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+509 \\d{4} \\d{4}$","currency":"^G \\d{1,3}( \\d{3})*,\\d{2}$","time (24h)":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^\\d{4} \\d{4}$","time (12h)":"^(1[0-2]|[1-9]):[0-5]\\d (AM|PM)$","date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?\\d{1,3}( \\d{3})*,\\d+$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ht-HT.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ms-MY'})
SET f.display_name = 'ms-MY Formatting',
    f.content = 'Formatting rules for ms-MY',
    f.llm_context = 'ms-MY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: RM (Ringgit Malaysia) before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januari","Februari","Mac","April","Mei","Jun","Julai","Ogos","September","Oktober","November","Disember"],"month_abbrev":[],"day_names":["Isnin","Selasa","Rabu","Khamis","Jumaat","Sabtu","Ahad"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"PG (Pagi)","pm_indicator":"PTG (Petang)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PTG"},{"input":"09:00","output":"9:00 PG"},{"input":"23:59:59","output":"11:59:59 PTG"}],"incorrect_examples":[]}',
    f.currency = '{"code":"MYR (ISO 4217)","symbol":"RM (Ringgit Malaysia)","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"RM10.50"},{"input":"1234.56","output":"RM1,234.56"},{"input":"0.99","output":"RM0.99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Unit/Floor], [Building Name]\n[Street Number], [Street Name]\n[Taman/Area]\n[Postal Code] [City]\n[State]\nMALAYSIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Unit 12-3A, Menara XYZ\nNo. 88, Jalan Ampang\nTaman KLCC\n50450 Kuala Lumpur\nWilayah Persekutuan\nMALAYSIA"],"postal_code_examples":["50450"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard ISO paper size |","Malaysia uses the metric system for all official purposes","Road signs display distances in kilometers","Fuel is sold by the liter","Traditional unit \"kati\" (catty = approximately 0.6 kg) still used in wet markets alongside kg","\"Tahil\" (1/16 kati = approximately 37.5 g) used for precious items like gold"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["32°C"]}',
    f.validation_patterns = '{"phone (mobile)":"^0\\d{2}-\\d{3}\\s\\d{4}$","phone (landline)":"^0\\d-\\d{4}\\s\\d{4}$","phone (international)":"^\\+60\\s\\d{1,2}-\\d{3,4}\\s\\d{4}$","currency":"^RM\\d{1,3}(,\\d{3})*\\.\\d{2}$","time":"^\\d{1,2}:\\d{2}\\s(PG\\|PTG)$","date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","mykad (nric)":"^\\d{6}-\\d{2}-\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ms-MY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'sq-AL'})
SET f.display_name = 'sq-AL Formatting',
    f.content = 'Formatting rules for sq-AL',
    f.llm_context = 'sq-AL: Numbers use \'.\' decimal, \',\' thousands. Dates: D.M.YYYY (period separator, no leading zeros) (gregorian) Time: 24-hour Currency: Lek after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D.M.YYYY (period separator, no leading zeros)","short_pattern":"D.M.YY","long_pattern":"D MMMM YYYY (e.g., \"15 janar 2025\")","full_pattern":null,"date_separator":".","month_names":["janar","shkurt","mars","prill","maj","qershor","korrik","gusht","shtator","tetor","nentor","dhjetor"],"month_abbrev":[],"day_names":["e hene","e marte","e merkure","e enjte","e premte","e shtune","e diel"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.1.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"ALL (Albanian Lek)","symbol":"Lek","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10","output":"10 Lek"},{"input":"1234","output":"1 234 Lek"},{"input":"5000","output":"5 000 Lek"},{"input":"10.50","output":"10,50 Lek"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nALBANIA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Rruga e Dibres 123\n1001 Tirane\nALBANIA"],"postal_code_examples":["1001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210x297mm) |","Albania uses metric system exclusively","Imperial units not used in everyday contexts"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+355 \\d{1,2} \\d{3} \\d{4}$","number":"^-?\\d{1,3}( \\d{3})*,\\d+$","currency":"^\\d{1,3}( \\d{3})* Lek$","date":"^\\d{1,2}\\.\\d{1,2}\\.\\d{4}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^0\\d{1,2} \\d{3} \\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/sq-AL.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'mk-MK'})
SET f.display_name = 'mk-MK Formatting',
    f.content = 'Formatting rules for mk-MK',
    f.llm_context = 'mk-MK: Numbers use \'.\' decimal, \',\' thousands. Dates: d.M.yyyy (period separator, no leading zeros) (gregorian) Time: 24-hour Currency: ден. after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d.M.yyyy (period separator, no leading zeros)","short_pattern":"d.M.yy","long_pattern":"d MMMM yyyy (e.g., \"15 јануари 2025\")","full_pattern":null,"date_separator":".","month_names":["јануари","февруари","март","април","мај","јуни","јули","август","септември","октомври","ноември","декември"],"month_abbrev":[],"day_names":["понеделник","вторник","среда","четврток","петок","сабота","недела"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.1.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format with leading zero)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"MKD (Macedonian Denar)","symbol":"ден.","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 ден."},{"input":"1234.56","output":"1.234,56 ден."},{"input":"0.99","output":"0,99 ден."}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nNORTH MACEDONIA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ул. Македонија бр. 12\n1000 Скопје\nNORTH MACEDONIA"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210x297mm) |","North Macedonia uses metric system exclusively","Imperial units not used in everyday contexts"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"phone (national)":"^0\\d{2} \\d{3} \\d{3}$","phone (international)":"^\\+389 \\d{2} \\d{3} \\d{3}$","date":"^\\d{1,2}\\.\\d{1,2}\\.\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} ден\\.$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/mk-MK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'mr-IN'})
SET f.display_name = 'mr-IN Formatting',
    f.content = 'Formatting rules for mr-IN',
    f.llm_context = 'mr-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM, YYYY","full_pattern":null,"date_separator":"/","month_names":["जानेवारी","फेब्रुवारी","मार्च","एप्रिल","मे","जून","जुलै","ऑगस्ट","सप्टेंबर","ऑक्टोबर","नोव्हेंबर","डिसेंबर"],"month_abbrev":[],"day_names":["सोमवार","मंगळवार","बुधवार","गुरुवार","शुक्रवार","शनिवार","रविवार"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"म.पू.","pm_indicator":"म.उ.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 म.उ."},{"input":"09:00","output":"9:00 म.पू."},{"input":"23:59:59","output":"11:59:59 म.उ."}],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"₹10.50"},{"input":"1234.56","output":"₹1,234.56"},{"input":"0.99","output":"₹0.99"},{"input":"100000","output":"₹1,00,000"},{"input":"10000000","output":"₹1,00,00,000"},{"input":"750000.50","output":"₹7,50,000.50"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name / नाव]\n[Building/House Number], [Street/Area Name]\n[Locality / परिसर], [City/District / शहर] - [PIN Code]\n[State / राज्य]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["श्री. महेश पाटील\n१२३, शिवाजी रोड\nडेक्कन जिमखाना\nपुणे - 411004\nमहाराष्ट्र\nINDIA"],"postal_code_examples":["400001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","India uses metric system exclusively","Exceptions: Some traditional measurements still used in agriculture (e.g., guntha, acre for land in Maharashtra)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^\\d{5}\\s\\d{5}$","phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM\\|म\\.पू\\.\\|म\\.उ\\.)$","currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/mr-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-CU'})
SET f.display_name = 'es-CU Formatting',
    f.content = 'Formatting rules for es-CU',
    f.llm_context = 'es-CU: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"January 15, 2025"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"(not used)","pm_indicator":"(not used)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"},{"input":"20:00","output":"20:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"CUP","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"$10,50"},{"input":"1234.56","output":"$1 234,56"},{"input":"0.99","output":"$0,99"},{"input":"5000.00","output":"$5 000,00"},{"input":"24500.00","output":"$24 500,00"},{"input":"150.00","output":"$150,00"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] No. [Number] [Apartment/Floor]\ne/ [Cross Street 1] y [Cross Street 2]\n[Reparto/Barrio], [Municipio]\n[Postal Code] [City], [Provincia]\nCUBA","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Calle 23 No. 1251 Apto. 3\ne/ 12 y 14, Vedado\nPlaza de la Revolucion\n10400 La Habana, La Habana\nCUBA","Avenida de los Presidentes No. 405\ne/ 17 y 19, Vedado\nPlaza de la Revolucion\n10400 La Habana, La Habana\nCUBA"],"postal_code_examples":["10100"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US Letter (8.5\" x 11\") historically common |","Metric system is official and strictly enforced in Cuba","\"Libras\" (pounds) used colloquially for body weight and market produce","\"Caballerias\" (traditional land unit, 1 cab = 13.4 ha) still used for agricultural land","Fuel sold by liter at Servicentros (Cupet)","Speed limits shown in km/h exclusively","No imperial measurements in official contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","currency":"^\\$\\d{1,3}(\\s\\d{3})*(,\\d{2})?$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (international)":"^\\+53\\s\\d{1,2}\\s\\d{3}\\s\\d{4}$","phone (national)":"^\\d{1,2}\\s\\d{2,3}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-CU.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ru-KG'})
SET f.display_name = 'ru-KG Formatting',
    f.content = 'Formatting rules for ru-KG',
    f.llm_context = 'ru-KG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: сом (full name, commonly used) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY г.","full_pattern":null,"date_separator":".","month_names":["января","февраля","марта","апреля","мая","июня","июля","августа","сентября","октября","ноября","декабря"],"month_abbrev":[],"day_names":["понедельник","вторник","среда","четверг","пятница","суббота","воскресенье"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-11","output":"11.01.2026"},{"input":"2026-07-15","output":"15.07.2026"},{"input":"2026-03-21","output":"21.03.2026"}],"incorrect_examples":[{"input":"2026-01-11","output":"01/11/2026"},{"input":"2026-01-11","output":"11/01/2026"},{"input":"2026-01-11","output":"2026-01-11"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"KGS (Kyrgyzstani som)","symbol":"сом (full name, commonly used)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 сом"},{"input":"1234.56","output":"1 234,56 сом"},{"input":"0.99","output":"0,99 сом"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"{Street type} {Street Name}, {Building}, {Apartment if applicable}\n{Postal Code}, {City type} {City}\n{Oblast}\nКЫРГЫЗСТАН / KYRGYZSTAN","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ул. Чуй, 120, кв. 45\n720001, г. Бишкек\nКЫРГЫЗСТАН","пр. Манаса, 40, офис 305\n720017, г. Бишкек\nКЫРГЫЗСТАН","мкр. Асанбай, 25/1, кв. 67\n720064, г. Бишкек\nКЫРГЫЗСТАН","ул. Курманжан Датка, 271\n723500, г. Ош\nОшская область\nКЫРГЫЗСТАН","ул. Советская, 12\n722360, г. Чолпон-Ата\nИссык-Кульская область\nКЫРГЫЗСТАН"],"postal_code_examples":["720001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Пуд","unit":"пуд","symbol":"Traditional: 1 пуд = 16.38 кг (bulk goods, bazaars)","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null},{"category":"Altitude","unit":"Meters","symbol":"м","notes":null},{"category":"Location","unit":"Altitude","symbol":"Formatted","notes":null}],"paper_size":"A4","notes":["| Standard international |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["-15°C","-5°C"]}',
    f.validation_patterns = '{"passport (series an/ac)":"^A(N\\|C)\\d{7}$","phone (mobile)":"^\\+996 [57]\\d{2} \\d{2}-\\d{2}-\\d{2}$","phone (international)":"^\\+996 \\d{3} \\d{2}-\\d{2}-\\d{2}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? сом$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d$","phone (landline bishkek)":"^\\(312\\) \\d{2}-\\d{2}-\\d{2}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","phone (landline osh)":"^\\(322\\) \\d-\\d{2}-\\d{2}$","инн (tax id)":"^\\d{14}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ru-KG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ig-NG'})
SET f.display_name = 'ig-NG Formatting',
    f.content = 'Formatting rules for ig-NG',
    f.llm_context = 'ig-NG: Numbers use \'.\' decimal, \',\' thousands. Dates: D/M/YYYY (gregorian) Time: 12-hour Currency: ₦ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D/M/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Jenụwarị","Febrụwarị","Maachị","Epreel","Mee","Juun","Julaị","Ọgọọst","Septemba","Ọktoba","Novemba","Disemba"],"month_abbrev":[],"day_names":["Mọnde","Tuzdee","Wenezdee","Tọọzdee","Fraịdee","Satọdee","Sọndee"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"N\'ụtụtụ","pm_indicator":"N\'ehihie/N\'abalị","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"NGN","symbol":"₦","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Nọmba Ụlọ/Plot Aha Okporo Ụzọ\nMpaghara/Ogbe\nObodo\nSteeti\nNọmba Ozi\nNAỊJỊRỊA","postal_code_pattern":"NNNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Nọmba 25 Ogui Road\nNew Haven\nEnugu\nEnugu State\n400001\nNAỊJỊRỊA"],"postal_code_examples":["400001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilomita","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogram","symbol":"kg","notes":null},{"category":"Volume","unit":"Lita","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Usoro ISO 216 |","Nigeria na-eji usoro metric n\'ụzọ gọọmentị kwadoro","A na-egosi ebe ụzọ na kilomita","A na-ere mmanụ ụgbọala na lita","Ibu ahụ nwere ike ịbụ kilogram ma ọ bụ paụnd na nkwurịta okwu"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+234\\s?[7-9][0-1][0-9]{8}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(AM\\|PM)$","number":"^-?[\\d,]+(\\.\\d+)?$","phone (national)":"^0[7-9][0-1][0-9]{8}$","currency":"^₦[\\d,]+(\\.\\d{2})?$","date":"^([1-9]\\|[12][0-9]\\|3[01])/([1-9]\\|1[0-2])/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ig-NG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ka-GE'})
SET f.display_name = 'ka-GE Formatting',
    f.content = 'Formatting rules for ka-GE',
    f.llm_context = 'ka-GE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: ₾ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM, YYYY წ.","full_pattern":null,"date_separator":".","month_names":["იანვარი","თებერვალი","მარტი","აპრილი","მაისი","ივნისი","ივლისი","აგვისტო","სექტემბერი","ოქტომბერი","ნოემბერი","დეკემბერი"],"month_abbrev":[],"day_names":["ორშაბათი","სამშაბათი","ოთხშაბათი","ხუთშაბათი","პარასკევი","შაბათი","კვირა"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"GEL (ISO 4217)","symbol":"₾","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 ₾"},{"input":"1234.56","output":"1 234,56 ₾"},{"input":"0.99","output":"0,99 ₾"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Name] [Number]\n[Postal Code] [City]\nGEORGIA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["გიორგი მამულაშვილი\nრუსთაველის გამზირი 12\n0108 თბილისი\nGEORGIA"],"postal_code_examples":["0108"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"კმ","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"კგ","notes":null},{"category":"Volume","unit":"Liters","symbol":"ლ","notes":null}],"paper_size":"A4","notes":["| ISO standard paper sizes |","Always use metric units as primary - Georgia uses the metric system exclusively","Exception: aviation uses feet worldwide, but this is technical/international context"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^\\d{3} \\d{2} \\d{2} \\d{2}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","currency":"^-?\\d{1,3}( \\d{3})*(,\\d{2})? ₾$","phone (international)":"^\\+995 \\d{3} \\d{2} \\d{2} \\d{2}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ka-GE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-BH'})
SET f.display_name = 'ar-BH Formatting',
    f.content = 'Formatting rules for ar-BH',
    f.llm_context = 'ar-BH: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: د.ب after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"arabic-indic","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["يناير","فبراير","مارس","أبريل","مايو","يونيو","يوليو","أغسطس","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"١٥/٠١/٢٠٢٥"},{"input":"2025-12-31","output":"٣١/١٢/٢٠٢٥"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"٢:٣٠ م"},{"input":"09:00","output":"٩:٠٠ ص"},{"input":"23:59:59","output":"١١:٥٩:٥٩ م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"BHD","symbol":"د.ب","symbol_position":"after","space_between":true,"decimal_places":3,"subunit":null,"correct_examples":[{"input":"10.500","output":"١٠٫٥٠٠ د.ب"},{"input":"1234.560","output":"١٬٢٣٤٫٥٦٠ د.ب"},{"input":"0.999","output":"٠٫٩٩٩ د.ب"},{"input":"0.500","output":"٠٫٥٠٠ د.ب"},{"input":"0.160","output":"٠٫١٦٠ د.ب"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building Number] [Road Number]\n[Block Number]، [Area Name]\n[Governorate] [Postal Code]\nمملكة البحرين","postal_code_pattern":"NNN or NNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["مبنى ١٢٣ طريق ٤٥٦٧\nمجمع ٣١٨ المنامة\nمحافظة العاصمة ٣١٦\nمملكة البحرين","برج البحرين المالي، طابق ١٥\nطريق ٢٨٠٣، مجمع ٤٢٨\nالمنطقة الدبلوماسية ٣١٧\nمملكة البحرين","شقة ٤، مبنى ٧٨٩\nطريق ١٢٣٤، مجمع ٢٠٩\nعراد، محافظة المحرق ٢٢٨\nمملكة البحرين"],"postal_code_examples":["٣١٦"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°م","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"كم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"كجم","notes":null},{"category":"Volume","unit":"Liters","symbol":"ل","notes":null},{"category":"Oil","unit":"Barrels","symbol":"برميل","notes":null},{"category":"Gold","unit":"Tola","symbol":"تولة","notes":null}],"paper_size":"A4","notes":["| International standard (210 × 297 mm) |","Bahrain uses the metric system exclusively for official and daily use","Temperature always in Celsius (°م) for weather, medical, cooking","Distance in kilometers (كم) for road signs, maps","**Oil industry context**: As an oil-producing nation, Bahrain uses barrels (برميل) for oil measurements and may use imperial units (feet, inches) in technical petroleum contexts","**Gold trade**: Traditional tola (تولة) unit still used in souks alongside grams","Speed limits: Posted in km/h (e.g., ١٢٠ كم/س on highways)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","٥٠","12.5","١٠"]}',
    f.temperature = '{"format":"{number}°م","default_unit":"celsius","examples":["22°C","٢٢°C","45°C","٤٥°C","-5°C"]}',
    f.validation_patterns = '{"time":"^[١-٩٠]{1,2}:[٠-٥٩]{2}\\s[صم]$","currency (bhd)":"^[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]{3})?\\sد\\.ب$","number":"^[؜\\-+]?[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]+)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^[٣٦١]\\d{3}\\s\\d{4}$","phone (international)":"^\\+٩٧٣\\s\\d{4}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-BH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-LU'})
SET f.display_name = 'fr-LU Formatting',
    f.content = 'Formatting rules for fr-LU',
    f.llm_context = 'fr-LU: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY or DD.MM.YYYY (period also common due to German influence) (gregorian) Time: 24-hour (exclusively in Luxembourg) Currency: EUR after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","STATEC (statistics Luxembourg"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY or DD.MM.YYYY (period also common due to German influence)","short_pattern":"DD/MM/YY or DD.MM.YY","long_pattern":"D MMMM YYYY","full_pattern":"EEEE D MMMM YYYY","date_separator":"/","month_names":["janvier","fevrier","mars","avril","mai","juin","juillet","aout","septembre","octobre","novembre","decembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour (exclusively in Luxembourg)","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (Euro)","symbol":"EUR","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":"centime (1 EUR = 100 centimes)","correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Number], [Street Type] [Street Name]\nL-[Postal Code] [City]\nLUXEMBOURG","postal_code_pattern":"L-NNNN (L- prefix + 4 digits)","postal_code_position":"before_city","city_format":"Standard case (not necessarily uppercase)","street_types":null,"po_box_format":null,"example_addresses":["M. Jean SCHMIT\n42, avenue de la Liberte\nL-1930 Luxembourg\nLUXEMBOURG","Banque Internationale a Luxembourg\n69, route d\'Esch\nL-2953 Luxembourg\nLUXEMBOURG","Centre Administratif de l\'Etat\n1, rue du Pllebiscite\nL-2341 Luxembourg\nLUXEMBOURG"],"postal_code_examples":["L-1930"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Fuel","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Luxembourg uses metric system exclusively","Imperial units never used in official contexts","Exception: aviation uses feet for altitude (international standard)","Fuel prices displayed per liter (attract cross-border buyers from FR/BE/DE)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C","0°C"]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} (EUR\\|€)$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","iban":"^LU\\d{2} \\d{4} \\d{4} \\d{4} \\d{4}( \\d{2})?$","phone (national 6-digit)":"^\\d{2} \\d{2} \\d{2}$","phone (national 9-digit)":"^\\d{3} \\d{3} \\d{3}$","phone (international)":"^\\+352 \\d{2,3}( \\d{2,3}){1,3}$","rcs (company)":"^B\\d{5,6}$","date":"^\\d{2}[/\\.]\\d{2}[/\\.]\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-LU.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-ZW'})
SET f.display_name = 'en-ZW Formatting',
    f.content = 'Formatting rules for en-ZW',
    f.llm_context = 'en-ZW: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (Day-Month-Year order, British convention) (gregorian) Time: 24-hour (formal/official), 12-hour (informal/casual) Currency: $ or US$ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (Day-Month-Year order, British convention)","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour (formal/official), 12-hour (informal/casual)","pattern":"HH:mm (24-hour) or h:mm a (12-hour)","pattern_with_seconds":"HH:mm:ss or h:mm:ss a","time_separator":":","am_indicator":"AM (uppercase)","pm_indicator":"PM (uppercase)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"USD (ISO 4217: United States Dollar - official since 2009)","symbol":"$ or US$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[Suburb/Area]\n[City/Town]\nZIMBABWE","postal_code_pattern":"Not widely used (physical addresses and P.O. Boxes preferred)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["14 Samora Machel Avenue\nEastlea\nHarare\nZIMBABWE","P.O. Box 1234\nHarare\nZIMBABWE","25 Doyle Road\nBorrowdale\nHarare\nZIMBABWE","45 Fife Street\nCity Centre\nBulawayo\nZIMBABWE","Stand 5678\nHighfield\nHarare\nZIMBABWE"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","Zimbabwe uses the metric system as the official measurement standard","British imperial units may occasionally appear due to colonial history but metric is standard","Exception: aviation uses feet and nautical miles worldwide","Land area: often measured in hectares; acres may appear in older documents","Agricultural produce: commonly sold by kilogram","Fuel: sold in litres; fuel economy expressed in km/L","Height: officially in metres/centimetres; informally feet/inches may be used"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["25°C","12°C","35°C","37°C"]}',
    f.validation_patterns = '{"currency":"^\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","phone (national)":"^0[4-9][0-9]?\\s?[0-9]{3}\\s?[0-9]{3,4}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","phone (international)":"^\\+263\\s?[4-9][0-9]?\\s?[0-9]{3}\\s?[0-9]{3,4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-ZW.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-IN'})
SET f.display_name = 'en-IN Formatting',
    f.content = 'Formatting rules for en-IN',
    f.llm_context = 'en-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: Rs. or ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"Rs. or ₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Mr. Rajesh Kumar\n123, MG Road\nKoramangala\nBengaluru - 560034\nKarnataka\nINDIA"],"postal_code_examples":["110001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210 x 297 mm) |","India uses metric system exclusively for official purposes","Some traditional measurements still used informally (e.g., tola for gold, bigha for land)","Aviation follows international standards (feet for altitude)","Speed limits displayed in km/h"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^\\d{5}\\s\\d{5}$","phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(AM\\|PM)$","currency":"^(₹\\|Rs\\.)\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'zh-TW'})
SET f.display_name = 'zh-TW Formatting',
    f.content = 'Formatting rules for zh-TW',
    f.llm_context = 'zh-TW: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY年M月D日 (gregorian) Time: 12-hour Currency: NT$ or $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY年M月D日","short_pattern":"YYYY/M/D","long_pattern":"YYYY年M月D日","full_pattern":null,"date_separator":"/","month_names":["1月  # yī yuè","2月  # èr yuè","3月  # sān yuè","4月  # sì yuè","5月  # wǔ yuè","6月  # liù yuè","7月  # qī yuè","8月  # bā yuè","9月  # jiǔ yuè","10月  # shí yuè","11月  # shíyī yuè","12月  # shí\'èr yuè"],"month_abbrev":[],"day_names":["星期一  # xīngqī yī (also 禮拜一 lǐbài yī in Taiwan)","星期二  # xīngqī èr (also 禮拜二)","星期三  # xīngqī sān (also 禮拜三)","星期四  # xīngqī sì (also 禮拜四)","星期五  # xīngqī wǔ (also 禮拜五)","星期六  # xīngqī liù (also 禮拜六)","星期日  # xīngqī rì (also 禮拜天/禮拜日 in Taiwan)"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025年1月15日"},{"input":"2025-12-31","output":"2025年12月31日"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025-1-15"}]}',
    f.time = '{"system":"12-hour","pattern":"ah:mm","pattern_with_seconds":"ah:mm:ss","time_separator":":","am_indicator":"上午","pm_indicator":"下午","prayer_times":null,"correct_examples":[{"input":"14:30","output":"下午2:30"},{"input":"09:00","output":"上午9:00"},{"input":"23:59:59","output":"下午11:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"TWD","symbol":"NT$ or $","symbol_position":"before","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10","output":"NT$10"},{"input":"1234","output":"NT$1,234"},{"input":"99","output":"NT$99"},{"input":"1000","output":"NT$1,000"},{"input":"50000","output":"NT$50,000"},{"input":"1000000","output":"NT$1,000,000"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Postal Code]\n[County/City] [District/Township] [Road/Street] [Section] [Lane] [Alley] [Number] [Floor]\nTAIWAN","postal_code_pattern":"NNN or NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["106\n台北市大安區忠孝東路四段216巷27弄12號5樓\nTAIWAN","10690\n台北市大安區忠孝東路四段216巷27弄12號5樓\nTAIWAN"],"postal_code_examples":["100"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210x297mm) |","**Taiwan jin (台斤)**: 1 斤 = 600g (different from China\'s 500g)","**Taiwan liang (台兩)**: 1 兩 = 37.5g (used in traditional markets)","**Ping (坪)**: 1 坪 = 3.3058 m2 (standard for real estate)","**Metric primary**: All official and technical contexts use metric","**No imperial**: Imperial units not used domestically"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"time":"^[上下]午\\d{1,2}:\\d{2}(:\\d{2})?$","date":"^\\d{4}年\\d{1,2}月\\d{1,2}日$","currency":"^NT\\$-?\\d{1,3}(,\\d{3})*$","phone (international)":"^\\+886 \\d{1,2}-\\d{3,4}-\\d{3,4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","phone (national)":"^0\\d{1,2}-\\d{3,4}-\\d{3,4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/zh-TW.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'de-CH'})
SET f.display_name = 'de-CH Formatting',
    f.content = 'Formatting rules for de-CH',
    f.llm_context = 'de-CH: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day, month, year with period separators) (gregorian) Time: 24-hour Currency: CHF or Fr. before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day, month, year with period separators)","short_pattern":"DD.MM.YY (two-digit year)","long_pattern":"D. MMMM YYYY (e.g., \"15. Januar 2025\")","full_pattern":null,"date_separator":".","month_names":["Januar","Februar","März","April","Mai","Juni","Juli","August","September","Oktober","November","Dezember"],"month_abbrev":[],"day_names":["Montag","Dienstag","Mittwoch","Donnerstag","Freitag","Samstag","Sonntag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format with leading zeros)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"CHF (ISO 4217)","symbol":"CHF or Fr.","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"CHF 10.50"},{"input":"1234.56","output":"CHF 1\'234.55"},{"input":"0.99","output":"CHF 1.00"},{"input":"19.95","output":"CHF 19.95"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[First Name] [Last Name]\n[Street Name] [House Number]\n[Postal Code] [City]\nSWITZERLAND","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Hans Muster\nBahnhofstrasse 42\n8001 Zürich\nSWITZERLAND"],"postal_code_examples":["8001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L or l","notes":null}],"paper_size":"A4","notes":["| Standard: 210 x 297 mm |","Metric system used exclusively for all measurements","No imperial units in common use","Fuel consumption expressed as liters per 100 km (L/100 km)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C","36.5°C"]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\'\\d{3})*(\\.\\d+)?$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (international)":"^\\+41\\s\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","phone (national)":"^0\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","currency":"^CHF\\s\\d{1,3}(\'\\d{3})*(\\.\\d{2})?$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/de-CH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'pt-BR'})
SET f.display_name = 'pt-BR Formatting',
    f.content = 'Formatting rules for pt-BR',
    f.llm_context = 'pt-BR: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour (primarily), 12-hour (informal) Currency: R$ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D \'de\' MMMM \'de\' YYYY","full_pattern":null,"date_separator":"/","month_names":["janeiro","fevereiro","março","abril","maio","junho","julho","agosto","setembro","outubro","novembro","dezembro"],"month_abbrev":[],"day_names":["segunda-feira","terça-feira","quarta-feira","quinta-feira","sexta-feira","sábado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour (primarily), 12-hour (informal)","pattern":"HH:mm (24-hour)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"AM (rarely used, informal contexts)","pm_indicator":"PM (rarely used, informal contexts)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"BRL (ISO 4217)","symbol":"R$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"R$ 10,50"},{"input":"1234.56","output":"R$ 1.234,56"},{"input":"0.99","output":"R$ 0,99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name], [Number] [- Complement]\n[Neighborhood]\n[City] - [State] [Postal Code]\nBRASIL","postal_code_pattern":"NNNNN-NNN (8 digits: 5 digits, hyphen, 3 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Rua Paulista, 1578 - Conjunto 42\nBela Vista\nSão Paulo - SP 01310-200\nBRASIL"],"postal_code_examples":["01310-200"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 × 297 mm |","Brazil uses the metric system exclusively for official purposes","Imperial units may appear in informal contexts (e.g., screen sizes in inches) but metric is always primary"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","currency":"^R\\$ \\d{1,3}(\\.\\d{3})*(,\\d{2})$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national landline)":"^\\(\\d{2}\\) \\d{4}-\\d{4}$","phone (international)":"^\\+55 \\d{2} \\d{4,5}-\\d{4}$","phone (national mobile)":"^\\(\\d{2}\\) \\d{5}-\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/pt-BR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fa-IR'})
SET f.display_name = 'fa-IR Formatting',
    f.content = 'Formatting rules for fa-IR',
    f.llm_context = 'fa-IR: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY/MM/DD (hijri) Time: 24-hour Currency: ﷼ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY/MM/DD","short_pattern":"YY/MM/DD","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["فروردین (Farvardin)  # March 21 - April 20","اردیبهشت (Ordibehesht)  # April 21 - May 21","خرداد (Khordad)  # May 22 - June 21","تیر (Tir)  # June 22 - July 22","مرداد (Mordad)  # July 23 - August 22","شهریور (Shahrivar)  # August 23 - September 22","مهر (Mehr)  # September 23 - October 22","آبان (Aban)  # October 23 - November 21","آذر (Azar)  # November 22 - December 21","دی (Dey)  # December 22 - January 20","بهمن (Bahman)  # January 21 - February 19","اسفند (Esfand)  # February 20 - March 20"],"month_abbrev":[],"day_names":["دوشنبه (Doshanbe)","سه‌شنبه (Seshanbe)","چهارشنبه (Chaharshanbe)","پنج‌شنبه (Panjshanbe)","جمعه (Jom\'e)","شنبه (Shanbe)","یکشنبه (Yekshanbe)"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"۱۴۰۳/۱۰/۲۵"},{"input":"2025-12-31","output":"۱۴۰۴/۱۰/۱۰"}],"incorrect_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"ق.ظ (qabl az zohr - before noon)","pm_indicator":"ب.ظ (ba\'d az zohr - after noon)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"۱۴:۳۰"},{"input":"09:00","output":"۰۹:۰۰"},{"input":"23:59:59","output":"۲۳:۵۹:۵۹"}],"incorrect_examples":[]}',
    f.currency = '{"code":"IRR","symbol":"﷼","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10.50","output":"۱۱ ریال"},{"input":"1234.56","output":"۱,۲۳۵ ریال"},{"input":"0.99","output":"۱ ریال"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street name], [Building/Unit number]\n[Neighborhood/District], [City]\n[Province]\n[Postal Code], IRAN","postal_code_pattern":"NNNNNNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["خیابان آزادی، پلاک ۷۸، واحد ۱۲\nمحله ستارخان، تهران\nاستان تهران\nکد پستی: ۱۴۳۵۸۸۳۷۴۱\nایران"],"postal_code_examples":["۱۴۱۵۷۸۳۷۴۱"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard international |","Iran uses the metric system exclusively for official and everyday measurements","Traditional Persian units (مان، من، سیر) are obsolete but occasionally used in informal/rural contexts","Aviation and some technical fields follow international standards"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["25","۲۵"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["۲۲°C"]}',
    f.validation_patterns = '{"time":"^[۰-۹]{2}:[۰-۹]{2}(:[۰-۹]{2})?$","phone (national)":"^۰[۰-۹]{2,3}\\s[۰-۹]{3,4}\\s[۰-۹]{4}$","date":"^[۰-۹]{4}/[۰-۹]{2}/[۰-۹]{2}$","number":"^−?[۰-۹]{1,3}(,[۰-۹]{3})*(/[۰-۹]+)?$","phone (international)":"^\\+۹۸\\s[۰-۹]{2,3}\\s[۰-۹]{3,4}\\s[۰-۹]{4}$","currency":"^[۰-۹]{1,3}(,[۰-۹]{3})*\\s(ریال\\|تومان)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fa-IR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'de-AT'})
SET f.display_name = 'de-AT Formatting',
    f.content = 'Formatting rules for de-AT',
    f.llm_context = 'de-AT: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day, month, year with period separators) (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day, month, year with period separators)","short_pattern":"DD.MM.YY (two-digit year)","long_pattern":"D. MMMM YYYY (e.g., \"15. Jänner 2025\")","full_pattern":null,"date_separator":".","month_names":["Jänner (AUSTRIAN - not \"Januar\")","Februar (also informal: Feber)","März","April","Mai","Juni","Juli","August","September","Oktober","November","Dezember"],"month_abbrev":[],"day_names":["Montag","Dienstag","Mittwoch","Donnerstag","Freitag","Samstag","Sonntag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"15. Januar 2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format with leading zeros)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (ISO 4217)","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1.234,56 €"},{"input":"0.99","output":"0,99 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Straße] [Hausnummer]/[Stiege]/[Türnummer]\n[Postleitzahl] [Stadt]\nÖSTERREICH","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Mariahilfer Straße 45/2/12\n1060 Wien\nÖSTERREICH"],"postal_code_examples":["1010"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometer","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogramm","symbol":"kg","notes":null},{"category":"Volume","unit":"Liter","symbol":"L or l","notes":null}],"paper_size":"A4","notes":["| Standard: 210 x 297 mm |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100","0,5"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"date":"^\\d{1,2}\\.\\d{1,2}\\.\\d{4}$","phone (national)":"^0\\d{1,4}[\\s]\\d{2,3}([\\s]\\d{2,3}){1,2}$","currency":"^\\d{1,3}([\\.\\s]\\d{3})*(,\\d{2})?\\s€$","steuernummer":"^\\d{2}[\\s-]?\\d{6}$","time":"^([01]\\d\\|2[0-3])[:\\.][0-5]\\d([:\\.][0-5]\\d)?$","number":"^-?\\d{1,3}([\\.\\s]\\d{3})*(,\\d+)?$","personalausweis":"^[A-Z]\\d{7}$","sozialversicherungsnummer (svnr)":"^\\d{4}\\s?\\d{6}$","kennzeichen (plates)":"^[A-Z]{1,2}\\s\\d{1,5}\\s?[A-Z]{0,3}$","reisepass":"^[A-Z]\\d{7}$","phone (international)":"^\\+43[\\s]\\d{1,4}([\\s]\\d{2,3}){2,3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/de-AT.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-TN'})
SET f.display_name = 'fr-TN Formatting',
    f.content = 'Formatting rules for fr-TN',
    f.llm_context = 'fr-TN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: DT (dinar tunisien) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["janvier","fevrier","mars","avril","mai","juin","juillet","aout","septembre","octobre","novembre","decembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"TND","symbol":"DT (dinar tunisien)","symbol_position":"after","space_between":true,"decimal_places":3,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Type] [Street Name]\n[Postal Code] [City]\n[Governorate]\nTUNISIE","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["42 avenue Habib Bourguiba\n1000 Tunis\nGouvernorat de Tunis\nTUNISIE"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogrammes","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","La Tunisie utilise exclusivement le systeme metrique (heritage colonial francais)","Les unites imperiales (miles, livres, Fahrenheit) ne sont pas utilisees"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+216\\s\\d{2}\\s\\d{3}\\s\\d{3}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","number":"^-?\\d{1,3}(\\s\\d{3})*,\\d+$","phone (national)":"^\\d{2}\\s\\d{3}\\s\\d{3}$","date":"^\\d{2}/\\d{2}/\\d{4}$","currency":"^\\d{1,3}(\\s\\d{3})*,\\d{3}\\sDT$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-TN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-TT'})
SET f.display_name = 'en-TT Formatting',
    f.content = 'Formatting rules for en-TT',
    f.llm_context = 'en-TT: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: TT$ or $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"TTD (ISO 4217)","symbol":"TT$ or $","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Number] [Street Name]\n[City/Town], [POSTAL CODE]\nTRINIDAD AND TOBAGO","postal_code_pattern":"6 digits (NNNNNN) - Note: postal codes are not widely used in Trinidad and Tobago","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Mr. John Williams\n15 Frederick Street\nPort of Spain\nTRINIDAD AND TOBAGO"],"postal_code_examples":["190101"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US Letter (8.5x11 in) commonly used |","Official metric system since 1970s, but imperial units persist in daily conversation","Fuel sold in litres; body weight often discussed in pounds"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM)$","phone (international)":"^\\+1\\s?\\d{3}\\s?\\d{3}\\s?\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","currency":"^(TT\\$|\\$)\\d{1,3}(,\\d{3})*\\.\\d{2}$","phone (national)":"^\\(\\d{3}\\)\\s?\\d{3}-\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-TT.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'is-IS'})
SET f.display_name = 'is-IS Formatting',
    f.content = 'Formatting rules for is-IS',
    f.llm_context = 'is-IS: Numbers use \'.\' decimal, \',\' thousands. Dates: D.M.YYYY (day.month.year, no leading zeros) (gregorian) Time: 24-hour Currency: kr. after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D.M.YYYY (day.month.year, no leading zeros)","short_pattern":"D.M.YYYY","long_pattern":"D. MMMM YYYY","full_pattern":null,"date_separator":".","month_names":["janúar","febrúar","mars","apríl","maí","júní","júlí","ágúst","september","október","nóvember","desember"],"month_abbrev":[],"day_names":["mánudagur","þriðjudagur","miðvikudagur","fimmtudagur","föstudagur","laugardagur","sunnudagur"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.1.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"ISK (Icelandic Krona)","symbol":"kr.","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10","output":"10 kr."},{"input":"1234","output":"1.234 kr."},{"input":"1000000","output":"1.000.000 kr."},{"input":"1000","output":"1.000 kr."}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nICELAND","postal_code_pattern":"NNN (3 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Laugavegur 28\n101 Reykjavík\nICELAND"],"postal_code_examples":["101"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210x297 mm) |","Iceland uses the metric system exclusively for all measurements","Imperial units should only be shown in parentheses when relevant for international context"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C","10°C"]}',
    f.validation_patterns = '{"currency":"^-?\\d{1,3}(\\.\\d{3})* kr\\.$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^\\d{3} \\d{4}$","phone (international)":"^\\+354 \\d{3} \\d{4}$","date":"^\\d{1,2}\\.\\d{1,2}\\.\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/is-IS.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ca-ES'})
SET f.display_name = 'ca-ES Formatting',
    f.content = 'Formatting rules for ca-ES',
    f.llm_context = 'ca-ES: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: EUR after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["gener","febrer","marc","abril","maig","juny","juliol","agost","setembre","octubre","novembre","desembre"],"month_abbrev":[],"day_names":["dilluns","dimarts","dimecres","dijous","divendres","dissabte","diumenge"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"24-hour","pattern":"H:mm","pattern_with_seconds":"H:mm:ss","time_separator":":","am_indicator":"a. m.","pm_indicator":"p. m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"9:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"0:00"},{"input":"12:00","output":"12:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"EUR","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 EUR"},{"input":"1234.56","output":"1.234,56 EUR"},{"input":"0.99","output":"0,99 EUR"},{"input":"1500.00","output":"1.500,00 EUR"},{"input":"10.50","output":"10,50 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name], [Number] [Floor] [Door]\n[Postal Code] [City]\n[Province]\nESPANYA","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Carrer de Balmes, 123 4t 2a\n08008 Barcelona\nBarcelona\nESPANYA"],"postal_code_examples":["08001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO A4 standard (210 mm x 297 mm) |","Metric system is official and standard throughout Spain and Catalonia","EU standards apply for all measurements","Food products labeled in grams/kilograms and milliliters/liters","Road signs show distances in kilometers and speed limits in km/h","Weather temperatures always in Celsius"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^\\d{3} \\d{2} \\d{2} \\d{2}$","phone (international)":"^\\+34 \\d{3} \\d{2} \\d{2} \\d{2}$","date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","time":"^\\d{1,2}:\\d{2}(:\\d{2})?$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} (EUR|€)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ca-ES.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ru-KZ'})
SET f.display_name = 'ru-KZ Formatting',
    f.content = 'Formatting rules for ru-KZ',
    f.llm_context = 'ru-KZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: ₸ (tenge symbol, Unicode U+20B8) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY г.","full_pattern":null,"date_separator":".","month_names":["января","февраля","марта","апреля","мая","июня","июля","августа","сентября","октября","ноября","декабря"],"month_abbrev":[],"day_names":["понедельник","вторник","среда","четверг","пятница","суббота","воскресенье"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-11","output":"11.01.2026"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2026-01-11","output":"01/11/2026"},{"input":"2026-01-11","output":"2026-01-11"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"KZT (Kazakhstani Tenge)","symbol":"₸ (tenge symbol, Unicode U+20B8)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"500","output":"500 ₸"},{"input":"2000","output":"2 000 ₸"},{"input":"15000","output":"15 000 ₸"},{"input":"1234.56","output":"1 234,56 ₸"},{"input":"0.99","output":"0,99 ₸"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name], д. [Building], кв. [Apartment]\n[Postal Code], г. [City]\n[Region/Oblast or City of Republican Significance]\nКАЗАХСТАН","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["проспект Абая, д. 52, кв. 15\n050008, г. Алматы\nКАЗАХСТАН","улица Кунаева, д. 12/1, офис 305\n010000, г. Астана\nКАЗАХСТАН","улица Ленина, д. 45, кв. 8\n100012, г. Караганда\nКарагандинская область\nКАЗАХСТАН"],"postal_code_examples":["050000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["vast distances (2 800 км east-west) |","| Standard format |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["-25°C","-45°C"]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? ₸$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","time":"^([01]\\d|2[0-3]):[0-5]\\d$","phone (international)":"^\\+7 7\\d{2,3} \\d{2,3}-\\d{2}-\\d{2}$","phone (national)":"^8 \\(7\\d{2,3}\\) \\d{2,3}-\\d{2}-\\d{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ru-KZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'kn-IN'})
SET f.display_name = 'kn-IN Formatting',
    f.content = 'Formatting rules for kn-IN',
    f.llm_context = 'kn-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["ಜನವರಿ","ಫೆಬ್ರವರಿ","ಮಾರ್ಚ್","ಏಪ್ರಿಲ್","ಮೇ","ಜೂನ್","ಜುಲೈ","ಆಗಸ್ಟ್","ಸೆಪ್ಟೆಂಬರ್","ಅಕ್ಟೋಬರ್","ನವೆಂಬರ್","ಡಿಸೆಂಬರ್"],"month_abbrev":[],"day_names":["ಸೋಮವಾರ","ಮಂಗಳವಾರ","ಬುಧವಾರ","ಗುರುವಾರ","ಶುಕ್ರವಾರ","ಶನಿವಾರ","ಭಾನುವಾರ"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ಪೂರ್ವಾಹ್ನ","pm_indicator":"ಅಪರಾಹ್ನ","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ಶ್ರೀ ರಾಜೇಶ್ ಕುಮಾರ್\n123, ಎಂ.ಜಿ. ರಸ್ತೆ\nಕೋರಮಂಗಲ\nಬೆಂಗಳೂರು - 560034\nಕರ್ನಾಟಕ\nINDIA"],"postal_code_examples":["560001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","India uses metric system exclusively","Exceptions: Some traditional measurements still used in agriculture (e.g., acre, guntha for land in Karnataka)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM\\|ಪೂರ್ವಾಹ್ನ\\|ಅಪರಾಹ್ನ)$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","phone (national)":"^\\d{5}\\s\\d{5}$","phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/kn-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ro-RO'})
SET f.display_name = 'ro-RO Formatting',
    f.content = 'Formatting rules for ro-RO',
    f.llm_context = 'ro-RO: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (period separator) (gregorian) Time: 24-hour Currency: lei (plural), leu (singular) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (period separator)","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":".","month_names":["ianuarie","februarie","martie","aprilie","mai","iunie","iulie","august","septembrie","octombrie","noiembrie","decembrie"],"month_abbrev":[],"day_names":["luni","marti","miercuri","joi","vineri","sambata","duminica"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"RON (Romanian New Leu - introduced 2005, replacing ROL at 10,000:1)","symbol":"lei (plural), leu (singular)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Strada [Street Name], nr. [Number], bl. [Block], sc. [Staircase], et. [Floor], ap. [Apartment]\n[Postal Code] [City], [Judet/Sector]\nROMANIA","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Strada Victoriei, nr. 15, bl. A1, sc. B, et. 3, ap. 12\n010063 Bucuresti, Sector 1\nROMANIA","Strada Mihai Eminescu, nr. 24, ap. 8\n400001 Cluj-Napoca, Cluj\nROMANIA"],"postal_code_examples":["010063"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210 x 297 mm) |","Romania uses metric system exclusively for all measurements (EU standard)","Imperial units rarely used; if needed, always show metric first with imperial in parentheses"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","currency":"^-?\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\slei$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^0\\d{3}\\s\\d{3}\\s\\d{3}$","cnp":"^[1-8]\\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\\d|3[01])(0[1-9]|[1-4]\\d|5[0-2])\\d{4}$","phone (international)":"^\\+40\\s\\d{3}\\s\\d{3}\\s\\d{3}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","cui":"^\\d{2,10}$","cif":"^RO\\d{2,10}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ro-RO.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'gu-IN'})
SET f.display_name = 'gu-IN Formatting',
    f.content = 'Formatting rules for gu-IN',
    f.llm_context = 'gu-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM, YYYY","full_pattern":null,"date_separator":"/","month_names":["જાન્યુઆરી","ફેબ્રુઆરી","માર્ચ","એપ્રિલ","મે","જૂન","જુલાઈ","ઑગસ્ટ","સપ્ટેમ્બર","ઑક્ટોબર","નવેમ્બર","ડિસેમ્બર"],"month_abbrev":[],"day_names":["સોમવાર","મંગળવાર","બુધવાર","ગુરુવાર","શુક્રવાર","શનિવાર","રવિવાર"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"23:59:59","output":"11:59:59 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"₹10.50"},{"input":"1234.56","output":"₹1,234.56"},{"input":"0.99","output":"₹0.99"},{"input":"100000","output":"₹1,00,000"},{"input":"10000000","output":"₹1,00,00,000"},{"input":"750000.50","output":"₹7,50,000.50"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["શ્રી રાજેશ પટેલ\n123, આશ્રમ રોડ\nનવરંગપુરા\nઅમદાવાદ - 380009\nગુજરાત\nINDIA"],"postal_code_examples":["380001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","India uses metric system exclusively","Exceptions: Some traditional measurements still used in agriculture (e.g., vigha/વીઘા for land in Gujarat)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","phone (national)":"^\\d{5}\\s\\d{5}$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM)$","phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/gu-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'pt-AO'})
SET f.display_name = 'pt-AO Formatting',
    f.content = 'Formatting rules for pt-AO',
    f.llm_context = 'pt-AO: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: Kz after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D \'de\' MMMM \'de\' YYYY","full_pattern":null,"date_separator":"/","month_names":["janeiro","fevereiro","marco","abril","maio","junho","julho","agosto","setembro","outubro","novembro","dezembro"],"month_abbrev":[],"day_names":["segunda-feira","terca-feira","quarta-feira","quinta-feira","sexta-feira","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15/01/2026"},{"input":"2026-12-31","output":"31/12/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"2026/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"(not used - 24-hour system standard)","pm_indicator":"(not used - 24-hour system standard)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"AOA (ISO 4217)","symbol":"Kz","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 Kz"},{"input":"1234.56","output":"1 234,56 Kz"},{"input":"0.99","output":"0,99 Kz"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name], [Number]\n[Bairro/Neighbourhood]\n[Municipio], [Provincia]\nANGOLA","postal_code_pattern":"Not standardized (use Caixa Postal: C.P. NNNN)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Rua Rainha Ginga, 45\nMaianga\nLuanda, Luanda\nANGOLA","Avenida 4 de Fevereiro, 102\nC.P. 1234\nLuanda, Luanda\nANGOLA"],"postal_code_examples":["C.P. 1234"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Angola uses the metric system exclusively for official purposes","Inherited from Portuguese colonial administration","Fuel prices displayed per liter (approximately 300-400 Kz/L in 2026)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"bi (bilhete de identidade)":"^\\d{9}[A-Z]{2}\\d{3}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2}) Kz$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","date":"^\\d{2}/\\d{2}/\\d{4}$","nif (numero de identificacao fiscal)":"^\\d{10}$","phone (international)":"^\\+244 \\d{3} \\d{3} \\d{3}$","phone (national)":"^\\d{3} \\d{3} \\d{3}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/pt-AO.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ny-MW'})
SET f.display_name = 'ny-MW Formatting',
    f.content = 'Formatting rules for ny-MW',
    f.llm_context = 'ny-MW: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: MK before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januwale","Febuluwale","Malichi","Epulo","Meyi","Juni","Julai","Ogasiti","Seputembala","Okutobala","Novembala","Disembala"],"month_abbrev":[],"day_names":["Lolemba","Lachiwiri","Lachitatu","Lachinayi","Lachisanu","Loweruka","Lamulungu"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"MWK","symbol":"MK","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[House/Plot Number] [Street Name]\n[Area/Township]\n[City/Town]\n[Postal Code]\nMALAWI","postal_code_pattern":"No standard postal code system in widespread use","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Plot 23 Victoria Avenue\nOld Town\nLilongwe 3\nMALAWI","P.O. Box 234\nLilongwe\nMALAWI","Private Bag 1\nZomba\nMALAWI","House 45 Chileka Road\nSunnyside\nBlantyre\nMALAWI","Village: Mwanza\nTraditional Authority: Mwanza\nMwanza District\nMALAWI"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","Malawi uses the metric system as the official measurement standard","British imperial units may occasionally appear in informal contexts due to colonial history","Exception: aviation uses feet and nautical miles worldwide","Land area: often measured in hectares; acres used informally (colonial legacy)","Agricultural produce: commonly sold by kilogram; local markets use traditional measures (pails, basins, bags - 50kg and 90kg)","Height: sometimes expressed in feet/inches informally, but metres/centimetres officially","Fuel: sold in litres; fuel economy expressed in km/L","Traditional measures: \"mtanga\" (basket) and \"chikho\" (cup) still used in rural markets"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["25°C","15°C","35°C","37°C"]}',
    f.validation_patterns = '{"number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","phone (national)":"^0[89][0-9] [0-9]{3} [0-9]{4}$","time":"^(0?[1-9]\\|1[0-2]):[0-5][0-9] (AM\\|PM)$","phone (international)":"^\\+265 [89][0-9] [0-9]{3} [0-9]{4}$","currency":"^MK [0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ny-MW.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'or-IN'})
SET f.display_name = 'or-IN Formatting',
    f.content = 'Formatting rules for or-IN',
    f.llm_context = 'or-IN: Numbers use Indian numbering system (lakhs, crores). Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"odia","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":[],"month_abbrev":[],"day_names":[],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":false,"decimal_places":2,"subunit":"paisa","correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+91","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"","postal_code_pattern":"","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":[],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[],"paper_size":"A4","notes":[]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/or-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'zh-CN'})
SET f.display_name = 'zh-CN Formatting',
    f.content = 'Formatting rules for zh-CN',
    f.llm_context = 'zh-CN: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY年M月D日 (gregorian) Time: 24-hour Currency: ¥ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY年M月D日","short_pattern":"YYYY/M/D","long_pattern":"YYYY年M月D日","full_pattern":null,"date_separator":"/","month_names":["1月  # yī yuè","2月  # èr yuè","3月  # sān yuè","4月  # sì yuè","5月  # wǔ yuè","6月  # liù yuè","7月  # qī yuè","8月  # bā yuè","9月  # jiǔ yuè","10月  # shí yuè","11月  # shíyī yuè","12月  # shí\'èr yuè"],"month_abbrev":[],"day_names":["星期一  # xīngqī yī","星期二  # xīngqī èr","星期三  # xīngqī sān","星期四  # xīngqī sì","星期五  # xīngqī wǔ","星期六  # xīngqī liù","星期日  # xīngqī rì (also 星期天)"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025年1月15日"},{"input":"2025-12-31","output":"2025年12月31日"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025-1-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"上午","pm_indicator":"下午","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"CNY","symbol":"¥","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"¥10.50"},{"input":"1234.56","output":"¥1,234.56"},{"input":"0.99","output":"¥0.99"},{"input":"1000","output":"¥1,000.00"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Country] [Province/Municipality] [City] [District]\n[Street Name] [Building/Number]\n[Postal Code]","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["中国北京市朝阳区\n建国门外大街1号\n100020","CHINA, Beijing Municipality, Chaoyang District\nJianguomenwai Avenue No. 1\n100020"],"postal_code_examples":["100020"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210×297mm) |","**Traditional units**: 斤 (jīn), 两 (liǎng), 里 (lǐ) still used in markets and daily life","**Metric primary**: All official and technical contexts use metric","**No imperial**: Imperial units (miles, pounds, Fahrenheit) not used domestically"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+86 1[3-9]\\d \\d{4} \\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","date":"^\\d{4}年\\d{1,2}月\\d{1,2}日$","currency":"^¥\\d{1,3}(,\\d{3})*\\.\\d{2}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^1[3-9]\\d \\d{4} \\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/zh-CN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'sn-ZW'})
SET f.display_name = 'sn-ZW Formatting',
    f.content = 'Formatting rules for sn-ZW',
    f.llm_context = 'sn-ZW: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (Day-Month-Year order, British convention) (gregorian) Time: 24-hour (formal/official), 12-hour (informal/casual) Currency: $ or US$ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (Day-Month-Year order, British convention)","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Ndira","Kukadzi","Kurume","Kubvumbi","Chivabvu","Chikumi","Chikunguru","Nyamavhuvhu","Gunyana","Gumiguru","Mbudzi","Zvita"],"month_abbrev":[],"day_names":["Muvhuro","Chipiri","Chitatu","China","Chishanu","Mugovera","Svondo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour (formal/official), 12-hour (informal/casual)","pattern":"HH:mm (24-hour) or h:mm a (12-hour)","pattern_with_seconds":"HH:mm:ss or h:mm:ss a","time_separator":":","am_indicator":"AM (uppercase) or mangwanani (morning)","pm_indicator":"PM (uppercase) or masikati (afternoon/evening)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"USD (ISO 4217: United States Dollar - official since 2009)","symbol":"$ or US$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[Suburb/Area]\n[City/Town]\nZIMBABWE","postal_code_pattern":"Not widely used (physical addresses and P.O. Boxes preferred)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["14 Samora Machel Avenue\nEastlea\nHarare\nZIMBABWE","P.O. Box 1234\nHarare\nZIMBABWE","25 Doyle Road\nBorrowdale\nHarare\nZIMBABWE","45 Fife Street\nCity Centre\nBulawayo\nZIMBABWE","Stand 5678\nHighfield\nHarare\nZIMBABWE"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","Zimbabwe uses the metric system as the official measurement standard","British imperial units may occasionally appear due to colonial history but metric is standard","Exception: aviation uses feet and nautical miles worldwide","Land area: often measured in hectares (mahekitari); acres may appear in older documents","Agricultural produce: commonly sold by kilogram","Fuel: sold in litres; fuel economy expressed in km/L","Height: officially in metres/centimetres; informally feet/inches may be used"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["25°C","12°C","35°C","37°C"]}',
    f.validation_patterns = '{"time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","phone (international)":"^\\+263\\s?[4-9][0-9]?\\s?[0-9]{3}\\s?[0-9]{3,4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","phone (national)":"^0[4-9][0-9]?\\s?[0-9]{3}\\s?[0-9]{3,4}$","currency":"^\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/sn-ZW.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-JO'})
SET f.display_name = 'ar-JO Formatting',
    f.content = 'Formatting rules for ar-JO',
    f.llm_context = 'ar-JO: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: د.أ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"arabic-indic","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["كانون الثاني","شباط","آذار","نيسان","أيار","حزيران","تموز","آب","أيلول","تشرين الأول","تشرين الثاني","كانون الأول"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"١٥/٠١/٢٠٢٥"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"٣١/١٢/٢٠٢٥"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15 يناير 2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"٢:٣٠ م"},{"input":"14:30","output":"2:30 م"},{"input":"09:00","output":"٩:٠٠ ص"},{"input":"23:59:59","output":"١١:٥٩:٥٩ م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"JOD","symbol":"د.أ","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.500","output":"١٠٫٥٠٠ د.أ"},{"input":"10.500","output":"10.500 د.أ"},{"input":"1234.567","output":"١٬٢٣٤٫٥٦٧ د.أ"},{"input":"0.750","output":"٠٫٧٥٠ د.أ"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building/House Number] [Street Name]\n[District/Area]\n[City] [Postal Code]\nالأردن","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["٤٥ شارع الملك عبدالله الثاني\nجبل عمان - الدوار الثالث\nعمّان ١١١٨١\nالأردن"],"postal_code_examples":["١١١٨١"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°م","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"كم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"كجم","notes":null},{"category":"Volume","unit":"Liters","symbol":"ل","notes":null}],"paper_size":"A4","notes":["| International standard (210 × 297 mm) |","Jordan uses the metric system exclusively for official purposes","Temperature always in Celsius (°م) for weather, medical, cooking","Distance in kilometers (كم) for road signs, maps, GPS","Land area often measured in dunums (دونم) - 1 dunum = 1000 m²","Traditional weight unit: rotl (رطل) sometimes used in markets (approximately 2.5-3 kg)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","٥٠","50","50","12.5"]}',
    f.temperature = '{"format":"{number}°م","default_unit":"celsius","examples":["22°C","٢٢°C","22°C","22°C","-5°C"]}',
    f.validation_patterns = '{"time":"^[١-٩٠1-9]{1,2}:[٠-٥٩0-59]{2}\\s[صم]$","phone (international)":"^\\+٩٦٢\\s\\d{1,2}\\s\\d{3}\\s\\d{4}$","number":"^[؜\\-+]?[٠-٩0-9]{1,3}([٬,][٠-٩0-9]{3})*([٫.][٠-٩0-9]+)?$","currency":"^[٠-٩0-9]{1,3}([٬,][٠-٩0-9]{3})*[٫.][٠-٩0-9]{3}\\sد\\.أ$","phone (national)":"^٠[٧7][٧-٩7-9]\\s\\d{3}\\s\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-JO.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-KW'})
SET f.display_name = 'ar-KW Formatting',
    f.content = 'Formatting rules for ar-KW',
    f.llm_context = 'ar-KW: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: د.ك after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"arabic-indic","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["يناير","فبراير","مارس","أبريل","مايو","يونيو","يوليو","أغسطس","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"١٥/٠١/٢٠٢٥"},{"input":"2025-12-31","output":"٣١/١٢/٢٠٢٥"},{"input":"2025-02-25","output":"٢٥/٠٢/٢٠٢٥"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"٢:٣٠ م"},{"input":"09:00","output":"٩:٠٠ ص"},{"input":"23:59:59","output":"١١:٥٩:٥٩ م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"KWD","symbol":"د.ك","symbol_position":"after","space_between":true,"decimal_places":3,"subunit":null,"correct_examples":[{"input":"10.500","output":"١٠٫٥٠٠ د.ك"},{"input":"1234.567","output":"١٬٢٣٤٫٥٦٧ د.ك"},{"input":"0.999","output":"٠٫٩٩٩ د.ك"},{"input":"0.085","output":"٠٫٠٨٥ د.ك"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building/Block/Street]\n[Area/District]\n[Postal Code] الكويت\nالكويت","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["قطعة ٣، شارع ١٢، منزل ٤٥\nمنطقة السالمية\n٢٢٠٠١ الكويت\nالكويت","مبنى ١٠، شارع الميناء\nمنطقة الأحمدي الصناعية\n٦٤٠٠٠ الكويت\nالكويت"],"postal_code_examples":["١٣٠٠١"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°م","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"كم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"كجم","notes":null},{"category":"Volume","unit":"Liters","symbol":"ل","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","Kuwait uses the metric system exclusively","Temperature always in Celsius (°م) for weather, medical, cooking","Distance in kilometers (كم) for road signs, maps","Exception: Oil and gas industry uses imperial units extensively:","Oil: Barrels (برميل) - Kuwait produces ~2.7 million barrels/day","Depth: Feet for drilling","Pressure: PSI in technical contexts"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","٥٠","12.5"]}',
    f.temperature = '{"format":"{number}°م","default_unit":"celsius","examples":["22°C","٢٢°C","48°C","٤٨°C","-5°C"]}',
    f.validation_patterns = '{"currency":"^[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]{3})?\\sد\\.ك$","number":"^[؜\\-+]?[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]+)?$","phone (international)":"^\\+٩٦٥\\s\\d{4}\\s\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^[١-٩٠]{1,2}:[٠-٥٩]{2}\\s[صم]$","phone (national)":"^[٥٦٩٢]\\d{3}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-KW.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-AU'})
SET f.display_name = 'en-AU Formatting',
    f.content = 'Formatting rules for en-AU',
    f.llm_context = 'en-AU: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour (common speech) / 24-hour (digital/transport) Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour (common speech) / 24-hour (digital/transport)","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"am","pm_indicator":"pm","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"AUD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Unit/Level (optional)\nStreet Number + Street Name + Street Type\nSuburb/Locality + STATE ABBREV + Postcode\nAUSTRALIA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"PO Box","example_addresses":["123 George Street\nSydney NSW 2000\nAUSTRALIA","Unit 5\n456 Collins Street\nMelbourne VIC 3000\nAUSTRALIA","Level 10, Suite 1002\n789 Pitt Street\nSydney NSW 2000\nAUSTRALIA","PO Box 1234\nBrisbane QLD 4001\nAUSTRALIA","42 Main Street\nBallarat VIC 3350\nAUSTRALIA"],"postal_code_examples":["2000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| 210mm x 297mm (not US Letter) |","Australia uses metric system exclusively (adopted 1970s)","Imperial units only for informal/colloquial use (e.g., \"6 foot tall\", \"half a pound\")","Fuel economy: litres per 100 km (L/100km)","Cooking: metric cups (250 mL), tablespoons (20 mL - NOTE: Australian tablespoon is 20 mL, not 15 mL), teaspoons (5 mL)","Pool temperatures: always Celsius","Land area: hectares (ha) for farms, square metres (m2) for residential"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50","10","11.5","4.35"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","abn":"^[0-9]{2} [0-9]{3} [0-9]{3} [0-9]{3}$","phone (international)":"^\\+61 [2-478]? ?[0-9]{4} [0-9]{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (am\\|pm)$","tfn":"^[0-9]{3} [0-9]{3} [0-9]{3}$","medicare":"^[0-9]{4} [0-9]{5} [0-9]$","phone (mobile)":"^04[0-9]{2} [0-9]{3} [0-9]{3}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","phone (landline)":"^0[2378] [0-9]{4} [0-9]{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-AU.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-FR'})
SET f.display_name = 'fr-FR Formatting',
    f.content = 'Formatting rules for fr-FR',
    f.llm_context = 'fr-FR: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (e.g., 15/01/2026) (gregorian) Time: 24-hour (exclusively in France) Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","INSEE (national statistics"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (e.g., 15/01/2026)","short_pattern":"DD/MM/YY (e.g., 15/01/26)","long_pattern":"D MMMM YYYY (e.g., 15 janvier 2026)","full_pattern":"EEEE D MMMM YYYY (e.g., mercredi 15 janvier 2026)","date_separator":"/","month_names":["janvier","février","mars","avril","mai","juin","juillet","août","septembre","octobre","novembre","décembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15/01/2026"},{"input":"2026-12-31","output":"31/12/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"2026-01-15"}]}',
    f.time = '{"system":"24-hour (exclusively in France)","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss (e.g., 14:30:45)","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (Euro)","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":"centime (1 EUR = 100 centimes)","correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1 234,56 €"},{"input":"0.99","output":"0,99 €"},{"input":"15000.00","output":"15 000,00 €"},{"input":"99.00","output":"99,00 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Number] [Street Type] [Street Name]\n[Building/Apt info - optional]\n[Postal Code] [CITY IN CAPS]\n[CEDEX info - optional for business]\nFRANCE","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":"UPPERCASE","street_types":null,"po_box_format":null,"example_addresses":["M. Jean DUPONT\n42 rue de la Paix\n75002 PARIS\nFRANCE","Société EXEMPLE\nDirection Commerciale\n15 avenue des Champs-Élysées\n75008 PARIS CEDEX 08\nFRANCE"],"postal_code_examples":["75001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Fuel","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","France uses metric system exclusively (since 1795)","Imperial units never used in official contexts","Exception: aviation uses feet for altitude (international standard)","Exception: nautical uses knots and nautical miles"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C","0°C"]}',
    f.validation_patterns = '{"currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? €$","nir (social security)":"^[12]\\d{2}(0[1-9]\\|1[0-2])\\d{2}\\d{3}\\d{3}\\d{2}$","siret (establishment)":"^\\d{14}$","siren (company)":"^\\d{9}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^0[1-9]( \\d{2}){4}$","phone (international)":"^\\+33 [1-9]( \\d{2}){4}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-FR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-SN'})
SET f.display_name = 'fr-SN Formatting',
    f.content = 'Formatting rules for fr-SN',
    f.llm_context = 'fr-SN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (e.g., 15/01/2026) (gregorian) Time: 24-hour (French convention) Currency: F CFA after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","ANSD Senegal (national statistics"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (e.g., 15/01/2026)","short_pattern":"DD/MM/YY (e.g., 15/01/26)","long_pattern":"D MMMM YYYY (e.g., 15 janvier 2026)","full_pattern":"EEEE D MMMM YYYY (e.g., mercredi 15 janvier 2026)","date_separator":"/","month_names":["janvier","février","mars","avril","mai","juin","juillet","août","septembre","octobre","novembre","décembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15/01/2026"},{"input":"2026-12-31","output":"31/12/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"2026-01-15"}]}',
    f.time = '{"system":"24-hour (French convention)","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss (e.g., 14:30:45)","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"XOF (Franc CFA BCEAO - West African CFA Franc)","symbol":"F CFA","symbol_position":"after","space_between":true,"decimal_places":1,"subunit":"none in circulation","correct_examples":[{"input":"1500","output":"1 500 F CFA"},{"input":"25000","output":"25 000 F CFA"},{"input":"150000","output":"150 000 F CFA"},{"input":"5000000","output":"5 000 000 F CFA"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Number] [Street Type] [Street Name] (if applicable)\n[Quartier/Neighborhood]\n[BP Number] [City]\nSENEGAL","postal_code_pattern":"BP NNNN (Boite Postale) or NNNNN (5-digit regional codes)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["M. Moussa DIALLO\n25 avenue Léopold Sédar Senghor\nPlateau\nBP 1234 DAKAR\nSENEGAL","Mme Fatou NDIAYE\nBP 5678 DAKAR\nSENEGAL","SARL TERANGA SERVICES\nQuartier Escale\nBP 123 SAINT-LOUIS\nSENEGAL"],"postal_code_examples":["BP 1234"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Fuel","unit":"Liters","symbol":"L","notes":null},{"category":"Hectare","unit":"ha","symbol":"Agriculture","notes":null},{"category":"Kilogram","unit":"kg","symbol":"Market trade","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Senegal uses French metric system exclusively","Local markets may use traditional measures (tas, bol) alongside metric","Imperial units not used in any context"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["35°C","30°C","25°C"]}',
    f.validation_patterns = '{"phone (landline)":"^33 \\d{3} \\d{2} \\d{2}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","cni (national id)":"^\\d{13}$","phone (international)":"^\\+221 (33\\|7[0678]) \\d{3} \\d{2} \\d{2}$","currency":"^\\d{1,3}( \\d{3})* F CFA$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","ninea (company id)":"^\\d{7}[A-Z]\\d[A-Z]$","phone (mobile)":"^7[0678] \\d{3} \\d{2} \\d{2}$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-SN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'pa-IN'})
SET f.display_name = 'pa-IN Formatting',
    f.content = 'Formatting rules for pa-IN',
    f.llm_context = 'pa-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["ਜਨਵਰੀ","ਫ਼ਰਵਰੀ","ਮਾਰਚ","ਅਪ੍ਰੈਲ","ਮਈ","ਜੂਨ","ਜੁਲਾਈ","ਅਗਸਤ","ਸਤੰਬਰ","ਅਕਤੂਬਰ","ਨਵੰਬਰ","ਦਸੰਬਰ"],"month_abbrev":[],"day_names":["ਸੋਮਵਾਰ","ਮੰਗਲਵਾਰ","ਬੁੱਧਵਾਰ","ਵੀਰਵਾਰ","ਸ਼ੁੱਕਰਵਾਰ","ਸ਼ਨਿੱਚਰਵਾਰ","ਐਤਵਾਰ"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ਪੂ.ਦੁ.","pm_indicator":"ਬਾ.ਦੁ.","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ਸ੍ਰੀ ਗੁਰਪ੍ਰੀਤ ਸਿੰਘ\n123, ਮਾਲ ਰੋਡ\nਮਾਡਲ ਟਾਊਨ\nਲੁਧਿਆਣਾ - 141001\nਪੰਜਾਬ\nINDIA"],"postal_code_examples":["141001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","India uses metric system exclusively","Exceptions: Some traditional measurements still used in agriculture (e.g., ਕਿੱਲਾ/killa, ਮਰਲਾ/marla for land in Punjab)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM\\|ਪੂ\\.ਦੁ\\.\\|ਬਾ\\.ਦੁ\\.)$","phone (national)":"^\\d{5}\\s\\d{5}$","phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/pa-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'cs-CZ'})
SET f.display_name = 'cs-CZ Formatting',
    f.content = 'Formatting rules for cs-CZ',
    f.llm_context = 'cs-CZ: Numbers use \'.\' decimal, \',\' thousands. Dates: d. M. yyyy (day-first, period separators) (gregorian) Time: 24-hour Currency: Kč after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d. M. yyyy (day-first, period separators)","short_pattern":"d. M. yy","long_pattern":"d. MMMM yyyy","full_pattern":null,"date_separator":".","month_names":["leden","únor","březen","duben","květen","červen","červenec","srpen","září","říjen","listopad","prosinec"],"month_abbrev":[],"day_names":["pondělí","úterý","středa","čtvrtek","pátek","sobota","neděle"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15. 1. 2025"},{"input":"2025-12-31","output":"31. 12. 2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"H:mm (no leading zero for single-digit hours preferred in casual use, but HH:mm in formal contexts)","pattern_with_seconds":"H:mm:ss (or HH:mm:ss)","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"9:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"CZK (Czech koruna)","symbol":"Kč","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 Kč"},{"input":"1234.56","output":"1 234,56 Kč"},{"input":"0.99","output":"0,99 Kč"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number/Orientation Number]\n[Postal Code] [City]\nCZECH REPUBLIC","postal_code_pattern":"NNN NN (5 digits: 3 digits, space, 2 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Václavské náměstí 846/1\n110 00 Praha 1\nCZECH REPUBLIC"],"postal_code_examples":["110 00"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"l or L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210 × 297 mm) |","Metric system exclusively - no imperial units in standard use","Exception: Some technical fields (aviation, computing) may use international standards"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"date":"^\\d{1,2}\\.\\s?\\d{1,2}\\.\\s?\\d{4}$","phone (national)":"^\\d{3}\\s\\d{3}\\s\\d{3}$","number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","time":"^\\d{1,2}:\\d{2}(:\\d{2})?$","phone (international)":"^\\+420\\s\\d{3}\\s\\d{3}\\s\\d{3}$","currency":"^\\d{1,3}(\\s\\d{3})*(,\\d{2})?\\s?Kč$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/cs-CZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-CL'})
SET f.display_name = 'es-CL Formatting',
    f.content = 'Formatting rules for es-CL',
    f.llm_context = 'es-CL: Numbers use \'.\' decimal, \',\' thousands. Dates: DD-MM-YYYY (gregorian) Time: 12-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD-MM-YYYY","short_pattern":"DD-MM-YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"-","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miércoles","jueves","viernes","sábado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15-01-2025"},{"input":"2025-12-31","output":"31-12-2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a. m.","pm_indicator":"p. m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p. m."},{"input":"09:00","output":"9:00 a. m."},{"input":"23:59:59","output":"11:59:59 p. m."},{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"CLP","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10500","output":"$10.500"},{"input":"1234567","output":"$1.234.567"},{"input":"990","output":"$990"},{"input":"1500000","output":"$1.500.000"},{"input":"99999","output":"$99.999"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number] [Apartment/Office]\n[Comuna]\n[City], [Region]\n[Postal Code]\nCHILE","postal_code_pattern":"NNNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Av. Libertador Bernardo O\'Higgins 1234 Dpto. 56\nSantiago Centro\nSantiago, Región Metropolitana\n8320000\nCHILE"],"postal_code_examples":["7500000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US letter size (8.5\" × 11\") commonly used in business |","Metric system is official and standard throughout Chile","Paper size commonly uses US Letter format (21.59 cm × 27.94 cm) in business contexts, A4 in academic/government","Speed limits shown in km/h","Fuel sold by liter at gas stations"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+56\\s\\d\\s\\d{4}\\s\\d{4}$","date":"^\\d{2}-\\d{2}-\\d{4}$","time":"^\\d{1,2}:\\d{2}(:\\d{2})?(\\s[ap]\\.\\s?m\\.)?$","phone (national)":"^\\d\\s\\d{4}\\s\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","currency":"^\\$\\d{1,3}(\\.\\d{3})*$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-CL.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'nl-NL'})
SET f.display_name = 'nl-NL Formatting',
    f.content = 'Formatting rules for nl-NL',
    f.llm_context = 'nl-NL: Numbers use \'.\' decimal, \',\' thousands. Dates: DD-MM-YYYY (gregorian) Time: 24-hour Currency: EUR before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD-MM-YYYY","short_pattern":"DD-MM-YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"-","month_names":["januari","februari","maart","april","mei","juni","juli","augustus","september","oktober","november","december"],"month_abbrev":[],"day_names":["maandag","dinsdag","woensdag","donderdag","vrijdag","zaterdag","zondag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":null,"pm_indicator":null,"prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"EUR","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Straatnaam] [Huisnummer][Toevoeging]\n[Postcode] [Plaatsnaam]\nNETHERLANDS","postal_code_pattern":"NNNN AA","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Kalverstraat 92\n1012 PH Amsterdam\nNETHERLANDS","Coolsingel 40\n3011 AD Rotterdam\nNETHERLANDS","Herengracht 182 hs\n1016 BR Amsterdam\nNETHERLANDS","Lange Voorhout 8\n2514 ED Den Haag\nNETHERLANDS"],"postal_code_examples":["1012 PH"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"degC","notes":null},{"category":"Distance","unit":"Kilometer","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogram","symbol":"kg","notes":null},{"category":"Volume","unit":"Liter","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm; European standard |","Metric system is universal in the Netherlands (since 1816)","\"Pond\" (500 gram) still used informally at markets: \"een pondje kaas\"","Imperial units never used except in international contexts","Aviation uses feet, nautical miles (international ICAO standard)","Bicycle distances often in km: \"het is maar 5 km fietsen\""]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["21","9","0"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"bsn":"^\\d{9}$","phone (mobile)":"^06\\s?\\d{4}\\s?\\d{4}$","date (standard)":"^\\d{2}-\\d{2}-\\d{4}$","kenteken (plates)":"^[A-Z0-9]{2,3}-[A-Z0-9]{2,3}-[A-Z0-9]{2}$","date (short)":"^\\d{2}-\\d{2}-\\d{2}$","time (hh:mm)":"^([01]\\d\\|2[0-3]):[0-5]\\d$","phone (landline)":"^0[1-9]\\d{1,2}\\s?\\d{3}\\s?\\d{4}$","currency":"^EUR\\s\\d{1,3}(\\.\\d{3})*(,\\d{2})$","phone (intl)":"^\\+31\\s?[1-9]\\d?\\s?\\d{4}\\s?\\d{4}$","time (hh:mm:ss)":"^([01]\\d\\|2[0-3]):[0-5]\\d:[0-5]\\d$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/nl-NL.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-HN'})
SET f.display_name = 'es-HN Formatting',
    f.content = 'Formatting rules for es-HN',
    f.llm_context = 'es-HN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: L before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p.m."},{"input":"09:00","output":"9:00 a.m."},{"input":"23:59:59","output":"11:59:59 p.m."},{"input":"12:00","output":"12:00 p.m."},{"input":"00:00","output":"12:00 a.m."},{"input":"06:00","output":"6:00 a.m."}],"incorrect_examples":[]}',
    f.currency = '{"code":"HNL","symbol":"L","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"L10.50"},{"input":"1234.56","output":"L1,234.56"},{"input":"0.99","output":"L0.99"},{"input":"15000.00","output":"L15,000.00"},{"input":"999999.99","output":"L999,999.99"},{"input":"25.00","output":"L25.00"},{"input":"500.00","output":"L500.00"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name/Type] [Number]\n[Barrio/Colonia], [City]\n[Departamento]\n[Postal Code]\nHONDURAS","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Avenida Juan Pablo II, Casa 456\nColonia Palmira, Tegucigalpa\nFrancisco Morazan\n11101\nHONDURAS","Calle Principal, Edificio Corporativo\nBo. Guamilito, San Pedro Sula\nCortes\n21102\nHONDURAS"],"postal_code_examples":["11101"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US Letter (8.5\" x 11\") - US influence |","Metric system is official in Honduras","Exception: \"Libras\" (pounds) extremely common in markets for produce, meat","Gasoline often sold by gallon at gas stations (US influence)","Paper size uses US Letter format due to regional US business influence","Speed limits shown in km/h on highways","Land often measured in \"manzanas\" (traditional unit, approx 0.7 hectares)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^\\d{4}-\\d{4}$","currency":"^L\\d{1,3}(,\\d{3})*\\.\\d{2}$","time":"^\\d{1,2}:\\d{2}(:\\d{2})?\\s[ap]\\.m\\.$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (international)":"^\\+504\\s\\d{4}\\s\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-HN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'bg-BG'})
SET f.display_name = 'bg-BG Formatting',
    f.content = 'Formatting rules for bg-BG',
    f.llm_context = 'bg-BG: Numbers use \'.\' decimal, \',\' thousands. Dates: D.MM.YYYY (period separator, no leading zero on day) (gregorian) Time: 24-hour Currency: лв. after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D.MM.YYYY (period separator, no leading zero on day)","short_pattern":"D.MM.YY","long_pattern":"D MMMM YYYY (e.g., \"15 януари 2025 г.\")","full_pattern":null,"date_separator":".","month_names":["януари","февруари","март","април","май","юни","юли","август","септември","октомври","ноември","декември"],"month_abbrev":[],"day_names":["понеделник","вторник","сряда","четвъртък","петък","събота","неделя"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"H:mm (24-hour format, no leading zero on hour)","pattern_with_seconds":"H:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"9:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"BGN (Bulgarian Lev)","symbol":"лв.","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 лв."},{"input":"1234.56","output":"1 234,56 лв."},{"input":"0.99","output":"0,99 лв."}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name] [Number], [Apartment]\n[Postal Code] [City]\nBULGARIA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ул. Витоша 89, ет. 3, ап. 12\n1000 София\nBULGARIA"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210x297mm) |","Bulgaria uses metric system exclusively","Imperial units not used in everyday contexts"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"date":"^\\d{1,2}\\.\\d{2}\\.\\d{4}$","phone (national)":"^0\\d{2,3} \\d{3} \\d{3,4}$","phone (international)":"^\\+359 \\d{2,3} \\d{3} \\d{3,4}$","currency":"^\\d{1,3}( \\d{3})*,\\d{2} лв\\.$","number":"^-?\\d{1,3}( \\d{3})*,\\d+$","time":"^([0-9]|1\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/bg-BG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'sd-PK'})
SET f.display_name = 'sd-PK Formatting',
    f.content = 'Formatting rules for sd-PK',
    f.llm_context = 'sd-PK: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: Rs before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["جنوري","فيبروري","مارچ","اپريل","مئي","جون","جولاءِ","آگسٽ","سيپٽمبر","آڪٽوبر","نومبر","ڊسمبر"],"month_abbrev":[],"day_names":["سومر","اڱارو","اربع","خميس","جمعو","ڇنڇر","آچر"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"صبح","pm_indicator":"شام","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PKR","symbol":"Rs","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[House/Building Number], [Street Name]\n[Sector/Block/Neighborhood]\n[City] - [Postal Code]\nPAKISTAN","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123، شاهراهه فيصل\nڪليفٽن بلاڪ 5\nڪراچي - 75600\nپاڪستان"],"postal_code_examples":["75600"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km / ڪلوميٽر","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg / ڪلوگرام","notes":null},{"category":"Volume","unit":"Liters","symbol":"L / لٽر","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","Pakistan officially uses the metric system","Temperature always in Celsius (°C) for weather, medical, cooking","Distance in kilometers (km) for road signs, maps","Traditional units still used locally: seer (approximately 933g), maund (approximately 37.3kg), but metric is official","Exception: Some imported goods may show imperial units alongside metric"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","50","12.5","12.5"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","22°C","-5°C","-5°C"]}',
    f.validation_patterns = '{"phone (national)":"^0[3][0-4]\\d-\\d{7}$","phone (international)":"^\\+92\\s\\d{3}\\s\\d{7}$","currency":"^Rs\\d{1,3}(,\\d{2,3})*(\\.\\d{2})?$","number":"^-?[\\d,]+(\\.\\d+)?$","time":"^(1[0-2]\\|[1-9]):[0-5]\\d\\s(صبح\\|شام)$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/sd-PK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-GH'})
SET f.display_name = 'en-GH Formatting',
    f.content = 'Formatting rules for en-GH',
    f.llm_context = 'en-GH: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: GH₵ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"},{"input":"2025-03-06","output":"06/03/2025"},{"input":"2025-07-01","output":"01/07/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-01-15","output":"January 15, 2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"23:59:59","output":"11:59:59 PM"},{"input":"00:00","output":"12:00 AM"},{"input":"12:00","output":"12:00 PM"},{"input":"17:45","output":"5:45 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"GHS","symbol":"GH₵","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"GH₵10.50"},{"input":"1234.56","output":"GH₵1,234.56"},{"input":"0.99","output":"GH₵0.99"},{"input":"1000000","output":"GH₵1,000,000.00"},{"input":"5.00","output":"GH₵5.00"},{"input":"99.95","output":"GH₵99.95"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[Neighbourhood/Area]\n[City], [Region]\n[Digital Address] (optional)\nGHANA","postal_code_pattern":"Ghana Digital Address (GA-XXX-XXXX format)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["15 Independence Avenue\nOsu\nAccra, Greater Accra Region\nGA-183-6548\nGHANA","P.O. Box 1234\nAccra\nGHANA","45 Cantonments Road\nCantonments\nAccra, Greater Accra Region\nGHANA"],"postal_code_examples":["GA-183-6548"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","Ghana uses the metric system as the official measurement standard","British imperial units may occasionally appear in informal contexts due to colonial history","Exception: aviation uses feet and nautical miles worldwide","Land area: often measured in hectares or acres (acres from colonial era)","Cooking: metric measurements (grams, millilitres) preferred","Height: sometimes expressed in feet/inches informally, but metres/centimetres officially"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["28°C","22°C","35°C","37°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+233 [2-5][0-9] [0-9]{3} [0-9]{4}$","phone (national)":"^0[2-5][0-9] [0-9]{3} [0-9]{4}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^GH₵[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-GH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-PH'})
SET f.display_name = 'en-PH Formatting',
    f.content = 'Formatting rules for en-PH',
    f.llm_context = 'en-PH: Numbers use \'.\' decimal, \',\' thousands. Dates: MM/DD/YYYY (gregorian) Time: 12-hour Currency: P (peso sign) before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"MM/DD/YYYY","short_pattern":"M/D/YY","long_pattern":"MMMM D, YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PHP","symbol":"P (peso sign)","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[House/Building Number] [Street Name]\n[Barangay], [City/Municipality]\n[Province] [Postal Code]\nPHILIPPINES","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Juan dela Cruz\n123 Ayala Avenue, Brgy. San Lorenzo\nMakati City\nMetro Manila 1226\nPHILIPPINES","Maria Santos\n456 Rizal Street, Brgy. Poblacion\nCebu City\nCebu 6000\nPHILIPPINES","Anna Reyes\nUnit 1201, One Bonifacio High Street\n5th Avenue, Brgy. Bonifacio Global City\nTaguig City\nMetro Manila 1634\nPHILIPPINES"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| ISO 216 standard, US Letter also used in business |","The Philippines officially uses the metric system for all government and commercial purposes","US customary units (feet, inches, pounds) are commonly understood due to historical American influence","Height is often expressed in feet and inches colloquially (e.g., \"5\'6\")","Screen sizes use inches (e.g., \"55-inch TV\")","Land area may use square meters or hectares (not acres)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["30°C","25°C","37°C","100°C"]}',
    f.validation_patterns = '{"time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9](:[0-5][0-9])? (AM\\|PM)$","phone (international)":"^\\+63 \\d{3} \\d{3} \\d{4}$","currency":"^P\\d{1,3}(,\\d{3})*\\.\\d{2}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","date":"^(0[1-9]\\|1[0-2])\\/(0[1-9]\\|[12][0-9]\\|3[01])\\/\\d{4}$","phone (national)":"^0\\d{3} \\d{3} \\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-PH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'sl-SI'})
SET f.display_name = 'sl-SI Formatting',
    f.content = 'Formatting rules for sl-SI',
    f.llm_context = 'sl-SI: Numbers use \'.\' decimal, \',\' thousands. Dates: d. M. yyyy (day-first, period + space separators) (gregorian) Time: 24-hour Currency: EUR (code used as symbol in Slovenia) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d. M. yyyy (day-first, period + space separators)","short_pattern":"d. M. yy","long_pattern":"d. MMMM yyyy","full_pattern":null,"date_separator":".","month_names":["januar","februar","marec","april","maj","junij","julij","avgust","september","oktober","november","december"],"month_abbrev":[],"day_names":["ponedeljek","torek","sreda","cetrtek","petek","sobota","nedelja"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15. 1. 2025"},{"input":"2025-12-31","output":"31. 12. 2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"}]}',
    f.time = '{"system":"24-hour","pattern":"H:mm (single-digit hour without leading zero common, HH:mm in formal contexts)","pattern_with_seconds":"H:mm:ss (or HH:mm:ss)","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"9:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (Euro)","symbol":"EUR (code used as symbol in Slovenia)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 EUR"},{"input":"1234.56","output":"1.234,56 EUR"},{"input":"0.99","output":"0,99 EUR"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nSLOVENIA","postal_code_pattern":"NNNN (4 digits, no separators)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Presernov trg 1\n1000 Ljubljana\nSLOVENIA"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"l or L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210 x 297 mm) |","Metric system exclusively - no imperial units in standard use","Exception: Aviation uses feet and nautical miles per international standards"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","time":"^\\d{1,2}:\\d{2}(:\\d{2})?$","phone (international)":"^\\+386\\s\\d{1,2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","date":"^\\d{1,2}\\.\\s?\\d{1,2}\\.\\s?\\d{4}$","phone (national)":"^0\\d{1,2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","currency":"^\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\s?EUR$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/sl-SI.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-HK'})
SET f.display_name = 'en-HK Formatting',
    f.content = 'Formatting rules for en-HK',
    f.llm_context = 'en-HK: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour (colloquial), 24-hour (official/transport) Currency: HK$ or $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"},{"input":"2025-07-01","output":"01/07/2025"},{"input":"2025-10-01","output":"01/10/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"January 15, 2025"},{"input":"2025-01-15","output":"15.01.2025"}]}',
    f.time = '{"system":"12-hour (colloquial), 24-hour (official/transport)","pattern":"h:mm a (12-hour) / HH:mm (24-hour)","pattern_with_seconds":"h:mm:ss a / HH:mm:ss","time_separator":":","am_indicator":"am / AM","pm_indicator":"pm / PM","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 pm"},{"input":"09:00","output":"9:00 am"},{"input":"23:59:59","output":"11:59:59 pm"},{"input":"00:00","output":"12:00 am"},{"input":"12:00","output":"12:00 pm"},{"input":"17:45","output":"5:45 pm"}],"incorrect_examples":[]}',
    f.currency = '{"code":"HKD","symbol":"HK$ or $","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"HK$10.50"},{"input":"1234.56","output":"HK$1,234.56"},{"input":"0.99","output":"HK$0.99"},{"input":"1000000","output":"HK$1,000,000.00"},{"input":"5.00","output":"HK$5.00"},{"input":"99.95","output":"HK$99.95"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Flat/Room], [Floor], [Building Name]\n[Number] [Street Name]\n[District]\nHONG KONG","postal_code_pattern":"None (Hong Kong does not use postal codes for general mail)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Flat A, 12/F, Tower 1, Pacific Place\n88 Queensway\nAdmiralty\nHONG KONG","Room 1201, 12/F, Block A\nMei Foo Sun Chuen\nLai Chi Kok\nHONG KONG","Suite 2501, 25/F\nOne International Finance Centre\n1 Harbour View Street, Central\nHONG KONG"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| 210 × 297 mm (not Letter) |","Hong Kong uses the metric system as the official measurement system","Traditional Chinese units sometimes used colloquially (catty/tael for weight at wet markets)","Exception: aviation uses feet and nautical miles worldwide","Property: square feet (sq ft) commonly used for real estate (legacy from British rule)","British spelling used: metres, litres, kilometres (not meters, liters, kilometers)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","0°C","37°C","100°C","35°C"]}',
    f.validation_patterns = '{"time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (am\\|pm\\|AM\\|PM)$","time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","phone (international)":"^\\+852 [2356789][0-9]{3} [0-9]{4}$","phone (national)":"^[2356789][0-9]{3} [0-9]{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^(HK)?\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-HK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-MU'})
SET f.display_name = 'en-MU Formatting',
    f.content = 'Formatting rules for en-MU',
    f.llm_context = 'en-MU: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: Rs before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"am","pm_indicator":"pm","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"MUR","symbol":"Rs","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Recipient Name\nBuilding/House Number, Street Name\nVillage/Town\nDistrict\nMAURITIUS","postal_code_pattern":"","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Raj Doorgakant\n25 Royal Road\nCurepipe\nPlaines Wilhems\nMAURITIUS","Marie-Claire Lafleur\nLa Croisette Shopping Mall, Unit 12\nGrand Baie\nRiviere du Rempart\nMAURITIUS","Sun Resorts Ltd\nLong Beach Hotel\nBelle Mare\nFlacq\nMAURITIUS"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Mauritius uses the metric system exclusively (former British colony, adopted metric post-independence)","Road signs display distances in kilometres (e.g., Port Louis 15 km)","Speed limits in km/h: 40 km/h (urban), 80 km/h (rural), 110 km/h (motorway)","Fuel efficiency: L/100km or km/L (never MPG)","Sugar cane yields measured in tonnes per hectare (historic sugar industry)","Land area: arpent (traditional) still used alongside hectares (1 arpent = 0.4221 hectares)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","phone (international)":"^\\+230\\s?[2-6][0-9]{2}\\s?[0-9]{4}$","phone (landline)":"^[246][0-9]{2}\\s?[0-9]{4}$","phone (mobile)":"^5[2479][0-9]{2}\\s?[0-9]{4}$","time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","number":"^-?[\\d,]+(\\.\\d+)?$","currency":"^Rs\\s[\\d,]+(\\.\\d{2})?$","time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(am\\|pm)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-MU.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'et-EE'})
SET f.display_name = 'et-EE Formatting',
    f.content = 'Formatting rules for et-EE',
    f.llm_context = 'et-EE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (e.g., 15.01.2025) (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (e.g., 15.01.2025)","short_pattern":"DD.MM.YY (e.g., 15.01.25)","long_pattern":"D. MMMM YYYY (e.g., 15. jaanuar 2025)","full_pattern":null,"date_separator":".","month_names":["jaanuar","veebruar","marts","aprill","mai","juuni","juuli","august","september","oktoober","november","detsember"],"month_abbrev":[],"day_names":["esmaspäev","teisipäev","kolmapäev","neljapäev","reede","laupäev","pühapäev"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss (e.g., 14:30:00)","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1 234,56 €"},{"input":"0.99","output":"0,99 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\n[County] (optional)\nESTONIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Narva maantee 7\n10117 Tallinn\nESTONIA"],"postal_code_examples":["10117"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard European |","Estonia uses metric system exclusively","Temperature always in Celsius","Road distances in kilometers","Fuel consumption in liters per 100 km (l/100 km)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (international)":"^\\+372\\s\\d{3,4}\\s\\d{4}$","currency":"^-?\\d{1,3}(\\s\\d{3})*(,\\d{2})?\\s€$","number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","phone (national)":"^\\d{3,4}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/et-EE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-SV'})
SET f.display_name = 'es-SV Formatting',
    f.content = 'Formatting rules for es-SV',
    f.llm_context = 'es-SV: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"USD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name] [Number], [Building/Local]\n[Colonia/Barrio/Residencial]\n[Municipality], [Department]\nCP [Postal Code]\nEL SALVADOR","postal_code_pattern":"","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Boulevard El Hipodromo 234, Local 5\nCol. San Benito\nSan Salvador, San Salvador\nCP 1101\nEL SALVADOR","Pasaje Los Almendros, Casa 15\nRes. Altamira\nAntiguo Cuscatlan, La Libertad\nCP 1501\nEL SALVADOR","5a Calle Poniente 123\nBarrio El Centro\nSanta Ana, Santa Ana\nCP 2201\nEL SALVADOR"],"postal_code_examples":["CP 1101"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Fuel","unit":"Gallons","symbol":"gal","notes":null},{"category":"Unit","unit":"Equivalent","symbol":"Usage","notes":null}],"paper_size":"Letter","notes":["| US letter (8.5\" x 11\") standard |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","dui":"^\\d{8}-\\d$","phone (national)":"^\\d{4}-\\d{4}$","currency":"^\\$\\d{1,3}(,\\d{3})*\\.\\d{2}$","nit":"^\\d{4}-\\d{6}-\\d{3}-\\d$","time":"^\\d{1,2}:\\d{2}(:\\d{2})?\\s?(a\\.m\\.|p\\.m\\.)?$","phone (international)":"^\\+503 \\d{4} \\d{4}$","isss":"^\\d{9}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-SV.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'pt-CH'})
SET f.display_name = 'pt-CH Formatting',
    f.content = 'Formatting rules for pt-CH',
    f.llm_context = 'pt-CH: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day, month, year with period separators) (gregorian) Time: 24-hour Currency: CHF or Fr. before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day, month, year with period separators)","short_pattern":"DD.MM.YY (two-digit year)","long_pattern":"D \'de\' MMMM \'de\' YYYY (e.g., \"15 de janeiro de 2025\")","full_pattern":null,"date_separator":".","month_names":["janeiro","fevereiro","março","abril","maio","junho","julho","agosto","setembro","outubro","novembro","dezembro"],"month_abbrev":[],"day_names":["segunda-feira","terça-feira","quarta-feira","quinta-feira","sexta-feira","sábado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format with leading zeros)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"CHF (ISO 4217)","symbol":"CHF or Fr.","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"CHF 10.50"},{"input":"1234.56","output":"CHF 1\'234.55"},{"input":"0.99","output":"CHF 1.00"},{"input":"19.95","output":"CHF 19.95"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[First Name] [Last Name]\n[Street Name] [House Number]\n[Postal Code] [City]\nSUÍÇA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Maria Santos\nBahnhofstrasse 42\n8001 Zürich\nSUÍÇA"],"postal_code_examples":["8001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L or l","notes":null}],"paper_size":"A4","notes":["| Standard: 210 x 297 mm |","Metric system used exclusively for all measurements","No imperial units in common use","Fuel consumption expressed as liters per 100 km (L/100 km)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C","36.5°C"]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\'\\d{3})*(\\.\\d+)?$","currency":"^CHF\\s\\d{1,3}(\'\\d{3})*(\\.\\d{2})?$","phone (national)":"^0\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","phone (international)":"^\\+41\\s\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/pt-CH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-PR'})
SET f.display_name = 'es-PR Formatting',
    f.content = 'Formatting rules for es-PR',
    f.llm_context = 'es-PR: Numbers use \'.\' decimal, \',\' thousands. Dates: MM/DD/YYYY (gregorian) Time: 12-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"MM/DD/YYYY","short_pattern":"MM/DD/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miércoles","jueves","viernes","sábado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-12-31","output":"12/31/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"01-15-2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p.m."},{"input":"09:00","output":"9:00 a.m."},{"input":"23:59:59","output":"11:59:59 p.m."},{"input":"12:00","output":"12:00 p.m."},{"input":"00:00","output":"12:00 a.m."}],"incorrect_examples":[]}',
    f.currency = '{"code":"USD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"$10.50"},{"input":"1234.56","output":"$1,234.56"},{"input":"0.99","output":"$0.99"},{"input":"1500.00","output":"$1,500.00"},{"input":"999999.99","output":"$999,999.99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name] [Unit]\n[City], PR [ZIP Code]\nUNITED STATES","postal_code_pattern":"NNNNN or NNNNN-NNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["456 Ave. Ashford Suite 301\nSan Juan, PR 00907\nUNITED STATES"],"postal_code_examples":["00901"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Fahrenheit","symbol":"°F","notes":null},{"category":"Distance","unit":"Miles","symbol":"mi","notes":null},{"category":"Weight","unit":"Pounds","symbol":"lb","notes":null},{"category":"Volume","unit":"Gallons","symbol":"gal","notes":null}],"paper_size":"Letter","notes":["| US letter size (8.5\" x 11\") standard |","US customary system is standard throughout Puerto Rico as US territory","Exception: Some metric measurements appear on imported products and highway signs","Body weight commonly given in pounds (libras)","Gas stations sell fuel exclusively by gallon","Speed limits shown in miles per hour (mph)","Land area measured in cuerdas (traditional unit, 1 cuerda = 0.97 acres)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\$\\d{1,3}(,\\d{3})*\\.\\d{2}$","time":"^\\d{1,2}:\\d{2}(:\\d{2})? [ap]\\.m\\.$","phone (international)":"^\\+1 \\(\\d{3}\\) \\d{3}-\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^\\(\\d{3}\\) \\d{3}-\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-PR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ur-PK'})
SET f.display_name = 'ur-PK Formatting',
    f.content = 'Formatting rules for ur-PK',
    f.llm_context = 'ur-PK: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: Rs before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["جنوری","فروری","مارچ","اپریل","مئی","جون","جولائی","اگست","ستمبر","اکتوبر","نومبر","دسمبر"],"month_abbrev":[],"day_names":["پیر","منگل","بدھ","جمعرات","جمعہ","ہفتہ","اتوار"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"صبح","pm_indicator":"شام","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 شام"},{"input":"09:00","output":"9:00 صبح"},{"input":"23:59:59","output":"11:59:59 رات"}],"incorrect_examples":[]}',
    f.currency = '{"code":"PKR","symbol":"Rs","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"Rs10.50"},{"input":"1234.56","output":"Rs1,234.56"},{"input":"0.99","output":"Rs0.99"},{"input":"100000","output":"Rs1,00,000"},{"input":"10000000","output":"Rs1,00,00,000"},{"input":"750000.50","output":"Rs7,50,000.50"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Mohalla Name]\n[Sector/Area], [City] - [Postal Code]\n[Province]\nPAKISTAN","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["جناب احمد علی\n123، گلی نمبر 5\nجی-10/4، اسلام آباد - 44000\nوفاقی دارالحکومت\nPAKISTAN","Mr. Ahmed Ali\n123, Street No. 5\nG-10/4, Islamabad - 44000\nFederal Capital\nPAKISTAN"],"postal_code_examples":["44000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm (international standard) |","Pakistan uses the metric system officially","Temperature always in Celsius for weather, medical, cooking","Distance in kilometers for road signs, maps","Traditional units still used locally: tola (gold weight ~11.66g), maund (weight ~37.3kg), marla/kanal (land area)","Exception: Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","50","12.5","12.5"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","22°C","-5°C","-5°C"]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^\\d{1,2}:\\d{2}\\s?(صبح\\|شام\\|دوپہر\\|رات\\|AM\\|PM)$","phone (international)":"^\\+92\\s\\d{3}\\s\\d{7}$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","currency":"^Rs\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","phone (national)":"^0\\d{3}-\\d{7}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ur-PK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-ZM'})
SET f.display_name = 'en-ZM Formatting',
    f.content = 'Formatting rules for en-ZM',
    f.llm_context = 'en-ZM: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: K before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"ZMW","symbol":"K","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Plot/House Number] [Street Name]\n[Area/Township/Compound]\n[City/Town]\n[Postal Code]\nZAMBIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Plot 456 Independence Avenue\nLongacres\nLusaka\n10101\nZAMBIA","P.O. Box 31958\nLusaka\n10101\nZAMBIA","House 25\nKalingalinga Compound\nLusaka\n10101\nZAMBIA","Plot 12 Chingola Road\nNkana East\nKitwe\n10101\nZAMBIA"],"postal_code_examples":["10101"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","Zambia uses the metric system as the official measurement standard","British imperial units may occasionally appear in informal contexts due to colonial history","Exception: aviation uses feet and nautical miles worldwide","Land area: often measured in hectares; some use acres informally (colonial legacy)","Agricultural produce: commonly sold by kilogram; informal markets may use local measures (90kg bags, 50kg bags)","Height: sometimes expressed in feet/inches informally, but metres/centimetres officially","Fuel: sold in litres; fuel economy expressed in km/L"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["25°C","10°C","32°C","37°C"]}',
    f.validation_patterns = '{"date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","phone (national)":"^0[29][0-9] [0-9]{3} [0-9]{4}$","phone (international)":"^\\+260 [29][0-9] [0-9]{3} [0-9]{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^K [0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","time":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-ZM.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-OM'})
SET f.display_name = 'ar-OM Formatting',
    f.content = 'Formatting rules for ar-OM',
    f.llm_context = 'ar-OM: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: ر.ع. after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"arabic-indic","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["يناير","فبراير","مارس","أبريل","مايو","يونيو","يوليو","أغسطس","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"١٥/٠١/٢٠٢٥"},{"input":"2025-12-31","output":"٣١/١٢/٢٠٢٥"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"٢:٣٠ م"},{"input":"09:00","output":"٩:٠٠ ص"},{"input":"23:59:59","output":"١١:٥٩:٥٩ م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"OMR","symbol":"ر.ع.","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.500","output":"١٠٫٥٠٠ ر.ع."},{"input":"1234.567","output":"١٬٢٣٤٫٥٦٧ ر.ع."},{"input":"0.999","output":"٠٫٩٩٩ ر.ع."},{"input":"1.000","output":"١٫٠٠٠ ر.ع."}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building Name/Number] [Street Name]\n[District/Area]\n[City/Governorate]\n[Postal Code]\nسلطنة عُمان","postal_code_pattern":"NNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"ص.ب.","example_addresses":["مبنى ٤٥٦، شارع السلطان قابوس\nحي الخوير\nمسقط ١١١\nسلطنة عُمان"],"postal_code_examples":["١١١"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°م","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"كم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"كجم","notes":null},{"category":"Volume","unit":"Liters","symbol":"ل","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","Oman uses the metric system exclusively for official purposes","Temperature always in Celsius (°م) for weather, medical, cooking","Distance in kilometers (كم) for road signs, maps","Exception: Oil and gas industry uses imperial units (barrels, feet) in technical contexts","Traditional units: Some traditional markets may use local units for specific goods"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","٥٠","12.5"]}',
    f.temperature = '{"format":"{number}°م","default_unit":"celsius","examples":["22°C","٢٢°C","48°C","٤٨°C"]}',
    f.validation_patterns = '{"currency":"^[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]{3})?\\sر\\.ع\\.$","phone (national)":"^[٩٧٢]\\d{3}\\s\\d{4}$","number":"^[؜\\-+]?[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]+)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^[١-٩٠]{1,2}:[٠-٥٩]{2}\\s[صم]$","phone (international)":"^\\+٩٦٨\\s\\d{4}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-OM.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ru-MD'})
SET f.display_name = 'ru-MD Formatting',
    f.content = 'Formatting rules for ru-MD',
    f.llm_context = 'ru-MD: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: лей (lei) / L after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY г.","full_pattern":null,"date_separator":".","month_names":["января","февраля","марта","апреля","мая","июня","июля","августа","сентября","октября","ноября","декабря"],"month_abbrev":[],"day_names":["понедельник","вторник","среда","четверг","пятница","суббота","воскресенье"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15.01.2026"},{"input":"2026-08-27","output":"27.08.2026"},{"input":"2026-03-01","output":"01.03.2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"15/01/2026"},{"input":"2026-01-15","output":"2026-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"MDL (Moldovan Leu / Молдавский лей)","symbol":"лей (lei) / L","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":"бан (ban), plural: бань (bani)","correct_examples":[{"input":"10.50","output":"10,50 лей"},{"input":"1234.56","output":"1 234,56 лей"},{"input":"0.99","output":"0,99 лей"},{"input":"6.00","output":"6,00 лей"},{"input":"150.00","output":"150,00 лей"},{"input":"800.00","output":"800,00 лей"},{"input":"0.50","output":"50 бань"},{"input":"0.01","output":"1 бан"},{"input":"0.25","output":"25 бань"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Улица] [ул./бд./пер.] [Name], д. [Building], кв. [Apartment]\nMD-[Postal Code], [City/г./мун./с.]\n[Raion/район] (if applicable)\nМОЛДОВА / MOLDOVA","postal_code_pattern":"","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ул. Штефана чел Маре, д. 75, кв. 12\nMD-2001, мун. Кишинёв\nМОЛДОВА","ул. Индепенденцей, д. 45, кв. 8\nMD-3100, мун. Бельцы\nМОЛДОВА","ул. Главная, д. 23\nMD-6516, с. Костулены\nУнгенский район\nМОЛДОВА","ул. Ленина, д. 56\nMD-4300, г. Комрат\nАТО Гагаузия\nМОЛДОВА"],"postal_code_examples":["MD-2001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null},{"category":"Unit","unit":"Symbol","symbol":"Equivalent","notes":null}],"paper_size":"A4","notes":["| Standard European |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50","12,5","20"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"phone (chișinău landline)":"^022\\s\\d{2}\\s\\d{2}\\s\\d{2}$","phone (mobile national)":"^0[67][0-9]\\s\\d{3}\\s\\d{3}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? лей$","time":"^([01]\\d|2[0-3]):[0-5]\\d$","idnp (personal id)":"^\\d{13}$","phone (mobile international)":"^\\+373\\s[67][0-9]\\s\\d{3}\\s\\d{3}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ru-MD.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'mt-MT'})
SET f.display_name = 'mt-MT Formatting',
    f.content = 'Formatting rules for mt-MT',
    f.llm_context = 'mt-MT: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: EUR after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D ta\' MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Jannar","Frar","Marzu","April","Mejju","Gunju","Lulju","Awwissu","Settembru","Ottubru","Novembru","Dicembru"],"month_abbrev":[],"day_names":["It-Tnejn","It-Tlieta","L-Erbgha","Il-Hamis","Il-Gimgha","Is-Sibt","Il-Hadd"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system predominantly used)","pm_indicator":"N/A (24-hour system predominantly used)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"EUR","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[Locality]\n[Postal Code]\nMALTA","postal_code_pattern":"AAA NNNN (3 letters + space + 4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["25 Triq Ir-Repubblika\nValletta\nVLT 1117\nMALTA"],"postal_code_examples":["VLT 1000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard European format (210 x 297 mm) |","Metric system used exclusively in Malta (EU standard)","Imperial units rarely used except in informal contexts (some British colonial legacy)","Temperature always in Celsius for weather and cooking"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^\\d{4}\\s\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^([01]\\d|2[0-3]):([0-5]\\d)$","phone (international)":"^\\+356\\s\\d{4}\\s\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","currency":"^-?\\d{1,3}(,\\d{3})*(\\.\\d{2})?\\sEUR$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/mt-MT.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'eu-ES'})
SET f.display_name = 'eu-ES Formatting',
    f.content = 'Formatting rules for eu-ES',
    f.llm_context = 'eu-ES: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY/MM/DD (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY/MM/DD","short_pattern":"YY/MM/DD","long_pattern":"YYYY(e)ko MMMM(r)en D(a)","full_pattern":null,"date_separator":"/","month_names":["urtarila","otsaila","martxoa","apirila","maiatza","ekaina","uztaila","abuztua","iraila","urria","azaroa","abendua"],"month_abbrev":[],"day_names":["astelehena","asteartea","asteazkena","osteguna","ostirala","larunbata","igandea"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-12-31","output":"2025/12/31"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":null,"pm_indicator":null,"prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1.234,56 €"},{"input":"0.99","output":"0,99 €"},{"input":"1500.00","output":"1.500,00 €"},{"input":"999999.99","output":"999.999,99 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name], [Number] [Floor/Door]\n[Postal Code] [City]\n[Province/Territory]\nESPAINIA","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Urkijo zumarkalea, 50 2.B\n48011 Bilbo\nBizkaia\nESPAINIA"],"postal_code_examples":["48001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| European standard (210 x 297 mm) |","Basque Country uses metric system exclusively for all measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts","EU regulations require metric system for all commercial purposes","A4 paper size is standard (not US Letter)","Speed limits shown in km/h throughout Spain and Basque Country"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$","phone (national)":"^[6-9]\\d{2}( \\d{2}){3}$","phone (international)":"^\\+34 [6-9]\\d{2}( \\d{2}){3}$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} €$","date":"^\\d{4}/\\d{2}/\\d{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/eu-ES.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ja-JP'})
SET f.display_name = 'ja-JP Formatting',
    f.content = 'Formatting rules for ja-JP',
    f.llm_context = 'ja-JP: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY/MM/DD (gregorian) Time: 24-hour Currency: ¥ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY/MM/DD","short_pattern":"YY/MM/DD","long_pattern":"YYYY年M月D日","full_pattern":null,"date_separator":"/","month_names":["1月","2月","3月","4月","5月","6月","7月","8月","9月","10月","11月","12月"],"month_abbrev":[],"day_names":["月曜日","火曜日","水曜日","木曜日","金曜日","土曜日","日曜日"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-12-31","output":"2025/12/31"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025-01-15"},{"input":"2025年1月15日","output":"2025年01月15日"}]}',
    f.time = '{"system":"24-hour","pattern":"H:mm","pattern_with_seconds":"H:mm:ss","time_separator":":","am_indicator":"午前","pm_indicator":"午後","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"9:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"JPY","symbol":"¥","symbol_position":"before","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10.50","output":"¥11"},{"input":"1234.56","output":"¥1,235"},{"input":"0.99","output":"¥1"},{"input":"1000","output":"¥1,000"},{"input":"50000","output":"¥50,000"},{"input":"1000000","output":"¥1,000,000"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"〒{postal-code}\n{prefecture}{city}{district}{block-number}\n{building-name} {room-number}\nJAPAN","postal_code_pattern":"NNN-NNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["〒150-0001\n東京都渋谷区神宮前1-2-3\n青山ビル5F\nJAPAN"],"postal_code_examples":["100-0001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard A-series paper |","Metric system is standard for all modern measurements","Traditional Japanese units (尺shaku, 貫kan, 升sho, 坪tsubo) still used in specific contexts: construction (坪), sake (升), kimono (尺)","No dual metric/imperial display needed - metric only"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","50","12.5","12.5","100","100","22","36.5"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C","36.5°C"]}',
    f.validation_patterns = '{"time":"^\\d{1,2}:\\d{2}(:\\d{2})?$","phone (international)":"^\\+81 \\d{1,4}-\\d{1,4}-\\d{4}$","date":"^\\d{4}/\\d{2}/\\d{2}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","currency":"^¥-?\\d{1,3}(,\\d{3})*$","phone (national)":"^0\\d{1,4}-\\d{1,4}-\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ja-JP.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'bn-IN'})
SET f.display_name = 'bn-IN Formatting',
    f.content = 'Formatting rules for bn-IN',
    f.llm_context = 'bn-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM, YYYY","full_pattern":null,"date_separator":"/","month_names":["জানুয়ারি","ফেব্রুয়ারি","মার্চ","এপ্রিল","মে","জুন","জুলাই","আগস্ট","সেপ্টেম্বর","অক্টোবর","নভেম্বর","ডিসেম্বর"],"month_abbrev":[],"day_names":["সোমবার","মঙ্গলবার","বুধবার","বৃহস্পতিবার","শুক্রবার","শনিবার","রবিবার"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["শ্রী অভিজিৎ বন্দ্যোপাধ্যায়\n১২৩, পার্ক স্ট্রিট\nবালিগঞ্জ\nকলকাতা - 700019\nপশ্চিমবঙ্গ\nINDIA"],"postal_code_examples":["700001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","India uses metric system exclusively","Exceptions: Some traditional measurements still used in agriculture (e.g., bigha/বিঘা for land)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM\\|পূর্বাহ্ণ\\|অপরাহ্ণ)$","phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^\\d{5}\\s\\d{5}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/bn-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'he-IL'})
SET f.display_name = 'he-IL Formatting',
    f.content = 'Formatting rules for he-IL',
    f.llm_context = 'he-IL: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY or DD.MM.YYYY (both accepted, slash more common) (gregorian) Time: 24-hour Currency: ₪ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY or DD.MM.YYYY (both accepted, slash more common)","short_pattern":"DD/MM/YY","long_pattern":"D בMMMM YYYY (e.g., \"15 בינואר 2025\")","full_pattern":null,"date_separator":"/","month_names":["ינואר","פברואר","מרץ","אפריל","מאי","יוני","יולי","אוגוסט","סeptember","אוקטובר","נובמבר","דצמבר"],"month_abbrev":[],"day_names":["יום שני","יום שלישי","יום רביעי","יום חמישי","יום שישי","שבת","יום ראשון"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"ILS (ISO 4217)","symbol":"₪","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10.50 ₪"},{"input":"1234.56","output":"1,234.56 ₪"},{"input":"0.99","output":"0.99 ₪"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[City] [Postal Code]\n[Country]","postal_code_pattern":"NNNNNNN (7 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["רחוב רוטשילד 1\nתל אביב-יפו 6688101\nישראל"],"postal_code_examples":["6688101"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard 210×297mm |","Israel uses metric system exclusively","Exception: Some product imports may show dual measurements (metric primary)","Temperature always in Celsius for weather, medical, cooking"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","50","12.5","12.5","100","100","22","36.6"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","36.6°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+972\\s?\\d{1,2}\\s?\\d{3}\\s?\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","time":"^([01]\\d|2[0-3]):([0-5]\\d)(:[0-5]\\d)?$","phone (national)":"^0\\d-?\\d{3}-?\\d{4}$","date":"^\\d{1,2}[./]\\d{1,2}[./]\\d{4}$","currency":"^\\d{1,3}(,\\d{3})*\\.\\d{2}\\s?₪$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/he-IL.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'zh-HK'})
SET f.display_name = 'zh-HK Formatting',
    f.content = 'Formatting rules for zh-HK',
    f.llm_context = 'zh-HK: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY年M月D日 (gregorian) Time: 12-hour (colloquial), 24-hour (official/transport) Currency: HK$ or $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY年M月D日","short_pattern":"D/M/YYYY","long_pattern":"YYYY年M月D日 (星期X)","full_pattern":null,"date_separator":"/","month_names":["1月  # yāt yuht (Cantonese)","2月  # yih yuht","3月  # sāam yuht","4月  # sei yuht","5月  # ńgh yuht","6月  # luhk yuht","7月  # chāt yuht","8月  # baat yuht","9月  # gáu yuht","10月  # sahp yuht","11月  # sahp yāt yuht","12月  # sahp yih yuht"],"month_abbrev":[],"day_names":["星期一  # sīng kèih yāt","星期二  # sīng kèih yih","星期三  # sīng kèih sāam","星期四  # sīng kèih sei","星期五  # sīng kèih ńgh","星期六  # sīng kèih luhk","星期日  # sīng kèih yaht (also 星期天 sīng kèih tīn)"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025年1月15日"},{"input":"2025-12-31","output":"2025年12月31日"},{"input":"2025-07-01","output":"2025年7月1日"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"},{"input":"2025-01-15","output":"2025年01月15日"}]}',
    f.time = '{"system":"12-hour (colloquial), 24-hour (official/transport)","pattern":"ah:mm (12-hour) / HH:mm (24-hour)","pattern_with_seconds":"ah:mm:ss / HH:mm:ss","time_separator":":","am_indicator":"上午","pm_indicator":"下午","prayer_times":null,"correct_examples":[{"input":"14:30","output":"下午2時30分"},{"input":"09:00","output":"上午9時"},{"input":"23:59:59","output":"晚上11時59分59秒"},{"input":"00:00","output":"凌晨12時"},{"input":"12:00","output":"中午12時"}],"incorrect_examples":[]}',
    f.currency = '{"code":"HKD","symbol":"HK$ or $","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"HK$10.50"},{"input":"1234.56","output":"HK$1,234.56"},{"input":"0.99","output":"HK$0.99"},{"input":"1000000","output":"HK$1,000,000.00"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Flat/Room], [Floor], [Building Name]\n[Number] [Street Name]\n[District]\nHONG KONG","postal_code_pattern":"None (Hong Kong does not use postal codes for general mail)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Flat A, 12/F, Tower 1, Pacific Place\n88 Queensway\nAdmiralty\nHONG KONG","香港\n金鐘\n金鐘道88號\n太古廣場1座12樓A室","香港\n荔枝角\n美孚新邨A座\n12樓1201室","Suite 2501, 25/F\nOne International Finance Centre\n1 Harbour View Street, Central\nHONG KONG"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210 x 297mm) |","**Metric system**: Official measurement system of Hong Kong","**Traditional Chinese units**: 斤 (catty), 兩 (tael) commonly used in wet markets and Chinese medicine","**Property**: Square feet (sq ft / 平方呎) predominantly used for real estate (British legacy)","**Aviation**: Uses feet and nautical miles per international standards","**British spelling**: metres, litres, kilometres (not American meters, liters, kilometers)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","35°C","-5°C"]}',
    f.validation_patterns = '{"time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9](:[0-5][0-9])?$","time (chinese)":"^(上午\\|下午\\|晚上\\|凌晨\\|早上\\|中午)\\d{1,2}時(\\d{1,2}分)?$","date (numeric)":"^(0?[1-9]\\|[12][0-9]\\|3[01])\\/(0?[1-9]\\|1[0-2])\\/\\d{4}$","phone (international)":"^\\+852 [2356789][0-9]{3} [0-9]{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^(HK)?\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","phone (national)":"^[2356789][0-9]{3} [0-9]{4}$","date (chinese)":"^\\d{4}年\\d{1,2}月\\d{1,2}日$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/zh-HK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-CO'})
SET f.display_name = 'es-CO Formatting',
    f.content = 'Formatting rules for es-CO',
    f.llm_context = 'es-CO: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a. m.","pm_indicator":"p. m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p. m."},{"input":"09:00","output":"9:00 a. m."},{"input":"23:59:59","output":"11:59:59 p. m."}],"incorrect_examples":[]}',
    f.currency = '{"code":"COP (Colombian Peso)","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10500","output":"$10.500"},{"input":"1234567","output":"$1.234.567"},{"input":"99000","output":"$99.000"},{"input":"1000","output":"$1.000"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name] [Number] [Building/Apt]\n[Neighborhood]\n[City], [Department]\n[Postal Code]\nCOLOMBIA","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Carrera 7 No. 71-21, Torre A, Oficina 1002\nChapinero\nBogota D.C., Cundinamarca\n110231\nCOLOMBIA"],"postal_code_examples":["110111"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US Letter (216 x 279 mm) preferred over A4 |","Colombia uses metric system officially, but US Letter paper size is standard","Feet and inches may be used informally for height (e.g., \"mido 1,75 m\" or \"mido 5\'9\\\"\")"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{1,2}:\\d{2}(\\s[ap]\\.\\sm\\.)?$","currency":"^\\$\\d{1,3}(\\.\\d{3})*$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","date":"^\\d{1,2}/\\d{2}/\\d{4}$","phone (national)":"^\\d{3}\\s\\d{3}\\s\\d{4}$","phone (international)":"^\\+57\\s\\d{3}\\s\\d{3}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-CO.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'de-LU'})
SET f.display_name = 'de-LU Formatting',
    f.content = 'Formatting rules for de-LU',
    f.llm_context = 'de-LU: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day, month, year with period separators) (gregorian) Time: 24-hour Currency: EUR (code used more than symbol in German context) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day, month, year with period separators)","short_pattern":"DD.MM.YY (two-digit year)","long_pattern":"D. MMMM YYYY (e.g., \"15. Januar 2026\")","full_pattern":null,"date_separator":".","month_names":["Januar","Februar","Marz","April","Mai","Juni","Juli","August","September","Oktober","November","Dezember"],"month_abbrev":[],"day_names":["Montag","Dienstag","Mittwoch","Donnerstag","Freitag","Samstag","Sonntag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format with leading zeros)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (ISO 4217)","symbol":"EUR (code used more than symbol in German context)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [House Number]\nL-[Postal Code] [City]\nLUXEMBOURG","postal_code_pattern":"L-NNNN (L- prefix + 4 digits, MANDATORY!)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Boulevard Royal 26\nL-2449 Luxembourg\nLUXEMBOURG","Rue de la Gare 15\nL-1616 Luxembourg\nLUXEMBOURG","Avenue de la Liberte 35\nL-1931 Luxembourg\nLUXEMBOURG"],"postal_code_examples":["L-1931"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L or l","notes":null}],"paper_size":"A4","notes":["| Standard: 210 x 297 mm |","Metric system used exclusively for all measurements","No imperial units in common use","Fuel consumption expressed as liters per 100 km (L/100 km)","Fuel prices among lowest in region (ca. 1,40-1,60 EUR/L) attracting cross-border buyers"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100","19"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","currency":"^\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\sEUR$","rcsl (company)":"^B\\d+$","phone (international)":"^\\+352\\s?\\d{2,3}(\\s?\\d{2,3}){1,2}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","phone (national)":"^\\d{2,3}(\\s\\d{2,3}){1,2}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/de-LU.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-BO'})
SET f.display_name = 'es-BO Formatting',
    f.content = 'Formatting rules for es-BO',
    f.llm_context = 'es-BO: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: Bs before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"BOB","symbol":"Bs","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name] [Number]\n[Zona/Barrio Name]\n[City]\n[Department]\nBOLIVIA","postal_code_pattern":"N/A","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Calle Potosi 456\nZona San Pedro\nLa Paz\nLa Paz\nBOLIVIA","Av. Banzer 2345\nBarrio Urbari\nSanta Cruz de la Sierra\nSanta Cruz\nBOLIVIA","Calle Bolivar esquina Junin 789\nZona Central\nCochabamba\nCochabamba\nBOLIVIA","Av. 6 de Agosto 1234\nZona Sopocachi\nLa Paz\nLa Paz\nBOLIVIA"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| Both US Letter (8.5\"x11\") and A4 common |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50","25.5","25,5","100","100","22","35"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","currency":"^Bs\\.? \\d{1,3}(\\.\\d{3})*,\\d{2}$","ci (carnet de identidad)":"^\\d{6,8} (LP\\|SC\\|CB\\|OR\\|PT\\|TJ\\|CH\\|BN\\|PA)$","date":"^\\d{2}/\\d{2}/\\d{4}$","nit (tax id)":"^\\d{9,10}$","phone (national)":"^[2-7] \\d{3} \\d{4}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (international)":"^\\+591 [2-7] \\d{3} \\d{4}$","cua (social security)":"^\\d{8}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-BO.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'th-TH'})
SET f.display_name = 'th-TH Formatting',
    f.content = 'Formatting rules for th-TH',
    f.llm_context = 'th-TH: Numbers use \'.\' decimal, \',\' thousands. Dates: d/M/yyyy (gregorian) Time: 24-hour Currency: ฿ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d/M/yyyy","short_pattern":"d/M/yy","long_pattern":"d MMMM yyyy","full_pattern":null,"date_separator":"/","month_names":["มกราคม","กุมภาพันธ์","มีนาคม","เมษายน","พฤษภาคม","มิถุนายน","กรกฎาคม","สิงหาคม","กันยายน","ตุลาคม","พฤศจิกายน","ธันวาคม"],"month_abbrev":[],"day_names":["วันจันทร์","วันอังคาร","วันพุธ","วันพฤหัสบดี","วันศุกร์","วันเสาร์","วันอาทิตย์"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/1/2568"},{"input":"2025-12-31","output":"31/12/2568"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2568"},{"input":"2025-01-15","output":"15.01.2568"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":null,"pm_indicator":null,"prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"THB","symbol":"฿","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"฿10.50"},{"input":"1234.56","output":"฿1,234.56"},{"input":"0.99","output":"฿0.99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[House Number] [Village/Building], [Soi], [Road]\n[Subdistrict (แขวง/ตำบล)], [District (เขต/อำเภอ)]\n[Province] [Postal Code]\nTHAILAND","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123/45 หมู่บ้านสุขใจ, ซอยสุขุมวิท 21\nแขวงคลองเตยเหนือ, เขตวัฒนา\nกรุงเทพมหานคร 10110\nTHAILAND"],"postal_code_examples":["10110"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard paper size |","Thailand uses metric system exclusively for official measurements","Traditional Thai units (วา, เส้น, บาท weight) exist but rarely used in modern contexts"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+66\\s\\d\\s\\d{4}\\s\\d{4}$","date":"^\\d{1,2}/\\d{1,2}/\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","currency":"^฿\\d{1,3}(,\\d{3})*\\.\\d{2}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (national)":"^0\\d-\\d{4}-\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/th-TH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'kk-KZ'})
SET f.display_name = 'kk-KZ Formatting',
    f.content = 'Formatting rules for kk-KZ',
    f.llm_context = 'kk-KZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: ₸ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY ж.","full_pattern":null,"date_separator":".","month_names":["қаңтар","ақпан","наурыз","сәуір","мамыр","маусым","шілде","тамыз","қыркүйек","қазан","қараша","желтоқсан"],"month_abbrev":[],"day_names":["дүйсенбі","сейсенбі","сәрсенбі","бейсенбі","жұма","сенбі","жексенбі"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025.01.15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"KZT (Kazakhstan Tenge)","symbol":"₸","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 ₸"},{"input":"1234.56","output":"1 234,56 ₸"},{"input":"0.99","output":"0,99 ₸"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name], [Building Number]\n[Postal Code], [City]\n[Region]\nKAZAKHSTAN","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Абай даңғылы, 52\n050000, Алматы қ.\nАлматы облысы\nҚАЗАҚСТАН"],"postal_code_examples":["050000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Kazakhstan uses metric system exclusively for all official purposes","Exception: aviation uses feet for altitude as per international standards"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+7\\s?\\d{3}\\s?\\d{3}-\\d{2}-\\d{2}$","number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","phone (national)":"^8\\s?\\(\\d{3}\\)\\s?\\d{3}-\\d{2}-\\d{2}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","currency":"^-?\\d{1,3}(\\s\\d{3})*(,\\d{2})?\\s₸$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/kk-KZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fi-FI'})
SET f.display_name = 'fi-FI Formatting',
    f.content = 'Formatting rules for fi-FI',
    f.llm_context = 'fi-FI: Numbers use \'.\' decimal, \',\' thousands. Dates: D.M.YYYY (e.g., 15.1.2025) (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D.M.YYYY (e.g., 15.1.2025)","short_pattern":"D.M.YYYY (same as standard)","long_pattern":"D. MMMM YYYY (e.g., 15. tammikuuta 2025)","full_pattern":null,"date_separator":".","month_names":["tammikuu","helmikuu","maaliskuu","huhtikuu","toukokuu","kesäkuu","heinäkuu","elokuu","syyskuu","lokakuu","marraskuu","joulukuu"],"month_abbrev":[],"day_names":["maanantai","tiistai","keskiviikko","torstai","perjantai","lauantai","sunnuntai"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.1.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"H.mm (e.g., 14.30)","pattern_with_seconds":"H.mm.ss","time_separator":".","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14.30"},{"input":"09:00","output":"9.00"},{"input":"23:59:59","output":"23.59.59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1 234,56 €"},{"input":"0.99","output":"0,99 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nFINLAND","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Mannerheimintie 12\n00100 Helsinki\nFINLAND"],"postal_code_examples":["00100"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard European |","Finland uses metric system exclusively","Temperature always in Celsius","Road distances in kilometers","Fuel consumption in liters per 100 km"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^0\\d{1,2}\\s\\d{3,4}\\s\\d{3,4}$","phone (international)":"^\\+358\\s\\d{1,2}\\s\\d{3,4}\\s\\d{3,4}$","time":"^\\d{1,2}\\.\\d{2}(.\\d{2})?$","date":"^\\d{1,2}\\.\\d{1,2}\\.\\d{4}$","number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","currency":"^-?\\d{1,3}(\\s\\d{3})*(,\\d{2})?\\s€$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fi-FI.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-TN'})
SET f.display_name = 'ar-TN Formatting',
    f.content = 'Formatting rules for ar-TN',
    f.llm_context = 'ar-TN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 24-hour Currency: د.ت after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["جانفي (Janfi)","فيفري (Fifri)","مارس","أفريل (Afril)","ماي","جوان (Jouan)","جويلية (Juiliya)","أوت (Out)","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-03-20","output":"20/03/2025"}],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:45","output":"23:45"}],"incorrect_examples":[]}',
    f.currency = '{"code":"TND","symbol":"د.ت","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"1.500","output":"1,500 د.ت"},{"input":"45.000","output":"45,000 د.ت"},{"input":"2500.750","output":"2 500,750 د.ت"},{"input":"0.990","output":"0,990 د.ت"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Number] [Street Type] [Street Name]\n[Postal Code] [City]\n[Governorate]\nTUNISIE","postal_code_pattern":"NNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["السيد أحمد بن علي\n15 نهج الحرية\n1000 تونس\nولاية تونس\nتونس","شركة الجنوب للتصدير\n23 شارع علي بلهوان\n3000 صفاقس\nولاية صفاقس\nتونس","Société Tunisienne d\'Électricité\n8 Rue de Rome\n1000 Tunis\nTUNISIE"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Tunisia uses the metric system exclusively (French colonial legacy)","Temperature: Celsius (°C) - summer 35-45°C","Distance: Kilometers for all road signs","Weight: Kilograms, grams, tonnes","Volume: Liters for fuel and liquids","Land: Hectares (ha) for agriculture"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["19","19","50","50","12,5","12,5"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["35°C","35°C","45°C","45°C","8°C","8°C"]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?[0-9]{1,3}(\\s[0-9]{3})*(,[0-9]+)?$","currency":"^[0-9]{1,3}(\\s[0-9]{3})*(,[0-9]{3})?\\sد\\.ت$","phone (international)":"^\\+216\\s[259]\\d\\s\\d{3}\\s\\d{3}$","landline":"^7[1-8]\\s\\d{3}\\s\\d{3}$","phone (mobile)":"^[259]\\d\\s\\d{3}\\s\\d{3}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-TN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-VN'})
SET f.display_name = 'en-VN Formatting',
    f.content = 'Formatting rules for en-VN',
    f.llm_context = 'en-VN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (day/month/year - following Vietnamese convention) (gregorian) Time: 24-hour (primary), 12-hour (also used in informal contexts) Currency: VND or ₫ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (day/month/year - following Vietnamese convention)","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY (e.g., 15 January 2025)","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour (primary), 12-hour (also used in informal contexts)","pattern":"HH:mm (24-hour format)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"AM (when using 12-hour format)","pm_indicator":"PM (when using 12-hour format)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"VND (ISO 4217 - Vietnamese Dong)","symbol":"VND or ₫","symbol_position":"before","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10000","output":"VND10,000"},{"input":"1234567","output":"VND1,234,567"},{"input":"50000","output":"VND50,000"},{"input":"1000000","output":"VND1,000,000"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building/House Number] [Street Name], [Ward]\n[District], [City/Province]\n[Postal Code]\nVIETNAM","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123 Nguyen Hue Street, Ben Nghe Ward\nDistrict 1, Ho Chi Minh City\n700000\nVIETNAM","Bitexco Financial Tower\n2 Hai Trieu Street, Ben Nghe Ward\nDistrict 1, Ho Chi Minh City\n700000\nVIETNAM"],"postal_code_examples":["700000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard for documents |","Vietnam uses the metric system exclusively","Imperial units rarely encountered except in some imported products or international contexts","British English spelling preferred (kilometres, litres) in formal English documents"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","time (24h)":"^\\d{2}:\\d{2}$","time (12h)":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM)$","phone (national)":"^0\\d{2,3}[\\s]\\d{3,4}[\\s]\\d{3,4}$","phone (international)":"^\\+84[\\s]\\d{2,3}[\\s]\\d{3,4}[\\s]\\d{3,4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","currency":"^(VND)?\\d{1,3}(,\\d{3})*(\\s?VND)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-VN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'tl-PH'})
SET f.display_name = 'tl-PH Formatting',
    f.content = 'Formatting rules for tl-PH',
    f.llm_context = 'tl-PH: Numbers use \'.\' decimal, \',\' thousands. Dates: MM/DD/YYYY (gregorian) Time: 12-hour Currency: P (peso sign) before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"MM/DD/YYYY","short_pattern":"M/D/YY","long_pattern":"MMMM D, YYYY","full_pattern":null,"date_separator":"/","month_names":["Enero","Pebrero","Marso","Abril","Mayo","Hunyo","Hulyo","Agosto","Setyembre","Oktubre","Nobyembre","Disyembre"],"month_abbrev":[],"day_names":["Lunes","Martes","Miyerkules","Huwebes","Biyernes","Sabado","Linggo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PHP","symbol":"P (peso sign)","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Pangalan ng Tumanggap]\n[Numero ng Bahay/Gusali] [Pangalan ng Kalye]\n[Barangay], [Lungsod/Bayan]\n[Lalawigan] [Postal Code]\nPILIPINAS","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Juan dela Cruz\n123 Kalye Rizal, Brgy. San Antonio\nLungsod ng Makati\nKalakhang Maynila 1226\nPILIPINAS","Maria Santos\n456 Kalye Bonifacio, Brgy. Poblacion\nLungsod ng Cebu\nCebu 6000\nPILIPINAS","Anna Reyes\nYunit 1201, One Bonifacio High Street\n5th Avenue, Brgy. Bonifacio Global City\nLungsod ng Taguig\nKalakhang Maynila 1634\nPILIPINAS"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometro","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogramo","symbol":"kg","notes":null},{"category":"Volume","unit":"Litro","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| ISO 216 standard, US Letter ginagamit din sa negosyo |","Ang Pilipinas ay opisyal na gumagamit ng sistemang metriko para sa lahat ng pamahalaan at komersyal na layunin","Ang mga yunit ng US (feet, inches, pounds) ay karaniwan pa ring maintindihan dahil sa impluwensyang Amerikano","Ang taas ay madalas na ipinapahayag sa feet at inches (hal. \"5\'6\")","Ang laki ng screen ay gumagamit ng inches (hal. \"55-inch na TV\")","Ang sukat ng lupa ay gumagamit ng metro kuwadrado o ektarya (hindi acres)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["30°C","25°C","37°C","100°C"]}',
    f.validation_patterns = '{"currency":"^P\\d{1,3}(,\\d{3})*\\.\\d{2}$","date":"^(0[1-9]\\|1[0-2])\\/(0[1-9]\\|[12][0-9]\\|3[01])\\/\\d{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9](:[0-5][0-9])? (AM\\|PM)$","phone (national)":"^0\\d{3} \\d{3} \\d{4}$","phone (international)":"^\\+63 \\d{3} \\d{3} \\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/tl-PH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-PK'})
SET f.display_name = 'en-PK Formatting',
    f.content = 'Formatting rules for en-PK',
    f.llm_context = 'en-PK: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: Rs. before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PKR","symbol":"Rs.","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[House/Building Number], [Street Name]\n[Sector/Block], [Area/Colony]\n[City] - [Postal Code]\n[Province/Territory]\nPAKISTAN","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Mr. Ahmed Khan\nHouse 45, Street 12\nF-7/2, Blue Area\nIslamabad - 44000\nFederal Capital\nPAKISTAN","Ms. Fatima Ali\nFlat 302, Pearl Tower\nClifton, Block 5\nKarachi - 75600\nSindh\nPAKISTAN"],"postal_code_examples":["44000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210 x 297 mm) |","Pakistan uses metric system officially","Traditional units like seer (for weight) and marla/kanal (for land) still used locally","Aviation follows international standards (feet for altitude, nautical miles for distance)","Fuel sold in litres"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^Rs\\.?\\s?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","phone (national)":"^0[0-9]{2,3}\\s[0-9]{7,8}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(AM\\|PM)$","phone (international)":"^\\+92\\s[0-9]{2,3}\\s[0-9]{7,8}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-PK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-CH'})
SET f.display_name = 'fr-CH Formatting',
    f.content = 'Formatting rules for fr-CH',
    f.llm_context = 'fr-CH: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: CHF or Fr. before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":".","month_names":["janvier","fevrier","mars","avril","mai","juin","juillet","aout","septembre","octobre","novembre","decembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"CHF","symbol":"CHF or Fr.","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"CHF 10.50"},{"input":"1234.56","output":"CHF 1\'234.56"},{"input":"0.99","output":"CHF 0.95"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[First Name] [Last Name]\n[Street Name] [Number]\n[Postal Code] [City]\nSUISSE","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Marie Rochat\nAvenue de la Gare 12\n1003 Lausanne\nSUISSE"],"postal_code_examples":["1201"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"degC","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","Switzerland uses metric system exclusively for all measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}([\' ]\\d{3})*,\\d+$","currency":"^(CHF\\s)?\\d{1,3}([\' ]\\d{3})*\\.\\d{2}(\\sCHF)?$","phone (international)":"^\\+41 [1-9]\\d( \\d{3})( \\d{2}){2}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","phone (national)":"^0[1-9]\\d( \\d{3})( \\d{2}){2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-CH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'cy-GB'})
SET f.display_name = 'cy-GB Formatting',
    f.content = 'Formatting rules for cy-GB',
    f.llm_context = 'cy-GB: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour (formal) / 12-hour (informal) Currency: £ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Ionawr","Chwefror","Mawrth","Ebrill","Mai","Mehefin","Gorffennaf","Awst","Medi","Hydref","Tachwedd","Rhagfyr"],"month_abbrev":[],"day_names":["Dydd Llun","Dydd Mawrth","Dydd Mercher","Dydd Iau","Dydd Gwener","Dydd Sadwrn","Dydd Sul"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour (formal) / 12-hour (informal)","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"y.b. (y bore - the morning)","pm_indicator":"y.h. (y hwyr - the evening)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"GBP","symbol":"£","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"£10.50"},{"input":"1234.56","output":"£1,234.56"},{"input":"0.99","output":"£0.99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Rhif Stryd\nTref/Dinas\nSir (dewisol)\nCod Post\nY DEYRNAS UNEDIG","postal_code_pattern":"ANA NAA or AANA NAA (alphanumeric)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["42 Heol y Frenhines\nCaerdydd\nCF10 2HQ\nY DEYRNAS UNEDIG"],"postal_code_examples":["CF10 1AB"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","UK uses metric system for most measurements, but roads use miles","Body weight often expressed in stone/pounds informally","Fuel economy: miles per gallon (mpg) commonly used"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","12.5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C","37.5°C"]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^0[1-9]\\d{2,4} \\d{5,6}$","currency":"^£\\d{1,3}(,\\d{3})*\\.\\d{2}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","phone (international)":"^\\+44 [1-9]\\d{2,4} \\d{5,6}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/cy-GB.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-FJ'})
SET f.display_name = 'en-FJ Formatting',
    f.content = 'Formatting rules for en-FJ',
    f.llm_context = 'en-FJ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: $ or FJ$ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"23:59:59","output":"11:59:59 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"FJD (ISO 4217)","symbol":"$ or FJ$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"$10.50"},{"input":"1234.56","output":"$1,234.56"},{"input":"0.99","output":"$0.99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Number] [Street Name]\n[Suburb/Area]\n[City/Town]\nFIJI","postal_code_pattern":"None (Fiji does not use postal codes)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["John Smith\n42 Victoria Parade\nSuva Central\nSuva\nFIJI"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| International standard |","Fiji uses the metric system exclusively for official purposes","Exception: aviation uses feet for altitude as per international convention"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","phone (national)":"^[3-9]\\d{2}\\s?\\d{4}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(AM\\|PM)$","phone (international)":"^\\+679\\s?[3-9]\\d{2}\\s?\\d{4}$","currency":"^\\$?\\d{1,3}(,\\d{3})*\\.\\d{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-FJ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'af-ZA'})
SET f.display_name = 'af-ZA Formatting',
    f.content = 'Formatting rules for af-ZA',
    f.llm_context = 'af-ZA: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY/MM/DD (official), DD/MM/YYYY (common) (gregorian) Time: 24-hour (official), 12-hour (informal) Currency: R before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY/MM/DD (official), DD/MM/YYYY (common)","short_pattern":"YYYY/MM/DD","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januarie","Februarie","Maart","April","Mei","Junie","Julie","Augustus","September","Oktober","November","Desember"],"month_abbrev":[],"day_names":["Maandag","Dinsdag","Woensdag","Donderdag","Vrydag","Saterdag","Sondag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-12-31","output":"2025/12/31"},{"input":"2025-04-27","output":"2025/04/27"},{"input":"2025-12-16","output":"2025/12/16"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-01-15","output":"January 15, 2025"},{"input":"2025-01-15","output":"15-01-25"}]}',
    f.time = '{"system":"24-hour (official), 12-hour (informal)","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"vm. (voormiddag)","pm_indicator":"nm. (namiddag)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"},{"input":"17:45","output":"17:45"}],"incorrect_examples":[]}',
    f.currency = '{"code":"ZAR","symbol":"R","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"R10,50"},{"input":"1234.56","output":"R1 234,56"},{"input":"0.99","output":"R0,99"},{"input":"1000000","output":"R1 000 000,00"},{"input":"5.00","output":"R5,00"},{"input":"99.95","output":"R99,95"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Number] [Street Name]\n[Suburb/District]\n[City]\n[Postal Code]\nSOUTH AFRICA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123 Kerkstraat\nHatfield\nPretoria\n0083\nSOUTH AFRICA","Eenheid 5, 456 Langstraat\nStellenbosch\n7600\nSOUTH AFRICA","789 Adderleystraat, Kaapstad-Middestad, Kaapstad, 8001\nSOUTH AFRICA"],"postal_code_examples":["0001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","South Africa uses the metric system exclusively","Distance on road signs: kilometers","Speed limits: km/h (commonly 60, 80, 100, 120)","Body weight: kilograms","Height: centimeters or meters (e.g., 1,75 m)","Exception: aviation uses feet and nautical miles worldwide","Cooking: milliliters, grams, liters (metric only)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","0,5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","0°C","37°C","100°C","-5°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+27 [0-9]{2} [0-9]{3} [0-9]{4}$","date":"^\\d{4}\\/(0[1-9]\\|1[0-2])\\/(0[1-9]\\|[12][0-9]\\|3[01])$","currency":"^R[0-9]{1,3}( [0-9]{3})*,[0-9]{2}$","phone (national)":"^0[0-9]{2} [0-9]{3} [0-9]{4}$","number":"^-?[0-9]{1,3}( [0-9]{3})*(,[0-9]+)?$","time":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/af-ZA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-JM'})
SET f.display_name = 'en-JM Formatting',
    f.content = 'Formatting rules for en-JM',
    f.llm_context = 'en-JM: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: J$ (preferred) or $ (in local context only) before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"JMD","symbol":"J$ (preferred) or $ (in local context only)","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Number] [Street Name]\n[District/Town/City] [Kingston Number if applicable]\n[Parish]\nJAMAICA","postal_code_pattern":"NONE - Jamaica does NOT use postal codes","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Devon House\n26 Hope Road\nKingston 10\nSt. Andrew\nJAMAICA","Rose Hall Great House\nRose Hall\nMontego Bay\nSt. James\nJAMAICA","Blue Mountain Coffee Estate\nIrish Town\nSt. Andrew\nJAMAICA","Jamaica Tourist Board\nP.O. Box 360\nKingston 10\nJAMAICA","Digicel Jamaica Limited\n14 Ocean Boulevard\nKingston\nJAMAICA"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| 8.5\" x 11\" - NOT A4 (US influence) |","Jamaica officially adopted metric system but imperial usage persists from British colonial heritage","Road signs: Kilometres (km) officially, but locals often reference miles","Body weight: Frequently expressed in pounds (lb) or stone","Height: Often expressed in feet and inches (e.g., 5\'10\") colloquially","Land area: Measured in acres (historical convention, especially for farms)","Market produce: Often sold by pound in local markets and supermarkets","Cooking: Mix of metric, US cups, and imperial (US media influence)","Fuel: Sold by litre at gas stations","Aviation: Uses feet and nautical miles (international standard)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","15","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","trn (tax id)":"^[0-9]{3}-[0-9]{3}-[0-9]{3}$","passport":"^JM[0-9]{7}$","currency":"^J\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","phone (national 658)":"^\\(658\\) [0-9]{3}-[0-9]{4}$","phone (national 876)":"^\\(876\\) [0-9]{3}-[0-9]{4}$","phone (international)":"^\\+1 (876\\|658) [0-9]{3} [0-9]{4}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-JM.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'da-DK'})
SET f.display_name = 'da-DK Formatting',
    f.content = 'Formatting rules for da-DK',
    f.llm_context = 'da-DK: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day-first) (gregorian) Time: 24-hour Currency: kr. after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day-first)","short_pattern":"DD.MM.YY","long_pattern":"D. MMMM YYYY","full_pattern":null,"date_separator":".","month_names":["januar","februar","marts","april","maj","juni","juli","august","september","oktober","november","december"],"month_abbrev":[],"day_names":["mandag","tirsdag","onsdag","torsdag","fredag","lørdag","søndag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"DKK (Danish Krone)","symbol":"kr.","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 kr."},{"input":"1234.56","output":"1.234,56 kr."},{"input":"0.99","output":"0,99 kr."}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number][, Floor][, Side]\n[Postal Code] [City]\nDENMARK","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Nørregade 10\n1165 København K\nDENMARK"],"postal_code_examples":["1000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210×297 mm) |","Denmark uses the metric system exclusively for all measurements","Imperial units should only be shown in parentheses when relevant for international context (e.g., product specifications)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C","37°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+45 \\d{2} \\d{2} \\d{2} \\d{2}$","phone (national)":"^\\d{2} \\d{2} \\d{2} \\d{2}$","currency":"^-?\\d{1,3}(\\.\\d{3})*(,\\d{2})? kr\\.$","time":"^([0-1]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/da-DK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-US'})
SET f.display_name = 'en-US Formatting',
    f.content = 'Formatting rules for en-US',
    f.llm_context = 'en-US: Numbers use \'.\' decimal, \',\' thousands. Dates: MM/DD/YYYY (gregorian) Time: 12-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"MM/DD/YYYY","short_pattern":"M/D/YY","long_pattern":"MMMM D, YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-12-31","output":"12/31/2025"},{"input":"2025-07-04","output":"07/04/2025"},{"input":"2025-03-17","output":"03/17/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-01-15","output":"Jan 15, 2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"23:59:59","output":"11:59:59 PM"},{"input":"00:00","output":"12:00 AM"},{"input":"12:00","output":"12:00 PM"},{"input":"17:45","output":"5:45 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"USD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"$10.50"},{"input":"1234.56","output":"$1,234.56"},{"input":"0.99","output":"$0.99"},{"input":"1000000","output":"$1,000,000.00"},{"input":"5.00","output":"$5.00"},{"input":"99.95","output":"$99.95"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name] [Apt/Suite] (optional)\n[City], [State] [ZIP Code]\nUNITED STATES","postal_code_pattern":"NNNNN or NNNNN-NNNN (ZIP or ZIP+4)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123 Main Street\nNew York, NY 10001\nUNITED STATES","456 Broadway, Apt 5B\nLos Angeles, CA 90012\nUNITED STATES","789 Market Street\nSan Francisco, CA 94103-1234\nUNITED STATES"],"postal_code_examples":["10001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Fahrenheit","symbol":"°F","notes":null},{"category":"Distance","unit":"Miles","symbol":"mi","notes":null},{"category":"Weight","unit":"Pounds","symbol":"lb","notes":null},{"category":"Volume","unit":"Gallons","symbol":"gal","notes":null}],"paper_size":"Letter","notes":["| 8.5\" × 11\" (not A4) |","US uses imperial/customary units as primary system","Metric sometimes used in scientific, medical contexts (show metric with imperial in parentheses in those contexts)","Exception: aviation uses feet and nautical miles worldwide","Cooking: cups, tablespoons, teaspoons (not metric)","Height: feet and inches (e.g., 5\'10\")","Short distances: feet, yards, inches"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","phone (international)":"^\\+1 [0-9]{3} [0-9]{3} [0-9]{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","date":"^(0[1-9]\\|1[0-2])\\/(0[1-9]\\|[12][0-9]\\|3[01])\\/\\d{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","phone (national)":"^\\([0-9]{3}\\) [0-9]{3}-[0-9]{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-US.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'pt-PT'})
SET f.display_name = 'pt-PT Formatting',
    f.content = 'Formatting rules for pt-PT',
    f.llm_context = 'pt-PT: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour (standard) Currency: EUR after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D \'de\' MMMM \'de\' YYYY","full_pattern":null,"date_separator":"/","month_names":["janeiro","fevereiro","marco","abril","maio","junho","julho","agosto","setembro","outubro","novembro","dezembro"],"month_abbrev":[],"day_names":["segunda-feira","terca-feira","quarta-feira","quinta-feira","sexta-feira","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour (standard)","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"da manha (rarely used, informal contexts)","pm_indicator":"da tarde/da noite (rarely used, informal contexts)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (ISO 4217)","symbol":"EUR","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 EUR"},{"input":"1234.56","output":"1 234,56 EUR"},{"input":"0.99","output":"0,99 EUR"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name], [Number] [Floor/Door]\n[Postal Code] [City]\nPORTUGAL","postal_code_pattern":"NNNN-NNN (7 characters: 4 digits, hyphen, 3 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Avenida da Liberdade, 245 3.o Dto.\n1250-143 Lisboa\nPORTUGAL"],"postal_code_examples":["1200-195"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm (ISO 216) |","Portugal uses the metric system exclusively as mandated by EU regulations","Imperial units appear only in specialized contexts (e.g., nautical miles, screen sizes in inches)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+351 \\d{3} \\d{3} \\d{3}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (national)":"^\\d{3} \\d{3} \\d{3}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2}) EUR$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/pt-PT.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ga-IE'})
SET f.display_name = 'ga-IE Formatting',
    f.content = 'Formatting rules for ga-IE',
    f.llm_context = 'ga-IE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour (primary), 12-hour (also used) Currency: EUR before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Eanair","Feabhra","Marta","Aibrean","Bealtaine","Meitheamh","Iuil","Lunasa","Mean Fomhair","Deireadh Fomhair","Samhain","Nollaig"],"month_abbrev":[],"day_names":["De Luain","De Mairt","De Ceadaoin","Deardaoin","De hAoine","De Sathairn","De Domhnaigh"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour (primary), 12-hour (also used)","pattern":"HH:mm (24-hour) or h:mm a (12-hour)","pattern_with_seconds":"HH:mm:ss (24-hour) or h:mm:ss a (12-hour)","time_separator":":","am_indicator":"r.n. (roimh noon)","pm_indicator":"i.n. (iar noon)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"EUR","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[Town/Village]\n[City/Town]\n[County]\n[Eircode]\nEIRE","postal_code_pattern":"ANN ANNN (Eircode: letter, digit, digit, space, letter, digit, digit, digit)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["15 Sraid Ui Chonaill\nBaile Atha Cliath 1\nD01 X2P3\nEIRE","Teach an Bhaile Mhoir\nAn Baile Mor\nCo. na Gaillimhe\nH91 AB12\nEIRE","Aracan 4, 25 Sraid Grafton\nBaile Atha Cliath 2\nD02 Y3K4\nEIRE"],"postal_code_examples":["D01 X2P3"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Ciliameidear","symbol":"km","notes":null},{"category":"Weight","unit":"Cileagram","symbol":"kg","notes":null},{"category":"Volume","unit":"Liotair","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 mm x 297 mm (caighdean) |","Ireland uses metric system as official standard (EU member)","Road signs: distances in kilometres since 2005 (ciliameidir)","Speed limits: km/h (ciliameadar san uair)","Fuel: sold in litres (liotair)","Legacy imperial: pints for draught beer (piontai - legally defined as 568ml), stone/pounds sometimes for body weight","Exception: aviation uses feet and nautical miles worldwide","Land area: often in acres (acra) in rural/agricultural contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["20°C","0°C","37°C","100°C"]}',
    f.validation_patterns = '{"date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","phone (national)":"^0[1-9][0-9] [0-9]{3} [0-9]{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^EUR[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] [ri]\\.n\\.$","phone (international)":"^\\+353 [1-9][0-9]? [0-9]{3} [0-9]{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ga-IE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-AR'})
SET f.display_name = 'es-AR Formatting',
    f.content = 'Formatting rules for es-AR',
    f.llm_context = 'es-AR: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"(not commonly used)","pm_indicator":"(not commonly used)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"ARS (ISO 4217)","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number], [Floor/Apt]\n[Neighborhood/Barrio]\n[Postal Code] [City]\n[Province]\nARGENTINA","postal_code_pattern":"ANNNNAAA (1 letter + 4 digits + 3 letters, e.g., C1425ABC)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Avenida Corrientes 1234, Piso 5 Dto. B\nSan Nicolas\nC1043AAZ Ciudad Autonoma de Buenos Aires\nBuenos Aires\nARGENTINA"],"postal_code_examples":["C1425ABC"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Always use metric system; imperial units are not used","Height measured in meters and centimeters (e.g., 1,75 m)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\$\\d{1,3}(\\.\\d{3})*(,\\d{2})?$","phone (international)":"^\\+54\\s9?\\s?\\d{2}\\s?\\d{4}-?\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","phone (national)":"^\\(0\\d{2,4}\\)\\s?\\d{4}-\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-AR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'gl-ES'})
SET f.display_name = 'gl-ES Formatting',
    f.content = 'Formatting rules for gl-ES',
    f.llm_context = 'gl-ES: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: EUR after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["xaneiro","febreiro","marzo","abril","maio","xuno","xullo","agosto","setembro","outubro","novembro","decembro"],"month_abbrev":[],"day_names":["luns","martes","mercores","xoves","venres","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":null,"pm_indicator":null,"prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"EUR","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name], [Number] [Floor/Door]\n[Postal Code] [City]\n[Province]\nESPANA","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Rua do Franco, 28 3oB\n15705 Santiago de Compostela\nA Coruna\nESPANA"],"postal_code_examples":["15001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"oC","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| European standard (210 x 297 mm) |","Galicia uses metric system exclusively for all measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts","EU regulations require metric system for all commercial purposes","A4 paper size is standard (not US Letter)","Speed limits shown in km/h throughout Galicia and Spain"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} EUR$","phone (international)":"^\\+34 [6-9]\\d{2}( \\d{2}){3}$","phone (national)":"^[6-9]\\d{2}( \\d{2}){3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/gl-ES.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-ES'})
SET f.display_name = 'es-ES Formatting',
    f.content = 'Formatting rules for es-ES',
    f.llm_context = 'es-ES: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miércoles","jueves","viernes","sábado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":null,"pm_indicator":null,"prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1.234,56 €"},{"input":"0.99","output":"0,99 €"},{"input":"1500.00","output":"1.500,00 €"},{"input":"999999.99","output":"999.999,99 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name], [Number] [Floor/Door]\n[Postal Code] [City]\n[Province]\nESPAÑA","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Calle Alcalá, 50 2ºB\n28014 Madrid\nMadrid\nESPAÑA"],"postal_code_examples":["28001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| European standard (210 × 297 mm) |","Spain uses metric system exclusively for all measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts","EU regulations require metric system for all commercial purposes","A4 paper size is standard (not US Letter)","Speed limits shown in km/h throughout Spain"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"phone (national)":"^[6-9]\\d{2}( \\d{2}){3}$","phone (international)":"^\\+34 [6-9]\\d{2}( \\d{2}){3}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} €$","date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-ES.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'zh-TH'})
SET f.display_name = 'zh-TH Formatting',
    f.content = 'Formatting rules for zh-TH',
    f.llm_context = 'zh-TH: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY年M月D日 (gregorian) Time: 24-hour Currency: ฿ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY年M月D日","short_pattern":"D/M/YYYY","long_pattern":"YYYY年M月D日","full_pattern":null,"date_separator":"/","month_names":["1月  # yī yuè","2月  # èr yuè","3月  # sān yuè","4月  # sì yuè","5月  # wǔ yuè","6月  # liù yuè","7月  # qī yuè","8月  # bā yuè","9月  # jiǔ yuè","10月  # shí yuè","11月  # shíyī yuè","12月  # shí\'èr yuè"],"month_abbrev":[],"day_names":["星期一  # xīngqī yī","星期二  # xīngqī èr","星期三  # xīngqī sān","星期四  # xīngqī sì","星期五  # xīngqī wǔ","星期六  # xīngqī liù","星期日  # xīngqī rì (also 星期天)"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025年1月15日"},{"input":"2025-12-31","output":"2025年12月31日"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-01-15","output":"2025-1-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"上午","pm_indicator":"下午","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"THB","symbol":"฿","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"฿10.50"},{"input":"1234.56","output":"฿1,234.56"},{"input":"0.99","output":"฿0.99"},{"input":"1000","output":"฿1,000.00"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[House Number] [Village/Building], [Soi], [Road]\n[Subdistrict (แขวง/ตำบล)], [District (เขต/อำเภอ)]\n[Province] [Postal Code]\nTHAILAND","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123/45号 素坤逸路21巷\n北孔堤区 瓦塔纳区\n曼谷 10110\n泰国"],"postal_code_examples":["10110"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210x297mm) |","**Metric only**: Thailand uses metric system exclusively for official measurements","**Traditional Thai units**: Rarely used (วา/wa for area, บาท/baht for gold weight)","**Chinese terms**: Use Chinese metric terms (公里, 公斤, 升) not traditional Chinese units","**No imperial**: Imperial units (miles, pounds, Fahrenheit) not used"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","35°C"]}',
    f.validation_patterns = '{"date":"^\\d{4}年\\d{1,2}月\\d{1,2}日$","currency":"^฿\\d{1,3}(,\\d{3})*\\.\\d{2}$","phone (national)":"^0\\d-\\d{4}-\\d{4}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (international)":"^\\+66\\s\\d\\s\\d{4}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/zh-TH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'be-BY'})
SET f.display_name = 'be-BY Formatting',
    f.content = 'Formatting rules for be-BY',
    f.llm_context = 'be-BY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: Br after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY г.","full_pattern":null,"date_separator":".","month_names":["студзеня","лютага","сакавіка","красавіка","мая","чэрвеня","ліпеня","жніўня","верасня","кастрычніка","лістапада","снежня"],"month_abbrev":[],"day_names":["панядзелак","аўторак","серада","чацвер","пятніца","субота","нядзеля"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"BYN","symbol":"Br","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 Br"},{"input":"1234.56","output":"1 234,56 Br"},{"input":"0.99","output":"0,99 Br"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name], [Building Number], [Apartment]\n[Postal Code], г. [City]\n[Region/Voblast]\nБЕЛАРУСЬ","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["вуліца Незалежнасці, 12, кв. 34\n220030, г. Мінск\nМінская вобласць\nБЕЛАРУСЬ"],"postal_code_examples":["220030"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["| Standard European |","Belarus uses metric system exclusively for all measurements","Temperature always in Celsius","Road distances in kilometers, short distances in meters"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? Br$","phone (international)":"^\\+375 \\d{2} \\d{3}-\\d{2}-\\d{2}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","time":"^([01]\\d|2[0-3]):[0-5]\\d$","phone (national)":"^8 \\(\\d{2,3}\\) \\d{3}-\\d{2}-\\d{2}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/be-BY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'wo-SN'})
SET f.display_name = 'wo-SN Formatting',
    f.content = 'Formatting rules for wo-SN',
    f.llm_context = 'wo-SN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: CFA or F CFA after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Samwiye","Fewriye","Maars","Awril","Me","Suwe","Sulet","Ut","Septaambar","Oktoobar","Nowaambar","Desaambar"],"month_abbrev":[],"day_names":["Altine","Talaata","Alarba","Alxamis","Ajjuma","Gaawu","Dibeer"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"XOF","symbol":"CFA or F CFA","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Number] [Street Name]\n[Neighborhood/District]\n[City] [Postal Code]\nSENEGAL","postal_code_pattern":"BP NNNNN or NNNNN (BP = Boite Postale / PO Box)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"PO Box","example_addresses":["Mamadou Diop\n25 Rue Felix Faure\nPlateau\nDakar BP 12345\nSENEGAL"],"postal_code_examples":["BP 5000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |","Senegal uses metric system exclusively for all measurements","Imperial units (miles, pounds, Fahrenheit) are not used in official contexts"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\d{1,3}( \\d{3})* CFA$","phone (international)":"^\\+221 \\d{2} \\d{3} \\d{2} \\d{2}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^\\d{2} \\d{3} \\d{2} \\d{2}$","number":"^-?\\d{1,3}( \\d{3})*,\\d+$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/wo-SN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-NI'})
SET f.display_name = 'es-NI Formatting',
    f.content = 'Formatting rules for es-NI',
    f.llm_context = 'es-NI: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: C$ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p.m."},{"input":"09:00","output":"9:00 a.m."},{"input":"23:59:59","output":"11:59:59 p.m."},{"input":"12:00","output":"12:00 p.m."},{"input":"00:00","output":"12:00 a.m."},{"input":"18:00","output":"6:00 p.m."}],"incorrect_examples":[]}',
    f.currency = '{"code":"NIO","symbol":"C$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"C$10.50"},{"input":"1234.56","output":"C$1,234.56"},{"input":"0.99","output":"C$0.99"},{"input":"12500.00","output":"C$12,500.00"},{"input":"999999.99","output":"C$999,999.99"},{"input":"35.00","output":"C$35.00"},{"input":"100.00","output":"C$100.00"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Reference Point/Landmark], [Directions]\n[Barrio], [City]\n[Departamento]\n[Postal Code]\nNICARAGUA","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["[Street Name] [Number]\n[Barrio], [City]\n[Departamento]\n[Postal Code]\nNICARAGUA","De la Rotonda El Gueguense\n2 cuadras al sur, 1 cuadra al este\nCasa #234, Barrio Belmonte\nManagua\n11001\nNICARAGUA","Carretera a Masaya Km 7.5\nResidencial Las Colinas, Casa B-15\nManagua\n11001\nNICARAGUA"],"postal_code_examples":["11001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US Letter (8.5\" x 11\") common |","Metric system is official in Nicaragua","Exception: \"Libras\" (pounds) extremely common in markets for produce, meat, cheese","\"Varas\" (colonial measurement) and \"cuadras\" (city blocks) used informally for distance","Gasoline sold by GALLON at Petronic/Uno stations (not liter - unique in region)","Paper size uses US Letter format due to US business influence","Speed limits shown in km/h on highways","Land measured in \"manzanas\" (traditional unit, approx 0.7 hectares)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+505\\s\\d{4}\\s\\d{4}$","currency":"^C\\$\\d{1,3}(,\\d{3})*\\.\\d{2}$","time":"^\\d{1,2}:\\d{2}(:\\d{2})?\\s[ap]\\.m\\.$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^\\d{4}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-NI.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'sv-SE'})
SET f.display_name = 'sv-SE Formatting',
    f.content = 'Formatting rules for sv-SE',
    f.llm_context = 'sv-SE: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY-MM-DD (ISO 8601 - preferred in Sweden) (gregorian) Time: 24-hour Currency: kr after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY-MM-DD (ISO 8601 - preferred in Sweden)","short_pattern":"YY-MM-DD","long_pattern":"D MMMM YYYY (e.g., \"8 december 2025\")","full_pattern":null,"date_separator":"-","month_names":["januari","februari","mars","april","maj","juni","juli","augusti","september","oktober","november","december"],"month_abbrev":[],"day_names":["måndag","tisdag","onsdag","torsdag","fredag","lördag","söndag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025-01-15"},{"input":"2025-12-31","output":"2025-12-31"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (e.g., \"14:30\")","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"SEK (Swedish Krona)","symbol":"kr","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 kr"},{"input":"1234.56","output":"1 234,56 kr"},{"input":"0.99","output":"0,99 kr"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nSWEDEN","postal_code_pattern":"NNN NN (5 digits with space after third digit, e.g., \"123 45\")","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Drottninggatan 95\n111 60 Stockholm\nSWEDEN"],"postal_code_examples":["111 60"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard European paper size (210 × 297 mm) |","Sweden uses metric system exclusively","No imperial units in common use"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+46\\s?\\d{1,3}\\s?\\d{3}\\s?\\d{2}\\s?\\d{2}$","date":"^\\d{4}-\\d{2}-\\d{2}$","number":"^-?[\\d\\s]+,\\d+$","currency":"^-?[\\d\\s]+,\\d{2}\\s?kr$","phone (national)":"^0\\d{1,3}[\\s-]?\\d{3}[\\s]?\\d{2}[\\s]?\\d{2}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/sv-SE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'hr-HR'})
SET f.display_name = 'hr-HR Formatting',
    f.content = 'Formatting rules for hr-HR',
    f.llm_context = 'hr-HR: Numbers use \'.\' decimal, \',\' thousands. Dates: d. M. yyyy. (day-first, period separators, trailing period on year) (gregorian) Time: 24-hour Currency: EUR or euro (written out in Croatian contexts) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d. M. yyyy. (day-first, period separators, trailing period on year)","short_pattern":"d. M. yy.","long_pattern":"d. MMMM yyyy.","full_pattern":null,"date_separator":".","month_names":["sijecanj","veljaca","ozujak","travanj","svibanj","lipanj","srpanj","kolovoz","rujan","listopad","studeni","prosinac"],"month_abbrev":[],"day_names":["ponedjeljak","utorak","srijeda","cetvrtak","petak","subota","nedjelja"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15. 1. 2025."},{"input":"2025-12-31","output":"31. 12. 2025."}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (leading zero for hours is standard)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (Euro - Croatia joined the Eurozone on 1 January 2023)","symbol":"EUR or euro (written out in Croatian contexts)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 EUR"},{"input":"1234.56","output":"1.234,56 EUR"},{"input":"0.99","output":"0,99 EUR"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nCROATIA","postal_code_pattern":"NNNNN (5 digits, no spaces)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Ilica 1\n10000 Zagreb\nCROATIA"],"postal_code_examples":["10000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"l or L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (210 x 297 mm) |","Metric system exclusively - no imperial units in standard use","Exception: Aviation uses feet and nautical miles per international standards"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","phone (national)":"^0\\d{1,2}\\s\\d{3}\\s\\d{3,4}$","phone (international)":"^\\+385\\s\\d{1,2}\\s\\d{3}\\s\\d{3,4}$","currency":"^\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\s?EUR$","date":"^\\d{1,2}\\.\\s?\\d{1,2}\\.\\s?\\d{4}\\.$","time":"^\\d{2}:\\d{2}(:\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/hr-HR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-CI'})
SET f.display_name = 'fr-CI Formatting',
    f.content = 'Formatting rules for fr-CI',
    f.llm_context = 'fr-CI: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (e.g., 15/01/2026) (gregorian) Time: 24-hour (French convention) Currency: F CFA after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","INS Cote d\'Ivoire (national statistics"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (e.g., 15/01/2026)","short_pattern":"DD/MM/YY (e.g., 15/01/26)","long_pattern":"D MMMM YYYY (e.g., 15 janvier 2026)","full_pattern":"EEEE D MMMM YYYY (e.g., mercredi 15 janvier 2026)","date_separator":"/","month_names":["janvier","février","mars","avril","mai","juin","juillet","août","septembre","octobre","novembre","décembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15/01/2026"},{"input":"2026-12-31","output":"31/12/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"2026-01-15"}]}',
    f.time = '{"system":"24-hour (French convention)","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss (e.g., 14:30:45)","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"XOF (Franc CFA BCEAO - West African CFA Franc)","symbol":"F CFA","symbol_position":"after","space_between":true,"decimal_places":1,"subunit":"none in circulation","correct_examples":[{"input":"1500","output":"1 500 F CFA"},{"input":"25000","output":"25 000 F CFA"},{"input":"150000","output":"150 000 F CFA"},{"input":"10000000","output":"10 000 000 F CFA"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Number] [Street Type] [Street Name] (if applicable)\n[Commune/District]\n[City] [District Code or BP]\nCOTE D\'IVOIRE","postal_code_pattern":"NN (Abidjan district 01-28) or BP NNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["M. Kouadio KOFFI\nImmeuble CCIA, 4eme etage\nAvenue Franchet d\'Esperey\nPlateau\nAbidjan 01\nCOTE D\'IVOIRE","Mme Aminata TOURE\nVilla 25, Rue des Jardins\nII Plateaux Vallon\nCocody\nAbidjan 08\nCOTE D\'IVOIRE","SARL IVOIRE COMMERCE\nZone Industrielle de Yopougon\nBP 1234 Abidjan 16\nCOTE D\'IVOIRE","M. Jean-Baptiste GNAMBA\nQuartier Commerce\nBP 567 BOUAKE\nCOTE D\'IVOIRE"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Fuel","unit":"Liters","symbol":"L","notes":null},{"category":"Kilogram","unit":"kg","symbol":"Key export products","notes":null},{"category":"Hectare","unit":"ha","symbol":"Agriculture, plantations","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Cote d\'Ivoire uses French metric system exclusively","Agricultural exports (cocoa, coffee) measured in tonnes and kg","Imperial units not used in any context"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["32°C","35°C","25°C","28°C"]}',
    f.validation_patterns = '{"phone (landline)":"^2[17] \\d{2} \\d{2} \\d{2} \\d{2}$","ncc (company id)":"^CI-[A-Z]{3}-\\d{4}-[A-Z]-\\d{5}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","currency":"^\\d{1,3}( \\d{3})* F CFA$","cni (national id)":"^C\\d{9}[A-Z]{2}$","phone (international)":"^\\+225 (0[157]\\|2[17]) \\d{2} \\d{2} \\d{2} \\d{2}$","phone (mobile)":"^0[157] \\d{2} \\d{2} \\d{2} \\d{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-CI.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ku-TR'})
SET f.display_name = 'ku-TR Formatting',
    f.content = 'Formatting rules for ku-TR',
    f.llm_context = 'ku-TR: Numbers use \'.\' decimal, \',\' thousands. Dates: D.MM.YYYY (day.month.year) (gregorian) Time: 24-hour Currency: ₺ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D.MM.YYYY (day.month.year)","short_pattern":"D.MM.YY","long_pattern":"D\'ê MMMM YYYY (with ezafe construction)","full_pattern":null,"date_separator":".","month_names":["Rêbendan","Reşemeh","Adar","Avrêl","Gulan","Pûşper","Tîrmeh","Gelawêj","Rezber","Kewçêr","Sermawez","Berfanbar"],"month_abbrev":[],"day_names":["Duşem","Sêşem","Çarşem","Pêncşem","Înî","Şemî","Yekşem"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"TRY (Turkish Lira)","symbol":"₺","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 ₺"},{"input":"1234.56","output":"1.234,56 ₺"},{"input":"0.99","output":"0,99 ₺"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Kolanname No: Hejmar, Tax\nKoda Posteyî Navçe/Bajar\nTURKEY","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Gazi Caddesi No: 45, Baglar\n21100 Baglar/Amed (Diyarbakir)\nTURKEY"],"postal_code_examples":["21100"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard international size (210x297mm) |","Turkey uses the metric system exclusively","International contexts may require dual units (metric first, imperial in parentheses)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\s₺$","date":"^\\d{1,2}\\.\\d{2}\\.\\d{4}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^0[2-5]\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","phone (international)":"^\\+90\\s\\d{3}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ku-TR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-BW'})
SET f.display_name = 'en-BW Formatting',
    f.content = 'Formatting rules for en-BW',
    f.llm_context = 'en-BW: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour (formal), 12-hour (informal) Currency: P before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour (formal), 12-hour (informal)","pattern":"HH:mm (24-hour) or h:mm a (12-hour)","pattern_with_seconds":"HH:mm:ss or h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"BWP (Botswana Pula)","symbol":"P","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":"thebe (100 thebe = 1 pula)","correct_examples":[{"input":"10.50","output":"P10.50"},{"input":"1234.56","output":"P1,234.56"},{"input":"0.99","output":"P0.99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Recipient Name\nPlot Number/Street Address\nWard/Area Name\nCity/Town\nBOTSWANA","postal_code_pattern":"Not used (Botswana uses physical addresses and plot numbers)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Mr. Thabo Modise\nPlot 1234, Queens Road\nThe Village\nGaborone\nBOTSWANA"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| International standard (ISO 216) |","Always use metric units; imperial units are not commonly understood","Vehicle speed limits in km/h (e.g., 120 km/h on highways)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","currency":"^P\\d{1,3}(,\\d{3})*\\.\\d{2}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (national)":"^(7[1-7])\\s?\\d{3}\\s?\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (international)":"^\\+267\\s?(7[1-7])\\s?\\d{3}\\s?\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-BW.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-IQ'})
SET f.display_name = 'ar-IQ Formatting',
    f.content = 'Formatting rules for ar-IQ',
    f.llm_context = 'ar-IQ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: د.ع after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"arabic-indic","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["كانون الثاني","شباط","آذار","نيسان","أيار","حزيران","تموز","آب","أيلول","تشرين الأول","تشرين الثاني","كانون الأول"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"١٥/٠١/٢٠٢٥"},{"input":"2025-12-31","output":"٣١/١٢/٢٠٢٥"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"٢:٣٠ م"},{"input":"09:00","output":"٩:٠٠ ص"},{"input":"23:59:59","output":"١١:٥٩:٥٩ م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"IQD","symbol":"د.ع","symbol_position":"after","space_between":true,"decimal_places":33,"subunit":null,"correct_examples":[{"input":"10.500","output":"١٠٫٥٠٠ د.ع"},{"input":"1234.567","output":"١٬٢٣٤٫٥٦٧ د.ع"},{"input":"0.999","output":"٠٫٩٩٩ د.ع"},{"input":"250000","output":"٢٥٠٬٠٠٠ د.ع"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building/House Number] [Street Name]\n[District/Neighborhood]\n[City], [Governorate]\n[Postal Code]\nالعراق","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["١٢٣ شارع الرشيد\nمحلة الكرادة\nبغداد، محافظة بغداد\n١٠٠٠١\nالعراق"],"postal_code_examples":["١٠٠٠١"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°م","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"كم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"كجم","notes":null},{"category":"Volume","unit":"Liters","symbol":"ل","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","Iraq uses the metric system for official and commercial purposes","Temperature always in Celsius (°م) for weather, medical, cooking","Distance in kilometers (كم) for road signs, maps","Oil and gas industry may use imperial units (barrels, feet) in technical contexts","Traditional units: راتل (ratl) for weight still used informally in markets"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","٥٠","12.5"]}',
    f.temperature = '{"format":"{number}°م","default_unit":"celsius","examples":["22°C","٢٢°C","-5°C","45°C","٤٥°C"]}',
    f.validation_patterns = '{"number":"^[؜\\-+]?[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]+)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^[١-٩٠]{1,2}:[٠-٥٩]{2}\\s[صم]$","phone (national)":"^٠[٠-٩]{3}\\s[٠-٩]{3}\\s[٠-٩]{4}$","phone (international)":"^\\+٩٦٤\\s[٠-٩]{3}\\s[٠-٩]{3}\\s[٠-٩]{4}$","currency":"^[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]{3})?\\sد\\.ع$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-IQ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'pa-PK'})
SET f.display_name = 'pa-PK Formatting',
    f.content = 'Formatting rules for pa-PK',
    f.llm_context = 'pa-PK: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: Rs before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["جنوری","فروری","مارچ","اپریل","مئی","جون","جولائی","اگست","ستمبر","اکتوبر","نومبر","دسمبر"],"month_abbrev":[],"day_names":["پیر","منگل","بدھ","جمعرات","جمعہ","ہفتہ","اتوار"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"ش","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PKR","symbol":"Rs","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[House/Building Number], [Street/Sector Name]\n[Area/Mohalla]\n[City] - [Postal Code]\n[Province]\nPAKISTAN","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["محمد علی خان\n23، سیکٹر G-9/1\nاسلام آباد - 44000\nوفاقی دارالحکومت\nPAKISTAN"],"postal_code_examples":["44000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","Pakistan uses the metric system officially","Temperature always in Celsius for weather, medical, cooking","Distance in kilometers for road signs, maps","Traditional units (seer, tola, marla, kanal) still used for gold, land measurement"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","50","12.5","12.5"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","phone (international)":"^\\+92\\s\\d{3}\\s\\d{7}$","number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","currency":"^Rs\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM\\|ص\\|ش)$","phone (national)":"^0\\d{3}-\\d{7}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/pa-PK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-DO'})
SET f.display_name = 'es-DO Formatting',
    f.content = 'Formatting rules for es-DO',
    f.llm_context = 'es-DO: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: US$ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","JCE (Cedula format"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"USD","symbol":"US$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name] [Number], [Apt/Suite]\n[Sector/Ensanche/Urbanizacion]\n[City], [Province]\n[Postal Code]\nREPUBLICA DOMINICANA","postal_code_pattern":"","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Calle El Sol #123, Apto. 4B\nSector Los Rios\nSanto Domingo, Distrito Nacional\n10205\nREPUBLICA DOMINICANA","Av. Abraham Lincoln #456\nEnsanche Piantini\nSanto Domingo, Distrito Nacional\n10147\nREPUBLICA DOMINICANA","Carretera Luperon Km 3.5\nPlaya Dorada\nPuerto Plata, Puerto Plata\n57000\nREPUBLICA DOMINICANA"],"postal_code_examples":["10101"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Weight","unit":"Libra","symbol":"0.4536 kg (1 lb)","notes":null},{"category":"Weight","unit":"Quintal","symbol":"100 libras (45.36 kg)","notes":null},{"category":"Weight","unit":"Onza","symbol":"28.35 g","notes":null},{"category":"Volume","unit":"Galon","symbol":"3.785 L","notes":null},{"category":"Length","unit":"Pie","symbol":"30.48 cm (1 ft)","notes":null},{"category":"Length","unit":"Pulgada","symbol":"2.54 cm (1 in)","notes":null}],"paper_size":"Letter","notes":[]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (alt national)":"^\\d{3}-\\d{3}-\\d{4}$","currency (dop)":"^RD\\$\\d{1,3}(,\\d{3})*\\.\\d{2}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","rnc":"^\\d{9}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^\\(\\d{3}\\) \\d{3}-\\d{4}$","cedula":"^\\d{3}-\\d{7}-\\d{1}$","time":"^\\d{1,2}:\\d{2}(:\\d{2})? [ap]\\.m\\.$","currency (usd)":"^US\\$\\d{1,3}(,\\d{3})*\\.\\d{2}$","nss":"^\\d{9}$","phone (international)":"^\\+1 \\(\\d{3}\\) \\d{3}-\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-DO.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-CR'})
SET f.display_name = 'es-CR Formatting',
    f.content = 'Formatting rules for es-CR',
    f.llm_context = 'es-CR: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₡ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","Tribunal Supremo de Elecciones (TSE"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre (also \"setiembre\" in Costa Rican usage)","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a. m.","pm_indicator":"p. m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p. m."},{"input":"09:00","output":"9:00 a. m."},{"input":"23:59:59","output":"11:59:59 p. m."},{"input":"12:00","output":"12:00 p. m."},{"input":"00:00","output":"12:00 a. m."}],"incorrect_examples":[]}',
    f.currency = '{"code":"CRC","symbol":"₡","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"₡10,50"},{"input":"3500","output":"₡3 500"},{"input":"1234.56","output":"₡1 234,56"},{"input":"25000","output":"₡25 000"},{"input":"999999.99","output":"₡999 999,99"},{"input":"10.50","output":"CRC 10,50"},{"input":"3500","output":"CRC 3 500"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Reference Point], [Distance], [Direction]\n[Neighborhood/Barrio]\n[District (Distrito)], [Canton]\n[Province (Provincia)]\n[Postal Code]\nCOSTA RICA","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Del parque central, 200 metros norte y 100 metros oeste\nBarrio Amon\nCarmen, San Jose\nSan Jose\n10101\nCOSTA RICA","Avenida Central, Calles 5 y 7\nEdificio Torre Mercedes, Piso 3\nSan Jose Centro\nSan Jose, San Jose\n10101\nCOSTA RICA","De la escuela de La Fortuna, 3 km camino al volcan\nLa Fortuna\nLa Fortuna, San Carlos\nAlajuela\n21007\nCOSTA RICA"],"postal_code_examples":["10101"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US letter size (8.5\" x 11\") predominant |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["25","100","3,5","25"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","dimex":"^\\d{11,12}$","phone (international)":"^\\+506 \\d{4} \\d{4}$","currency (symbol)":"^₡\\d{1,3}( \\d{3})*(,\\d{2})?$","time":"^\\d{1,2}:\\d{2}(:\\d{2})? [ap]\\. m\\.$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","cedula":"^[1-9]-\\d{4}-\\d{4}$","currency (code)":"^CRC \\d{1,3}( \\d{3})*,\\d{2}$","phone (national)":"^\\d{4}-\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-CR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ps-AF'})
SET f.display_name = 'ps-AF Formatting',
    f.content = 'Formatting rules for ps-AF',
    f.llm_context = 'ps-AF: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY/MM/DD (hijri) Time: 24-hour Currency: ؋ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"arabic-indic","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY/MM/DD","short_pattern":"YY/MM/DD","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["وری (Wray)  # March 21 - April 20","غویی (Ghuway)  # April 21 - May 21","غبرګولی (Gbargolay)  # May 22 - June 21","چنګاښ (Changakh)  # June 22 - July 22","زمری (Zmaray)  # July 23 - August 22","وږی (Wagay)  # August 23 - September 22","تله (Tala)  # September 23 - October 22","لړم (Laram)  # October 23 - November 21","لیندۍ (Lindai)  # November 22 - December 21","مرغومی (Marghumay)  # December 22 - January 20","سلواغه (Salwagha)  # January 21 - February 19","کب (Kab)  # February 20 - March 20"],"month_abbrev":[],"day_names":["دوشنبه (Doshanba)","سه‌شنبه (Seshanba)","چهارشنبه (Chaharshanba)","پنجشنبه (Panjshanba)","جمعه (Juma)","شنبه (Shanba)","یکشنبه (Yakshanba)"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"۱۴۰۳/۱۰/۲۵"},{"input":"2025-12-31","output":"۱۴۰۴/۱۰/۱۰"}],"incorrect_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"غ.م (ghwaramkha - morning)","pm_indicator":"م.و (mazmakha - evening)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"۱۴:۳۰"},{"input":"09:00","output":"۰۹:۰۰"},{"input":"23:59:59","output":"۲۳:۵۹:۵۹"}],"incorrect_examples":[]}',
    f.currency = '{"code":"AFN","symbol":"؋","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10.50","output":"۱۱ ؋"},{"input":"1234.56","output":"۱٬۲۳۵ ؋"},{"input":"0.99","output":"۱ ؋"},{"input":"1000","output":"زر افغانی"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name]، [Building/House Number]\n[District/Nahia]، [City]\n[Province]\n[Postal Code]\nافغانستان","postal_code_pattern":"NNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["سرک میوند، کور نمبر ۱۲۳\nناحیه ۳، کابل\nولایت کابل\n۱۰۰۱\nافغانستان"],"postal_code_examples":["۱۰۰۱"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"کم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"کیلو","notes":null},{"category":"Volume","unit":"Liters","symbol":"لتر","notes":null}],"paper_size":"A4","notes":["| International standard (210 × 297 mm) |","Afghanistan officially uses the metric system","Traditional Afghan units persist in local markets and rural areas:","سېر (seer) = approximately 7 kg (used for grains, produce)","من (man) = approximately 4.5 kg (regional variations exist)","جريب (jerib) = land measurement unit (2000 m2)","Distance is measured in kilometers on road signs","Temperature always in Celsius for weather and medical contexts"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","۵۰","12.5"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","۲۲°C","-5°C"]}',
    f.validation_patterns = '{"time":"^[۰-۹]{2}:[۰-۹]{2}(:[۰-۹]{2})?$","phone (national)":"^۰[۰-۹]{2}\\s[۰-۹]{3}\\s[۰-۹]{4}$","phone (international)":"^\\+۹۳\\s[۰-۹]{2}\\s[۰-۹]{3}\\s[۰-۹]{4}$","date":"^[۰-۹]{4}/[۰-۹]{2}/[۰-۹]{2}$","currency":"^[۰-۹]{1,3}(٬[۰-۹]{3})*\\s؋$","number":"^−?[۰-۹]{1,3}(٬[۰-۹]{3})*(٫[۰-۹]+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ps-AF.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-CM'})
SET f.display_name = 'fr-CM Formatting',
    f.content = 'Formatting rules for fr-CM',
    f.llm_context = 'fr-CM: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: FCFA (Franc CFA) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["janvier","fevrier","mars","avril","mai","juin","juillet","aout","septembre","octobre","novembre","decembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"XAF (ISO 4217 - Central African CFA franc)","symbol":"FCFA (Franc CFA)","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Number] [Street Type] [Street Name] (if known)\n[Quartier/Neighborhood]\nBP [Number] [City] (if postal box)\n[City]\nCAMEROUN","postal_code_pattern":"None (Cameroon does not use street postal codes)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"PO Box","example_addresses":["M. Jean-Pierre Nguema\nQuartier Bastos\nFace Ambassade de France\nYaounde\nCAMEROUN","Societe Generale des Entreprises\n123 avenue Charles de Gaulle\nQuartier Bonanjo\nBP 4567 Douala\nCAMEROUN","Mme Marthe Fouda\nQuartier Akwa Nord\nPres du Marche Central\nDouala\nCAMEROUN"],"postal_code_examples":[]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogrammes","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null},{"category":"Panier","unit":"Variable","symbol":"Fruits, vegetables","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm standard |"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["32°C","25°C","18°C"]}',
    f.validation_patterns = '{"passport":"^CMR\\d{7}$","phone (landline yaounde)":"^2( \\d{2}){4}$","phone (landline douala)":"^3( \\d{2}){4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (mobile)":"^6[5-9]\\d( \\d{2}){3}$","phone (international)":"^\\+237 \\d( \\d{2}){4}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^\\d( \\d{2}){4}$","bp (boite postale)":"^BP \\d{1,5} [A-Za-z]+$","currency":"^\\d{1,3}( \\d{3})* FCFA$","cni (carte nationale d\'identite)":"^\\d{10}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","niu (numero d\'identification unique)":"^M\\d{13}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-CM.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'el-GR'})
SET f.display_name = 'el-GR Formatting',
    f.content = 'Formatting rules for el-GR',
    f.llm_context = 'el-GR: Numbers use \'.\' decimal, \',\' thousands. Dates: d/M/yyyy (e.g., 15/1/2025) (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d/M/yyyy (e.g., 15/1/2025)","short_pattern":"d/M/yy (e.g., 15/1/25)","long_pattern":"d MMMM yyyy (e.g., 15 Ιανουαρίου 2025)","full_pattern":null,"date_separator":"/","month_names":["Ιανουάριος","Φεβρουάριος","Μάρτιος","Απρίλιος","Μάιος","Ιούνιος","Ιούλιος","Αύγουστος","Σεπτέμβριος","Οκτώβριος","Νοέμβριος","Δεκέμβριος"],"month_abbrev":[],"day_names":["Δευτέρα","Τρίτη","Τετάρτη","Πέμπτη","Παρασκευή","Σάββατο","Κυριακή"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/1/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15/01/2025"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss (e.g., 14:30:45)","time_separator":":","am_indicator":"π.μ. (πρωί - not commonly used)","pm_indicator":"μ.μ. (μεσημέρι/απόγευμα - not commonly used)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1.234,56 €"},{"input":"0.99","output":"0,99 €"},{"input":"3.50","output":"3,50 €"},{"input":"450.00","output":"450,00 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\n[Prefecture (optional)]\nGREECE","postal_code_pattern":"NNN NN (5 digits with space: 3 digits, space, 2 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Λεωφόρος Βασιλίσσης Σοφίας 5\n106 74 Αθήνα\nGREECE","Οδός Τσιμισκή 120\n546 21 Θεσσαλονίκη\nGREECE","Λεωφόρος 62 Μαρτύρων 15\n712 01 Ηράκλειο\nΚρήτη\nGREECE","Χώρα Μυκόνου\n846 00 Μύκονος\nΚυκλάδες\nGREECE"],"postal_code_examples":["106 74"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Greece uses the metric system exclusively for all measurements","Temperature always in Celsius (summer highs 35-40°C, winter lows 5-10°C in Athens)","Road distances in kilometers (Athens-Thessaloniki: 500 km)","Ferry distances often in nautical miles (ναυτικά μίλια) - 1 nm = 1,852 km","Traditional land measure: 1 στρέμμα = 1.000 m² (used in property listings)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","12,5","99,9"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C","38°C"]}',
    f.validation_patterns = '{"time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","date":"^\\d{1,2}/\\d{1,2}/\\d{4}$","phone (national)":"^(2\\d{2}\\|69\\d)\\s?\\d{3}\\s?\\d{4}$","currency":"^-?\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\s€$","passport":"^AE\\s?\\d{7}$","αφμ (tax id)":"^\\d{9}$","phone (international)":"^\\+30\\s?(2\\d{2}\\|69\\d)\\s?\\d{3}\\s?\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","ταυτότητα (id card)":"^[Α-Ω]{2}\\s?\\d{6}$","αμκα (social security)":"^\\d{11}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/el-GR.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'hi-IN'})
SET f.display_name = 'hi-IN Formatting',
    f.content = 'Formatting rules for hi-IN',
    f.llm_context = 'hi-IN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₹ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["जनवरी","फ़रवरी","मार्च","अप्रैल","मई","जून","जुलाई","अगस्त","सितंबर","अक्तूबर","नवंबर","दिसंबर"],"month_abbrev":[],"day_names":["सोमवार","मंगलवार","बुधवार","गुरुवार","शुक्रवार","शनिवार","रविवार"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"पूर्वाह्न","pm_indicator":"अपराह्न","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"23:59:59","output":"11:59:59 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"INR","symbol":"₹","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"₹10.50"},{"input":"1234.56","output":"₹1,234.56"},{"input":"0.99","output":"₹0.99"},{"input":"100000","output":"₹1,00,000"},{"input":"10000000","output":"₹1,00,00,000"},{"input":"750000.50","output":"₹7,50,000.50"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Area Name]\n[Locality], [City/District] - [PIN Code]\n[State]\nINDIA","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["श्री राजेश कुमार\n123, एमजी रोड\nकोरमंगला\nबेंगलुरु - 560034\nकर्नाटक\nINDIA"],"postal_code_examples":["110001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 × 297 mm |","India uses metric system exclusively","Exceptions: Some traditional measurements still used in agriculture (e.g., bigha for land)","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d+)?$","phone (national)":"^\\d{5}\\s\\d{5}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM\\|पूर्वाह्न\\|अपराह्न)$","phone (international)":"^\\+91\\s\\d{5}\\s\\d{5}$","currency":"^₹\\d{1,3}(,\\d{2})*,?\\d{3}(\\.\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/hi-IN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'qu-PE'})
SET f.display_name = 'qu-PE Formatting',
    f.content = 'Formatting rules for qu-PE',
    f.llm_context = 'qu-PE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: S/ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY watapi","full_pattern":null,"date_separator":"/","month_names":["qhulla puquy","hatun puquy","pauqar waray","ayriwa","aymuray","inti raymi","anta sitwa","qhapaq sitwa","uma raymi","qoya raymi","ayamarka","qhapaq raymi"],"month_abbrev":[],"day_names":["killachaw","atipachaw","quyllurchaw","illapachaw","chaskachaw","kuychichaw","intichaw"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"tutamanta","pm_indicator":"chisimanta","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PEN","symbol":"S/","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name] [Number] [Interior/Apt]\n[District]\n[City] [Postal Code]\nPIRUW","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Av. Sol 1234 Int. 5B\nWanchaq\nQusqu 08001\nPIRUW"],"postal_code_examples":["08001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| A4 format (210 mm x 297 mm) standard |","Metric system is official and standard throughout Peru","Paper size uses A4 format (European standard)","Traditional Quechua measurements (tupu for land area) may be used locally but official documents use metric","Gas stations sell fuel by gallon (US gallon) despite metric system for other uses","Speed limits shown in km/h","Agricultural land often measured in hectares (ha) or traditional topos"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+51 \\d{3} \\d{3} \\d{3}$","currency":"^S/ \\d{1,3}(,\\d{3})*\\.\\d{2}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","phone (mobile)":"^\\d{3} \\d{3} \\d{3}$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/qu-PE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'sr-RS'})
SET f.display_name = 'sr-RS Formatting',
    f.content = 'Formatting rules for sr-RS',
    f.llm_context = 'sr-RS: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (period separator) (gregorian) Time: 24-hour Currency: din. or RSD after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (period separator)","short_pattern":"D.M.YYYY","long_pattern":"D. MMMM YYYY. (e.g., \"15. januar 2025.\")","full_pattern":null,"date_separator":".","month_names":["januar","februar","mart","april","maj","jun","jul","avgust","septembar","oktobar","novembar","decembar"],"month_abbrev":[],"day_names":["ponedeljak","utorak","sreda","cetvrtak","petak","subota","nedelja"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025."},{"input":"2025-12-31","output":"31.12.2025."}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"RSD (Serbian Dinar)","symbol":"din. or RSD","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 din."},{"input":"1234.56","output":"1.234,56 din."},{"input":"0.99","output":"0,99 din."}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nSERBIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Knez Mihailova 25\n11000 Beograd\nSERBIA"],"postal_code_examples":["11000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210x297mm) |","Serbia uses metric system exclusively","Imperial units not used in everyday contexts"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (international)":"^\\+381 \\d{2} \\d{3} \\d{3,4}$","phone (national)":"^0\\d{2} \\d{3} \\d{3,4}$","currency":"^\\d{1,3}(\\.\\d{3})*,\\d{2} din\\.$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}\\.$","number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/sr-RS.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ms-BN'})
SET f.display_name = 'ms-BN Formatting',
    f.content = 'Formatting rules for ms-BN',
    f.llm_context = 'ms-BN: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: B$ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januari","Februari","Mac","April","Mei","Jun","Julai","Ogos","September","Oktober","November","Disember"],"month_abbrev":[],"day_names":["Isnin","Selasa","Rabu","Khamis","Jumaat","Sabtu","Ahad"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15/01/2026"},{"input":"2026-12-31","output":"31/12/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"2026/01/15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"PG (Pagi)","pm_indicator":"PTG (Petang)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PTG"},{"input":"09:00","output":"9:00 PG"},{"input":"23:59:59","output":"11:59:59 PTG"}],"incorrect_examples":[]}',
    f.currency = '{"code":"BND (ISO 4217)","symbol":"B$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"B$10,50"},{"input":"1234.56","output":"B$1.234,56"},{"input":"0.99","output":"B$0,99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building Name/Number], [Street Name]\n[Kampong/Village]\n[District]\n[Postal Code]\nNEGARA BRUNEI DARUSSALAM","postal_code_pattern":"AANNNN (2 letters + 4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["No. 23, Simpang 88, Jalan Kebangsaan\nKampong Kiulap\nBandar Seri Begawan\nBE1518\nNEGARA BRUNEI DARUSSALAM"],"postal_code_examples":["BE1518"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard ISO paper size |","Brunei uses the metric system exclusively for official purposes","Road signs display distances in kilometers","Fuel prices are among the lowest in the world due to heavy government subsidies (RON95 at B$0,53/L)","Traditional unit \"kati\" (catty = 0.6 kg) may be used informally in wet markets"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["28°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+673\\s\\d{3}\\s\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^\\d{3}\\s\\d{4}$","currency":"^B\\$\\d{1,3}(\\.\\d{3})*(,\\d{2})?$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","time":"^\\d{1,2}:\\d{2}\\s(PG\\|PTG)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ms-BN.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-UY'})
SET f.display_name = 'es-UY Formatting',
    f.content = 'Formatting rules for es-UY',
    f.llm_context = 'es-UY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: $U before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","setiembre  # Note: \"setiembre\" (not \"septiembre\") in Uruguayan Spanish","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"(not commonly used)","pm_indicator":"(not commonly used)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"},{"input":"21:00","output":"21:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"UYU","symbol":"$U","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"$U 10,50"},{"input":"1234.56","output":"$U 1.234,56"},{"input":"0.99","output":"$U 0,99"},{"input":"65000.00","output":"$U 65.000,00"},{"input":"999999.99","output":"$U 999.999,99"},{"input":"450.00","output":"$U 450,00"},{"input":"4500.00","output":"$U 4.500,00"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number], [Apartment]\n[Postal Code] [City]\n[Departamento]\nURUGUAY","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Avenida 18 de Julio 1453, Apto. 302\n11200 Montevideo\nMontevideo\nURUGUAY","Rambla Republica de Mexico 6125\n11400 Montevideo\nMontevideo\nURUGUAY","Calle Sarandi 456\n60000 Paysandu\nPaysandu\nURUGUAY"],"postal_code_examples":["11000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (European influence) |","Metric system is official and strictly used (no imperial)","Strong European influence in measurement standards","Height measured in meters and centimeters (e.g., 1,75 m)","Land area measured in hectares (ha) for agriculture","Fuel sold by liter at Ancap stations"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","phone (international)":"^\\+598\\s\\d{1,2}\\s\\d{3}\\s\\d{3,4}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (national landline)":"^\\d{4}\\s\\d{4}$","phone (national mobile)":"^09\\d\\s\\d{3}\\s\\d{3}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","currency":"^\\$U?\\s\\d{1,3}(\\.\\d{3})*(,\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-UY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'zu-ZA'})
SET f.display_name = 'zu-ZA Formatting',
    f.content = 'Formatting rules for zu-ZA',
    f.llm_context = 'zu-ZA: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY-MM-DD (CLDR standard for zu-ZA) (gregorian) Time: 24-hour (official/formal), 12-hour (informal) Currency: R before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY-MM-DD (CLDR standard for zu-ZA)","short_pattern":"YYYY-MM-DD or M/D/YY","long_pattern":"MMMM D, YYYY","full_pattern":null,"date_separator":"-","month_names":["Januwari","Februwari","Mashi","Ephreli","Meyi","Juni","Julayi","Agasti","Septhemba","Okthoba","Novemba","Disemba"],"month_abbrev":[],"day_names":["UMsombuluko","ULwesibili","ULwesithathu","ULwesine","ULwesihlanu","UMgqibelo","ISonto"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour (official/formal), 12-hour (informal)","pattern":"HH:mm (24-hour) or h:mm a (12-hour)","pattern_with_seconds":"HH:mm:ss or h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"ZAR","symbol":"R","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Number] [Street Name]\n[Suburb/Township]\n[City]\n[Postal Code]\nSOUTH AFRICA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123 Langalibalele Street\nImbali\nPietermaritzburg\n3201\nSOUTH AFRICA","1234 KwaMashu Unit M\nKwaMashu\nDurban\n4360\nSOUTH AFRICA","456 Dr Pixley KaSeme Street, Durban CBD, Durban, 4001\nSOUTH AFRICA"],"postal_code_examples":["3201"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (ISO 216 standard) |","South Africa uses the metric system exclusively","Distance on road signs: kilometres","Speed limits: km/h (commonly 60, 80, 100, 120)","Body weight: kilograms","Height: centimetres or metres (e.g., 1,75 m)","Exception: aviation uses feet and nautical miles worldwide","Cooking: millilitres, grams, litres (metric only)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","0,5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","0°C","37°C","100°C","-5°C"]}',
    f.validation_patterns = '{"number":"^-?[0-9]{1,3}( [0-9]{3})*(,[0-9]+)?$","phone (international)":"^\\+27 [0-9]{2} [0-9]{3} [0-9]{4}$","currency":"^R[0-9]{1,3}( [0-9]{3})*,[0-9]{2}$","time":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","date":"^\\d{4}-(0[1-9]\\|1[0-2])-(0[1-9]\\|[12][0-9]\\|3[01])$","phone (national)":"^0[0-9]{2} [0-9]{3} [0-9]{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/zu-ZA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-BF'})
SET f.display_name = 'fr-BF Formatting',
    f.content = 'Formatting rules for fr-BF',
    f.llm_context = 'fr-BF: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (e.g., 15/01/2026) (gregorian) Time: 24-hour (French convention) Currency: FCFA after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","INSD Burkina Faso (national statistics"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (e.g., 15/01/2026)","short_pattern":"DD/MM/YY (e.g., 15/01/26)","long_pattern":"D MMMM YYYY (e.g., 15 janvier 2026)","full_pattern":"EEEE D MMMM YYYY (e.g., mercredi 15 janvier 2026)","date_separator":"/","month_names":["janvier","fevrier","mars","avril","mai","juin","juillet","aout","septembre","octobre","novembre","decembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour (French convention)","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss (e.g., 14:30:45)","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"XOF (Franc CFA BCEAO - West African CFA Franc)","symbol":"FCFA","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":"none in circulation","correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street/Avenue - if applicable]\n[Secteur/Quartier]\n[XX BP NNNN City XX]\nBURKINA FASO","postal_code_pattern":"NONE - Burkina Faso does not use postal codes","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["M. Moussa OUEDRAOGO\n01 BP 5678\nOuagadougou 01\nBURKINA FASO","Mme Aminata TRAORE\nSecteur 15, Quartier Gounghin\n06 BP 9876\nOuagadougou 06\nBURKINA FASO","SOCIETE EXAMPLE SARL\nAvenue Kwame Nkrumah\nSecteur 4\n01 BP 1234\nOuagadougou 01\nBURKINA FASO","Centre Culturel Henri Matisse\nSecteur 8\n03 BP 567\nBobo-Dioulasso 03\nBURKINA FASO"],"postal_code_examples":["01 BP 1234"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Fuel","unit":"Liters","symbol":"L","notes":null},{"category":"Hectare","unit":"ha","symbol":"Agriculture, livestock","notes":null},{"category":"Kilogram","unit":"kg","symbol":"Export crops","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Burkina Faso uses French metric system exclusively","Cotton industry uses metric tons (coton-graine)","Local markets may use traditional measures (canari, bol, tas) alongside metric","Imperial units not used in any official context"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (mobile)":"^[567]\\d \\d{2} \\d{2} \\d{2}$","currency":"^\\d{1,3}( \\d{3})* FCFA$","phone (landline)":"^2[045] \\d{2} \\d{2} \\d{2}$","rccm (commerce)":"^BF-[A-Z]{3}-\\d{4}$","bp (boite postale)":"^\\d{2} BP \\d{1,5}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","cnib (national id)":"^B\\d{4}\\d{6,8}$","ifu (tax id)":"^\\d{8}$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (international)":"^\\+226 [2567]\\d \\d{2} \\d{2} \\d{2}$","passport":"^A\\d{7}$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-BF.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-SA'})
SET f.display_name = 'en-SA Formatting',
    f.content = 'Formatting rules for en-SA',
    f.llm_context = 'en-SA: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: SAR or SR before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"SAR","symbol":"SAR or SR","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building Number] [Street Name]\n[District/Neighbourhood]\n[City] [Postal Code]\n[Additional Number]\nSAUDI ARABIA","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["4567 King Fahd Road\nAl Olaya District\nRiyadh 11564\n8765\nSAUDI ARABIA","Building 2501 King Abdullah Financial District\nAl Aqiq District\nRiyadh 13519\n3421\nSAUDI ARABIA","Villa 15 Prince Sultan Road\nAl Rawdah District\nJeddah 21432\n5678\nSAUDI ARABIA","Unit 100 The Line\nNEOM Bay District\nNEOM 49643\n1234\nSAUDI ARABIA"],"postal_code_examples":["11564"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","Saudi Arabia uses metric system exclusively for official purposes","Temperature always in Celsius (C) - critical for extreme heat warnings (45C+)","Distance in kilometres (km) for all road signs and navigation","Exception: Oil/gas industry uses imperial (barrels, MCF, feet) in technical contexts","Real estate: Square metres (m2) standard; commercial may show sq ft for expat market","Aviation: Feet for altitude, knots for airspeed (international ICAO standard)","Gold: Grams standard; traditional tola (11.66g) sometimes used in souks"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","15","12.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^(SAR\\|SR)\\s[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","phone (national)":"^0[5][0-9] [0-9]{3} [0-9]{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","phone (international)":"^\\+966 [5][0-9] [0-9]{3} [0-9]{4}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","landline":"^01[1-7] [0-9]{3} [0-9]{4}$","iqama (resident)":"^2[0-9]{9}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-SA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'id-ID'})
SET f.display_name = 'id-ID Formatting',
    f.content = 'Formatting rules for id-ID',
    f.llm_context = 'id-ID: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: Rp before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januari","Februari","Maret","April","Mei","Juni","Juli","Agustus","September","Oktober","November","Desember"],"month_abbrev":[],"day_names":["Senin","Selasa","Rabu","Kamis","Jumat","Sabtu","Minggu"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH.mm","pattern_with_seconds":"HH.mm.ss","time_separator":".","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14.30"},{"input":"09:00","output":"09.00"},{"input":"23:59:59","output":"23.59.59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"IDR","symbol":"Rp","symbol_position":"before","space_between":true,"decimal_places":20,"subunit":null,"correct_examples":[{"input":"10.50","output":"Rp10,50"},{"input":"1234.56","output":"Rp1.234,56"},{"input":"0.99","output":"Rp0,99"},{"input":"1000","output":"Rp1.000"},{"input":"50000","output":"Rp50.000"},{"input":"1000000","output":"Rp1.000.000"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Address with Number]\n[District/Subdistrict], [City] [Postal Code]\n[Province]\nINDONESIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Budi Santoso\nJl. Sudirman No. 123\nMenteng, Jakarta Pusat 10310\nDKI Jakarta\nINDONESIA"],"postal_code_examples":["10310"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Indonesia uses the metric system exclusively for official and commercial purposes","Imperial units may appear in informal contexts due to international influence, but metric is always primary"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{2}\\.\\d{2}(:\\d{2})?$","phone (international)":"^\\+62\\s?\\d{2,3}\\s?\\d{4}\\s?\\d{4,5}$","phone (national)":"^0\\d{2,3}\\s?\\d{4}\\s?\\d{4,5}$","date":"^\\d{2}/\\d{2}/\\d{4}$","currency":"^Rp\\d{1,3}(\\.\\d{3})*(,\\d{2})?$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/id-ID.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-QA'})
SET f.display_name = 'ar-QA Formatting',
    f.content = 'Formatting rules for ar-QA',
    f.llm_context = 'ar-QA: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: ر.ق after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"arabic-indic","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["يناير","فبراير","مارس","أبريل","مايو","يونيو","يوليو","أغسطس","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"١٥/٠١/٢٠٢٥"},{"input":"2025-12-18","output":"١٨/١٢/٢٠٢٥"}],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"٢:٣٠ م"},{"input":"09:00","output":"٩:٠٠ ص"},{"input":"22:00","output":"١٠:٠٠ م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"QAR","symbol":"ر.ق","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"150","output":"١٥٠ ر.ق"},{"input":"7500","output":"٧٬٥٠٠ ر.ق"},{"input":"3.50","output":"٣٫٥٠ ر.ق"},{"input":"1200000","output":"١٬٢٠٠٬٠٠٠ ر.ق"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Recipient Name\nBuilding Number, Street Number\nZone NN, District Name\nCity\nقطر","postal_code_pattern":"None","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["برج الدفنة التجاري\nمبنى ٤٥، شارع ٢٣\nالمنطقة ٦١، الدفنة\nالدوحة\nقطر","شقة ١٢٠٥، برج ٥\nالمنطقة ٦٦، اللؤلؤة\nالدوحة\nقطر","Al Fardan Towers\nBuilding 14, Street 801\nZone 61, West Bay\nDoha\nQatar"],"postal_code_examples":["المنطقة ٦١"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°م","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"كم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"كجم","notes":null},{"category":"Volume","unit":"Liters","symbol":"ل","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Qatar uses the metric system exclusively","Temperature: Celsius (°م) - summer regularly 40-50°C","Distance: Kilometers for all road signs","Exception: Oil and gas industry uses imperial (barrels, feet)","Real estate: Square meters primary; square feet sometimes mentioned"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["5","٥","25","٢٥","12.5"]}',
    f.temperature = '{"format":"{number}°م","default_unit":"celsius","examples":["45°C","٤٥°C","25°C","٢٥°C","20°C","٢٠°C"]}',
    f.validation_patterns = '{"phone (mobile)":"^[٣٥٦٧]\\d{3}\\s\\d{4}$","phone (international)":"^\\+٩٧٤\\s\\d{4}\\s\\d{4}$","number":"^[؜\\-+]?[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]+)?$","time":"^[١-٩٠]{1,2}:[٠-٥٩]{2}\\s[صم]$","currency":"^[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]{2})?\\sر\\.ق$","date":"^\\d{2}/\\d{2}/\\d{4}$","zone":"^المنطقة\\s[١-٩][٠-٩]?$","phone (landline)":"^٤\\d{3}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-QA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-VE'})
SET f.display_name = 'es-VE Formatting',
    f.content = 'Formatting rules for es-VE',
    f.llm_context = 'es-VE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: Bs. before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miércoles","jueves","viernes","sábado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"},{"input":"2025-01-15","output":"15-01-2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p.m."},{"input":"09:00","output":"9:00 a.m."},{"input":"23:59:59","output":"11:59:59 p.m."},{"input":"14:30","output":"14:30"},{"input":"00:00","output":"12:00 a.m."}],"incorrect_examples":[]}',
    f.currency = '{"code":"VES","symbol":"Bs.","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"Bs. 10,50"},{"input":"1234.56","output":"Bs. 1.234,56"},{"input":"0.99","output":"Bs. 0,99"},{"input":"1500.00","output":"Bs. 1.500,00"},{"input":"999999.99","output":"Bs. 999.999,99"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name] [Number]\n[Urbanization/Sector]\n[City], Estado [State]\n[Postal Code]\nVENEZUELA","postal_code_pattern":"NNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Av. Francisco de Miranda, Torre HP\nPiso 12, Ofic. 12-B\nUrb. Los Palos Grandes\nCaracas, Distrito Capital\n1060\nVENEZUELA"],"postal_code_examples":["1010"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US letter size (8.5\" x 11\") more common than A4 |","Metric system is official and standard throughout Venezuela","Paper size commonly uses US Letter format (21.59 cm x 27.94 cm) due to North American influence","Fuel sold by liters at gas stations","Speed limits shown in km/h","Body weight often discussed in kilograms, though some informal use of pounds (libras)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{1,2}:\\d{2}(:\\d{2})?\\s?(a\\.m\\.|p\\.m\\.)?$","phone (international)":"^\\+58 \\d{3} \\d{7}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","phone (national)":"^0\\d{3}-\\d{7}$","currency":"^Bs\\.\\s\\d{1,3}(\\.\\d{3})*,\\d{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-VE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'az-AZ'})
SET f.display_name = 'az-AZ Formatting',
    f.content = 'Formatting rules for az-AZ',
    f.llm_context = 'az-AZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day.month.year) (gregorian) Time: 24-hour Currency: ₼ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day.month.year)","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":".","month_names":["Yanvar","Fevral","Mart","Aprel","May","İyun","İyul","Avqust","Sentyabr","Oktyabr","Noyabr","Dekabr"],"month_abbrev":[],"day_names":["Bazar ertəsi","Çərşənbə axşamı","Çərşənbə","Cümə axşamı","Cümə","Şənbə","Bazar"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"AZN (Azerbaijani Manat)","symbol":"₼","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 ₼"},{"input":"1234.56","output":"1.234,56 ₼"},{"input":"0.99","output":"0,99 ₼"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nAZERBAIJAN","postal_code_pattern":"AZ NNNN (AZ followed by 4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Neftçilər prospekti 153\nAZ 1010 Bakı\nAZERBAIJAN"],"postal_code_examples":["AZ 1000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard international size (210x297mm) |","Azerbaijan uses the metric system exclusively","International contexts may require dual units (metric first, imperial in parentheses)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(\\.\\d{3})*,\\d+$","phone (national)":"^0[1-9]\\d\\s\\d{3}\\s\\d{2}\\s\\d{2}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","phone (international)":"^\\+994\\s\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","currency":"^\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\s₼$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/az-AZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'de-DE'})
SET f.display_name = 'de-DE Formatting',
    f.content = 'Formatting rules for de-DE',
    f.llm_context = 'de-DE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day, month, year with period separators) (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day, month, year with period separators)","short_pattern":"DD.MM.YY (two-digit year)","long_pattern":"D. MMMM YYYY (e.g., \"15. Januar 2025\")","full_pattern":null,"date_separator":".","month_names":["Januar","Februar","März","April","Mai","Juni","Juli","August","September","Oktober","November","Dezember"],"month_abbrev":[],"day_names":["Montag","Dienstag","Mittwoch","Donnerstag","Freitag","Samstag","Sonntag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format with leading zeros)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (ISO 4217)","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1.234,56 €"},{"input":"0.99","output":"0,99 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [House Number]\n[Postal Code] [City]\nGERMANY","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Hauptstraße 123\n10115 Berlin\nGERMANY"],"postal_code_examples":["10115"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L or l","notes":null}],"paper_size":"A4","notes":["| Standard: 210 × 297 mm |","Metric system used exclusively for all measurements","No imperial units in common use","Fuel consumption expressed as liters per 100 km (L/100 km)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^0\\d{2,5}\\s\\d{4,10}$","currency":"^\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\s€$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","phone (international)":"^\\+49\\s\\d{2,5}\\s\\d{4,10}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/de-DE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-SA'})
SET f.display_name = 'ar-SA Formatting',
    f.content = 'Formatting rules for ar-SA',
    f.llm_context = 'ar-SA: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: ر.س after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":"arabic-indic","correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["يناير","فبراير","مارس","أبريل","مايو","يونيو","يوليو","أغسطس","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":["محرم","صفر","ربيع الأول","ربيع الثاني","جمادى الأولى","جمادى الآخرة","رجب","شعبان","رمضان","شوال","ذو القعدة","ذو الحجة"],"calendar_system":"hijri","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":{"fajr":"5:00","dhuhr":"12:30","asr":"3:30","maghrib":"6:00","isha":"7:30"},"correct_examples":[{"input":"14:30","output":"٢:٣٠ م"},{"input":"05:15","output":"٥:١٥ ص"},{"input":"21:00:00","output":"٩:٠٠:٠٠ م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"SAR","symbol":"ر.س","symbol_position":"after","space_between":true,"decimal_places":20,"subunit":null,"correct_examples":[{"input":"150","output":"١٥٠ ر.س"},{"input":"5000","output":"٥٬٠٠٠ ر.س"},{"input":"2.50","output":"٢٫٥٠ ر.س"},{"input":"350000","output":"٣٥٠٬٠٠٠ ر.س"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Building Number] [Street Name]\n[District]\n[City] [Postal Code]\nالمملكة العربية السعودية","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"ص.ب.","example_addresses":["محمد عبدالله الغامدي\nمبنى ٧٦٥٤ شارع الملك فهد\nحي العليا\nالرياض ١٢٢١١\nالمملكة العربية السعودية","شركة أرامكو السعودية\nص.ب. ٥٠٠٠\nالظهران ٣١٣١١\nالمملكة العربية السعودية"],"postal_code_examples":["١٢٢١١"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°م","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"كم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"كجم","notes":null},{"category":"Volume","unit":"Liters","symbol":"ل","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Saudi Arabia uses the metric system exclusively","Temperature: Celsius (summer commonly 45-50°C in Riyadh)","Distance: Kilometers for all road signs and maps","Exception: Oil industry uses barrels (برميل) and feet for technical contexts","Gold: Measured in grams (جرام) and تولة (tola = 11.66g)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["15","١٥","7.5"]}',
    f.temperature = '{"format":"{number}°م","default_unit":"celsius","examples":["45°C","٤٥°C","22°C","٢٢°C","18°C","١٨°C"]}',
    f.validation_patterns = '{"date (gregorian)":"^\\d{2}/\\d{2}/\\d{4}$","phone (mobile)":"^٠٥[٠-٩]\\s\\d{3}\\s\\d{4}$","phone (international)":"^\\+٩٦٦\\s٥[٠-٩]\\s\\d{3}\\s\\d{4}$","number":"^[؜\\-+]?[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]+)?$","currency":"^[٠-٩]{1,3}(٬[٠-٩]{3})*(٫[٠-٩]{2})?\\sر\\.س$","time":"^[١-٩٠]{1,2}:[٠-٥٩]{2}\\s[صم]$","date (hijri)":"^\\d{2}/\\d{2}/\\d{4}\\sهـ$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-SA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-MA'})
SET f.display_name = 'fr-MA Formatting',
    f.content = 'Formatting rules for fr-MA',
    f.llm_context = 'fr-MA: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 24-hour Currency: DH (or د.م. in Arabic contexts) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","HCP (Haut-Commissariat au Plan"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":"EEEE D MMMM YYYY","date_separator":"/","month_names":["janvier","fevrier","mars","avril","mai","juin","juillet","aout","septembre","octobre","novembre","decembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"MAD (Moroccan Dirham)","symbol":"DH (or د.م. in Arabic contexts)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":"centime (1 MAD = 100 centimes)","correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Recipient Name\nNumber, Street Type, Street Name\nQuartier/District (optional)\nPostal Code CITY\nMAROC","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":"Often UPPERCASE in formal addresses","street_types":null,"po_box_format":null,"example_addresses":["M. Ahmed BENNANI\n45 boulevard Mohammed V\nQuartier Maarif\n20100 CASABLANCA\nMAROC","Societe EXEMPLE SARL\nDirection Commerciale\n12 avenue Hassan II\nAgdal\n10080 RABAT\nMAROC","Riad DAR TAZI\n15 Derb El Miter\nFes El Bali\n30110 FES\nMAROC"],"postal_code_examples":["20000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Fuel","unit":"Liters","symbol":"L","notes":null},{"category":"Land","unit":"Hectares","symbol":"ha","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Morocco uses the metric system exclusively for all official measurements","Imperial units (miles, pounds, Fahrenheit) are not used in any context","Exception: aviation uses feet for altitude (international standard)","Traditional units exist but are not used officially (e.g., \"quintal\" = 100 kg for agriculture)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"passport":"^[A-Z]{2}\\d{7}$","cin (carte d\'identite nationale)":"^[A-Z]{1,2}\\d{5,6}$","landline (national)":"^05[2-9]\\d( \\d{2}){3}$","phone (international)":"^\\+212 [567]( \\d{2}){4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","cnss (securite sociale)":"^\\d{9}$","ice (identifiant commun entreprise)":"^\\d{15}$","phone (national)":"^0[567]( \\d{2}){4}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? DH$","if (identifiant fiscal)":"^\\d{8}$","rc (registre commerce)":"^\\d+$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","patente":"^\\d+$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-MA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'uz-UZ'})
SET f.display_name = 'uz-UZ Formatting',
    f.content = 'Formatting rules for uz-UZ',
    f.llm_context = 'uz-UZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (day/month/year) (gregorian) Time: 24-hour Currency: soʻm (or so\'m) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (day/month/year)","short_pattern":"DD/MM/YY","long_pattern":"D-MMMM, YYYY-yil","full_pattern":null,"date_separator":"/","month_names":["yanvar","fevral","mart","aprel","may","iyun","iyul","avgust","sentabr","oktabr","noyabr","dekabr"],"month_abbrev":[],"day_names":["dushanba","seshanba","chorshanba","payshanba","juma","shanba","yakshanba"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"UZS (Uzbek Sum / O\'zbek so\'mi)","symbol":"soʻm (or so\'m)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Name], [Building Number], [Apartment if applicable]\n[District/Neighborhood]\n[City], [Region]\n[Postal Code]\nUZBEKISTAN","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Karimov Alisher\nAmir Temur ko\'chasi, 25-uy, 12-xonadon\nMirzo Ulug\'bek tumani\nToshkent shahri, Toshkent viloyati\n100000\nUZBEKISTAN"],"postal_code_examples":["100000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard international size (210x297mm) |","Uzbekistan uses the metric system exclusively","Imperial units are not used in official or everyday contexts"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^\\d{1,3}(\\s\\d{3})*(,\\d{2})?\\sso[ʻ\']m$","phone (international)":"^\\+998\\s\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^\\d{2}\\s\\d{3}[- ]\\d{2}[- ]\\d{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/uz-UZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'lv-LV'})
SET f.display_name = 'lv-LV Formatting',
    f.content = 'Formatting rules for lv-LV',
    f.llm_context = 'lv-LV: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (e.g., 15.01.2025) (gregorian) Time: 24-hour Currency: EUR after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (e.g., 15.01.2025)","short_pattern":"DD.MM.YYYY (same as standard)","long_pattern":"YYYY. gada D. MMMM (e.g., 2025. gada 15. janvaris)","full_pattern":null,"date_separator":".","month_names":["janvaris","februaris","marts","aprilis","maijs","junijs","julijs","augusts","septembris","oktobris","novembris","decembris"],"month_abbrev":[],"day_names":["pirmdena","otrdiena","tresdiena","ceturtdiena","piektdiena","sestdiena","svetdiena"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"EUR","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number][-[Apartment]]\n[City/Town], LV-[Postal Code]\nLATVIA","postal_code_pattern":"LV-NNNN (LV- prefix followed by 4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Brivibas iela 123-45\nRiga, LV-1010\nLATVIA"],"postal_code_examples":["LV-1010"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard European |","Latvia uses metric system exclusively","Temperature always in Celsius","Road distances in kilometers","Fuel consumption in liters per 100 km"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (international)":"^\\+371\\s\\d{4}\\s\\d{4}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","currency":"^-?\\d{1,3}(\\s\\d{3})*(,\\d{2})?\\sEUR$","phone (national)":"^\\d{4}\\s\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/lv-LV.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'xh-ZA'})
SET f.display_name = 'xh-ZA Formatting',
    f.content = 'Formatting rules for xh-ZA',
    f.llm_context = 'xh-ZA: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY-MM-DD (official), DD/MM/YYYY (common) (gregorian) Time: 24-hour (official), 12-hour (informal) Currency: R before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY-MM-DD (official), DD/MM/YYYY (common)","short_pattern":"YYYY-MM-DD","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"-","month_names":["Januwari","Februwari","Matshi","Epreli","Meyi","Juni","Julayi","Agasti","Septemba","Okthoba","Novemba","Disemba"],"month_abbrev":[],"day_names":["uMvulo","uLwesibini","uLwesithathu","uLwesine","uLwesihlanu","uMgqibelo","iCawa"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour (official), 12-hour (informal)","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"ekuseni (morning)","pm_indicator":"emva kwemini (afternoon)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"ZAR","symbol":"R","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Number] [Street Name]\n[Suburb/Township]\n[City/Town]\n[Postal Code]\nSOUTH AFRICA","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["45 Madiba Drive\nScenery Park\nEast London\n5247\nSOUTH AFRICA","Stand 1234\nKwaNobuhle\nKariega\n6242\nSOUTH AFRICA","78 Sutherland Street, Quigney, East London, 5201\nSOUTH AFRICA"],"postal_code_examples":["5200"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","South Africa uses the metric system exclusively","Distance on road signs: kilometers","Speed limits: km/h (commonly 60, 80, 100, 120)","Body weight: kilograms","Height: centimeters or meters (e.g., 1,75 m)","Exception: aviation uses feet and nautical miles worldwide","Cooking: milliliters, grams, liters (metric only)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","0,5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","0°C","37°C","100°C","-5°C"]}',
    f.validation_patterns = '{"number":"^-?[0-9]{1,3}( [0-9]{3})*(,[0-9]+)?$","phone (national)":"^0[0-9]{2} [0-9]{3} [0-9]{4}$","phone (international)":"^\\+27 [0-9]{2} [0-9]{3} [0-9]{4}$","date":"^\\d{4}-(0[1-9]\\|1[0-2])-(0[1-9]\\|[12][0-9]\\|3[01])$","time":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","currency":"^R[0-9]{1,3}( [0-9]{3})*,[0-9]{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/xh-ZA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ro-MD'})
SET f.display_name = 'ro-MD Formatting',
    f.content = 'Formatting rules for ro-MD',
    f.llm_context = 'ro-MD: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (period separator) (gregorian) Time: 24-hour Currency: lei (plural), leu (singular) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (period separator)","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":".","month_names":["ianuarie","februarie","martie","aprilie","mai","iunie","iulie","august","septembrie","octombrie","noiembrie","decembrie"],"month_abbrev":[],"day_names":["luni","marti","miercuri","joi","vineri","sambata","duminica"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"MDL (Moldovan Leu)","symbol":"lei (plural), leu (singular)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"str. [Street Name], nr. [Number], [Apartment/Building]\nMD-[Postal Code], [City]\nMOLDOVA","postal_code_pattern":"MD-NNNN (MD- prefix + 4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["str. Stefan cel Mare si Sfant, nr. 134, ap. 25\nMD-2012, Chisinau\nMOLDOVA"],"postal_code_examples":["MD-2001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210 x 297 mm) |","Moldova uses metric system exclusively for all official measurements","Imperial units rarely used; if needed, always show metric first with imperial in parentheses","Wine industry: Traditional vedra (~12.3L) sometimes referenced; world\'s highest per-capita wine consumption"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"passport":"^[A-Z]{1,2}\\d{7}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","idnp (personal id)":"^\\d{13}$","buletin (id card)":"^[A-C]\\d{7}$","currency":"^-?\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\slei$","phone (national)":"^0\\d{2}\\s\\d{3}\\s\\d{3}$","phone (international)":"^\\+373\\s\\d{2}\\s\\d{3}\\s\\d{3}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ro-MD.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-NG'})
SET f.display_name = 'en-NG Formatting',
    f.content = 'Formatting rules for en-NG',
    f.llm_context = 'en-NG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₦ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"NGN","symbol":"₦","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"{Building Number/Name} {Street Name}\n{Area/Estate Name}\n{Local Government Area}\n{State}\nNIGERIA","postal_code_pattern":"N/A","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["{Building Number/Name} {Street Name}\n{Near/Opposite/Behind} {Landmark}\n{Area/Estate Name}, {Local Government Area}\n{State}\nNIGERIA","15 Admiralty Way\nLekki Phase 1\nEti-Osa Local Government Area\nLagos State\nNIGERIA","Plot 234 Aminu Kano Crescent\nOpposite Banex Plaza\nWuse 2\nAbuja Municipal Area Council\nFCT Abuja\nNIGERIA","Block 5, Flat 3\nRainbow Town Estate\nPort Harcourt\nRivers State\nNIGERIA"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Metric system is the official standard for all measurements","Fuel (PMS - Premium Motor Spirit) is sold in litres (₦600-800/L as of 2026)","Road distances in kilometres on all FRSC/Federal Road signs","Market measurements may use traditional units: mudu, derica, congo (for grains)","Body weight commonly expressed in kilograms"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"nin":"^\\d{11}$","number":"^-?[\\d,]+(\\.\\d+)?$","currency":"^₦[\\d,]+(\\.\\d{2})?$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","phone (international)":"^\\+234\\s?[7-9][0-9]{2}\\s?[0-9]{3}\\s?[0-9]{4}$","tin":"^\\d{10}$","bvn":"^\\d{11}$","time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(AM\\|PM)$","landline (lagos)":"^01\\s?[0-9]{3}\\s?[0-9]{4}$","phone (national)":"^0[7-9][0-9]{2}\\s?[0-9]{3}\\s?[0-9]{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-NG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'it-IT'})
SET f.display_name = 'it-IT Formatting',
    f.content = 'Formatting rules for it-IT',
    f.llm_context = 'it-IT: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["gennaio","febbraio","marzo","aprile","maggio","giugno","luglio","agosto","settembre","ottobre","novembre","dicembre"],"month_abbrev":[],"day_names":["lunedì","martedì","mercoledì","giovedì","venerdì","sabato","domenica"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system used)","pm_indicator":"N/A (24-hour system used)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1.234,56 €"},{"input":"0.99","output":"0,99 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name], [Number]\n[Postal Code] [City] ([Province Abbr.])\nITALIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Via Giuseppe Verdi, 15\n20121 Milano (MI)\nITALIA"],"postal_code_examples":["00100"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard European format (210 × 297 mm) |","Metric system used exclusively in Italy","Imperial units rarely used except in specialized contexts (aviation, shipping)","Temperature always in Celsius for weather and cooking"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+39\\s0\\d{1,3}\\s\\d{3,4}\\s\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^([01]\\d|2[0-3]):([0-5]\\d)$","phone (national)":"^0\\d{1,3}\\s\\d{3,4}\\s\\d{4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","currency":"^-?\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\s€$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/it-IT.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'no-NO'})
SET f.display_name = 'no-NO Formatting',
    f.content = 'Formatting rules for no-NO',
    f.llm_context = 'no-NO: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day.month.year) (gregorian) Time: 24-hour Currency: kr after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day.month.year)","short_pattern":"DD.MM.YY","long_pattern":"D. MMMM YYYY","full_pattern":null,"date_separator":".","month_names":["januar","februar","mars","april","mai","juni","juli","august","september","oktober","november","desember"],"month_abbrev":[],"day_names":["mandag","tirsdag","onsdag","torsdag","fredag","lørdag","søndag"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"NOK (Norwegian Krone)","symbol":"kr","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 kr"},{"input":"1234.56","output":"1 234,56 kr"},{"input":"0.99","output":"0,99 kr"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Street Name] [Number]\n[Postal Code] [City]\nNORWAY","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Ola Nordmann\nStorgata 12\n0155 Oslo\nNORWAY"],"postal_code_examples":["0155"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard size |","Norway uses the metric system exclusively","Imperial measurements may be shown in parentheses for international contexts","Exception: Aviation and maritime contexts may use international standards (nautical miles, feet)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","phone (international)":"^\\+47 \\d{3} \\d{2} \\d{3}$","number":"^-?\\d{1,3}( \\d{3})*,\\d+$","currency":"^\\d{1,3}( \\d{3})*,\\d{2} kr$","time":"^([01]\\d|2[0-3]):[0-5]\\d$","phone (national)":"^\\d{3} \\d{2} \\d{3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/no-NO.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'sk-SK'})
SET f.display_name = 'sk-SK Formatting',
    f.content = 'Formatting rules for sk-SK',
    f.llm_context = 'sk-SK: Numbers use \'.\' decimal, \',\' thousands. Dates: d. M. yyyy (e.g., 15. 1. 2025) (gregorian) Time: 24-hour Currency: euro after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d. M. yyyy (e.g., 15. 1. 2025)","short_pattern":"d. M. yy (e.g., 15. 1. 25)","long_pattern":"d. MMMM yyyy (e.g., 15. januara 2025)","full_pattern":null,"date_separator":".","month_names":["januar","februar","marec","april","maj","jun","jul","august","september","oktober","november","december"],"month_abbrev":[],"day_names":["pondelok","utorok","streda","stvrtok","piatok","sobota","nedela"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15. 1. 2025"},{"input":"2025-12-31","output":"31. 12. 2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"H:mm (e.g., 9:30, 14:30)","pattern_with_seconds":"H:mm:ss (e.g., 14:30:45)","time_separator":":","am_indicator":"(not used - 24-hour system)","pm_indicator":"(not used - 24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"9:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR (ISO 4217)","symbol":"euro","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 euro"},{"input":"1234.56","output":"1 234,56 euro"},{"input":"0.99","output":"0,99 euro"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Building Number]\n[Postal Code] [City]\nSLOVENSKO","postal_code_pattern":"NNN NN (5 digits with space after first 3)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Hlavna ulica 15\n811 01 Bratislava\nSLOVENSKO"],"postal_code_examples":["811 01"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"l","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Metric system used exclusively - no imperial conversions needed","Exception: aviation uses feet for altitude worldwide"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","25,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{1,2}:\\d{2}(:\\d{2})?$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","date":"^\\d{1,2}\\. \\d{1,2}\\. \\d{4}$","phone (international)":"^\\+421 \\d{3} \\d{3} \\d{3}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? euro$","phone (national)":"^0\\d{3} \\d{3} \\d{3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/sk-SK.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'gn-PY'})
SET f.display_name = 'gn-PY Formatting',
    f.content = 'Formatting rules for gn-PY',
    f.llm_context = 'gn-PY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: Gs. (Guaranies) or ₲ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D jasypo MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["jasyteĩ","jasykõi","jasyapy","jasyrundy","jasypo","jasypoteĩ","jasypokõi","jasypoapy","jasyporundy","jasypa","jasypateĩ","jasypakõi"],"month_abbrev":["In practice","Spanish month names (enero","febrero","marzo","etc.) are commonly used in bilingual contexts."],"day_names":["arakõi","araapy","ararundy","arapo","arapoteĩ","arapokõi","arateĩ"],"day_abbrev":["Spanish day names (lunes","martes","etc.) are commonly used in bilingual Paraguay."],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"pyhareve (morning) - rarely used formally","pm_indicator":"ka\'aru (afternoon) - rarely used formally","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PYG (ISO 4217)","symbol":"Gs. (Guaranies) or ₲","symbol_position":"before","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Tape/Street Name] [Papapy/Number]\n[Tava\'i/Neighborhood]\n[Codigo Postal] [Tava/City]\n[Tetaguasu/Department]\nPARAGUAI","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Tape Mariscal Lopez 1234\nVilla Morra\n1209 Paraguaype (Asuncion)\nTetaguasu Central\nPARAGUAI"],"postal_code_examples":["1209"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Always use metric system; imperial units are not used in Paraguay","Land area often measured in hectares (ha) for agricultural purposes","Traditional Guarani measurements (like \"legua\" for distance) are archaic"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["22°C"]}',
    f.validation_patterns = '{"currency":"^(Gs\\.|₲)\\s?\\d{1,3}(\\.\\d{3})*$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","phone (international)":"^\\+595\\s?\\d{2,3}\\s?\\d{3}\\s?\\d{3,4}$","phone (national)":"^\\(0\\d{2,3}\\)\\s?\\d{3}\\s?\\d{3,4}$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/gn-PY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'uk-UA'})
SET f.display_name = 'uk-UA Formatting',
    f.content = 'Formatting rules for uk-UA',
    f.llm_context = 'uk-UA: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day.month.year) (gregorian) Time: 24-hour Currency: ₴ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day.month.year)","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY р. (day month year with \"р.\" = року/year)","full_pattern":null,"date_separator":".","month_names":["січня","лютого","березня","квітня","травня","червня","липня","серпня","вересня","жовтня","листопада","грудня"],"month_abbrev":[],"day_names":["понеділок","вівторок","середа","четвер","п\'ятниця","субота","неділя"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"UAH (Ukrainian hryvnia)","symbol":"₴","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 ₴"},{"input":"1234.56","output":"1 234,56 ₴"},{"input":"0.99","output":"0,99 ₴"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"вул. [Street Name], [Number]/[Apartment]\nм. [City], [Postal Code]\n[Oblast/Region], UKRAINE","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["вул. Хрещатик, 22/5\nм. Київ, 01001\nКиївська область\nUKRAINE"],"postal_code_examples":["01001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["| European standard |","Ukraine uses the metric system exclusively","Temperature always in Celsius","Distance in kilometers and meters","Weight in kilograms and grams"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","currency":"^-?\\d{1,3}(\\s\\d{3})*(,\\d{2})?\\s₴$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (national)":"^0\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$","phone (international)":"^\\+380\\s\\d{2}\\s\\d{3}\\s\\d{2}\\s\\d{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/uk-UA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'pl-PL'})
SET f.display_name = 'pl-PL Formatting',
    f.content = 'Formatting rules for pl-PL',
    f.llm_context = 'pl-PL: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (period separator) (gregorian) Time: 24-hour Currency: zł after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (period separator)","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY (e.g., \"15 stycznia 2025\")","full_pattern":null,"date_separator":".","month_names":["styczeń","luty","marzec","kwiecień","maj","czerwiec","lipiec","sierpień","wrzesień","październik","listopad","grudzień"],"month_abbrev":[],"day_names":["poniedziałek","wtorek","środa","czwartek","piątek","sobota","niedziela"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (24-hour format)","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"PLN (Polski złoty)","symbol":"zł","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 zł"},{"input":"1234.56","output":"1 234,56 zł"},{"input":"0.99","output":"0,99 zł"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"ul. [Street Name] [Number]/[Apartment]\n[Postal Code] [City]\nPOLAND","postal_code_pattern":"NN-NNN (2 digits, hyphen, 3 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["ul. Marszałkowska 142/5\n00-061 Warszawa\nPOLAND"],"postal_code_examples":["00-061"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| European standard (210×297mm) |","Poland uses metric system exclusively","Imperial units rarely used except in specialized contexts (aviation, some technical fields)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","number":"^-?\\d{1,3}( \\d{3})*,\\d+$","currency":"^\\d{1,3}( \\d{3})*,\\d{2} zł$","phone (national)":"^\\d{2} \\d{3} \\d{2} \\d{2}$","phone (international)":"^\\+48 \\d{2} \\d{3} \\d{2} \\d{2}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/pl-PL.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'zh-SG'})
SET f.display_name = 'zh-SG Formatting',
    f.content = 'Formatting rules for zh-SG',
    f.llm_context = 'zh-SG: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY年M月D日 (gregorian) Time: 12-hour Currency: S$ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY年M月D日","short_pattern":"DD/MM/YYYY","long_pattern":"YYYY年M月D日 星期X","full_pattern":null,"date_separator":"/","month_names":["1月  # yī yuè","2月  # èr yuè","3月  # sān yuè","4月  # sì yuè","5月  # wǔ yuè","6月  # liù yuè","7月  # qī yuè","8月  # bā yuè","9月  # jiǔ yuè","10月  # shí yuè","11月  # shíyī yuè","12月  # shí\'èr yuè"],"month_abbrev":[],"day_names":["星期一  # xīngqī yī","星期二  # xīngqī èr","星期三  # xīngqī sān","星期四  # xīngqī sì","星期五  # xīngqī wǔ","星期六  # xīngqī liù","星期日  # xīngqī rì (also 星期天)"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"上午/下午h:mm","pattern_with_seconds":"上午/下午h:mm:ss","time_separator":":","am_indicator":"上午","pm_indicator":"下午","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"SGD","symbol":"S$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Unit Number, Building Name (optional)\nBlock/House Number Street Name\n#Floor-Unit (for HDB/condos)\nSingapore Postal Code\nSINGAPORE","postal_code_pattern":"NNNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["淡滨尼42街456座\n#12-345\n新加坡520456\n新加坡","Blk 456 Tampines Street 42\n#12-345\nSingapore 520456\nSINGAPORE"],"postal_code_examples":["520456"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Singapore uses metric system exclusively for all official purposes","Road signs display distances in kilometres (公里)","Fuel sold in litres (升), fuel economy in km/L","Body weight typically expressed in kilograms (公斤)","Traditional Chinese units (斤, jīn ≈ 0.6kg in Singapore) occasionally used in wet markets"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["32°C","28°C"]}',
    f.validation_patterns = '{"date (chinese)":"^\\d{4}年\\d{1,2}月\\d{1,2}日$","phone (national)":"^[689]\\d{3}\\s?\\d{4}$","date (numeric)":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","currency":"^S?\\$[\\d,]+(\\.\\d{2})?$","time (24h)":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","number":"^-?[\\d,]+(\\.\\d+)?$","phone (international)":"^\\+65\\s?[689]\\d{3}\\s?\\d{4}$","time (chinese)":"^[上下]午\\d{1,2}:\\d{2}(:\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/zh-SG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-CA'})
SET f.display_name = 'en-CA Formatting',
    f.content = 'Formatting rules for en-CA',
    f.llm_context = 'en-CA: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY-MM-DD (official Canadian standard) (gregorian) Time: 12-hour (common in everyday use), 24-hour (official and technical) Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY-MM-DD (official Canadian standard)","short_pattern":"YYYY-MM-DD","long_pattern":"MMMM D, YYYY","full_pattern":null,"date_separator":"-","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"2025-01-15"},{"input":"2025-12-31","output":"2025-12-31"},{"input":"2025-07-01","output":"2025-07-01"},{"input":"2025-11-11","output":"2025-11-11"},{"input":"2025-12-25","output":"2025-12-25"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour (common in everyday use), 24-hour (official and technical)","pattern":"h:mm a (12-hour) or HH:mm (24-hour)","pattern_with_seconds":"h:mm:ss a (12-hour) or HH:mm:ss (24-hour)","time_separator":":","am_indicator":"a.m.","pm_indicator":"p.m.","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 p.m."},{"input":"09:00","output":"9:00 a.m."},{"input":"23:59:59","output":"11:59:59 p.m."},{"input":"00:00","output":"12:00 a.m."},{"input":"12:00","output":"12:00 p.m."},{"input":"17:45","output":"5:45 p.m."}],"incorrect_examples":[]}',
    f.currency = '{"code":"CAD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"$10.50"},{"input":"1234.56","output":"$1,234.56"},{"input":"0.99","output":"$0.99"},{"input":"1000000","output":"$1,000,000.00"},{"input":"5.00","output":"$5.00"},{"input":"99.95","output":"$99.95"},{"input":"10.50","output":"C$10.50"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name] [Unit] (optional)\n[City] [Province] [Postal Code]\nCANADA","postal_code_pattern":"ANA NAN (alternating letter-number-letter space number-letter-number)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["123 Main Street\nToronto ON  M5V 3A8\nCANADA","456 King Street West, Unit 1205\nToronto ON  M5V 1M3\nCANADA","150 Wellington Street\nOttawa ON  K1A 0A9\nCANADA"],"postal_code_examples":["M5V 3A8"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| 8.5\" x 11\" (same as US, not A4) |","Canada officially uses the metric system since 1970","Imperial units still common in everyday speech (feet, pounds, Fahrenheit for oven)","Road signs: kilometres and km/h","Body weight: people often use pounds informally","Height: feet and inches still common informally (e.g., 5\'10\")","Cooking: mix of metric and imperial (cups, tablespoons common)","Construction: imperial measurements still common (lumber, plumbing)","Exception: aviation uses feet and nautical miles worldwide"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","0°C","37°C","100°C","-40°C"]}',
    f.validation_patterns = '{"time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (a\\.m\\.\\|p\\.m\\.)$","phone (international)":"^\\+1 [0-9]{3} [0-9]{3} [0-9]{4}$","time (24h)":"^([01][0-9]\\|2[0-3]):[0-5][0-9]$","date":"^\\d{4}-(0[1-9]\\|1[0-2])-(0[1-9]\\|[12][0-9]\\|3[01])$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","phone (national)":"^\\([0-9]{3}\\) [0-9]{3}-[0-9]{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-CA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-AE'})
SET f.display_name = 'en-AE Formatting',
    f.content = 'Formatting rules for en-AE',
    f.llm_context = 'en-AE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: AED or Dhs before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM","pm_indicator":"PM","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"AED","symbol":"AED or Dhs","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":"Fils (1 AED = 100 fils)","correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building Name/Number], [Street Name]\n[District/Area]\n[Emirate]\nP.O. Box [NUMBER]\nUNITED ARAB EMIRATES","postal_code_pattern":"None - UAE does not use postal codes","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"PO Box","example_addresses":["Office 2501, Emirates Towers\nSheikh Zayed Road\nDubai\nP.O. Box 54321\nUNITED ARAB EMIRATES","Villa 15, Al Bateen Area\nCorniche Road West\nAbu Dhabi\nUNITED ARAB EMIRATES","Dubai Mall, Ground Floor\nFinancial Centre Road\nDowntown Dubai, Dubai\nMakani: 1234567890\nUNITED ARAB EMIRATES"],"postal_code_examples":["P.O. Box 123456"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","UAE uses metric system for all official purposes","Temperature always in Celsius (summer highs 45-50C, winter 15-25C)","Distance in kilometres (Dubai to Abu Dhabi: 140 km)","Real estate: Square feet commonly used alongside square metres (Dubai convention)","Gold: Measured in grams and tolas (1 tola = 11.66 grams) at Gold Souk","Oil industry: Uses barrels (international oil standard)","Aviation: Feet for altitude, nautical miles for distance (international standards)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^(AED\\|Dhs)\\s[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","phone (national)":"^0[0-9]{2} [0-9]{3} [0-9]{4}$","makani":"^\\d{10}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (AM\\|PM)$","phone (international)":"^\\+971 [0-9]{2} [0-9]{3} [0-9]{4}$","alphanumeric":"201/2025/1234567","trn (tax)":"^\\d{15}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-AE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-CY'})
SET f.display_name = 'en-CY Formatting',
    f.content = 'Formatting rules for en-CY',
    f.llm_context = 'en-CY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (e.g., 15/01/2025) (gregorian) Time: 12-hour (casual) / 24-hour (official) Currency: EUR after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (e.g., 15/01/2025)","short_pattern":"D/M/YY (e.g., 15/1/25)","long_pattern":"D MMMM YYYY (e.g., 15 January 2025)","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour (casual) / 24-hour (official)","pattern":"h:mm a (12-hour) or HH:mm (24-hour)","pattern_with_seconds":"h:mm:ss a (12-hour) or HH:mm:ss (24-hour)","time_separator":":","am_indicator":"am","pm_indicator":"pm","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"EUR","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Number] [Street Name]\n[City] [Postal Code]\nCYPRUS","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["45 Grivas Digenis Avenue\nNicosia 1066\nCYPRUS","Flat 5, 78 Kennedy Avenue\nLimassol 3106\nCYPRUS","12 Stasicratous Street\nNicosia 1065\nCYPRUS"],"postal_code_examples":["1065"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Cyprus uses the metric system exclusively for all measurements","Temperature always in Celsius","Road distances in kilometres, height in metres","Exception: aviation uses feet and nautical miles worldwide","Legacy: some older Cypriots may reference British imperial units informally"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","12.5","99.9"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+357\\s?(22\\|23\\|24\\|25\\|26\\|94\\|95\\|96\\|97\\|99)\\s?\\d{6}$","time (12h)":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (am\\|pm)$","time (24h)":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^-?[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}\\s?EUR$","phone (national)":"^(22\\|23\\|24\\|25\\|26\\|94\\|95\\|96\\|97\\|99)\\s?\\d{6}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-CY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ru-RU'})
SET f.display_name = 'ru-RU Formatting',
    f.content = 'Formatting rules for ru-RU',
    f.llm_context = 'ru-RU: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: ₽ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY г.","full_pattern":null,"date_separator":".","month_names":["января","февраля","марта","апреля","мая","июня","июля","августа","сентября","октября","ноября","декабря"],"month_abbrev":[],"day_names":["понедельник","вторник","среда","четверг","пятница","суббота","воскресенье"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"RUB","symbol":"₽","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 ₽"},{"input":"1234.56","output":"1 234,56 ₽"},{"input":"0.99","output":"0,99 ₽"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name], д. [Building], кв. [Apartment]\nг. [City], [Postal Code]\n[Region/Oblast]\nРОССИЯ","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["улица Тверская, д. 12, кв. 34\nг. Москва, 125009\nМосковская область\nРОССИЯ"],"postal_code_examples":["101000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["| Standard European |","Russia uses metric system exclusively for all measurements","Temperature always in Celsius","Road distances in kilometers, short distances in meters"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","time":"^([01]\\d|2[0-3]):[0-5]\\d$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? ₽$","phone (international)":"^\\+7 \\d{3} \\d{3}-\\d{2}-\\d{2}$","phone (national)":"^8 \\(\\d{3}\\) \\d{3}-\\d{2}-\\d{2}$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ru-RU.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'mi-NZ'})
SET f.display_name = 'mi-NZ Formatting',
    f.content = 'Formatting rules for mi-NZ',
    f.llm_context = 'mi-NZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour (common speech) / 24-hour (digital/transport) Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Kohi-tatea","Hui-tanguru","Poutu-te-rangi","Paenga-whawha","Haratua","Pipiri","Hongongoi","Here-turi-koka","Mahuru","Whiringa-a-nuku","Whiringa-a-rangi","Hakihea"],"month_abbrev":[],"day_names":["Rahina","Ratu","Raapa","Rapare","Ramere","Rahoroi","Ratapu"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour (common speech) / 24-hour (digital/transport)","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"am","pm_indicator":"pm","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"NZD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Unit/Flat] (optional)\n[Number] [Street Name] [Street Type]\n[Suburb]\n[City] [Postcode]\nAOTEAROA NEW ZEALAND","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"PO Box","example_addresses":["123 Queen Street\nAuckland Central\nTamaki Makaurau 1010\nAOTEAROA NEW ZEALAND","Flat 5\n456 Lambton Quay\nWellington Central\nTe Whanganui-a-Tara 6011\nAOTEAROA NEW ZEALAND","789 Victoria Street\nKirikiriroa 3204\nAOTEAROA NEW ZEALAND","PO Box 1234\nOtautahi 8140\nAOTEAROA NEW ZEALAND","123 Smith Road\nRD 3\nWhangarei 0173\nAOTEAROA NEW ZEALAND"],"postal_code_examples":["1010"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| 210mm x 297mm (not US Letter) |","Aotearoa uses metric system exclusively (adopted 1970s)","Imperial units only for informal/colloquial use (e.g., \"6 foot tall\")","Heights sometimes given in cm and feet/inches colloquially","Fuel economy: litres per 100 km (L/100km)","Cooking: metric cups (250 mL), tablespoons (15 mL), teaspoons (5 mL)","Pool temperatures and weather always in Celsius","Land area: hectares (ha) for farms, square metres for residential"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (international)":"^\\+64 [2-9]?[0-9]? [0-9]{3} [0-9]{4}$","phone (landline)":"^0[3-9] [0-9]{3} [0-9]{4}$","phone (mobile)":"^02[1279] [0-9]{3} [0-9]{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (am\\|pm)$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency":"^\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/mi-NZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-PE'})
SET f.display_name = 'es-PE Formatting',
    f.content = 'Formatting rules for es-PE',
    f.llm_context = 'es-PE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: S/ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"a. m.","pm_indicator":"p. m.","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PEN","symbol":"S/","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Type] [Street Name] [Number] [Interior/Apt]\n[District]\n[City] [Postal Code]\nPERU","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Av. Larco 1234 Int. 5B\nMiraflores\nLima 15074\nPERU"],"postal_code_examples":["15001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| A4 format (210 mm x 297 mm) standard |","Metric system is official and standard throughout Peru","Paper size uses A4 format (European standard)","Gas stations sell fuel by gallon (US gallon) despite metric system for other uses","Speed limits shown in km/h","Agricultural land often measured in hectares (ha)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (international)":"^\\+51 \\d{3} \\d{3} \\d{3}$","number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","date":"^\\d{2}/\\d{2}/\\d{4}$","currency":"^S/ \\d{1,3}(,\\d{3})*\\.\\d{2}$","phone (mobile)":"^\\d{3} \\d{3} \\d{3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-PE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'yo-NG'})
SET f.display_name = 'yo-NG Formatting',
    f.content = 'Formatting rules for yo-NG',
    f.llm_context = 'yo-NG: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: ₦ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"Ọjọ́ D Oṣù MMMM, Ọdún YYYY","full_pattern":null,"date_separator":"/","month_names":["Oṣù Kínní","Oṣù Kejì","Oṣù Kẹta","Oṣù Kẹrin","Oṣù Karùn","Oṣù Kẹfà","Oṣù Keje","Oṣù Kẹjọ","Oṣù Kẹsàn","Oṣù Kẹwà","Oṣù Kọkànlá","Oṣù Kejìlá"],"month_abbrev":[],"day_names":["Ọjọ́ Ajé","Ọjọ́ Ìṣẹ́gun","Ọjọ́rú","Ọjọ́bọ","Ọjọ́ Ẹtì","Ọjọ́ Àbámẹ́ta","Ọjọ́ Àìkú"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"Àárọ̀","pm_indicator":"Ọsán","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"NGN","symbol":"₦","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Nọ́mbà Ilé, Ojú Ọ̀nà (House Number, Street Name)\nÀdúgbò (Area/District)\nÌlú (City)\nÌpínlẹ̀ (State)\nKóòdù Pósítì (Postal Code)\nNÀÌJÍRÍÀ","postal_code_pattern":"NNNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Nọ́mbà 25 Ojú Ọ̀nà Broad\nMarina\nErékùṣù Èkó\nÌpínlẹ̀ Èkó\n101001\nNÀÌJÍRÍÀ"],"postal_code_examples":["900001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Metric system is the official standard for all measurements","Distances on road signs are in kilometres","Fuel is sold and priced in litres","Traditional Yoruba measurement terms exist but metric is used for official purposes","Body weight expressed in kilograms officially; informal use may include local terms"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?[\\d,]+(\\.\\d+)?$","phone (national)":"^0[7-9][0-1][0-9]{8}$","phone (international)":"^\\+234\\s?[7-9][0-1][0-9]{8}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])/(0[1-9]\\|1[0-2])/\\d{4}$","currency":"^₦[\\d,]+(\\.\\d{2})?$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9]\\s?(Àárọ̀\\|Ọsán\\|Alẹ́\\|AM\\|PM)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/yo-NG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'tk-TM'})
SET f.display_name = 'tk-TM Formatting',
    f.content = 'Formatting rules for tk-TM',
    f.llm_context = 'tk-TM: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (day.month.year) (gregorian) Time: 24-hour Currency: m (or man.) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY (day.month.year)","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":".","month_names":["Ýanwar","Fewral","Mart","Aprel","Maý","Iýun","Iýul","Awgust","Sentýabr","Oktýabr","Noýabr","Dekabr"],"month_abbrev":[],"day_names":["Duşenbe","Sişenbe","Çarşenbe","Penşenbe","Anna","Şenbe","Ýekşenbe"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"TMT (Turkmenistani Manat)","symbol":"m (or man.)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 m"},{"input":"1234.56","output":"1 234,56 m"},{"input":"0.99","output":"0,99 m"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name], [Building Number]\n[Postal Code] [City]\n[Region/Welayat]\nTURKMENISTAN","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Bitarap Turkmenistan sayoly, 17\n744000 Ashgabat\nAhal welayaty\nTURKMENISTAN"],"postal_code_examples":["744000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard international size (210x297mm) |","Turkmenistan uses the metric system exclusively","All official documents and signage use metric units"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","phone (international)":"^\\+993\\s\\d{2}\\s\\d{6}$","number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","currency":"^\\d{1,3}(\\s\\d{3})*(,\\d{2})?\\sm$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","phone (national)":"^8\\s\\d{2}\\s\\d{6}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/tk-TM.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-PY'})
SET f.display_name = 'es-PY Formatting',
    f.content = 'Formatting rules for es-PY',
    f.llm_context = 'es-PY: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: Gs. or ₲ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"(not commonly used)","pm_indicator":"(not commonly used)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"},{"input":"20:00","output":"20:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"PYG","symbol":"Gs. or ₲","symbol_position":"before","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"1000","output":"Gs. 1.000"},{"input":"50000","output":"Gs. 50.000"},{"input":"3500000","output":"Gs. 3.500.000"},{"input":"500","output":"Gs. 500"},{"input":"150000","output":"Gs. 150.000"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Barrio/Neighborhood]\n[Postal Code] [City]\n[Departamento]\nPARAGUAY","postal_code_pattern":"NNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Avenida Espana 2028\nBarrio Sajonia\n1209 Asuncion\nCentral\nPARAGUAY","Calle Presidente Franco 780\nCentro\n1001 Asuncion\nCentral\nPARAGUAY"],"postal_code_examples":["1001"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard (European influence) |","Metric system is official and strictly used","No imperial units in common use","Land area often measured in hectares (ha) for agriculture (soy, cattle)","\"Leguas\" (leagues) sometimes used informally in rural areas","Height measured in meters and centimeters (e.g., 1,75 m)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (national)":"^\\(0\\d{2,3}\\)\\s\\d{3}\\s\\d{3}$","phone (international)":"^\\+595\\s\\d{2,3}\\s\\d{3}\\s\\d{3}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","currency":"^Gs\\.\\s\\d{1,3}(\\.\\d{3})*$","date":"^\\d{2}/\\d{2}/\\d{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-PY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ha-NG'})
SET f.display_name = 'ha-NG Formatting',
    f.content = 'Formatting rules for ha-NG',
    f.llm_context = 'ha-NG: Numbers use \'.\' decimal, \',\' thousands. Dates: D/M/YYYY (gregorian) Time: 24-hour Currency: ₦ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"D/M/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM, YYYY","full_pattern":null,"date_separator":"/","month_names":["Janairu","Faburairu","Maris","Afirilu","Mayu","Yuni","Yuli","Agusta","Satumba","Oktoba","Nuwamba","Disamba"],"month_abbrev":[],"day_names":["Litinin","Talata","Laraba","Alhamis","Jumma\'a","Asabar","Lahadi"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"Safiya","pm_indicator":"Yamma","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"NGN","symbol":"₦","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Lamba/Plot Number Sunan Titi\nUnguwa/Yanki\nBirni\nJiha\nLambar Wasiƙa\nNIJERIYA","postal_code_pattern":"NNNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Lamba 25 Murtala Muhammad Way\nNassarawa\nKano\nKano State\n700001\nNIJERIYA"],"postal_code_examples":["700001"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilomita","symbol":"km","notes":null},{"category":"Weight","unit":"Kilogram","symbol":"kg","notes":null},{"category":"Volume","unit":"Lita","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Ma\'aunin ISO 216 |","Nigeria tana amfani da tsarin metric a hukumance","Nisantar hanya ana nuna ta da kilomita","Ana sayar da man fetur da lita","Nauyin jiki ana iya bayyana shi da kilogram ko fam a magana"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^([01]?[0-9]\\|2[0-3]):[0-5][0-9]$","phone (national)":"^0[7-9][0-1][0-9]{8}$","date":"^([1-9]\\|[12][0-9]\\|3[01])/([1-9]\\|1[0-2])/\\d{4}$","currency":"^₦[\\d,]+(\\.\\d{2})?$","phone (international)":"^\\+234\\s?[7-9][0-1][0-9]{8}$","number":"^-?[\\d,]+(\\.\\d+)?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ha-NG.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'el-CY'})
SET f.display_name = 'el-CY Formatting',
    f.content = 'Formatting rules for el-CY',
    f.llm_context = 'el-CY: Numbers use \'.\' decimal, \',\' thousands. Dates: d/M/yyyy (e.g., 15/1/2025) (gregorian) Time: 24-hour Currency: € after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d/M/yyyy (e.g., 15/1/2025)","short_pattern":"d/M/yy (e.g., 15/1/25)","long_pattern":"d MMMM yyyy (e.g., 15 Ιανουαρίου 2025)","full_pattern":null,"date_separator":"/","month_names":["Ιανουάριος","Φεβρουάριος","Μάρτιος","Απρίλιος","Μάιος","Ιούνιος","Ιούλιος","Αύγουστος","Σεπτέμβριος","Οκτώβριος","Νοέμβριος","Δεκέμβριος"],"month_abbrev":[],"day_names":["Δευτέρα","Τρίτη","Τετάρτη","Πέμπτη","Παρασκευή","Σάββατο","Κυριακή"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/1/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss (e.g., 14:30:45)","time_separator":":","am_indicator":"π.μ. (not commonly used)","pm_indicator":"μ.μ. (not commonly used)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"EUR","symbol":"€","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 €"},{"input":"1234.56","output":"1.234,56 €"},{"input":"0.99","output":"0,99 €"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Postal Code] [City]\nCYPRUS","postal_code_pattern":"NNNN (4 digits - NOT 5 like Greece)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Λεωφόρος Αρχιεπισκόπου Μακαρίου ΙΙΙ 123\n1065 Λευκωσία\nCYPRUS","Λεωφόρος 28ης Οκτωβρίου 45\n4002 Λεμεσός\nCYPRUS","Οδός Ποσειδώνος 78\n8042 Πάφος\nCYPRUS"],"postal_code_examples":["1065"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Cyprus uses metric system for all official measurements","Temperature always in Celsius","Road distances in kilometers (changed from miles post-independence)","**British Colonial Legacy**: LEFT-HAND TRAFFIC (only EU country), Type G UK 3-pin sockets (230V, 50Hz)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","12,5","99,9"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","-5°C"]}',
    f.validation_patterns = '{"phone (international)":"^\\+357\\s?(22|23|24|25|26|94|95|96|97|98|99)\\s?\\d{6}$","date":"^\\d{1,2}/\\d{1,2}/\\d{4}$","time":"^([01]\\d|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","currency":"^-?\\d{1,3}(\\.\\d{3})*(,\\d{2})?\\s?€$","phone (national)":"^(22|23|24|25|26|94|95|96|97|98|99)\\s?\\d{6}$","tin (cyprus)":"^[0-9]{8}[A-Z]$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/el-CY.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'jv-ID'})
SET f.display_name = 'jv-ID Formatting',
    f.content = 'Formatting rules for jv-ID',
    f.llm_context = 'jv-ID: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 24-hour Currency: Rp before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Januari","Februari","Maret","April","Mei","Juni","Juli","Agustus","September","Oktober","November","Desember"],"month_abbrev":[],"day_names":["Senin","Selasa","Rebo","Kemis","Jemuwah","Setu","Minggu"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"24-hour","pattern":"HH.mm","pattern_with_seconds":"HH.mm.ss","time_separator":".","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"IDR","symbol":"Rp","symbol_position":"before","space_between":true,"decimal_places":20,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Street Address with Number]\n[District/Subdistrict], [City] [Postal Code]\n[Province]\nINDONESIA","postal_code_pattern":"NNNNN (5 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Bambang Suryanto\nJl. Malioboro No. 45\nSosromenduran, Yogyakarta 55271\nDaerah Istimewa Yogyakarta\nINDONESIA"],"postal_code_examples":["55271"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Indonesia uses the metric system exclusively for official and commercial purposes","Imperial units may appear in informal contexts due to international influence, but metric is always primary"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^\\d{2}\\.\\d{2}(:\\d{2})?$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","phone (international)":"^\\+62\\s?\\d{2,3}\\s?\\d{4}\\s?\\d{4,5}$","phone (national)":"^0\\d{2,3}\\s?\\d{4}\\s?\\d{4,5}$","currency":"^Rp\\d{1,3}(\\.\\d{3})*(,\\d{2})?$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/jv-ID.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'km-KH'})
SET f.display_name = 'km-KH Formatting',
    f.content = 'Formatting rules for km-KH',
    f.llm_context = 'km-KH: Numbers use \'.\' decimal, \',\' thousands. Dates: d/M/yyyy (gregorian) Time: 24-hour Currency: ៛ after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"d/M/yyyy","short_pattern":"d/M/yy","long_pattern":"d MMMM yyyy","full_pattern":null,"date_separator":"/","month_names":["មករា","កុម្ភៈ","មីនា","មេសា","ឧសភា","មិថុនា","កក្កដា","សីហា","កញ្ញា","តុលា","វិច្ឆិកា","ធ្នូ"],"month_abbrev":[],"day_names":["ថ្ងៃច័ន្ទ","ថ្ងៃអង្គារ","ថ្ងៃពុធ","ថ្ងៃព្រហស្បតិ៍","ថ្ងៃសុក្រ","ថ្ងៃសៅរ៍","ថ្ងៃអាទិត្យ"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2026-01-15","output":"15/1/2026"},{"input":"2026-12-31","output":"31/12/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"15.01.2026"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"ព្រឹក","pm_indicator":"ល្ងាច","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"14:30","output":"2:30 ល្ងាច"},{"input":"09:00","output":"9:00 ព្រឹក"}],"incorrect_examples":[]}',
    f.currency = '{"code":"KHR","symbol":"៛","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[House Number], [Street Name/Number]\n[Sangkat (សង្កាត់)]\n[Khan/District (ខណ្ឌ)] [City/Province]\n[Postal Code]\nCAMBODIA","postal_code_pattern":"NNNNN","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["#123, St. 271\nSangkat Toul Tompong\nKhan Chamkar Mon, Phnom Penh\n12305\nCAMBODIA","#១២៣, ផ្លូវលេខ ២៧១\nសង្កាត់ទួលទំពូង\nខណ្ឌចំការមន រាជធានីភ្នំពេញ\n១២៣០៥\nកម្ពុជា"],"postal_code_examples":["12305"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| Standard paper size |","Cambodia uses metric system exclusively for official measurements","Land area traditionally measured in hectares and ares (១ហិកតា = 10,000 m2)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (national)":"^0\\d{2}\\s\\d{3}\\s\\d{3,4}$","number":"^-?\\d{1,3}(\\.\\d{3})*(,\\d+)?$","currency (usd)":"^\\$\\d{1,3}(,\\d{3})*\\.\\d{2}$","currency (khr)":"^\\d{1,3}(\\.\\d{3})*៛$","date":"^\\d{1,2}/\\d{1,2}/\\d{4}$","phone (international)":"^\\+855\\s\\d{2}\\s\\d{3}\\s\\d{3,4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/km-KH.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'so-SO'})
SET f.display_name = 'so-SO Formatting',
    f.content = 'Formatting rules for so-SO',
    f.llm_context = 'so-SO: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour (common), 24-hour (official/military) Currency: S or Sh after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["Janaayo","Febraayo","Maarso","Abriil","May","Juun","Luuliyo","Ogost","Sebtembar","Oktoobar","Nofembar","Desembar"],"month_abbrev":[],"day_names":["Isniin","Talaado","Arbaco","Khamiis","Jimco","Sabti","Axad"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"},{"input":"2025-07-01","output":"01/07/2025"},{"input":"2025-06-26","output":"26/06/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-01-15","output":"January 15, 2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"12-hour (common), 24-hour (official/military)","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"GH (Galabnimo Hore) or subaxnimo","pm_indicator":"GD (Galabnimo Dambe) or galabnimo","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 GD"},{"input":"09:00","output":"9:00 GH"},{"input":"23:59:59","output":"11:59:59 GD"},{"input":"00:00","output":"12:00 GH"},{"input":"12:00","output":"12:00 GD"},{"input":"17:45","output":"5:45 GD"}],"incorrect_examples":[]}',
    f.currency = '{"code":"SOS","symbol":"S or Sh","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10.50 S"},{"input":"1234.56","output":"1,234.56 S"},{"input":"0.99","output":"0.99 S"},{"input":"1000000","output":"1,000,000 S"},{"input":"5.00","output":"5.00 S"},{"input":"99.95","output":"99.95 S"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name/Number or Building Name]\n[Neighborhood/District]\n[City]\n[Region/State]\nSOMALIA","postal_code_pattern":"None (Somalia does not have a postal code system)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Villa 23, Wadada Makka Al-Mukarama\nHodan\nMuqdisho\nBanaadir\nSOMALIA","Suuqa Bakaaraha\nWardhiigley\nMuqdisho\nBanaadir\nSOMALIA","Guri No. 45, Xaafadda Waberi\nWaberi\nMuqdisho\nBanaadir\nSOMALIA"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210mm x 297mm (international standard) |","Somalia uses the metric system (inherited from colonial period)","Distance on road signs: kilometers","Speed limits: km/h (common speeds: 50, 60, 80, 100)","Body weight: kilograms","Height: centimeters or meters (e.g., 1.75 m)","Exception: aviation uses feet and nautical miles worldwide","Traditional measures exist but metric is official","Land area: hectares or square kilometers"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["30°C","25°C","40°C","100°C","0°C"]}',
    f.validation_patterns = '{"date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","phone (international)":"^\\+252 [1-9][0-9]? [0-9]{3} [0-9]{3,4}$","phone (national)":"^[1-9][0-9] [0-9]{3} [0-9]{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (GH\\|GD)$","currency":"^[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]{2})? (S\\|Sh)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/so-SO.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-AE'})
SET f.display_name = 'ar-AE Formatting',
    f.content = 'Formatting rules for ar-AE',
    f.llm_context = 'ar-AE: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour (general) / 24-hour (business/transport) Currency: د.إ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["يناير","فبراير","مارس","أبريل","مايو","يونيو","يوليو","أغسطس","سبتمبر","أكتوبر","نوفمبر","ديسمبر"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour (general) / 24-hour (business/transport)","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"AM / ص","pm_indicator":"PM / م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 PM"},{"input":"09:00","output":"9:00 AM"},{"input":"22:00","output":"10:00 PM"}],"incorrect_examples":[]}',
    f.currency = '{"code":"AED","symbol":"د.إ","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"12500","output":"AED 12,500"},{"input":"3.75","output":"AED 3.75"},{"input":"1850000","output":"AED 1,850,000"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Building Name/Number], [Street Name]\n[Area/Community]\n[Emirate]\nUnited Arab Emirates","postal_code_pattern":"None","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"ص.ب.","example_addresses":["Emirates Tower Offices\nLevel 41, Emirates Office Tower\nSheikh Zayed Road, DIFC\nDubai\nUnited Arab Emirates","Etihad Towers\nTower 2, Apartment 2305\nCorniche Road\nAbu Dhabi\nUnited Arab Emirates","Makani: 3374297834\nAl Wasl Road, Jumeirah\nDubai, UAE","ABC Company\nP.O. Box 12345\nDubai\nUnited Arab Emirates"],"postal_code_examples":[]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","UAE uses metric system for official purposes","Temperature: Celsius (°C) - summer regularly 40-50°C","Distance: Kilometers for road signs, maps","Real estate: Square feet (sq ft) commonly used alongside square meters","Example: \"1,200 sq ft apartment\" = ~111 m²","Gold: Measured in grams and tola (11.66g)","Oil industry: Barrels, feet (technical contexts)"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["5","5","25","25","٥"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["45°C","45°C","24°C","24°C","18°C","18°C"]}',
    f.validation_patterns = '{"makani":"^\\d{10}$","phone (international)":"^\\+971\\s5[0268]\\s\\d{3}\\s\\d{4}$","time":"^\\d{1,2}:\\d{2}\\s?(AM\\|PM)$","phone (mobile)":"^05[0268]\\s\\d{3}\\s\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?[\\d,]+(\\.\\d+)?$","currency (english)":"^(AED\\|Dhs)\\s[\\d,]+(\\.\\d{2})?$","currency (arabic)":"^[٠-٩,]+(\\٫[٠-٩]{2})?\\sد\\.إ$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-AE.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'en-NZ'})
SET f.display_name = 'en-NZ Formatting',
    f.content = 'Formatting rules for en-NZ',
    f.llm_context = 'en-NZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour (common speech) / 24-hour (digital/transport) Currency: $ before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"D/M/YY","long_pattern":"D MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["January","February","March","April","May","June","July","August","September","October","November","December"],"month_abbrev":[],"day_names":["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour (common speech) / 24-hour (digital/transport)","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"am","pm_indicator":"pm","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"NZD","symbol":"$","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"Unit/Flat (optional)\nStreet Number + Street Name + Street Type\nSuburb\nCity/Town + Postcode\nNEW ZEALAND","postal_code_pattern":"NNNN (4 digits)","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":"PO Box","example_addresses":["123 Queen Street\nAuckland Central\nAuckland 1010\nNEW ZEALAND","45 Lambton Quay\nTe Aro\nTe Whanganui-a-Tara 6011\nNEW ZEALAND","Flat 5\n456 Lambton Quay\nWellington Central\nWellington 6011\nNEW ZEALAND","Level 10, Suite 1002\n789 Victoria Street West\nAuckland CBD\nAuckland 1010\nNEW ZEALAND","PO Box 1234\nChristchurch 8140\nNEW ZEALAND","123 Smith Road\nRD 3\nWhangarei 0173\nNEW ZEALAND","78 Gloucester Street\nChristchurch Central\nChristchurch 8013\nNEW ZEALAND","15 Victoria Avenue\nWhanganui 4500\nNEW ZEALAND"],"postal_code_examples":["1010"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometres","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Litres","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| 210mm x 297mm (not US Letter) |","New Zealand uses metric system exclusively (adopted 1970s)","Imperial units only for informal/colloquial use (e.g., \"6 foot tall\")","Fuel economy: litres per 100 km (L/100km)","Cooking: metric cups (250 mL), tablespoons (15 mL - NOTE: NZ tablespoon is 15 mL, same as UK/US), teaspoons (5 mL)","Pool temperatures: always Celsius","Land area: hectares (ha) for farms, square metres (m2) for residential","Dairy industry: often uses kg for milk solids"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12.5","0.5","50","15","3","5.50"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","phone (mobile)":"^02[1279] [0-9]{3} [0-9]{4}$","phone (landline)":"^0[3-9] [0-9]{3} [0-9]{4}$","phone (international)":"^\\+64 [2-9]?[0-9]? [0-9]{3} [0-9]{4}$","currency":"^\\$[0-9]{1,3}(,[0-9]{3})*\\.[0-9]{2}$","date":"^(0[1-9]\\|[12][0-9]\\|3[01])\\/(0[1-9]\\|1[0-2])\\/\\d{4}$","time":"^(1[0-2]\\|0?[1-9]):[0-5][0-9] (am\\|pm)$","nhi":"^[A-HJ-NP-Z]{3}[0-9]{4}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/en-NZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ne-NP'})
SET f.display_name = 'ne-NP Formatting',
    f.content = 'Formatting rules for ne-NP',
    f.llm_context = 'ne-NP: Numbers use \'.\' decimal, \',\' thousands. Dates: YYYY/MM/DD (gregorian) Time: 12-hour Currency: रु॰ or रू before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"YYYY/MM/DD","short_pattern":"YY/MM/DD","long_pattern":"YYYY MMMM D","full_pattern":null,"date_separator":"/","month_names":["जनवरी","फेब्रुअरी","मार्च","अप्रिल","मे","जुन","जुलाई","अगस्ट","सेप्टेम्बर","अक्टोबर","नोभेम्बर","डिसेम्बर"],"month_abbrev":[],"day_names":["सोमबार","मंगलबार","बुधबार","बिहिबार","शुक्रबार","शनिबार","आइतबार"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"२०२५/०१/१५"},{"input":"2025-12-31","output":"२०२५/१२/३१"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025-01-15"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"बिहान","pm_indicator":"दिउँसो / बेलुका","prayer_times":null,"correct_examples":[{"input":"14:30","output":"२:३० दिउँसो"},{"input":"09:00","output":"९:०० बिहान"},{"input":"23:59:59","output":"११:५९:५९ राति"}],"incorrect_examples":[]}',
    f.currency = '{"code":"NPR","symbol":"रु॰ or रू","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"रु॰ १०.५०"},{"input":"1234.56","output":"रु॰ १,२३४.५६"},{"input":"0.99","output":"रु॰ ०.९९"},{"input":"100000","output":"रु॰ १,००,०००"},{"input":"10000000","output":"रु॰ १,००,००,०००"},{"input":"750000.50","output":"रु॰ ७,५०,०००.५०"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Name]\n[Building/House Number], [Street/Tole Name]\n[Ward Number], [Municipality/VDC]\n[District]\n[Province] - [Postal Code]\nNEPAL","postal_code_pattern":"NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["श्री राम बहादुर श्रेष्ठ\nवडा नं. १२, बागबजार\nकाठमाडौं महानगरपालिका\nकाठमाडौं जिल्ला\nबागमती प्रदेश - ४४६००\nNEPAL"],"postal_code_examples":["४४६००"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km (कि.मि.)","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg (के.जी.)","notes":null},{"category":"Volume","unit":"Liters","symbol":"L (लि.)","notes":null}],"paper_size":"A4","notes":["| 210 × 297 mm |","Nepal uses metric system officially","Traditional measurements still used locally: ropani/anna for land, mana/pathi for grains","Aviation follows international standards (feet for altitude)"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"currency":"^रु॰\\s[०-९]{1,3}(,[०-९]{2})*,?[०-९]{3}(\\.[०-९]{2})?$","date":"^[०-९]{4}/[०-९]{2}/[०-९]{2}$","phone (international)":"^\\+977\\s\\d{4}\\s\\d{6}$","phone (national)":"^[०-९]{4}\\s[०-९]{6}$","number":"^-?[०-९]{1,3}(,[०-९]{2})*,?[०-९]{3}(\\.[०-९]+)?$","time":"^[०-९]{1,2}:[०-९]{2}\\s?(बिहान\\|दिउँसो\\|बेलुका\\|राति)$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ne-NP.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'fr-DZ'})
SET f.display_name = 'fr-DZ Formatting',
    f.content = 'Formatting rules for fr-DZ',
    f.llm_context = 'fr-DZ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (e.g., 15/01/2026) (hijri) Time: 24-hour (French convention used officially) Currency: DA after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones","ONS Algeria (national statistics"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY (e.g., 15/01/2026)","short_pattern":"DD/MM/YY (e.g., 15/01/26)","long_pattern":"D MMMM YYYY (e.g., 15 janvier 2026)","full_pattern":"EEEE D MMMM YYYY (e.g., mercredi 15 janvier 2026)","date_separator":"/","month_names":["janvier","février","mars","avril","mai","juin","juillet","août","septembre","octobre","novembre","décembre"],"month_abbrev":[],"day_names":["lundi","mardi","mercredi","jeudi","vendredi","samedi","dimanche"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2026-01-15","output":"15/01/2026"},{"input":"2026-12-31","output":"31/12/2026"}],"incorrect_examples":[{"input":"2026-01-15","output":"01/15/2026"},{"input":"2026-01-15","output":"2026-01-15"}]}',
    f.time = '{"system":"24-hour (French convention used officially)","pattern":"HH:mm (e.g., 14:30)","pattern_with_seconds":"HH:mm:ss (e.g., 14:30:45)","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"},{"input":"00:00","output":"00:00"},{"input":"12:00","output":"12:00"}],"incorrect_examples":[]}',
    f.currency = '{"code":"DZD (Dinar algérien)","symbol":"DA","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":"centime (1 DZD = 100 centimes, but rarely used)","correct_examples":[{"input":"1500","output":"1 500 DA"},{"input":"25000","output":"25 000 DA"},{"input":"150000","output":"150 000 DA"},{"input":"1500000","output":"1 500 000 DA"}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Recipient Name]\n[Number] [Street Type] [Street Name]\n[Commune/District]\n[Postal Code] [City]\n[Wilaya Name]\nALGERIE","postal_code_pattern":"NNNNN (5 digits, first 2 = wilaya code)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["M. Karim BENALI\n45 rue Didouche Mourad\nEl Mouradia\n16000 ALGER\nWilaya d\'Alger\nALGERIE","SARL EXEMPLE ALGERIE\nZone Industrielle Es-Senia\nBP 234\n31000 ORAN\nWilaya d\'Oran\nALGERIE"],"postal_code_examples":["16000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null},{"category":"Fuel","unit":"Liters","symbol":"L","notes":null},{"category":"Hectare","unit":"ha","symbol":"Common for agriculture","notes":null}],"paper_size":"A4","notes":["| 210 x 297 mm |","Algeria uses French metric system exclusively","Quintal (100 kg) commonly used for agricultural products","Oil/gas industry uses international (metric) standards","Imperial units not used in any context"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":["50","12,5","100"]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":["35°C","45°C","10°C","0°C"]}',
    f.validation_patterns = '{"time":"^([01]\\d\\|2[0-3]):[0-5]\\d(:[0-5]\\d)?$","number":"^-?\\d{1,3}( \\d{3})*(,\\d+)?$","nis (social security)":"^\\d{12}$","phone (national)":"^0[5-7]\\d{2}( \\d{2}){3}$","phone (landline)":"^0(2[1-9]\\|3[1-9]\\|4[1-9])( \\d{2}){3}$","date":"^\\d{2}/\\d{2}/\\d{4}$","nif (tax id)":"^\\d{15}$","phone (international)":"^\\+213 [5-7]\\d{2}( \\d{2}){3}$","currency":"^\\d{1,3}( \\d{3})*(,\\d{2})? DA$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/fr-DZ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'ar-LB'})
SET f.display_name = 'ar-LB Formatting',
    f.content = 'Formatting rules for ar-LB',
    f.llm_context = 'ar-LB: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (hijri) Time: 12-hour Currency: ل.ل. after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"DD MMMM YYYY","full_pattern":null,"date_separator":"/","month_names":["كانون الثاني","شباط","آذار","نيسان","أيار","حزيران","تموز","آب","أيلول","تشرين الأول","تشرين الثاني","كانون الأول"],"month_abbrev":[],"day_names":["الاثنين","الثلاثاء","الأربعاء","الخميس","الجمعة","السبت","الأحد"],"day_abbrev":[],"hijri_months":[],"calendar_system":"hijri","correct_examples":[{"input":"2025-01-15","output":"15/01/2025"},{"input":"2025-12-31","output":"31/12/2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"15 يناير 2025"}]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"ص","pm_indicator":"م","prayer_times":null,"correct_examples":[{"input":"14:30","output":"2:30 م"},{"input":"09:00","output":"9:00 ص"},{"input":"23:59:59","output":"11:59:59 م"}],"incorrect_examples":[]}',
    f.currency = '{"code":"LBP","symbol":"ل.ل.","symbol_position":"after","space_between":true,"decimal_places":0,"subunit":null,"correct_examples":[{"input":"10000","output":"10,000 ل.ل."},{"input":"1500000","output":"1,500,000 ل.ل."},{"input":"50000","output":"50,000 ل.ل."}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Building Name/Number] [Street Name]\n[District/Neighborhood]\n[City] [Postal Code]\nلبنان","postal_code_pattern":"NNNN NNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["بناية السلام، شارع الحمرا\nرأس بيروت\nبيروت 1103 2020\nلبنان"],"postal_code_examples":["1103 2020"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"كم","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"كغ","notes":null},{"category":"Volume","unit":"Liters","symbol":"ل","notes":null}],"paper_size":"A4","notes":["| International standard (210 x 297 mm) |","Lebanon uses the metric system for official and daily purposes","Temperature always in Celsius (°C) for weather, medical, cooking","Distance in kilometers (كم) for road signs, maps, distances","Historical French influence: some older measurements (ratl, irdeb) may appear in traditional markets","Colloquial weight unit: \"ratl\" (رطل) = approximately 2.5 kg for produce"]}',
    f.percentage = '{"format":"{number}%","space_before_symbol":false,"examples":["50","50","٥٠","12.5","12.5"]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":["22°C","22°C","-5°C","-5°C"]}',
    f.validation_patterns = '{"time":"^[1-9]|1[0-2]:[0-5][0-9]\\s[صم]$","date":"^\\d{2}/\\d{2}/\\d{4}$","number":"^-?[0-9]{1,3}(,[0-9]{3})*(\\.[0-9]+)?$","currency (lbp)":"^[0-9]{1,3}(,[0-9]{3})*\\sل\\.ل\\.$","phone (national)":"^0[1-9]\\s?\\d{2,3}\\s?\\d{3}$","phone (international)":"^\\+961\\s[1-9]\\s\\d{3}\\s\\d{3}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/ar-LB.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'tg-TJ'})
SET f.display_name = 'tg-TJ Formatting',
    f.content = 'Formatting rules for tg-TJ',
    f.llm_context = 'tg-TJ: Numbers use \'.\' decimal, \',\' thousands. Dates: DD.MM.YYYY (gregorian) Time: 24-hour Currency: сом. (abbreviated) or сомонӣ (full) after amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD.MM.YYYY","short_pattern":"DD.MM.YY","long_pattern":"D MMMM YYYY с.","full_pattern":null,"date_separator":".","month_names":["Январ","Феврал","Март","Апрел","Май","Июн","Июл","Август","Сентябр","Октябр","Ноябр","Декабр"],"month_abbrev":[],"day_names":["Душанбе","Сешанбе","Чоршанбе","Панҷшанбе","Ҷумъа","Шанбе","Якшанбе"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[{"input":"2025-01-15","output":"15.01.2025"},{"input":"2025-12-31","output":"31.12.2025"}],"incorrect_examples":[{"input":"2025-01-15","output":"01/15/2025"},{"input":"2025-01-15","output":"2025/01/15"}]}',
    f.time = '{"system":"24-hour","pattern":"HH:mm","pattern_with_seconds":"HH:mm:ss","time_separator":":","am_indicator":"N/A (24-hour system)","pm_indicator":"N/A (24-hour system)","prayer_times":null,"correct_examples":[{"input":"14:30","output":"14:30"},{"input":"09:00","output":"09:00"},{"input":"23:59:59","output":"23:59:59"}],"incorrect_examples":[]}',
    f.currency = '{"code":"TJS (ISO 4217)","symbol":"сом. (abbreviated) or сомонӣ (full)","symbol_position":"after","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[{"input":"10.50","output":"10,50 сом."},{"input":"1234.56","output":"1 234,56 сом."},{"input":"0.99","output":"0,99 сом."}],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Postal Code], [City/Region]\n[Street Name], [Building Number]\n[Apartment/Office]\nТОҶИКИСТОН","postal_code_pattern":"NNNNNN (6 digits)","postal_code_position":"before_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["734000, ш. Душанбе\nкӯчаи Рӯдакӣ, бинои 37\nхонаи 15\nТОҶИКИСТОН"],"postal_code_examples":["734000"]}',
    f.measurement = '{"system":"metric","units":[{"category":"Temperature","unit":"Celsius","symbol":"°C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"км","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"кг","notes":null},{"category":"Volume","unit":"Liters","symbol":"л","notes":null}],"paper_size":"A4","notes":["| ISO 216 standard |","Always display metric units (Tajikistan uses the metric system exclusively)","Exception: aviation uses feet for altitude as per international standards"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number} °C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"phone (national)":"^\\(\\d{3}\\)\\s?\\d{2}-\\d{2}-\\d{2}$","currency":"^-?\\d{1,3}(\\s\\d{3})*(,\\d{2})?\\s?сом\\.$","number":"^-?\\d{1,3}(\\s\\d{3})*(,\\d+)?$","date":"^\\d{2}\\.\\d{2}\\.\\d{4}$","time":"^\\d{2}:\\d{2}(:\\d{2})?$","phone (international)":"^\\+992\\s?\\d{3}\\s?\\d{2}\\s?\\d{2}\\s?\\d{2}$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/tg-TJ.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

MERGE (f:Formatting {key: 'es-PA'})
SET f.display_name = 'es-PA Formatting',
    f.content = 'Formatting rules for es-PA',
    f.llm_context = 'es-PA: Numbers use \'.\' decimal, \',\' thousands. Dates: DD/MM/YYYY (gregorian) Time: 12-hour Currency: B/. (PAB) / $ (USD) before amount',
    f.data_sources = '["CLDR (primary","ISO 4217 (currency","IANA (timezones"]',
    f.number = '{"decimal_separator":".","thousands_separator":",","negative_sign":"-","positive_sign":"+","grouping_pattern":3,"numeral_system":null,"correct_examples":[],"incorrect_examples":[]}',
    f.date = '{"pattern":"DD/MM/YYYY","short_pattern":"DD/MM/YY","long_pattern":"D de MMMM de YYYY","full_pattern":null,"date_separator":"/","month_names":["enero","febrero","marzo","abril","mayo","junio","julio","agosto","septiembre","octubre","noviembre","diciembre"],"month_abbrev":[],"day_names":["lunes","martes","miercoles","jueves","viernes","sabado","domingo"],"day_abbrev":[],"hijri_months":null,"calendar_system":"gregorian","correct_examples":[],"incorrect_examples":[]}',
    f.time = '{"system":"12-hour","pattern":"h:mm a","pattern_with_seconds":"h:mm:ss a","time_separator":":","am_indicator":"a. m.","pm_indicator":"p. m.","prayer_times":null,"correct_examples":[],"incorrect_examples":[]}',
    f.currency = '{"code":"PAB / USD","symbol":"B/. (PAB) / $ (USD)","symbol_position":"before","space_between":true,"decimal_places":2,"subunit":null,"correct_examples":[],"incorrect_examples":[]}',
    f.phone = '{"country_code":"+1","national_pattern":"","international_pattern":"","mobile_prefixes":[],"landline_prefixes":[],"special_prefixes":null,"digit_count":10,"correct_examples":[],"incorrect_examples":[]}',
    f.address = '{"pattern":"[Street Name] [Number]\n[Neighborhood/Corregimiento]\n[District], [Province]\n[Postal Zone]\nPANAMA","postal_code_pattern":"NNNN-NNNNN","postal_code_position":"after_city","city_format":null,"street_types":null,"po_box_format":null,"example_addresses":["Avenida Balboa, Torre BAC\nPiso 25, Oficina 2501\nMarbella, Panama City\n0816-07567\nPANAMA"],"postal_code_examples":["0816-00000"]}',
    f.measurement = '{"system":"imperial","units":[{"category":"Temperature","unit":"Celsius","symbol":"C","notes":null},{"category":"Distance","unit":"Kilometers","symbol":"km","notes":null},{"category":"Weight","unit":"Kilograms","symbol":"kg","notes":null},{"category":"Volume","unit":"Liters","symbol":"L","notes":null}],"paper_size":"Letter","notes":["| US letter size (8.5\" x 11\") more common than A4 |","Metric system is official and standard throughout Panama","Exception: Paper size commonly uses US Letter format (21.59 cm x 27.94 cm) due to US influence","Due to close ties with the US, imperial measurements are widely understood","Fuel sold by liter at gas stations (gasolineras)","Real estate often quoted in square feet and square meters","Vehicle speedometers show km/h; speed limits in km/h"]}',
    f.percentage = '{"format":"{number} %","space_before_symbol":true,"examples":[]}',
    f.temperature = '{"format":"{number}°C","default_unit":"celsius","examples":[]}',
    f.validation_patterns = '{"number":"^-?\\d{1,3}(,\\d{3})*(\\.\\d+)?$","currency (pab)":"^B/\\.\\d{1,3}(,\\d{3})*\\.\\d{2}$","phone (international)":"^\\+507 \\d{4} \\d{4}$","currency (usd)":"^\\$\\d{1,3}(,\\d{3})*\\.\\d{2}$","phone (national)":"^\\d{4}-\\d{4}$","date":"^\\d{2}/\\d{2}/\\d{4}$","time":"^\\d{1,2}:\\d{2}(:\\d{2})? [ap]\\. m\\.$"}',
    f.template_version = '2.0',
    f.source_file = '2-rules-formatting/es-PA.md',
    f.created_by = 'seed:locale',
    f.created_at = datetime(), f.updated_at = datetime();

// ----------------------------------------------------------------------------
// PART 2: Arcs Locale → Formatting
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'ceb-PH'})
MATCH (f:Formatting {key: 'ceb-PH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'mn-MN'})
MATCH (f:Formatting {key: 'mn-MN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-MY'})
MATCH (f:Formatting {key: 'en-MY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'bn-BD'})
MATCH (f:Formatting {key: 'bn-BD'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-TZ'})
MATCH (f:Formatting {key: 'en-TZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-SG'})
MATCH (f:Formatting {key: 'en-SG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ta-LK'})
MATCH (f:Formatting {key: 'ta-LK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-ZA'})
MATCH (f:Formatting {key: 'en-ZA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-BB'})
MATCH (f:Formatting {key: 'en-BB'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'si-LK'})
MATCH (f:Formatting {key: 'si-LK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'bs-BA'})
MATCH (f:Formatting {key: 'bs-BA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'it-CH'})
MATCH (f:Formatting {key: 'it-CH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'tr-TR'})
MATCH (f:Formatting {key: 'tr-TR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-MX'})
MATCH (f:Formatting {key: 'es-MX'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-MA'})
MATCH (f:Formatting {key: 'ar-MA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-LY'})
MATCH (f:Formatting {key: 'ar-LY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-MG'})
MATCH (f:Formatting {key: 'fr-MG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-EG'})
MATCH (f:Formatting {key: 'ar-EG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'my-MM'})
MATCH (f:Formatting {key: 'my-MM'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ta-IN'})
MATCH (f:Formatting {key: 'ta-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-CD'})
MATCH (f:Formatting {key: 'fr-CD'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'as-IN'})
MATCH (f:Formatting {key: 'as-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'lt-LT'})
MATCH (f:Formatting {key: 'lt-LT'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'sw-KE'})
MATCH (f:Formatting {key: 'sw-KE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'hy-AM'})
MATCH (f:Formatting {key: 'hy-AM'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'pt-MZ'})
MATCH (f:Formatting {key: 'pt-MZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'nl-BE'})
MATCH (f:Formatting {key: 'nl-BE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'te-IN'})
MATCH (f:Formatting {key: 'te-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ml-IN'})
MATCH (f:Formatting {key: 'ml-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'vi-VN'})
MATCH (f:Formatting {key: 'vi-VN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-BE'})
MATCH (f:Formatting {key: 'fr-BE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ru-IL'})
MATCH (f:Formatting {key: 'ru-IL'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'rw-RW'})
MATCH (f:Formatting {key: 'rw-RW'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-KY'})
MATCH (f:Formatting {key: 'en-KY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ky-KG'})
MATCH (f:Formatting {key: 'ky-KG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-IE'})
MATCH (f:Formatting {key: 'en-IE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'su-ID'})
MATCH (f:Formatting {key: 'su-ID'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-UG'})
MATCH (f:Formatting {key: 'en-UG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'sw-TZ'})
MATCH (f:Formatting {key: 'sw-TZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'hu-HU'})
MATCH (f:Formatting {key: 'hu-HU'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-EC'})
MATCH (f:Formatting {key: 'es-EC'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'mg-MG'})
MATCH (f:Formatting {key: 'mg-MG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ca-AD'})
MATCH (f:Formatting {key: 'ca-AD'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ko-KR'})
MATCH (f:Formatting {key: 'ko-KR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ln-CD'})
MATCH (f:Formatting {key: 'ln-CD'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-KE'})
MATCH (f:Formatting {key: 'en-KE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-DZ'})
MATCH (f:Formatting {key: 'ar-DZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-CA'})
MATCH (f:Formatting {key: 'fr-CA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-GB'})
MATCH (f:Formatting {key: 'en-GB'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-RW'})
MATCH (f:Formatting {key: 'fr-RW'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ru-BY'})
MATCH (f:Formatting {key: 'ru-BY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-GT'})
MATCH (f:Formatting {key: 'es-GT'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ms-SG'})
MATCH (f:Formatting {key: 'ms-SG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ht-HT'})
MATCH (f:Formatting {key: 'ht-HT'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ms-MY'})
MATCH (f:Formatting {key: 'ms-MY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'sq-AL'})
MATCH (f:Formatting {key: 'sq-AL'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'mk-MK'})
MATCH (f:Formatting {key: 'mk-MK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'mr-IN'})
MATCH (f:Formatting {key: 'mr-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-CU'})
MATCH (f:Formatting {key: 'es-CU'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ru-KG'})
MATCH (f:Formatting {key: 'ru-KG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ig-NG'})
MATCH (f:Formatting {key: 'ig-NG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ka-GE'})
MATCH (f:Formatting {key: 'ka-GE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-BH'})
MATCH (f:Formatting {key: 'ar-BH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-LU'})
MATCH (f:Formatting {key: 'fr-LU'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-ZW'})
MATCH (f:Formatting {key: 'en-ZW'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-IN'})
MATCH (f:Formatting {key: 'en-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'zh-TW'})
MATCH (f:Formatting {key: 'zh-TW'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'de-CH'})
MATCH (f:Formatting {key: 'de-CH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'pt-BR'})
MATCH (f:Formatting {key: 'pt-BR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fa-IR'})
MATCH (f:Formatting {key: 'fa-IR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'de-AT'})
MATCH (f:Formatting {key: 'de-AT'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-TN'})
MATCH (f:Formatting {key: 'fr-TN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-TT'})
MATCH (f:Formatting {key: 'en-TT'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'is-IS'})
MATCH (f:Formatting {key: 'is-IS'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ca-ES'})
MATCH (f:Formatting {key: 'ca-ES'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ru-KZ'})
MATCH (f:Formatting {key: 'ru-KZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'kn-IN'})
MATCH (f:Formatting {key: 'kn-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ro-RO'})
MATCH (f:Formatting {key: 'ro-RO'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'gu-IN'})
MATCH (f:Formatting {key: 'gu-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'pt-AO'})
MATCH (f:Formatting {key: 'pt-AO'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ny-MW'})
MATCH (f:Formatting {key: 'ny-MW'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'or-IN'})
MATCH (f:Formatting {key: 'or-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'zh-CN'})
MATCH (f:Formatting {key: 'zh-CN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'sn-ZW'})
MATCH (f:Formatting {key: 'sn-ZW'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-JO'})
MATCH (f:Formatting {key: 'ar-JO'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-KW'})
MATCH (f:Formatting {key: 'ar-KW'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-AU'})
MATCH (f:Formatting {key: 'en-AU'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-FR'})
MATCH (f:Formatting {key: 'fr-FR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-SN'})
MATCH (f:Formatting {key: 'fr-SN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'pa-IN'})
MATCH (f:Formatting {key: 'pa-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'cs-CZ'})
MATCH (f:Formatting {key: 'cs-CZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-CL'})
MATCH (f:Formatting {key: 'es-CL'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'nl-NL'})
MATCH (f:Formatting {key: 'nl-NL'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-HN'})
MATCH (f:Formatting {key: 'es-HN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'bg-BG'})
MATCH (f:Formatting {key: 'bg-BG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'sd-PK'})
MATCH (f:Formatting {key: 'sd-PK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-GH'})
MATCH (f:Formatting {key: 'en-GH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-PH'})
MATCH (f:Formatting {key: 'en-PH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'sl-SI'})
MATCH (f:Formatting {key: 'sl-SI'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-HK'})
MATCH (f:Formatting {key: 'en-HK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-MU'})
MATCH (f:Formatting {key: 'en-MU'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'et-EE'})
MATCH (f:Formatting {key: 'et-EE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-SV'})
MATCH (f:Formatting {key: 'es-SV'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'pt-CH'})
MATCH (f:Formatting {key: 'pt-CH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-PR'})
MATCH (f:Formatting {key: 'es-PR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ur-PK'})
MATCH (f:Formatting {key: 'ur-PK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-ZM'})
MATCH (f:Formatting {key: 'en-ZM'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-OM'})
MATCH (f:Formatting {key: 'ar-OM'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ru-MD'})
MATCH (f:Formatting {key: 'ru-MD'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'mt-MT'})
MATCH (f:Formatting {key: 'mt-MT'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'eu-ES'})
MATCH (f:Formatting {key: 'eu-ES'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ja-JP'})
MATCH (f:Formatting {key: 'ja-JP'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'bn-IN'})
MATCH (f:Formatting {key: 'bn-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'he-IL'})
MATCH (f:Formatting {key: 'he-IL'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'zh-HK'})
MATCH (f:Formatting {key: 'zh-HK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-CO'})
MATCH (f:Formatting {key: 'es-CO'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'de-LU'})
MATCH (f:Formatting {key: 'de-LU'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-BO'})
MATCH (f:Formatting {key: 'es-BO'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'th-TH'})
MATCH (f:Formatting {key: 'th-TH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'kk-KZ'})
MATCH (f:Formatting {key: 'kk-KZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fi-FI'})
MATCH (f:Formatting {key: 'fi-FI'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-TN'})
MATCH (f:Formatting {key: 'ar-TN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-VN'})
MATCH (f:Formatting {key: 'en-VN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'tl-PH'})
MATCH (f:Formatting {key: 'tl-PH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-PK'})
MATCH (f:Formatting {key: 'en-PK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-CH'})
MATCH (f:Formatting {key: 'fr-CH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'cy-GB'})
MATCH (f:Formatting {key: 'cy-GB'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-FJ'})
MATCH (f:Formatting {key: 'en-FJ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'af-ZA'})
MATCH (f:Formatting {key: 'af-ZA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-JM'})
MATCH (f:Formatting {key: 'en-JM'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'da-DK'})
MATCH (f:Formatting {key: 'da-DK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-US'})
MATCH (f:Formatting {key: 'en-US'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'pt-PT'})
MATCH (f:Formatting {key: 'pt-PT'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ga-IE'})
MATCH (f:Formatting {key: 'ga-IE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-AR'})
MATCH (f:Formatting {key: 'es-AR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'gl-ES'})
MATCH (f:Formatting {key: 'gl-ES'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-ES'})
MATCH (f:Formatting {key: 'es-ES'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'zh-TH'})
MATCH (f:Formatting {key: 'zh-TH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'be-BY'})
MATCH (f:Formatting {key: 'be-BY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'wo-SN'})
MATCH (f:Formatting {key: 'wo-SN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-NI'})
MATCH (f:Formatting {key: 'es-NI'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'sv-SE'})
MATCH (f:Formatting {key: 'sv-SE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'hr-HR'})
MATCH (f:Formatting {key: 'hr-HR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-CI'})
MATCH (f:Formatting {key: 'fr-CI'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ku-TR'})
MATCH (f:Formatting {key: 'ku-TR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-BW'})
MATCH (f:Formatting {key: 'en-BW'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-IQ'})
MATCH (f:Formatting {key: 'ar-IQ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'pa-PK'})
MATCH (f:Formatting {key: 'pa-PK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-DO'})
MATCH (f:Formatting {key: 'es-DO'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-CR'})
MATCH (f:Formatting {key: 'es-CR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ps-AF'})
MATCH (f:Formatting {key: 'ps-AF'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-CM'})
MATCH (f:Formatting {key: 'fr-CM'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'el-GR'})
MATCH (f:Formatting {key: 'el-GR'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'hi-IN'})
MATCH (f:Formatting {key: 'hi-IN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'qu-PE'})
MATCH (f:Formatting {key: 'qu-PE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'sr-RS'})
MATCH (f:Formatting {key: 'sr-RS'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ms-BN'})
MATCH (f:Formatting {key: 'ms-BN'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-UY'})
MATCH (f:Formatting {key: 'es-UY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'zu-ZA'})
MATCH (f:Formatting {key: 'zu-ZA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-BF'})
MATCH (f:Formatting {key: 'fr-BF'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-SA'})
MATCH (f:Formatting {key: 'en-SA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'id-ID'})
MATCH (f:Formatting {key: 'id-ID'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-QA'})
MATCH (f:Formatting {key: 'ar-QA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-VE'})
MATCH (f:Formatting {key: 'es-VE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'az-AZ'})
MATCH (f:Formatting {key: 'az-AZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'de-DE'})
MATCH (f:Formatting {key: 'de-DE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-SA'})
MATCH (f:Formatting {key: 'ar-SA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-MA'})
MATCH (f:Formatting {key: 'fr-MA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'uz-UZ'})
MATCH (f:Formatting {key: 'uz-UZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'lv-LV'})
MATCH (f:Formatting {key: 'lv-LV'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'xh-ZA'})
MATCH (f:Formatting {key: 'xh-ZA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ro-MD'})
MATCH (f:Formatting {key: 'ro-MD'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-NG'})
MATCH (f:Formatting {key: 'en-NG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'it-IT'})
MATCH (f:Formatting {key: 'it-IT'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'no-NO'})
MATCH (f:Formatting {key: 'no-NO'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'sk-SK'})
MATCH (f:Formatting {key: 'sk-SK'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'gn-PY'})
MATCH (f:Formatting {key: 'gn-PY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'uk-UA'})
MATCH (f:Formatting {key: 'uk-UA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'pl-PL'})
MATCH (f:Formatting {key: 'pl-PL'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'zh-SG'})
MATCH (f:Formatting {key: 'zh-SG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-CA'})
MATCH (f:Formatting {key: 'en-CA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-AE'})
MATCH (f:Formatting {key: 'en-AE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-CY'})
MATCH (f:Formatting {key: 'en-CY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ru-RU'})
MATCH (f:Formatting {key: 'ru-RU'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'mi-NZ'})
MATCH (f:Formatting {key: 'mi-NZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-PE'})
MATCH (f:Formatting {key: 'es-PE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'yo-NG'})
MATCH (f:Formatting {key: 'yo-NG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'tk-TM'})
MATCH (f:Formatting {key: 'tk-TM'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-PY'})
MATCH (f:Formatting {key: 'es-PY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ha-NG'})
MATCH (f:Formatting {key: 'ha-NG'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'el-CY'})
MATCH (f:Formatting {key: 'el-CY'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'jv-ID'})
MATCH (f:Formatting {key: 'jv-ID'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'km-KH'})
MATCH (f:Formatting {key: 'km-KH'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'so-SO'})
MATCH (f:Formatting {key: 'so-SO'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-AE'})
MATCH (f:Formatting {key: 'ar-AE'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'en-NZ'})
MATCH (f:Formatting {key: 'en-NZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ne-NP'})
MATCH (f:Formatting {key: 'ne-NP'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'fr-DZ'})
MATCH (f:Formatting {key: 'fr-DZ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'ar-LB'})
MATCH (f:Formatting {key: 'ar-LB'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'tg-TJ'})
MATCH (f:Formatting {key: 'tg-TJ'})
MERGE (l)-[:HAS_FORMATTING]->(f);

MATCH (l:Locale {key: 'es-PA'})
MATCH (f:Formatting {key: 'es-PA'})
MERGE (l)-[:HAS_FORMATTING]->(f);

