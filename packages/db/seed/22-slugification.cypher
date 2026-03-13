// ============================================================================
// SLUGIFICATION SEED - Generated from ATH 2-rules-slug
// Generated: 2026-03-12 (v0.19.0 standard properties)
// Source: /Users/thibaut/Projects/traduction_ai/ath-know-l10n/outputs/localization-data/2-rules-slug/
// Locales: 200
// ============================================================================
// ----------------------------------------------------------------------------
// PART 2: Slugification nodes (200 locales)
// ----------------------------------------------------------------------------

MERGE (s:Slugification {key: 'af-ZA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Afrikaans (South Africa) Slugification',
    s.content = 'URL slug generation rules for af-ZA',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"verb":["is","het","was","sal","kan"],"pronoun":["wat","wie","dit","ons"],"conjunction":["en","of","maar","as"],"adverb":["nie","ook","dan"],"article":["die"],"preposition":["in","op","van","met","na","vir","uit","by","tot","oor"]}',
    s.stopwords_count = 27,
    s.regional_additions = '[{"word":"","category":"nie","reason":"adverb"},{"word":"","category":"ook","reason":"adverb"},{"word":"","category":"as","reason":"conjunction"},{"word":"","category":"dan","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"> 60% stopwords removed","message":"Content may be too generic"}]',
    s.examples = '[{"input":"Hoe om jou besigheid te laat groei","output":"hoe-jou-besigheid-laat-groei","rules_applied":["Stopwords: om","te removed"]},{"input":"Die beste plekke in Kaapstad","output":"beste-plekke-kaapstad","rules_applied":["Stopwords: die","in removed"]},{"input":"Wenke vir gesonde eetgewoontes","output":"wenke-gesonde-eetgewoontes","rules_applied":["Stopwords: vir removed"]},{"input":"Reis na die Drakensberge","output":"reis-drakensberge","rules_applied":["Stopwords: na","die removed"]},{"input":"Geld spaar met hierdie idees","output":"geld-spaar-hierdie-idees","rules_applied":["Stopwords: met removed"]},{"input":"Top 10 vakansiebestemmings vir 2025","output":"top-10-vakansiebestemmings-2025","rules_applied":["Numbers preserved","stopwords: vir removed"]},{"input":"Die volledige gids tot sukses in jou loopbaan en persoonlike lewe met praktiese wenke en voorbeelde","output":"volledige-gids-sukses-jou-loopbaan-persoonlike-lewe-praktiese-wenke-voorbeelde","rules_applied":["Truncated at 80 chars","multiple stopwords removed"]},{"input":"Kaapstad: \'n Stad van Kulture & Geskiedenis!","output":"kaapstad-stad-kulture-geskiedenis","rules_applied":["Punctuation removed","stopwords: n","van removed"]},{"input":"\"Boerewors\" - Suid-Afrika se Ikoniese Gereg","output":"boerewors-suid-afrika-ikoniese-gereg","rules_applied":["Quotes removed","stopwords: se removed"]},{"input":"Proteas en Springbokke: Sport in SA","output":"proteas-springbokke-sport-sa","rules_applied":["Colon removed","stopwords: en","in removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/af-ZA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-AE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Arabic (UAE) Slugification',
    s.content = 'URL slug generation rules for ar-AE',
    s.slug_rule = 'native_script',
    s.stopwords = '{"currency":["درهم"],"pronoun":["هو","هي"],"preposition":["من","في","على","إلى","عن","مع"],"conjunction":["و","أو","أن"],"demonstrative":["هذا","هذه","ذلك"],"verb":["كان"]}',
    s.stopwords_count = 16,
    s.regional_additions = '[{"word":"","category":"الإمارات","reason":"proper noun"},{"word":"","category":"الإماراتي","reason":"adjective"},{"word":"","category":"الإماراتية","reason":"adjective"},{"word":"","category":"دولة","reason":"noun"},{"word":"","category":"درهم","reason":"currency"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":"remove_tashkeel","numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful indexing"},{"condition":"Slug > 60 chars","message":"Consider shortening for display"},{"condition":"Mixed scripts","message":"Arabic + Latin detected - verify intentional (brand names acceptable)"},{"condition":"Tashkeel present","message":"Diacritical marks should be removed"},{"condition":"Duplicate hyphens","message":"Check normalization pipeline"}]',
    s.examples = '[{"input":"وظائف شاغرة في دبي","output":"وظائف-شاغرة-دبي","rules_applied":["Stopword (في) removed","Dubai preserved"]},{"input":"شقق للإيجار في أبوظبي 2026","output":"شقق-للإيجار-أبوظبي-2026","rules_applied":["Stopword (في) removed","numerals preserved"]},{"input":"دليل زيارة برج خليفة والنافورة الراقصة","output":"دليل-زيارة-برج-خليفة-والنافورة-الراقصة","rules_applied":["Conjunction (و) kept as prefix","landmarks preserved"]},{"input":"أفضل مطاعم نخلة جميرا","output":"أفضل-مطاعم-نخلة-جميرا","rules_applied":["Direct conversion","Palm Jumeirah preserved"]},{"input":"رحلات طيران الإمارات إلى لندن","output":"رحلات-طيران-الإمارات-لندن","rules_applied":["Stopword (إلى) removed","Emirates (airline) preserved"]},{"input":"عروض مهرجان دبي للتسوق 2026","output":"عروض-مهرجان-دبي-للتسوق-2026","rules_applied":["Dubai Shopping Festival - numerals and event preserved"]},{"input":"استراتيجيات الاستثمار العقاري في الإمارات العربية المتحدة للمستثمرين الأجانب والمقيمين خلال العام الجاري","output":"استراتيجيات-الاستثمار-العقاري-الإمارات-العربية-المتحدة-للمستثمرين-الأجانب-والمقيم","rules_applied":["Long title truncated at 80 chars","stopword (في) removed"]},{"input":"ما هي أفضل الفنادق؟ للعائلات!","output":"أفضل-الفنادق-للعائلات","rules_applied":["Punctuation removed","stopwords (ما، هي) removed"]},{"input":"\"دبي مول\" - أكبر مركز تسوق في الشرق الأوسط","output":"دبي-مول-أكبر-مركز-تسوق-الشرق-الأوسط","rules_applied":["Quotes removed","stopword (في) removed"]},{"input":"إمارة الشارقة: متاحف و تراث إسلامي عريق","output":"إمارة-الشارقة-متاحف-تراث-إسلامي-عريق","rules_applied":["Colon removed","stopword (و) removed"]},{"input":"Arabic","output":"Standard Romanization","rules_applied":["Avoid"]},{"input":"دبي","output":"dubai","rules_applied":["dubayy","dubay"]},{"input":"أبوظبي","output":"abu-dhabi","rules_applied":["abuzabi","abu-zabi"]},{"input":"الشارقة","output":"sharjah","rules_applied":["al-shariqah","shariqa"]},{"input":"عجمان","output":"ajman","rules_applied":["ajmaan"]},{"input":"رأس الخيمة","output":"ras-al-khaimah","rules_applied":["ras-ul-khaima"]},{"input":"الفجيرة","output":"fujairah","rules_applied":["fujayrah"]},{"input":"أم القيوين","output":"umm-al-quwain","rules_applied":["umm-ul-qaiwain"]},{"input":"نخلة جميرا","output":"palm-jumeirah","rules_applied":["nakhlat-jumairah"]},{"input":"برج خليفة","output":"burj-khalifa","rules_applied":["burj-khaleefa"]},{"input":"Arabic Query","output":"Expected Slug Pattern","rules_applied":[]},{"input":"","output":"دبي","rules_applied":["دبي (always preserved - key geographic identifier)"]},{"input":"أبوظبي","output":"أبوظبي (always preserved)","rules_applied":[]},{"input":"فيزا-الإمارات","output":"","rules_applied":["وظائف دبي"]},{"input":"","output":"شقق دبي","rules_applied":["شقق-دبي"]},{"input":"عقارات أبوظبي","output":"عقارات-أبوظبي","rules_applied":[]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-AE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-BH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (BH) Slugification',
    s.content = 'URL slug generation rules for ar-BH',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[{"word":"","category":"البحرين","reason":"noun"},{"word":"","category":"البحرينية","reason":"adjective"},{"word":"","category":"البحريني","reason":"adjective"},{"word":"","category":"المنامة","reason":"noun"},{"word":"","category":"المحرق","reason":"noun"},{"word":"","category":"مملكة","reason":"noun"},{"word":"","category":"المملكة","reason":"noun"},{"word":"### Financial Sector Stopwords (Bahrain-specific)","category":"Word","reason":"Category"},{"word":"","category":"مصرف","reason":"noun"},{"word":"","category":"بنك","reason":"noun"},{"word":"","category":"خدمات","reason":"noun"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":"remove_tashkeel","numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Arabic mixed with non-Arabic (except numerals)"},{"condition":"Diacritics present","message":"Tashkeel should be removed for consistency"},{"condition":"Non-standard composition","message":"Check Unicode normalization"},{"condition":"Excessive hyphens","message":"More than 8 hyphens may indicate over-fragmentation"}]',
    s.examples = '[{"input":"دليل السياحة في المنامة","output":"دليل-السياحة","rules_applied":["Stopwords (في","المنامة) removed"]},{"input":"أفضل المطاعم البحرينية 2025","output":"أفضل-المطاعم-2025","rules_applied":["Stopword (البحرينية) removed","number preserved"]},{"input":"كيفية فتح حساب مصرفي في البحرين","output":"كيفية-فتح-حساب-مصرفي","rules_applied":["Stopwords (في","البحرين) removed"]},{"input":"خدمات الصيرفة الإسلامية للأفراد والشركات","output":"الصيرفة-الإسلامية-للأفراد-والشركات","rules_applied":["Stopword (خدمات) removed"]},{"input":"معرض البحرين الدولي للطيران في صخير","output":"معرض-الدولي-للطيران-صخير","rules_applied":["Stopword (البحرين","في) removed"]},{"input":"أسعار العقارات في جفير وسترة لعام 2025","output":"أسعار-العقارات-جفير-وسترة-لعام-2025","rules_applied":["Stopword (في) removed","districts preserved"]},{"input":"دليل شامل للمقيمين الجدد في مملكة البحرين عن الإقامة والعمل والخدمات الحكومية الإلكترونية","output":"دليل-شامل-للمقيمين-الجدد-الإقامة-العمل-الحكومية-الإلكترونية","rules_applied":["Long title","multiple stopwords (في","مملكة","البحرين","عن","و) removed"]},{"input":"الفعاليات & المهرجانات: موسم التمور في البحرين!","output":"الفعاليات-المهرجانات-موسم-التمور","rules_applied":["Special chars (&",":","!) removed","stopwords removed"]},{"input":"\"جائزة البحرين الكبرى\" - سباق الفورمولا 1","output":"جائزة-الكبرى-سباق-الفورمولا-1","rules_applied":["Quotes and dash removed","stopword (البحرين) removed"]},{"input":"نصائح لزيارة قلعة البحرين (موقع اليونسكو للتراث العالمي)","output":"نصائح-لزيارة-قلعة-موقع-اليونسكو-للتراث-العالمي","rules_applied":["Parentheses removed","stopword (البحرين) removed"]},{"input":"ترخيص التكنولوجيا المالية من مصرف البحرين المركزي","output":"ترخيص-التكنولوجيا-المالية-المركزي","rules_applied":["Stopwords (من","مصرف","البحرين) removed"]},{"input":"الخدمات المصرفية الرقمية للشركات الناشئة","output":"المصرفية-الرقمية-للشركات-الناشئة","rules_applied":["Stopword (الخدمات) removed"]},{"input":"دليل الاستثمار في مرفأ البحرين المالي","output":"دليل-الاستثمار-مرفأ-المالي","rules_applied":["Stopwords (في","البحرين) removed"]},{"input":"فتح حساب بنكي للأجانب في البحرين","output":"فتح-حساب-بنكي-للأجانب","rules_applied":["Stopwords (في","البحرين) removed"]},{"input":"شركات الدفع الإلكتروني المرخصة","output":"شركات-الدفع-الإلكتروني-المرخصة","rules_applied":["No stopwords","preserved as-is"]},{"input":"Feature","output":"ar-BH (Bahrain)","rules_applied":["ar-JO (Jordan)"]},{"input":"Capital stopword","output":"المنامة (Manama)","rules_applied":["عمان (Amman)"]},{"input":"Country stopword","output":"البحرين","rules_applied":["الأردن"]},{"input":"Adjective stopword","output":"البحرينية/البحريني","rules_applied":["الأردنية"]},{"input":"Geographic terms","output":"المحرق، جفير، الرفاع","rules_applied":["البتراء، جرش، العقبة"]},{"input":"Formal prefix","output":"مملكة","rules_applied":["المملكة الهاشمية"]},{"input":"Industry focus","output":"Financial services, fintech","rules_applied":["Tourism","heritage"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-BH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-DZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (DZ) Slugification',
    s.content = 'URL slug generation rules for ar-DZ',
    s.slug_rule = 'native_script',
    s.stopwords = '{"adverb":["كيفاش"]}',
    s.stopwords_count = 1,
    s.regional_additions = '[{"word":"","category":"الجزائر","reason":"noun"},{"word":"","category":"الجزائرية","reason":"adjective"},{"word":"","category":"العاصمة","reason":"noun"},{"word":"","category":"ولاية","reason":"noun"},{"word":"","category":"واش","reason":"particle"},{"word":"","category":"كيفاش","reason":"adverb"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":"remove_tashkeel","numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Arabic mixed with non-Arabic (except numerals)"},{"condition":"Diacritics present","message":"Tashkeel should be removed for consistency"},{"condition":"Non-standard composition","message":"Check Unicode normalization"}]',
    s.examples = '[{"input":"دليل السياحة في وهران","output":"دليل-السياحة-وهران","rules_applied":["Stopword (في) removed"]},{"input":"أفضل المطاعم في الجزائر العاصمة","output":"أفضل-المطاعم-العاصمة","rules_applied":["Stopwords (في","الجزائر) removed"]},{"input":"كيفية الحصول على جواز السفر الجزائري","output":"كيفية-الحصول-جواز-السفر-الجزائري","rules_applied":["Stopword (على) removed"]},{"input":"تاريخ الثورة الجزائرية والاستقلال","output":"تاريخ-الثورة-الجزائرية-والاستقلال","rules_applied":["Conjunction و kept in compound word"]},{"input":"أسعار العقارات في ولاية سطيف 2024","output":"أسعار-العقارات-سطيف-2024","rules_applied":["Stopwords (في","ولاية) removed","number preserved"]},{"input":"أحداث ثقافية لصيف 2024 في قسنطينة","output":"أحداث-ثقافية-لصيف-2024-قسنطينة","rules_applied":["Number preserved","stopword (في) removed"]},{"input":"دليل شامل للمغتربين العائدين إلى الجزائر: الإجراءات والوثائق المطلوبة للعام 2024-2025","output":"دليل-شامل-للمغتربين-العائدين-الإجراءات-والوثائق-المطلوبة-للعام-2024-2025","rules_applied":["Long title truncated at 80 chars","stopwords (إلى","الجزائر) removed"]},{"input":"الكسكسي & المأكولات التقليدية: وصفات جزائرية!","output":"الكسكسي-المأكولات-التقليدية-وصفات-جزائرية","rules_applied":["Special chars (&",":","!) removed"]},{"input":"\"الجو اليوم\" - نشرة الأرصاد الجوية","output":"الجو-اليوم-نشرة-الأرصاد-الجوية","rules_applied":["Quotes and dash removed"]},{"input":"واش راك دايرين (عبارات جزائرية شائعة)","output":"راك-دايرين-عبارات-جزائرية-شائعة","rules_applied":["Darja stopword (واش) removed","parentheses removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-DZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-EG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (EG) Slugification',
    s.content = 'URL slug generation rules for ar-EG',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["و","أو","ثم","عشان","علشان"],"pronoun":["هو","هي","هم"],"preposition":["في","من","على","إلى","عن","مع","زي"],"demonstrative":["هذا","هذه","ذلك","دي","ده","دول"],"verb":["كان","يكون"],"interrogative":["ازاي"],"filler":["يعني"],"possessive":["بتاع","بتاعت"],"adverb":["كده"],"negation":["لا"]}',
    s.stopwords_count = 29,
    s.regional_additions = '[{"word":"","category":"دي","reason":"demonstrative"},{"word":"","category":"ده","reason":"demonstrative"},{"word":"","category":"دول","reason":"demonstrative"},{"word":"","category":"بتاع","reason":"possessive"},{"word":"","category":"بتاعت","reason":"possessive"},{"word":"","category":"كده","reason":"adverb"},{"word":"","category":"عشان","reason":"conjunction"},{"word":"","category":"علشان","reason":"conjunction"},{"word":"","category":"زي","reason":"preposition"},{"word":"","category":"يعني","reason":"filler"},{"word":"","category":"اللي","reason":"relative pronoun"},{"word":"","category":"ازاي","reason":"interrogative"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Arabic mixed with Latin (except embedded technical terms)"},{"condition":"Non-standard diacritics","message":"Uncommon Unicode composition detected"}]',
    s.examples = '[{"input":"دليل السياحة في القاهرة","output":"دليل-السياحة-القاهرة","rules_applied":["NFC","stopword \"في\" removed","spaces to hyphens"]},{"input":"أفضل المطاعم المصرية","output":"أفضل-المطاعم-المصرية","rules_applied":["NFC","spaces to hyphens"]},{"input":"رحلة إلى الأقصر وأسوان","output":"رحلة-الأقصر-أسوان","rules_applied":["NFC","stopwords \"إلى\" and \"و\" removed"]},{"input":"تاريخ الأهرامات العظيمة","output":"تاريخ-الأهرامات-العظيمة","rules_applied":["NFC","spaces to hyphens"]},{"input":"نصائح للتسوق في خان الخليلي","output":"نصائح-للتسوق-خان-الخليلي","rules_applied":["NFC","stopword \"في\" removed"]},{"input":"أحداث القاهرة 2025","output":"أحداث-القاهرة-2025","rules_applied":["NFC","numbers preserved","spaces to hyphens"]},{"input":"الدليل الشامل والمفصل لزيارة المتحف المصري الكبير في الجيزة","output":"الدليل-الشامل-المفصل-لزيارة-المتحف-المصري-الكبير-الجيزة","rules_applied":["NFC","stopwords \"و\" and \"في\" removed","truncated at 80 chars"]},{"input":"الطعام المصري: كشري، فول، وطعمية!","output":"الطعام-المصري-كشري-فول-طعمية","rules_applied":["NFC","punctuation removed","stopword \"و\" removed"]},{"input":"\"أم الدنيا\" مصر الجميلة","output":"أم-الدنيا-مصر-الجميلة","rules_applied":["NFC","quotes removed","spaces to hyphens"]},{"input":"أخبار الأهلي والزمالك في الدوري","output":"أخبار-الأهلي-الزمالك-الدوري","rules_applied":["NFC","stopwords \"و\" and \"في\" removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-EG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-IQ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (IQ) Slugification',
    s.content = 'URL slug generation rules for ar-IQ',
    s.slug_rule = 'native_script',
    s.stopwords = '{"preposition":["في","من","على","الى","عن","مع"],"pronoun":["هو","هي","هم"],"interrogative":["شلون","شنو"],"conjunction":["و","او","لكن"],"article":["ال"],"adverb":["هواي"],"verb":["كان","يكون","اكو","ماكو"],"demonstrative":["هذا","هذه","ذلك"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[{"word":"","category":"بغداد","reason":"place"},{"word":"","category":"شلون","reason":"interrogative"},{"word":"","category":"شنو","reason":"interrogative"},{"word":"","category":"هواي","reason":"adverb"},{"word":"","category":"اكو","reason":"verb"},{"word":"","category":"ماكو","reason":"verb"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful content"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Arabic with non-Arabic letters detected"},{"condition":"No Arabic chars","message":"Slug should contain native script"}]',
    s.examples = '[{"input":"اخبار العراق اليوم","output":"اخبار-العراق-اليوم","rules_applied":["Spaces to hyphens","stopwords kept (meaningful)"]},{"input":"دليل السفر الى بغداد","output":"دليل-السفر-بغداد","rules_applied":["Stopword \"الى\" removed"]},{"input":"اسعار النفط في البصرة","output":"اسعار-النفط-البصرة","rules_applied":["Stopword \"في\" removed"]},{"input":"مطاعم الكباب العراقي","output":"مطاعم-الكباب-العراقي","rules_applied":["Basic conversion"]},{"input":"تاريخ الحضارة السومرية","output":"تاريخ-الحضارة-السومرية","rules_applied":["Basic conversion"]},{"input":"افضل 10 اماكن سياحية 2025","output":"افضل-10-اماكن-سياحية-2025","rules_applied":["Numbers preserved"]},{"input":"الدليل الشامل للسياحة والسفر والاستكشاف في مدن العراق التاريخية القديمة والحديثة","output":"الدليل-الشامل-للسياحة-والسفر-والاستكشاف-مدن-العراق-التاريخية-القديمة-والحديثة","rules_applied":["Truncated at 80 chars","stopword \"في\" removed"]},{"input":"الموصل: تراث وثقافة!","output":"الموصل-تراث-وثقافة","rules_applied":["Punctuation removed (colon","exclamation)"]},{"input":"قصيدة \"بغداد يا بغداد\" للشاعر","output":"قصيدة-بغداد-يا-بغداد-للشاعر","rules_applied":["Quotes removed"]},{"input":"كيفية كتابة الـURL بالعربي","output":"كيفية-كتابة-الurl-بالعربي","rules_applied":["Mixed script: Arabic preserved","Latin lowercase"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-IQ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-JO'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (JO) Slugification',
    s.content = 'URL slug generation rules for ar-JO',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[{"word":"","category":"الأردن","reason":"noun"},{"word":"","category":"الأردنية","reason":"adjective"},{"word":"","category":"الأردني","reason":"adjective"},{"word":"","category":"عمان","reason":"noun"},{"word":"","category":"المملكة","reason":"noun"},{"word":"","category":"الهاشمية","reason":"adjective"},{"word":"MSA Form","category":"Context","reason":""},{"word":"فرص عمل","category":"وظائف","reason":"فرص عمل"},{"word":"","category":"سيارة","reason":"سيارة"},{"word":"","category":"شقة","reason":"شقة"},{"word":"","category":"هسا","reason":"الآن"},{"word":"","category":"كتير","reason":"كثير"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":"remove_tashkeel","numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Arabic mixed with non-Arabic (except numerals)"},{"condition":"Diacritics present","message":"Tashkeel should be removed for consistency"},{"condition":"Non-standard composition","message":"Check Unicode normalization"}]',
    s.examples = '[{"input":"دليل السياحة في البتراء","output":"دليل-السياحة-البتراء","rules_applied":["Stopword (في) removed"]},{"input":"أفضل المطاعم في عمان","output":"أفضل-المطاعم","rules_applied":["Stopwords (في","عمان) removed"]},{"input":"فرص عمل في الزرقاء 2024","output":"فرص-عمل-الزرقاء-2024","rules_applied":["Stopword (في) removed","number preserved"]},{"input":"السياحة العلاجية في البحر الميت","output":"السياحة-العلاجية-البحر-الميت","rules_applied":["Stopword (في) removed","Dead Sea preserved"]},{"input":"رحلة إلى وادي رم الصحراوي","output":"رحلة-وادي-رم-الصحراوي","rules_applied":["Stopword (إلى) removed"]},{"input":"جولة في آثار جرش الرومانية 2024","output":"جولة-آثار-جرش-الرومانية-2024","rules_applied":["Stopword (في) removed","numerals preserved"]},{"input":"دليل شامل للاستثمار العقاري في المملكة الأردنية الهاشمية للمستثمرين الأجانب والمحليين","output":"دليل-شامل-للاستثمار-العقاري-للمستثمرين-الأجانب-والمحليين","rules_applied":["Multiple stopwords (في","المملكة","الأردنية","الهاشمية) removed","truncated"]},{"input":"الصحة & اللياقة: نصائح للحياة!","output":"الصحة-اللياقة-نصائح-للحياة","rules_applied":["Special chars (&",":","!) removed"]},{"input":"\"قلعة عجلون\" - التاريخ والجمال","output":"قلعة-عجلون-التاريخ-والجمال","rules_applied":["Quotes and dash removed"]},{"input":"السياحة في العقبة (البحر الأحمر)","output":"السياحة-العقبة-البحر-الأحمر","rules_applied":["Stopword (في) removed","parentheses removed"]},{"input":"Category","output":"Example Keywords","rules_applied":["Slug Pattern"]},{"input":"Tourism","output":"البتراء، وادي رم، البحر الميت","rules_applied":["[attraction]-دليل-زيارة"]},{"input":"Jobs","output":"فرص عمل، توظيف، وظائف شاغرة","rules_applied":["فرص-عمل-[industry]-[city]"]},{"input":"Real Estate","output":"شقق للبيع، أراضي، عقارات","rules_applied":["[property-type]-[action]-[location]"]},{"input":"Education","output":"جامعة، تعليم، منح دراسية","rules_applied":["[institution]-[program]-الأردن"]},{"input":"Healthcare","output":"مستشفى، علاج، صحة","rules_applied":["[service]-[specialty]-[city]"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-JO.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-KW'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (KW) Slugification',
    s.content = 'URL slug generation rules for ar-KW',
    s.slug_rule = 'native_script',
    s.stopwords = '{"pronoun":["هو","هي"],"verb":["كان"],"demonstrative":["هذا","هذه","ذلك"],"preposition":["من","في","على","إلى","عن","مع"],"currency":["دينار"],"conjunction":["و","أو","أن"]}',
    s.stopwords_count = 16,
    s.regional_additions = '[{"word":"","category":"الكويت","reason":"proper noun"},{"word":"","category":"الكويتي","reason":"adjective"},{"word":"","category":"الكويتية","reason":"adjective"},{"word":"","category":"دينار","reason":"currency"},{"word":"","category":"البلد","reason":"noun"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":"remove_tashkeel","numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful indexing"},{"condition":"Slug > 60 chars","message":"Consider shortening for display"},{"condition":"Mixed scripts","message":"Arabic + Latin detected - verify intentional (brand names acceptable)"},{"condition":"Tashkeel present","message":"Diacritical marks should be removed"},{"condition":"Duplicate hyphens","message":"Check normalization pipeline"}]',
    s.examples = '[{"input":"وظائف شاغرة حولي مطلوب محاسب","output":"وظائف-شاغرة-حولي-مطلوب-محاسب","rules_applied":["Hawalli district preserved","direct conversion"]},{"input":"شقق للإيجار السالمية 2026","output":"شقق-للإيجار-السالمية-2026","rules_applied":["Salmiya area preserved","numerals kept"]},{"input":"رحلات الخطوط الجوية الكويتية لندن","output":"رحلات-الخطوط-الجوية-لندن","rules_applied":["Kuwait Airways","stopword (الكويتية) removed"]},{"input":"ديوانية تراثية منطقة الشرق","output":"ديوانية-تراثية-منطقة-الشرق","rules_applied":["Diwaniya (traditional gathering)","Sharq area"]},{"input":"أسعار صرف الدينار الكويتي اليوم","output":"أسعار-صرف-اليوم","rules_applied":["Stopwords removed (الدينار، الكويتي)","KWD exchange rates"]},{"input":"فعاليات مهرجان هلا فبراير 2026 الأفنيوز","output":"فعاليات-مهرجان-هلا-فبراير-2026-الأفنيوز","rules_applied":["Hala February festival","Avenues Mall","numerals preserved"]},{"input":"احتفالات العيد الوطني الكويتي 25 فبراير وعيد التحرير 26 فبراير في ساحة الإرادة وأبراج الكويت","output":"احتفالات-العيد-الوطني-25-فبراير-وعيد-التحرير-26-فبراير-ساحة-الإرادة-وأبراج","rules_applied":["Long title truncated at 80 chars","National/Liberation Days","stopwords removed"]},{"input":"ما هي شروط التأشيرة الكويتية؟ للعمالة المنزلية","output":"شروط-التأشيرة-للعمالة-المنزلية","rules_applied":["Punctuation removed","stopwords removed (ما، هي، الكويتية)"]},{"input":"\"بنك الكويت الوطني\" NBK - فرع السالمية","output":"بنك-الوطني-nbk-فرع-السالمية","rules_applied":["Quotes removed","mixed script (NBK brand)","stopword (الكويت) removed"]},{"input":"زين الكويت باقات 5G إنترنت منزلي","output":"زين-باقات-5g-إنترنت-منزلي","rules_applied":["Zain Kuwait telecom","stopword (الكويت) removed","5G preserved"]},{"input":"Arabic Query","output":"Expected Slug Pattern","rules_applied":[]},{"input":"","output":"وظائف الكويت","rules_applied":["وظائف (country name removed)"]},{"input":"شقق الكويت","output":"شقق (country name removed)","rules_applied":[]},{"input":"تأشيرة (country name removed)","output":"","rules_applied":["رواتب الكويت"]},{"input":"","output":"ديوانية","rules_applied":["ديوانية (Kuwaiti cultural term preserved)"]},{"input":"دينار كويتي","output":"Filtered in pricing contexts","rules_applied":[]},{"input":"الأفنيوز (major mall - always preserved)","output":"","rules_applied":["أبراج الكويت"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-KW.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-LB'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (LB) Slugification',
    s.content = 'URL slug generation rules for ar-LB',
    s.slug_rule = 'native_script',
    s.stopwords = '{"verb":["كان","يكون"],"adverb":["هيك","كتير","هلق","وين","كيف"],"conjunction":["و","او","ان"],"preposition":["في","من","الى","على","عن","مع"],"article":["ال"],"pronoun":["هذا","هذه","هو","هي","شو"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[{"word":"","category":"هيك","reason":"adverb"},{"word":"","category":"كتير","reason":"adverb"},{"word":"","category":"هلق","reason":"adverb"},{"word":"","category":"شو","reason":"pronoun"},{"word":"","category":"وين","reason":"adverb"},{"word":"","category":"كيف","reason":"adverb"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Review for consistency (except technical terms)"},{"condition":"Non-standard Unicode composition","message":"Check normalization"}]',
    s.examples = '[{"input":"افضل مطاعم بيروت","output":"افضل-مطاعم-بيروت","rules_applied":["stopwords","spaces->hyphens"]},{"input":"دليل السياحة في لبنان","output":"دليل-السياحة-لبنان","rules_applied":["stopwords (في)","spaces->hyphens"]},{"input":"الحياة الليلية في الجميزة","output":"الحياة-الليلية-الجميزة","rules_applied":["stopwords (في)","spaces->hyphens"]},{"input":"اكلات لبنانية تقليدية","output":"اكلات-لبنانية-تقليدية","rules_applied":["spaces->hyphens"]},{"input":"جبل لبنان والطبيعة","output":"جبل-لبنان-الطبيعة","rules_applied":["stopwords (و)","spaces->hyphens"]},{"input":"اهم 10 اماكن سياحية في بيروت","output":"اهم-10-اماكن-سياحية-بيروت","rules_applied":["numbers preserved","stopwords (في)"]},{"input":"دليل شامل للسياحة والاستكشاف في لبنان مع افضل النصائح للمسافرين العرب والاجانب","output":"دليل-شامل-للسياحة-الاستكشاف-لبنان-افضل-النصائح-للمسافرين-العرب-الاجانب","rules_applied":["truncation at 80 chars","stopwords"]},{"input":"مطعم \"ليوان\" - افضل المأكولات!","output":"مطعم-ليوان-افضل-المأكولات","rules_applied":["punctuation removed"]},{"input":"مقهى \"الكريستال\" في الحمرا","output":"مقهى-الكريستال-الحمرا","rules_applied":["quotes removed","stopwords (في)"]},{"input":"كنيسة مار مارون - سيدة لبنان","output":"كنيسة-مار-مارون-سيدة-لبنان","rules_applied":["dash normalized","special chars removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-LB.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-LY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (LY) Slugification',
    s.content = 'URL slug generation rules for ar-LY',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["و","أو","ثم","لكن"],"adverb":["هناك","هكي","توا","برشا"],"article":["ال","الـ"],"verb":["كان","يكون"],"pronoun":["هو","هي","هذا","هذه","ذلك"],"preposition":["في","من","إلى","على","عن","مع"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[{"word":"","category":"باهي","reason":"adjective"},{"word":"","category":"هكي","reason":"adverb"},{"word":"","category":"توا","reason":"adverb"},{"word":"","category":"برشا","reason":"adverb"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":"remove_tashkeel","numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Warn if non-Arabic letters present (except numbers)"},{"condition":"Tashkeel present","message":"Should be stripped before slugification"}]',
    s.examples = '[{"input":"أفضل المطاعم في طرابلس","output":"أفضل-المطاعم-طرابلس","rules_applied":["Stopword في removed","spaces to hyphens"]},{"input":"دليل السياحة في ليبيا","output":"دليل-السياحة-ليبيا","rules_applied":["Stopword في removed","Arabic preserved"]},{"input":"أخبار الاقتصاد الليبي","output":"أخبار-الاقتصاد-الليبي","rules_applied":["Spaces to hyphens","script preserved"]},{"input":"صيانة السيارات من الخبراء","output":"صيانة-السيارات-الخبراء","rules_applied":["Stopword من removed"]},{"input":"رحلات إلى سبها ومصراتة","output":"رحلات-سبها-ومصراتة","rules_applied":["Stopword إلى removed"]},{"input":"أسعار النفط 2025 في ليبيا","output":"أسعار-النفط-2025-ليبيا","rules_applied":["Numbers preserved","stopword في removed"]},{"input":"كيفية الحصول على تأشيرة السفر إلى ليبيا والإقامة فيها بطريقة قانونية وسريعة","output":"كيفية-الحصول-تأشيرة-السفر-ليبيا-والإقامة-فيها-بطريقة-قانونية-وسريعة","rules_applied":["Long title truncated to 80 chars","stopwords removed"]},{"input":"المعالم السياحية: لبدة الكبرى!","output":"المعالم-السياحية-لبدة-الكبرى","rules_applied":["Punctuation removed (: and !)"]},{"input":"مقولة \"الصحراء جميلة\" في الثقافة الليبية","output":"مقولة-الصحراء-جميلة-الثقافة-الليبية","rules_applied":["Quotes removed","stopword في removed"]},{"input":"آثار لبدة ماغنا الرومانية","output":"آثار-لبدة-ماغنا-الرومانية","rules_applied":["Alef with madda (آ) preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-LY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-MA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (MA) Slugification',
    s.content = 'URL slug generation rules for ar-MA',
    s.slug_rule = 'native_script',
    s.stopwords = '{"pronoun":["هو","هي"],"conjunction":["و","أو","لكن"],"preposition":["في","من","إلى","على","عن","مع","بين"],"verb":["كان","يكون","غادي","راه"],"article":["شي"],"demonstrative":["هذا","هذه","ذلك"],"possessive":["ديال"],"interrogative":["فين","كيفاش","علاش","واش"]}',
    s.stopwords_count = 25,
    s.regional_additions = '[{"word":"","category":"ديال","reason":"possessive"},{"word":"","category":"فين","reason":"interrogative"},{"word":"","category":"كيفاش","reason":"interrogative"},{"word":"","category":"علاش","reason":"interrogative"},{"word":"","category":"شي","reason":"article"},{"word":"","category":"واش","reason":"interrogative"},{"word":"","category":"غادي","reason":"verb"},{"word":"","category":"راه","reason":"verb"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Arabic with Latin detected (except numbers)"},{"condition":"Non-standard Unicode","message":"Check NFC normalization"},{"condition":"French loanwords","message":"Consider Arabic equivalent for SEO"}]',
    s.examples = '[{"input":"أفضل المطاعم في الدار البيضاء","output":"أفضل-المطاعم-الدار-البيضاء","rules_applied":["Stopword في removed","spaces to hyphens"]},{"input":"دليل السفر إلى مراكش","output":"دليل-السفر-مراكش","rules_applied":["Stopword إلى removed"]},{"input":"الطاجين المغربي التقليدي","output":"الطاجين-المغربي-التقليدي","rules_applied":["Spaces to hyphens"]},{"input":"أخبار الرياضة والكرة المغربية","output":"أخبار-الرياضة-الكرة-المغربية","rules_applied":["Stopword و removed"]},{"input":"السياحة في المملكة المغربية","output":"السياحة-المملكة-المغربية","rules_applied":["Stopword في removed"]},{"input":"10 أماكن سياحية في فاس 2025","output":"10-أماكن-سياحية-فاس-2025","rules_applied":["Numbers preserved","stopword في removed"]},{"input":"دليل شامل للسياحة والسفر في المغرب العربي والمناطق الصحراوية الجنوبية","output":"دليل-شامل-للسياحة-السفر-المغرب-العربي-المناطق-الصحراوية-الجنوبية","rules_applied":["Long title truncated at word boundary","stopwords removed"]},{"input":"كيفاش تحضر الحريرة؟ والشباكية!","output":"كيفاش-تحضر-الحريرة-الشباكية","rules_applied":["Punctuation removed (؟ and !)"]},{"input":"\"أسرار\" الطبخ المغربي و\'التوابل\'","output":"أسرار-الطبخ-المغربي-التوابل","rules_applied":["Quotes removed","stopword و removed"]},{"input":"ءادم وحواء","output":"ءادم-حواء","rules_applied":["Hamza at start preserved","stopword و removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-MA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-OM'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (OM) Slugification',
    s.content = 'URL slug generation rules for ar-OM',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[{"word":"","category":"عمان","reason":"proper noun"},{"word":"","category":"سلطنة","reason":"noun"},{"word":"","category":"العمانية","reason":"adjective"},{"word":"","category":"العماني","reason":"adjective"},{"word":"","category":"ولاية","reason":"noun"},{"word":"","category":"والي","reason":"noun"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":"remove_tashkeel","numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO effectiveness"},{"condition":"Slug > 60 chars","message":"Consider shortening for better usability"},{"condition":"Mixed scripts","message":"Arabic mixed with Latin (except technical terms)"},{"condition":"Tashkeel present","message":"Diacritics should be removed for consistency"},{"condition":"Non-standard composition","message":"Verify Unicode NFC normalization"}]',
    s.examples = '[{"input":"دليل السياحة في مسقط","output":"دليل-السياحة-مسقط","rules_applied":["Stopword (في) removed","Muscat preserved"]},{"input":"أفضل فنادق صلالة خلال موسم الخريف","output":"أفضل-فنادق-صلالة-خلال-موسم-الخريف","rules_applied":["Salalah monsoon season reference preserved"]},{"input":"زيارة قلعة نزوى التاريخية","output":"زيارة-قلعة-نزوى-التاريخية","rules_applied":["Nizwa Fort heritage site preserved"]},{"input":"رحلات طيران عمان إلى لندن 2026","output":"رحلات-طيران-لندن-2026","rules_applied":["Stopwords (عمان","إلى) removed","Oman Air context clear","year preserved"]},{"input":"خدمات بنك مسقط للأفراد والشركات","output":"خدمات-بنك-مسقط-للأفراد-والشركات","rules_applied":["Bank Muscat services","conjunction (و) kept in compound"]},{"input":"مهرجان الخريف في صلالة: الفعاليات والأنشطة للعام 2026","output":"مهرجان-الخريف-صلالة-الفعاليات-والأنشطة-للعام-2026","rules_applied":["Khareef Festival","colon removed","stopword (في) removed"]},{"input":"دليل شامل لزيارة الجبل الأخضر ورمال الوهيبة والمعالم السياحية في سلطنة عمان للسياح","output":"دليل-شامل-لزيارة-الجبل-الأخضر-ورمال-الوهيبة-والمعالم-السياحية-للسياح","rules_applied":["Truncated at 80 chars","stopwords (في","سلطنة","عمان) removed"]},{"input":"سوق مطرح & التسوق التقليدي!","output":"سوق-مطرح-التسوق-التقليدي","rules_applied":["Mutrah Souq","special chars (&","!) removed"]},{"input":"\"دار الأوبرا السلطانية\" - فعاليات موسيقية","output":"دار-الأوبرا-السلطانية-فعاليات-موسيقية","rules_applied":["Royal Opera House","quotes and dash removed"]},{"input":"وظائف شركة OQ للطاقة (عمان للنفط سابقا)","output":"وظائف-شركة-oq-للطاقة-للنفط-سابقا","rules_applied":["OQ Energy jobs","parentheses removed","stopword (عمان) removed","Latin brand preserved"]},{"input":"Arabic","output":"Standard Transliteration","rules_applied":["Note"]},{"input":"مسقط","output":"muscat","rules_applied":["Not \"masqat\" or \"mascate\""]},{"input":"صلالة","output":"salalah","rules_applied":["Not \"salala\" or \"salale\""]},{"input":"نزوى","output":"nizwa","rules_applied":["Not \"nazwa\" or \"nizwah\""]},{"input":"صور","output":"sur","rules_applied":["Not \"sour\" or \"suhr\""]},{"input":"صحار","output":"sohar","rules_applied":["Not \"sahar\" or \"suhar\""]},{"input":"الوهيبة","output":"wahiba","rules_applied":["Not \"wahibah\""]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-OM.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-QA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (QA) Slugification',
    s.content = 'URL slug generation rules for ar-QA',
    s.slug_rule = 'native_script',
    s.stopwords = '{"verb":["كان"],"preposition":["من","في","على","الى","عن"],"demonstrative":["هذا","هذه"],"conjunction":["ان","و","او"],"currency":["ريال"]}',
    s.stopwords_count = 12,
    s.regional_additions = '[{"word":"","category":"قطر","reason":"proper noun"},{"word":"","category":"الدوحة","reason":"proper noun"},{"word":"","category":"ريال","reason":"currency"},{"word":"","category":"الخليجي","reason":"adjective"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":"remove_tashkeel","numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful URL"},{"condition":"Slug > 60 chars","message":"Consider shortening for usability"},{"condition":"Mixed scripts","message":"Latin/Arabic mix detected - verify intentional"},{"condition":"Tashkeel present","message":"Diacritics should be removed"}]',
    s.examples = '[{"input":"استاد لوسيل ارث مونديال 2022","output":"استاد-لوسيل-ارث-مونديال-2022","rules_applied":["World Cup legacy venue","numbers preserved"]},{"input":"وظائف شاغرة مؤسسة قطر التعليمية","output":"وظائف-شاغرة-مؤسسة-التعليمية","rules_applied":["Stopwords removed (قطر)","Qatar Foundation jobs"]},{"input":"سوق واقف جولة تراثية الدوحة","output":"سوق-واقف-جولة-تراثية","rules_applied":["Stopwords removed (الدوحة)","heritage market"]},{"input":"تذاكر الخطوط الجوية القطرية","output":"تذاكر-الخطوط-الجوية-القطرية","rules_applied":["Qatar Airways tickets","basic transformation"]},{"input":"شقق فاخرة للبيع اللؤلؤة قطر 2026","output":"شقق-فاخرة-للبيع-اللؤلؤة-2026","rules_applied":["Numbers preserved","The Pearl-Qatar real estate","stopwords removed (قطر)"]},{"input":"باقات اوريدو انترنت فايبر المنزلي","output":"باقات-اوريدو-انترنت-فايبر-المنزلي","rules_applied":["Ooredoo fiber packages","telecom services"]},{"input":"احتفالات اليوم الوطني القطري 18 ديسمبر في كتارا القرية الثقافية وسوق واقف التراثي","output":"احتفالات-اليوم-الوطني-القطري-18-ديسمبر-كتارا-القرية-الثقافية-سوق-واقف-التراثي","rules_applied":["Long title truncated at 80 chars","Qatar National Day","stopwords removed (في، و)"]},{"input":"كيف احصل على فيزا قطر للعمل؟","output":"كيف-احصل-فيزا-للعمل","rules_applied":["Punctuation removed (؟)","stopwords removed (على، قطر)","visa queries"]},{"input":"\"بي ان سبورتس\" - بث مباشر المباريات","output":"بي-ان-سبورتس-بث-مباشر-المباريات","rules_applied":["Quotes removed","dash normalized","beIN Sports"]},{"input":"فرع بنك قطر الوطني QNB المدينة التعليمية","output":"فرع-بنك-الوطني-qnb-المدينة-التعليمية","rules_applied":["Mixed script preserved (QNB brand)","stopwords removed (قطر)","Education City"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-QA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-SA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Arabic (Saudi Arabia) Slugification',
    s.content = 'URL slug generation rules for ar-SA',
    s.slug_rule = 'native_script',
    s.stopwords = '{"verb":["كان"],"negation":["لم"],"currency":["ريال"],"demonstrative":["هذا","هذه"],"preposition":["من","في","على","الى","عن"],"conjunction":["ان"]}',
    s.stopwords_count = 11,
    s.regional_additions = '[{"word":"","category":"السعودية","reason":"proper noun"},{"word":"","category":"المملكة","reason":"proper noun"},{"word":"","category":"ريال","reason":"currency"},{"word":"","category":"سعودي","reason":"adjective"},{"word":"","category":"المقيم","reason":"noun"},{"word":"","category":"الاقامة","reason":"noun"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":"remove_tashkeel","numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful URL"},{"condition":"Slug > 60 chars","message":"Consider shortening for usability"},{"condition":"Mixed scripts","message":"Latin/Arabic mix detected - verify intentional"},{"condition":"Tashkeel present","message":"Diacritics should be removed"}]',
    s.examples = '[{"input":"وظائف الرياض للسعوديين","output":"وظائف-الرياض-للسعوديين","rules_applied":["Spaces to hyphens","Arabic preserved"]},{"input":"عقارات جدة للبيع","output":"عقارات-جدة-للبيع","rules_applied":["Spaces to hyphens","city name preserved"]},{"input":"فنادق قريبة من الحرم المكي","output":"فنادق-قريبة-الحرم-المكي","rules_applied":["Stopwords removed (من)","Mecca reference preserved"]},{"input":"موسم الرياض فعاليات 2025","output":"موسم-الرياض-فعاليات-2025","rules_applied":["Numbers preserved","Riyadh Season event"]},{"input":"تاشيرة زيارة السعودية في المنصة الموحدة","output":"تاشيرة-زيارة-المنصة-الموحدة","rules_applied":["Stopwords removed (في)","country name removed"]},{"input":"مشاريع نيوم السياحية والترفيهية في تبوك شمال غرب المملكة العربية السعودية","output":"مشاريع-نيوم-السياحية-والترفيهية-تبوك-شمال-غرب-العربية","rules_applied":["Long title truncated at 80 chars","stopwords removed"]},{"input":"ما هي شروط الحصول على رخصة قيادة؟","output":"شروط-الحصول-رخصة-قيادة","rules_applied":["Punctuation removed (؟)","stopwords removed (ما، هي، على)"]},{"input":"\"العلا\" وجهة سياحية عالمية","output":"العلا-وجهة-سياحية-عالمية","rules_applied":["Quotes removed","AlUla destination preserved"]},{"input":"رحلات طيران السعودية و الخطوط الجوية","output":"رحلات-طيران-الخطوط-الجوية","rules_applied":["Stopwords removed (و)","Saudia Airlines reference"]},{"input":"خدمات مقيم للمقيمين في نطاقات","output":"خدمات-مقيم-للمقيمين-نطاقات","rules_applied":["Stopwords removed (في)","Muqeem/Nitaqat preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-SA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ar-TN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AR (TN) Slugification',
    s.content = 'URL slug generation rules for ar-TN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"interrogative":["كيف"],"article":["ال"],"pronoun":["هو","هي"],"demonstrative":["هذا","هذه","هاذي","هاذا"],"conjunction":["و","أو","ثم","فما"],"verb":["كان","يكون"],"preposition":["في","من","إلى","على","عن","مع"],"adverb":["برشا","توة"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[{"word":"","category":"فما","reason":"conjunction"},{"word":"","category":"كيف","reason":"interrogative"},{"word":"","category":"برشا","reason":"adverb"},{"word":"","category":"توة","reason":"adverb"},{"word":"","category":"هاذي","reason":"demonstrative"},{"word":"","category":"هاذا","reason":"demonstrative"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Arabic + Latin mixing may harm consistency"},{"condition":"No Arabic chars","message":"Slug should contain native script"}]',
    s.examples = '[{"input":"دليل السياحة في تونس","output":"دليل-السياحة-تونس","rules_applied":["Stopwords: \"في\" removed"]},{"input":"أفضل المطاعم التونسية","output":"أفضل-المطاعم-التونسية","rules_applied":["Standard processing"]},{"input":"وصفات الكسكسي التقليدي","output":"وصفات-الكسكسي-التقليدي","rules_applied":["Standard processing"]},{"input":"أخبار الاقتصاد و السياسة","output":"أخبار-الاقتصاد-السياسة","rules_applied":["Stopwords: \"و\" removed"]},{"input":"شواطئ جربة الجميلة","output":"شواطئ-جربة-الجميلة","rules_applied":["Standard processing"]},{"input":"10 نصائح للسفر إلى تونس في 2025","output":"10-نصائح-للسفر-تونس-2025","rules_applied":["Numbers preserved","\"إلى\" and \"في\" removed"]},{"input":"دليل شامل ومفصل لزيارة المدينة العتيقة بتونس والتعرف على معالمها التاريخية والثقافية","output":"دليل-شامل-مفصل-لزيارة-المدينة-العتيقة-بتونس-التعرف-معالمها-التاريخية-الثقافية","rules_applied":["Truncated at 80 chars","\"و\" removed"]},{"input":"هل تونس آمنة؟ نصائح السفر!","output":"هل-تونس-آمنة-نصائح-السفر","rules_applied":["Punctuation \"؟\" and \"!\" removed"]},{"input":"«المدينة العتيقة» في تونس","output":"المدينة-العتيقة-تونس","rules_applied":["Guillemets removed","\"في\" removed"]},{"input":"مسجد الزيتونة والجامع الكبير بتونس","output":"مسجد-الزيتونة-الجامع-الكبير-بتونس","rules_applied":["Stopword \"و\" removed","Arabic preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ar-TN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'as-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AS (IN) Slugification',
    s.content = 'URL slug generation rules for as-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"verb":["হৈছে","আছে","কৰে"],"article":["এটা","এখন","এজন"],"demonstrative":["এই","সেই"],"conjunction":["আৰু","কিন্তু","বা","যদিও"],"pronoun":["মই","তুমি","তেওঁ","আমি"]}',
    s.stopwords_count = 16,
    s.regional_additions = '[{"word":"","category":"বাবে","reason":"postposition"},{"word":"","category":"সৈতে","reason":"postposition"},{"word":"","category":"বিষয়ে","reason":"postposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful content"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Bengali mixed with other scripts (except digits)"},{"condition":"Non-standard composition","message":"NFC normalization recommended"}]',
    s.examples = '[{"input":"অসমীয়া ভাষা শিকক","output":"অসমীয়া-ভাষা-শিকক","rules_applied":["Space to hyphen"]},{"input":"গুৱাহাটী ভ্ৰমণ গাইড","output":"গুৱাহাটী-ভ্ৰমণ-গাইড","rules_applied":["Space to hyphen","conjuncts preserved"]},{"input":"অসমৰ বিহু উৎসৱ","output":"অসমৰ-বিহু-উৎসৱ","rules_applied":["Space to hyphen"]},{"input":"কামৰূপ জিলাৰ ইতিহাস","output":"কামৰূপ-জিলাৰ-ইতিহাস","rules_applied":["Space to hyphen"]},{"input":"মাজুলী দ্বীপ আৰু সংস্কৃতি","output":"মাজুলী-দ্বীপ-সংস্কৃতি","rules_applied":["Stopword আৰু removed"]},{"input":"অসমীয়া ৰন্ধন ২০২৫","output":"অসমীয়া-ৰন্ধন-২০২৫","rules_applied":["Bengali digits preserved"]},{"input":"অসমৰ চাহ বাগিচাসমূহৰ বিষয়ে এটা সম্পূৰ্ণ গাইড আৰু পৰ্যটকসকলৰ বাবে প্ৰয়োজনীয় তথ্য","output":"অসমৰ-চাহ-বাগিচাসমূহৰ-বিষয়ে-সম্পূৰ্ণ-গাইড-পৰ্যটকসকলৰ-প্ৰয়োজনীয়-তথ্য","rules_applied":["Truncated to 80 chars","stopwords removed"]},{"input":"শংকৰদেৱ: অসমৰ মহাপুৰুষ!","output":"শংকৰদেৱ-অসমৰ-মহাপুৰুষ","rules_applied":["Colon and exclamation removed"]},{"input":"\"নামঘৰ\" অসমৰ পৰম্পৰা","output":"নামঘৰ-অসমৰ-পৰম্পৰা","rules_applied":["Quotes removed"]},{"input":"ব্ৰহ্মপুত্ৰ নদী ও জলপথ","output":"ব্ৰহ্মপুত্ৰ-নদী-জলপথ","rules_applied":["Conjunct ব্ৰ preserved","ও stopword removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/as-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'az-AZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'AZ (AZ) Slugification',
    s.content = 'URL slug generation rules for az-AZ',
    s.slug_rule = 'native_script',
    s.stopwords = '{"article":["bir"],"pronoun":["bu","o","nə","hansı"],"conjunction":["və","ki","amma","lakin","yəni"],"verb":["edir","olur","var","yox"],"adverb":["necə","daha","artıq"]}',
    s.stopwords_count = 17,
    s.regional_additions = '[{"word":"","category":"daha","reason":"adverb"},{"word":"","category":"artıq","reason":"adverb"},{"word":"","category":"amma","reason":"conjunction"},{"word":"","category":"lakin","reason":"conjunction"},{"word":"","category":"yəni","reason":"conjunction"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"ə stripped to e","message":"Character conversion detected - use native_script"},{"condition":"ı stripped to i","message":"Incorrect dotless i handling"}]',
    s.examples = '[{"input":"Bakıda ən yaxşı restoranlar","output":"bakıda-ən-yaxşı-restoranlar","rules_applied":["Lowercase","hyphenation","ə and ı preserved"]},{"input":"Azərbaycan mətbəxi haqqında","output":"azərbaycan-mətbəxi","rules_applied":["Stopword removed (haqqında)","ə preserved"]},{"input":"Bu il üçün ən yaxşı səyahət yerləri","output":"il-ən-yaxşı-səyahət-yerləri","rules_applied":["Stopwords removed (bu","üçün)","hyphenation"]},{"input":"Qarabağ və Azərbaycan tarixi","output":"qarabağ-azərbaycan-tarixi","rules_applied":["Conjunction removed (və)","lowercase"]},{"input":"Bakı şəhəri: turizm bələdçisi","output":"bakı-şəhəri-turizm-bələdçisi","rules_applied":["Colon removed","ə and ş preserved"]},{"input":"10 səbəb Azərbaycana səyahət etmək üçün","output":"10-səbəb-azərbaycana-səyahət-etmək","rules_applied":["Number preserved","stopword removed (üçün)"]},{"input":"Şuşa şəhərinin tarixi abidələri və mədəni irsi haqqında ətraflı məlumat","output":"şuşa-şəhərinin-tarixi-abidələri-mədəni-irsi-ətraflı-məlumat","rules_applied":["Truncated at 80 chars","stopwords removed (və","haqqında)"]},{"input":"Neft sənayesi & iqtisadiyyat: Bakı!","output":"neft-sənayesi-iqtisadiyyat-bakı","rules_applied":["Special chars removed (&",":","!)","lowercase"]},{"input":"\"Qarabağ şikəstəsi\" - Azərbaycan musiqisi","output":"qarabağ-şikəstəsi-azərbaycan-musiqisi","rules_applied":["Quotes and dash removed","ş and ə preserved"]},{"input":"İnşaat işləri: YARDIM lazımdır?","output":"inşaat-işləri-yardım-lazımdır","rules_applied":["İ→i applied","question mark removed","I→ı in YARDIM"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/az-AZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'be-BY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'BE (BY) Slugification',
    s.content = 'URL slug generation rules for be-BY',
    s.slug_rule = 'native_script',
    s.stopwords = '{"pronoun":["што","які","гэта","гэты","той","яго","яе","ён","яна","яны","мы","вы","ты","я"],"conjunction":["і","як","але","ці","бо"],"preposition":["ў","на","у","да","з","за","ад","па","пра"],"verb":["ёсць","быў","была","было","будзе","мае"]}',
    s.stopwords_count = 34,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Cyrillic with Latin (except technical terms)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"},{"condition":"Russian characters used","message":"Prefer Belarusian orthography (і not и, ў not у where appropriate)"}]',
    s.examples = '[{"input":"Падарожжа па Беларусі","output":"падарожжа-беларусі","rules_applied":["Preposition removed (па)","lowercase","hyphenation"]},{"input":"Гісторыя і культура Мінска","output":"гісторыя-культура-мінска","rules_applied":["Conjunction removed (і)","lowercase","possessive preserved"]},{"input":"Беларуская кухня: традыцыйныя рэцэпты","output":"беларуская-кухня-традыцыйныя-рэцэпты","rules_applied":["Colon removed","all Belarusian characters preserved"]},{"input":"Прырода ў Белавежскай пушчы","output":"прырода-белавежскай-пушчы","rules_applied":["Preposition removed (ў)","lowercase"]},{"input":"Як дабрацца да Браслаўскіх азёр","output":"дабрацца-браслаўскіх-азёр","rules_applied":["Conjunction (як) and preposition (да) removed"]},{"input":"10 месцаў для наведвання ў Гродна","output":"10-месцаў-наведвання-гродна","rules_applied":["Number preserved","prepositions removed (для","ў)"]},{"input":"Беларускія замкі і палацы: Мірскі замак, Нясвіжскі палац і іншыя архітэктурныя помнікі краіны","output":"беларускія-замкі-палацы-мірскі-замак-нясвіжскі-палац-іншыя-архітэктурныя-помнікі","rules_applied":["Truncated to 80 chars","conjunction (і) removed","colon removed"]},{"input":"Фестывалі & падзеі: што паглядзець у Мінску!","output":"фестывалі-падзеі-паглядзець-мінску","rules_applied":["Special chars removed (&",":","!)","stopwords removed (што","у)"]},{"input":"Рэстараны \"Беларуская хата\" ці \"Камяніца\"?","output":"рэстараны-беларуская-хата-камяніца","rules_applied":["Quotes removed","conjunction (ці) removed","question mark removed"]},{"input":"Мінск vs. Гродна: якое горад лепшы для турыстаў","output":"мінск-гродна-горад-лепшы-турыстаў","rules_applied":["Punctuation removed","stopword (якое","для) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/be-BY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'bg-BG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Bulgarian (Bulgaria) Slugification',
    s.content = 'URL slug generation rules for bg-BG',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["и","като"],"preposition":["в","на","за","с","от","по","при","до","към"],"verb":["е","бе","са","съм"]}',
    s.stopwords_count = 15,
    s.regional_additions = '[{"word":"","category":"бе","reason":"verb"},{"word":"","category":"са","reason":"verb"},{"word":"","category":"ще","reason":"particle"},{"word":"","category":"съм","reason":"verb"},{"word":"","category":"ли","reason":"particle"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed Cyrillic/Latin","message":"May indicate inconsistent transliteration"},{"condition":"Only Latin characters","message":"Consider using Cyrillic for Bulgarian content"}]',
    s.examples = '[{"input":"Новини от България","output":"новини-българия","rules_applied":["lowercase","stopword \"от\" removed","hyphen"]},{"input":"Как да готвим баница","output":"готвим-баница","rules_applied":["lowercase","stopwords \"как\"","\"да\" removed"]},{"input":"София е столицата на България","output":"софия-столицата-българия","rules_applied":["lowercase","stopwords \"е\"","\"на\" removed"]},{"input":"Пътеводител за Пловдив","output":"пътеводител-пловдив","rules_applied":["lowercase","stopword \"за\" removed"]},{"input":"Рецепти с българско кисело мляко","output":"рецепти-българско-кисело-мляко","rules_applied":["lowercase","stopword \"с\" removed"]},{"input":"10 най-добри плажове в България","output":"10-най-добри-плажове-българия","rules_applied":["numbers preserved","stopword \"в\" removed"]},{"input":"Пътуване до Варна и Бургас през лятото на 2025 година за целия семеен отпуск","output":"пътуване-варна-бургас-лятото-2025-година-целия-семеен-отпуск","rules_applied":["truncated to 80 chars","stopwords removed"]},{"input":"Шопска салата: традиционна рецепта!","output":"шопска-салата-традиционна-рецепта","rules_applied":["colon and exclamation removed"]},{"input":"\"Под игото\" - роман от Иван Вазов","output":"под-игото-роман-иван-вазов","rules_applied":["quotes and dash removed","stopword \"от\" removed"]},{"input":"СОФИЯ 2025: Европейска столица","output":"софия-2025-европейска-столица","rules_applied":["uppercase to lowercase","colon removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/bg-BG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'bn-BD'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Bengali (Bangladesh) Slugification',
    s.content = 'URL slug generation rules for bn-BD',
    s.slug_rule = 'native_script',
    s.stopwords = '{"article":["এ","এই","একটি","এক"],"pronoun":["যে","যা","তা","সে","তার","আমি","আমার","তুমি","আপনি"],"verb":["হয়","ছিল","হবে","করা","করে","আছে","নেই","হয়েছে","হচ্ছে"],"conjunction":["ও","এবং","কিন্তু","তবে"],"preposition":["থেকে","জন্য","সাথে","মধ্যে","উপর","নিচে","কাছে","দ্বারা"],"interrogative":["কেমন"]}',
    s.stopwords_count = 35,
    s.regional_additions = '[{"word":"","category":"কেমন","reason":"interrogative"},{"word":"","category":"আছে","reason":"verb"},{"word":"","category":"নেই","reason":"verb"},{"word":"","category":"হয়েছে","reason":"verb"},{"word":"","category":"হচ্ছে","reason":"verb"},{"word":"","category":"দেশের","reason":"genitive"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Review unless technical term"},{"condition":"Contains only Latin","message":"Consider using Bengali for local SEO"}]',
    s.examples = '[{"input":"বাংলাদেশের সেরা খাবার","output":"বাংলাদেশের-সেরা-খাবার","rules_applied":["preserve script","space→hyphen"]},{"input":"ঢাকা শহরের ইতিহাস","output":"ঢাকা-শহরের-ইতিহাস","rules_applied":["preserve script","space→hyphen"]},{"input":"কক্সবাজার ভ্রমণ গাইড","output":"কক্সবাজার-ভ্রমণ-গাইড","rules_applied":["preserve script","conjuncts preserved"]},{"input":"সুন্দরবন একটি সুন্দর বন","output":"সুন্দরবন-সুন্দর-বন","rules_applied":["stopword একটি removed"]},{"input":"মুক্তিযুদ্ধের গল্প ও কাহিনী","output":"মুক্তিযুদ্ধের-গল্প-কাহিনী","rules_applied":["stopword ও removed"]},{"input":"বাংলাদেশ ২০২৬ সালে উন্নয়ন","output":"বাংলাদেশ-২০২৬-সালে-উন্নয়ন","rules_applied":["Bengali digits preserved"]},{"input":"বাংলাদেশের অর্থনৈতিক উন্নয়ন এবং ভবিষ্যৎ পরিকল্পনা সম্পর্কে বিস্তারিত আলোচনা","output":"বাংলাদেশের-অর্থনৈতিক-উন্নয়ন-ভবিষ্যৎ-পরিকল্পনা-সম্পর্কে-বিস্তারিত-আলোচনা","rules_applied":["truncate to 80 chars","stopword এবং removed"]},{"input":"রবীন্দ্রনাথ ঠাকুর: জীবনী!","output":"রবীন্দ্রনাথ-ঠাকুর-জীবনী","rules_applied":["colon and exclamation removed"]},{"input":"\"পদ্মা সেতু\" নির্মাণ","output":"পদ্মা-সেতু-নির্মাণ","rules_applied":["quotes removed"]},{"input":"বাংলা+ইংরেজি+Mixed","output":"বাংলাইংরেজিmixed","rules_applied":["plus signs removed","mixed script warning"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/bn-BD.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'bn-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'BN (IN) Slugification',
    s.content = 'URL slug generation rules for bn-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"article":["একটি","কোনো"],"verb":["হয়","আছে","করা"],"conjunction":["ও","এবং","কিন্তু","বা"],"pronoun":["তা"]}',
    s.stopwords_count = 10,
    s.regional_additions = '[{"word":"","category":"পশ্চিমবঙ্গ","reason":"proper noun"},{"word":"","category":"কলকাতা","reason":"proper noun"},{"word":"","category":"প্রতি","reason":"postposition"},{"word":"","category":"সাথে","reason":"postposition"},{"word":"","category":"জন্য","reason":"postposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Unexpected Latin/Bengali mixing (except numbers/technical terms)"},{"condition":"Non-NFC normalization","message":"Unicode should be NFC normalized"}]',
    s.examples = '[{"input":"ভারতের সেরা পর্যটন স্থান","output":"ভারতের-সেরা-পর্যটন-স্থান","rules_applied":["NFC","space to hyphen"]},{"input":"কলকাতায় দুর্গাপূজা উৎসব","output":"কলকাতায়-দুর্গাপূজা-উৎসব","rules_applied":["NFC","space to hyphen"]},{"input":"বাংলা সাহিত্যের ইতিহাস","output":"বাংলা-সাহিত্যের-ইতিহাস","rules_applied":["NFC","space to hyphen"]},{"input":"রবীন্দ্রনাথ ঠাকুরের কবিতা","output":"রবীন্দ্রনাথ-ঠাকুরের-কবিতা","rules_applied":["NFC","conjuncts preserved"]},{"input":"সুন্দরবনের রয়্যাল বেঙ্গল টাইগার","output":"সুন্দরবনের-রয়্যাল-বেঙ্গল-টাইগার","rules_applied":["NFC","nukta preserved"]},{"input":"২০২৬ সালের বাজেট বিশ্লেষণ","output":"২০২৬-সালের-বাজেট-বিশ্লেষণ","rules_applied":["NFC","Bengali digits preserved"]},{"input":"পশ্চিমবঙ্গের ঐতিহাসিক স্থাপত্য ও সাংস্কৃতিক ঐতিহ্যের সম্পূর্ণ নির্দেশিকা আপনার জন্য","output":"পশ্চিমবঙ্গের-ঐতিহাসিক-স্থাপত্য-সাংস্কৃতিক-ঐতিহ্যের-সম্পূর্ণ-নির্দেশিকা-আপনার","rules_applied":["Truncated at 80 chars","stopwords \"ও\"","\"জন্য\" removed"]},{"input":"মিষ্টি দই: বাংলার ঐতিহ্য!","output":"মিষ্টি-দই-বাংলার-ঐতিহ্য","rules_applied":["Punctuation (: !) removed"]},{"input":"\"আমার সোনার বাংলা\" গানের ইতিহাস","output":"আমার-সোনার-বাংলা-গানের-ইতিহাস","rules_applied":["Quotes removed"]},{"input":"হাওড়া ব্রিজ ও গঙ্গা নদী","output":"হাওড়া-ব্রিজ-গঙ্গা-নদী","rules_applied":["Stopword \"ও\" removed","conjuncts preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/bn-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'bs-BA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'BS (BA) Slugification',
    s.content = 'URL slug generation rules for bs-BA',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["i","a","ali","ili","da","te","pa","jer"],"preposition":["u","na","za","sa","od","do","po","iz","kod","o","kroz"]}',
    s.stopwords_count = 19,
    s.regional_additions = '[{"word":"","category":"te","reason":"conjunction"},{"word":"","category":"pa","reason":"conjunction"},{"word":"","category":"jer","reason":"conjunction"},{"word":"","category":"kod","reason":"preposition"},{"word":"","category":"o","reason":"preposition"},{"word":"","category":"kroz","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Missing diacritics","message":"May indicate encoding issue"}]',
    s.examples = '[{"input":"Kako napraviti domaću pitu","output":"kako-napraviti-domaću-pitu","rules_applied":["lowercase","stopwords (none)","hyphenate"]},{"input":"Sarajevo u srcu Bosne","output":"sarajevo-srcu-bosne","rules_applied":["lowercase","stopword (u) removed","hyphenate"]},{"input":"Đevđelija i Mostar","output":"đevđelija-mostar","rules_applied":["lowercase","preserve đ","stopword (i) removed"]},{"input":"Čevapi sa kajmakom","output":"čevapi-kajmakom","rules_applied":["lowercase","preserve č","stopword (sa) removed"]},{"input":"Šta je novo u Zenici","output":"šta-novo-zenici","rules_applied":["lowercase","preserve š","stopwords (je","u) removed"]},{"input":"10 najboljih plaža 2025","output":"10-najboljih-plaža-2025","rules_applied":["numbers preserved","preserve ž"]},{"input":"Bosanskohercegovačka tradicionalna kuhinja i sve što trebate znati o pripremi jela za ramazan","output":"bosanskohercegovačka-tradicionalna-kuhinja-sve-što-trebate-znati-pripremi-jela-ramazan","rules_applied":["truncated at 80 chars","stopwords removed"]},{"input":"Mostar: Stari Most & rijeka Neretva!","output":"mostar-stari-most-rijeka-neretva","rules_applied":["punctuation removed","ampersand removed"]},{"input":"\"Sevdalinka\" - duša Bosne","output":"sevdalinka-duša-bosne","rules_applied":["quotes removed","dash normalized"]},{"input":"Džamija cara Mehmeda u Sarajevu","output":"džamija-cara-mehmeda-sarajevu","rules_applied":["preserve dž digraph","stopword (u) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/bs-BA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ca-AD'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'CA (AD) Slugification',
    s.content = 'URL slug generation rules for ca-AD',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["el","la","els","les","un","una"],"conjunction":["i","o","que"],"contraction":["del","al","pel"],"verb":["és"],"preposition":["de","a","en","per","amb"]}',
    s.stopwords_count = 18,
    s.regional_additions = '[{"word":"","category":"del","reason":"contraction"},{"word":"","category":"al","reason":"contraction"},{"word":"","category":"pel","reason":"contraction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters"}]',
    s.examples = '[{"input":"Les millors pistes d\'esquí d\'Andorra","output":"millors-pistes-desquí-dandorra","rules_applied":["Stopwords (les","d\')","lowercase","apostrophe handling"]},{"input":"Guia de la Parròquia d\'Escaldes-Engordany","output":"guia-parròquia-descaldes-engordany","rules_applied":["Stopwords (de","la","d\')","diacritics preserved"]},{"input":"El Pas de la Casa: destinació turística","output":"pas-casa-destinació-turística","rules_applied":["Stopwords (el","de","la)","punctuation removed"]},{"input":"Andorra la Vella, capital del Principat","output":"andorra-vella-capital-principat","rules_applied":["Stopwords (la","del)","comma removed"]},{"input":"Museu del Tabac a Sant Julià de Lòria","output":"museu-tabac-sant-julià-lòria","rules_applied":["Stopwords (del","a","de)","diacritics preserved"]},{"input":"10 consells per a l\'hivern 2025","output":"10-consells-lhivern-2025","rules_applied":["Numbers preserved","stopwords (per","a","l\') removed"]},{"input":"Experiència gastronòmica als restaurants de muntanya del Principat d\'Andorra amb vistes espectaculars","output":"experiència-gastronòmica-restaurants-muntanya-principat-dandorra-vistes","rules_applied":["Long title truncated at word boundary","stopwords removed"]},{"input":"Què fer a Ordino? Activitats i excursions!","output":"què-fer-ordino-activitats-excursions","rules_applied":["Punctuation (?","!) removed","stopwords (a","i) removed"]},{"input":"L\'art romànic a les esglésies d\'Andorra","output":"lart-romànic-esglésies-dandorra","rules_applied":["Apostrophe handling","stopwords (a","les","d\') removed"]},{"input":"Banca i finances: el sector financer d\'Andorra","output":"banca-finances-sector-financer-dandorra","rules_applied":["Colon removed","stopwords (i","el","d\') removed","diacritics preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ca-AD.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ca-ES'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'CA (ES) Slugification',
    s.content = 'URL slug generation rules for ca-ES',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["el","la","els","les","un","una","uns","unes"],"conjunction":["i","o","ni","pero","sino","que","com"],"pronoun":["aquest","aquesta","aquell","aquella"],"preposition":["de","a","en","per","amb","sense","sobre","sota","entre","cap","fins"],"verb":["es","ser","estar","haver"]}',
    s.stopwords_count = 34,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Latin with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"Guia per a Barcelona","output":"guia-barcelona","rules_applied":["Stopwords removed (per","a)","lowercase","hyphenation"]},{"input":"La cuina catalana tradicional","output":"cuina-catalana-tradicional","rules_applied":["Article removed (la)","lowercase"]},{"input":"Els millors restaurants de Girona","output":"millors-restaurants-girona","rules_applied":["Articles removed (els)","preposition removed (de)","lowercase"]},{"input":"Consells per viatjar a Mallorca","output":"consells-viatjar-mallorca","rules_applied":["Prepositions removed (per","a)","lowercase"]},{"input":"Historia i cultura de Catalunya","output":"historia-cultura-catalunya","rules_applied":["Conjunction removed (i)","preposition removed (de)","accents preserved"]},{"input":"10 llocs per visitar a Tarragona","output":"10-llocs-visitar-tarragona","rules_applied":["Number preserved","stopwords removed (per","a)"]},{"input":"L\'art modernista a Barcelona: una guia completa sobre les obres de Gaudi i la seva influencia en l\'arquitectura catalana","output":"lart-modernista-barcelona-guia-completa-obres-gaudi-seva-influencia-larquitectura","rules_applied":["Truncated to 80 chars","stopwords removed (a","una","sobre","les","de","i","la","en)"]},{"input":"Excursions & senderisme: rutes pels Pirineus!","output":"excursions-senderisme-rutes-pirineus","rules_applied":["Special chars removed (&",":","!)","stopword removed (pels)","lowercase"]},{"input":"Vins \"de la terra\" o embotellats?","output":"vins-terra-embotellats","rules_applied":["Quotes removed","stopwords (de","la","o) removed","question mark removed"]},{"input":"Pel·licules en catala: el cinema a casa nostra","output":"pel·licules-catala-cinema-casa-nostra","rules_applied":["Ela geminada preserved (l·l)","colon removed","stopwords removed (en","el","a)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ca-ES.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ceb-PH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'CEB (PH) Slugification',
    s.content = 'URL slug generation rules for ceb-PH',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"demonstrative":["kini"],"preposition":["sa","tungod","para"],"conjunction":["ug","o","kay","apan","bisan","aron"],"adverb":["usab"],"pronoun":["ako","siya"],"article":["ang"]}',
    s.stopwords_count = 14,
    s.regional_additions = '[{"word":"","category":"bisan","reason":"conjunction"},{"word":"","category":"tungod","reason":"preposition"},{"word":"","category":"para","reason":"preposition"},{"word":"","category":"aron","reason":"conjunction"},{"word":"","category":"usab","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Excessive stopword removal","message":"More than 60% words removed"}]',
    s.examples = '[{"input":"Ang Labing Maayo nga mga Lugar sa Cebu","output":"labing-maayo-lugar-cebu","rules_applied":["Stopwords (ang","nga","mga","sa) removed","lowercase"]},{"input":"Unsaon Pagluto ug Adobo sa Balay","output":"unsaon-pagluto-adobo-balay","rules_applied":["Stopwords (ug","sa) removed","lowercase"]},{"input":"Mga Tip para sa Negosyo sa 2025","output":"tip-negosyo-2025","rules_applied":["Stopwords (mga","para","sa) removed","numbers kept"]},{"input":"Kinabuhi sa Probinsya ug sa Siyudad","output":"kinabuhi-probinsya-siyudad","rules_applied":["Stopwords (sa","ug) removed","lowercase"]},{"input":"Ang Istorya ni Rizal: Bayani sa Pilipinas","output":"istorya-rizal-bayani-pilipinas","rules_applied":["Stopwords (ang","ni","sa) removed","colon stripped"]},{"input":"Top 10 nga Pagkaon sa Bisaya","output":"top-10-pagkaon-bisaya","rules_applied":["Stopwords (nga","sa) removed","number preserved"]},{"input":"Ang Kasaysayan sa Pilipinas gikan sa Panahon sa mga Kastila hangtod Karon nga Panahon ug ang mga Epekto Niini","output":"kasaysayan-pilipinas-gikan-panahon-kastila-hangtod-karon-panahon-epekto-niini","rules_applied":["Truncated at 80 chars","stopwords removed"]},{"input":"Unsa ang Impormasyon? Mga Tubag & Tips!","output":"unsa-impormasyon-tubag-tips","rules_applied":["Punctuation (?","&","!) removed","stopwords stripped"]},{"input":"\"Maayong Buntag\" sa Sinugbuanon","output":"maayong-buntag-sinugbuanon","rules_applied":["Quotes removed","stopword (sa) removed"]},{"input":"Mga Pulong nga May Ñ: Señorita ug Año","output":"pulong-may-n-senorita-ano","rules_applied":["Ñ stripped to n","stopwords removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ceb-PH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'cs-CZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Czech (Czechia) Slugification',
    s.content = 'URL slug generation rules for cs-CZ',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["je","jsou","byl","byla","bylo","být","mít","má"],"preposition":["v","na","do","z","ze","k","ke","o","s","se","pro","při","po","od","před","za","mezi","pod","nad"],"pronoun":["ten","to","ta","který","která","které"],"conjunction":["a","ale","i","nebo","že","když","protože"]}',
    s.stopwords_count = 40,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Should contain only Latin script"},{"condition":"NFC normalization failed","message":"Check diacritic composition"}]',
    s.examples = '[{"input":"Jak vařit guláš v roce 2025","output":"jak-vařit-guláš-roce-2025","rules_applied":["Stopwords removed (v","v)","diacritics preserved"]},{"input":"Nejlepší česká restaurace na Starém Městě","output":"nejlepší-česká-restaurace-starém-městě","rules_applied":["Stopwords removed (na)","diacritics preserved"]},{"input":"Průvodce Prahou pro turisty","output":"průvodce-prahou-turisty","rules_applied":["Stopwords removed (pro)","diacritics preserved"]},{"input":"Škoda Auto: Historie a současnost","output":"škoda-auto-historie-současnost","rules_applied":["Special chars removed (:)","stopwords removed (a)","diacritics preserved"]},{"input":"Žijeme v digitální době","output":"žijeme-digitální-době","rules_applied":["Stopwords removed (v)","diacritics preserved"]},{"input":"10 tipů jak ušetřit na dovolené","output":"10-tipů-ušetřit-dovolené","rules_applied":["Numbers preserved","stopwords removed (jak","na)","diacritics preserved"]},{"input":"Komplexní průvodce správou financí: Od základů k pokročilým strategiím pro moderní investory","output":"komplexní-průvodce-správou-financí-základů-pokročilým-strategiím-moderní","rules_applied":["Truncated at 80 chars","stopwords removed (k","pro)","special chars removed"]},{"input":"Co je to \"umělá inteligence\"?","output":"umělá-inteligence","rules_applied":["Stopwords removed (co","je","to)","quotes removed","question mark removed"]},{"input":"Život s Čechy: Příběh o přátelství","output":"život-čechy-příběh-přátelství","rules_applied":["Stopwords removed (s","o)","colon removed","diacritics preserved"]},{"input":"Řízení automobilů se systémem eCall & GPS","output":"řízení-automobilů-systémem-ecall-gps","rules_applied":["Stopwords removed (se)","ampersand removed","unique Czech ř preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/cs-CZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'cy-GB'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'CY (GB) Slugification',
    s.content = 'URL slug generation rules for cy-GB',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["yng","ym","wrth","heb","fel"]}',
    s.stopwords_count = 5,
    s.regional_additions = '[{"word":"","category":"yng","reason":"preposition"},{"word":"","category":"ym","reason":"preposition"},{"word":"","category":"wrth","reason":"preposition"},{"word":"","category":"heb","reason":"preposition"},{"word":"","category":"fel","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Verify intentional use"}]',
    s.examples = '[{"input":"Croeso i Gymru","output":"croeso-gymru","rules_applied":["stopwords: i"]},{"input":"Castell Caernarfon","output":"castell-caernarfon","rules_applied":["lowercase"]},{"input":"Yr Eisteddfod Genedlaethol","output":"eisteddfod-genedlaethol","rules_applied":["stopwords: yr"]},{"input":"Bwyd a Diod yng Nghymru","output":"bwyd-diod-nghymru","rules_applied":["stopwords: a","yng"]},{"input":"Dŵr Cymru a\'r Amgylchedd","output":"dŵr-cymru-amgylchedd","rules_applied":["stopwords: a","preserve ŵ"]},{"input":"10 Peth i\'w Weld yn Eryri","output":"10-peth-weld-eryri","rules_applied":["numbers kept","stopwords: i","yn"]},{"input":"Hanes Hir y Genedl Gymreig a\'i Thraddodiadau Diwylliannol Cyfoethog drwy\'r Oesoedd","output":"hanes-hir-genedl-gymreig-thraddodiadau-diwylliannol-cyfoethog-drwyr-oesoedd","rules_applied":["truncated ≤80","stopwords: y","a"]},{"input":"Cerddoriaeth, Celf a Dawns","output":"cerddoriaeth-celf-dawns","rules_applied":["punctuation removed","stopwords: a"]},{"input":"\"Cymru Am Byth\" - Arwyddair Cenedlaethol","output":"cymru-byth-arwyddair-cenedlaethol","rules_applied":["quotes removed","stopwords: am"]},{"input":"Llŷn ac Eifionydd","output":"llŷn-eifionydd","rules_applied":["preserve ŷ","stopwords: ac"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/cy-GB.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'da-DK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Danish (Denmark) Slugification',
    s.content = 'URL slug generation rules for da-DK',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["der","som","sig"],"verb":["er"],"article":["en","et","den","det","de"],"preposition":["i","på","af","til","for","med","fra","om"],"conjunction":["og","eller","men"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO value"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"No alphabetic chars","message":"Slug must contain at least one letter"}]',
    s.examples = '[{"input":"Den bedste kaffebar i København","output":"bedste-kaffebar-københavn","rules_applied":["Stopwords removed (den","i)","æ→æ","ø→ø preserved"]},{"input":"Sådan laver du hjemmelavet smørrebrød","output":"sådan-laver-hjemmelavet-smørrebrød","rules_applied":["å→å","ø→ø preserved","stopwords removed (du)"]},{"input":"Guide til de 10 bedste strande i Danmark","output":"guide-10-bedste-strande-danmark","rules_applied":["Stopwords removed (til","de","i)","number preserved"]},{"input":"Hvorfor er hygge så vigtigt for danskere?","output":"hygge-vigtigt-danskere","rules_applied":["Stopwords removed (hvorfor","er","så","for)","? removed"]},{"input":"Bæredygtig transport & grøn energi","output":"bæredygtig-transport-grøn-energi","rules_applied":["æ→æ","ø→ø preserved","& removed"]},{"input":"\"Nyt fra designverdenen\" - ugens highlights","output":"nyt-designverdenen-ugens-highlights","rules_applied":["Quotes removed","stopwords removed (fra)"]},{"input":"Fjordlandet: oplev naturens skønhed med egen båd","output":"fjordlandet-oplev-naturens-skønhed-egen-båd","rules_applied":["ø→ø preserved","stopwords removed (med)",": removed"]},{"input":"En omfattende guide til økologisk landbrug i det 21. århundrede","output":"omfattende-guide-økologisk-landbrug-21-århundrede","rules_applied":["Long title truncated if >80 chars","ø→ø preserved"]},{"input":"Læs mere om Københavns gader og deres historier","output":"læs-mere-københavns-gader-deres-historier","rules_applied":["æ→æ","ø→ø preserved","stopwords removed (om","og)"]},{"input":"Årets bedste opskrifter på æbleskiver og frikadeller","output":"årets-bedste-opskrifter-æbleskiver-frikadeller","rules_applied":["å→å","æ→æ preserved","stopwords removed (på","og)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/da-DK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'de-AT'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'German (Austria) Slugification',
    s.content = 'URL slug generation rules for de-AT',
    s.slug_rule = 'latin_transform',
    s.stopwords = '{"verb":["ist","sind","hat","haben","wird","werden"],"conjunction":["und","oder","aber"],"article":["der","die","das","ein","eine"],"preposition":["in","im","an","am","auf","aus","bei","mit","nach","von","vor","zu","zum","zur","fuer","durch","um","ueber","unter"]}',
    s.stopwords_count = 33,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"No umlauts in original","message":"Expected for German content"}]',
    s.examples = '[{"input":"Die besten Schnitzel in Wien","output":"besten-schnitzel-wien","rules_applied":["stopwords (die","in)","lowercase","spaces→hyphens"]},{"input":"Österreichische Küche und Tradition","output":"oesterreichische-kueche-tradition","rules_applied":["ö→oe","ü→ue","stopword (und)"]},{"input":"Skifahren in Tirol für Anfänger","output":"skifahren-tirol-anfaenger","rules_applied":["stopwords (in","fuer)","ä→ae"]},{"input":"Wiener Kaffeehäuser im Überblick","output":"wiener-kaffeehaeuser-ueberblick","rules_applied":["ä→ae","ü→ue","stopword (im)"]},{"input":"Wandern am Großglockner","output":"wandern-grossglockner","rules_applied":["ß→ss","stopword (am)"]},{"input":"10 Tipps für den Städtetrip nach Salzburg","output":"10-tipps-staedtetrip-salzburg","rules_applied":["number kept","ä→ae","stopwords (fuer","den","nach)"]},{"input":"Die schönsten Wanderwege durch Österreichs Alpen und Täler im Sommer","output":"schoensten-wanderwege-oesterreichs-alpen-taeler-sommer","rules_applied":["truncation not needed","ö→oe","ä→ae","stopwords (die","durch","und","im)"]},{"input":"Kunst & Kultur: Was gibt es zu sehen?","output":"kunst-kultur-was-gibt-sehen","rules_applied":["ampersand removed","punctuation removed","stopwords (es","zu)"]},{"input":"Der \"Heurige\" – Wiens Weinkultur","output":"heurige-wiens-weinkultur","rules_applied":["quotes removed","dash removed","stopword (der)"]},{"input":"Größe und Maße: Das müssen Sie wissen","output":"groesse-masse-muessen-wissen","rules_applied":["ö→oe","ß→ss","ü→ue","stopwords (und","das","sie)","punctuation removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/de-AT.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'de-CH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'German (Switzerland) Slugification',
    s.content = 'URL slug generation rules for de-CH',
    s.slug_rule = 'latin_transform',
    s.stopwords = '{"conjunction":["und","oder","aber"],"verb":["ist","sind","wird","werden"],"preposition":["in","auf","mit","für","von","zu","bei","uf","bim","zum","zur","vom","im","am"],"article":["der","die","das","ein","eine"]}',
    s.stopwords_count = 26,
    s.regional_additions = '[{"word":"","category":"uf","reason":"preposition"},{"word":"","category":"bim","reason":"preposition"},{"word":"","category":"zum","reason":"preposition"},{"word":"","category":"zur","reason":"preposition"},{"word":"","category":"vom","reason":"preposition"},{"word":"","category":"im","reason":"preposition"},{"word":"","category":"am","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Original contained umlauts","message":"Expected for German content"},{"condition":"French characters detected","message":"Verify proper stripping for multilingual Swiss content"}]',
    s.examples = '[{"input":"Die besten Wanderwege der Schweiz","output":"besten-wanderwege-schweiz","rules_applied":["stopwords: die","der; lowercase"]},{"input":"Zürich Hauptbahnhof Öffnungszeiten","output":"zuerich-hauptbahnhof-oeffnungszeiten","rules_applied":["ü→ue","Ö→oe; lowercase"]},{"input":"Unsere Käsespezialitäten aus dem Emmental","output":"unsere-kaesespezialtaeten-emmental","rules_applied":["ä→ae; stopwords: aus","dem"]},{"input":"Ferien in der Romandie für Familien","output":"ferien-romandie-familien","rules_applied":["stopwords: in","der","für"]},{"input":"Das Matterhorn und die Alpen","output":"matterhorn-alpen","rules_applied":["stopwords: das","und","die"]},{"input":"10 Tipps für Skifahren 2025","output":"10-tipps-skifahren-2025","rules_applied":["numbers preserved; stopwords: für"]},{"input":"Die umfassende Anleitung zur Einrichtung Ihres neuen Smart-Home-Systems in der Schweiz mit allen wichtigen Details","output":"umfassende-anleitung-einrichtung-ihres-neuen-smart-home-systems-schweiz-allen-wi","rules_applied":["truncated at 80 chars; stopwords removed"]},{"input":"Rösti, Fondue & Raclette: Schweizer Küche!","output":"roesti-fondue-raclette-schweizer-kueche","rules_applied":["ö→oe","ü→ue; &:!; removed"]},{"input":"\"Grüezi mitenand\" - Schweizer Begrüssung","output":"grueezi-mitenand-schweizer-begruessung","rules_applied":["ü→ue; quotes and hyphen removed"]},{"input":"ÖV-Verbindungen Genève-Bern","output":"oev-verbindungen-geneve-bern","rules_applied":["Ö→oe; è→e (French stripped)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/de-CH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'de-DE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'German (Germany) Slugification',
    s.content = 'URL slug generation rules for de-DE',
    s.slug_rule = 'latin_transform',
    s.stopwords = '{"conjunction":["und","oder","aber"],"verb":["ist","sind","war","sein","werden"],"article":["der","die","das","ein","eine","des","dem","den"],"contraction":["im","am","zum","zur","vom","beim"],"preposition":["in","an","auf","für","von","mit","zu","bei","aus"]}',
    s.stopwords_count = 31,
    s.regional_additions = '[{"word":"","category":"des","reason":"article"},{"word":"","category":"dem","reason":"article"},{"word":"","category":"den","reason":"article"},{"word":"","category":"vom","reason":"contraction"},{"word":"","category":"beim","reason":"contraction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Original contained umlauts","message":"Expected for German (informational)"}]',
    s.examples = '[{"input":"Die besten Bücher für Anfänger","output":"besten-buecher-anfaenger","rules_applied":["Stopwords (die","für) removed","ü→ue"]},{"input":"Größe und Qualität in Deutschland","output":"groesse-qualitaet-deutschland","rules_applied":["Stopwords (und","in) removed","ö→oe","ß→ss"]},{"input":"Über uns - Unser Team im Jahr 2025","output":"ueber-uns-unser-team-jahr-2025","rules_applied":["Stopwords (im) removed","Ü→ue","numbers kept"]},{"input":"Müller & Söhne GmbH: Qualität seit 1950","output":"mueller-soehne-gmbh-qualitaet-seit-1950","rules_applied":["Ampersand removed","ü→ue","ö→oe","colon removed"]},{"input":"Fußball-Bundesliga 2024/2025 Spielplan","output":"fussball-bundesliga-2024-2025-spielplan","rules_applied":["ß→ss","slash converted to hyphen"]},{"input":"Österreich: Kultur, Natur und Tradition","output":"oesterreich-kultur-natur-tradition","rules_applied":["Stopwords (und) removed","Ö→oe"]},{"input":"Die 10 größten Städte Deutschlands sind Berlin, Hamburg und München im Süden oder Norden","output":"10-groessten-staedte-deutschlands-berlin-hamburg-muenchen-sueden","rules_applied":["Stopwords removed","ö→oe","ä→ae","truncated to 80 chars"]},{"input":"Schnäppchen & Sonderangebote! Bis zu 70% reduziert @ Müller-Shop","output":"schnaeppchen-sonderangebote-bis-70-reduziert-mueller-shop","rules_applied":["Special chars (!","@","%) removed","ä→ae","ü→ue"]},{"input":"\"Der neue Volkswagen\" - Testbericht 2025","output":"neue-volkswagen-testbericht-2025","rules_applied":["Stopwords (der) removed","quotes removed"]},{"input":"Ärzte für Kinder: Pädiatrie-Facharzt Dr. Müller in Köln","output":"aerzte-kinder-paediatrie-facharzt-dr-mueller-koeln","rules_applied":["Stopwords (für","in) removed","Ä→ae","ü→ue","ö→oe"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/de-DE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'de-LU'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'DE (LU) Slugification',
    s.content = 'URL slug generation rules for de-LU',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"contraction":["im","am","zum","zur","vom","beim"],"preposition":["in","an","auf","fur","von","mit","zu","bei","aus"],"conjunction":["und","oder","aber"],"verb":["ist","sind","war","sein","werden"],"article":["der","die","das","ein","eine","des","dem","den"]}',
    s.stopwords_count = 31,
    s.regional_additions = '[{"word":"","category":"des","reason":"article"},{"word":"","category":"dem","reason":"article"},{"word":"","category":"den","reason":"article"},{"word":"","category":"vom","reason":"contraction"},{"word":"","category":"beim","reason":"contraction"},{"word":"","category":"luxemburg","reason":"proper noun"},{"word":"","category":"grossherzogtum","reason":"proper noun"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"High stopword density","message":"More than 60% words removed"}]',
    s.examples = '[{"input":"Die besten Restaurants in Luxemburg","output":"besten-restaurants","rules_applied":["Stopwords (die","in","luxemburg) removed"]},{"input":"Finanzplatz Luxemburg: Banken und Versicherungen","output":"finanzplatz-banken-versicherungen","rules_applied":["Stopwords (und) removed","colon removed"]},{"input":"Uber das Grossherzogtum Luxemburg","output":"uber","rules_applied":["Stopwords (das","grossherzogtum","luxemburg) removed","u stripped"]},{"input":"Kulturelle Vielfalt: Deutsch, Franzosisch und Luxemburgisch","output":"kulturelle-vielfalt-deutsch-franzosisch-luxemburgisch","rules_applied":["Stopwords (und) removed","o stripped","colon removed"]},{"input":"Wohnungsmarkt im Jahr 2025","output":"wohnungsmarkt-jahr-2025","rules_applied":["Stopwords (im) removed","numbers kept"]},{"input":"Die 10 grossten Arbeitgeber in Luxemburg-Stadt","output":"10-grossten-arbeitgeber-stadt","rules_applied":["Stopwords (die","in) removed","numbers kept","o stripped"]},{"input":"Europaische Union und das Europaische Parlament: Institutionen im Grossherzogtum Luxemburg mit Sitz in Kirchberg","output":"europaische-union-europaische-parlament-institutionen-sitz-kirchberg","rules_applied":["Stopwords removed","a stripped","truncated to 80 chars"]},{"input":"Muller & Sohne GmbH - Qualitat seit 1890!","output":"muller-sohne-gmbh-qualitat-seit-1890","rules_applied":["Ampersand removed","exclamation removed","u and o stripped"]},{"input":"\"Die beste Wahl\" fur Ihr Unternehmen","output":"beste-wahl-ihr-unternehmen","rules_applied":["Quotes removed","stopwords (die","fur) removed","u stripped"]},{"input":"Arztehaus Petrusse: Dr. Muller & Partner @ Gare","output":"arztehaus-petrusse-dr-muller-partner-gare","rules_applied":["Special chars (@","&",":) removed","a and u stripped"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/de-LU.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'el-CY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EL (CY) Slugification',
    s.content = 'URL slug generation rules for el-CY',
    s.slug_rule = 'native_script',
    s.stopwords = '{"adverb":["δαμέ","εκεί"],"interrogative":["πού","πώς"],"conjunction":["και","τζιαι"],"pronoun":["αυτό","μας","σας"],"preposition":["με","για","στο","από"],"article":["το","την","τον","της","του"],"verb":["είναι"],"negation":["δεν","έννεν"]}',
    s.stopwords_count = 21,
    s.regional_additions = '[{"word":"","category":"τζιαι","reason":"conjunction"},{"word":"","category":"έννεν","reason":"negation"},{"word":"","category":"δαμέ","reason":"adverb"},{"word":"","category":"εκεί","reason":"adverb"},{"word":"","category":"πολλά","reason":"adjective"},{"word":"","category":"μας","reason":"pronoun"},{"word":"","category":"σας","reason":"pronoun"},{"word":"","category":"πού","reason":"interrogative"},{"word":"","category":"πώς","reason":"interrogative"},{"word":"","category":"μεν","reason":"particle"}]',
    s.script_config = '{"primary_script":"greek","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Avoid mixing Greek with Latin (except brand names)"},{"condition":"Non-standard Unicode","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"Εργασία στην Κύπρο 2026","output":"εργασία-κύπρο-2026","rules_applied":["Stopwords removed (στην)","year preserved","lowercase"]},{"input":"Ακίνητα προς πώληση στη Λεμεσό","output":"ακίνητα-πώληση-λεμεσό","rules_applied":["Stopwords removed (προς","στη)","lowercase","hyphenation"]},{"input":"Ξενοδοχεία στην Πάφο με πισίνα","output":"ξενοδοχεία-πάφο-πισίνα","rules_applied":["Stopwords removed (στην","με)","lowercase"]},{"input":"Κυπριακό χαλούμι: Η παραδοσιακή συνταγή","output":"κυπριακό-χαλούμι-παραδοσιακή-συνταγή","rules_applied":["Colon removed","article (η) removed"]},{"input":"Σιέφταλια τζιαι λούντζα από την Τρόοδο","output":"σιέφταλια-λούντζα-τρόοδο","rules_applied":["Cypriot conjunction (τζιαι) removed","stopwords removed (από","την)"]},{"input":"5 λόγοι για να επισκεφτείτε την Αγία Νάπα","output":"5-λόγοι-επισκεφτείτε-αγία-νάπα","rules_applied":["Number preserved","stopwords removed (για","να","την)"]},{"input":"Οδηγός για την Λευκωσία: Μνημεία, μουσεία, εστιατόρια και αξιοθέατα της πρωτεύουσας","output":"οδηγός-λευκωσία-μνημεία-μουσεία-εστιατόρια-αξιοθέατα-πρωτεύουσας","rules_applied":["Truncated to 80 chars","stopwords removed"]},{"input":"Bank of Cyprus & Hellenic Bank: Σύγκριση τραπεζών!","output":"bank-of-cyprus-hellenic-bank-σύγκριση-τραπεζών","rules_applied":["Ampersand removed","exclamation removed","colon removed"]},{"input":"Κομμανταρία \"η γλυκιά\" vs Μαυροδάφνη","output":"κομμανταρία-γλυκιά-μαυροδάφνη","rules_applied":["Quotes removed","article (η) removed","vs removed"]},{"input":"Πρωταράς ή Αγία Νάπα; Ποιο θέρετρο να διαλέξω","output":"πρωταράς-αγία-νάπα-ποιο-θέρετρο-διαλέξω","rules_applied":["Conjunction (ή) removed","question mark removed","stopword (να) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/el-CY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'el-GR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Greek (Greece) Slugification',
    s.content = 'URL slug generation rules for el-GR',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["και","ή","αλλά","όμως","επειδή","ότι"],"verb":["είναι","έχω","γίνομαι","πάμε"],"article":["το","τη","την","της","τον","του","των","ο","η","οι","τα","ένα","μια","ένας"],"pronoun":["αυτό","αυτή","αυτός","εκείνο","εκείνη"],"adverb":["δεν","εδώ","τώρα","σήμερα"],"preposition":["σε","στο","στη","στην","στα","από","με","για","προς","κατά","μετά","παρά","χωρίς","ως"]}',
    s.stopwords_count = 47,
    s.regional_additions = '[{"word":"","category":"μπες","reason":"imperative"},{"word":"","category":"δες","reason":"imperative"},{"word":"","category":"πάμε","reason":"verb"},{"word":"","category":"εδώ","reason":"adverb"},{"word":"","category":"τώρα","reason":"adverb"},{"word":"","category":"σήμερα","reason":"adverb"}]',
    s.script_config = '{"primary_script":"greek","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Greek with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"Οδηγός για την Αθήνα","output":"οδηγός-αθήνα","rules_applied":["Stopwords removed (για","την)","lowercase","hyphenation"]},{"input":"Τα καλύτερα εστιατόρια στη Θεσσαλονίκη","output":"καλύτερα-εστιατόρια-θεσσαλονίκη","rules_applied":["Articles removed (τα","στη)","lowercase"]},{"input":"Διακοπές στη Μύκονο και τη Σαντορίνη","output":"διακοπές-μύκονο-σαντορίνη","rules_applied":["Stopwords removed (στη","και","τη)","lowercase"]},{"input":"Επίσκεψη στην Ακρόπολη και τα Μετέωρα","output":"επίσκεψη-ακρόπολη-μετέωρα","rules_applied":["Stopwords removed (στην","και","τα)","accents preserved"]},{"input":"Η ιστορία του σουβλακιού στην Ελλάδα","output":"ιστορία-σουβλακιού-ελλάδα","rules_applied":["Articles removed (η","του","στην)","genitive preserved"]},{"input":"25 Μαρτίου: Εθνική Επέτειος της Ελλάδας","output":"25-μαρτίου-εθνική-επέτειος-ελλάδας","rules_applied":["Number preserved","colon removed","stopwords (της) removed"]},{"input":"Τα ομορφότερα νησιά του Αιγαίου: Ρόδος, Κέρκυρα, Ηράκλειο και η μαγεία της Χαλκιδικής στην Ελλάδα","output":"ομορφότερα-νησιά-αιγαίου-ρόδος-κέρκυρα-ηράκλειο-μαγεία-χαλκιδικής-ελλάδα","rules_applied":["Truncated to 80 chars","stopwords removed (τα","του","και","η","της","στην)","commas removed"]},{"input":"Cosmote & Vodafone: Σύγκριση τιμών!","output":"cosmote-vodafone-σύγκριση-τιμών","rules_applied":["Special chars removed (&",":","!)","mixed script allowed for brands"]},{"input":"\"Μουσακάς\" ή \"Παστίτσιο\"; Τι προτιμούν οι Έλληνες","output":"μουσακάς-παστίτσιο-τι-προτιμούν-έλληνες","rules_applied":["Quotes removed","conjunction (ή) removed","question mark removed","stopwords (οι) removed"]},{"input":"Αρχαία Ολυμπία: Ο τόπος γέννησης των Ολυμπιακών Αγώνων","output":"αρχαία-ολυμπία-τόπος-γέννησης-ολυμπιακών-αγώνων","rules_applied":["Colon removed","articles (ο","των) removed","iconic Greek heritage site"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/el-GR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-AE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (AE) Slugification',
    s.content = 'URL slug generation rules for en-AE',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["of","in","to","for","on","with"],"verb":["is","are","was","were","be"],"pronoun":["it","this","that"],"currency":["dirhams","aed"],"article":["the","a","an"],"conjunction":["and","or","but"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[{"word":"","category":"uae","reason":"acronym"},{"word":"","category":"emirates","reason":"noun"},{"word":"","category":"dubai","reason":"proper noun"},{"word":"","category":"dirhams","reason":"currency"},{"word":"","category":"aed","reason":"currency"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"> 60% stopwords removed","message":"Title may be too generic"}]',
    s.examples = '[{"input":"Best Hotels in Dubai for Business Travellers","output":"best-hotels-dubai-business-travellers","rules_applied":["lowercase","stopwords (in","for)","British spelling kept"]},{"input":"The Top 10 Shopping Malls in the UAE","output":"top-10-shopping-malls","rules_applied":["stopwords (the","in","the","uae)","numbers kept"]},{"input":"Guide to Eid al-Fitr Celebrations 2025","output":"guide-eid-al-fitr-celebrations-2025","rules_applied":["stopwords (to)","special chars removed","year kept"]},{"input":"Abu Dhabi vs Dubai: Where to Invest","output":"abu-dhabi-vs-dubai-where-invest","rules_applied":["stopwords (to)","colon removed"]},{"input":"Free Zone Company Registration & Licensing","output":"free-zone-company-registration-licensing","rules_applied":["ampersand removed"]},{"input":"Top 5 Restaurants Near Burj Khalifa 2025","output":"top-5-restaurants-near-burj-khalifa-2025","rules_applied":["numbers preserved","year kept"]},{"input":"The Ultimate Guide to Living and Working in the United Arab Emirates: Everything You Need to Know About Relocating","output":"ultimate-guide-living-working-united-arab-emirates-everything-you-need-know","rules_applied":["truncated at 80 chars","stopwords removed"]},{"input":"What\'s New in Dubai? Latest Updates!","output":"whats-new-dubai-latest-updates","rules_applied":["apostrophe removed","question mark removed","exclamation removed"]},{"input":"\"Premium\" Properties for Sale in Al Ain","output":"premium-properties-sale-al-ain","rules_applied":["quotes removed","stopwords (for","in) removed"]},{"input":"Al Wasl Rd & Sheikh Zayed Rd Intersection","output":"al-wasl-rd-sheikh-zayed-rd-intersection","rules_applied":["ampersand removed","abbreviations kept"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-AE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-AU'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (AU) Slugification',
    s.content = 'URL slug generation rules for en-AU',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["in","on","at","to","for","of","with","by"],"article":["the","a","an"],"conjunction":["and","or","but","as"],"verb":["is","are","was","were","be","been"],"pronoun":["it","this","that"]}',
    s.stopwords_count = 24,
    s.regional_additions = '[{"word":"","category":"mate","reason":"colloquial"},{"word":"","category":"arvo","reason":"colloquial"},{"word":"","category":"reckon","reason":"colloquial"},{"word":"","category":"heaps","reason":"colloquial"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Stopwords > 60%","message":"Too many stopwords removed, title may need review"}]',
    s.examples = '[{"input":"The Best Coffee Shops in Melbourne","output":"best-coffee-shops-melbourne","rules_applied":["Stopwords (the","in) removed","lowercase"]},{"input":"Sydney\'s Top 10 Beaches for Summer","output":"sydneys-top-10-beaches-summer","rules_applied":["Possessive apostrophe removed","stopword (for) removed"]},{"input":"How to Save Money on Groceries","output":"how-save-money-groceries","rules_applied":["Stopwords (to","on) removed","lowercase"]},{"input":"AFL Grand Final 2025: Complete Guide","output":"afl-grand-final-2025-complete-guide","rules_applied":["Colon removed","hyphenated"]},{"input":"Where to Find the Best Barramundi","output":"where-find-best-barramundi","rules_applied":["Stopwords (to","the) removed"]},{"input":"Top 5 National Parks Near Brisbane","output":"top-5-national-parks-near-brisbane","rules_applied":["Numbers preserved","stopword handling"]},{"input":"The Ultimate Australian Outback Adventure: Exploring the Red Centre and Beyond the Beaten Track","output":"ultimate-australian-outback-adventure-exploring-red-centre-beyond-beaten-track","rules_applied":["Truncated at 80 chars","stopwords removed"]},{"input":"What\'s On: Events & Festivals!","output":"whats-events-festivals","rules_applied":["Apostrophe removed","ampersand removed","punctuation removed"]},{"input":"\"Fair Dinkum\" Aussie Barbecue Tips","output":"fair-dinkum-aussie-barbecue-tips","rules_applied":["Quotes removed","lowercase"]},{"input":"Cafe-style Flat White at Home","output":"cafe-style-flat-white-home","rules_applied":["Hyphen preserved","stopword (at) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-AU.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-BB'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (BB) Slugification',
    s.content = 'URL slug generation rules for en-BB',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"pronoun":["it","that","this","wuh","yuh"],"conjunction":["and","or","but"],"article":["the","a","an"],"verb":["is","are","was","were","be","been","being"],"preposition":["in","on","at","to","for","of","with","by","from","fuh","wid","bout"]}',
    s.stopwords_count = 30,
    s.regional_additions = '[{"word":"","category":"wuh","reason":"pronoun"},{"word":"","category":"yuh","reason":"pronoun"},{"word":"","category":"duh","reason":"determiner"},{"word":"","category":"fuh","reason":"preposition"},{"word":"","category":"wid","reason":"preposition"},{"word":"","category":"bout","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for effective SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Stopwords removed > 60%","message":"Title may be too generic"}]',
    s.examples = '[{"input":"Crop Over Festival Kadooment Day Celebration","output":"crop-over-festival-kadooment-day-celebration","rules_applied":["No stopwords removed; Barbados national festival"]},{"input":"Mount Gay Rum Distillery Tour Bridgetown","output":"mount-gay-rum-distillery-tour-bridgetown","rules_applied":["No stopwords removed; oldest rum distillery"]},{"input":"Oistins Fish Fry Friday Night Guide","output":"oistins-fish-fry-friday-night-guide","rules_applied":["No stopwords removed; famous local event"]},{"input":"Apartments for Rent in St Michael Parish","output":"apartments-rent-st-michael-parish","rules_applied":["Stopwords removed: for","in"]},{"input":"Speightstown Heritage Walking Tour and Museum","output":"speightstown-heritage-walking-tour-museum","rules_applied":["Stopwords removed: and"]},{"input":"10 Best Cou Cou Recipes with Flying Fish 2025","output":"10-best-cou-cou-recipes-flying-fish-2025","rules_applied":["Stopwords removed: with; numbers preserved"]},{"input":"The Complete Guide to Harrison\'s Cave Underground Adventure and Bathsheba Rock Formations","output":"complete-guide-harrisons-cave-underground-adventure-bathsheba-rock-formations","rules_applied":["Stopwords: the","to","and; apostrophe removed; truncated at 80 chars"]},{"input":"CIBC FirstCaribbean Bank Services & Digicel Barbados Plans!","output":"cibc-firstcaribbean-bank-services-digicel-barbados-plans","rules_applied":["Ampersand and exclamation removed"]},{"input":"What\'s the Best Macaroni Pie Recipe? A Bajan Classic","output":"best-macaroni-pie-recipe-bajan-classic","rules_applied":["Stopwords: what","s","the","a; apostrophe and question mark removed"]},{"input":"Flow Barbados Internet - Christ Church Installation @ Holetown","output":"flow-barbados-internet-christ-church-installation-holetown","rules_applied":["Hyphen connector and at symbol removed; parish name preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-BB.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-BW'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (BW) Slugification',
    s.content = 'URL slug generation rules for en-BW',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["and","or","but"],"preposition":["of","to","in","for","at","by","with"],"verb":["is","are"],"article":["the","a","an"],"pronoun":["it","this"]}',
    s.stopwords_count = 17,
    s.regional_additions = '[{"word":"","category":"at","reason":"preposition"},{"word":"","category":"by","reason":"preposition"},{"word":"","category":"with","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Too many stopwords removed","message":"More than 60% of words removed"}]',
    s.examples = '[{"input":"The Best Safari Lodges in Botswana","output":"best-safari-lodges-botswana","rules_applied":["stopwords: the","in"]},{"input":"Okavango Delta Travel Guide","output":"okavango-delta-travel-guide","rules_applied":["lowercase","spacing"]},{"input":"Traditional Kgotla Meeting Places","output":"traditional-kgotla-meeting-places","rules_applied":["lowercase","spacing"]},{"input":"A Guide to Botswana\'s Wildlife","output":"guide-botswanas-wildlife","rules_applied":["stopwords: a","to; apostrophe removed"]},{"input":"Gaborone & Francistown City Tours","output":"gaborone-francistown-city-tours","rules_applied":["ampersand removed"]},{"input":"Top 10 Things to Do in Maun 2026","output":"top-10-things-do-maun-2026","rules_applied":["stopwords: to","in; numbers kept"]},{"input":"Understanding Botho Philosophy and Traditional Values in Modern Botswana Society Today","output":"understanding-botho-philosophy-traditional-values-modern-botswana-society-today","rules_applied":["truncated at 80 chars; stopwords: and","in"]},{"input":"What\'s New: Chobe National Park Updates!","output":"whats-new-chobe-national-park-updates","rules_applied":["punctuation removed; colon removed"]},{"input":"\"Pula\" Means Both Rain and Blessings","output":"pula-means-both-rain-blessings","rules_applied":["quotes removed; stopwords: and"]},{"input":"Combi Routes: Gaborone–Lobatse–Kanye","output":"combi-routes-gaborone-lobatse-kanye","rules_applied":["colon removed; en-dash to hyphen"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-BW.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-CA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (CA) Slugification',
    s.content = 'URL slug generation rules for en-CA',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"pronoun":["it","this","that"],"conjunction":["and","or","but","as","if"],"adverb":["not"],"interjection":["eh"],"article":["the","a","an"],"preposition":["of","in","to","for","on","at","by","with","from"],"verb":["is","are","was","be"]}',
    s.stopwords_count = 26,
    s.regional_additions = '[{"word":"","category":"eh","reason":"interjection"},{"word":"","category":"aboot","reason":"regional"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful content"},{"condition":"Slug > 60 chars","message":"Consider shortening for better UX"},{"condition":"Too many stopwords removed (> 60%)","message":"Title may be too generic"},{"condition":"All-numeric slug","message":"Consider adding descriptive text"}]',
    s.examples = '[{"input":"Guide to the Canadian Rockies","output":"guide-canadian-rockies","rules_applied":["Stopwords removed (to","the)","lowercase","hyphenation"]},{"input":"Winter Toque and Mitten Care","output":"winter-toque-mitten-care","rules_applied":["Stopwords removed (and)","lowercase"]},{"input":"Best Poutine in Montreal","output":"best-poutine-montreal","rules_applied":["Stopwords removed (in)","lowercase"]},{"input":"How to Get Your G2 Licence","output":"how-get-your-g2-licence","rules_applied":["Stopwords removed (to)","Canadian spelling (licence)"]},{"input":"Understanding the Canadian Loonie and Toonie","output":"understanding-canadian-loonie-toonie","rules_applied":["Stopwords removed (the","and)","lowercase"]},{"input":"10 Things to Do in Vancouver","output":"10-things-do-vancouver","rules_applied":["Number preserved","stopwords removed (to","in)","lowercase"]},{"input":"A Comprehensive Guide to Visiting Niagara Falls: Everything You Need to Know About Canada\'s Most Famous Waterfall","output":"comprehensive-guide-visiting-niagara-falls-everything-you-need-know-canadas","rules_applied":["Truncated to 80 chars","stopwords removed (a","to","about)"]},{"input":"Hiking & Camping: Banff National Park!","output":"hiking-camping-banff-national-park","rules_applied":["Special chars removed (&",":","!)","lowercase"]},{"input":"What\'s a \"Double-Double\"? Tim Hortons Coffee Guide","output":"whats-double-double-tim-hortons-coffee-guide","rules_applied":["Apostrophe removed","quotes removed","question mark removed"]},{"input":"Hockey Night in Canada vs. NFL Sunday: Which is Better, Eh?","output":"hockey-night-canada-vs-nfl-sunday-which-better","rules_applied":["Stopwords removed (in","is)","punctuation removed (:","?",")"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-CA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-CY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (CY) Slugification',
    s.content = 'URL slug generation rules for en-CY',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"article":["a","an","the"],"preposition":["of","in","on","at","to","for","with","by","from","as"],"pronoun":["it","that","this"],"conjunction":["and","or","but"],"verb":["is","are","was","were","be","been","being"]}',
    s.stopwords_count = 26,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"}]',
    s.examples = '[{"input":"Best Halloumi Cheese in Cyprus","output":"best-halloumi-cheese-cyprus","rules_applied":["Stopwords removed: in"]},{"input":"Property for Sale in Limassol","output":"property-sale-limassol","rules_applied":["Stopwords removed: for","in"]},{"input":"A Guide to Nicosia Old Town","output":"guide-nicosia-old-town","rules_applied":["Stopwords removed: a","to"]},{"input":"Cyprus Tourism and Travel Tips","output":"cyprus-tourism-travel-tips","rules_applied":["Stopwords removed: and"]},{"input":"The Mediterranean Lifestyle Experience","output":"mediterranean-lifestyle-experience","rules_applied":["Stopwords removed: the"]},{"input":"Top 10 Beaches in Cyprus for 2025","output":"top-10-beaches-cyprus-2025","rules_applied":["Stopwords removed: in","for; numbers preserved"]},{"input":"The Complete Guide to Relocating to Cyprus: Tax Benefits, Residency Requirements and Business Opportunities","output":"complete-guide-relocating-cyprus-tax-benefits-residency-requirements-business","rules_applied":["Stopwords removed: the","to","and; colon removed; truncated at 80 chars"]},{"input":"Restaurants & Tavernas in Paphos!","output":"restaurants-tavernas-paphos","rules_applied":["Stopwords removed: in; ampersand and exclamation removed"]},{"input":"What\'s New in Cyprus\'s Tech Scene? Latest Developments","output":"new-cyprus-tech-scene-latest-developments","rules_applied":["Stopwords removed: in; apostrophes and question mark removed"]},{"input":"Villa #42 - Luxury Accommodation & Spa","output":"villa-42-luxury-accommodation-spa","rules_applied":["Hash","hyphen","ampersand removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-CY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-FJ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (FJ) Slugification',
    s.content = 'URL slug generation rules for en-FJ',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["and","or","but"],"preposition":["in","on","at","to","for","of","with","by","from","as","ni","mai"],"article":["the","a","an","na"],"pronoun":["it","this","that"],"verb":["is","are","was","were","be","been","being"]}',
    s.stopwords_count = 29,
    s.regional_additions = '[{"word":"","category":"na","reason":"article"},{"word":"","category":"ni","reason":"preposition"},{"word":"","category":"ko","reason":"particle"},{"word":"","category":"mai","reason":"preposition"},{"word":"","category":"bula","reason":"greeting"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Too many stopwords removed","message":"More than 60% of words were stopwords"}]',
    s.examples = '[{"input":"Luxury Bure Accommodation on Denarau Island","output":"luxury-bure-accommodation-denarau-island","rules_applied":["lowercase","stopword (on) removed"]},{"input":"Blue Lagoon Cruises Yasawa Itinerary","output":"blue-lagoon-cruises-yasawa-itinerary","rules_applied":["lowercase","no stopwords removed"]},{"input":"Traditional Lovo Feast at Pacific Harbour","output":"traditional-lovo-feast-pacific-harbour","rules_applied":["lowercase","stopword (at) removed"]},{"input":"Vodafone Fiji Prepaid SIM Card Guide","output":"vodafone-fiji-prepaid-sim-card-guide","rules_applied":["lowercase","no stopwords removed"]},{"input":"Coral Coast Snorkeling and Diving Spots","output":"coral-coast-snorkeling-diving-spots","rules_applied":["lowercase","stopword (and) removed"]},{"input":"7 Best Kava Ceremonies in Savusavu 2026","output":"7-best-kava-ceremonies-savusavu-2026","rules_applied":["numbers preserved","stopword (in) removed"]},{"input":"The Complete Guide to Exploring the Mamanuca Islands and Yasawa Islands for First-Time Visitors to Fiji","output":"complete-guide-exploring-mamanuca-islands-yasawa-islands-first-time-visitors","rules_applied":["truncate at 80 chars","stopwords (the","to","and","for) removed"]},{"input":"Fiji Airways Nadi to Suva: Domestic Flight Tips & Booking!","output":"fiji-airways-nadi-suva-domestic-flight-tips-booking","rules_applied":["colon removed","ampersand removed","exclamation removed","stopword (to) removed"]},{"input":"\"Fiji Time\" Culture - What It Really Means","output":"fiji-time-culture-really-means","rules_applied":["quotes removed","hyphen removed","stopwords (what","it) removed"]},{"input":"Kokoda Recipe: Fresh Walu Ceviche with Coconut Cream","output":"kokoda-recipe-fresh-walu-ceviche-coconut-cream","rules_applied":["colon removed","stopword (with) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-FJ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-GB'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'English (United Kingdom) Slugification',
    s.content = 'URL slug generation rules for en-GB',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["of","to","in","for","on","with","at","by","from"],"article":["the","a","an"],"verb":["is","are","be"],"conjunction":["and","or","but","as"],"pronoun":["it","this","that"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for effective SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Stopwords > 60% removed","message":"Original title may be too generic"}]',
    s.examples = '[{"input":"The Best Restaurants in London","output":"best-restaurants-london","rules_applied":["stopwords: the","in"]},{"input":"How to Find a Flat in Manchester","output":"how-find-flat-manchester","rules_applied":["stopwords: to","a","in"]},{"input":"Colour Trends for Interior Design","output":"colour-trends-interior-design","rules_applied":["stopwords: for; diacritic-free"]},{"input":"NHS Guide to GP Services","output":"nhs-guide-gp-services","rules_applied":["stopwords: to"]},{"input":"British Airways & Train Travel","output":"british-airways-train-travel","rules_applied":["ampersand removed"]},{"input":"Top 10 Pubs in the United Kingdom","output":"top-10-pubs-united-kingdom","rules_applied":["stopwords: in","the; numbers preserved"]},{"input":"The Complete Beginner\'s Guide to Understanding the British Property Market and Buying Your First Home","output":"complete-beginners-guide-understanding-british-property-market-buying-your-first","rules_applied":["truncated at 80 chars; stopwords: the","to","and"]},{"input":"What\'s New? The Latest Updates!","output":"whats-new-latest-updates","rules_applied":["punctuation removed; stopwords: the"]},{"input":"\'Best of British\' Theatre Productions","output":"best-british-theatre-productions","rules_applied":["quotes removed; stopwords: of"]},{"input":"BBC & ITV: UK Broadcasting Rivals","output":"bbc-itv-uk-broadcasting-rivals","rules_applied":["ampersand and colon removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-GB.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-GH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (GH) Slugification',
    s.content = 'URL slug generation rules for en-GH',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"pronoun":["it","that","this"],"article":["the","a","an"],"verb":["is","are","was","were","be","been","being"],"conjunction":["and","or","but"],"preposition":["in","on","at","to","for","with","by","from","of","as"],"interjection":["ehn"]}',
    s.stopwords_count = 27,
    s.regional_additions = '[{"word":"","category":"ooo","reason":"particle"},{"word":"","category":"oo","reason":"particle"},{"word":"","category":"paaa","reason":"intensifier"},{"word":"","category":"koraa","reason":"intensifier"},{"word":"","category":"kraa","reason":"intensifier"},{"word":"","category":"abi","reason":"tag question"},{"word":"","category":"ehn","reason":"interjection"},{"word":"","category":"saa","reason":"intensifier"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"}]',
    s.examples = '[{"input":"Best Waakye Joints in Kaneshie Accra","output":"best-waakye-joints-kaneshie-accra","rules_applied":["Stopwords removed: in"]},{"input":"How to Register for MTN MoMo in Ghana","output":"register-mtn-momo-ghana","rules_applied":["Stopwords removed: how","to","for","in"]},{"input":"Chale Wote Street Art Festival Cape Coast 2026","output":"chale-wote-street-art-festival-cape-coast-2026","rules_applied":["Numbers preserved; location preserved"]},{"input":"GCB Bank Personal Loans Kumasi Branch","output":"gcb-bank-personal-loans-kumasi-branch","rules_applied":["All content words preserved"]},{"input":"Ghana Jollof Recipe Wins African Cooking Award","output":"ghana-jollof-recipe-wins-african-cooking-award","rules_applied":["Cultural pride topic preserved"]},{"input":"Top 10 Kelewele Spots in Osu Night Market 2026","output":"top-10-kelewele-spots-osu-night-market-2026","rules_applied":["Stopwords removed: in; numbers preserved"]},{"input":"The Complete Guide to Applying for Ghana Visa at Kotoka International Airport and Immigration Requirements","output":"complete-guide-applying-ghana-visa-kotoka-international-airport-immigration","rules_applied":["Stopwords removed: the","to","for","at","and; truncated at 80 chars"]},{"input":"Elmina Castle & Cape Coast: Ghanas Heritage Sites!","output":"elmina-castle-cape-coast-ghanas-heritage-sites","rules_applied":["Stopwords: none; colon","ampersand","exclamation removed"]},{"input":"Whats Hot at Panafest 2026?","output":"hot-panafest-2026","rules_applied":["Stopwords removed: what","s","at; apostrophe and question mark removed"]},{"input":"Akwaaba to Kakum National Park Canopy Walkway Ashanti Region","output":"akwaaba-kakum-national-park-canopy-walkway-ashanti-region","rules_applied":["Stopwords removed: to; Ghanaian welcome word preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-GH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-HK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (HK) Slugification',
    s.content = 'URL slug generation rules for en-HK',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["and","or","but"],"article":["the","a","an"],"preposition":["in","on","at","to","for","of","with","by"],"verb":["is","are","was","were","be","been","being","have","has","had","do","does","did"],"pronoun":["it","this","that"],"abbreviation":["hk"]}',
    s.stopwords_count = 31,
    s.regional_additions = '[{"word":"","category":"hk","reason":"abbreviation"},{"word":"","category":"hongkong","reason":"noun"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed","message":"More than 60% of words removed"}]',
    s.examples = '[{"input":"Best Dim Sum Restaurants in Central","output":"best-dim-sum-restaurants-central","rules_applied":["stopwords: in removed"]},{"input":"Top 10 Things to Do in Hong Kong","output":"top-10-things-hong-kong","rules_applied":["stopwords: to","do","in removed"]},{"input":"MTR Guide for Visitors","output":"mtr-guide-visitors","rules_applied":["stopwords: for removed"]},{"input":"Yum Cha Culture and Traditions","output":"yum-cha-culture-traditions","rules_applied":["stopwords: and removed"]},{"input":"Victoria Harbour Night View","output":"victoria-harbour-night-view","rules_applied":["British spelling preserved (no diacritics)"]},{"input":"The 5 Best Hiking Trails in the New Territories","output":"5-best-hiking-trails-new-territories","rules_applied":["stopwords: the","in","the removed"]},{"input":"A Comprehensive Guide to Shopping in Tsim Sha Tsui and Causeway Bay for Budget-Conscious Travellers Looking for Authentic Experiences","output":"comprehensive-guide-shopping-tsim-sha-tsui-causeway-bay-budget-conscious-travel","rules_applied":["truncated at 80 chars","stopwords: a","to","in","and","for removed"]},{"input":"What\'s New: Events & Festivals 2025","output":"whats-new-events-festivals-2025","rules_applied":["special chars: \'",":","& removed"]},{"input":"Hong Kong\'s \"Pearl of the Orient\" Skyline Tour","output":"hong-kongs-pearl-orient-skyline-tour","rules_applied":["stopwords: the","of removed; quotes removed"]},{"input":"Café Culture: Kowloon\'s Best Coffee Spots","output":"cafe-culture-kowloons-best-coffee-spots","rules_applied":["diacritics: é → e; special chars: :","\' removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-HK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-IE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (IE) Slugification',
    s.content = 'URL slug generation rules for en-IE',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"contraction":["tis","twas"],"article":["the","a","an"],"preposition":["of","to","in","for","on","with","at","by","from"],"verb":["is","are","was","be"],"conjunction":["and","or","but","as"],"pronoun":["it","this","that","yere","meself","yerself"],"filler":["sure","so","like"]}',
    s.stopwords_count = 31,
    s.regional_additions = '[{"word":"","category":"tis","reason":"contraction"},{"word":"","category":"twas","reason":"contraction"},{"word":"","category":"yere","reason":"pronoun"},{"word":"","category":"meself","reason":"pronoun"},{"word":"","category":"yerself","reason":"pronoun"},{"word":"","category":"sure","reason":"filler"},{"word":"","category":"so","reason":"filler"},{"word":"","category":"like","reason":"filler"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Stopwords > 60%","message":"May indicate poor title quality"},{"condition":"Irish placenames stripped","message":"Verify fada removal acceptable"}]',
    s.examples = '[{"input":"Best Pubs in Dublin for a Night Out","output":"best-pubs-dublin-night-out","rules_applied":["lowercase","stopwords (in","for","a)","hyphens"]},{"input":"The Wild Atlantic Way Road Trip Guide","output":"wild-atlantic-way-road-trip-guide","rules_applied":["lowercase","stopwords (the)","hyphens"]},{"input":"Top 10 GAA Clubs in Cork","output":"top-10-gaa-clubs-cork","rules_applied":["lowercase","stopwords (in)","numbers preserved"]},{"input":"How to Get Your Driving Licence in Ireland","output":"how-get-your-driving-licence-ireland","rules_applied":["lowercase","stopwords (to","in)","hyphens"]},{"input":"Craic & Culture: A Weekend in Galway","output":"craic-culture-weekend-galway","rules_applied":["lowercase","stopwords (a","in)","ampersand removed"]},{"input":"15 Things to Do in Killarney This Summer","output":"15-things-do-killarney-summer","rules_applied":["lowercase","stopwords (to","in","this)","numbers preserved"]},{"input":"The Ultimate Guide to Finding a Flat in Dublin: Everything You Need to Know About Renting in Ireland\'s Capital City","output":"ultimate-guide-finding-flat-dublin-everything-you-need-know-about-renting-irelan","rules_applied":["truncated at 80 chars","stopwords removed"]},{"input":"What\'s On: Theatre, Music & Events in Cork!","output":"whats-theatre-music-events-cork","rules_applied":["lowercase","stopwords (on","in)","punctuation removed","ampersand removed"]},{"input":"\"Ireland\'s Best B&B\'s\" - A Traveller\'s Guide","output":"irelands-best-bbs-travellers-guide","rules_applied":["quotes removed","apostrophe removed","stopwords (a)"]},{"input":"Sláinte! Traditional Irish Céilí Music","output":"slainte-traditional-irish-ceili-music","rules_applied":["fada stripped (á→a","é→e","í→i)","lowercase"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-IE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (IN) Slugification',
    s.content = 'URL slug generation rules for en-IN',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"verb":["is","are","be"],"pronoun":["it","this","that"],"conjunction":["and","or","but","as"],"preposition":["of","to","in","for","on","with","at","by","from"],"article":["the","a","an"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for effective SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Stopwords > 60% removed","message":"Original title may be too generic"}]',
    s.examples = '[{"input":"The Best Street Food in Mumbai","output":"best-street-food-mumbai","rules_applied":["stopwords: the","in"]},{"input":"How to Apply for a PAN Card","output":"how-apply-pan-card","rules_applied":["stopwords: to","for","a"]},{"input":"Diwali Shopping Guide for Families","output":"diwali-shopping-guide-families","rules_applied":["stopwords: for"]},{"input":"Top Restaurants in Bangalore","output":"top-restaurants-bangalore","rules_applied":["stopwords: in"]},{"input":"Understanding GST and Income Tax","output":"understanding-gst-income-tax","rules_applied":["stopwords: and"]},{"input":"10 Lakh Budget Cars in India 2025","output":"10-lakh-budget-cars-india-2025","rules_applied":["stopwords: in; numbers preserved"]},{"input":"The Complete Beginner\'s Guide to Starting a Successful Small Business in India and Building Your Financial Future","output":"complete-beginners-guide-starting-successful-small-business-india-building-your","rules_applied":["truncated at 80 chars; stopwords: the","to","a","in","and"]},{"input":"What\'s New? The Latest Bollywood Updates!","output":"whats-new-latest-bollywood-updates","rules_applied":["punctuation removed; stopwords: the"]},{"input":"\'Made in India\' Products to Buy","output":"made-india-products-buy","rules_applied":["quotes removed; stopwords: in","to"]},{"input":"IRCTC & RedBus: Online Booking Guide","output":"irctc-redbus-online-booking-guide","rules_applied":["ampersand and colon removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-JM'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (JM) Slugification',
    s.content = 'URL slug generation rules for en-JM',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"verb":["is","are","was","were","be","been","being"],"article":["a","an","the"],"pronoun":["it","that","this"],"conjunction":["and","or","but"],"preposition":["of","in","on","at","to","for","with","by","from","as"]}',
    s.stopwords_count = 26,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"}]',
    s.examples = '[{"input":"Best Jerk Chicken Recipes from Jamaica","output":"best-jerk-chicken-recipes-jamaica","rules_applied":["Stopwords removed: from"]},{"input":"Reggae Music Festival Guide and Schedule","output":"reggae-music-festival-guide-schedule","rules_applied":["Stopwords removed: and"]},{"input":"Top 10 Beaches in Montego Bay","output":"top-10-beaches-montego-bay","rules_applied":["Stopwords removed: in"]},{"input":"Blue Mountains Coffee Plantation Tour","output":"blue-mountains-coffee-plantation-tour","rules_applied":["No stopwords removed"]},{"input":"Authentic Jamaican Patty with Coco Bread","output":"authentic-jamaican-patty-coco-bread","rules_applied":["Stopwords removed: with"]},{"input":"7 Reasons to Visit Negril in 2025","output":"7-reasons-visit-negril-2025","rules_applied":["Stopwords removed: to","in; numbers preserved"]},{"input":"The Complete Guide to Dancehall Culture and Street Dance Traditions in Kingston Jamaica","output":"complete-guide-dancehall-culture-street-dance-traditions-kingston-jamaica","rules_applied":["Stopwords removed: the","to","and","in; truncated at 80 chars"]},{"input":"Rum Bars & Nightlife Tours!","output":"rum-bars-nightlife-tours","rules_applied":["Ampersand and exclamation removed"]},{"input":"What\'s the Best Season for a Jamaica Holiday?","output":"best-season-jamaica-holiday","rules_applied":["Stopwords removed: what","s","the","for","a; apostrophe","question mark removed"]},{"input":"Dunn\'s River Falls - Adventure & Exploration Guide","output":"dunns-river-falls-adventure-exploration-guide","rules_applied":["Apostrophe removed; ampersand removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-JM.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-KE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (KE) Slugification',
    s.content = 'URL slug generation rules for en-KE',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"adverb":["pia","sana","tu"],"preposition":["in","on","at","to","for","with","by","from","of","as","ya","wa","kwa","la","za"],"conjunction":["and","or","but","na","au"],"verb":["is","are","was","were","be","been","being","ni","iko","sema"],"article":["the","a","an"],"pronoun":["it","that","this","hii","hiyo"]}',
    s.stopwords_count = 41,
    s.regional_additions = '[{"word":"","category":"na","reason":"conjunction"},{"word":"","category":"ya","reason":"preposition"},{"word":"","category":"wa","reason":"preposition"},{"word":"","category":"kwa","reason":"preposition"},{"word":"","category":"ni","reason":"verb"},{"word":"","category":"la","reason":"preposition"},{"word":"","category":"za","reason":"preposition"},{"word":"","category":"hii","reason":"pronoun"},{"word":"","category":"hiyo","reason":"pronoun"},{"word":"","category":"au","reason":"conjunction"},{"word":"","category":"pia","reason":"adverb"},{"word":"","category":"sana","reason":"adverb"},{"word":"**Sheng Function Words (Remove)**","category":"Word","reason":"Category"},{"word":"","category":"iko","reason":"verb"},{"word":"","category":"sema","reason":"verb"},{"word":"","category":"tu","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"},{"condition":"Mixed Swahili/English stopwords","message":"Check both language lists"}]',
    s.examples = '[{"input":"How to Send Money via M-Pesa Paybill na Till Number","output":"send-money-m-pesa-paybill-till-number","rules_applied":["Stopwords removed: how","to","via","na (Swahili)"]},{"input":"Best Nyama Choma Spots na Grills in Nairobi CBD","output":"best-nyama-choma-spots-grills-nairobi-cbd","rules_applied":["Stopwords removed: na (Swahili)","in"]},{"input":"Jobs in Nairobi for Marketing Managers at Safaricom","output":"jobs-nairobi-marketing-managers-safaricom","rules_applied":["Stopwords removed: in","for","at"]},{"input":"Houses for Rent in Mombasa Nyali Beach Area","output":"houses-rent-mombasa-nyali-beach-area","rules_applied":["Stopwords removed: for","in"]},{"input":"Safari ya Maasai Mara: Complete Kenya Wildlife Guide","output":"safari-maasai-mara-complete-kenya-wildlife-guide","rules_applied":["Stopwords removed: ya (Swahili); punctuation removed"]},{"input":"10 Best Matatu Routes from Nairobi CBD to Westlands","output":"10-best-matatu-routes-nairobi-cbd-westlands","rules_applied":["Stopwords removed: from","to; numbers preserved"]},{"input":"The Complete Guide to Opening an Equity Bank Account in Nairobi and Using Equitel Mobile Banking with M-Pesa Integration for Fuliza Loans","output":"complete-guide-opening-equity-bank-account-nairobi-using-equitel-mobile-banking","rules_applied":["Stopwords removed: the","to","an","in","and","with","for; truncated at 80 chars"]},{"input":"Jamhuri Day & Madaraka Day: Celebrating Kenya\'s Independence!","output":"jamhuri-day-madaraka-day-celebrating-kenyas-independence","rules_applied":["Ampersand","colon","exclamation removed; apostrophe removed"]},{"input":"What\'s the Best Ugali Recipe? A Kenyan Kitchen Guide","output":"best-ugali-recipe-kenyan-kitchen-guide","rules_applied":["Stopwords removed: what","s","the","a; apostrophes removed"]},{"input":"Flights @ JKIA Kenya Airways - Safari Packages na Deals 2025","output":"flights-jkia-kenya-airways-safari-packages-deals-2025","rules_applied":["At symbol","hyphen in title removed; na (Swahili) removed; numbers preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-KE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-KY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (KY) Slugification',
    s.content = 'URL slug generation rules for en-KY',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"pronoun":["it"],"preposition":["of","in","to","for","on","at","by"],"verb":["is","are","was","be"],"article":["the","a","an"],"conjunction":["and","or","but"]}',
    s.stopwords_count = 18,
    s.regional_additions = '[{"word":"","category":"on","reason":"preposition"},{"word":"","category":"at","reason":"preposition"},{"word":"","category":"by","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO value"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Stopwords removed > 60%","message":"Title may be too generic"}]',
    s.examples = '[{"input":"Best Dive Sites in Grand Cayman","output":"best-dive-sites-grand-cayman","rules_applied":["Lowercase","stopwords (in) removed"]},{"input":"Top 10 Offshore Banking Services","output":"top-10-offshore-banking-services","rules_applied":["Lowercase","numbers preserved"]},{"input":"Stingray City: A Complete Visitor\'s Guide to the Cayman Islands Most Popular Attraction","output":"stingray-city-complete-visitors-guide-cayman-islands-most-popular-attraction","rules_applied":["Truncated at 80 chars","punctuation removed","stopwords removed"]},{"input":"George Town Restaurants & Nightlife","output":"george-town-restaurants-nightlife","rules_applied":["Ampersand removed","lowercase"]},{"input":"\"Seven Mile Beach\" Vacation Rentals","output":"seven-mile-beach-vacation-rentals","rules_applied":["Quotes removed","lowercase"]},{"input":"2025 Hurricane Season Preparedness","output":"2025-hurricane-season-preparedness","rules_applied":["Leading numbers preserved","lowercase"]},{"input":"Cayman Islands Real Estate Market Overview: Luxury Properties, Investment Opportunities, and Buyer\'s Guide for International Investors","output":"cayman-islands-real-estate-market-overview-luxury-properties-investment-opportu","rules_applied":["Truncated at 80 chars","punctuation removed"]},{"input":"What\'s New at the Turtle Centre?","output":"whats-new-turtle-centre","rules_applied":["Apostrophe removed","question mark removed","stopwords removed"]},{"input":"Tax-Free Living: Why Expats Choose the Cayman Islands","output":"tax-free-living-why-expats-choose-cayman-islands","rules_applied":["Colon removed","lowercase","stopwords removed"]},{"input":"CI$500,000 Luxury Condos for Sale","output":"ci500000-luxury-condos-sale","rules_applied":["Currency symbols removed","stopwords removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-KY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-MU'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (MU) Slugification',
    s.content = 'URL slug generation rules for en-MU',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"article":["the","a","an"],"pronoun":["it","this","our","your"],"verb":["is","are"],"preposition":["of","in","to","for","with","from","at"],"conjunction":["and","but","or"]}',
    s.stopwords_count = 19,
    s.regional_additions = '[{"word":"","category":"our","reason":"pronoun"},{"word":"","category":"your","reason":"pronoun"},{"word":"","category":"with","reason":"preposition"},{"word":"","category":"from","reason":"preposition"},{"word":"","category":"at","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Stopwords > 60% removed","message":"Original meaning may be lost"}]',
    s.examples = '[{"input":"Best Hotels in Port Louis","output":"best-hotels-port-louis","rules_applied":["stopwords: in; lowercase"]},{"input":"Mauritius Beach Resort & Spa","output":"mauritius-beach-resort-spa","rules_applied":["ampersand removed; lowercase"]},{"input":"Top 10 Things to Do in Mauritius","output":"top-10-things-do-mauritius","rules_applied":["stopwords: to","in; numbers kept"]},{"input":"Le Morne Brabant Hiking Guide","output":"le-morne-brabant-hiking-guide","rules_applied":["lowercase; French article kept (proper noun)"]},{"input":"Chamarel Seven Coloured Earths","output":"chamarel-seven-coloured-earths","rules_applied":["British spelling preserved in ASCII"]},{"input":"Port Louis Market: A Visitor\'s Guide","output":"port-louis-market-visitors-guide","rules_applied":["colon removed; apostrophe removed"]},{"input":"Dodo Bird: The History of Mauritius\' Most Famous Extinct Species and Its Cultural Legacy","output":"dodo-bird-history-mauritius-most-famous-extinct-species-its-cultural","rules_applied":["truncation at 80 chars; stopwords: the","of"]},{"input":"\"Sega\" Music & Dance Traditions","output":"sega-music-dance-traditions","rules_applied":["quotes removed; ampersand removed"]},{"input":"Île aux Cerfs Day Trip","output":"ile-aux-cerfs-day-trip","rules_applied":["diacritics stripped: Î→i"]},{"input":"Dholl Puri & Roti: Street Food","output":"dholl-puri-roti-street-food","rules_applied":["ampersand removed; colon removed; Mauritian cuisine terms"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-MU.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-MY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (MY) Slugification',
    s.content = 'URL slug generation rules for en-MY',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["and","or","but"],"article":["the","a","an"],"verb":["is","are","was","were","be","been","being"],"pronoun":["it","this","that"],"preposition":["in","on","at","to","for","with","by","from","of"]}',
    s.stopwords_count = 25,
    s.regional_additions = '[{"word":"","category":"lah","reason":"discourse particle"},{"word":"","category":"mah","reason":"discourse particle"},{"word":"","category":"lor","reason":"discourse particle"},{"word":"","category":"kan","reason":"discourse particle"},{"word":"","category":"bah","reason":"discourse particle"},{"word":"","category":"kot","reason":"discourse particle"},{"word":"","category":"jer","reason":"discourse particle"},{"word":"","category":"punya","reason":"possessive marker"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"}]',
    s.examples = '[{"input":"Best Nasi Lemak Spots in Kuala Lumpur","output":"best-nasi-lemak-spots-kuala-lumpur","rules_applied":["Stopwords removed: in"]},{"input":"Penang Georgetown Heritage Walking Tour Guide","output":"penang-georgetown-heritage-walking-tour-guide","rules_applied":["No stopwords removed"]},{"input":"How to Reload Touch n Go eWallet Malaysia","output":"reload-touch-n-go-ewallet-malaysia","rules_applied":["Stopwords removed: how","to; apostrophe removed"]},{"input":"Maybank2u Online Banking Registration Tutorial","output":"maybank2u-online-banking-registration-tutorial","rules_applied":["No stopwords removed"]},{"input":"Jobs in KL for Fresh Graduates 2026","output":"jobs-kl-fresh-graduates-2026","rules_applied":["Stopwords removed: in","for; numbers preserved"]},{"input":"Top 10 Mamak Restaurants in Johor Bahru for Late Night Teh Tarik","output":"top-10-mamak-restaurants-johor-bahru-late-night-teh-tarik","rules_applied":["Stopwords removed: in","for; numbers preserved"]},{"input":"The Complete Guide to Understanding Your EPF Withdrawal Options and How to Apply for KWSP i-Lestari Benefits in Malaysia","output":"complete-guide-understanding-epf-withdrawal-options-apply-kwsp-i-lestari","rules_applied":["Stopwords removed: the","to","your","and","how","for","in; truncated at 80 chars"]},{"input":"Langkawi Duty-Free Shopping: Best Deals on Chocolates & Alcohol!","output":"langkawi-duty-free-shopping-best-deals-chocolates-alcohol","rules_applied":["Colon","ampersand","exclamation removed; stopwords removed: on"]},{"input":"\"Confirm Plus Chop Lah\" - Understanding Malaysian Slang Expressions","output":"confirm-plus-chop-understanding-malaysian-slang-expressions","rules_applied":["Quotes removed; stopwords removed: lah"]},{"input":"AirAsia KLIA2 Terminal Guide: Check-In & Baggage Drop Mah!","output":"airasia-klia2-terminal-guide-check-in-baggage-drop","rules_applied":["Colon","ampersand","exclamation removed; stopwords removed: mah"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-MY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-NG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (NG) Slugification',
    s.content = 'URL slug generation rules for en-NG',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["and","or","but"],"pronoun":["it","that","this"],"auxiliary":["na"],"article":["a","an","the"],"preposition":["of","in","on","at","to","for","with","by","from","as"],"verb":["is","are","was","were","be","been","being","dey"],"interrogative":["wetin"]}',
    s.stopwords_count = 29,
    s.regional_additions = '[{"word":"","category":"na","reason":"auxiliary"},{"word":"","category":"dey","reason":"verb"},{"word":"","category":"sef","reason":"particle"},{"word":"","category":"wetin","reason":"interrogative"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"}]',
    s.examples = '[{"input":"Best Restaurants in Lagos Island","output":"best-restaurants-lagos-island","rules_applied":["Stopwords removed: in"]},{"input":"How to Start a Business in Nigeria","output":"start-business-nigeria","rules_applied":["Stopwords removed: how","to","a","in"]},{"input":"Nollywood Movies to Watch This Weekend","output":"nollywood-movies-watch-weekend","rules_applied":["Stopwords removed: to","this"]},{"input":"Guide to Investing in Nigerian Real Estate","output":"guide-investing-nigerian-real-estate","rules_applied":["Stopwords removed: to","in"]},{"input":"Top Fintech Companies in Africa","output":"top-fintech-companies-africa","rules_applied":["Stopwords removed: in"]},{"input":"10 Best Suya Spots in Abuja 2026","output":"10-best-suya-spots-abuja-2026","rules_applied":["Stopwords removed: in; numbers preserved"]},{"input":"The Complete Guide to Understanding Nigerian Tax Laws and Compliance Requirements for Small and Medium Enterprises","output":"complete-guide-understanding-nigerian-tax-laws-compliance-requirements-small","rules_applied":["Stopwords removed: the","to","and","for; truncated at 80 chars"]},{"input":"Lagos Traffic! How to Navigate the Hustle & Bustle","output":"lagos-traffic-navigate-hustle-bustle","rules_applied":["Stopwords removed: how","to","the; exclamation and ampersand removed"]},{"input":"What\'s New in Nigeria\'s Tech Ecosystem?","output":"new-nigerias-tech-ecosystem","rules_applied":["Stopwords removed: what","s","in; apostrophe and question mark removed"]},{"input":"Dangote Refinery: A Game-Changer for Nigeria\'s Economy","output":"dangote-refinery-gamechanger-nigerias-economy","rules_applied":["Stopwords removed: a","for; colon and hyphen handled"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-NG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-NZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (NZ) Slugification',
    s.content = 'URL slug generation rules for en-NZ',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"pronoun":["it","this"],"article":["the","a","an","te","nga"],"preposition":["of","to","in","for"],"verb":["is","are","was","be"],"conjunction":["and","or","but"]}',
    s.stopwords_count = 18,
    s.regional_additions = '[{"word":"","category":"kia","reason":"particle"},{"word":"","category":"te","reason":"article"},{"word":"","category":"nga","reason":"article"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed","message":"More than 60% of words were stopwords"}]',
    s.examples = '[{"input":"Best Cafés in Auckland","output":"best-cafes-auckland","rules_applied":["lowercase","strip accent (é→e)","stopword (in)"]},{"input":"Top 10 Tramping Tracks in New Zealand","output":"top-10-tramping-tracks-new-zealand","rules_applied":["lowercase","stopword (in)"]},{"input":"The Ultimate Guide to Rotorua Hot Pools","output":"ultimate-guide-rotorua-hot-pools","rules_applied":["lowercase","stopwords (the","to)"]},{"input":"Māori Culture and Traditions","output":"maori-culture-traditions","rules_applied":["lowercase","macron strip (ā→a)","stopword (and)"]},{"input":"What to Pack for a Bach Holiday","output":"what-pack-bach-holiday","rules_applied":["lowercase","stopwords (to","for","a)"]},{"input":"Kiwi Bird Conservation: 2025 Update","output":"kiwi-bird-conservation-2025-update","rules_applied":["lowercase","colon removed"]},{"input":"A Comprehensive Analysis of the New Zealand Housing Market and Its Impact on First-Home Buyers Across All Regions","output":"comprehensive-analysis-new-zealand-housing-market-its-impact-first-home-buyers","rules_applied":["truncate at 80 chars","stopwords (a","of","the","and","on","across","all)"]},{"input":"Fish & Chips: NZ\'s Favourite Takeaway!","output":"fish-chips-nzs-favourite-takeaway","rules_applied":["lowercase","ampersand removed","colon removed","exclamation removed"]},{"input":"\"She\'ll Be Right\" - The Kiwi Attitude","output":"shell-right-kiwi-attitude","rules_applied":["quotes removed","apostrophe removed","hyphen removed","stopwords (be","the)"]},{"input":"Te Reo Māori Language Week 2025","output":"reo-maori-language-week-2025","rules_applied":["lowercase","macron strip (ā→a)","stopword (te)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-NZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-PH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (PH) Slugification',
    s.content = 'URL slug generation rules for en-PH',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"pronoun":["it","that","this"],"conjunction":["and","or","but","kasi"],"preposition":["of","in","on","at","to","for","with","by","from","as"],"interrogative":["ano"],"article":["a","an","the"],"verb":["is","are","was","were","be","been","being"]}',
    s.stopwords_count = 28,
    s.regional_additions = '[{"word":"","category":"po","reason":"honorific particle"},{"word":"","category":"opo","reason":"honorific particle"},{"word":"","category":"naman","reason":"discourse particle"},{"word":"","category":"diba","reason":"discourse particle"},{"word":"","category":"ano","reason":"interrogative"},{"word":"","category":"sige","reason":"discourse particle"},{"word":"","category":"talaga","reason":"intensifier"},{"word":"","category":"lang","reason":"limiter"},{"word":"","category":"pala","reason":"discourse particle"},{"word":"","category":"nga","reason":"discourse particle"},{"word":"","category":"ba","reason":"question marker"},{"word":"","category":"kasi","reason":"conjunction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"}]',
    s.examples = '[{"input":"Best Lechon Cebu Restaurants Near Ayala Center","output":"best-lechon-cebu-restaurants-near-ayala-center","rules_applied":["No stopwords in title"]},{"input":"How to Send Money via GCash to Maya Account","output":"send-money-via-gcash-maya-account","rules_applied":["Stopwords removed: how","to","to"]},{"input":"Condo for Rent in BGC Taguig Near Uptown Mall","output":"condo-rent-bgc-taguig-near-uptown-mall","rules_applied":["Stopwords removed: for","in"]},{"input":"Top 10 Boracay Beach Resorts for Balikbayan Visitors","output":"top-10-boracay-beach-resorts-balikbayan-visitors","rules_applied":["Stopwords removed: for; numbers preserved"]},{"input":"Jollibee Chickenjoy vs Greenwich Pizza: Which is Better Naman?","output":"jollibee-chickenjoy-vs-greenwich-pizza-which-better","rules_applied":["Stopwords removed: is","naman; colon","question mark removed"]},{"input":"5 Siargao Surfing Spots Every Cloud 9 Beginner Should Visit 2026","output":"5-siargao-surfing-spots-every-cloud-9-beginner-should-visit-2026","rules_applied":["Numbers preserved; no stopwords match"]},{"input":"The Complete Guide to BDO and BPI Online Banking Setup for OFW Remittance Transfer to Philippine Peso PHP Account","output":"complete-guide-bdo-bpi-online-banking-setup-ofw-remittance-transfer-philippine","rules_applied":["Stopwords removed: the","to","and","for","to; truncated at 80 chars"]},{"input":"SM Mall of Asia: Sinulog Festival Sale & Holiday Promo!","output":"sm-mall-asia-sinulog-festival-sale-holiday-promo","rules_applied":["Stopwords removed: of; colon","ampersand","exclamation removed"]},{"input":"What\'s the Best Adobo Recipe Talaga Ba? Pinoy Style!","output":"best-adobo-recipe-pinoy-style","rules_applied":["Stopwords removed: the","talaga","ba; apostrophe","question mark","exclamation removed"]},{"input":"Globe vs Smart vs PLDT Home Fibr WiFi - Comparison Guide for Metro Manila Makati","output":"globe-vs-smart-vs-pldt-home-fibr-wifi-comparison-guide-metro-manila-makati","rules_applied":["Stopwords removed: for; hyphen cleaned"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-PH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-PK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (PK) Slugification',
    s.content = 'URL slug generation rules for en-PK',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["of","in","on","at","to","for","with","by","from"],"verb":["is","are","was","were","be","been","being","have","has","had"],"article":["the","a","an"],"pronoun":["it","this","that","these","those"],"honorific":["sahib","ji"],"conjunction":["and","or","but"]}',
    s.stopwords_count = 32,
    s.regional_additions = '[{"word":"","category":"sahib","reason":"honorific"},{"word":"","category":"ji","reason":"honorific"},{"word":"","category":"bhai","reason":"informal address"},{"word":"","category":"yaar","reason":"informal address"},{"word":"","category":"wala","reason":"suffix"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed (> 60%)","message":"May lose meaning"},{"condition":"Only numbers remain","message":"Consider adding descriptive words"}]',
    s.examples = '[{"input":"Best Restaurants in Karachi","output":"best-restaurants-karachi","rules_applied":["Stopwords removed (in)","lowercase"]},{"input":"The Complete Guide to Pakistani Cuisine","output":"complete-guide-pakistani-cuisine","rules_applied":["Article removed (the","to)","lowercase"]},{"input":"Lahore vs Islamabad: Which City is Better","output":"lahore-vs-islamabad-which-city-better","rules_applied":["Colon removed","stopword (is) removed"]},{"input":"Top 10 Tourist Spots in Pakistan","output":"top-10-tourist-spots-pakistan","rules_applied":["Number preserved","stopword (in) removed"]},{"input":"How to Apply for a NADRA Card in 2025","output":"how-apply-nadra-card-2025","rules_applied":["Stopwords removed (to","for","a","in)","number preserved"]},{"input":"Cricket World Cup: Pakistan\'s Road to Victory and the Final Match Against India at the Historic Stadium","output":"cricket-world-cup-pakistans-road-victory-final-match-against-india-historic","rules_applied":["Truncated at 80 chars","stopwords removed (the","to","and","at)"]},{"input":"Fashion & Style: Eid Collection 2025!","output":"fashion-style-eid-collection-2025","rules_applied":["Special chars removed (&",":","!)","lowercase"]},{"input":"\"Best Chai\" in Old Lahore - A Food Lover\'s Guide","output":"best-chai-old-lahore-food-lovers-guide","rules_applied":["Quotes removed","stopwords (in","a) removed","apostrophe simplified"]},{"input":"Rs. 5000 Budget Travel Tips for Students","output":"rs-5000-budget-travel-tips-students","rules_applied":["Period removed","stopword (for) removed"]},{"input":"E-commerce Growth: Online Shopping Trends & Digital Payments","output":"e-commerce-growth-online-shopping-trends-digital-payments","rules_applied":["Special chars removed (&",":)","hyphen in e-commerce preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-PK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-SA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (SA) Slugification',
    s.content = 'URL slug generation rules for en-SA',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["in","on","at","to","for","of","with","by"],"conjunction":["and","or","but"],"verb":["is","are"],"article":["the","a","an"]}',
    s.stopwords_count = 16,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful URL"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"> 60% words removed","message":"Too many stopwords; title may need revision"}]',
    s.examples = '[{"input":"Best Hotels in Riyadh for Business Travellers","output":"best-hotels-riyadh-business-travellers","rules_applied":["lowercase","stopwords (in","for)","hyphenate"]},{"input":"Top 10 Shopping Malls in Jeddah","output":"top-10-shopping-malls-jeddah","rules_applied":["lowercase","stopwords (in)","numbers kept"]},{"input":"A Guide to the Saudi Vision 2030 Programme","output":"guide-saudi-vision-2030-programme","rules_applied":["lowercase","stopwords (a","to","the)","hyphenate"]},{"input":"Dining and Entertainment at Boulevard City","output":"dining-entertainment-boulevard-city","rules_applied":["lowercase","stopwords (and","at)","hyphenate"]},{"input":"The Complete Guide to Umrah for Visitors","output":"complete-guide-umrah-visitors","rules_applied":["lowercase","stopwords (the","to","for)","hyphenate"]},{"input":"50 Things to Do in Al Khobar This Weekend","output":"50-things-do-al-khobar-weekend","rules_applied":["lowercase","stopwords (to","in","this)","numbers kept"]},{"input":"Exploring the Historical District of Diriyah and Understanding Its Cultural Significance for Tourism","output":"exploring-historical-district-diriyah-understanding-its-cultural-significance","rules_applied":["truncated at 80 chars","stopwords removed"]},{"input":"What\'s New? Check Out the Latest Restaurants & Cafes!","output":"whats-new-check-out-latest-restaurants-cafes","rules_applied":["lowercase","punctuation removed","stopwords (the)"]},{"input":"\"Saudi Arabia\'s Best-Kept Secrets\" - A Traveller\'s Perspective","output":"saudi-arabias-best-kept-secrets-travellers-perspective","rules_applied":["quotes removed","stopwords (a)","apostrophe in possessive retained"]},{"input":"SAR 500 Off: Exclusive Deals for Expats & Residents","output":"sar-500-off-exclusive-deals-expats-residents","rules_applied":["lowercase","ampersand removed","stopwords (for)","colon removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-SA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-SG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (SG) Slugification',
    s.content = 'URL slug generation rules for en-SG',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"classifier":["one"],"preposition":["of","in","on","at","to","for","with","by","from","as"],"article":["a","an","the"],"pronoun":["it","that","this"],"conjunction":["and","or","but"],"verb":["is","are","was","were","be","been","being"]}',
    s.stopwords_count = 27,
    s.regional_additions = '[{"word":"","category":"lah","reason":"discourse particle"},{"word":"","category":"leh","reason":"discourse particle"},{"word":"","category":"lor","reason":"discourse particle"},{"word":"","category":"meh","reason":"discourse particle"},{"word":"","category":"ah","reason":"discourse particle"},{"word":"","category":"hor","reason":"discourse particle"},{"word":"","category":"sia","reason":"discourse particle"},{"word":"","category":"one","reason":"classifier"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"}]',
    s.examples = '[{"input":"Best Hawker Centres in Singapore for Local Food","output":"best-hawker-centres-singapore-local-food","rules_applied":["Stopwords removed: in","for"]},{"input":"Guide to Buying HDB Flat in Tampines","output":"guide-buying-hdb-flat-tampines","rules_applied":["Stopwords removed: to","in"]},{"input":"MRT Network Expansion: Thomson-East Coast Line","output":"mrt-network-expansion-thomson-east-coast-line","rules_applied":["Colon removed"]},{"input":"How to Apply for CPF Housing Grant","output":"apply-cpf-housing-grant","rules_applied":["Stopwords removed: how","to","for"]},{"input":"Top 10 Cafés in Tiong Bahru for Brunch","output":"top-10-cafes-tiong-bahru-brunch","rules_applied":["Stopwords removed: in","for; accent stripped from cafés"]},{"input":"5 Things to Know About the New COE System in 2025","output":"5-things-know-new-coe-system-2025","rules_applied":["Stopwords removed: to","about","the","in; numbers preserved"]},{"input":"The Complete Guide to Understanding Your Singapore Medisave Contributions and How They Affect Your Retirement Planning","output":"complete-guide-understanding-singapore-medisave-contributions-affect-retirement","rules_applied":["Stopwords removed: the","to","your","and","how","they","your; truncated at 80 chars"]},{"input":"Gardens by the Bay: Night Light Show & Tickets!","output":"gardens-bay-night-light-show-tickets","rules_applied":["Stopwords removed: by","the; colon","ampersand","exclamation removed"]},{"input":"Shiok Ah! What\'s the Best Char Kway Teow in Singapore?","output":"shiok-best-char-kway-teow-singapore","rules_applied":["Stopwords removed: ah","the","in; apostrophe","exclamation","question mark removed"]},{"input":"NTUC FairPrice vs Sheng Siong - Which Supermarket Is Better Lah?","output":"ntuc-fairprice-vs-sheng-siong-which-supermarket-better","rules_applied":["Stopwords removed: is","lah; hyphen and question mark removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-SG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-TT'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (TT) Slugification',
    s.content = 'URL slug generation rules for en-TT',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"verb":["is","are","was","be"],"pronoun":["it","this","that","yuh","dey","dem"],"article":["the","a","an"],"conjunction":["and","or","but"],"preposition":["in","on","at","to","for","of","with","by","wid"],"interjection":["eh","nah"]}',
    s.stopwords_count = 27,
    s.regional_additions = '[{"word":"","category":"yuh","reason":"pronoun"},{"word":"","category":"dey","reason":"pronoun"},{"word":"","category":"dem","reason":"pronoun"},{"word":"","category":"eh","reason":"interjection"},{"word":"","category":"nah","reason":"interjection"},{"word":"","category":"wid","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for effective SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Stopwords removed > 60%","message":"Title may be too generic"}]',
    s.examples = '[{"input":"Best Beaches in Trinidad and Tobago","output":"best-beaches-trinidad-tobago","rules_applied":["Stopwords: in","and"]},{"input":"Carnival 2025 Guide for Visitors","output":"carnival-2025-guide-visitors","rules_applied":["Stopwords: for"]},{"input":"The History of Port of Spain","output":"history-port-spain","rules_applied":["Stopwords: the","of","of"]},{"input":"Top 10 Doubles Vendors in Port of Spain","output":"top-10-doubles-vendors-port-spain","rules_applied":["Stopwords: in","of"]},{"input":"Where to Go Liming in Trinidad","output":"where-go-liming-trinidad","rules_applied":["Stopwords: to","in"]},{"input":"5 Best Roti Shops for 2025","output":"5-best-roti-shops-2025","rules_applied":["Stopwords: for"]},{"input":"The Complete Guide to Exploring Tobago\'s Best Beaches and Natural Wonders for First-Time Visitors","output":"complete-guide-exploring-tobagos-best-beaches-natural-wonders-first-time-visitors","rules_applied":["Truncated to 80 chars","stopwords removed"]},{"input":"What\'s Happening at Queen\'s Park Savannah?","output":"whats-happening-queens-park-savannah","rules_applied":["Apostrophes and question mark removed"]},{"input":"\"Ole Talk\" About Trini Street Food Culture","output":"ole-talk-about-trini-street-food-culture","rules_applied":["Quotes removed"]},{"input":"Maracas Bay & Bake-and-Shark: A Local\'s Guide","output":"maracas-bay-bake-shark-locals-guide","rules_applied":["Ampersand and colon removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-TT.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-TZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (TZ) Slugification',
    s.content = 'URL slug generation rules for en-TZ',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["and","or","but","na","kama","au"],"preposition":["of","in","on","at","to","for","with","by","from","as","ya","wa","kwa","la","za","katika"],"pronoun":["it","that","this","hii","hiyo"],"article":["a","an","the"],"verb":["is","are","was","were","be","been","being","ni"]}',
    s.stopwords_count = 38,
    s.regional_additions = '[{"word":"","category":"na","reason":"conjunction"},{"word":"","category":"ya","reason":"preposition"},{"word":"","category":"wa","reason":"preposition"},{"word":"","category":"kwa","reason":"preposition"},{"word":"","category":"ni","reason":"verb"},{"word":"","category":"la","reason":"preposition"},{"word":"","category":"za","reason":"preposition"},{"word":"","category":"katika","reason":"preposition"},{"word":"","category":"hii","reason":"pronoun"},{"word":"","category":"hiyo","reason":"pronoun"},{"word":"","category":"kama","reason":"conjunction"},{"word":"","category":"au","reason":"conjunction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"},{"condition":"Mixed Swahili/English stopwords","message":"Check both language lists"}]',
    s.examples = '[{"input":"Best Hotels in Dar es Salaam for Business Travellers","output":"best-hotels-dar-es-salaam-business-travellers","rules_applied":["Stopwords removed: in","for"]},{"input":"How to Get Vodacom na Airtel SIM Cards in Tanzania","output":"get-vodacom-airtel-sim-cards-tanzania","rules_applied":["Stopwords removed: how","to","na (Swahili)","in"]},{"input":"Zanzibar Stone Town Heritage Walk and Spice Tour Guide","output":"zanzibar-stone-town-heritage-walk-spice-tour-guide","rules_applied":["Stopwords removed: and"]},{"input":"Climbing Mount Kilimanjaro: 7-Day Machame Route Guide","output":"climbing-mount-kilimanjaro-7-day-machame-route-guide","rules_applied":["Punctuation removed; numbers preserved"]},{"input":"Serengeti ya Tanzania: Best Safari Parks for Wildlife","output":"serengeti-tanzania-best-safari-parks-wildlife","rules_applied":["Stopwords removed: ya (Swahili)","for"]},{"input":"10 Best Bongo Flava Artists na Wasanii wa 2025","output":"10-best-bongo-flava-artists-wasanii-2025","rules_applied":["Stopwords removed: na","wa (Swahili); numbers preserved"]},{"input":"The Complete Guide to Opening a CRDB Bank Account in Dodoma and Transferring Money with NMB TZ Pesa Mobile Banking","output":"complete-guide-opening-crdb-bank-account-dodoma-transferring-money-nmb-tz-pesa","rules_applied":["Stopwords removed: the","to","a","in","and","with; truncated at 80 chars"]},{"input":"Dala-Dala Routes & Bajaji Fares: Getting Around Arusha!","output":"dala-dala-routes-bajaji-fares-getting-around-arusha","rules_applied":["Ampersand","colon","exclamation removed; hyphen preserved in dala-dala"]},{"input":"What\'s the Best Chipsi Mayai? A Tanzanian Street Food Guide","output":"best-chipsi-mayai-tanzanian-street-food-guide","rules_applied":["Stopwords removed: what","s","the","a; apostrophes removed"]},{"input":"Ngorongoro Crater ni Conservation Area - Visiting Tips @ Karatu","output":"ngorongoro-crater-conservation-area-visiting-tips-karatu","rules_applied":["Stopwords removed: ni (Swahili); at symbol","hyphen in title removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-TZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-UG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (UG) Slugification',
    s.content = 'URL slug generation rules for en-UG',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"article":["the","a","an"],"preposition":["in","on","at","to","for","with","by","from","of","as","mu","ku","okwa"],"conjunction":["and","or","but","ne","oba","nga","era"],"pronoun":["it","that","this","kino","ekyo"],"adverb":["wano"],"verb":["is","are","was","were","be","been","being","eri"]}',
    s.stopwords_count = 37,
    s.regional_additions = '[{"word":"","category":"ne","reason":"conjunction"},{"word":"","category":"oba","reason":"conjunction"},{"word":"","category":"mu","reason":"preposition"},{"word":"","category":"ku","reason":"preposition"},{"word":"","category":"nga","reason":"conjunction"},{"word":"","category":"era","reason":"conjunction"},{"word":"","category":"okwa","reason":"preposition"},{"word":"","category":"eri","reason":"verb"},{"word":"","category":"kino","reason":"pronoun"},{"word":"","category":"ekyo","reason":"pronoun"},{"word":"","category":"buli","reason":"determiner"},{"word":"","category":"wano","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"},{"condition":"Mixed Luganda/English stopwords","message":"Check both language lists"}]',
    s.examples = '[{"input":"Best Rolex Stands in Kampala for Street Food Adventures","output":"best-rolex-stands-kampala-street-food-adventures","rules_applied":["Stopwords removed: in","for"]},{"input":"How to Register for MTN MoMo Mobile Money in Uganda","output":"register-mtn-momo-mobile-money-uganda","rules_applied":["Stopwords removed: how","to","for","in"]},{"input":"Gorilla Trekking Permits at Bwindi Impenetrable Forest","output":"gorilla-trekking-permits-bwindi-impenetrable-forest","rules_applied":["Stopwords removed: at"]},{"input":"SafeBoda ne Boda-Boda: Rider Safety Tips for Kampala","output":"safeboda-boda-boda-rider-safety-tips-kampala","rules_applied":["Stopwords removed: ne (Luganda)","for; hyphen preserved in boda-boda"]},{"input":"Matooke and Luwombo Recipes for Traditional Ugandan Cooking","output":"matooke-luwombo-recipes-traditional-ugandan-cooking","rules_applied":["Stopwords removed: and","for"]},{"input":"10 Houses for Rent in Entebbe Near the International Airport","output":"10-houses-rent-entebbe-near-international-airport","rules_applied":["Stopwords removed: for","in","the; numbers preserved"]},{"input":"The Complete Guide to Opening a dfcu Bank Account in Kampala and Applying for Stanbic Uganda Business Loans","output":"complete-guide-opening-dfcu-bank-account-kampala-applying-stanbic-uganda","rules_applied":["Stopwords removed: the","to","a","in","and","for; truncated at 80 chars"]},{"input":"Murchison Falls & Source of the Nile: Uganda Safari Adventures!","output":"murchison-falls-source-nile-uganda-safari-adventures","rules_applied":["Stopwords removed: of","the; ampersand","colon","exclamation removed"]},{"input":"What\'s the Best Waragi Brand? A Ugandan Spirits Buyer\'s Guide","output":"best-waragi-brand-ugandan-spirits-buyers-guide","rules_applied":["Stopwords removed: what","s","the","a; apostrophes removed"]},{"input":"Jobs #2025 - Marketing Manager @ MTN Uganda Tower Kampala","output":"jobs-2025-marketing-manager-mtn-uganda-tower-kampala","rules_applied":["Hash","at symbol removed; numbers preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-UG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-US'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'English (United States) Slugification',
    s.content = 'URL slug generation rules for en-US',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"verb":["is","are","was","were","be","been","being"],"preposition":["of","in","on","at","to","for","with","by","from","as"],"conjunction":["and","or","but"],"pronoun":["it","that","this"],"article":["a","an","the"]}',
    s.stopwords_count = 26,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords","message":">60% of words removed"}]',
    s.examples = '[{"input":"Top 10 Marketing Strategies for Small Business","output":"top-10-marketing-strategies-small-business","rules_applied":["Stopwords removed: for"]},{"input":"How to Build a Successful E-commerce Website","output":"build-successful-ecommerce-website","rules_applied":["Stopwords removed: how","to","a; hyphen removed from e-commerce"]},{"input":"Best Practices for Remote Team Management","output":"best-practices-remote-team-management","rules_applied":["Stopwords removed: for"]},{"input":"Customer Service Excellence: A Complete Guide","output":"customer-service-excellence-complete-guide","rules_applied":["Stopwords removed: a; colon removed"]},{"input":"Digital Transformation in Healthcare Industry","output":"digital-transformation-healthcare-industry","rules_applied":["Stopwords removed: in"]},{"input":"5 Ways to Improve Your SEO in 2025","output":"5-ways-improve-seo-2025","rules_applied":["Stopwords removed: to","your","in; numbers preserved"]},{"input":"The Ultimate Guide to Content Marketing Strategy and Implementation for Businesses","output":"ultimate-guide-content-marketing-strategy-implementation","rules_applied":["Stopwords removed: the","to","and","for","businesses; truncated at 80 chars"]},{"input":"Free Tools & Resources for Developers!","output":"free-tools-resources-developers","rules_applied":["Stopwords removed: for; ampersand and exclamation removed"]},{"input":"What\'s New in JavaScript? Latest Features & Updates","output":"new-javascript-latest-features-updates","rules_applied":["Stopwords removed: what","s","in; apostrophe","question mark","ampersand removed"]},{"input":"Product #247 - User Authentication & Authorization","output":"product-247-user-authentication-authorization","rules_applied":["Hash","hyphen","ampersand removed; stopwords removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-US.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-VN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (VN) Slugification',
    s.content = 'URL slug generation rules for en-VN',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"article":["the","a","an"],"verb":["is","are","was","were","be"],"conjunction":["and","or","but"],"abbreviation":["vn"],"preposition":["of","to","in","for","with","at","by","from","on"]}',
    s.stopwords_count = 21,
    s.regional_additions = '[{"word":"","category":"vietnam","reason":"geographic"},{"word":"","category":"vietnamese","reason":"adjective"},{"word":"","category":"vn","reason":"abbreviation"},{"word":"","category":"saigon","reason":"geographic"},{"word":"","category":"hanoi","reason":"geographic"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO value"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Stopwords > 60%","message":"Too many common words removed"}]',
    s.examples = '[{"input":"Best Pho Restaurants in Ho Chi Minh City","output":"best-pho-restaurants-ho-chi-minh-city","rules_applied":["lowercase","stopwords (in)","spaces to hyphens"]},{"input":"Top 10 Coffee Shops for Digital Nomads","output":"top-10-coffee-shops-digital-nomads","rules_applied":["lowercase","stopwords (for)","numbers preserved"]},{"input":"Understanding Vietnamese Business Culture","output":"understanding-business-culture","rules_applied":["lowercase","stopwords (vietnamese)","spaces to hyphens"]},{"input":"Guide to Doing Business in Vietnam","output":"guide-doing-business","rules_applied":["lowercase","stopwords (to","in","vietnam)","spaces to hyphens"]},{"input":"Da Nang Beach Resort & Spa Review","output":"da-nang-beach-resort-spa-review","rules_applied":["lowercase","ampersand removed","spaces to hyphens"]},{"input":"5 Reasons to Visit Halong Bay in 2025","output":"5-reasons-visit-halong-bay-2025","rules_applied":["lowercase","stopwords (to","in)","numbers preserved"]},{"input":"The Complete Expatriate Relocation Guide to Vietnam: Everything You Need to Know About Moving and Working in Southeast Asia","output":"complete-expatriate-relocation-guide-everything-you-need-know-about-moving-wor","rules_applied":["truncation at 80 chars","stopwords (the","to","vietnam","and","in)"]},{"input":"What\'s New: Saigon Restaurant Scene!","output":"whats-new-restaurant-scene","rules_applied":["lowercase","punctuation removed","stopwords (saigon)"]},{"input":"\"Hidden Gems\" of Hue\'s Imperial City","output":"hidden-gems-hues-imperial-city","rules_applied":["quotes removed","apostrophe handled","lowercase"]},{"input":"Café Culture & Street Food Experiences","output":"cafe-culture-street-food-experiences","rules_applied":["diacritic stripped (é→e)","ampersand removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-VN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-ZA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (ZA) Slugification',
    s.content = 'URL slug generation rules for en-ZA',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"adverb":["mos"],"article":["the","a","an"],"conjunction":["and","or","but","as"],"preposition":["of","in","on","at","to","for","with","by","from","into"],"verb":["is","are","was","were","be","been","being","have","has","had","do","does","did","will","would","could","should"],"pronoun":["it","its","this","that","these","those"],"interjection":["ja","ne","hey"]}',
    s.stopwords_count = 44,
    s.regional_additions = '[{"word":"","category":"ja","reason":"interjection"},{"word":"","category":"ne","reason":"interjection"},{"word":"","category":"hey","reason":"interjection"},{"word":"","category":"mos","reason":"adverb"},{"word":"","category":"lekker","reason":"adjective"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful URL"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Over 60% stopwords removed","message":"Title may need rewording"}]',
    s.examples = '[{"input":"The Best Braai Spots in Cape Town","output":"best-braai-spots-cape-town","rules_applied":["lowercase","stopwords (the","in)","spaces to hyphens"]},{"input":"How to Buy a Bakkie in South Africa","output":"how-buy-bakkie-south-africa","rules_applied":["stopwords (to","a","in)","lowercase"]},{"input":"Top 10 Safari Lodges in Kruger Park","output":"top-10-safari-lodges-kruger-park","rules_applied":["stopwords (in)","numbers preserved"]},{"input":"Load Shedding Schedule & Tips for 2025","output":"load-shedding-schedule-tips-2025","rules_applied":["ampersand removed","stopwords (for)","numbers preserved"]},{"input":"Understanding the South African Rand (ZAR)","output":"understanding-south-african-rand-zar","rules_applied":["stopwords (the)","parentheses removed","lowercase"]},{"input":"25 Things to Do in Johannesburg This Weekend","output":"25-things-do-johannesburg-weekend","rules_applied":["stopwords (to","in","this)","numbers preserved"]},{"input":"The Ultimate Guide to Biltong and Droewors: Traditional South African Dried Meats for Beginners","output":"ultimate-guide-biltong-droewors-traditional-south-african-dried-meats","rules_applied":["truncated to 80 chars","stopwords (the","to","and","for)","colon removed"]},{"input":"What\'s the Best Robot-Free Route in Durban?","output":"whats-best-robot-free-route-durban","rules_applied":["apostrophe removed","stopwords (the","in)","question mark removed"]},{"input":"\"Eish!\" - A Guide to South African Slang & Expressions","output":"eish-guide-south-african-slang-expressions","rules_applied":["quotes removed","stopwords (a","to)","ampersand removed","exclamation removed"]},{"input":"Afrikaans-English Code-Switching in the Workplace","output":"afrikaans-english-code-switching-workplace","rules_applied":["stopwords (in","the)","hyphens in compound preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-ZA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-ZM'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (ZM) Slugification',
    s.content = 'URL slug generation rules for en-ZM',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"verb":["is","are","was","be"],"adverb":["also","just","very","even"],"article":["the","a","an"],"preposition":["of","to","in","for","on","with","at","by","from"],"conjunction":["and","or","but","as"]}',
    s.stopwords_count = 24,
    s.regional_additions = '[{"word":"","category":"also","reason":"adverb"},{"word":"","category":"just","reason":"adverb"},{"word":"","category":"very","reason":"adverb"},{"word":"","category":"even","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Too many stopwords removed","message":"More than 60% of words filtered"}]',
    s.examples = '[{"input":"The Best Restaurants in Lusaka","output":"best-restaurants-lusaka","rules_applied":["Stopwords removed (the","in)"]},{"input":"Nshima Recipes for the Family","output":"nshima-recipes-family","rules_applied":["Stopwords removed (for","the)"]},{"input":"Victoria Falls Travel Guide 2026","output":"victoria-falls-travel-guide-2026","rules_applied":["Number preserved"]},{"input":"Copper Mining and the Zambian Economy","output":"copper-mining-zambian-economy","rules_applied":["Stopwords removed (and","the)"]},{"input":"Top 10 Things to Do in the Copperbelt","output":"top-10-things-do-copperbelt","rules_applied":["Stopwords removed (to","in","the)"]},{"input":"Kafue National Park: Wildlife & Safari Tours","output":"kafue-national-park-wildlife-safari-tours","rules_applied":["Colon and ampersand removed"]},{"input":"How the South Luangwa National Park Became One of Africa\'s Premier Safari Destinations for Wildlife Enthusiasts","output":"how-south-luangwa-national-park-became-one-africas-premier-safari-destinatio","rules_applied":["Truncated at 80 chars","stopwords removed"]},{"input":"\"Insaka\" Community Meeting Spaces in Zambian Villages!","output":"insaka-community-meeting-spaces-zambian-villages","rules_applied":["Quotes and exclamation removed"]},{"input":"What\'s New at Mosi-oa-Tunya?","output":"whats-new-mosi-oa-tunya","rules_applied":["Apostrophe removed","question mark removed"]},{"input":"Bemba & Nyanja: The Languages of Zambia","output":"bemba-nyanja-languages-zambia","rules_applied":["Ampersand","colon removed; stopwords removed (the","of)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-ZM.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'en-ZW'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EN (ZW) Slugification',
    s.content = 'URL slug generation rules for en-ZW',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"article":["the","a","an"],"verb":["is","are","was","were","be","been","being","have","has","had","do","does","did"],"adverb":["not","too","very","just","only","also","how","when","where","why"],"pronoun":["it","this","that","these","those","what","which","who","your","our","their","its"],"conjunction":["and","or","but","as","if","so","than"],"preposition":["of","in","to","for","on","with","at","by","from"]}',
    s.stopwords_count = 54,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Stopwords removed > 60%","message":"May lose meaning, review original title"}]',
    s.examples = '[{"input":"Best Kombis in Harare for Daily Commute","output":"best-kombis-harare-daily-commute","rules_applied":["lowercase","stopwords (in","for)","hyphenate"]},{"input":"Guide to Victoria Falls National Park","output":"guide-victoria-falls-national-park","rules_applied":["lowercase","stopwords (to)","hyphenate"]},{"input":"How to Braai the Perfect Steak","output":"braai-perfect-steak","rules_applied":["lowercase","stopwords (how","to","the)","hyphenate"]},{"input":"Tuckshop Business Ideas for 2025","output":"tuckshop-business-ideas-2025","rules_applied":["lowercase","stopwords (for)","numbers kept"]},{"input":"Exploring the Eastern Highlands","output":"exploring-eastern-highlands","rules_applied":["lowercase","stopwords (the)","hyphenate"]},{"input":"Top 10 Things to Do in Bulawayo","output":"top-10-things-bulawayo","rules_applied":["lowercase","stopwords (to","do","in)","numbers kept"]},{"input":"Understanding Zimbabwean Property Law and Real Estate Investment Opportunities for First-Time Buyers","output":"understanding-zimbabwean-property-law-real-estate-investment-opportunities-first","rules_applied":["truncate at 80 chars","stopwords (and","for)","hyphenate"]},{"input":"What\'s New at Mbare Musika Market?","output":"whats-new-mbare-musika-market","rules_applied":["lowercase","apostrophe removed","stopwords (at)","question mark removed"]},{"input":"\"The Herald\" Newspaper\'s History","output":"herald-newspapers-history","rules_applied":["lowercase","quotes removed","apostrophe removed","stopwords (the)"]},{"input":"Robots vs Roundabouts: Traffic in Harare","output":"robots-roundabouts-traffic-harare","rules_applied":["colon removed","stopwords (vs","in)","hyphenate"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/en-ZW.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-AR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (AR) Slugification',
    s.content = 'URL slug generation rules for es-AR',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["es","ser","sos","tenés","podés"],"interjection":["che","boludo","dale","bueno"],"preposition":["de","en","con","para","por"],"contraction":["del","al"],"conjunction":["que","y","o"],"adverb":["muy","más"],"article":["el","la","los","las","un","una"]}',
    s.stopwords_count = 27,
    s.regional_additions = '[{"word":"","category":"che","reason":"interjection"},{"word":"","category":"boludo","reason":"interjection"},{"word":"","category":"dale","reason":"interjection"},{"word":"","category":"bueno","reason":"interjection"},{"word":"","category":"sos","reason":"verb"},{"word":"","category":"tenés","reason":"verb"},{"word":"","category":"podés","reason":"verb"},{"word":"","category":"re","reason":"intensifier"},{"word":"","category":"muy","reason":"adverb"},{"word":"","category":"más","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO indexing"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability and social sharing"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"Accents missing","message":"Argentine Spanish content typically contains diacritics"}]',
    s.examples = '[{"input":"Trabajo Remoto en Buenos Aires 2026","output":"trabajo-remoto-buenos-aires-2026","rules_applied":["Stopwords removed (en)","number preserved","location kept"]},{"input":"Cómo Abrir Cuenta en Mercado Pago con CUIT","output":"cómo-abrir-cuenta-mercado-pago-cuit","rules_applied":["Stopwords removed (en","con)","brand/tax ID preserved"]},{"input":"Alquiler Temporario en Palermo Soho","output":"alquiler-temporario-palermo-soho","rules_applied":["Stopwords removed (en)","neighborhood names preserved"]},{"input":"Guía del Asado Argentino: Cortes y Técnicas","output":"guía-asado-argentino-cortes-técnicas","rules_applied":["Stopwords removed (del","y)","colon removed","diacritics kept"]},{"input":"Los 15 Mejores Restaurantes de Mendoza","output":"15-mejores-restaurantes-mendoza","rules_applied":["Stopwords removed (los","de)","number preserved"]},{"input":"Vuelos Baratos a Bariloche: Patagonia 2026","output":"vuelos-baratos-bariloche-patagonia-2026","rules_applied":["Stopwords removed (a)","colon removed","locations preserved"]},{"input":"Cómo Transferir Plata por CBU desde Banco Galicia a YPF App sin Comisión ni Demora","output":"cómo-transferir-plata-cbu-banco-galicia-ypf-app-comisión-demora","rules_applied":["Stopwords removed (por","desde","a","sin","ni)","truncated at 80 chars"]},{"input":"¿Dónde Probar las Mejores Empanadas Salteñas?","output":"dónde-probar-mejores-empanadas-salteñas","rules_applied":["Question marks removed","stopwords removed (las)","ñ preserved"]},{"input":"\"El Eternauta\" de Oesterheld: Análisis del Cómic","output":"eternauta-oesterheld-análisis-cómic","rules_applied":["Quotes removed","stopwords removed (el","de","del)"]},{"input":"¿Qué hacer en Ushuaia durante el fin de semana largo del 25 de mayo?","output":"hacer-ushuaia-fin-semana-largo-25-mayo","rules_applied":["Stopwords removed (qué","en","durante","el","de","del)","date preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-AR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-BO'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (BO) Slugification',
    s.content = 'URL slug generation rules for es-BO',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["es","son"],"adverb":["nomas","bien"],"preposition":["de","en","con","para","por"],"conjunction":["y","o","que"],"article":["el","la","los","las","un","una"],"pronoun":["su","sus"],"contraction":["del","al"],"interjection":["che"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[{"word":"","category":"pues","reason":"discourse marker"},{"word":"","category":"nomas","reason":"adverb"},{"word":"","category":"che","reason":"interjection"},{"word":"","category":"bien","reason":"adverb"},{"word":"","category":"ahi","reason":"locative"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"ASCII-only slug","message":"Verify source - Bolivian Spanish typically includes diacritics"}]',
    s.examples = '[{"input":"Mejores Saltenas de La Paz","output":"mejores-saltenas-paz","rules_applied":["Stopwords removed (de","la)","lowercase"]},{"input":"Carnaval de Oruro: Patrimonio UNESCO","output":"carnaval-oruro-patrimonio-unesco","rules_applied":["Stopwords removed (de)","colon removed","diacritics preserved"]},{"input":"Guia Completa del Salar de Uyuni 2025","output":"guia-completa-salar-uyuni-2025","rules_applied":["Stopwords removed (del","de)","number preserved"]},{"input":"Receta Tradicional de Silpancho Cochabambino","output":"receta-tradicional-silpancho-cochabambino","rules_applied":["Stopwords removed (de)","Bolivian dish preserved"]},{"input":"Como Preparar Pique Macho Autentico?","output":"como-preparar-pique-macho-autentico","rules_applied":["Question mark removed","diacritics preserved"]},{"input":"15 Trabajos Disponibles en Santa Cruz","output":"15-trabajos-disponibles-santa-cruz","rules_applied":["Number preserved","stopwords removed (en)"]},{"input":"Entrada del Gran Poder: Celebracion Anual de La Paz con Miles de Danzarines y Fraternidades Folkloricas que Recorren las Calles","output":"entrada-gran-poder-celebracion-anual-paz-miles-danzarines-fraternidades-folklori","rules_applied":["Stopwords removed","truncated at 80 chars"]},{"input":"Alquiler de Departamentos & Casas en Cochabamba","output":"alquiler-departamentos-casas-cochabamba","rules_applied":["Ampersand removed","stopwords removed (de","en)"]},{"input":"\"La Diablada de Oruro\" - Danza Patrimonio","output":"diablada-oruro-danza-patrimonio","rules_applied":["Quotes and dash removed","stopwords removed (la","de)"]},{"input":"Por que la Feria de Alasitas es unica en el mundo?","output":"feria-alasitas-unica-mundo","rules_applied":["Question mark removed","stopwords removed (por","que","la","es","en","el)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-BO.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-CL'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (CL) Slugification',
    s.content = 'URL slug generation rules for es-CL',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["es","son"],"article":["el","la","los","las","un","una"],"contraction":["del","al"],"conjunction":["y","o","pero","que","si"],"adverb":["muy"],"pronoun":["su","sus","mi","mis"],"preposition":["de","en","a","con","por","para"]}',
    s.stopwords_count = 26,
    s.regional_additions = '[{"word":"","category":"po","reason":"particle"},{"word":"","category":"muy","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"ASCII-only slug","message":"Missing expected Spanish diacritics - verify source text"}]',
    s.examples = '[{"input":"Mejores Empanadas de Pino en Santiago","output":"mejores-empanadas-pino-santiago","rules_applied":["Stopwords removed (de","en)","diacritics preserved"]},{"input":"Guía de Viña del Mar 2025","output":"guía-viña-mar-2025","rules_applied":["Stopwords removed (de","del)","diacritics preserved","number kept"]},{"input":"¿Cómo Preparar Caldillo de Congrio?","output":"cómo-preparar-caldillo-congrio","rules_applied":["Punctuation removed","stopwords removed (de)","diacritics preserved"]},{"input":"Recetas de la Abuela: Pastel de Choclo","output":"recetas-abuela-pastel-choclo","rules_applied":["Stopwords removed (de","la)","colon removed"]},{"input":"Fiestas Patrias: Tradiciones Chilenas y Cueca","output":"fiestas-patrias-tradiciones-chilenas-cueca","rules_applied":["Stopwords removed (y)","colon removed"]},{"input":"10 Lugares Turísticos de la Patagonia","output":"10-lugares-turísticos-patagonia","rules_applied":["Number preserved","stopwords removed (de","la)","diacritics preserved"]},{"input":"Historia del Vino Chileno: Desde los Valles del Maipo hasta las Cepas más Reconocidas Internacionalmente","output":"historia-vino-chileno-valles-maipo-cepas-reconocidas-internacionalmente","rules_applied":["Stopwords removed","truncated at 80 chars"]},{"input":"Mejores Restaurantes & Bares en Valparaíso","output":"mejores-restaurantes-bares-valparaíso","rules_applied":["Ampersand removed","stopwords removed (en)","diacritics preserved"]},{"input":"\"La Casa de los Espíritus\" - Análisis de Isabel Allende","output":"casa-espíritus-análisis-isabel-allende","rules_applied":["Quotes removed","stopwords removed (la","de","los)","diacritics preserved"]},{"input":"¿Por qué el Sánguche es tan Popular en Chile?","output":"sánguche-popular-chile","rules_applied":["Question marks removed","stopwords removed (por","qué","el","es","tan","en)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-CL.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-CO'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (CO) Slugification',
    s.content = 'URL slug generation rules for es-CO',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["el","la","los","las","un","una"],"adverb":["ahi"],"filler":["pues"],"verb":["es","son"],"preposition":["de","en","con","para","por","pa","pal","onde"],"conjunction":["y","o","que"]}',
    s.stopwords_count = 21,
    s.regional_additions = '[{"word":"","category":"pa","reason":"preposition"},{"word":"","category":"pal","reason":"preposition"},{"word":"","category":"onde","reason":"preposition"},{"word":"","category":"pues","reason":"filler"},{"word":"","category":"ahi","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Check content consistency"}]',
    s.examples = '[{"input":"El Mejor Café de Colombia","output":"mejor-café-colombia","rules_applied":["Lowercase","stopwords (el","de)","preserve ñ"]},{"input":"Cómo Viajar a Cartagena","output":"cómo-viajar-cartagena","rules_applied":["Lowercase","stopwords (a)","preserve ó"]},{"input":"Música Vallenata Tradicional","output":"música-vallenata-tradicional","rules_applied":["Lowercase","preserve ú"]},{"input":"Recetas de Ajiaco Bogotano","output":"recetas-ajiaco-bogotano","rules_applied":["Lowercase","stopwords (de)","preserve content"]},{"input":"Guía Turística de Medellín","output":"guía-turística-medellín","rules_applied":["Lowercase","stopwords (de)","preserve í"]},{"input":"Los 10 Mejores Restaurantes en Bogotá","output":"10-mejores-restaurantes-bogotá","rules_applied":["Lowercase","stopwords (los","en)","numbers preserved"]},{"input":"Festival Internacional de Teatro de Manizales y la Mejor Programación Cultural de la Región Cafetera","output":"festival-internacional-teatro-manizales-mejor-programación-cultural-región-cafetera","rules_applied":["Truncated at 80 chars","stopwords removed"]},{"input":"Arepa: La Tradición Gastronómica","output":"arepa-tradición-gastronómica","rules_applied":["Colon removed","stopwords (la)","preserve ó"]},{"input":"\"Cien Años de Soledad\" en Aracataca","output":"cien-años-soledad-aracataca","rules_applied":["Quotes removed","stopwords (de","en)","preserve ñ"]},{"input":"¿Dónde Comprar Café Orgánico?","output":"dónde-comprar-café-orgánico","rules_applied":["Inverted question marks removed","preserve ó é"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-CO.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-CR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (CR) Slugification',
    s.content = 'URL slug generation rules for es-CR',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["de","en","con","para","por"],"conjunction":["y","o","pero"],"verb":["es","está","son"],"article":["el","la","los","las","un","una"],"contraction":["del","al"],"demonstrative":["este"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"muy","reason":"intensifier"},{"word":"","category":"más","reason":"intensifier"},{"word":"","category":"como","reason":"relative"},{"word":"","category":"este","reason":"demonstrative"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Verify intentional mixing"}]',
    s.examples = '[{"input":"Guía de Playas en Costa Rica","output":"guía-playas-costa-rica","rules_applied":["stopwords: de","en"]},{"input":"Los Mejores Restaurantes de San José","output":"mejores-restaurantes-san-josé","rules_applied":["stopwords: los","de"]},{"input":"Volcán Arenal: Una Aventura Natural","output":"volcán-arenal-aventura-natural","rules_applied":["stopwords: una; punctuation removed"]},{"input":"El Arte de la Comida Tica","output":"arte-comida-tica","rules_applied":["stopwords: el","de","la"]},{"input":"Parque Nacional Manuel Antonio","output":"parque-nacional-manuel-antonio","rules_applied":["no stopwords removed"]},{"input":"10 Lugares para Visitar en 2025","output":"10-lugares-visitar-2025","rules_applied":["stopwords: para","en; numbers preserved"]},{"input":"Descubriendo la Biodiversidad del Bosque Nuboso de Monteverde y sus Especies Únicas","output":"descubriendo-biodiversidad-bosque-nuboso-monteverde-especies-únicas","rules_applied":["truncated at 80 chars; stopwords: la","del","de","y","sus"]},{"input":"¿Cómo Preparar Gallo Pinto?","output":"cómo-preparar-gallo-pinto","rules_applied":["stopwords: none; punctuation removed"]},{"input":"\"Pura Vida\": El Significado de la Expresión","output":"pura-vida-significado-expresión","rules_applied":["stopwords: el","de","la; quotes removed"]},{"input":"Ñoño el Pingüino","output":"ñoño-pingüino","rules_applied":["stopwords: el; ñ and ü preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-CR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-CU'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (CU) Slugification',
    s.content = 'URL slug generation rules for es-CU',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["y","o"],"article":["el","la","los","las","un","una"],"pronoun":["que"],"contraction":["pa","na"],"adverb":["aca","asi"],"verb":["es","son"],"preposition":["de","en","con","por","para"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"aca","reason":"adverb"},{"word":"","category":"asi","reason":"adverb"},{"word":"","category":"pa","reason":"contraction"},{"word":"","category":"na","reason":"contraction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Review for consistency"}]',
    s.examples = '[{"input":"Las mejores playas de Cuba","output":"mejores-playas-cuba","rules_applied":["Stopwords (las","de) removed","lowercase"]},{"input":"Recetas de la cocina cubana","output":"recetas-cocina-cubana","rules_applied":["Stopwords (de","la) removed","lowercase"]},{"input":"Musica tradicional en La Habana","output":"musica-tradicional-habana","rules_applied":["Stopwords (en","la) removed","accent preserved"]},{"input":"El ron cubano y su historia","output":"ron-cubano-su-historia","rules_applied":["Stopwords (el","y) removed","lowercase"]},{"input":"Guia para visitar Trinidad","output":"guia-visitar-trinidad","rules_applied":["Stopword (para) removed","accent preserved"]},{"input":"Los 10 mejores paladares en 2025","output":"10-mejores-paladares-2025","rules_applied":["Stopwords (los","en) removed","numbers preserved"]},{"input":"Un recorrido por las calles historicas del centro de La Habana Vieja y sus monumentos coloniales","output":"recorrido-calles-historicas-centro-habana-vieja-sus-monumentos-coloniales","rules_applied":["Truncated at 80 chars","stopwords removed"]},{"input":"Donde comer en Cuba? Los restaurantes mas populares!","output":"donde-comer-cuba-restaurantes-mas-populares","rules_applied":["Stopwords removed","punctuation cleaned"]},{"input":"\"La bodeguita del medio\" y su famoso mojito","output":"bodeguita-medio-su-famoso-mojito","rules_applied":["Quotes removed","stopwords (la","del","y) removed"]},{"input":"Guantanamera: cancion emblematica","output":"guantanamera-cancion-emblematica","rules_applied":["Colon removed","accents preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-CU.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-DO'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (DO) Slugification',
    s.content = 'URL slug generation rules for es-DO',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["es","son"],"interjection":["ombe"],"adverb":["tato"],"pronoun":["que"],"preposition":["de","en","con","para","por","pa","pal"],"article":["el","la","los","las","un","una"],"conjunction":["y","o"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"pa","reason":"preposition"},{"word":"","category":"pal","reason":"preposition"},{"word":"","category":"tato","reason":"adverb"},{"word":"","category":"ombe","reason":"interjection"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for usability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"}]',
    s.examples = '[{"input":"Guía de Turismo en Santo Domingo","output":"guía-turismo-santo-domingo","rules_applied":["Stopwords (de","en) removed","diacritics preserved"]},{"input":"Las Mejores Playas de Punta Cana","output":"mejores-playas-punta-cana","rules_applied":["Article (las) and preposition (de) removed"]},{"input":"Cómo Preparar Mangú Dominicano","output":"cómo-preparar-mangú-dominicano","rules_applied":["Accents preserved on ó and ú"]},{"input":"Carnaval de La Vega 2025","output":"carnaval-vega-2025","rules_applied":["Stopwords (de","la) removed","number preserved"]},{"input":"Historia del Merengue y la Bachata","output":"historia-merengue-bachata","rules_applied":["Stopwords (del","y","la) removed"]},{"input":"10 Lugares para Visitar en República Dominicana","output":"10-lugares-visitar-república-dominicana","rules_applied":["Number preserved","stopwords (para","en) removed"]},{"input":"Consejos para Viajar por Primera Vez a la Zona Colonial de Santo Domingo y Disfrutar de sus Monumentos Históricos","output":"consejos-viajar-primera-vez-zona-colonial-santo-domingo-disfrutar-monumentos","rules_applied":["Truncated at 80 chars","stopwords removed"]},{"input":"¿Qué Comer en Samaná?","output":"qué-comer-samaná","rules_applied":["Punctuation (¿?) removed","accents preserved"]},{"input":"\"El Mejor\" Mamajuana de la Isla","output":"mejor-mamajuana-isla","rules_applied":["Quotes removed","stopwords (el","de","la) removed"]},{"input":"Añorada Navidad Criolla","output":"añorada-navidad-criolla","rules_applied":["Ñ preserved in añorada"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-DO.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-EC'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (EC) Slugification',
    s.content = 'URL slug generation rules for es-EC',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["de","en","con","para","por","a"],"contraction":["del","al"],"conjunction":["que","y","o"],"pronoun":["se","su"],"adverb":["como"],"article":["el","la","los","las","un","una"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"verás","reason":"discourse marker"},{"word":"","category":"mismo","reason":"intensifier"},{"word":"","category":"pes","reason":"discourse marker"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"ASCII-only slug","message":"Missing expected Spanish diacritics - verify source text"}]',
    s.examples = '[{"input":"Empleo en Quito para Ingenieros 2026","output":"empleo-quito-ingenieros-2026","rules_applied":["Stopwords removed (en","para)","capital city preserved"]},{"input":"Arriendos de Departamentos en Guayaquil Norte","output":"arriendos-departamentos-guayaquil-norte","rules_applied":["Stopwords removed (de","en)","port city preserved"]},{"input":"Tours Islas Galápagos desde Quito","output":"tours-islas-galápagos-quito","rules_applied":["Stopwords removed (desde)","UNESCO site with accent"]},{"input":"Fiestas de Quito 6 de Diciembre Desfiles","output":"fiestas-quito-6-diciembre-desfiles","rules_applied":["Stopwords removed (de","de)","December festival date kept"]},{"input":"Receta Encebollado Guayaquileño Tradicional","output":"receta-encebollado-guayaquileño-tradicional","rules_applied":["Coastal fish soup dish preserved with demonym"]},{"input":"12 Mejores Hostales en Montañita Ecuador","output":"12-mejores-hostales-montañita-ecuador","rules_applied":["Number preserved","beach town with tilde preserved"]},{"input":"Inti Raymi Otavalo: Celebración del Solsticio de Junio con Danzas Ancestrales Indígenas y Rituales del Pueblo Kichwa","output":"inti-raymi-otavalo-celebración-solsticio-junio-danzas-ancestrales-indígenas","rules_applied":["Stopwords removed","truncated at 80 chars"]},{"input":"Banco Pichincha & Produbanco: Créditos Hipotecarios","output":"banco-pichincha-produbanco-créditos-hipotecarios","rules_applied":["Ampersand removed","local banks preserved"]},{"input":"\"El Mejor Locro de Papa\" en la Sierra Ecuatoriana","output":"mejor-locro-papa-sierra-ecuatoriana","rules_applied":["Quotes removed","stopwords removed (el","de","en","la)"]},{"input":"Donde Comprar Sombreros de Paja Toquilla en Cuenca?","output":"comprar-sombreros-paja-toquilla-cuenca","rules_applied":["Question mark removed","stopwords removed (donde","de","en)","UNESCO heritage craft"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-EC.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-ES'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Spanish (Spain) Slugification',
    s.content = 'URL slug generation rules for es-ES',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["y","o","pero","que","si","pues"],"verb":["es","son"],"adverb":["muy"],"article":["el","la","los","las","un","una"],"preposition":["de","en","a","con","por","para"],"contraction":["del","al"],"interjection":["vale"],"pronoun":["su","sus","mi","mis"]}',
    s.stopwords_count = 28,
    s.regional_additions = '[{"word":"","category":"pues","reason":"conjunction"},{"word":"","category":"vale","reason":"interjection"},{"word":"","category":"muy","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"ASCII-only slug","message":"Missing expected Spanish diacritics - verify source text"}]',
    s.examples = '[{"input":"Mejores Restaurantes en Madrid","output":"mejores-restaurantes-madrid","rules_applied":["Stopwords removed (en)","diacritics preserved"]},{"input":"Guia Completa de Tapas Espanolas","output":"guia-completa-tapas-espanolas","rules_applied":["Stopwords removed (de)","diacritics preserved"]},{"input":"Recetas de Paella Valenciana Tradicional","output":"recetas-paella-valenciana-tradicional","rules_applied":["Stopwords removed (de)","diacritics preserved"]},{"input":"Como Alquilar un Piso en Barcelona","output":"como-alquilar-piso-barcelona","rules_applied":["Stopwords removed (un","en)","diacritics preserved"]},{"input":"Fiestas de San Fermin en Pamplona","output":"fiestas-san-fermin-pamplona","rules_applied":["Stopwords removed (de","en)","diacritics preserved"]},{"input":"10 Mejores Playas de la Costa Brava 2025","output":"10-mejores-playas-costa-brava-2025","rules_applied":["Number preserved","stopwords removed (de","la)"]},{"input":"Historia del Camino de Santiago: Desde sus Origenes Medievales hasta la Peregrinacion Moderna Contemporanea","output":"historia-camino-santiago-origenes-medievales-peregrinacion-moderna","rules_applied":["Stopwords removed (del","de","desde","sus","hasta","la)","truncated at 80 chars"]},{"input":"Flamenco & Guitarra: Tradiciones Andaluzas","output":"flamenco-guitarra-tradiciones-andaluzas","rules_applied":["Ampersand and colon removed","diacritics preserved"]},{"input":"\"El Quijote\" - Analisis de la Obra de Cervantes","output":"quijote-analisis-obra-cervantes","rules_applied":["Quotes removed","stopwords removed (el","de","la)"]},{"input":"Por que el aceite de oliva espanol es el mejor del mundo","output":"aceite-oliva-espanol-mejor-mundo","rules_applied":["Question marks removed","stopwords removed (por","que","el","de","es","del)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-ES.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-GT'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (GT) Slugification',
    s.content = 'URL slug generation rules for es-GT',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["vos"],"contraction":["del","al"],"preposition":["de","en","con","para","por","a"],"conjunction":["que","y","o"],"article":["el","la","los","las","un","una"],"interjection":["puchis","va","sho"]}',
    s.stopwords_count = 21,
    s.regional_additions = '[{"word":"","category":"puchis","reason":"interjection"},{"word":"","category":"chilero","reason":"adjective"},{"word":"","category":"va","reason":"interjection"},{"word":"","category":"vos","reason":"pronoun"},{"word":"","category":"sho","reason":"interjection"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"Maya glottal stop","message":"Apostrophe will be removed from K\'iche\', Kaqchikel words"}]',
    s.examples = '[{"input":"Empleos en Ciudad de Guatemala Zona 10","output":"empleos-ciudad-guatemala-zona-10","rules_applied":["Stopwords removed (en","de)","number preserved"]},{"input":"Tours Guiados a Tikal desde Flores Peten","output":"tours-guiados-tikal-flores-peten","rules_applied":["Stopwords removed (a","desde)","place names preserved"]},{"input":"Semana Santa Antigua Guatemala 2026","output":"semana-santa-antigua-guatemala-2026","rules_applied":["Diacritics preserved","number kept"]},{"input":"Receta Tradicional de Pepian Guatemalteco","output":"receta-tradicional-pepian-guatemalteco","rules_applied":["Stopwords removed (de)","diacritics preserved"]},{"input":"Apartamentos Alquiler Zona 14 Precio Quetzales GTQ","output":"apartamentos-alquiler-zona-14-precio-quetzales-gtq","rules_applied":["Number preserved","currency abbrev kept"]},{"input":"15 Mejores Comedores para Probar Chuchitos en Xela","output":"15-mejores-comedores-probar-chuchitos-xela","rules_applied":["Number preserved","stopwords removed (para","en)"]},{"input":"Guia Completa del Mercado de Chichicastenango: Historia Artesanias y Tradiciones Mayas Ancestrales","output":"guia-completa-mercado-chichicastenango-historia-artesanias-tradiciones-mayas","rules_applied":["Stopwords removed (del","de","y)","truncated at 80 chars"]},{"input":"Tigo Guatemala & Claro: Comparacion Planes Moviles","output":"tigo-guatemala-claro-comparacion-planes-moviles","rules_applied":["Ampersand removed","brand names preserved"]},{"input":"\"La Torre\" vs \"Hiper Paiz\" - Mejores Supermercados","output":"torre-hiper-paiz-mejores-supermercados","rules_applied":["Quotes removed","stopwords removed (la","vs)"]},{"input":"Por que el Quetzal GTQ se Devaluo en 2025","output":"quetzal-gtq-devaluo-2025","rules_applied":["Question words removed (por","que)","stopwords removed (el","se","en)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-GT.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-HN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (HN) Slugification',
    s.content = 'URL slug generation rules for es-HN',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["el","la","los","las","un","una"],"verb":["es","son"],"pronoun":["vos"],"preposition":["de","en","con","para","por","a"],"contraction":["del","al"],"interjection":["va"],"conjunction":["que","y","o","pues"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[{"word":"","category":"pues","reason":"conjunction"},{"word":"","category":"va","reason":"interjection"},{"word":"","category":"maje","reason":"vocative"},{"word":"","category":"vos","reason":"pronoun"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"ASCII-only slug","message":"Missing expected Spanish diacritics - verify source text"}]',
    s.examples = '[{"input":"Empleos en Tegucigalpa para Profesionales","output":"empleos-tegucigalpa-profesionales","rules_applied":["Stopwords removed (en","para)","diacritics preserved"]},{"input":"Casas en Venta en San Pedro Sula","output":"casas-venta-san-pedro-sula","rules_applied":["Stopwords removed (en)","diacritics preserved"]},{"input":"Guía Turística de las Ruinas de Copán","output":"guía-turística-ruinas-copán","rules_applied":["Stopwords removed (de","las)","diacritics preserved"]},{"input":"Las Mejores Baleadas de La Ceiba","output":"mejores-baleadas-ceiba","rules_applied":["Stopwords removed (las","de","la)","diacritics preserved"]},{"input":"Feria Juniana 2026 en San Pedro Sula","output":"feria-juniana-2026-san-pedro-sula","rules_applied":["Number preserved","stopwords removed (en)"]},{"input":"10 Lugares para Visitar en Roatán y Utila","output":"10-lugares-visitar-roatán-utila","rules_applied":["Number preserved","stopwords removed (para","en","y)","diacritics preserved"]},{"input":"Historia de la Semana Morazánica: Celebración del Héroe Nacional Francisco Morazán y su Legado Centroamericano","output":"historia-semana-morazánica-celebración-héroe-nacional-francisco-morazán","rules_applied":["Stopwords removed","truncated at 80 chars"]},{"input":"BAC Credomatic & Banco Atlántida: Servicios Bancarios","output":"bac-credomatic-banco-atlántida-servicios-bancarios","rules_applied":["Ampersand removed","stopwords removed"]},{"input":"\"El Cipote de la Colonia\" - Comedia Hondureña","output":"cipote-colonia-comedia-hondureña","rules_applied":["Quotes removed","stopwords removed (el","de","la)"]},{"input":"¿Por qué Comayagua fue la Capital de Honduras?","output":"comayagua-capital-honduras","rules_applied":["Question marks removed","stopwords removed (por","qué","fue","la","de)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-HN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-MX'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Spanish (Mexico) Slugification',
    s.content = 'URL slug generation rules for es-MX',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"adverb":["más","muy","aquí","como"],"article":["el","la","los","las","un","una"],"preposition":["de","en","a","con","por","para"],"contraction":["del","al"],"verb":["es","son"],"conjunction":["y","o","pero","que","si","pues"],"pronoun":["su","sus"]}',
    s.stopwords_count = 28,
    s.regional_additions = '[{"word":"","category":"pues","reason":"conjunction"},{"word":"","category":"más","reason":"adverb"},{"word":"","category":"muy","reason":"adverb"},{"word":"","category":"aquí","reason":"adverb"},{"word":"","category":"como","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"ASCII-only slug","message":"Missing expected Spanish diacritics - verify source text"}]',
    s.examples = '[{"input":"Mejores Tacos al Pastor en CDMX","output":"mejores-tacos-pastor-cdmx","rules_applied":["Stopwords removed (al","en)","CDMX preserved lowercase"]},{"input":"Guía de Vuelos Baratos a Cancún 2026","output":"guía-vuelos-baratos-cancún-2026","rules_applied":["Stopwords removed (de","a)","diacritics preserved","number kept"]},{"input":"¿Cómo Preparar Pozole Rojo Estilo Jalisco?","output":"cómo-preparar-pozole-rojo-estilo-jalisco","rules_applied":["Punctuation removed","diacritics preserved"]},{"input":"Recetas de Mole Oaxaqueño: Tradición Familiar","output":"recetas-mole-oaxaqueño-tradición-familiar","rules_applied":["Stopwords removed (de)","colon removed","diacritics preserved"]},{"input":"Día de Muertos en Oaxaca: Ofrendas y Cempasúchil","output":"día-muertos-oaxaca-ofrendas-cempasúchil","rules_applied":["Stopwords removed (de","en","y)","diacritics preserved"]},{"input":"15 Departamentos en Renta Guadalajara 2026","output":"15-departamentos-renta-guadalajara-2026","rules_applied":["Number preserved","stopwords removed (en)","diacritics preserved"]},{"input":"Historia de las Pirámides de Teotihuacán: Desde la Época Prehispánica hasta el Turismo Moderno en México","output":"historia-pirámides-teotihuacán-época-prehispánica-turismo-moderno-méxico","rules_applied":["Stopwords removed","truncated at 80 chars"]},{"input":"Mejores Ofertas Buen Fin: Telcel & Liverpool en CDMX","output":"mejores-ofertas-buen-fin-telcel-liverpool-cdmx","rules_applied":["Ampersand removed","stopwords removed (en)","brands preserved"]},{"input":"\"El Laberinto de la Soledad\" - Análisis de Octavio Paz","output":"laberinto-soledad-análisis-octavio-paz","rules_applied":["Quotes removed","stopwords removed (el","de","la)","diacritics preserved"]},{"input":"¿Por qué las Quesadillas de la CDMX no llevan Queso?","output":"quesadillas-cdmx-llevan-queso","rules_applied":["Question marks removed","stopwords removed (por","qué","las","de","la","no)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-MX.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-NI'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (NI) Slugification',
    s.content = 'URL slug generation rules for es-NI',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["de","en","a","con","por","para"],"conjunction":["y","o","pero","que","si","pues"],"adverb":["entonces"],"article":["el","la","los","las","un","una"],"interjection":["ideay"],"pronoun":["su","sus","mi","mis"],"contraction":["del","al"],"verb":["es","son"]}',
    s.stopwords_count = 28,
    s.regional_additions = '[{"word":"","category":"pues","reason":"conjunction"},{"word":"","category":"entonces","reason":"adverb"},{"word":"","category":"ideay","reason":"interjection"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"ASCII-only slug","message":"Missing expected Spanish diacritics - verify source text"}]',
    s.examples = '[{"input":"Mejores Vigorones de Granada","output":"mejores-vigorones-granada","rules_applied":["Stopwords removed (de)","diacritics preserved"]},{"input":"Guía de Turismo en la Isla de Ometepe","output":"guía-turismo-isla-ometepe","rules_applied":["Stopwords removed (de","en","la)","diacritics preserved"]},{"input":"¿Cómo Preparar Nacatamal Nicaragüense?","output":"cómo-preparar-nacatamal-nicaragüense","rules_applied":["Punctuation removed","diacritics preserved"]},{"input":"Playas del Pacífico en San Juan del Sur","output":"playas-pacífico-san-juan-sur","rules_applied":["Stopwords removed (del","en)","diacritics preserved"]},{"input":"Historia de León: Ciudad Universitaria","output":"historia-león-ciudad-universitaria","rules_applied":["Stopwords removed (de)","colon removed","diacritics preserved"]},{"input":"10 Platillos Típicos de Nicaragua","output":"10-platillos-típicos-nicaragua","rules_applied":["Number preserved","stopwords removed (de)"]},{"input":"Festival Internacional de Poesía de Granada: Encuentro de Escritores Latinoamericanos y Europeos en la Ciudad Colonial","output":"festival-internacional-poesía-granada-encuentro-escritores-latinoamericanos","rules_applied":["Stopwords removed","truncated at 80 chars"]},{"input":"Mejores Fritangas & Comedores en Managua","output":"mejores-fritangas-comedores-managua","rules_applied":["Ampersand removed","stopwords removed (en)"]},{"input":"\"El Güegüense\" - Patrimonio Cultural de la Humanidad","output":"güegüense-patrimonio-cultural-humanidad","rules_applied":["Quotes removed","stopwords removed (el","de","la)"]},{"input":"¿Por qué el Córdoba es la moneda de Nicaragua?","output":"córdoba-moneda-nicaragua","rules_applied":["Question marks removed","stopwords removed (por","qué","el","es","la","de)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-NI.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-PA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (PA) Slugification',
    s.content = 'URL slug generation rules for es-PA',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["es","son","ser","estar"],"article":["el","la","los","las","un","una"],"conjunction":["y","o","e","u","que","pero","ni","como"],"preposition":["de","del","en","a","al","con","por","para","sin","sobre"]}',
    s.stopwords_count = 28,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"}]',
    s.examples = '[{"input":"Guía para Comprar en la Zona Libre de Colón","output":"guía-comprar-zona-libre-colón","rules_applied":["lowercase","stopwords (para","en","la","de)","preserve ñ and accents"]},{"input":"El Canal de Panamá: Historia y Futuro","output":"canal-panamá-historia-futuro","rules_applied":["lowercase","stopwords (el","de","y)","preserve á","colon removed"]},{"input":"Los Mejores Restaurantes en Ciudad de Panamá","output":"mejores-restaurantes-ciudad-panamá","rules_applied":["lowercase","stopwords (los","en","de)","preserve á"]},{"input":"Cómo Llegar al Casco Antiguo","output":"cómo-llegar-casco-antiguo","rules_applied":["lowercase","stopwords (al)","preserve ó"]},{"input":"Fiestas Patrias de Noviembre en Panamá","output":"fiestas-patrias-noviembre-panamá","rules_applied":["lowercase","stopwords (de","en)","preserve á"]},{"input":"10 Playas Increíbles para Visitar en 2025","output":"10-playas-increíbles-visitar-2025","rules_applied":["lowercase","stopwords (para","en)","numbers preserved","preserve í"]},{"input":"Guía Completa de Turismo y Aventura en las Islas de San Blas para Viajeros Internacionales que Buscan Experiencias Auténticas","output":"guía-completa-turismo-aventura-islas-san-blas-viajeros-internacionales-buscan-e","rules_applied":["truncation at 80 chars","stopwords removed","accents preserved"]},{"input":"¡Bienvenidos a Panamá! ¿Cómo Disfrutar tu Viaje?","output":"bienvenidos-panamá-cómo-disfrutar-viaje","rules_applied":["inverted punctuation removed","stopwords (a","tu)","preserve ó and á"]},{"input":"\"La Pollera\" y el Vestido Típico Panameño","output":"pollera-vestido-típico-panameño","rules_applied":["quotes removed","stopwords (la","y","el)","preserve í and ñ"]},{"input":"Señales de Tránsito y Conducción en Panamá","output":"señales-tránsito-conducción-panamá","rules_applied":["lowercase","stopwords (de","y","en)","preserve ñ and accents"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-PA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-PE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (PE) Slugification',
    s.content = 'URL slug generation rules for es-PE',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["y","o"],"pronoun":["que","su","se"],"article":["el","la","los","las","un","una","lo"],"abbreviation":["pe"],"preposition":["de","en","con","para","por"],"verb":["es","son"],"contraction":["del","al"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[{"word":"","category":"pe","reason":"abbreviation"},{"word":"","category":"del","reason":"contraction"},{"word":"","category":"al","reason":"contraction"},{"word":"","category":"lo","reason":"article"},{"word":"","category":"su","reason":"pronoun"},{"word":"","category":"se","reason":"pronoun"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Verify intentional use of non-Latin characters"}]',
    s.examples = '[{"input":"Guía de turismo en Lima","output":"guía-turismo-lima","rules_applied":["lowercase","stopwords (de","en) removed","diacritics preserved"]},{"input":"Cómo preparar ceviche peruano","output":"cómo-preparar-ceviche-peruano","rules_applied":["lowercase","diacritics preserved"]},{"input":"Los mejores restaurantes del Centro Histórico","output":"mejores-restaurantes-centro-histórico","rules_applied":["stopwords (los","del) removed","diacritics preserved"]},{"input":"Fiestas patrias en el Perú","output":"fiestas-patrias-perú","rules_applied":["stopwords (en","el) removed","diacritics preserved"]},{"input":"Artesanía de la sierra peruana","output":"artesanía-sierra-peruana","rules_applied":["stopwords (de","la) removed","diacritics preserved"]},{"input":"10 lugares turísticos para visitar en 2025","output":"10-lugares-turísticos-visitar-2025","rules_applied":["numbers preserved","stopwords (para","en) removed"]},{"input":"Guía completa para conocer la cultura y tradiciones de las comunidades andinas del Perú","output":"guía-completa-conocer-cultura-tradiciones-comunidades-andinas-perú","rules_applied":["truncation at 80 chars","multiple stopwords removed"]},{"input":"¿Qué hacer en Cusco?","output":"qué-hacer-cusco","rules_applied":["punctuation removed","stopwords (en) removed","diacritics preserved"]},{"input":"\"La gastronomía peruana es la mejor\"","output":"gastronomía-peruana-mejor","rules_applied":["quotes removed","stopwords (la","es) removed"]},{"input":"Machu Picchu & Líneas de Nazca","output":"machu-picchu-líneas-nazca","rules_applied":["ampersand removed","stopwords (de) removed","diacritics preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-PE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-PR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (PR) Slugification',
    s.content = 'URL slug generation rules for es-PR',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["es","son"],"article":["el","la","los","las","un","una"],"conjunction":["y","o","que"],"contraction":["del","al"],"preposition":["de","en","con","para","por","pa"]}',
    s.stopwords_count = 19,
    s.regional_additions = '[{"word":"","category":"del","reason":"contraction"},{"word":"","category":"al","reason":"contraction"},{"word":"","category":"pa","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Review for consistency"}]',
    s.examples = '[{"input":"Las Mejores Playas de Puerto Rico","output":"mejores-playas-puerto-rico","rules_applied":["stopwords: las","de"]},{"input":"Guía de Restaurantes en San Juan","output":"guía-restaurantes-san-juan","rules_applied":["stopwords: de","en; preserve: í"]},{"input":"El Morro y La Fortaleza","output":"morro-fortaleza","rules_applied":["stopwords: el","y","la"]},{"input":"Fiestas Patronales de Loíza","output":"fiestas-patronales-loíza","rules_applied":["stopwords: de; preserve: í"]},{"input":"Recetas de Mofongo con Camarones","output":"recetas-mofongo-camarones","rules_applied":["stopwords: de","con"]},{"input":"10 Lugares para Visitar en Ponce 2026","output":"10-lugares-visitar-ponce-2026","rules_applied":["stopwords: para","en; numbers preserved"]},{"input":"Consejos para Viajar a Puerto Rico durante la Temporada de Huracanes y Prepararse para el Clima Tropical","output":"consejos-viajar-puerto-rico-durante-temporada-huracanes-prepararse-clima-tropical","rules_applied":["truncation at 80 chars; stopwords: para","a","la","de","y","el"]},{"input":"¿Dónde Comer Lechón Asado?","output":"dónde-comer-lechón-asado","rules_applied":["punctuation: ¿? removed; preserve: ó"]},{"input":"Música \"Típica\" del Jíbaro Puertorriqueño","output":"música-típica-jíbaro-puertorriqueño","rules_applied":["quotes removed; stopwords: del; preserve: í","ñ"]},{"input":"Chinchorreo: Ruta por la Isla","output":"chinchorreo-ruta-isla","rules_applied":["colon removed; stopwords: por","la"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-PR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-PY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (PY) Slugification',
    s.content = 'URL slug generation rules for es-PY',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["el","la","los","las","un","una"],"interjection":["che"],"pronoun":["nde"],"verb":["es","son"],"adverb":["gua\'u"],"conjunction":["y","o","que"],"preposition":["de","en","con","por","para"],"demonstrative":["ko"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"che","reason":"interjection"},{"word":"","category":"nde","reason":"pronoun"},{"word":"","category":"pio","reason":"particle"},{"word":"","category":"katu","reason":"particle"},{"word":"gua\'u","category":"adverb","reason":"Guarani-origin word meaning \"a little\" in casual speech"},{"word":"ko","category":"demonstrative","reason":"Guarani demonstrative often mixed into Spanish"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Review for consistency"}]',
    s.examples = '[{"input":"Guía de Turismo en Asunción","output":"guía-turismo-asunción","rules_applied":["lowercase","stopwords (de","en)","preserve ñ and ú"]},{"input":"El Chaco Paraguayo: Naturaleza y Aventura","output":"chaco-paraguayo-naturaleza-aventura","rules_applied":["stopwords (el","y)","colon removed","preserve accents"]},{"input":"Tereré: La Bebida Nacional del Paraguay","output":"tereré-bebida-nacional-paraguay","rules_applied":["stopwords (la","del)","colon removed","preserve é"]},{"input":"Cocina Paraguaya con Mandioca","output":"cocina-paraguaya-mandioca","rules_applied":["stopwords (con)","preserve accents"]},{"input":"Artesanía Ñandutí de Itauguá","output":"artesanía-ñandutí-itauguá","rules_applied":["preserve ñ","í","ú","no stopwords"]},{"input":"10 Lugares Turísticos en Ciudad del Este","output":"10-lugares-turísticos-ciudad-este","rules_applied":["stopwords (en","del)","numbers preserved"]},{"input":"Celebraciones Tradicionales del Bicentenario de la Independencia Paraguaya en el Año 2011","output":"celebraciones-tradicionales-bicentenario-independencia-paraguaya-año-2011","rules_applied":["truncation at 80 chars","stopwords removed"]},{"input":"¿Qué Visitar en Encarnación?","output":"qué-visitar-encarnación","rules_applied":["¿? removed","stopwords (en)","preserve é and ó"]},{"input":"\"Las Misiones Jesuíticas\" de Paraguay","output":"misiones-jesuíticas-paraguay","rules_applied":["quotes removed","stopwords (las","de)","preserve í"]},{"input":"Mbopi\'y: Murciélagos y Biodiversidad","output":"mbopiy-murciélagos-biodiversidad","rules_applied":["apostrophe removed","stopwords (y)","Guarani loanword preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-PY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-SV'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (SV) Slugification',
    s.content = 'URL slug generation rules for es-SV',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["el","la","los","las","un","una"],"preposition":["de","en","con","por","para","a"],"contraction":["al","del"],"verb":["es","son"],"conjunction":["y","o","que"]}',
    s.stopwords_count = 19,
    s.regional_additions = '[{"word":"","category":"al","reason":"contraction"},{"word":"","category":"del","reason":"contraction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"May indicate processing error"}]',
    s.examples = '[{"input":"Guía de turismo en El Salvador","output":"guía-turismo-salvador","rules_applied":["lowercase","stopwords (de","en","el)","preserve ía"]},{"input":"Las mejores pupusas de San Salvador","output":"mejores-pupusas-san-salvador","rules_applied":["stopwords (las","de)","preserve diacritics"]},{"input":"Cómo viajar a la costa del Pacífico","output":"cómo-viajar-costa-pacífico","rules_applied":["stopwords (a","la","del)","preserve ó","í"]},{"input":"Fútbol salvadoreño hoy","output":"fútbol-salvadoreño-hoy","rules_applied":["lowercase","preserve ú","ñ"]},{"input":"Clima y temperatura en Sonsonate","output":"clima-temperatura-sonsonate","rules_applied":["stopwords (y","en)","preserve text"]},{"input":"Top 10 restaurantes en Santa Ana 2025","output":"top-10-restaurantes-santa-ana-2025","rules_applied":["stopwords (en)","numbers preserved"]},{"input":"Las tradiciones culturales más importantes de la celebración del Día de la Independencia en El Salvador","output":"tradiciones-culturales-más-importantes-celebración-día-independencia-salvador","rules_applied":["truncated at 80 chars","stopwords removed"]},{"input":"Artesanías & productos típicos: guía completa","output":"artesanías-productos-típicos-guía-completa","rules_applied":["ampersand and colon removed","preserve í"]},{"input":"\"La Palma\" un pueblo con arte naíf","output":"palma-pueblo-arte-naíf","rules_applied":["quotes removed","stopwords (la","un","con)","preserve í"]},{"input":"El volcán de Santa Ana: Ilamatepec","output":"volcán-santa-ana-ilamatepec","rules_applied":["stopwords (el","de)","colon removed","preserve ó"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-SV.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-UY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (UY) Slugification',
    s.content = 'URL slug generation rules for es-UY',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["su","sus","mi","mis"],"conjunction":["y","o","pero","que","si"],"article":["el","la","los","las","un","una"],"verb":["es","son","sos"],"interjection":["bo","ta"],"contraction":["del","al"],"preposition":["de","en","a","con","por","para"],"adverb":["muy","más","tan"]}',
    s.stopwords_count = 31,
    s.regional_additions = '[{"word":"","category":"bo","reason":"interjection"},{"word":"","category":"sos","reason":"verb"},{"word":"","category":"ta","reason":"interjection"},{"word":"","category":"muy","reason":"adverb"},{"word":"","category":"más","reason":"adverb"},{"word":"","category":"tan","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"ASCII-only slug","message":"Missing expected Spanish diacritics - verify source text"}]',
    s.examples = '[{"input":"Mejores Playas de Punta del Este","output":"mejores-playas-punta-este","rules_applied":["Stopwords removed (de","del)","diacritics preserved"]},{"input":"Cómo Preparar un Chivito Uruguayo","output":"cómo-preparar-chivito-uruguayo","rules_applied":["Stopwords removed (un)","diacritics preserved"]},{"input":"Guía del Mate: Tradición Charrúa","output":"guía-mate-tradición-charrúa","rules_applied":["Stopwords removed (del)","colon removed","diacritics preserved"]},{"input":"Receta de Asado con Cuero Tradicional","output":"receta-asado-cuero-tradicional","rules_applied":["Stopwords removed (de","con)","diacritics preserved"]},{"input":"Los 10 Mejores Restaurantes de Montevideo","output":"10-mejores-restaurantes-montevideo","rules_applied":["Stopwords removed (los","de)","number preserved"]},{"input":"Fútbol en Uruguay: Peñarol vs Nacional 2025","output":"fútbol-uruguay-peñarol-vs-nacional-2025","rules_applied":["Stopwords removed (en)","diacritics preserved","number kept"]},{"input":"Historia del Candombe en Montevideo: Desde los Tambores Originales hasta las Llamadas Contemporáneas","output":"historia-candombe-montevideo-tambores-originales-llamadas-contemporáneas","rules_applied":["Stopwords removed (del","en","desde","los","hasta","las)","truncated at 80 chars"]},{"input":"¿Dónde Comprar Tannat Artesanal?","output":"dónde-comprar-tannat-artesanal","rules_applied":["Question marks removed","diacritics preserved"]},{"input":"\"El Pozo\" - Análisis del Cuento de Onetti","output":"pozo-análisis-cuento-onetti","rules_applied":["Quotes removed","stopwords removed (el","del","de)"]},{"input":"¿Por qué el dulce de leche es tan popular en Uruguay?","output":"dulce-leche-popular-uruguay","rules_applied":["Question marks removed","stopwords removed (por","qué","el","de","es","tan","en)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-UY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'es-VE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ES (VE) Slugification',
    s.content = 'URL slug generation rules for es-VE',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["es","son"],"interjection":["vale","mira"],"preposition":["de","en","a","con","por","para"],"contraction":["del","al"],"article":["el","la","los","las","un","una"],"pronoun":["su","sus","mi","mis"],"conjunction":["y","o","pero","que","si","pues"]}',
    s.stopwords_count = 28,
    s.regional_additions = '[{"word":"","category":"pues","reason":"conjunction"},{"word":"","category":"vale","reason":"interjection"},{"word":"","category":"mira","reason":"interjection"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"ASCII-only slug","message":"Missing expected Spanish diacritics - verify source text"}]',
    s.examples = '[{"input":"Mejores Arepas Rellenas en Caracas","output":"mejores-arepas-rellenas-caracas","rules_applied":["Stopwords removed (en)","diacritics preserved"]},{"input":"Guía Completa del Pabellón Criollo","output":"guía-completa-pabellón-criollo","rules_applied":["Stopwords removed (del)","diacritics preserved"]},{"input":"¿Cómo Preparar Hallacas Navideñas?","output":"cómo-preparar-hallacas-navideñas","rules_applied":["Punctuation removed","diacritics preserved"]},{"input":"Recetas de la Abuela: Cachapa con Queso de Mano","output":"recetas-abuela-cachapa-queso-mano","rules_applied":["Stopwords removed (de","la","con)","colon removed"]},{"input":"Carnaval de El Callao: Tradiciones y Patrimonio","output":"carnaval-callao-tradiciones-patrimonio","rules_applied":["Stopwords removed (de","el","y)","diacritics preserved"]},{"input":"10 Playas más Hermosas de Margarita 2025","output":"10-playas-hermosas-margarita-2025","rules_applied":["Number preserved","stopwords removed (más","de)"]},{"input":"Historia del Joropo Llanero en Venezuela: Desde sus Orígenes hasta la Música Contemporánea Actual","output":"historia-joropo-llanero-venezuela-orígenes-música-contemporánea-actual","rules_applied":["Stopwords removed (del","en","desde","sus","hasta","la)","truncated at 80 chars"]},{"input":"Mejores Restaurantes & Bares en Los Palos Grandes","output":"mejores-restaurantes-bares-palos-grandes","rules_applied":["Ampersand removed","stopwords removed (en","los)"]},{"input":"\"Doña Bárbara\" - Análisis de la Novela Venezolana","output":"doña-bárbara-análisis-novela-venezolana","rules_applied":["Quotes removed","stopwords removed (de","la)"]},{"input":"¿Por qué el petróleo es tan importante para Venezuela?","output":"petróleo-importante-venezuela","rules_applied":["Question marks removed","stopwords removed (por","qué","el","es","tan","para)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/es-VE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'et-EE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Estonian (Estonia) Slugification',
    s.content = 'URL slug generation rules for et-EE',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["ja","ning","või","aga","kuid","et"],"verb":["on","oli"],"pronoun":["see","mis","kes","ta","tema"],"adverb":["ka","veel","siin","seal","nüüd"],"preposition":["kui"],"possessive":["oma"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"eesti","reason":"adjective"},{"word":"","category":"siin","reason":"adverb"},{"word":"","category":"seal","reason":"adverb"},{"word":"","category":"nüüd","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Verify intentional use"}]',
    s.examples = '[{"input":"Tallinna vanalinn ja selle ajalugu","output":"tallinna-vanalinn-selle-ajalugu","rules_applied":["stopwords: ja"]},{"input":"Eesti looduse ilu","output":"looduse-ilu","rules_applied":["stopwords: eesti"]},{"input":"Kuidas õppida eesti keelt","output":"kuidas-õppida-keelt","rules_applied":["stopwords: eesti; preserve: õ"]},{"input":"Pärnu rannad ja suvituskohad","output":"pärnu-rannad-suvituskohad","rules_applied":["stopwords: ja; preserve: ä"]},{"input":"Saaremaa küla elu ning traditsioonid","output":"saaremaa-küla-elu-traditsioonid","rules_applied":["stopwords: ning; preserve: ü"]},{"input":"10 parimat restorani Tallinnas 2025","output":"10-parimat-restorani-tallinnas-2025","rules_applied":["numbers preserved"]},{"input":"Eesti Vabariigi aastapäeva tähistamine ning rahvuslikud kombed ja traditsioonid läbi aegade","output":"eesti-vabariigi-aastapäeva-tähistamine-rahvuslikud-kombed-traditsioonid-läbi-aegade","rules_applied":["truncation: 80 chars; stopwords: ning","ja"]},{"input":"Mis on \"startup\" kultuur?","output":"startup-kultuur","rules_applied":["stopwords: mis","on; punctuation removed"]},{"input":"Minu ema\'s retseptid & nõuanded","output":"minu-emas-retseptid-nõuanded","rules_applied":["apostrophe","ampersand removed; preserve: õ"]},{"input":"Žanri mõju Šostakovitši muusikale","output":"žanri-mõju-šostakovitši-muusikale","rules_applied":["preserve: ž","š","õ"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/et-EE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'eu-ES'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'EU (ES) Slugification',
    s.content = 'URL slug generation rules for eu-ES',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"quantifier":["guzti","batzuk"]}',
    s.stopwords_count = 2,
    s.regional_additions = '[{"word":"","category":"guzti","reason":"quantifier"},{"word":"","category":"batzuk","reason":"quantifier"},{"word":"","category":"beste","reason":"adjective"},{"word":"","category":"berri","reason":"adjective"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Latin with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"Donostiako jatetxe onenak","output":"donostiako-jatetxe-onenak","rules_applied":["Lowercase","hyphenation"]},{"input":"Euskal kulturaren historia","output":"euskal-kulturaren-historia","rules_applied":["Lowercase","no stopwords to remove"]},{"input":"Bilboko museo eta arte galeriak","output":"bilboko-museo-arte-galeriak","rules_applied":["Stopword removed (eta)","lowercase"]},{"input":"Nola bisitatu Gasteiz","output":"bisitatu-gasteiz","rules_applied":["Stopword removed (nola)","lowercase"]},{"input":"Mendizaletasuna Euskal Herrian","output":"mendizaletasuna-euskal-herrian","rules_applied":["No stopwords","lowercase","hyphenation"]},{"input":"10 tokik Gipuzkoan ikusteko","output":"10-tokik-gipuzkoan-ikusteko","rules_applied":["Number preserved","lowercase"]},{"input":"Euskal sukaldaritza tradizionala: gure herriko plater tipikoak eta haien prestaketa xehetasunak nola egin etxean","output":"euskal-sukaldaritza-tradizionala-gure-herriko-plater-tipikoak-haien-prestaketa","rules_applied":["Truncated to 80 chars","stopwords removed (eta","nola)"]},{"input":"Surf & hondartzak: Zarautz, Mundaka eta Sopela!","output":"surf-hondartzak-zarautz-mundaka-sopela","rules_applied":["Special chars removed (&",":","!)","stopword removed (eta)"]},{"input":"\"Euskara batua\" edo dialektoak?","output":"euskara-batua-dialektoak","rules_applied":["Quotes removed","stopword (edo) removed","question mark removed"]},{"input":"Txakoli ardoa: Getariako upeltegiak","output":"txakoli-ardoa-getariako-upeltegiak","rules_applied":["Digraph (tx) preserved","colon removed","lowercase"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/eu-ES.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fa-IR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Persian (Iran) Slugification',
    s.content = 'URL slug generation rules for fa-IR',
    s.slug_rule = 'native_script',
    s.stopwords = '{"preposition":["به","از","در","با","برای"],"auxiliary":["می"],"adverb":["نیز","هم","حتی"],"demonstrative":["این","آن"],"conjunction":["و","یا","که"]}',
    s.stopwords_count = 14,
    s.regional_additions = '[{"word":"","category":"می","reason":"auxiliary"},{"word":"","category":"نیز","reason":"adverb"},{"word":"","category":"هم","reason":"adverb"},{"word":"","category":"حتی","reason":"adverb"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Mixed Arabic/Latin detected (except numbers)"},{"condition":"Non-standard Unicode","message":"Non-NFC composition detected"}]',
    s.examples = '[{"input":"راهنمای سفر به ایران","output":"راهنمای-سفر-ایران","rules_applied":["Stopwords removed (به)","spaces→hyphens"]},{"input":"آموزش زبان فارسی برای مبتدیان","output":"آموزش-زبان-فارسی-مبتدیان","rules_applied":["Stopwords removed (برای)","preserved Persian script"]},{"input":"بهترین رستوران‌های تهران","output":"بهترین-رستوران‌های-تهران","rules_applied":["Zero-width non-joiner handled","native script preserved"]},{"input":"تاریخ و فرهنگ ایران","output":"تاریخ-فرهنگ-ایران","rules_applied":["Stopwords removed (و)","conjunction removed"]},{"input":"دستور پخت ته‌چین سنتی","output":"دستور-پخت-ته‌چین-سنتی","rules_applied":["Persian food term preserved","hyphen normalized"]},{"input":"۱۰ نکته برای موفقیت در کسب‌وکار","output":"۱۰-نکته-موفقیت-کسب‌وکار","rules_applied":["Persian-Indic digits preserved","stopwords removed (برای","در)"]},{"input":"چگونه در کمترین زمان به اهداف خود برسیم و موفق شویم در زندگی شخصی و حرفه‌ای","output":"چگونه-کمترین-زمان-اهداف-خود-برسیم-موفق-شویم-زندگی-شخصی-حرفه‌ای","rules_applied":["Long title truncated at 80 chars","multiple stopwords removed"]},{"input":"سفر به اصفهان: نصف جهان!","output":"سفر-اصفهان-نصف-جهان","rules_applied":["Punctuation removed (colon","exclamation)","stopwords removed (به)"]},{"input":"کتاب \"هزار و یک شب\" داستانی جذاب","output":"کتاب-هزار-یک-شب-داستانی-جذاب","rules_applied":["Quotes removed","stopwords removed (و)","numbers preserved"]},{"input":"پل خواجو؛ شاهکار معماری صفوی","output":"پل-خواجو-شاهکار-معماری-صفوی","rules_applied":["Persian semicolon removed","historical/cultural terms preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fa-IR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fi-FI'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Finnish (Finland) Slugification',
    s.content = 'URL slug generation rules for fi-FI',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["on","ei","oli","ovat"],"pronoun":["se","tämä","joka","mitä"],"conjunction":["ja","tai","sekä","että","mutta","vaan","eli","kun","jos","vaikka","koska","jotta"],"quantifier":["kaikki"],"adverb":["myös"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[{"word":"","category":"vuonna","reason":"temporal"},{"word":"","category":"vuoden","reason":"temporal"},{"word":"","category":"kuin","reason":"comparison"},{"word":"","category":"kaikki","reason":"quantifier"},{"word":"","category":"myös","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Non-Latin detected (unexpected for Finnish)"},{"condition":"Missing ä/ö/å in original","message":"Verify if diacritics were lost"}]',
    s.examples = '[{"input":"Parhaat Ravintolat Helsingissä","output":"parhaat-ravintolat-helsingissä","rules_applied":["Preserved ä","removed stopwords","lowercase"]},{"input":"Suomen Luonto ja Järvet","output":"suomen-luonto-järvet","rules_applied":["Removed \"ja\" (conjunction)","preserved ä"]},{"input":"Miten Valita Paras Älypuhelin","output":"miten-valita-paras-älypuhelin","rules_applied":["Preserved ä","lowercase","hyphenated"]},{"input":"Talvi Suomessa: Lumilautailu & Hiihto","output":"talvi-suomessa-lumilautailu-hiihto","rules_applied":["Removed &","preserved colon\'s semantic space"]},{"input":"Käsikirja Mökin Rakentamiseen","output":"käsikirja-mökin-rakentamiseen","rules_applied":["Preserved ä","ö in compound words"]},{"input":"10 Vinkkiä Saunomiseen 2025","output":"10-vinkkiä-saunomiseen-2025","rules_applied":["Kept numbers","preserved ä"]},{"input":"Opas Suomalaiseen Ruokakulttuuriin joka Toimii Aina Hyvin Kaikissa Tilanteissa","output":"opas-suomalaiseen-ruokakulttuuriin-toimii-aina-hyvin-kaikissa","rules_applied":["Removed \"joka\" (pronoun)","truncated at 80 chars"]},{"input":"\"Kesä Suomessa\" – Mitä Tehdä?","output":"kesä-suomessa-tehdä","rules_applied":["Removed quotes","question mark","\"mitä\" (pronoun)","preserved ä"]},{"input":"Työelämä & Ura: Näin Onnistut!","output":"työelämä-ura-näin-onnistut","rules_applied":["Removed & and !","preserved ö and ä"]},{"input":"Mökkiläisten Opas Saaristoon","output":"mökkiläisten-opas-saaristoon","rules_applied":["Preserved ä and ö in compound word \"mökkiläisten\""]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fi-FI.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-BE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (BE) Slugification',
    s.content = 'URL slug generation rules for fr-BE',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["de","du","à","au","aux","en","dans","pour","sur","avec"],"article":["le","la","les","un","une","des"],"conjunction":["et","ou"]}',
    s.stopwords_count = 18,
    s.regional_additions = '[{"word":"","category":"septante","reason":"numeral"},{"word":"","category":"nonante","reason":"numeral"},{"word":"","category":"kot","reason":"noun"},{"word":"","category":"navetteur","reason":"noun"},{"word":"","category":"gsm","reason":"noun"},{"word":"","category":"mutuelle","reason":"noun"},{"word":"","category":"onem","reason":"noun"},{"word":"","category":"cpas","reason":"noun"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for Belgian SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Latin characters detected (unexpected for fr-BE)"},{"condition":"All stopwords","message":"Title contains only stopwords - manual slug needed"},{"condition":"Bilingual titles","message":"Consider separate FR/NL versions for Belgian market"}]',
    s.examples = '[{"input":"Meilleures Friteries de Bruxelles","output":"meilleures-friteries-bruxelles","rules_applied":["Stopwords removed (de)","Belgian location preserved"]},{"input":"Guide des Gaufres Liégeoises à Liège","output":"guide-gaufres-liégeoises-liège","rules_applied":["Stopwords removed (des","à)","accent preserved"]},{"input":"Septante Recettes de Carbonade Flamande","output":"recettes-carbonade-flamande","rules_applied":["Belgian numeral stopword removed (septante)","stopword removed (de)"]},{"input":"L\'Atomium et le Manneken-Pis : Visiter Bruxelles","output":"atomium-manneken-pis-visiter-bruxelles","rules_applied":["Apostrophe/colon removed","stopwords removed (et","le)"]},{"input":"Top 21 Juillet 2025 : Fête Nationale Belge","output":"top-21-juillet-2025-fête-nationale-belge","rules_applied":["Numbers preserved","accents preserved","colon removed"]},{"input":"Proximus vs Orange Belgique : Comparer les Forfaits GSM","output":"proximus-orange-belgique-comparer-forfaits","rules_applied":["Belgian brands preserved","stopwords removed (vs","les)","gsm removed"]},{"input":"Carnaval de Binche : Les Plus Beaux Gilles et Costumes Traditionnels de Wallonie","output":"carnaval-binche-plus-beaux-gilles-costumes-traditionnels-wallonie","rules_applied":["Stopwords removed","truncated at 80 chars"]},{"input":"Belfius, ING Belgique & BNP Paribas Fortis - Comparer les Banques","output":"belfius-ing-belgique-bnp-paribas-fortis-comparer-banques","rules_applied":["Ampersand/hyphen removed","Belgian banks preserved"]},{"input":"\"Louvain-la-Neuve\" : Ville Universitaire et Kots Étudiants","output":"louvain-la-neuve-ville-universitaire-étudiants","rules_applied":["Quotes removed","kot stopword applied","compound city name preserved"]},{"input":"Moules-Frites à Namur : Où Trouver les Meilleurs ?","output":"moules-frites-namur-trouver-meilleurs","rules_applied":["Stopwords removed (à","les)","punctuation removed","Belgian dish preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-BE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-BF'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (BF) Slugification',
    s.content = 'URL slug generation rules for fr-BF',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["le","la","les","un","une","des"],"interjection":["bon","voila"],"filler":["quoi"],"adverb":["meme","aussi"],"preposition":["de","du","a","au","aux","en","pour","dans","sur","avec","chez"],"conjunction":["et","ou","wala","donc"]}',
    s.stopwords_count = 26,
    s.regional_additions = '[{"word":"","category":"wala","reason":"conjunction"},{"word":"","category":"bon","reason":"interjection"},{"word":"","category":"voila","reason":"interjection"},{"word":"","category":"donc","reason":"conjunction"},{"word":"","category":"quoi","reason":"filler"},{"word":"","category":"meme","reason":"adverb"},{"word":"","category":"aussi","reason":"adverb"},{"word":"","category":"chez","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO effectiveness"},{"condition":"Slug > 60 chars","message":"Consider shortening for better readability"},{"condition":"Mixed scripts detected","message":"Verify intentional use of multiple scripts"}]',
    s.examples = '[{"input":"Emploi a Ouagadougou pour les jeunes diplomes","output":"emploi-ouagadougou-jeunes-diplomes","rules_applied":["stopwords (a","pour","les)","Ouaga capital preserved"]},{"input":"Location maison a Bobo-Dioulasso quartier Sarfalao","output":"location-maison-bobo-dioulasso-quartier-sarfalao","rules_applied":["stopword (a)","compound city name and neighborhood preserved"]},{"input":"FESPACO 2025 : Programme du Festival Panafricain du Cinema","output":"fespaco-2025-programme-festival-panafricain-cinema","rules_applied":["stopwords (du)","remove colon","Burkinabe film festival"]},{"input":"Guide touristique des Cascades de Banfora et Pics de Sindou","output":"guide-touristique-cascades-banfora-pics-sindou","rules_applied":["stopwords (des","de","et)","Cascades region landmarks"]},{"input":"L\'economie du coton au Burkina Faso : or blanc du Sahel","output":"economie-coton-burkina-faso-blanc-sahel","rules_applied":["stopwords (l\'","du","au)","remove colon","cotton industry key sector"]},{"input":"Top 10 maquis poulet bicyclette a Ouaga secteur 15","output":"top-10-maquis-poulet-bicyclette-ouaga-secteur-15","rules_applied":["stopword (a)","numbers kept","Burkinabe grilled chicken specialty"]},{"input":"Semaine Nationale de la Culture SNC Bobo 2026 : Arts traditions et patrimoine culturel du pays des hommes integres","output":"semaine-nationale-culture-snc-bobo-2026-arts-traditions-patrimoine-culturel-pays","rules_applied":["truncate at 80 chars","stopwords removed","SNC Bobo festival"]},{"input":"Orange Burkina & Moov Africa : forfaits internet mobile XOF","output":"orange-burkina-moov-africa-forfaits-internet-mobile-xof","rules_applied":["remove punctuation and ampersand","Burkinabe telecoms and CFA franc"]},{"input":"Recette traditionnelle du to et sauce gombo : plat burkinabe au dolo","output":"recette-traditionnelle-to-sauce-gombo-plat-burkinabe-dolo","rules_applied":["remove colon","stopwords (du","et","au)","Burkinabe staple foods and millet beer"]},{"input":"Nuits Atypiques de Koudougou NAK : Festival musique et Coris Bank International sponsor","output":"nuits-atypiques-koudougou-nak-festival-musique-coris-bank-international-sponsor","rules_applied":["stopwords (de","et)","NAK music festival and Burkinabe bank preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-BF.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-CA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'French (Canada) Slugification',
    s.content = 'URL slug generation rules for fr-CA',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"adverb":["ben","icitte"],"pronoun":["qui","ce","cette","ces"],"article":["le","la","les","un","une","des"],"conjunction":["et","ou","mais","donc","que","pis"],"preposition":["de","du","dans","pour","avec","sur","par","en","aux","au","chez"],"verb":["est","sont"]}',
    s.stopwords_count = 31,
    s.regional_additions = '[{"word":"","category":"pis","reason":"conjunction"},{"word":"","category":"ben","reason":"adverb"},{"word":"","category":"chez","reason":"preposition"},{"word":"","category":"icitte","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Verify intentional (rare for fr-CA)"}]',
    s.examples = '[{"input":"Comment acheter un char au Québec","output":"comment-acheter-char-québec","rules_applied":["lowercase","stopwords (un","au)","preserve é"]},{"input":"Les meilleurs poutines de Montréal","output":"meilleurs-poutines-montréal","rules_applied":["stopwords (les","de)","preserve é"]},{"input":"Guide du sirop d\'érable pour les débutants","output":"guide-sirop-dérable-débutants","rules_applied":["stopwords (du","pour","les)","apostrophe removed"]},{"input":"Où trouver des dépanneurs ouverts la nuit","output":"où-trouver-dépanneurs-ouverts-nuit","rules_applied":["stopwords (des","la)","preserve ù"]},{"input":"Le hockey sur glace à Québec","output":"hockey-glace-québec","rules_applied":["stopwords (le","sur","à)","preserve é"]},{"input":"10 conseils pour l\'hiver 2025","output":"10-conseils-lhiver-2025","rules_applied":["numbers preserved","stopwords (pour)","apostrophe removed"]},{"input":"Comment préparer une tourtière traditionnelle du Lac-Saint-Jean pour les fêtes de fin d\'année au Québec","output":"comment-préparer-tourtière-traditionnelle-lac-saint-jean-fêtes-fin-dannée-québec","rules_applied":["truncation at 80 chars","stopwords removed"]},{"input":"Où manger? Les meilleurs restos de la Rive-Sud!","output":"où-manger-meilleurs-restos-rive-sud","rules_applied":["punctuation removed (?","!)","stopwords (les","de","la)"]},{"input":"L\'été à Trois-Rivières : activités et sorties","output":"lété-trois-rivières-activités-sorties","rules_applied":["apostrophe removed","colon removed","stopwords (à","et)"]},{"input":"Hébergement à l\'Île d\'Orléans","output":"hébergement-lîle-dorléans","rules_applied":["preserve é","î","apostrophe handling"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-CA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-CD'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (CD) Slugification',
    s.content = 'URL slug generation rules for fr-CD',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["eza"],"preposition":["de","du","à","au","aux","en","pour","dans","sur","avec"],"article":["le","la","les","un","une","des"],"adverb":["kaka","très","trop"],"conjunction":["et","ou"],"possessive":["ya"],"interrogative":["nini"],"negation":["te"]}',
    s.stopwords_count = 25,
    s.regional_additions = '[{"word":"","category":"na","reason":"connector"},{"word":"","category":"ya","reason":"possessive"},{"word":"","category":"te","reason":"negation"},{"word":"","category":"nini","reason":"interrogative"},{"word":"","category":"eza","reason":"verb"},{"word":"","category":"kaka","reason":"adverb"},{"word":"","category":"très","reason":"adverb"},{"word":"","category":"trop","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Latin characters detected (unexpected for fr-CD)"},{"condition":"All stopwords","message":"Title contains only stopwords - manual slug needed"}]',
    s.examples = '[{"input":"Emploi Kinshasa : Offres Vodacom Congo et Orange RDC","output":"emploi-kinshasa-offres-vodacom-congo-orange-rdc","rules_applied":["Punctuation removed","stopwords removed (et)","brand names preserved"]},{"input":"Guide M-Pesa RDC : Envoyer Francs Congolais CDF","output":"guide-m-pesa-rdc-envoyer-francs-congolais-cdf","rules_applied":["Hyphenated brand M-Pesa preserved","currency code kept"]},{"input":"Recette Poulet Moambé na Saka-Saka Traditionnel","output":"recette-poulet-moambé-saka-saka-traditionnel","rules_applied":["Stopword removed (na)","dish names preserved with accents"]},{"input":"Appartement à Louer Lubumbashi Quartier Makutano","output":"appartement-louer-lubumbashi-quartier-makutano","rules_applied":["Stopword removed (à)","neighborhood name preserved"]},{"input":"Concert Fally Ipupa Kinshasa Stade des Martyrs 30 Juin 2026","output":"concert-fally-ipupa-kinshasa-stade-martyrs-30-juin-2026","rules_applied":["Stopwords removed (des)","date preserved","artist name kept"]},{"input":"Top 10 Restaurants Goma avec Vue Lac Kivu","output":"top-10-restaurants-goma-vue-lac-kivu","rules_applied":["Number preserved","stopword removed (avec)","locations kept"]},{"input":"Rawbank et Trust Merchant Bank : Services Bancaires Équité BCDC pour Entreprises Congolaises Kinshasa","output":"rawbank-trust-merchant-bank-services-bancaires-équité-bcdc-entreprises-congolaises","rules_applied":["Stopwords removed (et","pour)","truncated at 80 chars"]},{"input":"Visa RDC & Formalités : Ambassade Bukavu - Goma - Kisangani","output":"visa-rdc-formalités-ambassade-bukavu-goma-kisangani","rules_applied":["Ampersand/colon/dash removed","city names preserved","accent kept"]},{"input":"\"Mbote na Bino\" : Expressions Lingala Populaires Kinshasa","output":"mbote-bino-expressions-lingala-populaires-kinshasa","rules_applied":["Quotes removed","stopword removed (na)","Lingala greeting preserved"]},{"input":"L\'Économie Minière Kolwezi Mbuji-Mayi : Cuivre et Diamants du Katanga","output":"économie-minière-kolwezi-mbuji-mayi-cuivre-diamants-katanga","rules_applied":["Apostrophe removed","stopwords removed (et","du)","hyphenated city preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-CD.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-CH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (CH) Slugification',
    s.content = 'URL slug generation rules for fr-CH',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["et","ou"],"article":["le","la","les","un","une"],"preposition":["de","du","des","à","au","aux","en","sur","pour","avec","dans","par"],"verb":["est","sont"],"pronoun":["que","qui"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[{"word":"","category":"sur","reason":"preposition"},{"word":"","category":"pour","reason":"preposition"},{"word":"","category":"avec","reason":"preposition"},{"word":"","category":"dans","reason":"preposition"},{"word":"","category":"par","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Review for consistency"}]',
    s.examples = '[{"input":"Les meilleures fondue de Gruyères","output":"meilleures-fondue-gruyères","rules_applied":["stopwords (les","de)","lowercase","preserve é","è"]},{"input":"Horlogerie suisse et tradition","output":"horlogerie-suisse-tradition","rules_applied":["stopwords (et)","lowercase","preserve"]},{"input":"Café du Commerce à Lausanne","output":"café-commerce-lausanne","rules_applied":["stopwords (du","à)","preserve é"]},{"input":"Randonnée dans les Alpes bernoises","output":"randonnée-alpes-bernoises","rules_applied":["stopwords (dans","les)","preserve é"]},{"input":"Guide des stations de ski en Valais","output":"guide-stations-ski-valais","rules_applied":["stopwords (des","de","en)","lowercase"]},{"input":"10 recettes de rösti traditionnelles","output":"10-recettes-rösti-traditionnelles","rules_applied":["stopwords (de)","preserve ö (German influence)"]},{"input":"Découvrez les plus beaux villages de la Suisse romande et leurs spécialités culinaires régionales","output":"découvrez-plus-beaux-villages-suisse-romande-leurs-spécialités-culinaires-régionales","rules_applied":["stopwords","truncate ≤80 chars","preserve accents"]},{"input":"L\'économie suisse : perspectives 2025","output":"léconomie-suisse-perspectives-2025","rules_applied":["remove punctuation (:)","preserve é","apostrophe handling"]},{"input":"«Bienvenue à Genève» dit le panneau","output":"bienvenue-genève-dit-panneau","rules_applied":["remove guillemets («»)","stopwords (à","le)","preserve è"]},{"input":"Müesli, rösti et autres spécialités","output":"müesli-rösti-autres-spécialités","rules_applied":["preserve ü","ö (Swiss German borrowings)","stopwords (et)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-CH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-CI'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (CI) Slugification',
    s.content = 'URL slug generation rules for fr-CI',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"filler":["quoi"],"conjunction":["et","ou"],"adverb":["meme","ici"],"interjection":["deh","ya"],"contraction":["cest"],"article":["le","la","les","un","une","des"],"preposition":["de","du","a","au","aux","en","pour","dans","sur","avec"]}',
    s.stopwords_count = 24,
    s.regional_additions = '[{"word":"","category":"deh","reason":"interjection"},{"word":"","category":"ya","reason":"interjection"},{"word":"","category":"quoi","reason":"filler"},{"word":"","category":"meme","reason":"adverb"},{"word":"","category":"cest","reason":"contraction"},{"word":"","category":"ici","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO effectiveness"},{"condition":"Slug > 60 chars","message":"Consider shortening for better readability"},{"condition":"Mixed scripts detected","message":"Verify intentional use of multiple scripts"}]',
    s.examples = '[{"input":"Emploi a Abidjan pour les jeunes diplomes","output":"emploi-abidjan-jeunes-diplomes","rules_applied":["stopwords (a","pour","les)","preserve accents"]},{"input":"Appartement a louer au Plateau Cocody","output":"appartement-louer-plateau-cocody","rules_applied":["stopwords (a","au)","Abidjan districts preserved"]},{"input":"Guide touristique de Grand-Bassam","output":"guide-touristique-grand-bassam","rules_applied":["stopword (de)","UNESCO heritage site preserved"]},{"input":"Le FEMUA Festival 2025 a Anoumabo","output":"femua-festival-2025-anoumabo","rules_applied":["stopwords (le","a)","keep numbers","Ivorian music festival"]},{"input":"L\'attieke ivoirien : preparation traditionnelle","output":"attieke-ivoirien-preparation-traditionnelle","rules_applied":["stopword (l\')","remove colon","Ivorian cassava couscous"]},{"input":"Top 10 garba et alloco a Adjame","output":"top-10-garba-alloco-adjame","rules_applied":["stopwords (et","a)","numbers preserved","Ivorian street food"]},{"input":"Yamoussoukro capitale politique de Cote d\'Ivoire : Basilique Notre-Dame de la Paix et tourisme culturel ivoirien","output":"yamoussoukro-capitale-politique-cote-divoire-basilique-notre-dame-paix-tourisme","rules_applied":["truncate at 80 chars","stopwords removed"]},{"input":"Orange Cote d\'Ivoire & MTN CI : forfaits internet mobile !","output":"orange-cote-divoire-mtn-ci-forfaits-internet-mobile","rules_applied":["remove punctuation and ampersand","Ivorian telecoms"]},{"input":"L\'economie du cacao : \"or brun\" de Cote d\'Ivoire","output":"economie-cacao-or-brun-cote-divoire","rules_applied":["remove quotes","preserve diacritics","cocoa industry"]},{"input":"OEuvres dart de la lagune Ebrie et du quartier Treichville","output":"oeuvres-dart-lagune-ebrie-quartier-treichville","rules_applied":["preserve ligature oe","stopwords (de","la","du","et)","Abidjan landmarks"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-CI.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-CM'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (CM) Slugification',
    s.content = 'URL slug generation rules for fr-CM',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"adverb":["dja","meme","ici"],"conjunction":["et","ou"],"contraction":["cest"],"article":["le","la","les","un","une"],"preposition":["de","du","des","a","au","aux","en","pour","dans","sur","avec"],"filler":["ca"],"verb":["go"]}',
    s.stopwords_count = 24,
    s.regional_additions = '[{"word":"","category":"ca","reason":"filler"},{"word":"","category":"dja","reason":"adverb"},{"word":"","category":"meme","reason":"adverb"},{"word":"","category":"ici","reason":"adverb"},{"word":"","category":"tara","reason":"noun"},{"word":"","category":"ya","reason":"existential"},{"word":"","category":"go","reason":"verb"},{"word":"","category":"cest","reason":"contraction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Latin characters detected (unexpected for fr-CM)"},{"condition":"All stopwords","message":"Title contains only stopwords - manual slug needed"}]',
    s.examples = '[{"input":"Emploi Douala : Offres MTN Cameroun et Orange Cameroun","output":"emploi-douala-offres-mtn-cameroun-orange-cameroun","rules_applied":["Punctuation removed","stopwords removed (et)","telecom brands preserved"]},{"input":"Appartement Yaounde Quartier Bastos 3 Chambres XAF","output":"appartement-yaounde-quartier-bastos-3-chambres-xaf","rules_applied":["Location preserved","number kept","currency code XAF kept"]},{"input":"Recette Ndole Traditionnel et Poulet DG Camerounais","output":"recette-ndole-traditionnel-poulet-dg-camerounais","rules_applied":["Stopwords removed (et)","dish names preserved","DG acronym kept"]},{"input":"Safari Waza Parc National Nord Cameroun Guide","output":"safari-waza-parc-national-nord-cameroun-guide","rules_applied":["Park name preserved","region Nord kept"]},{"input":"Festival Ngondo Douala : Fete Eau Peuple Sawa 2026","output":"festival-ngondo-douala-fete-eau-peuple-sawa-2026","rules_applied":["Cultural event preserved","date kept","punctuation removed"]},{"input":"Top 10 Hotels Kribi Plage Vacances Franc CFA","output":"top-10-hotels-kribi-plage-vacances-franc-cfa","rules_applied":["Number preserved","beach resort Kribi","currency preserved"]},{"input":"Afriland First Bank Bicec Ecobank Cameroun Services Bancaires Entreprises Douala Yaounde Bafoussam","output":"afriland-first-bank-bicec-ecobank-cameroun-services-bancaires-entreprises-douala","rules_applied":["Truncated at 80 chars","Cameroon banks preserved"]},{"input":"Actualites Bamenda & Bafoussam : Fete Nationale 20 Mai","output":"actualites-bamenda-bafoussam-fete-nationale-20-mai","rules_applied":["Ampersand/colon removed","Anglophone regions preserved","national day date"]},{"input":"\"Les Gars de Limbe\" : Miondo et Beignets-Haricots Garoua","output":"gars-limbe-miondo-beignets-haricots-garoua","rules_applied":["Quotes removed","stopwords removed (les","de","et)","Cameroon food preserved"]},{"input":"L\'Eru Cuisine Bamileke Mont Cameroun Bafoussam Region Ouest","output":"eru-cuisine-bamileke-mont-cameroun-bafoussam-region-ouest","rules_applied":["Apostrophe removed","ethnic cuisine Bamileke preserved","volcano name kept"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-CM.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-DZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (DZ) Slugification',
    s.content = 'URL slug generation rules for fr-DZ',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["que","qui","ce","se","il","elle","on"],"conjunction":["et","ou","mais","car"],"article":["le","la","les","un","une","el","al"],"verb":["est","sont","être","avoir"],"preposition":["de","du","des","à","au","aux","en","dans","pour","par","sur","avec","chez"]}',
    s.stopwords_count = 35,
    s.regional_additions = '[{"word":"","category":"chez","reason":"preposition"},{"word":"","category":"ben","reason":"particle"},{"word":"","category":"bou","reason":"particle"},{"word":"","category":"el","reason":"article"},{"word":"","category":"al","reason":"article"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Latin characters detected (unexpected for fr-DZ)"},{"condition":"All stopwords","message":"Title contains only stopwords - manual slug needed"}]',
    s.examples = '[{"input":"Guide Touristique de la Casbah d\'Alger","output":"guide-touristique-casbah-alger","rules_applied":["Stopwords removed (de","la","d\')","accents preserved"]},{"input":"Les Meilleurs Restaurants à Oran","output":"meilleurs-restaurants-oran","rules_applied":["Stopwords removed (les","à)","lowercase"]},{"input":"Recette du Couscous Algérois Traditionnel","output":"recette-couscous-algérois-traditionnel","rules_applied":["Stopwords removed (du)","accents preserved"]},{"input":"Patrimoine Culturel de Constantine","output":"patrimoine-culturel-constantine","rules_applied":["Stopwords removed (de)","accents preserved"]},{"input":"Voyage en Kabylie : Montagnes et Villages","output":"voyage-kabylie-montagnes-villages","rules_applied":["Stopwords removed (en","et)","punctuation removed"]},{"input":"Top 10 Plages à Visiter sur la Côte Algérienne","output":"top-10-plages-visiter-côte-algérienne","rules_applied":["Numbers preserved","stopwords removed (à","sur","la)"]},{"input":"L\'Histoire de l\'Algérie Indépendante : De 1962 à Aujourd\'hui avec les Grands Événements Politiques et Sociaux","output":"histoire-algérie-indépendante-1962-aujourdhui-grands-événements-politiques","rules_applied":["Stopwords removed","truncated at 80 chars"]},{"input":"Architecture & Design : Les Maisons Traditionnelles d\'Algérie","output":"architecture-design-maisons-traditionnelles-algérie","rules_applied":["Ampersand removed","stopwords removed (les","d\')"]},{"input":"\"La Vie Quotidienne\" : Traditions et Modernité à Alger","output":"vie-quotidienne-traditions-modernité-alger","rules_applied":["Quotes removed","stopwords removed (la","et","à)"]},{"input":"Œuvres d\'Art de l\'École d\'Alger : Héritage Franco-Algérien","output":"œuvres-art-école-alger-héritage-franco-algérien","rules_applied":["Ligature œ preserved","stopwords removed (d\'","de","l\')"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-DZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-FR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'French (France) Slugification',
    s.content = 'URL slug generation rules for fr-FR',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["et","ou","mais","car"],"preposition":["de","du","des","à","au","aux","en","dans","pour","par","sur","avec"],"article":["le","la","les","un","une"],"verb":["est","sont","être","avoir"],"pronoun":["que","qui","ce","se","il","elle","on"]}',
    s.stopwords_count = 32,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Latin characters detected (unexpected for fr-FR)"},{"condition":"All stopwords","message":"Title contains only stopwords - manual slug needed"}]',
    s.examples = '[{"input":"Meilleurs Cafés de Paris","output":"meilleurs-cafés-paris","rules_applied":["Stopwords removed (de)","accents preserved"]},{"input":"Guide du Château de Versailles","output":"guide-château-versailles","rules_applied":["Stopwords removed (du","de)","circumflex preserved"]},{"input":"Comment Choisir un Bon Vin ?","output":"comment-choisir-bon-vin","rules_applied":["Stopwords removed (un)","punctuation removed"]},{"input":"L\'Élégance à la Française","output":"élégance-française","rules_applied":["Apostrophe removed","stopwords removed (à","la)","accents kept"]},{"input":"Top 10 Restaurants Étoilés Michelin","output":"top-10-restaurants-étoilés-michelin","rules_applied":["Numbers preserved","accents preserved"]},{"input":"Recette Traditionnelle : Bœuf Bourguignon & Légumes","output":"recette-traditionnelle-bœuf-bourguignon-légumes","rules_applied":["Colon/ampersand removed","ligature œ preserved"]},{"input":"Les Plus Beaux Villages de France avec Photos et Conseils Pratiques pour Visiter","output":"plus-beaux-villages-france-photos-conseils-pratiques-visiter","rules_applied":["Stopwords removed (les","de","avec","et","pour)","truncated at 80 chars"]},{"input":"Mode & Beauté - Tendances Automne-Hiver 2025","output":"mode-beauté-tendances-automne-hiver-2025","rules_applied":["Special chars removed","accents preserved","date kept"]},{"input":"\"La Vie Est Belle\" : Citations et Réflexions","output":"vie-belle-citations-réflexions","rules_applied":["Quotes removed","stopwords removed (la","est","et)"]},{"input":"Œuvres d\'Art Contemporain : Où Trouver ?","output":"œuvres-art-contemporain-trouver","rules_applied":["Ligature œ preserved","stopwords removed (d\'","où)","punctuation removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-FR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-LU'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (LU) Slugification',
    s.content = 'URL slug generation rules for fr-LU',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["de","du","à","au","aux","en","dans","pour","sur","avec"],"article":["le","la","les","un","une","des"],"conjunction":["et","ou"]}',
    s.stopwords_count = 18,
    s.regional_additions = '[{"word":"","category":"moien","reason":"greeting"},{"word":"","category":"äddi","reason":"greeting"},{"word":"","category":"frontalier","reason":"noun"},{"word":"","category":"frontaliers","reason":"noun"},{"word":"","category":"grenzgänger","reason":"noun"},{"word":"","category":"cns","reason":"acronym"},{"word":"","category":"adem","reason":"acronym"},{"word":"","category":"cssf","reason":"acronym"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for Luxembourg SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Latin characters detected (unexpected for fr-LU)"},{"condition":"All stopwords","message":"Title contains only stopwords - manual slug needed"},{"condition":"Trilingual titles","message":"Consider separate FR/DE/LU versions for Luxembourg market"}]',
    s.examples = '[{"input":"Appartements à Louer à Kirchberg","output":"appartements-louer-kirchberg","rules_applied":["Stopwords removed (à)","Luxembourg financial district preserved"]},{"input":"Guide des Restaurants au Grund","output":"guide-restaurants-grund","rules_applied":["Stopwords removed (des","au)","historic quarter preserved"]},{"input":"Emploi Frontalier Luxembourg-Ville","output":"emploi-luxembourg-ville","rules_applied":["Frontalier stopword removed","capital city preserved"]},{"input":"La Schueberfouer 2025 : Foire du Glacis","output":"schueberfouer-2025-foire-glacis","rules_applied":["Stopwords removed (la","du)","traditional fair preserved"]},{"input":"POST Luxembourg vs Tango : Comparer les Forfaits","output":"post-luxembourg-tango-comparer-forfaits","rules_applied":["Stopwords removed (vs","les)","telecom brands preserved"]},{"input":"Top 10 Entreprises Fintech au Kirchberg","output":"top-10-entreprises-fintech-kirchberg","rules_applied":["Numbers preserved","stopwords removed (au)","financial hub preserved"]},{"input":"Spuerkeess et BGL BNP Paribas : Comparer les Banques Luxembourgeoises pour Particuliers et Entreprises","output":"spuerkeess-bgl-bnp-paribas-comparer-banques-luxembourgeoises-particuliers","rules_applied":["Stopwords removed","truncated at 80 chars","bank names preserved"]},{"input":"Fête Nationale du 23 Juin : Défilé & Feu d\'Artifice au Pont Adolphe","output":"fête-nationale-23-juin-défilé-feu-artifice-pont-adolphe","rules_applied":["Ampersand removed","stopwords removed (du","d\'","au)","accents preserved"]},{"input":"\"Moien\" et \"Äddi\" : Apprendre le Luxembourgeois","output":"apprendre-luxembourgeois","rules_applied":["Quotes removed","Luxembourgish greetings as stopwords","stopwords removed (et","le)"]},{"input":"Vianden, Clervaux & Esch-sur-Alzette : Découvrir le Nord","output":"vianden-clervaux-esch-sur-alzette-découvrir-nord","rules_applied":["Ampersand removed","compound city name preserved","stopwords removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-LU.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-MA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (MA) Slugification',
    s.content = 'URL slug generation rules for fr-MA',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["que","qui"],"conjunction":["et","ou"],"verb":["est","sont"],"article":["le","la","les","un","une"],"preposition":["de","du","des","en","dans","pour","sur","par","avec","au","aux"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Verify intentional use"}]',
    s.examples = '[{"input":"Le Guide du Voyageur à Marrakech","output":"guide-voyageur-marrakech","rules_applied":["Stopwords (le","du","à) removed","diacritics preserved"]},{"input":"Les Meilleures Adresses de Casablanca","output":"meilleures-adresses-casablanca","rules_applied":["Stopwords (les","de) removed","lowercase"]},{"input":"Découverte de la Médina de Fès","output":"découverte-médina-fès","rules_applied":["Diacritics (é","è) preserved","stopwords removed"]},{"input":"Café Traditionnel au Maroc","output":"café-traditionnel-maroc","rules_applied":["Stopword (au) removed","é preserved"]},{"input":"Art et Culture dans le Royaume","output":"art-culture-royaume","rules_applied":["Stopwords (et","dans","le) removed"]},{"input":"10 Conseils pour Visiter Rabat","output":"10-conseils-visiter-rabat","rules_applied":["Number preserved","stopword (pour) removed"]},{"input":"Comment Préparer un Tajine Authentique avec les Épices Traditionnelles du Maroc","output":"comment-préparer-tajine-authentique-épices-traditionnelles-maroc","rules_applied":["Truncated at 80 chars","stopwords removed","diacritics preserved"]},{"input":"L\'Hôtel & Spa de la Palmeraie!","output":"lhôtel-spa-palmeraie","rules_applied":["Apostrophe removed","& removed","! removed","stopwords removed"]},{"input":"\"Bienvenue\" au Riad de l\'Étoile","output":"bienvenue-riad-létoile","rules_applied":["Quotes removed","stopwords (au","de) removed","diacritics preserved"]},{"input":"Où Manger à Tanger ?","output":"où-manger-tanger","rules_applied":["Question mark removed","stopword (à) removed","ù preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-MA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-MG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (MG) Slugification',
    s.content = 'URL slug generation rules for fr-MG',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["de","du","a","au","aux","en","pour","dans","sur","avec","amin","ho"],"article":["le","la","les","un","une","des","ny"],"conjunction":["et","ou","sy","ary"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[{"word":"","category":"ny","reason":"article"},{"word":"","category":"sy","reason":"conjunction"},{"word":"","category":"ary","reason":"conjunction"},{"word":"","category":"dia","reason":"connector"},{"word":"","category":"izay","reason":"relative"},{"word":"","category":"amin","reason":"preposition"},{"word":"","category":"ho","reason":"preposition"},{"word":"","category":"eto","reason":"locative"},{"word":"","category":"any","reason":"locative"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Latin characters detected (unexpected for fr-MG)"},{"condition":"All stopwords","message":"Title contains only stopwords - manual slug needed"}]',
    s.examples = '[{"input":"Emploi Antananarivo : Offres Telma sy Orange Madagascar","output":"emploi-antananarivo-offres-telma-orange-madagascar","rules_applied":["Punctuation removed","stopword removed (sy)","telecom brands preserved"]},{"input":"Guide Ariary MGA : Taux Change BNI Madagascar Jovenna","output":"guide-ariary-mga-taux-change-bni-madagascar-jovenna","rules_applied":["Currency code preserved","local bank and fuel brand kept"]},{"input":"Recette Ravitoto ny Romazava Traditionnel Malagasy","output":"recette-ravitoto-romazava-traditionnel-malagasy","rules_applied":["Stopword removed (ny)","traditional dish names preserved"]},{"input":"Appartement a Louer Tana Quartier Analakely","output":"appartement-louer-tana-quartier-analakely","rules_applied":["Stopword removed (a)","neighborhood name preserved"]},{"input":"Festival Hiragasy Famadihana Alahamady Be Antsirabe 2026","output":"festival-hiragasy-famadihana-alahamady-be-antsirabe-2026","rules_applied":["Cultural events preserved","date kept","traditional celebrations"]},{"input":"Top 10 Hotels Nosy Be avec Vue Mer Emeraude","output":"top-10-hotels-nosy-be-vue-mer-emeraude","rules_applied":["Number preserved","stopword removed (avec)","island name kept"]},{"input":"Telma Mobile Money Orange Madagascar Airtel : Services Bancaires Socimad BFV-SG Fianarantsoa Toamasina","output":"telma-mobile-money-orange-madagascar-airtel-services-bancaires-socimad-bfv-sg","rules_applied":["Truncated at 80 chars","telecom and bank brands preserved"]},{"input":"Visa Madagascar & Formalites : Ambassade Diego-Suarez - Ile Sainte-Marie","output":"visa-madagascar-formalités-ambassade-diego-suarez-île-sainte-marie","rules_applied":["Ampersand/colon/dash removed","city names preserved","hyphenated names kept"]},{"input":"\"Mora Mora\" : Expressions Vazaha Tompoko Misaotra Madagascar","output":"mora-mora-expressions-vazaha-tompoko-misaotra-madagascar","rules_applied":["Quotes removed","Malagasy loanwords preserved (mora mora","vazaha","tompoko)"]},{"input":"L\'Economie Zebu Koba Vary Amin\'Anana : Agriculture Mahajanga Province Diana","output":"économie-zébu-koba-vary-aminanana-agriculture-mahajanga-province-diana","rules_applied":["Apostrophe removed","local products (zebu","koba","vary) and regions preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-MG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-RW'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (RW) Slugification',
    s.content = 'URL slug generation rules for fr-RW',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["est","sont"],"conjunction":["et","ou","mais","donc","na"],"pronoun":["que","qui","ce"],"preposition":["de","du","des","au","aux","en","pour","avec","sur","dans","muri","kuri"],"adverb":["aussi","ici"],"article":["le","la","les","un","une"]}',
    s.stopwords_count = 29,
    s.regional_additions = '[{"word":"","category":"muri","reason":"preposition"},{"word":"","category":"na","reason":"conjunction"},{"word":"","category":"kuri","reason":"preposition"},{"word":"","category":"aussi","reason":"adverb"},{"word":"","category":"ici","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Verify intentional mixing"}]',
    s.examples = '[{"input":"Guide des Parcs Nationaux du Rwanda","output":"guide-parcs-nationaux-rwanda","rules_applied":["Stopwords (des","du) removed","lowercase","hyphens"]},{"input":"L\'Hôtel des Mille Collines à Kigali","output":"lhôtel-mille-collines-kigali","rules_applied":["Apostrophe removed","stopwords (des","à) removed","diacritics preserved"]},{"input":"Économie et Développement au Rwanda","output":"économie-développement-rwanda","rules_applied":["Stopwords (et","au) removed","accents preserved"]},{"input":"Café Bourbon du Lac Kivu","output":"café-bourbon-lac-kivu","rules_applied":["Stopwords (du) removed","accent on café preserved"]},{"input":"Fête de l\'Indépendance Rwandaise","output":"fête-lindépendance-rwandaise","rules_applied":["Article contracted","stopwords removed"]},{"input":"Top 10 des Attractions à Visiter en 2025","output":"top-10-attractions-visiter-2025","rules_applied":["Numbers preserved","stopwords (des","à","en) removed"]},{"input":"Les Meilleurs Restaurants Gastronomiques de Kigali pour Célébrer une Occasion Spéciale","output":"meilleurs-restaurants-gastronomiques-kigali-célébrer-occasion-spéciale","rules_applied":["Truncated at word boundary before 80 chars","stopwords removed"]},{"input":"Musée & Mémorial du Génocide : Histoire et Réconciliation","output":"musée-mémorial-génocide-histoire-réconciliation","rules_applied":["Special chars (& :) removed","stopwords (du","et) removed"]},{"input":"\"Les Tambours\" : Patrimoine Culturel du Rwanda","output":"tambours-patrimoine-culturel-rwanda","rules_applied":["Quotes removed","stopwords (les","du) removed"]},{"input":"Irembo : Plateforme Électronique des Services Gouvernementaux","output":"irembo-plateforme-électronique-services-gouvernementaux","rules_applied":["Kinyarwanda term preserved","colon removed","stopwords (des) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-RW.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-SN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (SN) Slugification',
    s.content = 'URL slug generation rules for fr-SN',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["et","ou"],"preposition":["de","du","à","au","aux","en","pour","dans","sur","avec"],"filler":["quoi"],"interjection":["nak","bon","waw"],"article":["le","la","les","un","une","des"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[{"word":"","category":"nak","reason":"interjection"},{"word":"","category":"quoi","reason":"filler"},{"word":"","category":"bon","reason":"interjection"},{"word":"","category":"waw","reason":"interjection"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO effectiveness"},{"condition":"Slug > 60 chars","message":"Consider shortening for better readability"},{"condition":"Mixed scripts detected","message":"Verify intentional use of multiple scripts"}]',
    s.examples = '[{"input":"Emploi à Dakar pour les jeunes diplômés","output":"emploi-dakar-jeunes-diplômés","rules_applied":["stopwords (à","pour","les)","preserve accents"]},{"input":"Appartement à louer au Plateau Dakar","output":"appartement-louer-plateau-dakar","rules_applied":["stopwords (à","au)","preserve location names"]},{"input":"Guide touristique de l\'île de Gorée","output":"guide-touristique-île-gorée","rules_applied":["stopwords (de","l\')","preserve accent on île"]},{"input":"Le Grand Magal de Touba 2025","output":"grand-magal-touba-2025","rules_applied":["stopwords (le","de)","keep numbers"]},{"input":"La teranga sénégalaise : hospitalité et culture","output":"teranga-sénégalaise-hospitalité-culture","rules_applied":["stopwords (la","et)","remove colon","Wolof term preserved"]},{"input":"Top 10 restaurants thiéboudienne à Dakar","output":"top-10-restaurants-thiéboudienne-dakar","rules_applied":["stopword (à)","numbers preserved","Wolof dish name"]},{"input":"Saint-Louis du Sénégal : patrimoine mondial de l\'UNESCO et histoire coloniale française au cœur de l\'Afrique","output":"saint-louis-sénégal-patrimoine-mondial-unesco-histoire-coloniale-française-cœur","rules_applied":["truncate at 80 chars","stopwords removed"]},{"input":"Concert de mbalax : Youssou N\'Dour & Orchestra Baobab !","output":"concert-mbalax-youssou-ndour-orchestra-baobab","rules_applied":["remove punctuation and ampersand"]},{"input":"L\'économie du Sénégal : pétrole et gaz","output":"économie-sénégal-pétrole-gaz","rules_applied":["remove quotes","preserve diacritics"]},{"input":"Œuvres d\'art du Lac Rose et de la Petite Côte","output":"œuvres-art-lac-rose-petite-côte","rules_applied":["preserve ligature œ","stopwords (du","de","la","et)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-SN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'fr-TN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'FR (TN) Slugification',
    s.content = 'URL slug generation rules for fr-TN',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"abbreviation":["tn"],"conjunction":["et","ou"],"pronoun":["que","qui"],"preposition":["de","du","à","au","aux","en","pour","dans","sur","avec","par"],"article":["le","la","les","un","une","des"]}',
    s.stopwords_count = 22,
    s.regional_additions = '[{"word":"","category":"tn","reason":"abbreviation"},{"word":"","category":"tunisie","reason":"geonym"},{"word":"","category":"tunisien","reason":"adjective"},{"word":"","category":"tunisienne","reason":"adjective"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Trop court pour referencement"},{"condition":"Slug > 60 chars","message":"Envisager raccourcissement"},{"condition":"Mixed scripts","message":"Verifier usage intentionnel"}]',
    s.examples = '[{"input":"Vacances à Djerba en Famille","output":"vacances-djerba-famille","rules_applied":["Stopwords (à","en) removed","lowercase"]},{"input":"Emploi et Recrutement à Tunis","output":"emploi-recrutement-tunis","rules_applied":["Stopwords (et","à) removed"]},{"input":"Appartement à Louer aux Berges du Lac","output":"appartement-louer-berges-lac","rules_applied":["Stopwords (à","aux","du) removed"]},{"input":"Festival International de Carthage 2025","output":"festival-international-carthage-2025","rules_applied":["Stopword (de) removed","year preserved"]},{"input":"Guide Touristique de Sousse et Monastir","output":"guide-touristique-sousse-monastir","rules_applied":["Stopwords (de","et) removed"]},{"input":"10 Hôtels à Hammamet avec Piscine","output":"10-hôtels-hammamet-piscine","rules_applied":["Number preserved","stopwords (à","avec) removed","accent ô preserved"]},{"input":"Recette Traditionnelle du Lablabi à la Harissa avec les Secrets de Préparation de Grand-Mère","output":"recette-traditionnelle-lablabi-harissa-secrets-préparation-grand-mère","rules_applied":["Truncated 80 chars","stopwords removed"]},{"input":"Brik à l\'Oeuf & Ojja : Spécialités de Sfax !","output":"brik-loeuf-ojja-spécialités-sfax","rules_applied":["Punctuation removed","stopwords (à","de) removed"]},{"input":"\"Sahha\" : Le Guide des Expressions à Sidi Bou Saïd","output":"sahha-guide-expressions-sidi-bou-saïd","rules_applied":["Quotes removed","stopwords (le","des","à) removed","trema ï preserved"]},{"input":"Hammam Traditionnel à Kairouan : Où Aller ?","output":"hammam-traditionnel-kairouan-aller","rules_applied":["Punctuation removed","stopwords (à","où) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/fr-TN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ga-IE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'GA (IE) Slugification',
    s.content = 'URL slug generation rules for ga-IE',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["ag","ar","le","i","do","de","as","faoi"],"pronoun":["se","si"],"conjunction":["agus","ach","mar","no"],"verb":["is","ta"],"article":["an","na"],"demonstrative":["seo"]}',
    s.stopwords_count = 19,
    s.regional_additions = '[{"word":"","category":"go","reason":"particle"},{"word":"","category":"ni","reason":"particle"},{"word":"","category":"as","reason":"preposition"},{"word":"","category":"faoi","reason":"preposition"},{"word":"","category":"mar","reason":"conjunction"},{"word":"","category":"no","reason":"conjunction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Check for unintended script mixing"}]',
    s.examples = '[{"input":"An Ghaeilge sa Lá Inniu","output":"ghaeilge-lá-inniu","rules_applied":["stopwords(an","sa)","lowercase","fada preserved"]},{"input":"Ceol Traidisiunta na hEireann","output":"ceol-traidisiunta-heireann","rules_applied":["stopwords(na)","lowercase","fada preserved"]},{"input":"Stair agus Cultúr","output":"stair-cultúr","rules_applied":["stopwords(agus)","lowercase","fada preserved"]},{"input":"Oideachas i mBaile Átha Cliath","output":"oideachas-mbaile-átha-cliath","rules_applied":["stopwords(i)","lowercase","fada preserved"]},{"input":"Litriocht Ghaeilge an Fhichiú hAois","output":"litriocht-ghaeilge-fhichiú-haois","rules_applied":["stopwords(an)","lowercase","fada preserved"]},{"input":"10 mBealach le Gaeilge a Fhoghlaim","output":"10-mbealach-gaeilge-fhoghlaim","rules_applied":["stopwords(le","a)","lowercase","numbers kept"]},{"input":"Tuairisc ar Fhorbairt Eacnamaioch agus Shoisialach na hEireann le Deich mBliana Anuas","output":"tuairisc-fhorbairt-eacnamaioch-shoisialach-heireann-deich-mbliana-anuas","rules_applied":["truncated to 80 chars","stopwords removed"]},{"input":"Coras Iompair Eireann: Bealai Nua!","output":"córas-iompair-éireann-bealaí-nua","rules_applied":["punctuation removed","fada preserved"]},{"input":"\"An Bealach Ceart\" don Todhchai","output":"bealach-ceart-todhchaí","rules_applied":["quotes removed","stopwords(an","don)","fada preserved"]},{"input":"Urlabhrai & Teangacha Eile","output":"urlabhraí-teangacha-eile","rules_applied":["ampersand removed","fada preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ga-IE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'gl-ES'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'GL (ES) Slugification',
    s.content = 'URL slug generation rules for gl-ES',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["e","ou","nin","mais","pero","que","como"],"verb":["e","ser","estar","ter"],"article":["o","a","os","as","un","unha","uns","unhas"],"pronoun":["este","esta","ese","esa","aquel","aquela"],"preposition":["de","en","con","para","por","sobre","entre","sen","ata","desde"]}',
    s.stopwords_count = 35,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Latin with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"Guia para Santiago de Compostela","output":"guia-santiago-compostela","rules_applied":["Stopwords removed (para","de)","lowercase","hyphenation"]},{"input":"A cociña galega tradicional","output":"cociña-galega-tradicional","rules_applied":["Article removed (a)","lowercase","n preserved"]},{"input":"Os mellores restaurantes de Vigo","output":"mellores-restaurantes-vigo","rules_applied":["Article removed (os)","preposition removed (de)","lowercase"]},{"input":"Consellos para viaxar a Galicia","output":"consellos-viaxar-galicia","rules_applied":["Prepositions removed (para","a)","lowercase"]},{"input":"Historia e cultura de Ourense","output":"historia-cultura-ourense","rules_applied":["Conjunction removed (e)","preposition removed (de)","lowercase"]},{"input":"10 lugares para visitar na Coruna","output":"10-lugares-visitar-coruña","rules_applied":["Number preserved","stopwords removed (para","na)","n preserved"]},{"input":"A arquitectura romanica en Galicia: unha guia completa sobre as igrexas e mosteiros da rexion durante a Idade Media","output":"arquitectura-romanica-galicia-guia-completa-igrexas-mosteiros-rexion-idade-media","rules_applied":["Truncated to 80 chars","stopwords removed (a","en","unha","sobre","as","e","da","durante)"]},{"input":"Festas & tradicións: celebracións populares en Galicia!","output":"festas-tradicións-celebracións-populares-galicia","rules_applied":["Special chars removed (&",":","!)","stopword removed (en)","accents preserved"]},{"input":"Viños \"da terra\" ou embotellados?","output":"viños-terra-embotellados","rules_applied":["Quotes removed","stopwords (da","ou) removed","question mark removed","n preserved"]},{"input":"Anos difíciles: a crise económica e o futuro","output":"anos-difíciles-crise-económica-futuro","rules_applied":["Colon removed","stopwords removed (a","e","o)","accents preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/gl-ES.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'gn-PY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'GN (PY) Slugification',
    s.content = 'URL slug generation rules for gn-PY',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"adverb":["avei","voi","katu"]}',
    s.stopwords_count = 3,
    s.regional_additions = '[{"word":"","category":"avei","reason":"adverb"},{"word":"","category":"voi","reason":"adverb"},{"word":"","category":"katu","reason":"adverb"},{"word":"","category":"ningo","reason":"particle"},{"word":"","category":"nte","reason":"particle"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed","message":"> 60% content stripped"},{"condition":"All nasal vowels stripped","message":"Verify meaning preserved"}]',
    s.examples = '[{"input":"Mba\'eichapa reikuaa Guarani","output":"mbaeichapa-reikuaa-guarani","rules_applied":["Puso removed","nasal stripped"]},{"input":"Tembi\'u Paraguay pegua","output":"tembiu-paraguay-pegua","rules_applied":["Puso removed","postposition kept (semantic)"]},{"input":"Tetanguera ha tekohakuera","output":"tetanguera-tekohakuera","rules_applied":["Conjunction \"ha\" removed"]},{"input":"Yvytu pyahu ko arape","output":"yvytu-pyahu-arape","rules_applied":["Demonstrative \"ko\" removed"]},{"input":"Mombe\'u ore retame","output":"mombeu-retame","rules_applied":["Puso removed","pronoun \"ore\" removed"]},{"input":"10 mba\'e iporague Paraguaipe","output":"10-mbaeiporague-paraguaipe","rules_applied":["Numbers preserved","apostrophe removed"]},{"input":"Opaichagua tembi\'u iporague ha ivaiva tekotevehape Paraguai retame oikova\'eraguape","output":"opaichagua-tembiu-iporague-ivaiva-tekotevehape-paraguai-retame-oikova","rules_applied":["Long title truncated at 80 chars"]},{"input":"Mba\'e: tembi\'urape!","output":"mbaetembiu-rape","rules_applied":["Punctuation removed","puso stripped"]},{"input":"\"Che Retaguype\" niko","output":"retaguype-niko","rules_applied":["Quotes removed","pronoun \"che\" removed"]},{"input":"Ypykue ha Yvyra\'ija","output":"ypykue-yvyraija","rules_applied":["Puso removed","conjunction removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/gn-PY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'gu-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'GU (IN) Slugification',
    s.content = 'URL slug generation rules for gu-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["અને","પણ","કે","અથવા"],"quantifier":["બધા"]}',
    s.stopwords_count = 5,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Gujarati with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"},{"condition":"Broken conjuncts","message":"Virama should form proper conjuncts"}]',
    s.examples = '[{"input":"ગુજરાતનો ઇતિહાસ","output":"ગુજરાતનો-ઇતિહાસ","rules_applied":["Hyphenation applied"]},{"input":"અમદાવાદમાં શ્રેષ્ઠ રેસ્ટોરન્ટ","output":"અમદાવાદ-શ્રેષ્ઠ-રેસ્ટોરન્ટ","rules_applied":["Postposition (માં) removed","hyphenation"]},{"input":"ગાંધીનગર અને સુરત","output":"ગાંધીનગર-સુરત","rules_applied":["Conjunction (અને) removed","hyphenation"]},{"input":"કચ્છનું રણ વિશે માહિતી","output":"કચ્છ-રણ-માહિતી","rules_applied":["Possessive (નું) and postposition (વિશે) removed"]},{"input":"ગુજરાતી સાહિત્ય એક પરિચય","output":"ગુજરાતી-સાહિત્ય-પરિચય","rules_applied":["Article-like numeral (એક) removed"]},{"input":"2024માં ગુજરાત પ્રવાસન","output":"2024-ગુજરાત-પ્રવાસન","rules_applied":["Number preserved","postposition (માં) removed"]},{"input":"સોમનાથ મંદિર અને દ્વારકા તીર્થ: ગુજરાતનાં પવિત્ર સ્થળો વિશે સંપૂર્ણ માહિતી","output":"સોમનાથ-મંદિર-દ્વારકા-તીર્થ-ગુજરાત-પવિત્ર-સ્થળો-સંપૂર્ણ-માહિતી","rules_applied":["Truncated to 80 chars","stopwords and special chars removed"]},{"input":"ખાખરા, ઢોકળા & ફાફડા: ગુજરાતી નાસ્તો!","output":"ખાખરા-ઢોકળા-ફાફડા-ગુજરાતી-નાસ્તો","rules_applied":["Special chars removed (&",":","!)","hyphenation"]},{"input":"\"ગરબા\" અને \"રાસ\" ગુજરાતની સંસ્કૃતિ","output":"ગરબા-રાસ-ગુજરાત-સંસ્કૃતિ","rules_applied":["Quotes removed","conjunction (અને) removed","possessive (ની) removed"]},{"input":"ગીરનું જંગલ vs સાસણ: સિંહ સફારી માટે કયું છે શ્રેષ્ઠ","output":"ગીર-જંગલ-સાસણ-સિંહ-સફારી-કયું-શ્રેષ્ઠ","rules_applied":["vs and colon removed","stopwords (નું","માટે","છે) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/gu-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ha-NG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'HA (NG) Slugification',
    s.content = 'URL slug generation rules for ha-NG',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["ko","kuma","amma"],"negation":["ba"],"preposition":["a","cikin","daga","kan","zuwa","don"],"adverb":["sai"],"pronoun":["shi"]}',
    s.stopwords_count = 12,
    s.regional_additions = '[{"word":"","category":"kuma","reason":"conjunction"},{"word":"","category":"amma","reason":"conjunction"},{"word":"","category":"sai","reason":"adverb"},{"word":"","category":"shi","reason":"pronoun"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"All stopwords removed","message":"Slug may be too generic"}]',
    s.examples = '[{"input":"Yadda ake sayen mota a Najeriya","output":"yadda-ake-sayen-mota-najeriya","rules_applied":["lowercase","stopword (a) removed","spaces to hyphens"]},{"input":"Manyan Gidajen Cin Abinci na Lagos","output":"manyan-gidajen-cin-abinci-lagos","rules_applied":["lowercase","stopword (na) removed"]},{"input":"Wasanni 10 da Yara Suke So","output":"wasanni-10-yara-suke","rules_applied":["lowercase","stopwords (da) removed","numbers kept"]},{"input":"Ƙarshen Shekara: Abubuwan da Za a Yi","output":"karshen-shekara-abubuwan-za-yi","rules_applied":["hooked k converted","stopwords removed"]},{"input":"Me Ya Sa Mutane Suke Tafiya Abuja?","output":"me-sa-mutane-suke-tafiya-abuja","rules_applied":["lowercase","punctuation removed"]},{"input":"Littattafan Hausa 5 Mafi Kyau na 2025","output":"littattafan-hausa-5-mafi-kyau-2025","rules_applied":["stopword (na) removed","numbers kept"]},{"input":"Yadda Za Ku Kula da Lafiyar Jikinku da Kuma Abincin da Kuke Ci a Kowace Rana","output":"yadda-za-ku-kula-lafiyar-jikinku-abincin-kuke-ci-kowace-rana","rules_applied":["truncated at 80 chars","stopwords removed"]},{"input":"Labarai, Wasanni & Nishaɗi: Duk a Wuri Ɗaya!","output":"labarai-wasanni-nishadi-duk-wuri-daya","rules_applied":["hooked d converted","ampersand/punctuation removed"]},{"input":"\"Babban Labari\" na Wannan Mako","output":"babban-labari-wannan-mako","rules_applied":["quotes removed","stopword (na) removed"]},{"input":"\'Yan Sanda Sun Kama Ɓarayi a Kano","output":"yan-sanda-sun-kama-barayi-kano","rules_applied":["apostrophe removed","hooked b converted","stopword (a) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ha-NG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'he-IL'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Hebrew (Israel) Slugification',
    s.content = 'URL slug generation rules for he-IL',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["אך"],"adverb":["גם","רק"]}',
    s.stopwords_count = 3,
    s.regional_additions = '[{"word":"","category":"יש","reason":"existential"},{"word":"","category":"אין","reason":"negative existential"},{"word":"","category":"כל","reason":"determiner"},{"word":"","category":"גם","reason":"adverb"},{"word":"","category":"אך","reason":"conjunction"},{"word":"","category":"רק","reason":"adverb"}]',
    s.script_config = '{"primary_script":"hebrew","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Hebrew with Latin unless technical terms"},{"condition":"Nikud present","message":"Consider removing vowel points for cleaner URLs"}]',
    s.examples = '[{"input":"מדריך למשתמש החדש","output":"מדריך-משתמש-חדש","rules_applied":["Stopwords removed (ל","ה)","spaces to hyphens"]},{"input":"איך לבנות אתר אינטרנט","output":"איך-לבנות-אתר-אינטרנט","rules_applied":["Preserved technical term","removed stopwords"]},{"input":"חדשות ישראל 2025","output":"חדשות-ישראל-2025","rules_applied":["Numbers preserved","spaces to hyphens"]},{"input":"תל אביב - העיר שלא ישנה","output":"תל-אביב-עיר-שלא-ישנה","rules_applied":["Dash removed","stopwords (ה","שלא) handled"]},{"input":"המדריך השלם לטיולים בארץ","output":"המדריך-השלם-לטיולים-בארץ","rules_applied":["Definite article ה preserved in compound words"]},{"input":"10 טיפים לחיסכון בכסף","output":"10-טיפים-לחיסכון-בכסף","rules_applied":["Number preserved","prepositions (ל","ב) kept in compounds"]},{"input":"מה ההבדל בין פייסבוק לטוויטר והאם כדאי להשתמש בשניהם או רק באחד מהם?","output":"מה-ההבדל-בין-פייסבוק-לטוויטר-והאם-כדאי-להשתמש-בשניהם-או-רק","rules_applied":["Long title truncated at 80 chars","stopwords removed"]},{"input":"\"ירושלים של זהב\" - שיר ישראלי מפורסם","output":"ירושלים-זהב-שיר-ישראלי-מפורסם","rules_applied":["Quotes removed","stopwords removed (של)"]},{"input":"חג הפסח: מנהגים ומסורות","output":"חג-הפסח-מנהגים-ומסורות","rules_applied":["Colon removed","conjunction ו in ומסורות kept"]},{"input":"א״ב של קידום אתרים","output":"אב-קידום-אתרים","rules_applied":["Abbreviation marks removed","stopwords של removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/he-IL.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'hi-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Hindi (India) Slugification',
    s.content = 'URL slug generation rules for hi-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"preposition":["साथ"],"honorific":["जी"]}',
    s.stopwords_count = 2,
    s.regional_additions = '[{"word":"","category":"जी","reason":"honorific"},{"word":"","category":"जैसे","reason":"comparative"},{"word":"","category":"साथ","reason":"preposition"},{"word":"","category":"लिए","reason":"postposition"},{"word":"","category":"द्वारा","reason":"postposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Devanagari + Latin mixed (acceptable for technical terms)"},{"condition":"Non-standard composition","message":"Invalid Unicode composition detected"}]',
    s.examples = '[{"input":"भारत में यात्रा गाइड","output":"भारत-यात्रा-गाइड","rules_applied":["Stopwords (में) removed"]},{"input":"हिन्दी सीखने के लिए टिप्स","output":"हिन्दी-सीखने-टिप्स","rules_applied":["Stopwords (के","लिए) removed"]},{"input":"दिल्ली का इतिहास और संस्कृति","output":"दिल्ली-इतिहास-संस्कृति","rules_applied":["Stopwords (का","और) removed"]},{"input":"मुंबई की सर्वश्रेष्ठ रेस्टोरेंट","output":"मुंबई-सर्वश्रेष्ठ-रेस्टोरेंट","rules_applied":["Stopwords (की) removed"]},{"input":"भारतीय व्यंजन रेसिपी","output":"भारतीय-व्यंजन-रेसिपी","rules_applied":["No stopwords","clean slug"]},{"input":"तकनीक समाचार 2025","output":"तकनीक-समाचार-2025","rules_applied":["Numbers preserved"]},{"input":"यह है भारत का सबसे बड़ा मंदिर जो विश्व में प्रसिद्ध है","output":"यह-भारत-सबसे-बड़-मंदिर-जो-विश्व-प्रसिद्ध","rules_applied":["Long title truncated","stopwords (है","का","में","है) removed"]},{"input":"स्वास्थ्य और फिटनेस: व्यायाम टिप्स!","output":"स्वास्थ्य-फिटनेस-व्यायाम-टिप्स","rules_applied":["Punctuation removed","stopwords (और) removed"]},{"input":"\"भारत की आवाज़\" - रेडियो कार्यक्रम","output":"भारत-आवाज़-रेडियो-कार्यक्रम","rules_applied":["Quotes removed","stopwords (की) removed"]},{"input":"टेक्नोलॉजी & Innovation हब","output":"टेक्नोलॉजी-innovation-हब","rules_applied":["Ampersand removed","mixed Devanagari-Latin accepted"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/hi-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'hr-HR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Croatian (Croatia) Slugification',
    s.content = 'URL slug generation rules for hr-HR',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["taj","ta","to","koji","koja","koje","ovaj","ova","ovo"],"adverb":["kao"],"preposition":["u","na","za","s","sa","od","do","iz","o","kroz","prema","tijekom","nakon","prije"],"conjunction":["i","a","ali","ili","pa","te","ni","niti"],"verb":["je","su","biti"]}',
    s.stopwords_count = 35,
    s.regional_additions = '[{"word":"","category":"kroz","reason":"preposition"},{"word":"","category":"prema","reason":"preposition"},{"word":"","category":"tijekom","reason":"preposition"},{"word":"","category":"nakon","reason":"preposition"},{"word":"","category":"prije","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for usability"},{"condition":"Mixed scripts detected","message":"Verify intentional use"},{"condition":"Missing Croatian diacritics","message":"Check if source had special chars"}]',
    s.examples = '[{"input":"Najbolji restorani u Zagrebu","output":"najbolji-restorani-zagrebu","rules_applied":["Stopword removal (u)","lowercase"]},{"input":"Kako naučiti hrvatski jezik","output":"kako-naučiti-hrvatski-jezik","rules_applied":["Diacritics preserved (č)","lowercase"]},{"input":"Čokolada i kolači za praznike","output":"čokolada-kolači-praznike","rules_applied":["Stopwords removed (i","za)","č preserved"]},{"input":"Šibenik: Turistički vodič kroz grad","output":"šibenik-turistički-vodič-grad","rules_applied":["Punctuation removed","š preserved","kroz removed"]},{"input":"Đakovački vezovi na festivalu","output":"đakovački-vezovi-festivalu","rules_applied":["Đ preserved","stopword (na) removed"]},{"input":"10 razloga za posjet Dubrovniku","output":"10-razloga-posjet-dubrovniku","rules_applied":["Numbers kept","stopword (za) removed"]},{"input":"Zašto je Hrvatska jedna od najpopularnijih turističkih destinacija u Europi","output":"zašto-hrvatska-jedna-najpopularnijih-turističkih-destinacija-europi","rules_applied":["Truncated to 80 chars","stopwords removed"]},{"input":"Plitvička jezera: Prirodne ljepote & Nacionalni park","output":"plitvička-jezera-prirodne-ljepote-nacionalni-park","rules_applied":["Ampersand removed","diacritics preserved"]},{"input":"\"Život je lijep\" - Novi hrvatski film","output":"život-lijep-novi-hrvatski-film","rules_applied":["Quotes removed","stopwords (je) removed"]},{"input":"Nogometaš Luka Modrić osvojio nagradu","output":"nogometaš-luka-modrić-osvojio-nagradu","rules_applied":["Complex diacritics (š","ć) preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/hr-HR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ht-HT'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'HT (HT) Slugification',
    s.content = 'URL slug generation rules for ht-HT',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"demonstrative":["sa"],"quantifier":["tout"],"verb":["gen"],"negation":["pa"]}',
    s.stopwords_count = 4,
    s.regional_additions = '[{"word":"","category":"ki","reason":"relative pronoun"},{"word":"","category":"sa","reason":"demonstrative"},{"word":"","category":"tout","reason":"quantifier"},{"word":"","category":"pa","reason":"negation"},{"word":"","category":"gen","reason":"verb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed","message":"More than 60% of words removed - verify content retention"},{"condition":"Consecutive vowels after stripping","message":"May create unusual letter combinations"}]',
    s.examples = '[{"input":"Koman pou fe diri ak pwa","output":"koman-fe-diri-pwa","rules_applied":["Stopwords removed (pou","ak)","lowercase"]},{"input":"Istwa peyi Ayiti","output":"istwa-peyi-ayiti","rules_applied":["Lowercase","hyphenation"]},{"input":"Tout sa ou dwe konnen sou vodou","output":"dwe-konnen-vodou","rules_applied":["Stopwords removed (tout","sa","ou","sou)"]},{"input":"Resèt pou griyot ak bannann peze","output":"reset-griyot-bannann-peze","rules_applied":["Accent stripped (e)","stopword removed (pou","ak)"]},{"input":"Kote ki gen pi bon plaj nan Ayiti","output":"kote-pi-bon-plaj-ayiti","rules_applied":["Stopwords removed (ki","gen","nan)"]},{"input":"10 bèl kote pou vizite nan Pòtoprens","output":"10-bel-kote-vizite-potoprens","rules_applied":["Number preserved","accents stripped (e","o)","stopwords removed"]},{"input":"Kilti ak tradisyon pèp ayisyen nan karayib la: tout sa ou dwe konnen sou listwa ak eritaj kiltirèl","output":"kilti-tradisyon-pep-ayisyen-karayib-dwe-konnen-listwa-eritaj-kiltirel","rules_applied":["Truncated","accents stripped","many stopwords removed"]},{"input":"Manje kreyòl: bannann, pwa, & diri!","output":"manje-kreyol-bannann-pwa-diri","rules_applied":["Special chars removed (&",":","!)","accent stripped"]},{"input":"\"Konpa\" oswa \"kompa\" - ki jan yo ekri li?","output":"konpa-kompa-jan-ekri","rules_applied":["Quotes removed","stopwords removed (oswa","ki","yo","li)"]},{"input":"Wi vs Non: 2 mo ki pi enpòtan nan lang kreyòl la","output":"wi-non-2-mo-pi-enpotan-lang-kreyol","rules_applied":["Colon removed","accents stripped (o)","stopwords removed (ki","nan","la)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ht-HT.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'hu-HU'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Hungarian (Hungary) Slugification',
    s.content = 'URL slug generation rules for hu-HU',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["és","vagy","de","hogy"]}',
    s.stopwords_count = 4,
    s.regional_additions = '[{"word":"","category":"meg","reason":"particle"},{"word":"","category":"fel","reason":"particle"},{"word":"","category":"le","reason":"particle"},{"word":"","category":"ki","reason":"particle"},{"word":"","category":"be","reason":"particle"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Non-Latin characters detected"},{"condition":"Missing diacritics","message":"Check if original text had accents"}]',
    s.examples = '[{"input":"A legjobb magyar receptek","output":"legjobb-magyar-receptek","rules_applied":["Article \"a\" removed","lowercase"]},{"input":"Útmutató az online vásárláshoz","output":"útmutató-online-vásárláshoz","rules_applied":["Article \"az\" removed","diacritics preserved"]},{"input":"Budapest és környéke 2025","output":"budapest-környéke-2025","rules_applied":["Conjunction \"és\" removed","diacritics kept","numbers preserved"]},{"input":"Hogyan működik a rendszer?","output":"hogyan-működik-rendszer","rules_applied":["Article \"a\" removed","question mark removed","ö preserved"]},{"input":"Új termék: prémium kategória","output":"új-termék-prémium-kategória","rules_applied":["Colon removed","ú and é preserved"]},{"input":"5 tipp & trükk a sikeres SEO-hoz","output":"5-tipp-trükk-sikeres-seo-hoz","rules_applied":["Ampersand removed","articles removed","ü preserved"]},{"input":"A magyar nyelv történetének rövid áttekintése és legfontosabb mérföldköveinek bemutatása","output":"magyar-nyelv-történetének-rövid-áttekintése-legfontosabb","rules_applied":["Truncated at 80 chars","articles/conjunctions removed"]},{"input":"Kézzel készített termékek!","output":"kézzel-készített-termékek","rules_applied":["Exclamation mark removed","é preserved"]},{"input":"\"Életre szóló\" megoldások","output":"életre-szóló-megoldások","rules_applied":["Quotes removed","é and ó preserved"]},{"input":"Fejlesztői dokumentáció C++-hoz","output":"fejlesztői-dokumentáció-c-hoz","rules_applied":["Special chars (++) removed","ő preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/hu-HU.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'hy-AM'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'HY (AM) Slugification',
    s.content = 'URL slug generation rules for hy-AM',
    s.slug_rule = 'native_script',
    s.stopwords = '{"article":["-delays (the, definite article suffix)"],"adverb":["delays (naev - also, too)"],"verb":["delays (e - is)"],"preposition":["delays (het - with)","-delays (its - from)","delays (depi - to, towards)","delays (mej - in)","delays (hamar - for)","delays (vra - on, upon)"],"conjunction":["delays / և (yev - and)"],"pronoun":["delays (ays - this)","delays (ayn - that)","delays (vor - which, that)","delays (na - it, he/she)"]}',
    s.stopwords_count = 14,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Armenian with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"delays delays delays (Haykakanlurererevanits)","output":"delays-delays-delays","rules_applied":["Lowercase","hyphenation"]},{"input":"delays delays delays (Lavaguyn restorannerHayastanum)","output":"delays-delays-delays","rules_applied":["Lowercase"]},{"input":"delays delays delays delays (Haykakan mshakuyt yev avanduytsner)","output":"delays-delays-delays","rules_applied":["Conjunction yev removed","lowercase"]},{"input":"delays delays (Patmutyun Hayastani)","output":"delays-delays","rules_applied":["Lowercase"]},{"input":"delays delays delays (Zbosashrjanakanughetsuyts Hayastan)","output":"delays-delays-delays","rules_applied":["Lowercase"]},{"input":"delays 2024 delays (Haykakan 2024 asparezutyun)","output":"delays-2024-delays","rules_applied":["Number preserved","lowercase"]},{"input":"delays delays delays delays delays delays delays delays delays delays delays delays delays delays (Long title about Armenian Republic history)","output":"Truncated to 80 chars","rules_applied":["Truncated at 80 chars"]},{"input":"delays, delays: delays! (Barigalust, Yerevan: Geghecik!)","output":"delays-delays-delays","rules_applied":["Punctuation removed","lowercase"]},{"input":"\"delays\" delays delays (Hayerenev patmutyun)","output":"delays-delays","rules_applied":["Quotes removed","conjunction yev removed"]},{"input":"delays-delays delays delays (Ararat-Sar yev haykakan zharangutyan)","output":"delays-delays-delays-delays","rules_applied":["Existing hyphen preserved","conjunction yev removed","lowercase"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/hy-AM.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'id-ID'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Indonesian (Indonesia) Slugification',
    s.content = 'URL slug generation rules for id-ID',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["dan","atau"],"adverb":["juga"],"preposition":["di","ke","dari","untuk","dengan","pada","dalam","oleh","sebagai","antara","tentang"]}',
    s.stopwords_count = 14,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed","message":"> 60% of words removed"}]',
    s.examples = '[{"input":"Cara Membuat Website untuk Pemula","output":"cara-membuat-website-pemula","rules_applied":["Stopwords removed: untuk"]},{"input":"Tips Meningkatkan Penjualan Online","output":"tips-meningkatkan-penjualan-online","rules_applied":["No stopwords"]},{"input":"10 Strategi Marketing Digital Terbaik","output":"10-strategi-marketing-digital-terbaik","rules_applied":["Numbers preserved"]},{"input":"Panduan Lengkap Belajar Coding dari Nol","output":"panduan-lengkap-belajar-coding-nol","rules_applied":["Stopwords removed: dari"]},{"input":"Resep Nasi Goreng Spesial & Praktis","output":"resep-nasi-goreng-spesial-praktis","rules_applied":["Special char removed: &"]},{"input":"Wisata Jakarta: 5 Tempat yang Wajib Dikunjungi","output":"wisata-jakarta-5-tempat-wajib-dikunjungi","rules_applied":["Stopwords removed: yang","Punctuation removed: :"]},{"input":"Manfaat Olahraga untuk Kesehatan Tubuh dan Pikiran yang Optimal di Masa Pandemi 2024","output":"manfaat-olahraga-kesehatan-tubuh-pikiran-optimal-masa-pandemi-2024","rules_applied":["Stopwords removed: untuk","dan","yang","di; Truncated at 80 chars"]},{"input":"Berita Terkini! Update #COVID19 @ Indonesia","output":"berita-terkini-update-covid19-indonesia","rules_applied":["Special chars removed: !","#","@"]},{"input":"\"Rahasia Sukses\" - Cara Membangun Bisnis Online","output":"rahasia-sukses-cara-membangun-bisnis-online","rules_applied":["Quotes removed: \"","-"]},{"input":"Harga HP Samsung Terbaru (Desember 2024)","output":"harga-hp-samsung-terbaru-desember-2024","rules_applied":["Parentheses removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/id-ID.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ig-NG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'IG (NG) Slugification',
    s.content = 'URL slug generation rules for ig-NG',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"demonstrative":["ahu"]}',
    s.stopwords_count = 1,
    s.regional_additions = '[{"word":"","category":"ahu","reason":"demonstrative"},{"word":"","category":"ano","reason":"locative"},{"word":"","category":"ndi","reason":"prefix/noun"},{"word":"","category":"ebe","reason":"noun/adverb"},{"word":"","category":"ihe","reason":"noun"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"> 60% stopwords removed","message":"Content may be too generic"}]',
    s.examples = '[{"input":"Otu esi emepụta azụmahịa gị","output":"otu-esi-emeputa-azumahia","rules_applied":["Diacritics stripped: ụ→u","ị→i; stopwords: gi removed"]},{"input":"Ebe kacha mma na Lagos","output":"ebe-kacha-mma-lagos","rules_applied":["Stopwords: na removed"]},{"input":"Ntụziaka maka nri ọma","output":"ntuziaka-maka-nri-oma","rules_applied":["Diacritics stripped: ụ→u","ọ→o"]},{"input":"Njem gaa Onitsha ahịa","output":"njem-onitsha-ahia","rules_applied":["Stopwords: gaa removed; diacritics: ị→i"]},{"input":"Ego na akụ nke ụlọ","output":"ego-aku-ulo","rules_applied":["Stopwords: na","nke removed; diacritics: ụ→u","ọ→o"]},{"input":"Isi ihe 10 maka nkwado 2025","output":"isi-10-maka-nkwado-2025","rules_applied":["Numbers preserved; stopwords: ihe removed"]},{"input":"Ntuziaka zuru oke maka ịhụnanya na ndụ gị na otu esi eme ka ọ dịrị mma na nchekwa ego na azụmahịa","output":"ntuziaka-zuru-oke-maka-ihunanya-ndu-otu-esi-eme-dirir-mma-nchekwa-ego-azumahia","rules_applied":["Truncated at 80 chars; multiple diacritics and stopwords processed"]},{"input":"Nollywood: Fịlm Naịjịrịa & Nkwurịta Ọkụ!","output":"nollywood-film-naiiria-nkwurita-oku","rules_applied":["Punctuation removed; diacritics stripped: ị→i","ọ→o"]},{"input":"\"Jọlọf Raịs\" - Nri Naịjịrịa","output":"jolof-rais-nri-naiiria","rules_applied":["Quotes and dash removed; diacritics: ọ→o","ị→i"]},{"input":"Bọl na Atletiks: Egwuregwu na Naịjịrịa","output":"bol-atletiks-egwuregwu-naiiria","rules_applied":["Colon removed; stopwords: na removed twice; diacritics: ọ→o","ị→i"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ig-NG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'is-IS'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'IS (IS) Slugification',
    s.content = 'URL slug generation rules for is-IS',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["það","þetta","hann","hún","við","þeir"],"verb":["er","var","vera","hefur","verður"],"conjunction":["og","eða","en","sem"],"preposition":["á","í","til","um","af","frá","með","fyrir","við","eftir"],"article":["hinn","hin","hið"]}',
    s.stopwords_count = 28,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Check for encoding issues"}]',
    s.examples = '[{"input":"Íslensk matreiðsla","output":"íslensk-matreiðsla","rules_applied":["lowercase","stopwords removed","spaces to hyphens"]},{"input":"Þjóðarsöngurinn og landslagið","output":"þjóðarsöngurinn-landslagið","rules_applied":["lowercase","stopword \"og\" removed"]},{"input":"Ferðalag um Ísland","output":"ferðalag-ísland","rules_applied":["stopwords \"um\" removed"]},{"input":"Gullfoss og Geysir","output":"gullfoss-geysir","rules_applied":["stopword \"og\" removed"]},{"input":"Björk í Reykjavík","output":"björk-reykjavík","rules_applied":["lowercase","stopword \"í\" removed"]},{"input":"10 bestu veitingastaðirnir 2025","output":"10-bestu-veitingastaðirnir-2025","rules_applied":["numbers preserved","spaces to hyphens"]},{"input":"Saga Íslands frá landnámi til nútímans og áhrif á menningu þjóðarinnar","output":"saga-íslands-landnámi-nútímans-áhrif-menningu-þjóðarinnar","rules_applied":["truncated to 80 chars","stopwords removed"]},{"input":"Menning & listir: íslensk hefð","output":"menning-listir-íslensk-hefð","rules_applied":["punctuation removed","ampersand removed"]},{"input":"\"Heiðrún\" sagði hann","output":"heiðrún-sagði-hann","rules_applied":["quotes removed","stopwords removed"]},{"input":"Þórður Ðýrlæksson","output":"þórður-ðýrlæksson","rules_applied":["þ and ð preserved","lowercase applied"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/is-IS.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'it-CH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'IT (CH) Slugification',
    s.content = 'URL slug generation rules for it-CH',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["il","lo","la","i","gli","le","un","uno","una"],"preposition":["di","da","in","con","su","per","tra","fra","a","presso","entro"],"conjunction":["e","o","ma"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[{"word":"","category":"dal","reason":"article+preposition"},{"word":"","category":"dalla","reason":"article+preposition"},{"word":"","category":"dai","reason":"article+preposition"},{"word":"","category":"dalle","reason":"article+preposition"},{"word":"","category":"dallo","reason":"article+preposition"},{"word":"","category":"dagli","reason":"article+preposition"},{"word":"","category":"col","reason":"article+preposition"},{"word":"","category":"coi","reason":"article+preposition"},{"word":"","category":"presso","reason":"preposition"},{"word":"","category":"entro","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Review for consistency"}]',
    s.examples = '[{"input":"Guida Turistica del Canton Ticino","output":"guida-turistica-canton-ticino","rules_applied":["Stopwords removed (del)","diacritics preserved"]},{"input":"Le Migliori Grotti di Lugano","output":"migliori-grotti-lugano","rules_applied":["Stopwords removed (le","di)","lowercase"]},{"input":"Castelli e Fortezze della Svizzera Italiana","output":"castelli-fortezze-svizzera-italiana","rules_applied":["Stopwords removed (e","della)","preserve case"]},{"input":"Perché Visitare Bellinzona e il Suo Patrimonio UNESCO","output":"perché-visitare-bellinzona-suo-patrimonio-unesco","rules_applied":["Diacritics preserved (perché)","stopwords removed (e","il)"]},{"input":"Storia dell\'Orologeria Svizzera","output":"storia-orologeria-svizzera","rules_applied":["Elided article removed (dell\')","lowercase"]},{"input":"10 Sentieri da Percorrere nelle Alpi Ticinesi","output":"10-sentieri-percorrere-alpi-ticinesi","rules_applied":["Numbers preserved","stopwords removed (da","nelle)"]},{"input":"Come Richiedere la Cittadinanza Svizzera presso le Autorità Cantonali del Ticino e del Grigioni Italiano","output":"come-richiedere-cittadinanza-svizzera-autorità-cantonali-ticino-grigioni-italiano","rules_applied":["Long title truncated at 80 chars","multiple stopwords removed"]},{"input":"Polenta & Brasato: Specialità Ticinesi!","output":"polenta-brasato-specialità-ticinesi","rules_applied":["Special chars removed (&",":","!)","diacritics preserved (à)"]},{"input":"L\'Arte del Formaggio d\'Alpe","output":"arte-formaggio-alpe","rules_applied":["Elided articles removed (l\'","d\')","lowercase"]},{"input":"È Possibile Lavorare in Svizzera con un Permesso B?","output":"è-possibile-lavorare-svizzera-permesso-b","rules_applied":["Capital È preserved as lowercase è","question mark removed","stopwords removed (in","con","un)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/it-CH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'it-IT'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Italian (Italy) Slugification',
    s.content = 'URL slug generation rules for it-IT',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["di","da","in","con","su","per","tra","fra","a"],"conjunction":["e","o","ma"],"article":["il","lo","la","i","gli","le","un","uno","una"]}',
    s.stopwords_count = 21,
    s.regional_additions = '[{"word":"","category":"dal","reason":"article+preposition"},{"word":"","category":"dalla","reason":"article+preposition"},{"word":"","category":"dai","reason":"article+preposition"},{"word":"","category":"dalle","reason":"article+preposition"},{"word":"","category":"dallo","reason":"article+preposition"},{"word":"","category":"dagli","reason":"article+preposition"},{"word":"","category":"col","reason":"article+preposition"},{"word":"","category":"coi","reason":"article+preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters"}]',
    s.examples = '[{"input":"Guida Completa al Caffè Italiano","output":"guida-completa-caffè-italiano","rules_applied":["Stopwords removed (al)","diacritics preserved"]},{"input":"Le Migliori Pizzerie di Roma","output":"migliori-pizzerie-roma","rules_applied":["Stopwords removed (le","di)","lowercase"]},{"input":"Cucina Italiana: Ricette Tradizionali","output":"cucina-italiana-ricette-tradizionali","rules_applied":["Special chars removed (:)","stopwords preserved"]},{"input":"Perché Visitare la Città di Venezia","output":"perché-visitare-città-venezia","rules_applied":["Diacritics preserved (perché","città)","stopwords removed (la","di)"]},{"input":"Storia dell\'Arte Rinascimentale","output":"storia-arte-rinascimentale","rules_applied":["Articulated preposition removed (dell\')"]},{"input":"10 Posti da Vedere in Italia","output":"10-posti-vedere-italia","rules_applied":["Numbers preserved","stopwords removed (da","in)"]},{"input":"Come Preparare la Vera Pasta all\'Amatriciana secondo la Tradizione Romana del Centro Italia","output":"come-preparare-vera-pasta-amatriciana-secondo-tradizione-romana-centro","rules_applied":["Long title truncated at 80 chars","multiple stopwords removed (la","all\'","la","del)"]},{"input":"Vino & Formaggio: Gli Abbinamenti Perfetti!","output":"vino-formaggio-abbinamenti-perfetti","rules_applied":["Special chars removed (&",":","!)","stopwords removed (gli)"]},{"input":"L\'Arte del Gelato Artigianale","output":"arte-gelato-artigianale","rules_applied":["Elided article removed (l\')","diacritics in title handled"]},{"input":"È Possibile Imparare l\'Italiano in 3 Mesi?","output":"è-possibile-imparare-italiano-3-mesi","rules_applied":["Capital È preserved as lowercase è","question mark removed","elided article removed (l\')"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/it-IT.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ja-JP'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Japanese (Japan) Slugification',
    s.content = 'URL slug generation rules for ja-JP',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[]',
    s.script_config = '{"primary_script":"japanese","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Japanese characters detected (except numbers)"},{"condition":"Non-standard Unicode","message":"Unicode composition may cause issues"}]',
    s.examples = '[{"input":"東京観光ガイド","output":"東京観光ガイド","rules_applied":["Character preservation","no stopwords"]},{"input":"日本の文化と伝統","output":"日本-文化-伝統","rules_applied":["Particle removal (の","と)"]},{"input":"おすすめのレストラン","output":"おすすめ-レストラン","rules_applied":["Particle removal (の)"]},{"input":"初心者のための料理レシピ","output":"初心者-料理レシピ","rules_applied":["Particle removal (の","ため)"]},{"input":"最新ニュース2025年","output":"最新ニュース2025年","rules_applied":["Number preservation","no stopwords"]},{"input":"桜の名所ベスト10","output":"桜-名所ベスト10","rules_applied":["Particle removal (の)","numbers preserved"]},{"input":"ビジネスマナーとコミュニケーションスキルを向上させるための実践的なアドバイスとテクニック","output":"ビジネスマナー-コミュニケーションスキル-向上させる-実践的-アドバイス-テクニック","rules_applied":["Particle removal (と","を","ため","な)","truncated at 80 chars"]},{"input":"夏休み！家族旅行プラン","output":"夏休み-家族旅行プラン","rules_applied":["Punctuation removed (!)","spacing normalized"]},{"input":"「新商品」の紹介","output":"新商品-紹介","rules_applied":["Quotation marks removed (「」)","particle removal (の)"]},{"input":"スマートフォン vs タブレット","output":"スマートフォン-vs-タブレット","rules_applied":["Katakana preserved","Latin word (vs) preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ja-JP.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'jv-ID'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'JV (ID) Slugification',
    s.content = 'URL slug generation rules for jv-ID',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["ing","nang","menyang","saka","kanggo","karo","dening"],"conjunction":["lan","utawa"],"adverb":["mung"]}',
    s.stopwords_count = 10,
    s.regional_additions = '[{"word":"","category":"wonten","reason":"verb (krama)"},{"word":"","category":"ingkang","reason":"relative pronoun (krama)"},{"word":"","category":"kaliyan","reason":"conjunction (krama)"},{"word":"","category":"saking","reason":"preposition (krama)"},{"word":"","category":"dateng","reason":"preposition (krama)"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed","message":"> 60% of words removed"}]',
    s.examples = '[{"input":"Cara Gawe Website kanggo Pemula","output":"cara-gawe-website-pemula","rules_applied":["Stopwords removed: kanggo"]},{"input":"Tips Nambahi Dodolan Online","output":"tips-nambahi-dodolan-online","rules_applied":["No stopwords"]},{"input":"10 Strategi Marketing Digital sing Apik","output":"10-strategi-marketing-digital-apik","rules_applied":["Numbers preserved","Stopwords removed: sing"]},{"input":"Panuntun Sinau Coding saka Nol","output":"panuntun-sinau-coding-nol","rules_applied":["Stopwords removed: saka"]},{"input":"Resep Sega Goreng Istimewa & Gampang","output":"resep-sega-goreng-istimewa-gampang","rules_applied":["Special char removed: &"]},{"input":"Wisata Jogja: 5 Panggonan sing Kudu Ditiliki","output":"wisata-jogja-5-panggonan-kudu-ditiliki","rules_applied":["Stopwords removed: sing","Punctuation removed: :"]},{"input":"Manfaat Olahraga kanggo Kesehatan Awak lan Pikiran sing Optimal ing Wektu Pandemi 2024","output":"manfaat-olahraga-kesehatan-awak-pikiran-optimal-wektu-pandemi-2024","rules_applied":["Stopwords removed: kanggo","lan","sing","ing; Truncated at 80 chars"]},{"input":"Kabar Anyar! Update #COVID19 @ Indonesia","output":"kabar-anyar-update-covid19-indonesia","rules_applied":["Special chars removed: !","#","@"]},{"input":"\"Rahasia Sukses\" - Cara Mbangun Bisnis Online","output":"rahasia-sukses-cara-mbangun-bisnis-online","rules_applied":["Quotes removed: \"","-"]},{"input":"Rega HP Samsung Anyar (Desember 2024)","output":"rega-hp-samsung-anyar-desember-2024","rules_applied":["Parentheses removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/jv-ID.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ka-GE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'KA (GE) Slugification',
    s.content = 'URL slug generation rules for ka-GE',
    s.slug_rule = 'native_script',
    s.stopwords = '{"adverb":["სად","როგორ","რატომ","აქ"],"verb":["არის","იყო"],"conjunction":["და","ან","მაგრამ","თუ","რომ","როცა"],"pronoun":["ეს","ის","რა","ვინ","მე","შენ","ჩვენ","თქვენ"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"საქართველო","reason":"proper noun"},{"word":"","category":"თბილისი","reason":"proper noun"},{"word":"","category":"ქართული","reason":"adjective"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Latin and Georgian mixed may affect SEO"},{"condition":"All-Latin slug","message":"Consider using Georgian script for local SEO"}]',
    s.examples = '[{"input":"თბილისის ისტორია","output":"ისტორია","rules_applied":["NFC","stopword removal (თბილისის)","space to hyphen"]},{"input":"ქართული ღვინის კულტურა","output":"ღვინის-კულტურა","rules_applied":["NFC","stopword removal (ქართული)","space to hyphen"]},{"input":"საქართველოს მთები და ბუნება","output":"მთები-ბუნება","rules_applied":["NFC","stopword removal (საქართველოს","და)","space to hyphen"]},{"input":"მოგზაურობა კახეთში","output":"მოგზაურობა-კახეთში","rules_applied":["NFC","space to hyphen"]},{"input":"ძველი თბილისის არქიტექტურა","output":"ძველი-არქიტექტურა","rules_applied":["NFC","stopword removal","space to hyphen"]},{"input":"2024 წლის საუკეთესო რესტორნები","output":"2024-წლის-საუკეთესო-რესტორნები","rules_applied":["NFC","numbers preserved","space to hyphen"]},{"input":"საქართველოს ეროვნული მუზეუმის კოლექციის მიმოხილვა და ისტორიული ექსპონატები","output":"საქართველოს-ეროვნული-მუზეუმის-კოლექციის-მიმოხილვა-ისტორიული-ექსპონატები","rules_applied":["NFC","stopword removal (და)","truncation at 80 chars"]},{"input":"ხინკალი: რეცეპტი და მომზადება!","output":"ხინკალი-რეცეპტი-მომზადება","rules_applied":["NFC","punctuation removed","stopword removal (და)"]},{"input":"\"ქართული ცეკვა\" - ტრადიცია და თანამედროვეობა","output":"ქართული-ცეკვა-ტრადიცია-თანამედროვეობა","rules_applied":["NFC","quotes removed","dash normalized","stopword removal (და)"]},{"input":"სვანეთი — შუა საუკუნეების კოშკები","output":"სვანეთი-შუა-საუკუნეების-კოშკები","rules_applied":["NFC","em dash to hyphen","space normalization"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ka-GE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'kk-KZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'KK (KZ) Slugification',
    s.content = 'URL slug generation rules for kk-KZ',
    s.slug_rule = 'native_script',
    s.stopwords = '{"currency":["теңге"],"abbreviation":["ҚР","ТМД"],"pronoun":["сен","ол","біз"],"conjunction":["және","бірақ","немесе"]}',
    s.stopwords_count = 9,
    s.regional_additions = '[{"word":"","category":"ҚР","reason":"abbreviation"},{"word":"","category":"ТМД","reason":"abbreviation"},{"word":"","category":"теңге","reason":"currency"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Verify intentional (technical terms allowed)"},{"condition":"Non-standard Unicode composition","message":"Check normalization"}]',
    s.examples = '[{"input":"Қазақстан жаңалықтары","output":"қазақстан-жаңалықтары","rules_applied":["NFC","lowercase","space→hyphen"]},{"input":"Алматы қаласының тарихы","output":"алматы-қаласының-тарихы","rules_applied":["NFC","lowercase","space→hyphen"]},{"input":"Ұлттық музей туралы мәлімет","output":"ұлттық-музей-мәлімет","rules_applied":["NFC","lowercase","stopword \"туралы\" removed"]},{"input":"Қазақ тілін үйрену және дамыту","output":"қазақ-тілін-үйрену-дамыту","rules_applied":["NFC","lowercase","stopwords \"және\" removed"]},{"input":"Астана - елордамыз","output":"астана-елордамыз","rules_applied":["NFC","lowercase","dash normalized"]},{"input":"2024 жылғы экономикалық көрсеткіштер","output":"2024-жылғы-экономикалық-көрсеткіштер","rules_applied":["NFC","numbers preserved","lowercase"]},{"input":"Қазақстанның табиғи ресурстары мен экономикалық әлеуеті жайында толық ақпарат","output":"қазақстанның-табиғи-ресурстары-экономикалық-әлеуеті-жайында-толық-ақпарат","rules_applied":["NFC","lowercase","stopword \"мен\" removed","truncated to 80 chars"]},{"input":"Қарағанды облысы: өнеркәсіп орталығы!","output":"қарағанды-облысы-өнеркәсіп-орталығы","rules_applied":["NFC","lowercase","punctuation removed"]},{"input":"\"Қазақ елі\" ұлттық идеясы","output":"қазақ-елі-ұлттық-идеясы","rules_applied":["NFC","lowercase","quotes removed"]},{"input":"Әңгіме: Өмір мен өлім","output":"әңгіме-өмір-өлім","rules_applied":["NFC","lowercase","stopword \"мен\" removed","colon removed","Kazakh-specific chars preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/kk-KZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'km-KH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'KM (KH) Slugification',
    s.content = 'URL slug generation rules for km-KH',
    s.slug_rule = 'native_script',
    s.stopwords = '{"preposition":["នៅ","ក្នុង","របស់","ពី"],"adverb":["ផង"],"demonstrative":["នេះ","នោះ"],"conjunction":["និង"],"verb":["មាន","ជា"]}',
    s.stopwords_count = 10,
    s.regional_additions = '[{"word":"","category":"ផង","reason":"adverb"},{"word":"","category":"បាទ","reason":"particle"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Warn if Latin mixed with Khmer (except numbers)"}]',
    s.examples = '[{"input":"#","output":"Input (Romanized)","rules_applied":["Slug"]},{"input":"","output":"---","rules_applied":["-------------------"]},{"input":"portomann-thmei","output":"NFC, hyphenate","rules_applied":[]},{"input":"atthobat now knong protes (article in country)","output":"atthobat-protes","rules_applied":["Stopword removal (now","knong)"]},{"input":"3","output":"kampuchea sras saat (beautiful Cambodia)","rules_applied":["kampuchea-sras-saat"]},{"input":"","output":"4","rules_applied":["phnom penh reachtheanei (Phnom Penh capital)"]},{"input":"NFC, hyphenate","output":"","rules_applied":["5"]},{"input":"tesachor-kar-thveu-damnaoer","output":"Stopword removal (ning)","rules_applied":[]},{"input":"angkor wat 2025","output":"angkor-wat-2025","rules_applied":["Numbers preserved"]},{"input":"7","output":"[Title exceeding 80 characters in Khmer script]","rules_applied":["[Truncated at 80 chars]"]},{"input":"","output":"8","rules_applied":["ahar \"khmer\" chnganh (delicious \"Khmer\" food)"]},{"input":"Quotes removed","output":"","rules_applied":["9"]},{"input":"tae-avei-chea-brapeinei","output":"Punctuation removed","rules_applied":[]},{"input":"sompeak sralanh (beloved shirt - complex cluster)","output":"sompeak-sralanh","rules_applied":["Complex clusters preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/km-KH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'kn-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'KN (IN) Slugification',
    s.content = 'URL slug generation rules for kn-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"verb":["ಇದೆ","ಆಗಿದೆ","ಮಾಡಿ","ಬಂದ","ಹೋದ"],"demonstrative":["ಈ","ಆ"],"pronoun":["ಇದು","ಅದು","ಅವರು","ಅವನು","ಅವಳು"],"article":["ಒಂದು"],"conjunction":["ಮತ್ತು","ಹಾಗೂ","ಆದರೆ","ಅಥವಾ","ಆದ್ದರಿಂದ"],"interrogative":["ಏನು","ಯಾರು","ಹೇಗೆ","ಎಲ್ಲಿ","ಯಾವ"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Kannada with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"ಬೆಂಗಳೂರಿನ ಉತ್ತಮ ರೆಸ್ಟೋರೆಂಟ್‌ಗಳು","output":"ಬೆಂಗಳೂರಿನ-ಉತ್ತಮ-ರೆಸ್ಟೋರೆಂಟ್‌ಗಳು","rules_applied":["Spaces to hyphens","script preserved"]},{"input":"ಮೈಸೂರು ಅರಮನೆಯ ಇತಿಹಾಸ","output":"ಮೈಸೂರು-ಅರಮನೆಯ-ಇತಿಹಾಸ","rules_applied":["Spaces to hyphens","script preserved"]},{"input":"ಕರ್ನಾಟಕದ ಸಂಸ್ಕೃತಿ ಮತ್ತು ಸಂಪ್ರದಾಯ","output":"ಕರ್ನಾಟಕದ-ಸಂಸ್ಕೃತಿ-ಸಂಪ್ರದಾಯ","rules_applied":["Stopword removed (ಮತ್ತು)","script preserved"]},{"input":"ಈ ವಾರದ ಟಾಪ್ ಸುದ್ದಿಗಳು","output":"ವಾರದ-ಟಾಪ್-ಸುದ್ದಿಗಳು","rules_applied":["Stopword removed (ಈ)","script preserved"]},{"input":"ಹೇಗೆ ಮಾಡಬೇಕು ಬಿಸಿಬೇಳೆ ಬಾತ್","output":"ಮಾಡಬೇಕು-ಬಿಸಿಬೇಳೆ-ಬಾತ್","rules_applied":["Stopword removed (ಹೇಗೆ)","script preserved"]},{"input":"2025ರ ಐಪಿಎಲ್ ವೇಳಾಪಟ್ಟಿ","output":"2025ರ-ಐಪಿಎಲ್-ವೇಳಾಪಟ್ಟಿ","rules_applied":["Number preserved","script preserved"]},{"input":"ಬೆಂಗಳೂರು ನಗರದ ಅಭಿವೃದ್ಧಿ ಯೋಜನೆಗಳು ಮತ್ತು ಮೂಲಸೌಕರ್ಯ ಸುಧಾರಣೆಗಳ ಬಗ್ಗೆ ಸಂಪೂರ್ಣ ಮಾಹಿತಿ","output":"ಬೆಂಗಳೂರು-ನಗರದ-ಅಭಿವೃದ್ಧಿ-ಯೋಜನೆಗಳು-ಮೂಲಸೌಕರ್ಯ-ಸುಧಾರಣೆಗಳ-ಬಗ್ಗೆ-ಸಂಪೂರ್ಣ","rules_applied":["Truncated to 80 chars","stopword removed (ಮತ್ತು)"]},{"input":"ಕನ್ನಡ ಸಿನಿಮಾ: ಹೊಸ ಚಿತ್ರಗಳು!","output":"ಕನ್ನಡ-ಸಿನಿಮಾ-ಹೊಸ-ಚಿತ್ರಗಳು","rules_applied":["Special chars removed (:","!)","script preserved"]},{"input":"\"ಕೆಜಿಎಫ್\" ಚಿತ್ರದ ಯಶಸ್ಸು","output":"ಕೆಜಿಎಫ್-ಚಿತ್ರದ-ಯಶಸ್ಸು","rules_applied":["Quotes removed","script preserved"]},{"input":"ಶ್ರೀ ಕೃಷ್ಣದೇವರಾಯ & ವಿಜಯನಗರ ಸಾಮ್ರಾಜ್ಯ","output":"ಶ್ರೀ-ಕೃಷ್ಣದೇವರಾಯ-ವಿಜಯನಗರ-ಸಾಮ್ರಾಜ್ಯ","rules_applied":["Ampersand removed","conjunct consonants (ಕ್ಷ","ಕ್ಕ) preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/kn-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ko-KR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Korean (South Korea) Slugification',
    s.content = 'URL slug generation rules for ko-KR',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[]',
    s.script_config = '{"primary_script":"korean","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Ensure Latin text is necessary (e.g., brand names)"},{"condition":"Excessive hyphens","message":"More than 5 hyphens may indicate poor title structure"}]',
    s.examples = '[{"input":"서울 여행 가이드","output":"서울-여행-가이드","rules_applied":["NFC","space→hyphen"]},{"input":"한국의 문화와 전통","output":"한국-문화-전통","rules_applied":["NFC","particles removed (의","와)"]},{"input":"2025년 최고의 맛집","output":"2025년-최고-맛집","rules_applied":["NFC","particle removed (의)"]},{"input":"K-팝 아티스트 인기 순위","output":"k-팝-아티스트-인기-순위","rules_applied":["Latin lowercase","Hangul preserved"]},{"input":"AI 기술과 미래 사회","output":"ai-기술-미래-사회","rules_applied":["Latin lowercase","particle removed (과)"]},{"input":"부산에서 서울까지: 여행 코스","output":"부산-서울까지-여행-코스","rules_applied":["Particle removed (에서)","punctuation removed"]},{"input":"한국 전통 음식 - 김치, 불고기, 비빔밥 소개","output":"한국-전통-음식-김치-불고기-비빔밥-소개","rules_applied":["Long title","commas removed","truncated if needed"]},{"input":"삼성 vs LG: 스마트폰 비교!","output":"삼성-vs-lg-스마트폰-비교","rules_applied":["Latin lowercase","special chars removed"]},{"input":"\"겨울왕국\" 영화 리뷰","output":"겨울왕국-영화-리뷰","rules_applied":["Quotes removed"]},{"input":"제주도 한라산 등산 코스 (초보자용)","output":"제주도-한라산-등산-코스-초보자용","rules_applied":["Parentheses removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ko-KR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ku-TR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'KU (TR) Slugification',
    s.content = 'URL slug generation rules for ku-TR',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["u","an","le","ku","eger"],"article":["yek","hinek","hemû"],"verb":["e","bû","ye","ne","heye","nine"],"adverb":["iro","duh","sibe","geleki"],"pronoun":["ew","ev","ez","em","tu","hun"],"preposition":["di","de","ji","bi","li","ser","ber","nav"]}',
    s.stopwords_count = 32,
    s.regional_additions = '[{"word":"","category":"iro","reason":"adverb"},{"word":"","category":"duh","reason":"adverb"},{"word":"","category":"sibe","reason":"adverb"},{"word":"","category":"geleki","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Kurdish Latin with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"Ciheki gelek ciwan","output":"ciheki-ciwan","rules_applied":["Stopword removed (gelek)","lowercase","hyphenation"]},{"input":"Riya ji bo serketin","output":"riya-serketin","rules_applied":["Stopwords removed (ji","bo)","circumflex preserved"]},{"input":"Zimaneki nû di dibistane de","output":"zimaneki-nû-dibistane","rules_applied":["Stopwords removed (di","de)","circumflex preserved"]},{"input":"Xwarina kurdi u cejna Newroze","output":"xwarina-kurdi-cejna-newroze","rules_applied":["Stopword removed (u)","diacritics preserved"]},{"input":"Serleheng li dinyaye","output":"serleheng-dinyaye","rules_applied":["Stopword removed (li)","lowercase"]},{"input":"10 resimên jiyana rojane li Amedê","output":"10-resimên-jiyana-rojane-amedê","rules_applied":["Number preserved","stopword removed (li)","circumflex preserved"]},{"input":"Hunera kurdi: Muzik, dans, u helbesta klasik ji serdema berê heta iro","output":"hunera-kurdi-muzik-dans-helbesta-klasik-serdema-berê-heta","rules_applied":["Truncated to 80 chars","stopwords removed (u","ji)","special chars removed (:)"]},{"input":"Cejnên Newrozê! Kevnesoriya kurdi & kultura me","output":"cejnên-newrozê-kevnesoriya-kurdi-kultura","rules_applied":["Special chars removed (!","&)","lowercase"]},{"input":"Navenda \"Welat\" ya perendeperwerde li Tirkiye","output":"navenda-welat-perendeperwerde-tirkiye","rules_applied":["Quotes removed","stopwords removed (ya","li)"]},{"input":"Wergera \"sipan\" u \"serok\": ferqê maneya wan","output":"wergera-sipan-serok-ferqê-maneya-wan","rules_applied":["Quotes removed","stopword removed (u)","colon removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ku-TR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ky-KG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'KY (KG) Slugification',
    s.content = 'URL slug generation rules for ky-KG',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["жана"],"verb":["болуп"],"pronoun":["бул","ал","анын"],"negation":["эмес"]}',
    s.stopwords_count = 6,
    s.regional_additions = '[{"word":"","category":"эле","reason":"particle"},{"word":"","category":"го","reason":"particle"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Unexpected non-Cyrillic characters (except digits)"},{"condition":"Russian-only characters","message":"May indicate non-Kyrgyz content"}]',
    s.examples = '[{"input":"Кыргызстандын тарыхы","output":"кыргызстандын-тарыхы","rules_applied":["lowercase","stopwords","hyphens"]},{"input":"Бишкек шаарынын көрүнүштөрү","output":"бишкек-шаарынын-көрүнүштөрү","rules_applied":["lowercase","hyphens","ү/ө preserved"]},{"input":"Ысык-Көл жана анын сулуулугу","output":"ысык-көл-сулуулугу","rules_applied":["lowercase","stopwords (жана","анын) removed"]},{"input":"Манас эпосу боюнча изилдөө","output":"манас-эпосу-изилдөө","rules_applied":["stopwords (боюнча) removed"]},{"input":"Кыргыз тили үчүн окуу китеби","output":"кыргыз-тили-окуу-китеби","rules_applied":["stopwords (үчүн) removed"]},{"input":"2025-жылдагы экономикалык өнүгүү","output":"2025-жылдагы-экономикалык-өнүгүү","rules_applied":["numbers preserved","hyphens"]},{"input":"Кыргызстандагы мамлекеттик саясат жана коомдук турмуш маселелери","output":"кыргызстандагы-мамлекеттик-саясат-коомдук-турмуш-маселелери","rules_applied":["truncated","stopwords (жана) removed"]},{"input":"Ата-Мекендик согуш: эстелик!","output":"ата-мекендик-согуш-эстелик","rules_applied":["punctuation removed","hyphen preserved"]},{"input":"\"Көк бөрү\" оюну тууралуу","output":"көк-бөрү-оюну-тууралуу","rules_applied":["quotes removed","ө/ү preserved"]},{"input":"Ңарык менен Өзгөн ортосундагы жол","output":"ңарык-өзгөн-ортосундагы-жол","rules_applied":["stopwords (менен) removed","ң/ө preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ky-KG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ln-CD'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'LN (CD) Slugification',
    s.content = 'URL slug generation rules for ln-CD',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["ntango"],"conjunction":["kasi","soki"],"adverb":["mingi"]}',
    s.stopwords_count = 4,
    s.regional_additions = '[{"word":"","category":"kasi","reason":"conjunction"},{"word":"","category":"soki","reason":"conjunction"},{"word":"","category":"ntango","reason":"preposition"},{"word":"","category":"mingi","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"> 60% stopwords removed","message":"Title may be too generic"}]',
    s.examples = '[{"input":"Ndako ya bísó","output":"ndako-biso","rules_applied":["stopword (ya)","strip accent (í)"]},{"input":"Boyébi na Lingála","output":"boyebi-lingala","rules_applied":["stopword (na)","strip accents"]},{"input":"Mateya ya Bomwana","output":"mateya-bomwana","rules_applied":["stopword (ya)"]},{"input":"Músiki ya Kongó","output":"musiki-kongo","rules_applied":["stopword (ya)","strip accents"]},{"input":"Lisolo mpe Makambo","output":"lisolo-makambo","rules_applied":["stopword (mpe)"]},{"input":"Bato 10 ya Kinshasa","output":"bato-10-kinshasa","rules_applied":["stopword (ya)","number preserved"]},{"input":"Bokasi mpe Botɔngɔ ya Mboka na bísó: Makambo ya Koyéba mpenza mpo na Bana","output":"bokasi-botongo-mboka-biso-makambo-koyeba-mpenza-bana","rules_applied":["truncation at 80 chars","stopwords (mpe","ya","na)","map ɔ to o","strip accents"]},{"input":"Bilei & Masanga!","output":"bilei-masanga","rules_applied":["ampersand removed","exclamation removed"]},{"input":"\"Nkóló\" na Lopango","output":"nkolo-lopango","rules_applied":["quotes removed","stopword (na)","strip accent"]},{"input":"Kɔngɔ mpe Bɛlɛ","output":"kongo-bele","rules_applied":["map ɔ to o","map ɛ to e","stopword (mpe)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ln-CD.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'lt-LT'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Lithuanian (Lithuania) Slugification',
    s.content = 'URL slug generation rules for lt-LT',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"verb":["yra","buvo"],"pronoun":["tai","kas","kuris","kuri"],"preposition":["į","iš","su","be","prie","po","per","ant","už","nuo","apie","pas"],"conjunction":["ir","arba","bet","tačiau","kad","jei","nes","nors"]}',
    s.stopwords_count = 26,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Lithuanian with non-Latin scripts"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"Kelionė į Vilnių","output":"kelionė-vilnių","rules_applied":["Preposition removed (į)","diacritics preserved","lowercase"]},{"input":"Lietuvos istorija ir kultūra","output":"lietuvos-istorija-kultūra","rules_applied":["Conjunction removed (ir)","diacritics preserved"]},{"input":"Gidas po Trakų pilį","output":"gidas-trakų-pilį","rules_applied":["Preposition removed (po)","diacritics preserved"]},{"input":"Šaltibarščiai su bulvėmis","output":"šaltibarščiai-bulvėmis","rules_applied":["Preposition removed (su)","š and ė preserved"]},{"input":"Kaip pasiekti Kauną iš Vilniaus","output":"kaip-pasiekti-kauną-vilniaus","rules_applied":["Preposition removed (iš)","diacritics preserved"]},{"input":"10 geriausių restoranų Klaipėdoje","output":"10-geriausių-restoranų-klaipėdoje","rules_applied":["Number preserved","diacritics preserved"]},{"input":"Tradiciniai lietuviški patiekalai: cepelinai, kugelis ir šaltibarščiai","output":"tradiciniai-lietuviški-patiekalai-cepelinai-kugelis-šaltibarščiai","rules_applied":["Colon removed","conjunction (ir) removed","truncated if needed"]},{"input":"Muziejus & galerija: Vilniaus menas!","output":"muziejus-galerija-vilniaus-menas","rules_applied":["Special chars removed (&",":","!)","lowercase"]},{"input":"\"Žalgiris\" ar \"Rytas\": kuris geresnis?","output":"žalgiris-rytas-kuris-geresnis","rules_applied":["Quotes removed","conjunction (ar) removed","question mark removed"]},{"input":"Ąžuolų giraitė – Lietuvos gamtos paveldo objektas","output":"ąžuolų-giraitė-lietuvos-gamtos-paveldo-objektas","rules_applied":["Em dash removed","ą and ž preserved in word-initial position"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/lt-LT.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'lv-LV'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Latvian (Latvia) Slugification',
    s.content = 'URL slug generation rules for lv-LV',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["un","vai","bet","jo","ka","lai","kad","ja"],"pronoun":["tas","tā","šis","šī","kas"],"verb":["ir","nav"],"preposition":["no","uz","ar","par","pie","pēc","bez","līdz","pa","caur"]}',
    s.stopwords_count = 25,
    s.regional_additions = '[{"word":"","category":"lai","reason":"conjunction"},{"word":"","category":"kad","reason":"conjunction"},{"word":"","category":"ja","reason":"conjunction"},{"word":"","category":"līdz","reason":"preposition"},{"word":"","category":"pa","reason":"preposition"},{"word":"","category":"caur","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Check for unintended character mixing"}]',
    s.examples = '[{"input":"Rīgas vecpilsēta un tās vēsture","output":"rīgas-vecpilsēta-vēsture","rules_applied":["Lowercase","stopwords (un","tās) removed","diacritics preserved"]},{"input":"Latvijas dabas parks pie Gaujas","output":"latvijas-dabas-parks-gaujas","rules_applied":["Lowercase","stopwords (pie) removed","diacritics preserved"]},{"input":"Kā izvēlēties labāko produktu","output":"izvēlēties-labāko-produktu","rules_applied":["Lowercase","stopwords (kā) removed","diacritics preserved"]},{"input":"Dzīvokļu cenas Liepājā 2025","output":"dzīvokļu-cenas-liepājā-2025","rules_applied":["Lowercase","diacritics preserved","numbers kept"]},{"input":"Ceļojums uz Siguldu ar ģimeni","output":"ceļojums-siguldu-ģimeni","rules_applied":["Lowercase","stopwords (uz","ar) removed","diacritics preserved"]},{"input":"10 labākās kafejnīcas Rīgā 2025","output":"10-labākās-kafejnīcas-rīgā-2025","rules_applied":["Numbers preserved","diacritics preserved"]},{"input":"Kā pareizi sagatavot māju ziemai un kāpēc tas ir tik svarīgi Latvijas klimatā","output":"kā-pareizi-sagatavot-māju-ziemai-kāpēc-svarīgi-latvijas-klimatā","rules_applied":["Truncation at 80 chars","stopwords removed"]},{"input":"Pasākumi, koncerti & izstādes Rīgā!","output":"pasākumi-koncerti-izstādes-rīgā","rules_applied":["Punctuation removed (comma","ampersand","exclamation)","diacritics preserved"]},{"input":"\"Labākais\" restorāns Jūrmalā","output":"labākais-restorāns-jūrmalā","rules_applied":["Quotes removed","diacritics preserved"]},{"input":"Šķērsošanas noteikumi Eiropas Savienībā","output":"šķērsošanas-noteikumi-eiropas-savienībā","rules_applied":["Complex diacritics (š","ķ","ē) preserved correctly"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/lv-LV.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'mg-MG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'MG (MG) Slugification',
    s.content = 'URL slug generation rules for mg-MG',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"article":["ny","ilay"],"preposition":["amin","ho","any","eto","ao","momba","miaraka","avy","mankany"],"conjunction":["sy","na","fa","dia","ary"]}',
    s.stopwords_count = 16,
    s.regional_additions = '[{"word":"","category":"momba","reason":"preposition"},{"word":"","category":"miaraka","reason":"preposition"},{"word":"","category":"avy","reason":"preposition"},{"word":"","category":"mankany","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Stopwords removed > 60%","message":"Review title structure"},{"condition":"French loanword accents","message":"Verify stripping preserves meaning"}]',
    s.examples = '[{"input":"Ny toeram-pitsaboana vaovao","output":"toeram-pitsaboana-vaovao","rules_applied":["Stopword removal (ny)","lowercase"]},{"input":"Fambolena sy fiompiana any Antananarivo","output":"fambolena-fiompiana-antananarivo","rules_applied":["Stopword removal (sy","any)","lowercase"]},{"input":"Fomba fianarana ho an\'ny ankizy","output":"fomba-fianarana-ankizy","rules_applied":["Stopword removal (ho","ny)","apostrophe removed"]},{"input":"Vaovao momba ny toekarena malagasy","output":"vaovao-toekarena-malagasy","rules_applied":["Stopword removal (momba","ny)","lowercase"]},{"input":"Sakafo malagasy traditionnelle","output":"sakafo-malagasy-traditionnelle","rules_applied":["Lowercase only","no stopwords"]},{"input":"10 torohevitra ho an\'ny mpianatra","output":"10-torohevitra-mpianatra","rules_applied":["Number preserved","stopwords removed (ho","ny)"]},{"input":"Ny tantaran\'ny fanjakana malagasy tamin\'ny vanim-potoana samihafa dia nisy fiovan-kevitra maro","output":"tantaranny-fanjakana-malagasy-taminny-vanim-potoana-samihafa-nisy-fiovan","rules_applied":["Truncation at 80 chars","stopwords removed"]},{"input":"Vokatra vaovao: fambolena organika!","output":"vokatra-vaovao-fambolena-organika","rules_applied":["Punctuation removed (: !)","lowercase"]},{"input":"Ny \"fomban-drazana\" malagasy","output":"fomban-drazana-malagasy","rules_applied":["Quotes removed","stopword removed (ny)"]},{"input":"Hôtel sy trano fandraisam-bahiny","output":"hotel-trano-fandraisam-bahiny","rules_applied":["Circumflex stripped (ô→o)","stopword removed (sy)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/mg-MG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'mi-NZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'MI (NZ) Slugification',
    s.content = 'URL slug generation rules for mi-NZ',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[{"word":"","category":"aotearoa","reason":"place name"},{"word":"","category":"hou","reason":"adjective"},{"word":"","category":"pai","reason":"adjective"},{"word":"","category":"nui","reason":"adjective"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful URL"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"All stopwords removed","message":"Title may need rephrasing"},{"condition":"Macron stripping ambiguity","message":"Some Māori words differ only by macron (context needed)"}]',
    s.examples = '[{"input":"Te Reo Māori i te Kura","output":"reo-maori-kura","rules_applied":["Stopwords (te","i) removed","macron stripped"]},{"input":"Ngā Mahi Toi o Aotearoa","output":"mahi-toi","rules_applied":["Stopwords (nga","o","aotearoa) removed"]},{"input":"He Whakaaro Hou","output":"whakaaro","rules_applied":["Stopwords (he","hou) removed"]},{"input":"Tikanga Māori me te Ture","output":"tikanga-maori-ture","rules_applied":["Stopword (me","te) removed","macron stripped"]},{"input":"Kaiako ki te Whare Wānanga","output":"kaiako-whare-wananga","rules_applied":["Stopwords (ki","te) removed","macron stripped"]},{"input":"Top 10 Kupu Māori 2026","output":"top-10-kupu-maori-2026","rules_applied":["Numbers preserved","macron stripped"]},{"input":"He Kōrero mō ngā Tūpuna o te Waka Hourua me ngā Haerenga ki te Moana-nui-a-Kiwa","output":"korero-tupuna-waka-hourua-haerenga-moana-nui-kiwa","rules_applied":["Truncation at word boundary","stopwords removed"]},{"input":"Waiata: He Mea Taonga!","output":"waiata-mea-taonga","rules_applied":["Punctuation removed (colon","exclamation)"]},{"input":"\"Kia Ora\" - He Mihi","output":"kia-ora-mihi","rules_applied":["Quotes and dash removed","stopword (he) removed"]},{"input":"Whānau & Hapū","output":"whanau-hapu","rules_applied":["Ampersand removed","macrons stripped"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/mi-NZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'mk-MK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'MK (MK) Slugification',
    s.content = 'URL slug generation rules for mk-MK',
    s.slug_rule = 'native_script',
    s.stopwords = '{"preposition":["на","во","со","од","за","до","кон","при","низ"],"conjunction":["и","или","но","а"],"pronoun":["што","кој","која","кое","тој","таа","тоа","овој","оваа","ова"],"verb":["е","се","беше","има","нема"],"adverb":["како","кога","каде"]}',
    s.stopwords_count = 31,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Cyrillic with Latin (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"Водич за Скопје","output":"водич-скопје","rules_applied":["Stopword removed (за)","lowercase","hyphenation"]},{"input":"Историјата на Македонија","output":"историјата-македонија","rules_applied":["Preposition removed (на)","lowercase"]},{"input":"Најдобрите ресторани во Охрид","output":"најдобрите-ресторани-охрид","rules_applied":["Preposition removed (во)","lowercase"]},{"input":"Патување низ македонските планини","output":"патување-македонските-планини","rules_applied":["Preposition removed (низ)","lowercase"]},{"input":"Како да направите тавче гравче","output":"како-направите-тавче-гравче","rules_applied":["Conjunction removed (да)","lowercase"]},{"input":"10 причини за посета на Битола","output":"10-причини-посета-битола","rules_applied":["Number preserved","stopwords removed (за","на)"]},{"input":"Античка Македонија: Александар Велики и неговото царство низ вековите на историјата","output":"античка-македонија-александар-велики-неговото-царство-вековите-историјата","rules_applied":["Truncated to 80 chars","stopwords removed (и","низ","на)"]},{"input":"Рецепти & готвење: Традиционална кујна!","output":"рецепти-готвење-традиционална-кујна","rules_applied":["Special chars removed (&",":","!)","lowercase"]},{"input":"Кафе \"турско\" или \"домашно\"?","output":"кафе-турско-домашно","rules_applied":["Quotes removed","conjunction (или) removed","question mark removed"]},{"input":"Охридско Езеро vs. Преспанско: кое е подобро за одмор","output":"охридско-езеро-преспанско-кое-подобро-одмор","rules_applied":["Dot removed (vs.)","colon removed","stopwords (е","за) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/mk-MK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ml-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ML (IN) Slugification',
    s.content = 'URL slug generation rules for ml-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Malayalam with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"},{"condition":"Broken conjuncts","message":"Virama-based clusters should remain intact"}]',
    s.examples = '[{"input":"കേരളത്തിലെ മികച്ച ടൂറിസ്റ്റ് സ്ഥലങ്ങള്","output":"കേരളത്തിലെ-മികച്ച-ടൂറിസ്റ്റ്-സ്ഥലങ്ങള്","rules_applied":["Spaces to hyphens","conjuncts preserved"]},{"input":"മലയാളം സിനിമ ഒരു ആമുഖം","output":"മലയാളം-സിനിമ-ആമുഖം","rules_applied":["Stopword removed (ഒരു)","hyphenation"]},{"input":"തിരുവനന്തപുരം ആണ് ഇന്ത്യയിലെ ഏറ്റവും വൃത്തിയുള്ള നഗരം","output":"തിരുവനന്തപുരം-ഇന്ത്യയിലെ-ഏറ്റവും-വൃത്തിയുള്ള-നഗരം","rules_applied":["Stopword removed (ആണ്)","script preserved"]},{"input":"കോഴിക്കോട് മലബാര് ബിരിയാണി പാചകക്കുറിപ്പ്","output":"കോഴിക്കോട്-മലബാര്-ബിരിയാണി-പാചകക്കുറിപ്പ്","rules_applied":["Chillu letters preserved","spaces to hyphens"]},{"input":"ആയുര്വേദം എന്ന പാരമ്പര്യ ചികിത്സ","output":"ആയുര്വേദം-പാരമ്പര്യ-ചികിത്സ","rules_applied":["Stopwords removed (എന്ന)","consonant clusters preserved"]},{"input":"2025ല് കേരളത്തില് 10 പുതിയ പദ്ധതികള്","output":"2025ല്-കേരളത്തില്-10-പുതിയ-പദ്ധതികള്","rules_applied":["Numbers preserved","chillu preserved"]},{"input":"കേരള സംസ്കാരം: നാടന് കലകളും ഉത്സവങ്ങളും ഭക്ഷണ പാരമ്പര്യവും ഒരു സമഗ്ര വിശകലനം","output":"കേരള-സംസ്കാരം-നാടന്-കലകളും-ഉത്സവങ്ങളും-ഭക്ഷണ-പാരമ്പര്യവും-സമഗ്ര-വിശകലനം","rules_applied":["Truncated at 80 chars","stopword removed (ഒരു)","colon removed"]},{"input":"ഓണം & വിഷു: ആഘോഷങ്ങള്!","output":"ഓണം-വിഷു-ആഘോഷങ്ങള്","rules_applied":["Special chars removed (&",":","!)","hyphenation"]},{"input":"\"കഥകളി\" എന്ന് അറിയപ്പെടുന്ന കലാരൂപം","output":"കഥകളി-അറിയപ്പെടുന്ന-കലാരൂപം","rules_applied":["Quotes removed","stopword removed (എന്ന്)"]},{"input":"മലപ്പുറം vs. പാലക്കാട്: ഏത് ജില്ലയാണ് നല്ലത്?","output":"മലപ്പുറം-പാലക്കാട്-ഏത്-ജില്ല-നല്ലത്","rules_applied":["Special chars removed (vs.",":","?)","stopword removed (ആണ്)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ml-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'mn-MN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'MN (MN) Slugification',
    s.content = 'URL slug generation rules for mn-MN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"interrogative":["ямар","юу","хэн"],"conjunction":["бөгөөд","ба","болон","эсвэл","харин","гэхдээ"],"demonstrative":["энэ","тэр","ийм","тийм"]}',
    s.stopwords_count = 13,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Cyrillic with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"Монгол улсын түүх","output":"монгол-улсын-түүх","rules_applied":["Lowercase","hyphenation","Үү preserved"]},{"input":"Улаанбаатар хотын аялал","output":"улаанбаатар-хотын-аялал","rules_applied":["Lowercase","hyphenation"]},{"input":"Говь цөлийн амьтад","output":"говь-цөлийн-амьтад","rules_applied":["Lowercase","Өө preserved"]},{"input":"Монгол ардын уламжлалт хоол","output":"монгол-ардын-уламжлалт-хоол","rules_applied":["Lowercase","hyphenation"]},{"input":"Энэ бол миний гэр","output":"миний-гэр","rules_applied":["Stopwords removed (энэ","бол)","lowercase"]},{"input":"2024 оны шинэ жилийн баяр","output":"2024-оны-шинэ-жилийн-баяр","rules_applied":["Number preserved","lowercase"]},{"input":"Монгол улсын Ерөнхийлөгчийн сонгуулийн тухай дэлгэрэнгүй мэдээлэл ба түүний нөлөө","output":"монгол-улсын-ерөнхийлөгчийн-сонгуулийн-тухай-дэлгэрэнгүй-мэдээлэл-түүний-нөлөө","rules_applied":["Truncated at word boundary to 80 chars","stopword (ба) removed"]},{"input":"Хүннү гүрэн: эртний түүх!","output":"хүннү-гүрэн-эртний-түүх","rules_applied":["Special chars removed (: !)","Үү preserved"]},{"input":"\"Хөх тэнгэр\" дуу","output":"хөх-тэнгэр-дуу","rules_applied":["Quotes removed","Өө preserved"]},{"input":"Өвөр Монгол vs Монгол Улс: ялгаа юу вэ","output":"өвөр-монгол-монгол-улс-ялгаа","rules_applied":["Stopwords removed (vs","юу","вэ)","special chars removed (:)","Өө preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/mn-MN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'mr-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'MR (IN) Slugification',
    s.content = 'URL slug generation rules for mr-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"interrogative":["कसे","काय","कोण"],"verb":["आहे","असे","होते","केले","करणे"],"quantifier":["सर्व","अनेक"],"demonstrative":["हा","ही","हे","तो","ती","ते"],"conjunction":["आणि","व","किंवा","पण","परंतु","म्हणून","कारण","जर"]}',
    s.stopwords_count = 24,
    s.regional_additions = '[{"word":"","category":"कसे","reason":"interrogative"},{"word":"","category":"काय","reason":"interrogative"},{"word":"","category":"कोण","reason":"interrogative"},{"word":"","category":"सर्व","reason":"quantifier"},{"word":"","category":"अनेक","reason":"quantifier"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Devanagari with Latin (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"},{"condition":"Broken conjuncts","message":"Ensure virama-based conjuncts are intact"}]',
    s.examples = '[{"input":"मुंबईचा इतिहास","output":"मुंबईचा-इतिहास","rules_applied":["Basic hyphenation","Devanagari preserved"]},{"input":"महाराष्ट्रातील पर्यटन स्थळे","output":"महाराष्ट्रातील-पर्यटन-स्थळे","rules_applied":["Conjuncts preserved (ष्ट्र","ळ)","hyphenation"]},{"input":"पुणे ते मुंबई प्रवास","output":"पुणे-मुंबई-प्रवास","rules_applied":["Postposition removed (ते)","hyphenation"]},{"input":"गणपती बाप्पा मोरया","output":"गणपती-बाप्पा-मोरया","rules_applied":["Religious phrase preserved intact"]},{"input":"मराठी साहित्याची ओळख","output":"मराठी-साहित्याची-ओळख","rules_applied":["Genitive suffix preserved","Marathi ळ preserved"]},{"input":"२०२५ मधील सर्वोत्तम चित्रपट","output":"२०२५-सर्वोत्तम-चित्रपट","rules_applied":["Devanagari numeral preserved","postposition (मधील) and quantifier (सर्व) removed"]},{"input":"छत्रपती शिवाजी महाराज यांचे जीवनचरित्र आणि त्यांचा महाराष्ट्रातील योगदान","output":"छत्रपती-शिवाजी-महाराज-यांचे-जीवनचरित्र-त्यांचा-महाराष्ट्रातील-योगदान","rules_applied":["Truncated to 80 chars","conjunction (आणि) removed"]},{"input":"पाककृती: मसाला वडा पाव!","output":"पाककृती-मसाला-वडा-पाव","rules_applied":["Special chars removed (: !)","hyphenation"]},{"input":"\"आई\" म्हणजे काय?","output":"आई-म्हणजे","rules_applied":["Quotes removed","question word (काय) removed","question mark removed"]},{"input":"वडा पाव vs. मिसळ पाव","output":"वडा-पाव-मिसळ-पाव","rules_applied":["Latin characters removed (vs.)","periods removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/mr-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ms-BN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'MS (BN) Slugification',
    s.content = 'URL slug generation rules for ms-BN',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"auxiliary":["akan","telah"],"honorific":["baginda"],"conjunction":["dan","atau","agar"],"preposition":["di","ke","dari","untuk","pada","dengan","oleh","bagi","berkenaan","bersama"],"demonstrative":["ini","itu"],"adverb":["juga","saja"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"nya","reason":"clitic"},{"word":"","category":"saja","reason":"adverb"},{"word":"","category":"oleh","reason":"preposition"},{"word":"","category":"bagi","reason":"preposition"},{"word":"","category":"baginda","reason":"honorific"},{"word":"","category":"pihak","reason":"noun"},{"word":"","category":"berkenaan","reason":"preposition"},{"word":"","category":"telah","reason":"auxiliary"},{"word":"","category":"bersama","reason":"preposition"},{"word":"","category":"agar","reason":"conjunction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords (>60%)","message":"Title may be too generic"}]',
    s.examples = '[{"input":"Lawatan ke Masjid Omar Ali Saifuddien","output":"lawatan-masjid-omar-ali-saifuddien","rules_applied":["Stopwords removed: \"ke\""]},{"input":"Falsafah MIB dan Pembangunan Negara Brunei","output":"falsafah-mib-pembangunan-negara-brunei","rules_applied":["Stopwords removed: \"dan\""]},{"input":"Resepi Ambuyat Tradisional untuk Keluarga","output":"resepi-ambuyat-tradisional-keluarga","rules_applied":["Stopwords removed: \"untuk\""]},{"input":"Nasi Katok: Makanan Rakyat Brunei yang Popular","output":"nasi-katok-makanan-rakyat-brunei-popular","rules_applied":["Stopwords removed: \"yang\"; punctuation removed: \":\""]},{"input":"Meneroka Hutan Temburong Bersama Pemandu","output":"meneroka-hutan-temburong-pemandu","rules_applied":["Stopwords removed: \"bersama\""]},{"input":"10 Aktiviti Menarik di Kampong Ayer","output":"10-aktiviti-menarik-kampong-ayer","rules_applied":["Stopwords removed: \"di\"; numbers preserved"]},{"input":"Perkhidmatan Awam Kerajaan oleh Pihak Berkuasa Negara Brunei Darussalam untuk Rakyat dan Penduduk Tetap","output":"perkhidmatan-awam-kerajaan-berkuasa-negara-brunei-darussalam-rakyat-penduduk","rules_applied":["Stopwords removed: \"oleh\"","\"pihak\"","\"untuk\"","\"dan\"; truncated to 80 chars"]},{"input":"Sambutan Hari Kebangsaan 2024 & Perayaan Agama","output":"sambutan-hari-kebangsaan-2024-perayaan-agama","rules_applied":["Ampersand removed; numbers preserved"]},{"input":"\"Warisan Islam\" - Istana Nurul Iman","output":"warisan-islam-istana-nurul-iman","rules_applied":["Quotes removed; hyphen normalized"]},{"input":"Jerudong Park @ Pelancongan Brunei","output":"jerudong-park-pelancongan-brunei","rules_applied":["At symbol removed: \"@\" stripped"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ms-BN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ms-MY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Malay (Malaysia) Slugification',
    s.content = 'URL slug generation rules for ms-MY',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"negation":["tak"],"preposition":["di","ke","dari","untuk","pada","dengan","kat"],"demonstrative":["ini","itu"],"conjunction":["dan","atau"],"auxiliary":["akan","nak"],"adverb":["juga","gak","je","sahaja"]}',
    s.stopwords_count = 18,
    s.regional_additions = '[{"word":"","category":"lah","reason":"particle"},{"word":"","category":"kan","reason":"particle"},{"word":"","category":"mah","reason":"particle"},{"word":"","category":"meh","reason":"particle"},{"word":"","category":"lor","reason":"particle"},{"word":"","category":"kah","reason":"particle"},{"word":"","category":"kot","reason":"particle"},{"word":"","category":"gak","reason":"adverb"},{"word":"","category":"je","reason":"adverb"},{"word":"","category":"kat","reason":"preposition"},{"word":"","category":"sahaja","reason":"adverb"},{"word":"","category":"pun","reason":"particle"},{"word":"","category":"nak","reason":"auxiliary"},{"word":"","category":"tak","reason":"negation"},{"word":"","category":"nya","reason":"clitic"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords (>60%)","message":"Title may be too generic"}]',
    s.examples = '[{"input":"Panduan Grab Malaysia untuk Pengguna Baru","output":"panduan-grab-malaysia-pengguna-baru","rules_applied":["Stopwords removed: \"untuk\""]},{"input":"Cara Guna Touch n Go eWallet di Shopee","output":"cara-guna-touch-n-go-ewallet-shopee","rules_applied":["Stopwords removed: \"di\"; special chars cleaned"]},{"input":"Resepi Nasi Lemak Paling Sedap di KL","output":"resepi-nasi-lemak-paling-sedap-kl","rules_applied":["Stopwords removed: \"di\""]},{"input":"Tempat Menarik di Kuala Lumpur dan KLCC","output":"tempat-menarik-kuala-lumpur-klcc","rules_applied":["Stopwords removed: \"di\"","\"dan\""]},{"input":"Percutian Penang: George Town Heritage Trail","output":"percutian-penang-george-town-heritage-trail","rules_applied":["Special chars removed: \":\""]},{"input":"10 Resort Terbaik di Langkawi untuk 2025","output":"10-resort-terbaik-langkawi-2025","rules_applied":["Numbers preserved; stopwords removed: \"di\"","\"untuk\""]},{"input":"Panduan Lengkap Pemasaran Digital untuk Perniagaan Kecil dan Sederhana di Malaysia dengan Shopee dan Lazada","output":"panduan-lengkap-pemasaran-digital-perniagaan-kecil-sederhana-malaysia-shopee","rules_applied":["Truncated at 80 chars; stopwords removed: \"untuk\"","\"dan\"","\"di\"","\"dengan\""]},{"input":"Celcom vs Maxis: Pakej Data Terbaik 2025!","output":"celcom-vs-maxis-pakej-data-terbaik-2025","rules_applied":["Special chars removed: \":\"","\"!\""]},{"input":"Maybank2u \"Secure2u\" - Aplikasi Perbankan Digital","output":"maybank2u-secure2u-aplikasi-perbankan-digital","rules_applied":["Quotes and dash normalized"]},{"input":"Promosi E-Dompet @ Genting Highlands","output":"promosi-e-dompet-genting-highlands","rules_applied":["At symbol removed; hyphen in \"e-dompet\" preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ms-MY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ms-SG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'MS (SG) Slugification',
    s.content = 'URL slug generation rules for ms-SG',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["dan","atau"],"possessive":["punya"],"demonstrative":["ini","itu"],"negation":["tidak"],"preposition":["di","ke","dari","untuk","dengan","pada"],"auxiliary":["akan"],"adverb":["juga"]}',
    s.stopwords_count = 14,
    s.regional_additions = '[{"word":"","category":"lah","reason":"particle"},{"word":"","category":"lor","reason":"particle"},{"word":"","category":"hor","reason":"particle"},{"word":"","category":"leh","reason":"particle"},{"word":"","category":"kan","reason":"particle"},{"word":"","category":"punya","reason":"possessive"},{"word":"","category":"macam","reason":"intensifier"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO value"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Contains only numbers","message":"Likely missing descriptive content"},{"condition":"English-heavy slug","message":"Consider if Malay content appropriate"}]',
    s.examples = '[{"input":"Gerai Makanan Halal di Geylang Serai","output":"gerai-makanan-halal-geylang-serai","rules_applied":["Stopwords removed: \"di\""]},{"input":"Aktiviti Hari Raya Puasa di Kampong Glam","output":"aktiviti-hari-raya-puasa-kampong-glam","rules_applied":["Stopwords removed: \"di\""]},{"input":"Panduan Memohon Kerja Singtel 2024","output":"panduan-memohon-kerja-singtel-2024","rules_applied":["Numbers preserved","Singapore brand"]},{"input":"5 Kedai Mee Siam Terbaik Bedok","output":"5-kedai-mee-siam-terbaik-bedok","rules_applied":["Numbers preserved","local food + location"]},{"input":"Sewa Flat HDB Tampines dan Woodlands","output":"sewa-flat-hdb-tampines-woodlands","rules_applied":["Stopwords removed: \"dan\"","Singapore locations"]},{"input":"Resepi Lontong Singapura yang Sedap Lah","output":"resepi-lontong-singapura-sedap","rules_applied":["Stopwords removed: \"yang\"","\"lah\" (Singlish particle)"]},{"input":"Panduan Lengkap Makan Satay Shiok di Changi Village untuk Pelawat Singapura yang Mencari Makanan Tempatan","output":"panduan-lengkap-makan-satay-shiok-changi-village-pelawat-singapura-mencari","rules_applied":["Truncated 80 chars","stopwords: \"di\"","\"untuk\"","\"yang\""]},{"input":"DBS & OCBC: Bank Terbaik untuk Akaun Simpanan!","output":"dbs-ocbc-bank-terbaik-akaun-simpanan","rules_applied":["Special chars removed: \"&\"","\":\"","\"!\"; stopword: \"untuk\""]},{"input":"\"Chingay\" - Perarakan Hari Kebangsaan Singapura","output":"chingay-perarakan-hari-kebangsaan-singapura","rules_applied":["Quotes removed","Singapore National Day event"]},{"input":"Restoran Halal @ Marina Bay Sands Lor","output":"restoran-halal-marina-bay-sands","rules_applied":["At symbol removed; stopword: \"lor\" (Singlish particle)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ms-SG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'mt-MT'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'MT (MT) Slugification',
    s.content = 'URL slug generation rules for mt-MT',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"demonstrative":["dawn","dawk","dan","din"],"adverb":["hemm","hawn"],"quantifier":["kull"],"pronoun":["kollox"],"indefinite":["xi"]}',
    s.stopwords_count = 9,
    s.regional_additions = '[{"word":"","category":"dawn","reason":"demonstrative"},{"word":"","category":"dawk","reason":"demonstrative"},{"word":"","category":"dan","reason":"demonstrative"},{"word":"","category":"din","reason":"demonstrative"},{"word":"","category":"hemm","reason":"adverb"},{"word":"","category":"hawn","reason":"adverb"},{"word":"","category":"kollox","reason":"pronoun"},{"word":"","category":"xi","reason":"indefinite"},{"word":"","category":"kull","reason":"quantifier"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters"}]',
    s.examples = '[{"input":"Gwida għall-Belt Valletta","output":"gwida-belt-valletta","rules_applied":["Stopwords removed (għall)","diacritics preserved"]},{"input":"Il-Festi Maltin tal-Imnarja","output":"festi-maltin-imnarja","rules_applied":["Stopwords removed (il","tal)","lowercase"]},{"input":"L-Aqwa Ristoranti fi Malta","output":"aqwa-ristoranti-malta","rules_applied":["Stopwords removed (l","fi)","lowercase"]},{"input":"Kif Tħejji l-Pastizzi Tradizzjonali","output":"kif-tħejji-pastizzi-tradizzjonali","rules_applied":["Maltese ħ preserved","stopword removed (l)"]},{"input":"Għażliet ta\' Lukandi f\'Għawdex","output":"għażliet-lukandi-għawdex","rules_applied":["Maltese għ and ż preserved","stopwords removed (ta\'","f\')"]},{"input":"10 Postijiet Storiċi għaż-Żjara","output":"10-postijiet-storiċi-żjara","rules_applied":["Numbers preserved","ċ and ż preserved","stopword removed (għaż)"]},{"input":"L-Istorja tal-Kavallieri ta\' San Ġwann u l-Bini tal-Belt Valletta mill-1566 sal-Lum","output":"l-istorja-kavallieri-san-ġwann-bini-belt-valletta-mill-1566-sal-lum","rules_applied":["Long title truncated at 80 chars","multiple stopwords removed"]},{"input":"Ikel & Xorb: Riċetti Maltin!","output":"ikel-xorb-riċetti-maltin","rules_applied":["Special chars removed (&",":","!)","ċ preserved"]},{"input":"X\'inhu l-Aħjar Żmien għall-Vaganzi f\'Malta?","output":"xinhu-aħjar-żmien-vaganzi-malta","rules_applied":["Question mark removed","ħ and ż preserved","stopwords removed (l","għall","f\')"]},{"input":"Għaliex Malta Għandha l-Isbaħ Baħar fl-Ewropa","output":"għaliex-malta-għandha-isbaħ-baħar-ewropa","rules_applied":["Multiple għ and ħ preserved","stopword removed (l","fl)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/mt-MT.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'my-MM'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'MY (MM) Slugification',
    s.content = 'URL slug generation rules for my-MM',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[{"word":"","category":"ပါ","reason":"politeness particle"},{"word":"","category":"ပါတယ်","reason":"verb ending (polite)"},{"word":"","category":"တဲ့","reason":"relative clause marker"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful content"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Unexpected unless technical terms present"},{"condition":"Non-standard Unicode composition","message":"Check for proper NFC normalization"}]',
    s.examples = '[{"input":"မြန်မာနိုင်ငံ ခရီးသွားလမ်းညွှန်","output":"မြန်မာနိုင်ငံ-ခရီးသွားလမ်းညွှန်","rules_applied":["NFC","spaces to hyphens"]},{"input":"ရန်ကုန်မြို့ရှိ အကောင်းဆုံး စားသောက်ဆိုင်များ","output":"ရန်ကုန်မြို့ရှိ-အကောင်းဆုံး-စားသောက်ဆိုင်များ","rules_applied":["NFC","spaces to hyphens"]},{"input":"ပုဂံ ဘုရားများ နှင့် စေတီများ","output":"ပုဂံ-ဘုရားများ-စေတီများ","rules_applied":["NFC","stopword နှင့် removed"]},{"input":"မန္တလေးတောင် အမွေအနှစ် ဒေသ","output":"မန္တလေးတောင်-အမွေအနှစ်-ဒေသ","rules_applied":["NFC","spaces to hyphens"]},{"input":"အင်းလေးကန် ခရီးစဉ် လမ်းညွှန်","output":"အင်းလေးကန်-ခရီးစဉ်-လမ်းညွှန်","rules_applied":["NFC","spaces to hyphens"]},{"input":"၂၀၂၅ ခုနှစ် မြန်မာ့ရိုးရာ သင်္ကြန်ပွဲတော်","output":"၂၀၂၅-ခုနှစ်-မြန်မာ့ရိုးရာ-သင်္ကြန်ပွဲတော်","rules_applied":["NFC","Myanmar digits preserved"]},{"input":"ရှမ်းပြည်နယ်ရှိ တောင်တန်းဒေသများအတွက် ခရီးသွားလမ်းညွှန်ချက်များ နှင့် အကြံပြုချက်များ","output":"ရှမ်းပြည်နယ်ရှိ-တောင်တန်းဒေသများ-ခရီးသွားလမ်းညွှန်ချက်များ-အကြံပြုချက်များ","rules_applied":["Truncated to 80 chars","stopwords removed"]},{"input":"မြန်မာ့ရိုးရာ ထမင်းချက်နည်း။ အရသာရှိသော ဟင်းလျာများ!","output":"မြန်မာ့ရိုးရာ-ထမင်းချက်နည်း-အရသာရှိသော-ဟင်းလျာများ","rules_applied":["Myanmar punctuation ။ ! removed"]},{"input":"\"ရွှေတိဂုံ\" ဘုရားကြီး၏ သမိုင်း","output":"ရွှေတိဂုံ-ဘုရားကြီး-သမိုင်း","rules_applied":["Quotes removed","possessive ၏ removed"]},{"input":"ဗမာ့ English စကားပြော","output":"ဗမာ့-english-စကားပြော","rules_applied":["Mixed script preserved (loan words common)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/my-MM.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ne-NP'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'NE (NP) Slugification',
    s.content = 'URL slug generation rules for ne-NP',
    s.slug_rule = 'native_script',
    s.stopwords = '{"pronoun":["यो","त्यो","यी","ती","कुनै"],"adverb":["पनि"],"conjunction":["र","तथा","वा","तर"],"verb":["हो","छ","थियो","गर्नु","हुनु","भयो"]}',
    s.stopwords_count = 16,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Devanagari with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"},{"condition":"Broken conjuncts","message":"Verify halant+consonant sequences are preserved"}]',
    s.examples = '[{"input":"नेपाल भ्रमण गाइड","output":"नेपाल-भ्रमण-गाइड","rules_applied":["Spaces to hyphens","NFC normalization"]},{"input":"काठमाडौं को इतिहास","output":"काठमाडौं-इतिहास","rules_applied":["Postposition removed (को)","hyphenation"]},{"input":"हिमालय र प्रकृति","output":"हिमालय-प्रकृति","rules_applied":["Conjunction removed (र)","hyphenation"]},{"input":"नेपाली खाना बनाउने तरिका","output":"नेपाली-खाना-बनाउने-तरिका","rules_applied":["Basic slug generation","conjuncts preserved"]},{"input":"पोखरा मा घुम्ने ठाउँहरू","output":"पोखरा-घुम्ने-ठाउँहरू","rules_applied":["Postposition removed (मा)","chandrabindu preserved"]},{"input":"२०८१ सालको दसैं तिहार","output":"२०८१-सालको-दसैं-तिहार","rules_applied":["Devanagari numerals preserved"]},{"input":"नेपालको संस्कृति र परम्परा: पूर्वी हिमालयी क्षेत्रमा बस्ने विभिन्न जातजातिहरूको जीवनशैली बारे जानकारी","output":"नेपालको-संस्कृति-परम्परा-पूर्वी-हिमालयी-क्षेत्रमा-बस्ने-विभिन्न-जातजातिहरूको","rules_applied":["Truncated to 80 chars","conjunction removed (र)","colon removed"]},{"input":"नेपाली चिया & खाजा: स्वादिष्ट!","output":"नेपाली-चिया-खाजा-स्वादिष्ट","rules_applied":["Special chars removed (&",":","!)","conjunct preserved"]},{"input":"\"सगरमाथा\" वा \"एभरेस्ट\" कुन नाम?","output":"सगरमाथा-एभरेस्ट-कुन-नाम","rules_applied":["Quotes removed","conjunction (वा) removed","question mark removed"]},{"input":"बौद्धनाथ vs. पशुपतिनाथ मन्दिर","output":"बौद्धनाथ-पशुपतिनाथ-मन्दिर","rules_applied":["Latin chars (vs.) removed","dot removed","conjuncts preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ne-NP.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'nl-BE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'NL (BE) Slugification',
    s.content = 'URL slug generation rules for nl-BE',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["de","het","een","ne"],"preposition":["van","in","op","met","voor","aan","te","naar","bij","uit","om","over"],"conjunction":["en","of","maar","als"],"verb":["is","zijn","wordt"],"pronoun":["dat","die","dit","ge","gij","wa"]}',
    s.stopwords_count = 29,
    s.regional_additions = '[{"word":"","category":"ne","reason":"article"},{"word":"","category":"ge","reason":"pronoun"},{"word":"","category":"gij","reason":"pronoun"},{"word":"","category":"wa","reason":"pronoun"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Review for consistency"}]',
    s.examples = '[{"input":"De Beste Frieten van Brussel","output":"beste-frieten-brussel","rules_applied":["stopwords: de","van; lowercase"]},{"input":"Het Grote Markt in Antwerpen","output":"grote-markt-antwerpen","rules_applied":["stopwords: het","in; lowercase"]},{"input":"Café met Terras en Parking","output":"café-terras-parking","rules_applied":["stopwords: met","en; preserve é"]},{"input":"Winkelen op de Meir","output":"winkelen-meir","rules_applied":["stopwords: op","de; lowercase"]},{"input":"Appartement te Huur in Gent","output":"appartement-huur-gent","rules_applied":["stopwords: te","in; lowercase"]},{"input":"Top 10 Restaurants in België","output":"top-10-restaurants-belgië","rules_applied":["stopwords: in; preserve number","ë"]},{"input":"De Allerbeste Tips voor een Geslaagde Vakantie aan de Belgische Kust met het Hele Gezin","output":"allerbeste-tips-geslaagde-vakantie-belgische-kust-hele-gezin","rules_applied":["truncation at 80 chars; multiple stopwords"]},{"input":"Brugge: Een Stad met Geschiedenis!","output":"brugge-stad-geschiedenis","rules_applied":["punctuation removed; stopwords: een","met"]},{"input":"\"Manneken Pis\" & het Atomium","output":"manneken-pis-atomium","rules_applied":["quotes removed; & removed; stopwords: het"]},{"input":"Coördinatie van Naïeve Reële Projecten","output":"coördinatie-naïeve-reële-projecten","rules_applied":["preserve ö","ï","ë; stopwords: van"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/nl-BE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'nl-NL'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Dutch (Netherlands) Slugification',
    s.content = 'URL slug generation rules for nl-NL',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["deze","die","dit","dat","zij"],"conjunction":["en","of","maar","als"],"verb":["is","zijn","was","waren"],"preposition":["van","in","op","voor","met","aan","bij","door","naar","uit","over","om","na","ter","tot","bij"],"adverb":["ook","nog"],"article":["de","het","een","der","des"]}',
    s.stopwords_count = 36,
    s.regional_additions = '[{"word":"","category":"der","reason":"article"},{"word":"","category":"des","reason":"article"},{"word":"","category":"ter","reason":"preposition"},{"word":"","category":"tot","reason":"preposition"},{"word":"","category":"bij","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Unexpected non-Latin characters"}]',
    s.examples = '[{"input":"De beste fietsroutes in Nederland","output":"beste-fietsroutes-nederland","rules_applied":["Stopwords removed (de","in)","lowercase","spaces→hyphens"]},{"input":"Café op de hoek van de straat","output":"café-hoek-straat","rules_applied":["Diacritics preserved","stopwords removed (op","de","van","de)"]},{"input":"Amsterdam & Rotterdam: top 10 bezienswaardigheden","output":"amsterdam-rotterdam-top-10-bezienswaardigheden","rules_applied":["Special char (&",":) removed","stopwords removed"]},{"input":"Het IJsselmeer en de Waddeneilanden","output":"ijsselmeer-waddeneilanden","rules_applied":["IJ→ij lowercase","stopwords removed (het","en","de)"]},{"input":"Ontdek de mooiste plekken voor een weekend","output":"ontdek-mooiste-plekken-weekend","rules_applied":["Stopwords removed (de","voor","een)"]},{"input":"25 tips voor het perfecte verjaardagsfeest 2025","output":"25-tips-perfecte-verjaardagsfeest-2025","rules_applied":["Numbers kept","stopwords removed (voor","het)"]},{"input":"Hoe maak je een authentieke stroopwafel zoals oma die maakte vroeger in Utrecht?","output":"hoe-maak-authentieke-stroopwafel-oma-maakte-vroeger-utrecht","rules_applied":["Long title","stopwords removed (je","een","zoals","die)","truncated if >80 chars"]},{"input":"Wat is jouw mening? Praat mee!","output":"wat-mening-praat-mee","rules_applied":["Punctuation (?) removed","stopwords removed (is","jouw)"]},{"input":"Sinterklaas\' cadeautjes: ideeën & inspiratie","output":"sinterklaas-cadeautjes-ideeën-inspiratie","rules_applied":["Apostrophe removed","diacritics preserved (ë)","stopwords removed (&",":)"]},{"input":"Géïllustreerde gids über Nederlandse koffiecultuur","output":"géïllustreerde-gids-über-nederlandse-koffiecultuur","rules_applied":["Multiple diacritics preserved (é","ï","ü)","edge case with mixed diacritics"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/nl-NL.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'no-NO'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Norwegian (Norway) Slugification',
    s.content = 'URL slug generation rules for no-NO',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["ved","over","under","etter"]}',
    s.stopwords_count = 4,
    s.regional_additions = '[{"word":"","category":"ved","reason":"preposition"},{"word":"","category":"over","reason":"preposition"},{"word":"","category":"under","reason":"preposition"},{"word":"","category":"etter","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"},{"condition":"No Norwegian chars","message":"May indicate encoding issue if expected"}]',
    s.examples = '[{"input":"Beste måten å lære norsk på","output":"beste-måten-lære-norsk","rules_applied":["Stopwords removed (å","på)","æ preserved"]},{"input":"Guide til Oslo 2025","output":"guide-oslo-2025","rules_applied":["Stopword removed (til)","number kept"]},{"input":"Hvordan lage tradisjonell fårikål","output":"hvordan-lage-tradisjonell-fårikål","rules_applied":["å preserved in fårikål"]},{"input":"10 tips for å spare penger","output":"10-tips-spare-penger","rules_applied":["Stopwords removed (for","å)","number prefix"]},{"input":"Norges mest populære øyer og fjorder","output":"norges-mest-populære-øyer-fjorder","rules_applied":["Stopword removed (og)","ø preserved"]},{"input":"\"Smaken av Norge\" – en kulinarisk reise","output":"smaken-norge-kulinarisk-reise","rules_applied":["Quotes removed","stopwords (av","en) removed"]},{"input":"Trondheim & Bergen: sammenligning av storbyene i Norge","output":"trondheim-bergen-sammenligning-storbyene-norge","rules_applied":["Ampersand removed","stopwords (av","i) removed"]},{"input":"Å investere i eiendom – det du må vite om boligmarkedet i Oslo","output":"investere-eiendom-må-vite-boligmarkedet-oslo","rules_applied":["Long title","multiple stopwords removed","å from infinitive marker removed"]},{"input":"Historien bak vikingskipene: Oseberg, Gokstad og Tune","output":"historien-bak-vikingskipene-oseberg-gokstad-tune","rules_applied":["Colon removed","stopword (og) removed"]},{"input":"Hva koster det å bo i Stavanger?","output":"hva-koster-bo-stavanger","rules_applied":["Question mark removed","stopwords (det","å","i) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/no-NO.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ny-MW'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'NY (MW) Slugification',
    s.content = 'URL slug generation rules for ny-MW',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"interrogative":["ndani","chiyani"]}',
    s.stopwords_count = 2,
    s.regional_additions = '[{"word":"","category":"boma","reason":"noun"},{"word":"","category":"ndani","reason":"interrogative"},{"word":"","category":"chiyani","reason":"interrogative"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Stopwords removed > 60%","message":"Title may be too generic"},{"condition":"All diacritics stripped","message":"Verify slug remains meaningful"}]',
    s.examples = '[{"input":"Uthenga Wabwino wa Moyo","output":"uthenga-wabwino-moyo","rules_applied":["lowercase","stopwords (wa)","hyphen"]},{"input":"Malangizo a Zaumoyo ndi Thanzi","output":"malangizo-zaumoyo-thanzi","rules_applied":["lowercase","stopwords (a","ndi)","hyphen"]},{"input":"Nkhani za Malawi Lero","output":"nkhani-malawi-lero","rules_applied":["lowercase","stopwords (za)","hyphen"]},{"input":"Masewera a Mpira ku Lilongwe","output":"masewera-mpira-lilongwe","rules_applied":["lowercase","stopwords (a","ku)","hyphen"]},{"input":"Maphunziro a Ana Aang\'ono","output":"maphunziro-ana-aangono","rules_applied":["lowercase","stopwords (a)","strip diacritics","hyphen"]},{"input":"Zinthu 10 Zofunikira pa Nyumba","output":"zinthu-10-zofunikira-nyumba","rules_applied":["lowercase","numbers kept","stopwords (pa)","hyphen"]},{"input":"Njira Zopezera Ndalama Zambiri mu Chaka cha 2026 Kupyolera pa Malonda a pa Intaneti ku Malaŵi","output":"njira-zopezera-ndalama-zambiri-chaka-2026-kupyolera-malonda-intaneti-malawi","rules_applied":["truncation at 80 chars","stopwords (mu","cha","pa","a","ku)","strip w accent"]},{"input":"Zakudya za ku Malaŵi: Nsima ndi Ndiwo!","output":"zakudya-malawi-nsima-ndiwo","rules_applied":["punctuation removed","stopwords (za","ku","ndi)","strip w accent"]},{"input":"\"Chikondi\" ndi Mtendere mu Banja","output":"chikondi-mtendere-banja","rules_applied":["quotes removed","stopwords (ndi","mu)","hyphen"]},{"input":"Ŵanthu a ku Nyanja: Chikhalidwe Chawo","output":"wanthu-nyanja-chikhalidwe-chawo","rules_applied":["strip w accent","stopwords (a","ku)","colon removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ny-MW.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'or-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'OR (IN) Slugification',
    s.content = 'URL slug generation rules for or-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[]',
    s.script_config = '{"primary_script":"odia","diacritic_handling":null,"numeral_handling":"preserve","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"}]',
    s.examples = '[]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/or-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'pa-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'PA (IN) Slugification',
    s.content = 'URL slug generation rules for pa-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"honorific":["ਜੀ"]}',
    s.stopwords_count = 1,
    s.regional_additions = '[{"word":"","category":"ਜੀ","reason":"honorific"},{"word":"","category":"ਜਿਵੇਂ","reason":"comparative"},{"word":"","category":"ਵਾਲਾ","reason":"possessive suffix"},{"word":"","category":"ਵਾਲੀ","reason":"possessive suffix"},{"word":"","category":"ਵਾਲੇ","reason":"possessive suffix"},{"word":"","category":"ਰਾਹੀਂ","reason":"postposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Gurmukhi + Latin mixed (acceptable for technical terms)"},{"condition":"Non-standard composition","message":"Invalid Unicode composition detected"}]',
    s.examples = '[{"input":"ਪੰਜਾਬ ਵਿੱਚ ਯਾਤਰਾ ਗਾਈਡ","output":"ਪੰਜਾਬ-ਯਾਤਰਾ-ਗਾਈਡ","rules_applied":["Stopwords (ਵਿੱਚ) removed"]},{"input":"ਪੰਜਾਬੀ ਸਿੱਖਣ ਦੇ ਲਈ ਟਿਪਸ","output":"ਪੰਜਾਬੀ-ਸਿੱਖਣ-ਟਿਪਸ","rules_applied":["Stopwords (ਦੇ","ਲਈ) removed"]},{"input":"ਅੰਮ੍ਰਿਤਸਰ ਦਾ ਇਤਿਹਾਸ ਅਤੇ ਸੱਭਿਆਚਾਰ","output":"ਅੰਮ੍ਰਿਤਸਰ-ਇਤਿਹਾਸ-ਸੱਭਿਆਚਾਰ","rules_applied":["Stopwords (ਦਾ","ਅਤੇ) removed"]},{"input":"ਚੰਡੀਗੜ੍ਹ ਦੀ ਸਭ ਤੋਂ ਵਧੀਆ ਰੈਸਟੋਰੈਂਟ","output":"ਚੰਡੀਗੜ੍ਹ-ਸਭ-ਵਧੀਆ-ਰੈਸਟੋਰੈਂਟ","rules_applied":["Stopwords (ਦੀ","ਤੋਂ) removed"]},{"input":"ਪੰਜਾਬੀ ਪਕਵਾਨ ਰੈਸਿਪੀ","output":"ਪੰਜਾਬੀ-ਪਕਵਾਨ-ਰੈਸਿਪੀ","rules_applied":["No stopwords","clean slug"]},{"input":"ਤਕਨਾਲੋਜੀ ਖ਼ਬਰਾਂ 2025","output":"ਤਕਨਾਲੋਜੀ-ਖ਼ਬਰਾਂ-2025","rules_applied":["Numbers preserved"]},{"input":"ਇਹ ਹੈ ਪੰਜਾਬ ਦਾ ਸਭ ਤੋਂ ਵੱਡਾ ਗੁਰਦੁਆਰਾ ਜੋ ਦੁਨੀਆ ਵਿੱਚ ਮਸ਼ਹੂਰ ਹੈ","output":"ਪੰਜਾਬ-ਸਭ-ਵੱਡਾ-ਗੁਰਦੁਆਰਾ-ਦੁਨੀਆ-ਮਸ਼ਹੂਰ","rules_applied":["Long title truncated","stopwords removed"]},{"input":"ਸਿਹਤ ਅਤੇ ਤੰਦਰੁਸਤੀ: ਕਸਰਤ ਟਿਪਸ!","output":"ਸਿਹਤ-ਤੰਦਰੁਸਤੀ-ਕਸਰਤ-ਟਿਪਸ","rules_applied":["Punctuation removed","stopwords (ਅਤੇ) removed"]},{"input":"\"ਪੰਜਾਬ ਦੀ ਆਵਾਜ਼\" - ਰੇਡੀਓ ਪ੍ਰੋਗਰਾਮ","output":"ਪੰਜਾਬ-ਆਵਾਜ਼-ਰੇਡੀਓ-ਪ੍ਰੋਗਰਾਮ","rules_applied":["Quotes removed","stopwords (ਦੀ) removed"]},{"input":"ਟੈਕਨਾਲੋਜੀ & Innovation ਹੱਬ","output":"ਟੈਕਨਾਲੋਜੀ-innovation-ਹੱਬ","rules_applied":["Ampersand removed","mixed Gurmukhi-Latin accepted"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/pa-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'pa-PK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'PA (PK) Slugification',
    s.content = 'URL slug generation rules for pa-PK',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[{"word":"","category":"پاکستان","reason":"noun"},{"word":"","category":"پنجاب","reason":"noun"},{"word":"","category":"لاہور","reason":"noun"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":"preserve_arabic_indic","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Shahmukhi mixed with non-Shahmukhi (except numerals)"},{"condition":"Diacritics present","message":"Zabar/zer/pesh should be removed for consistency"},{"condition":"Non-standard composition","message":"Check Unicode normalization"}]',
    s.examples = '[{"input":"پنجابی کھانے دی گائیڈ","output":"پنجابی-کھانے-گائیڈ","rules_applied":["Stopword (دی) removed"]},{"input":"لاہور وچ بہترین جگہاں","output":"لاہور-بہترین-جگہاں","rules_applied":["Stopword (وچ) removed"]},{"input":"کرکٹ دے بارے سب کجھ جانو","output":"کرکٹ-بارے-سب-کجھ-جانو","rules_applied":["Stopword (دے) removed"]},{"input":"پاکستان دا سیاحتی گائیڈ","output":"سیاحتی-گائیڈ","rules_applied":["Stopwords (پاکستان","دا) removed"]},{"input":"ہفتہ وار بازار نال خریداری","output":"ہفتہ-وار-بازار-خریداری","rules_applied":["Stopword (نال) removed"]},{"input":"پنجابی موسیقی 2024 دیاں بہترین گیتاں","output":"پنجابی-موسیقی-2024-بہترین-گیتاں","rules_applied":["Number preserved","stopwords removed"]},{"input":"پنجابی زبان سکھن لئی مکمل گائیڈ جیہڑی تہاڈے لئی بوہت فائدہ مند ہووے گی ہمیشہ","output":"پنجابی-زبان-سکھن-مکمل-گائیڈ-جیہڑی-تہاڈے-بوہت-فائدہ-مند-ہووے-ہمیشہ","rules_applied":["Long title","stopword (لئی) removed twice","truncated at 80 chars"]},{"input":"پنجابی ادب & شاعری: چنگے شاعر!","output":"پنجابی-ادب-شاعری-چنگے-شاعر","rules_applied":["Special chars (&",":","!) removed"]},{"input":"\"سوہنی مہیوال\" - پنجابی لوک کہانی","output":"سوہنی-مہیوال-پنجابی-لوک-کہانی","rules_applied":["Quotes and dash removed"]},{"input":"سجی ہوئی گڈی (ٹرک آرٹ) دی روایت","output":"سجی-ہوئی-گڈی-ٹرک-آرٹ-روایت","rules_applied":["Parentheses removed","stopword (دی) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/pa-PK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'pl-PL'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Polish (Poland) Slugification',
    s.content = 'URL slug generation rules for pl-PL',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["w","z","do","na","o","od","po","dla","ze","we","przy","przez","pod","za"],"negation":["nie"],"conjunction":["i","a","oraz","czy","ale"],"pronoun":["to","się"],"verb":["jest"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short - consider longer title"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Ensure consistent script usage"},{"condition":"All stopwords removed","message":"Title contained only stopwords"}]',
    s.examples = '[{"input":"Najlepsze kawiarnie w Warszawie","output":"najlepsze-kawiarnie-warszawie","rules_applied":["Stopword \"w\" removed","lowercase"]},{"input":"Jak gotować tradycyjne polskie pierogi?","output":"jak-gotować-tradycyjne-polskie-pierogi","rules_applied":["Punctuation removed","diacritics preserved"]},{"input":"10 miejsc do zwiedzenia w Krakowie","output":"10-miejsc-zwiedzenia-krakowie","rules_applied":["Numbers kept","stopwords \"do\" and \"w\" removed"]},{"input":"Technologia & Innowacje 2025","output":"technologia-innowacje-2025","rules_applied":["Ampersand removed","year kept"]},{"input":"Przewodnik po \"Zdrowym Żywieniu\"","output":"przewodnik-zdrowym-żywieniu","rules_applied":["Stopword \"po\" removed","quotes removed","diacritics preserved"]},{"input":"Najnowsze wiadomości ze świata polityki","output":"najnowsze-wiadomości-świata-polityki","rules_applied":["Stopwords \"ze\" removed","ś and ą preserved"]},{"input":"Bardzo długi tytuł artykułu o historii Polski i jej wpływie na kulturę europejską w XX wieku","output":"bardzo-długi-tytuł-artykułu-historii-polski-jej-wpływie-kulturę-europejską-x","rules_applied":["Truncated at 80 chars","multiple stopwords removed"]},{"input":"Łódź - miasto przemysłu, sztuki i kultury!","output":"łódź-miasto-przemysłu-sztuki-kultury","rules_applied":["Punctuation removed","ł preserved","stopword \"i\" removed"]},{"input":"Co to jest \"sztuczna inteligencja\"?","output":"sztuczna-inteligencja","rules_applied":["Stopwords \"co\"","\"to\"","\"jest\" removed","quotes removed"]},{"input":"Małe ą, duże Ą, średnie ę vs Ę","output":"małe-ą-duże-ą-średnie-ę-vs-ę","rules_applied":["All Polish diacritics preserved in lowercase"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/pl-PL.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ps-AF'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'PS (AF) Slugification',
    s.content = 'URL slug generation rules for ps-AF',
    s.slug_rule = 'native_script',
    s.stopwords = '{"adverb":["هم","بس"],"conjunction":["خو"]}',
    s.stopwords_count = 3,
    s.regional_additions = '[{"word":"","category":"خو","reason":"conjunction"},{"word":"","category":"هم","reason":"adverb"},{"word":"","category":"بس","reason":"adverb"},{"word":"","category":"نور","reason":"adjective"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Mixed Arabic/Latin detected (except numbers)"},{"condition":"Non-standard Unicode","message":"Non-NFC composition detected"}]',
    s.examples = '[{"input":"د افغانستان تاریخ","output":"افغانستان-تاریخ","rules_applied":["Stopwords removed (د)","spaces to hyphens"]},{"input":"پښتو ژبې زده کړه","output":"پښتو-ژبې-زده-کړه","rules_applied":["Native Pashto script preserved"]},{"input":"کابل ښار ته سفر","output":"کابل-ښار-سفر","rules_applied":["Stopwords removed (ته)","city name preserved"]},{"input":"غوره خواړه او پخلی","output":"غوره-خواړه-پخلی","rules_applied":["Stopwords removed (او)","food terms preserved"]},{"input":"افغان فرهنګ او دودونه","output":"افغان-فرهنګ-دودونه","rules_applied":["Stopwords removed (او)","cultural terms preserved"]},{"input":"۱۰ لارښوونې د بریالیتوب لپاره","output":"۱۰-لارښوونې-بریالیتوب-لپاره","rules_applied":["Persian-Indic digits preserved","stopwords removed (د)"]},{"input":"څنګه کولی شو چې په لږ وخت کې خپل موخې ته ورسیږو او بریالي شو په شخصي او مسلکي ژوند کې","output":"څنګه-کولی-شو-لږ-وخت-کې-خپل-موخې-ورسیږو-بریالي-شو-شخصي-مسلکي-ژوند-کې","rules_applied":["Long title truncated at 80 chars","multiple stopwords removed"]},{"input":"د کندهار ښار: د افغانستان زړه!","output":"کندهار-ښار-افغانستان-زړه","rules_applied":["Punctuation removed (colon","exclamation)","stopwords removed (د)"]},{"input":"کتاب \"د پښتنو تاریخ\" یوه ښه اثر","output":"کتاب-پښتنو-تاریخ-یوه-ښه-اثر","rules_applied":["Quotes removed","stopwords removed (د)"]},{"input":"بند امیر؛ د افغانستان طبیعي ښکلا","output":"بند-امیر-افغانستان-طبیعي-ښکلا","rules_applied":["Pashto semicolon removed","natural landmark preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ps-AF.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'pt-AO'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'PT (AO) Slugification',
    s.content = 'URL slug generation rules for pt-AO',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"adverb":["bué","mais","então"],"verb":["é","ser","estar"],"interjection":["yá"],"preposition":["de","da","do","em","na","no","para","por","com","sem"],"conjunction":["e","ou","mas"],"pronoun":["se"],"article":["o","a","os","as","um","uma"]}',
    s.stopwords_count = 27,
    s.regional_additions = '[{"word":"","category":"bué","reason":"adverb"},{"word":"","category":"yá","reason":"interjection"},{"word":"","category":"kota","reason":"noun"},{"word":"","category":"mais","reason":"adverb"},{"word":"","category":"então","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"}]',
    s.examples = '[{"input":"Guia turístico de Luanda para visitantes","output":"guia-turístico-luanda-visitantes","rules_applied":["Stopwords removed (de","para)","diacritics preserved"]},{"input":"Receitas tradicionais da culinária angolana","output":"receitas-tradicionais-culinária-angolana","rules_applied":["Stopwords removed (da)","diacritics preserved (á)"]},{"input":"Melhores praias de Angola em 2025","output":"melhores-praias-angola-2025","rules_applied":["Stopwords removed (de","em)","number kept"]},{"input":"Música e cultura do Semba","output":"música-cultura-semba","rules_applied":["Stopwords removed (e","do)","diacritics preserved (ú)"]},{"input":"Festival de Kizomba: programação completa","output":"festival-kizomba-programação-completa","rules_applied":["Stopwords removed (de)","colon removed","diacritics preserved"]},{"input":"Top 10 restaurantes em Luanda","output":"top-10-restaurantes-luanda","rules_applied":["Number kept","stopwords removed (em)"]},{"input":"Economia angolana cresce 5% no terceiro trimestre","output":"economia-angolana-cresce-5-terceiro-trimestre","rules_applied":["Stopwords removed (no)","percentage symbol removed"]},{"input":"Parque Nacional da Quiçama: guia & dicas","output":"parque-nacional-quiçama-guia-dicas","rules_applied":["Stopwords removed (da)","ampersand removed","ç preserved"]},{"input":"O que fazer em Benguela durante as férias","output":"fazer-benguela-durante-férias","rules_applied":["Stopwords removed (o","que","em","as)","diacritics preserved"]},{"input":"Exposição de arte contemporânea africana celebra artistas angolanos com obras inéditas","output":"exposição-arte-contemporânea-africana-celebra-artistas-angolanos-obras-inéditas","rules_applied":["Long title","stopwords removed (de","com)","multiple diacritics preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/pt-AO.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'pt-BR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Portuguese (Brazil) Slugification',
    s.content = 'URL slug generation rules for pt-BR',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["se"],"article":["o","a","os","as","um","uma"],"preposition":["de","da","do","em","na","no","para","por","com","sem","pro","pra"],"conjunction":["e","ou","mas"],"filler":["tipo"],"verb":["é","ser","estar","tá"]}',
    s.stopwords_count = 27,
    s.regional_additions = '[{"word":"","category":"pro","reason":"preposition"},{"word":"","category":"pra","reason":"preposition"},{"word":"","category":"tá","reason":"verb"},{"word":"","category":"né","reason":"particle"},{"word":"","category":"tipo","reason":"filler"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"}]',
    s.examples = '[{"input":"Como fazer pão de queijo caseiro","output":"como-fazer-pão-queijo-caseiro","rules_applied":["Stopwords removed (de)","diacritics preserved"]},{"input":"Melhores praias do Brasil em 2025","output":"melhores-praias-brasil-2025","rules_applied":["Stopwords removed (do","em)","number kept"]},{"input":"São Paulo: Guia completo para turistas","output":"são-paulo-guia-completo-turistas","rules_applied":["Tilde preserved","colon removed","stopwords removed (para)"]},{"input":"Receita de brigadeiro & beijinho","output":"receita-brigadeiro-beijinho","rules_applied":["Stopwords removed (de)","ampersand removed"]},{"input":"O que é \"inteligência artificial\"?","output":"inteligência-artificial","rules_applied":["Stopwords removed (o","que","é)","quotes/question mark removed"]},{"input":"Flamengo vence Palmeiras na final","output":"flamengo-vence-palmeiras-final","rules_applied":["Stopwords removed (na)","lowercase applied"]},{"input":"Top 10 cidades brasileiras para visitar","output":"top-10-cidades-brasileiras-visitar","rules_applied":["Number kept","stopwords removed (para)"]},{"input":"Açúcar, café & chocolate: história completa","output":"açúcar-café-chocolate-história-completa","rules_applied":["Multiple diacritics preserved","ampersand/colon removed"]},{"input":"Entenda a diferença entre \"por que\" e \"porque\"","output":"entenda-diferença-entre-por-porque","rules_applied":["Quotes removed","stopwords removed (a","e)","\"por\" kept when part of expression"]},{"input":"Festival de música eletrônica acontece em março próximo ano","output":"festival-música-eletrônica-acontece-março-próximo-ano","rules_applied":["Long title","diacritics preserved (ó","ô","ê)","stopwords removed (de","em)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/pt-BR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'pt-CH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'PT (CH) Slugification',
    s.content = 'URL slug generation rules for pt-CH',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["o","a","os","as","um","uma"],"pronoun":["se"],"verb":["é","ser","estar"],"conjunction":["e","ou","mas"],"preposition":["de","da","do","em","na","no","para","por","com","sem","ao","aos","pelo","pela","neste","nesta"]}',
    s.stopwords_count = 29,
    s.regional_additions = '[{"word":"","category":"ao","reason":"preposition"},{"word":"","category":"aos","reason":"preposition"},{"word":"","category":"pelo","reason":"preposition"},{"word":"","category":"pela","reason":"preposition"},{"word":"","category":"neste","reason":"preposition"},{"word":"","category":"nesta","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Unexpected non-Latin characters detected"}]',
    s.examples = '[{"input":"Comunidade portuguesa na Suíça","output":"comunidade-portuguesa-suíça","rules_applied":["Stopwords removed (na)","diacritics preserved"]},{"input":"Serviços consulares em Zurique","output":"serviços-consulares-zurique","rules_applied":["Stopwords removed (em)","cedilla preserved"]},{"input":"Guia de integração para recém-chegados","output":"guia-integração-recém-chegados","rules_applied":["Stopwords removed (de","para)","hyphen in compound kept"]},{"input":"Associação Cultural Luso-Suíça em Genebra","output":"associação-cultural-luso-suíça-genebra","rules_applied":["Stopwords removed (em)","compound hyphen kept"]},{"input":"Gastronomia portuguesa: bacalhau e pastéis de nata","output":"gastronomia-portuguesa-bacalhau-pastéis-nata","rules_applied":["Colon removed","stopwords removed (e","de)"]},{"input":"Top 5 cidades suíças para viver","output":"top-5-cidades-suíças-viver","rules_applied":["Number kept","stopwords removed (para)"]},{"input":"Educação bilingue português-alemão nas escolas públicas do cantão de Zurique","output":"educação-bilingue-português-alemão-escolas-públicas-cantão-zurique","rules_applied":["Long title truncated","stopwords removed (nas","do","de)"]},{"input":"Eventos culturais lusófonos em Basileia!","output":"eventos-culturais-lusófonos-basileia","rules_applied":["Exclamation removed","stopwords removed (em)"]},{"input":"O fado \"tradicional\" e a saudade portuguesa","output":"fado-tradicional-saudade-portuguesa","rules_applied":["Quotes removed","stopwords removed (o","e","a)"]},{"input":"Câmara de Comércio Luso-Suíça & parcerias económicas","output":"câmara-comércio-luso-suíça-parcerias-económicas","rules_applied":["Ampersand removed","stopwords removed (de)","diacritics preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/pt-CH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'pt-MZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'PT (MZ) Slugification',
    s.content = 'URL slug generation rules for pt-MZ',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["e","ou","mas"],"article":["o","a","os","as","um","uma"],"pronoun":["que","se"],"contraction":["na","no","do","da","ao","pelo","pela"],"preposition":["de","em","para","com","por"],"verb":["é","são"]}',
    s.stopwords_count = 25,
    s.regional_additions = '[{"word":"","category":"na","reason":"contraction"},{"word":"","category":"no","reason":"contraction"},{"word":"","category":"do","reason":"contraction"},{"word":"","category":"da","reason":"contraction"},{"word":"","category":"ao","reason":"contraction"},{"word":"","category":"pelo","reason":"contraction"},{"word":"","category":"pela","reason":"contraction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Verify intentional mixed content"}]',
    s.examples = '[{"input":"Notícias de Maputo","output":"notícias-maputo","rules_applied":["Lowercase","stopword removal (de)","preserve diacritics"]},{"input":"O Futuro da Economia Moçambicana","output":"futuro-economia-moçambicana","rules_applied":["Stopwords (o","da)","preserve ç"]},{"input":"Guia de Turismo em Inhambane","output":"guia-turismo-inhambane","rules_applied":["Stopwords (de","em)","preserve diacritics"]},{"input":"Cultura e Tradição Local","output":"cultura-tradição-local","rules_applied":["Stopword (e)","preserve ã"]},{"input":"Música Marrabenta na Rádio","output":"música-marrabenta-rádio","rules_applied":["Stopword (na)","preserve ú","á"]},{"input":"Top 10 Praias em Moçambique","output":"top-10-praias-moçambique","rules_applied":["Numbers preserved","stopword (em)","preserve ç"]},{"input":"Desenvolvimento Sustentável e Preservação Ambiental na Província de Gaza","output":"desenvolvimento-sustentável-preservação-ambiental-província-gaza","rules_applied":["Long title truncation","multiple stopwords"]},{"input":"Entrevista: O Futuro do Maputo!","output":"entrevista-futuro-maputo","rules_applied":["Punctuation removed (: !)","stopwords (o","do)"]},{"input":"\"Xitique\" é Tradição Moçambicana","output":"xitique-tradição-moçambicana","rules_applied":["Quotes removed","stopword (é)","preserve ã"]},{"input":"Baía de Pemba & Arquipélago das Quirimbas","output":"baía-pemba-arquipélago-quirimbas","rules_applied":["Ampersand removed","stopwords (de","das)","preserve í"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/pt-MZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'pt-PT'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Portuguese (Portugal) Slugification',
    s.content = 'URL slug generation rules for pt-PT',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["o","a","os","as","um","uma"],"preposition":["de","da","do","em","no","na","para","por","com"],"pronoun":["que","se"],"contraction":["ao","aos","das","dos","pelo","pela","num","numa","duma","dum","nalgum","nalguma"],"conjunction":["e","ou","mas"]}',
    s.stopwords_count = 32,
    s.regional_additions = '[{"word":"","category":"duma","reason":"contraction"},{"word":"","category":"dum","reason":"contraction"},{"word":"","category":"nalgum","reason":"contraction"},{"word":"","category":"nalguma","reason":"contraction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Verify intentional use"}]',
    s.examples = '[{"input":"Guia de Viagem para Lisboa","output":"guia-viagem-lisboa","rules_applied":["stopwords (de","para)","lowercase","hyphen"]},{"input":"As Melhores Praias do Algarve","output":"melhores-praias-algarve","rules_applied":["stopwords (as","do)","lowercase","hyphen"]},{"input":"O Melhor Bacalhau à Brás","output":"melhor-bacalhau-à-brás","rules_applied":["stopwords (o)","preserve diacritics (à)","hyphen"]},{"input":"Receita de Pastéis de Nata","output":"receita-pastéis-nata","rules_applied":["stopwords (de)","preserve diacritics (é)","hyphen"]},{"input":"Castelo de São Jorge no Centro","output":"castelo-são-jorge-centro","rules_applied":["stopwords (de","no)","preserve diacritics (ã)","hyphen"]},{"input":"10 Dicas para Poupar em 2024","output":"10-dicas-poupar-2024","rules_applied":["stopwords (para","em)","keep numbers","hyphen"]},{"input":"Descubra os Encantos Escondidos da Região Vinícola do Douro e as suas Tradições Milenares","output":"descubra-encantos-escondidos-região-vinícola-douro-suas-tradições-milenares","rules_applied":["truncation at 80 chars","stopwords (os","da","do","e","as)"]},{"input":"Café & Pastelaria: Os Sabores de Portugal!","output":"café-pastelaria-sabores-portugal","rules_applied":["remove & : !","stopwords (os","de)","preserve diacritics"]},{"input":"A \"Saudade\" na Cultura Portuguesa","output":"saudade-cultura-portuguesa","rules_applied":["remove quotes","stopwords (a","na)","lowercase"]},{"input":"Açores: Ilhas de São Miguel e Terceira","output":"açores-ilhas-são-miguel-terceira","rules_applied":["preserve ç ã","stopwords (de","e)","remove colon"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/pt-PT.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'qu-PE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'QU (PE) Slugification',
    s.content = 'URL slug generation rules for qu-PE',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[{"word":"","category":"nin","reason":"quotative marker"},{"word":"","category":"ari","reason":"affirmation (yes)"},{"word":"","category":"riki","reason":"evidential (right?)"},{"word":"","category":"puni","reason":"emphatic (truly)"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Stopwords > 60% removed","message":"Title may be too generic"},{"condition":"All words removed","message":"Entire title was stopwords"}]',
    s.examples = '[{"input":"Runasimi yachachiy","output":"runasimi-yachachiy","rules_applied":["lowercase","stopwords"]},{"input":"Qusqu llaqtapi kawsay","output":"qusqu-llaqtapi-kawsay","rules_applied":["lowercase","stopwords"]},{"input":"Pachamama manta willakuy","output":"pachamama-willakuy","rules_applied":["lowercase","stopwords (manta removed)"]},{"input":"Inti Raymi fiesta Piruwpi","output":"inti-raymi-fiesta-piruwpi","rules_applied":["lowercase"]},{"input":"Tawa Suyu ayllu","output":"tawa-suyu-ayllu","rules_applied":["lowercase"]},{"input":"10 yachay wasikunamanta","output":"10-yachay-wasikunamanta","rules_applied":["lowercase","number preserved","stopwords (manta removed)"]},{"input":"Qhapaq nan hatun sistema rurasqa inka civilizacion ukhupi ancestral conocimiento reproduccion manta","output":"qhapaq-nan-hatun-sistema-rurasqa-inka-civilizacion-ukhupi-ancestral-conocimiento","rules_applied":["lowercase","stopwords","truncation (80 chars)"]},{"input":"Mikuy: papa & sara","output":"mikuy-papa-sara","rules_applied":["lowercase","punctuation removed","ampersand removed"]},{"input":"\"Allin kawsay\" nisqa rimay","output":"allin-kawsay-nisqa-rimay","rules_applied":["lowercase","quotes removed"]},{"input":"Chʼaki killapi yakuq maskay","output":"chaki-killapi-yakuq-maskay","rules_applied":["lowercase","ejective marker stripped"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/qu-PE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ro-MD'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'RO (MD) Slugification',
    s.content = 'URL slug generation rules for ro-MD',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["care","ce","acesta","aceasta"],"conjunction":["și","sau","dar","ori","deci"],"preposition":["de","la","pe","cu","din","pentru","prin","spre","fără"],"verb":["este","sunt","fost"],"article":["un","o","al","ale"]}',
    s.stopwords_count = 25,
    s.regional_additions = '[{"word":"","category":"din","reason":"preposition"},{"word":"","category":"pentru","reason":"preposition"},{"word":"","category":"prin","reason":"preposition"},{"word":"","category":"spre","reason":"preposition"},{"word":"","category":"fără","reason":"preposition"},{"word":"","category":"ori","reason":"conjunction"},{"word":"","category":"deci","reason":"conjunction"},{"word":"","category":"acesta","reason":"pronoun"},{"word":"","category":"aceasta","reason":"pronoun"},{"word":"","category":"fost","reason":"verb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for effective SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Potential encoding issue"}]',
    s.examples = '[{"input":"Ghid pentru utilizatori din Moldova","output":"ghid-utilizatori-moldova","rules_applied":["stopwords: pentru","din"]},{"input":"Rețete tradiționale moldovenești","output":"rețete-tradiționale-moldovenești","rules_applied":["diacritics preserved"]},{"input":"Știri și evenimente în Chișinău","output":"știri-evenimente-chișinău","rules_applied":["stopwords: și","în"]},{"input":"Cultura și arta în Republica Moldova","output":"cultura-arta-republica-moldova","rules_applied":["stopwords: și","în"]},{"input":"Turism în Moldova de Nord","output":"turism-moldova-nord","rules_applied":["stopwords: în","de"]},{"input":"Conferința 2025 la Chișinău","output":"conferința-2025-chișinău","rules_applied":["numbers preserved","stopwords: la"]},{"input":"Festivalul Internațional de Film Documentar și Drepturile Omului în Republica Moldova","output":"festivalul-internațional-film-documentar-drepturile-omului-republica-moldova","rules_applied":["truncated at 80 chars","stopwords: de","și","în"]},{"input":"Ce este & cum funcționează?","output":"funcționează","rules_applied":["stopwords: ce","este","cum; special chars removed"]},{"input":"\"Cele mai bune\" restaurante din Chișinău","output":"cele-mai-bune-restaurante-chișinău","rules_applied":["quotes removed","stopwords: din"]},{"input":"Ștefan cel Mare și Sfânt","output":"ștefan-cel-mare-sfânt","rules_applied":["stopwords: și; diacritics ș","â preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ro-MD.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ro-RO'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Romanian (Romania) Slugification',
    s.content = 'URL slug generation rules for ro-RO',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Non-Latin characters detected"},{"condition":"Cedilla instead of comma","message":"Use ș/ț (comma below) not ş/ţ (cedilla)"}]',
    s.examples = '[{"input":"Cele mai bune rețete românești","output":"cele-mai-bune-rețete-românești","rules_applied":["Lowercase","ț preserved"]},{"input":"Ghid de călătorie în România","output":"ghid-călătorie-românia","rules_applied":["Prepositions \"de\" and \"în\" removed","ă and î preserved"]},{"input":"București și împrejurimile sale","output":"bucurești-împrejurimile-sale","rules_applied":["Conjunction \"și\" removed","ș and î preserved"]},{"input":"Cum să înveți programare","output":"cum-înveți-programare","rules_applied":["Preposition \"să\" removed","î preserved"]},{"input":"Ofertă specială: produse tradiționale","output":"ofertă-specială-produse-tradiționale","rules_applied":["Colon removed","ă and ț preserved"]},{"input":"10 sfaturi pentru un stil de viață sănătos","output":"10-sfaturi-stil-viață-sănătos","rules_applied":["Articles and prepositions removed","numbers kept","ă preserved"]},{"input":"Istoria detaliată a culturii și tradițiilor românești din perioada medievală până în zilele noastre","output":"istoria-detaliată-culturii-tradițiilor-românești-perioada-medievală-până","rules_applied":["Truncated at 80 chars","articles and prepositions removed"]},{"input":"Artizanat tradițional & meșteșuguri!","output":"artizanat-tradițional-meșteșuguri","rules_applied":["Ampersand and exclamation removed","ș and ț preserved"]},{"input":"\"Dragoste eternă\" în literatura română","output":"dragoste-eternă-literatura-română","rules_applied":["Quotes removed","preposition \"în\" removed","ă preserved"]},{"input":"Fișiere .pdf și formate de export","output":"fișiere-pdf-formate-export","rules_applied":["Dot removed","conjunction and preposition removed","ș preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ro-RO.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ru-BY'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'RU (BY) Slugification',
    s.content = 'URL slug generation rules for ru-BY',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["и","а","но","или","что","как"],"preposition":["в","на","с","по","к","о","из","за","у","от","до","для","при"],"pronoun":["это","то","он","она","они","мы","вы","его","её","их"]}',
    s.stopwords_count = 29,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Cyrillic characters detected (except numbers)"},{"condition":"Non-standard Unicode","message":"Improper NFC normalization detected"}]',
    s.examples = '[{"input":"Добро пожаловать в Минск","output":"добро-пожаловать-минск","rules_applied":["Lowercase + stopword (в) removed"]},{"input":"Новости Беларуси 2025","output":"новости-беларуси-2025","rules_applied":["Lowercase + numbers preserved"]},{"input":"Брест - город на границе с Польшей","output":"брест-город-границе-польшей","rules_applied":["Lowercase + punctuation removed + stopwords (на","с) removed"]},{"input":"Беловежская пуща и её обитатели","output":"беловежская-пуща-обитатели","rules_applied":["Lowercase + stopwords (и","её) removed"]},{"input":"\"Мирский замок\" - объект ЮНЕСКО","output":"мирский-замок-объект-юнеско","rules_applied":["Quotes removed + hyphen handling"]},{"input":"10 достопримечательностей Гомеля","output":"10-достопримечательностей-гомеля","rules_applied":["Numbers at start preserved"]},{"input":"Путеводитель по Беларуси для туристов в 2025 году с фотографиями и советами местных жителей","output":"путеводитель-беларуси-туристов-2025-году-фотографиями-советами-местных-жителей","rules_applied":["Long title truncated + stopwords (по","для","в","с","и) removed"]},{"input":"Культура! Традиции? Беларусь...","output":"культура-традиции-беларусь","rules_applied":["All punctuation removed"]},{"input":"Что посмотреть в \"Несвижском дворце\" зимой?","output":"посмотреть-несвижском-дворце-зимой","rules_applied":["Stopwords (что","в) removed + quotes removed"]},{"input":"Минск: от древности до современности","output":"минск-древности-современности","rules_applied":["Colon removed + stopwords (от","до) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ru-BY.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ru-IL'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'RU (IL) Slugification',
    s.content = 'URL slug generation rules for ru-IL',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["и","а","но","или","что","как","если"],"preposition":["в","на","с","к","у","о","по","за","из","до","для","от","при"],"pronoun":["это","он","она","они","мы","вы","я","его","её","их"],"adverb":["тут","там","ещё","уже","тоже"]}',
    s.stopwords_count = 35,
    s.regional_additions = '[{"word":"","category":"тут","reason":"adverb"},{"word":"","category":"там","reason":"adverb"},{"word":"","category":"ещё","reason":"adverb"},{"word":"","category":"уже","reason":"adverb"},{"word":"","category":"тоже","reason":"adverb"}]',
    s.script_config = '{"primary_script":"hebrew","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful content"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed Cyrillic/Hebrew","message":"Verify intentional bilingual content"},{"condition":"No Cyrillic characters","message":"May indicate wrong locale selection"}]',
    s.examples = '[{"input":"Новости из Тель-Авива","output":"новости-тель-авива","rules_applied":["lowercase","stopword removal (из)","hyphen spacing"]},{"input":"Жизнь в Израиле для репатриантов","output":"жизнь-израиле-репатриантов","rules_applied":["stopword removal (в","для)","lowercase"]},{"input":"Лучшие рестораны Иерусалима","output":"лучшие-рестораны-иерусалима","rules_applied":["lowercase","hyphen spacing"]},{"input":"Как получить гражданство и визу","output":"получить-гражданство-визу","rules_applied":["stopword removal (как","и)","lowercase"]},{"input":"Русскоязычная община Хайфы","output":"русскоязычная-община-хайфы","rules_applied":["lowercase","hyphen spacing"]},{"input":"10 мест для отдыха в 2026 году","output":"10-мест-отдыха-2026-году","rules_applied":["numbers preserved","stopword removal (для","в)"]},{"input":"Полный путеводитель по израильской кухне для туристов и местных жителей","output":"полный-путеводитель-израильской-кухне-туристов-местных-жителей","rules_applied":["truncation at 80 chars","stopword removal (по","для","и)"]},{"input":"Что посмотреть? Достопримечательности!","output":"посмотреть-достопримечательности","rules_applied":["punctuation removed","stopword removal (что)"]},{"input":"«Великолепный Негев» — путешествие","output":"великолепный-негев-путешествие","rules_applied":["quotes removed","dash normalized"]},{"input":"Ёлка в Израиле: традиции русских","output":"ёлка-израиле-традиции-русских","rules_applied":["Ё preserved","stopword removal (в)","colon removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ru-IL.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ru-KG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'RU (KG) Slugification',
    s.content = 'URL slug generation rules for ru-KG',
    s.slug_rule = 'native_script',
    s.stopwords = '{"pronoun":["это"],"adverb":["там"],"preposition":["в","на","с","к","по","за","из","у","о","для","от","до","при"],"conjunction":["и","а","но","что","как"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"там","reason":"adverb"},{"word":"","category":"ну","reason":"particle"},{"word":"","category":"да","reason":"particle"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Cyrillic characters detected (except numbers)"},{"condition":"Kyrgyz-Russian mix","message":"Mixed Kyrgyz/Russian terms may need review"}]',
    s.examples = '[{"input":"Работа в Бишкеке для специалистов","output":"работа-бишкеке-специалистов","rules_applied":["Lowercase + stopwords (в","для) removed"]},{"input":"Отдых на Иссык-Куле 2025","output":"отдых-иссык-куле-2025","rules_applied":["Lowercase + stopword (на) removed + numbers preserved"]},{"input":"Квартиры в Оше и Караколе","output":"квартиры-оше-караколе","rules_applied":["Lowercase + stopwords (в","и) removed"]},{"input":"Юрты на Сон-Кёле для туристов","output":"юрты-сон-кёле-туристов","rules_applied":["Lowercase + stopwords (на","для) removed + compound name preserved"]},{"input":"Путеводитель по Нарыну и Джалал-Абаду","output":"путеводитель-нарыну-джалал-абаду","rules_applied":["Lowercase + stopwords (по","и) removed"]},{"input":"15 лучших мест в ущелье Ала-Арча","output":"15-лучших-мест-ущелье-ала-арча","rules_applied":["Numbers preserved + stopword (в) removed"]},{"input":"Как получить визу в Кыргызстан и оформить регистрацию для иностранных граждан на длительный срок","output":"как-получить-визу-кыргызстан-оформить-регистрацию-иностранных-граждан","rules_applied":["Long title truncated + stopwords (в","и","для","на) removed"]},{"input":"Кумыс! Манты? Традиции Кыргызстана...","output":"кумыс-манты-традиции-кыргызстана","rules_applied":["All punctuation removed"]},{"input":"\"Эпос Манас\" и его значение для культуры","output":"эпос-манас-значение-культуры","rules_applied":["Stopwords (и","его","для) removed + quotes removed"]},{"input":"День независимости: 31 августа в Бишкеке","output":"день-независимости-31-августа-бишкеке","rules_applied":["Colon removed + stopword (в) removed + date preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ru-KG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ru-KZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'RU (KZ) Slugification',
    s.content = 'URL slug generation rules for ru-KZ',
    s.slug_rule = 'native_script',
    s.stopwords = '{"preposition":["в","на","с","по","к","о","из","за","у","от","до","для","при","через"],"adverb":["ещё","очень","также"],"conjunction":["и","а","но","или","что","как"],"verb":["можно","нужно"],"pronoun":["это","то","он","она","они","мы","вы","его","её","их"]}',
    s.stopwords_count = 35,
    s.regional_additions = '[{"word":"","category":"ещё","reason":"adverb"},{"word":"","category":"очень","reason":"adverb"},{"word":"","category":"также","reason":"adverb"},{"word":"","category":"можно","reason":"verb"},{"word":"","category":"нужно","reason":"verb"},{"word":"","category":"через","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Cyrillic characters detected (except numbers)"},{"condition":"Non-standard Unicode","message":"Improper NFC normalization detected"}]',
    s.examples = '[{"input":"Работа в Алматы 2025","output":"работа-алматы-2025","rules_applied":["Lowercase + stopword (в) removed + numbers preserved"]},{"input":"Квартиры и дома в Астане","output":"квартиры-дома-астане","rules_applied":["Lowercase + stopwords (и","в) removed"]},{"input":"Kaspi кредит: как оформить онлайн","output":"kaspi-кредит-оформить-онлайн","rules_applied":["Mixed script preserved + stopword (как) removed + colon removed"]},{"input":"Туры на Бозжыру из Актау","output":"туры-бозжыру-актау","rules_applied":["Lowercase + stopwords (на","из) removed"]},{"input":"Air Astana: расписание рейсов Алматы-Астана","output":"air-astana-расписание-рейсов-алматы-астана","rules_applied":["Mixed script preserved + colon removed + hyphen preserved"]},{"input":"10 лучших ресторанов Шымкента","output":"10-лучших-ресторанов-шымкента","rules_applied":["Numbers at start preserved + lowercase"]},{"input":"Путеводитель по Туркестану для туристов с советами и маршрутами по историческим местам","output":"путеводитель-туркестану-туристов-советами-маршрутами-историческим-местам","rules_applied":["Truncation + stopwords (по","для","с","и) removed"]},{"input":"День Независимости Казахстана! Как отмечают 16 декабря?","output":"день-независимости-казахстана-отмечают-16-декабря","rules_applied":["Punctuation removed + stopword (как) removed"]},{"input":"\"Халык Банк\" - надёжный партнёр для бизнеса","output":"халык-банк-надёжный-партнёр-бизнеса","rules_applied":["Quotes removed + stopword (для) removed"]},{"input":"Чарынский каньон: от Алматы до природного чуда","output":"чарынский-каньон-алматы-природного-чуда","rules_applied":["Colon removed + stopwords (от","до) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ru-KZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ru-MD'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'RU (MD) Slugification',
    s.content = 'URL slug generation rules for ru-MD',
    s.slug_rule = 'native_script',
    s.stopwords = '{"conjunction":["и","а","но","что","как"],"adverb":["ещё","уже"],"preposition":["в","на","с","к","по","за","из","у","о","для","от","до","при","про"],"pronoun":["это","себе"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[{"word":"","category":"при","reason":"preposition"},{"word":"","category":"про","reason":"preposition"},{"word":"","category":"себе","reason":"pronoun"},{"word":"","category":"ещё","reason":"adverb"},{"word":"","category":"уже","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Cyrillic characters detected (except numbers)"},{"condition":"Romanian terms mixed","message":"Consider separate Romanian slug for bilingual content"}]',
    s.examples = '[{"input":"Добро пожаловать в Кишинёв","output":"добро-пожаловать-кишинёв","rules_applied":["Lowercase + stopword (в) removed + ё preserved"]},{"input":"Молдавское вино из Криково","output":"молдавское-вино-криково","rules_applied":["Lowercase + stopword (из) removed"]},{"input":"Работа в Кишинёве 2025","output":"работа-кишинёве-2025","rules_applied":["Lowercase + stopword (в) removed + numbers preserved"]},{"input":"Тирасполь и Бендеры - города на Днестре","output":"тирасполь-бендеры-города-днестре","rules_applied":["Stopwords (и","на) removed + hyphen normalized"]},{"input":"Цены на квартиры в Бельцах","output":"цены-квартиры-бельцах","rules_applied":["Stopwords (на","в) removed"]},{"input":"15 виноделен Молдовы для туристов","output":"15-виноделен-молдовы-туристов","rules_applied":["Number preserved + stopword (для) removed"]},{"input":"Путеводитель по винным погребам Криково и Милештий Мичь с дегустациями для туристов в 2025 году","output":"путеводитель-винным-погребам-криково-милештий-мичь-дегустациями-туристов-2025-году","rules_applied":["Long title + stopwords (по","и","с","для","в) removed"]},{"input":"Мамалыга, плачинта и токана: молдавская кухня!","output":"мамалыга-плачинта-токана-молдавская-кухня","rules_applied":["Punctuation removed + stopword (и) removed"]},{"input":"Отзывы о \"Victoriabank\" в Кишинёве","output":"отзывы-victoriabank-кишинёве","rules_applied":["Quotes removed + stopwords (о","в) removed + brand preserved"]},{"input":"Мэрцишор: весенний праздник в Молдове","output":"мэрцишор-весенний-праздник-молдове","rules_applied":["Colon removed + stopword (в) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ru-MD.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ru-RU'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Russian (Russia) Slugification',
    s.content = 'URL slug generation rules for ru-RU',
    s.slug_rule = 'native_script',
    s.stopwords = '{"pronoun":["это","то","он","она","они","мы","вы","его","её","их"],"preposition":["в","на","с","по","к","о","из","за","у","от","до","для","при"],"conjunction":["и","а","но","или","что","как"]}',
    s.stopwords_count = 29,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Cyrillic characters detected (except numbers)"},{"condition":"Non-standard Unicode","message":"Improper NFC normalization detected"}]',
    s.examples = '[{"input":"Добро пожаловать на сайт","output":"добро-пожаловать-сайт","rules_applied":["Lowercase + stopwords (на) removed"]},{"input":"Новости России 2025","output":"новости-россии-2025","rules_applied":["Lowercase + numbers preserved"]},{"input":"Москва - лучший город для туризма","output":"москва-лучший-город-туризма","rules_applied":["Lowercase + punctuation removed + stopwords (для) removed"]},{"input":"Как научиться программировать?","output":"как-научиться-программировать","rules_applied":["Lowercase + question mark removed + stopword (как) kept (meaningful)"]},{"input":"\"Анна Каренина\" - великий роман","output":"анна-каренина-великий-роман","rules_applied":["Quotes removed + hyphen preserved"]},{"input":"10 способов улучшить здоровье","output":"10-способов-улучшить-здоровье","rules_applied":["Numbers at start preserved"]},{"input":"Путеводитель по Санкт-Петербургу и Москве для туристов в 2025 году","output":"путеводитель-санкт-петербургу-москве-туристов-2025-году","rules_applied":["Long title + stopwords (по","и","для","в) removed + compound city name preserved"]},{"input":"Технологии! Инновации? Будущее…","output":"технологии-инновации-будущее","rules_applied":["All punctuation removed"]},{"input":"Что такое \"искусственный интеллект\" сегодня?","output":"искусственный-интеллект-сегодня","rules_applied":["Stopwords (что","такое) removed + quotes removed"]},{"input":"История России: от Киевской Руси до наших дней","output":"история-россии-киевской-руси-наших-дней","rules_applied":["Colon removed + stopwords (от","до) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ru-RU.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'rw-RW'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'RW (RW) Slugification',
    s.content = 'URL slug generation rules for rw-RW',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["muri"],"conjunction":["kandi","naho"],"interrogative":["none"]}',
    s.stopwords_count = 4,
    s.regional_additions = '[{"word":"","category":"muri","reason":"preposition"},{"word":"","category":"kandi","reason":"conjunction"},{"word":"","category":"naho","reason":"conjunction"},{"word":"","category":"none","reason":"interrogative"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"> 60% stopwords removed","message":"Content may be too generic"}]',
    s.examples = '[{"input":"Uko wakora umushinga ukomeye","output":"uko-wakora-umushinga-ukomeye","rules_applied":["No stopwords to remove"]},{"input":"Amakuru mashya mu Rwanda","output":"amakuru-mashya-rwanda","rules_applied":["Stopwords: mu removed"]},{"input":"Ubuzima bwiza na siporo","output":"ubuzima-bwiza-siporo","rules_applied":["Stopwords: na removed"]},{"input":"Inyigisho kuri tekinoloji","output":"inyigisho-tekinoloji","rules_applied":["Stopwords: kuri removed"]},{"input":"Ubukungu bw\'u Rwanda muri 2025","output":"ubukungu-bwu-rwanda-2025","rules_applied":["Stopwords: muri removed","apostrophe removed"]},{"input":"Top 10 ibibanza byiza byo gusura","output":"top-10-ibibanza-byiza-gusura","rules_applied":["Numbers preserved","stopwords: byo removed"]},{"input":"Amabwiriza yose ajyanye no gukora umushinga ukomeye kandi urambye mu Rwanda no mu bindi bihugu","output":"amabwiriza-yose-ajyanye-gukora-umushinga-ukomeye-urambye-rwanda-bindi-bihugu","rules_applied":["Truncated at 80 chars","stopwords: no","mu","kandi removed"]},{"input":"Kigali: Umujyi w\'Amahoro & Iterambere!","output":"kigali-umujyi-wamahoro-iterambere","rules_applied":["Punctuation removed","colon and ampersand stripped"]},{"input":"\"Isibo\" - Ubuhanzi bw\'u Rwanda","output":"isibo-ubuhanzi-bwu-rwanda","rules_applied":["Quotes and dash removed"]},{"input":"Inka n\'Ingabo: Umuco Nyarwanda","output":"inka-ningabo-umuco-nyarwanda","rules_applied":["Apostrophe contracted","colon removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/rw-RW.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'sd-PK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'SD (PK) Slugification',
    s.content = 'URL slug generation rules for sd-PK',
    s.slug_rule = 'native_script',
    s.stopwords = '{"article":["هڪ","هي","اهي"],"conjunction":["۽","يا","پر","جيڪو","ته"],"pronoun":["هو","هوءَ","اهو","اها"],"verb":["آهي","آهن","ٿيو","هئي","هئا","ڪيو","ڪري"]}',
    s.stopwords_count = 19,
    s.regional_additions = '[]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing Sindhi Arabic with other scripts (except numbers)"},{"condition":"Non-standard Unicode composition","message":"Ensure proper NFC normalization"}]',
    s.examples = '[{"input":"سنڌ جي تاريخ","output":"سنڌ-تاريخ","rules_applied":["Postposition removed (جي)","hyphenation"]},{"input":"ڪراچي ۾ سياحت","output":"ڪراچي-سياحت","rules_applied":["Postposition removed (۾)","hyphenation"]},{"input":"سنڌي ادب ۽ ثقافت","output":"سنڌي-ادب-ثقافت","rules_applied":["Conjunction removed (۽)","hyphenation"]},{"input":"پاڪستان جو قومي ڏينهن","output":"پاڪستان-قومي-ڏينهن","rules_applied":["Postposition removed (جو)","hyphenation"]},{"input":"ٺٽو شهر جي تعمير","output":"ٺٽو-شهر-تعمير","rules_applied":["Postposition removed (جي)","Sindhi letters preserved"]},{"input":"2025 ۾ سنڌ جي معيشت","output":"2025-سنڌ-معيشت","rules_applied":["Numbers preserved","postpositions removed (۾، جي)"]},{"input":"سنڌ جي قديم تمدن ۽ تاريخي ورثو: موهن جو دڙو کان ٺٽو تائين جو سفر","output":"سنڌ-قديم-تمدن-تاريخي-ورثو-موهن-دڙو-ٺٽو-تائين-سفر","rules_applied":["Truncated to 80 chars","stopwords removed"]},{"input":"سنڌي کاڌو: ڪچي بريان، ساڄ ۽ ٻيو!","output":"سنڌي-کاڌو-ڪچي-بريان-ساڄ-ٻيو","rules_applied":["Punctuation removed (:، !)","conjunction removed (۽)"]},{"input":"\"سنڌو درياءَ\" ڪهاڻي","output":"سنڌو-درياءَ-ڪهاڻي","rules_applied":["Quotes removed","hamza preserved"]},{"input":"ڪراچي بمقابلہ حيدرآباد: ڪهڙو شهر بهتر آهي","output":"ڪراچي-بمقابلہ-حيدرآباد-ڪهڙو-شهر-بهتر","rules_applied":["Colon removed","verb removed (آهي)","hyphenation"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/sd-PK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'si-LK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'SI (LK) Slugification',
    s.content = 'URL slug generation rules for si-LK',
    s.slug_rule = 'native_script',
    s.stopwords = '{"verb":["ඇත","නැත","කළ"],"pronoun":["එය","මම","ඔබ","ඔහු"],"demonstrative":["ඒ","මේ"],"conjunction":["සහ","හෝ","නමුත්"],"honorific":["ශ්‍රී"]}',
    s.stopwords_count = 13,
    s.regional_additions = '[{"word":"","category":"ලංකාවේ","reason":"locative"},{"word":"","category":"ශ්‍රී","reason":"honorific"},{"word":"","category":"අලුත්","reason":"adjective"},{"word":"","category":"පිළිබඳ","reason":"postposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts without context","message":"May indicate transliteration issue"},{"condition":"Pure ASCII slug","message":"Consider using Sinhala for local SEO"}]',
    s.examples = '[{"input":"ආයුබෝවන් ශ්‍රී ලංකාවට","output":"ආයුබෝවන්-ලංකාවට","rules_applied":["stopwords: ශ්‍රී removed"]},{"input":"කොළඹ නගර සංචාරය","output":"කොළඹ-නගර-සංචාරය","rules_applied":["spaces to hyphens"]},{"input":"සිංහල භාෂා මාර්ගෝපදේශය","output":"සිංහල-භාෂා-මාර්ගෝපදේශය","rules_applied":["NFC normalization","spaces to hyphens"]},{"input":"ශ්‍රී ලංකාවේ සංස්කෘතිය සහ සම්ප්‍රදාය","output":"ලංකාවේ-සංස්කෘතිය-සම්ප්‍රදාය","rules_applied":["stopwords: ශ්‍රී","සහ removed"]},{"input":"ගාල්ල බලකොටුව පිළිබඳ","output":"ගාල්ල-බලකොටුව","rules_applied":["stopwords: පිළිබඳ removed"]},{"input":"වසර 2026 සඳහා අලුත් ව්‍යාපෘති 10ක්","output":"වසර-2026-අලුත්-ව්‍යාපෘති-10ක්","rules_applied":["numbers preserved","stopwords: සඳහා removed"]},{"input":"දකුණු පළාත් සංචාරක ස්ථාන මාර්ගෝපදේශය සහ ප්‍රවේශ තොරතුරු 2026","output":"දකුණු-පළාත්-සංචාරක-ස්ථාන-මාර්ගෝපදේශය-ප්‍රවේශ-තොරතුරු-2026","rules_applied":["truncated to 80 chars","stopwords removed"]},{"input":"\"සුභ අලුත් අවුරුද්ද!\" ශ්‍රී ලංකාවේ","output":"සුභ-අලුත්-අවුරුද්ද-ලංකාවේ","rules_applied":["quotes removed","stopwords: ශ්‍රී removed"]},{"input":"මහින්ද රාජපක්ෂගේ \'නව දැක්ම\'","output":"මහින්ද-රාජපක්ෂගේ-නව-දැක්ම","rules_applied":["single quotes removed"]},{"input":"ක්‍රිකට් - ලංකා vs ඉන්දියාව","output":"ක්‍රිකට්-ලංකා-vs-ඉන්දියාව","rules_applied":["special chars removed","mixed script handled"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/si-LK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'sk-SK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Slovak (Slovakia) Slugification',
    s.content = 'URL slug generation rules for sk-SK',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["a","i","alebo","ale","no","lebo","pretože","aby","keď","ak","kým"],"verb":["je","sú","bol","bola","bolo","boli","byť"],"pronoun":["to","ten","tá","táto","tento","toto","ktorý","ktorá","ktoré"],"preposition":["v","vo","na","do","z","zo","s","so","k","ku","o","od","po","pre","pri","za","nad","pod","medzi","bez"],"adverb":["ako","tak","tu","tam","tiež","ešte","veľmi","len","už","práve"]}',
    s.stopwords_count = 57,
    s.regional_additions = '[{"word":"","category":"tiež","reason":"adverb"},{"word":"","category":"ešte","reason":"adverb"},{"word":"","category":"veľmi","reason":"adverb"},{"word":"","category":"len","reason":"adverb"},{"word":"","category":"už","reason":"adverb"},{"word":"","category":"práve","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts detected","message":"Verify intentional mixing"},{"condition":"Only numbers","message":"Should contain alphabetic characters"}]',
    s.examples = '[{"input":"Najlepšie reštaurácie v Bratislave","output":"najlepšie-reštaurácie-bratislave","rules_applied":["lowercase","stopword \"v\" removed","diacritics preserved"]},{"input":"Ako na správnu výslovnosť","output":"správnu-výslovnosť","rules_applied":["lowercase","stopwords \"ako\"","\"na\" removed","diacritics preserved"]},{"input":"Šport a zdravie pre každého","output":"šport-zdravie-každého","rules_applied":["lowercase","stopwords \"a\"","\"pre\" removed","diacritics preserved"]},{"input":"Tradičná slovenská kuchyňa","output":"tradičná-slovenská-kuchyňa","rules_applied":["lowercase","diacritics preserved"]},{"input":"História Vysokých Tatier","output":"história-vysokých-tatier","rules_applied":["lowercase","diacritics preserved"]},{"input":"10 tipov na výlet do Košíc","output":"10-tipov-výlet-košíc","rules_applied":["lowercase","stopwords \"na\"","\"do\" removed","numbers preserved"]},{"input":"Medzinárodný festival kultúry a umenia v centre mesta Žilina 2026","output":"medzinárodný-festival-kultúry-umenia-centre-mesta-žilina-2026","rules_applied":["lowercase","stopwords \"a\"","\"v\" removed","truncated to 80 chars"]},{"input":"Čo? Kde? Kedy! - Sprievodca Slovenskom","output":"čo-kde-kedy-sprievodca-slovenskom","rules_applied":["lowercase","punctuation removed","diacritics preserved"]},{"input":"\"Krásny nový svet\" - recenzia knihy","output":"krásny-nový-svet-recenzia-knihy","rules_applied":["lowercase","quotes removed","diacritics preserved"]},{"input":"Ľudové tradície na Záhorí","output":"ľudové-tradície-záhorí","rules_applied":["lowercase","stopword \"na\" removed","soft ľ preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/sk-SK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'sl-SI'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Slovenian (Slovenia) Slugification',
    s.content = 'URL slug generation rules for sl-SI',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["in","ali","ter","pa","a","ampak","vendar","da","ker","če","ko","kot"],"pronoun":["ta","ti","to","te","tisto","jaz","mi","vi","on","ona","oni","se","kaj","kdo","kar","ki","vse"],"adverb":["tudi","samo","le","zelo","bolj","lahko","kako","tako","potem","zdaj"],"verb":["je","so","sem","si","smo","ste","bo","bil","bila","biti","ima","imajo"],"preposition":["na","v","za","z","s","po","do","od","pri","med","nad","pod","pred","ob","o","k","iz"],"article":["en","ena","eno","eden","neki","neka","neke"]}',
    s.stopwords_count = 75,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Avoid mixing with non-Latin scripts"},{"condition":"Stripped diacritics","message":"Slovenian diacritics should be preserved"}]',
    s.examples = '[{"input":"Vodnik po Ljubljani","output":"vodnik-ljubljani","rules_applied":["Stopwords removed (po)","lowercase","hyphenation"]},{"input":"Najboljše restavracije v Sloveniji","output":"najboljše-restavracije-sloveniji","rules_applied":["Preposition removed (v)","diacritics preserved (š)"]},{"input":"Kako izbrati pravo avto zavarovanje","output":"izbrati-pravo-avto-zavarovanje","rules_applied":["Stopwords removed (kako)","lowercase"]},{"input":"Bled: Jezero in otok","output":"bled-jezero-otok","rules_applied":["Punctuation removed (:)","conjunction removed (in)"]},{"input":"Triglav je najvišja gora v Sloveniji","output":"triglav-najvišja-gora-sloveniji","rules_applied":["Verb removed (je)","preposition removed (v)"]},{"input":"10 razlogov za obisk Portoroža","output":"10-razlogov-obisk-portoroža","rules_applied":["Number preserved","preposition removed (za)","diacritics preserved (ž)"]},{"input":"Tradicionalna slovenska kuhinja: Štruklji, potica in kranjska klobasa na domači mizi","output":"tradicionalna-slovenska-kuhinja-štruklji-potica-kranjska-klobasa-domači-mizi","rules_applied":["Truncated to 80 chars","conjunction removed (in)","preposition removed (na)","diacritics preserved (š)"]},{"input":"Čudoviti slapovi: Savica & Peričnik!","output":"čudoviti-slapovi-savica-peričnik","rules_applied":["Special chars removed (&",":","!)","diacritics preserved (č)"]},{"input":"\"Živjo\" ali \"dober dan\"? Pozdravi v slovenščini","output":"živjo-dober-dan-pozdravi-slovenščini","rules_applied":["Quotes removed","conjunction removed (ali)","preposition removed (v)","question mark removed"]},{"input":"Škocjanske jame vs. Postojnska jama: katera je boljša za ogled","output":"škocjanske-jame-postojnska-jama-katera-boljša-ogled","rules_applied":["Punctuation removed (vs.",":)","verb removed (je)","preposition removed (za)","diacritics preserved (š)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/sl-SI.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'sn-ZW'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'SN (ZW) Slugification',
    s.content = 'URL slug generation rules for sn-ZW',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"adverb":["kupi"],"pronoun":["ani","chii"]}',
    s.stopwords_count = 3,
    s.regional_additions = '[{"word":"","category":"here","reason":"particle"},{"word":"","category":"sei","reason":"particle"},{"word":"","category":"ani","reason":"pronoun"},{"word":"","category":"chii","reason":"pronoun"},{"word":"","category":"kupi","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"> 60% stopwords removed","message":"Content may be too generic"}]',
    s.examples = '[{"input":"Mhuri Yakasimba muZimbabwe","output":"mhuri-yakasimba-zimbabwe","rules_applied":["Stopwords: mu removed"]},{"input":"Nzira Dzekuchengetedza Mari","output":"nzira-dzekuchengetedza-mari","rules_applied":["No stopwords present"]},{"input":"Zvokudya Zvinoyera muHarare","output":"zvokudya-zvinoyera-harare","rules_applied":["Stopwords: mu removed"]},{"input":"Mitambo yeNhasi na Mangwana","output":"mitambo-yenhasi-mangwana","rules_applied":["Stopwords: na removed"]},{"input":"Dzidzo yeVana muChikoro","output":"dzidzo-yevana-chikoro","rules_applied":["Stopwords: mu removed"]},{"input":"Makumi Maviri Emadhora 20 Vhiki Rega Rega","output":"makumi-maviri-emadhora-20-vhiki-rega-rega","rules_applied":["Numbers preserved"]},{"input":"Gwaro Rakakwana Rekubatsira Vanhu Vose Vanoda Kutanga Bhizimisi muZimbabwe Nhasi Uno","output":"gwaro-rakakwana-rekubatsira-vanhu-vose-vanoda-kutanga-bhizimisi-zimbabwe-nhasi","rules_applied":["Truncated at 80 chars","stopwords: mu removed"]},{"input":"Harare: Guta Guru reZimbabwe!","output":"harare-guta-guru-rezimbabwe","rules_applied":["Punctuation removed"]},{"input":"\"Sadza\" - Chikafu Chikuru cheVaZimba","output":"sadza-chikafu-chikuru-chevazimba","rules_applied":["Quotes and hyphen removed"]},{"input":"Musika weGreendale uye Borrowdale","output":"musika-wegreendale-borrowdale","rules_applied":["Stopwords: uye removed","compound word handling"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/sn-ZW.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'so-SO'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'SO (SO) Slugification',
    s.content = 'URL slug generation rules for so-SO',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["ka","ku","ilaa","dhexe"],"adverb":["sida","markii"],"pronoun":["waxaa","aan","uu","ay","waxa"],"verb":["waa","yahay","tahay"],"conjunction":["iyo","ama","laakiin","oo","ee","haddii"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[{"word":"","category":"waxa","reason":"pronoun"},{"word":"","category":"sida","reason":"adverb"},{"word":"","category":"haddii","reason":"conjunction"},{"word":"","category":"markii","reason":"adverb"},{"word":"","category":"ilaa","reason":"preposition"},{"word":"","category":"dhexe","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"> 60% stopwords removed","message":"Content may be too generic"}]',
    s.examples = '[{"input":"Sida loo kobciyo ganacsigaaga","output":"loo-kobciyo-ganacsigaaga","rules_applied":["Stopwords: sida removed"]},{"input":"Muqdisho iyo meelaha ugu fiican","output":"muqdisho-meelaha-ugu-fiican","rules_applied":["Stopwords: iyo removed"]},{"input":"Talooyinka caafimaadka iyo nafaqada","output":"talooyinka-caafimaadka-nafaqada","rules_applied":["Stopwords: iyo removed"]},{"input":"Dhaqanka Soomaaliyeed ee caanka ah","output":"dhaqanka-soomaaliyeed-caanka-ah","rules_applied":["Stopwords: ee removed"]},{"input":"Ganacsiga ka bilow Soomaaliya","output":"ganacsiga-bilow-soomaaliya","rules_applied":["Stopwords: ka removed"]},{"input":"10 Fursado Ganacsi oo Cusub 2025","output":"10-fursado-ganacsi-cusub-2025","rules_applied":["Numbers preserved","stopwords: oo removed"]},{"input":"Hagaha dhamaystiran ee ku saabsan dhismaha guryaha casriga ah ee Soomaaliya ku taal","output":"hagaha-dhamaystiran-saabsan-dhismaha-guryaha-casriga-ah-soomaaliya-taal","rules_applied":["Truncated at 80 chars","stopwords: ee","ku removed"]},{"input":"Xoolaha Soomaaliya: Geel, Ido & Lo\'!","output":"xoolaha-soomaaliya-geel-ido-lo","rules_applied":["Punctuation and special chars removed"]},{"input":"\"Hilib Ari\" - Cuntada Soomaaliga","output":"hilib-ari-cuntada-soomaaliga","rules_applied":["Quotes and dash formatting applied"]},{"input":"Kooxda Kubadda Cagta ee Soomaaliya","output":"kooxda-kubadda-cagta-soomaaliya","rules_applied":["Stopwords: ee removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/so-SO.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'sq-AL'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'SQ (AL) Slugification',
    s.content = 'URL slug generation rules for sq-AL',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"preposition":["në","me","për","nga","pa","prej","tek","te"],"conjunction":["dhe","por","ose","si","kur"]}',
    s.stopwords_count = 13,
    s.regional_additions = '[{"word":"","category":"tek","reason":"preposition"},{"word":"","category":"te","reason":"preposition"},{"word":"","category":"si","reason":"conjunction"},{"word":"","category":"kur","reason":"conjunction"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts detected","message":"Review for consistency"}]',
    s.examples = '[{"input":"Mirësevini në Shqipëri","output":"mirësevini-shqipëri","rules_applied":["Stopword \"në\" removed","ë preserved"]},{"input":"Historia e Tiranës","output":"historia-tiranës","rules_applied":["Stopword \"e\" removed","ë preserved"]},{"input":"Plazhi i Durrësit","output":"plazhi-durrësit","rules_applied":["Stopword \"i\" removed","ë preserved"]},{"input":"Udhëzues për Turistët","output":"udhëzues-turistët","rules_applied":["Stopword \"për\" removed","ë preserved"]},{"input":"Bukuria e Alpeve Shqiptare","output":"bukuria-alpeve-shqiptare","rules_applied":["Stopword \"e\" removed","lowercase"]},{"input":"10 Vendet Më të Bukura në 2025","output":"10-vendet-më-bukura-2025","rules_applied":["Numbers kept","stopwords removed"]},{"input":"Trashëgimia Kulturore dhe Historike e Shqipërisë dhe Ndikimi i saj në Evropë","output":"trashëgimia-kulturore-historike-shqipërisë-ndikimi-saj-evropë","rules_applied":["Truncated at word boundary","stopwords removed"]},{"input":"Çfarë është: Byrek?","output":"çfarë-byrek","rules_applied":["Punctuation removed","ç preserved"]},{"input":"Kënga \"Vallja e Tropojës\"","output":"kënga-vallja-tropojës","rules_applied":["Quotes removed","ë preserved"]},{"input":"Ëndërr në Bregdet","output":"ëndërr-bregdet","rules_applied":["Initial ë handled correctly"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/sq-AL.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'sr-RS'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'SR (RS) Slugification',
    s.content = 'URL slug generation rules for sr-RS',
    s.slug_rule = 'native_script',
    s.stopwords = '{"preposition":["у","на","за","са","од","до","по","о","из","ка","при","преко","кроз","између","око"],"pronoun":["ми","ви","он","она","они","то","ово","та"],"conjunction":["и","а","али","или","као","што","ако","када","док","него","већ","па","те"]}',
    s.stopwords_count = 36,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Latin characters detected in Cyrillic slug"},{"condition":"Non-standard Unicode","message":"Improper NFC normalization detected"}]',
    s.examples = '[{"input":"Добродошли у Србију","output":"добродошли-србију","rules_applied":["Lowercase + stopword (у) removed"]},{"input":"Најбоље туристичке дестинације","output":"најбоље-туристичке-дестинације","rules_applied":["Lowercase applied"]},{"input":"Водич за путовање кроз Београд","output":"водич-путовање-београд","rules_applied":["Stopwords (за","кроз) removed"]},{"input":"Историја и култура Србије","output":"историја-култура-србије","rules_applied":["Stopword (и) removed"]},{"input":"Рецепти из српске кухиње","output":"рецепти-српске-кухиње","rules_applied":["Stopword (из) removed"]},{"input":"10 разлога за посету Новом Саду","output":"10-разлога-посету-новом-саду","rules_applied":["Numbers preserved + stopword (за) removed"]},{"input":"Српска традиционална јела и пића која морате да пробате на путовању кроз Балкан","output":"српска-традиционална-јела-пића-која-морате-пробате-путовању-балкан","rules_applied":["Long title truncated + stopwords (и","да","на","кроз) removed"]},{"input":"Технологија! Иновације? Будућност...","output":"технологија-иновације-будућност","rules_applied":["All punctuation removed"]},{"input":"\"На Дрини ћуприја\" - чувени роман","output":"дрини-ћуприја-чувени-роман","rules_applied":["Quotes removed + stopword (на) removed"]},{"input":"Србија: од Копаоника до Златибора","output":"србија-копаоника-златибора","rules_applied":["Colon removed + stopwords (од","до) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/sr-RS.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'su-ID'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'SU (ID) Slugification',
    s.content = 'URL slug generation rules for su-ID',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["kanggo","dina","kana","tina"],"conjunction":["sareng"]}',
    s.stopwords_count = 5,
    s.regional_additions = '[{"word":"","category":"sareng","reason":"conjunction"},{"word":"","category":"kanggo","reason":"preposition"},{"word":"","category":"dina","reason":"preposition"},{"word":"","category":"kana","reason":"preposition"},{"word":"","category":"tina","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed (> 60%)","message":"Review original title length"},{"condition":"Non-ASCII characters detected","message":"Verify normalization pipeline"}]',
    s.examples = '[{"input":"Wisata ka Bandung","output":"wisata-bandung","rules_applied":["Stopword removed (ka)","lowercase"]},{"input":"Carita Rakyat Sunda","output":"carita-rakyat-sunda","rules_applied":["Lowercase","hyphenation"]},{"input":"Resep Masakan nu Ngeunah","output":"resep-masakan-ngeunah","rules_applied":["Stopword removed (nu)","lowercase"]},{"input":"Kota Bandung jeung Sakurilingna","output":"kota-bandung-sakurilingna","rules_applied":["Stopword removed (jeung)","lowercase"]},{"input":"Sejarah Karajaan Sunda di Tatar Pasundan","output":"sejarah-karajaan-sunda-tatar-pasundan","rules_applied":["Stopword removed (di)","lowercase"]},{"input":"10 Tempat Wisata nu Alus di Jawa Barat","output":"10-tempat-wisata-alus-jawa-barat","rules_applied":["Number preserved","stopwords removed (nu","di)"]},{"input":"Pangaweruh ngeunaan Budaya Sunda: Tradisi, Seni, jeung Kasenian Lokal nu Masih Hirup di Tatar Pasundan","output":"pangaweruh-ngeunaan-budaya-sunda-tradisi-seni-kasenian-lokal-masih-hirup-tatar","rules_applied":["Truncated at 80 chars","stopwords removed","special char removed (:)"]},{"input":"Kaulinan Barudak: Tradisi & Permainan!","output":"kaulinan-barudak-tradisi-permainan","rules_applied":["Special chars removed (&",":","!)","lowercase"]},{"input":"Basa Sunda \"Lemes\" atawa \"Kasar\"?","output":"basa-sunda-lemes-kasar","rules_applied":["Quotes removed","stopword (atawa) removed","question mark removed"]},{"input":"Ngaran Tutuwuhan Sunda vs. Indonesia","output":"ngaran-tutuwuhan-sunda-indonesia","rules_applied":["Period removed (vs.)","lowercase"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/su-ID.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'sv-SE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Swedish (Sweden) Slugification',
    s.content = 'URL slug generation rules for sv-SE',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["och","eller","men","samt"],"preposition":["i","på","av","för","med","till","från","om","vid","över","under","utan"],"article":["en","ett","den","det"],"verb":["är","var"],"pronoun":["som","de","du","han","hon","vi"]}',
    s.stopwords_count = 28,
    s.regional_additions = '[{"word":"","category":"samt","reason":"conjunction"},{"word":"","category":"över","reason":"preposition"},{"word":"","category":"under","reason":"preposition"},{"word":"","category":"utan","reason":"preposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Non-Latin characters detected"},{"condition":"Missing Swedish chars","message":"Check if å/ä/ö should be included"}]',
    s.examples = '[{"input":"Köpguide för bästa smarttelefoner 2025","output":"köpguide-bästa-smarttelefoner-2025","rules_applied":["Preserve å/ä/ö","remove stopwords (för)","keep numbers"]},{"input":"Så här installerar du WordPress på din webbserver","output":"installerar-wordpress-webbserver","rules_applied":["Remove stopwords (så","här","du","på","din)","preserve lowercase"]},{"input":"Recension av iPhone 15 Pro Max","output":"recension-iphone-15-pro-max","rules_applied":["Remove stopword (av)","keep product name","numbers"]},{"input":"Bästa restaurangerna i Stockholm & Göteborg","output":"bästa-restaurangerna-stockholm-göteborg","rules_applied":["Remove stopwords (i)","remove ampersand"]},{"input":"10 tips för effektiv programmering","output":"10-tips-effektiv-programmering","rules_applied":["Keep leading number","remove stopword (för)"]},{"input":"Guide: Så lyckas du med digital marknadsföring","output":"guide-lyckas-digital-marknadsföring","rules_applied":["Remove stopwords (så","du","med)","keep colon context"]},{"input":"Nyheter om AI och maskininlärning från KTH","output":"nyheter-ai-maskininlärning-kth","rules_applied":["Remove stopwords (om","och","från)","keep acronyms"]},{"input":"\"Framtidens energi\" – intervju med forskare","output":"framtidens-energi-intervju-forskare","rules_applied":["Remove quotes/dashes","remove stopword (med)"]},{"input":"Långt titel exempel som innehåller över åttio tecken för att testa trunkering av för långa sluggar","output":"långt-titel-exempel-innehåller-åttio-tecken-testa-trunkering-långa-sluga","rules_applied":["Truncate at 80 chars","remove multiple stopwords"]},{"input":"Årets bästa böcker & författare 2024/2025","output":"årets-bästa-böcker-författare-2024-2025","rules_applied":["Preserve å","remove ampersand","convert slash to hyphen"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/sv-SE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'sw-KE'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Swahili (Kenya) Slugification',
    s.content = 'URL slug generation rules for sw-KE',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"adverb":["sana","tu","kweli"],"interjection":["basi"]}',
    s.stopwords_count = 4,
    s.regional_additions = '[{"word":"","category":"sana","reason":"adverb"},{"word":"","category":"tu","reason":"adverb"},{"word":"","category":"basi","reason":"interjection"},{"word":"","category":"kweli","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"> 60% stopwords removed","message":"May lose meaning"}]',
    s.examples = '[{"input":"Jinsi ya Kupika Ugali","output":"jinsi-kupika-ugali","rules_applied":["Stopwords removed (ya)","lowercase"]},{"input":"Habari za Kenya na Tanzania","output":"habari-kenya-tanzania","rules_applied":["Stopwords removed (za","na)","lowercase"]},{"input":"Safari Bora katika Maasai Mara","output":"safari-bora-maasai-mara","rules_applied":["Stopwords removed (katika)","lowercase"]},{"input":"Teknolojia Mpya kwa Wakulima","output":"teknolojia-mpya-wakulima","rules_applied":["Stopwords removed (kwa)","lowercase"]},{"input":"Biashara na Uwekezaji 2025","output":"biashara-uwekezaji-2025","rules_applied":["Stopwords removed (na)","numbers preserved"]},{"input":"Michezo 10 Bora ya Mtandaoni","output":"michezo-10-bora-mtandaoni","rules_applied":["Stopwords removed (ya)","numbers preserved"]},{"input":"Mwongozo wa Kina kuhusu Usalama wa Mtandao na Ulinzi wa Data kwa Biashara Ndogo","output":"mwongozo-kina-usalama-mtandao-ulinzi-data-biashara-ndogo","rules_applied":["Truncated at 80 chars","stopwords removed"]},{"input":"Chakula & Vinywaji: Mapishi ya Kenya","output":"chakula-vinywaji-mapishi-kenya","rules_applied":["Ampersand removed","colon removed","stopwords removed (ya)"]},{"input":"Ng\'ombe Bora wa Maziwa","output":"ngombe-bora-maziwa","rules_applied":["Apostrophe removed (ng\'ombe -> ngombe)","stopwords removed (wa)"]},{"input":"M-Pesa: Jinsi ya Kutuma Pesa","output":"m-pesa-jinsi-kutuma-pesa","rules_applied":["Hyphen in M-Pesa preserved","colon removed","stopwords removed (ya)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/sw-KE.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'sw-TZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'SW (TZ) Slugification',
    s.content = 'URL slug generation rules for sw-TZ',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"adverb":["hapa","pale","sana"],"preposition":["ya","wa","kwa","katika","la"],"conjunction":["au","lakini","kama"],"demonstrative":["hii","hiyo","hizi","ile"]}',
    s.stopwords_count = 15,
    s.regional_additions = '[{"word":"","category":"hapa","reason":"adverb"},{"word":"","category":"pale","reason":"adverb"},{"word":"","category":"sana","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Too many stopwords removed (> 60%)","message":"Title may be too generic"}]',
    s.examples = '[{"input":"Habari za Asubuhi","output":"habari-asubuhi","rules_applied":["lowercase","stopword \"za\" removed"]},{"input":"Jinsi ya Kupika Ugali","output":"jinsi-kupika-ugali","rules_applied":["stopword \"ya\" removed"]},{"input":"Mazingira na Utalii Tanzania","output":"mazingira-utalii-tanzania","rules_applied":["stopword \"na\" removed"]},{"input":"Soko la Kariakoo Dar es Salaam","output":"soko-kariakoo-dar-es-salaam","rules_applied":["stopword \"la\" removed"]},{"input":"Kilimanjaro ni Mlima Mrefu","output":"kilimanjaro-mlima-mrefu","rules_applied":["stopword \"ni\" removed"]},{"input":"Bidhaa 10 Bora za Mwaka 2025","output":"bidhaa-10-bora-mwaka-2025","rules_applied":["numbers kept","stopword \"za\" removed"]},{"input":"Mwongozo Kamili wa Kutembelea Visiwa vya Zanzibar na Pemba kwa Watalii Wapya","output":"mwongozo-kamili-kutembelea-visiwa-zanzibar-pemba-watalii-wapya","rules_applied":["truncated","stopwords \"wa\"","\"vya\"","\"na\"","\"kwa\" removed"]},{"input":"Habari! Nini Kinaendelea?","output":"habari-nini-kinaendelea","rules_applied":["punctuation removed"]},{"input":"\"Hakuna Matata\" - Msemo wa Kiswahili","output":"hakuna-matata-msemo-kiswahili","rules_applied":["quotes removed","stopword \"wa\" removed"]},{"input":"Ng\'ombe na Kilimo","output":"ngombe-kilimo","rules_applied":["apostrophe in ng\' removed","stopword \"na\" removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/sw-TZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ta-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'TA (IN) Slugification',
    s.content = 'URL slug generation rules for ta-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"verb":["உள்ள","இருக்கும்","செய்யும்"],"pronoun":["அது","இது","அவர்","இவர்"],"interrogative":["என்ன","எப்படி","ஏன்"],"conjunction":["என்று","மற்றும்","ஆனால்","அல்லது"],"article":["ஒரு","அந்த","இந்த"]}',
    s.stopwords_count = 17,
    s.regional_additions = '[{"word":"","category":"தான்","reason":"emphatic particle"},{"word":"","category":"போல","reason":"postposition"},{"word":"","category":"வரை","reason":"postposition"},{"word":"","category":"விட","reason":"postposition"},{"word":"","category":"கூட","reason":"particle"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Tamil mixed with other non-Latin scripts"},{"condition":"No Tamil chars","message":"Slug contains no Tamil script characters"}]',
    s.examples = '[{"input":"தமிழ்நாடு சுற்றுலா வழிகாட்டி","output":"தமிழ்நாடு-சுற்றுலா-வழிகாட்டி","rules_applied":["NFC","spacing"]},{"input":"சென்னை உணவகங்கள் மற்றும் சாப்பாடு","output":"சென்னை-உணவகங்கள்-சாப்பாடு","rules_applied":["Stopword removal (மற்றும்)"]},{"input":"புதிய திரைப்படம் வெளியீடு","output":"புதிய-திரைப்படம்-வெளியீடு","rules_applied":["NFC","spacing"]},{"input":"கிரிக்கெட் போட்டி அறிவிப்பு","output":"கிரிக்கெட்-போட்டி-அறிவிப்பு","rules_applied":["NFC","spacing"]},{"input":"அந்த பழைய கோயில் வரலாறு","output":"பழைய-கோயில்-வரலாறு","rules_applied":["Stopword removal (அந்த)"]},{"input":"2025 தமிழ் புத்தாண்டு கொண்டாட்டம்","output":"2025-தமிழ்-புத்தாண்டு-கொண்டாட்டம்","rules_applied":["Numbers preserved"]},{"input":"இந்திய பொருளாதாரம் மற்றும் வளர்ச்சி குறித்த முழுமையான ஆய்வு அறிக்கை விவரங்கள் இங்கே காணலாம்","output":"இந்திய-பொருளாதாரம்-வளர்ச்சி-குறித்த-முழுமையான-ஆய்வு-அறிக்கை-விவரங்கள்-இங்கே-காணலாம்","rules_applied":["Truncated at 80 chars"]},{"input":"சென்னை: நகர வரைபடம் & போக்குவரத்து!","output":"சென்னை-நகர-வரைபடம்-போக்குவரத்து","rules_applied":["Special chars removed (: & !)"]},{"input":"\"புதுக்கோட்டை\" சரித்திரம்","output":"புதுக்கோட்டை-சரித்திரம்","rules_applied":["Quotes removed"]},{"input":"ஸ்ரீரங்கம் திருக்கோயில்","output":"ஸ்ரீரங்கம்-திருக்கோயில்","rules_applied":["Grantha letter (ஸ்ரீ) preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ta-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ta-LK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'TA (LK) Slugification',
    s.content = 'URL slug generation rules for ta-LK',
    s.slug_rule = 'native_script',
    s.stopwords = '{"article":["ஒரு"],"pronoun":["அது","இது","அவர்","அவள்"],"conjunction":["மற்றும்","அல்லது","ஆனால்"]}',
    s.stopwords_count = 8,
    s.regional_additions = '[{"word":"","category":"லங்கை","reason":"geographic"},{"word":"","category":"இலங்கை","reason":"geographic"},{"word":"","category":"சிங்கள","reason":"ethnic"},{"word":"","category":"யாழ்ப்பாணம்","reason":"geographic"},{"word":"","category":"கொழும்பு","reason":"geographic"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Tamil mixed with other non-Latin scripts detected"},{"condition":"No Tamil chars","message":"Slug contains only Latin/digits"}]',
    s.examples = '[{"input":"இலங்கை சுற்றுலா வழிகாட்டி","output":"சுற்றுலா-வழிகாட்டி","rules_applied":["stopword removal (இலங்கை)","hyphenation"]},{"input":"கொழும்பு உணவகங்கள்","output":"உணவகங்கள்","rules_applied":["stopword removal (கொழும்பு)","hyphenation"]},{"input":"தமிழ் இலக்கியம் மற்றும் கலை","output":"தமிழ்-இலக்கியம்-கலை","rules_applied":["stopword removal (மற்றும்)","hyphenation"]},{"input":"யாழ்ப்பாண பாரம்பரிய உணவுகள்","output":"பாரம்பரிய-உணவுகள்","rules_applied":["stopword removal","hyphenation"]},{"input":"சிறந்த கடற்கரை இடங்கள்","output":"சிறந்த-கடற்கரை-இடங்கள்","rules_applied":["hyphenation"]},{"input":"2025 புத்தாண்டு கொண்டாட்டங்கள்","output":"2025-புத்தாண்டு-கொண்டாட்டங்கள்","rules_applied":["number handling","hyphenation"]},{"input":"இலங்கையின் வரலாற்று முக்கியத்துவம் வாய்ந்த கோட்டைகள் மற்றும் அரண்மனைகள்","output":"இலங்கையின்-வரலாற்று-முக்கியத்துவம்-வாய்ந்த-கோட்டைகள்-அரண்மனைகள்","rules_applied":["stopword removal","truncation at 80 chars"]},{"input":"தேநீர்: இலங்கையின் பெருமை!","output":"தேநீர்-இலங்கையின்-பெருமை","rules_applied":["punctuation removed","hyphenation"]},{"input":"\"பொங்கல்\" திருநாள் சிறப்புகள்","output":"பொங்கல்-திருநாள்-சிறப்புகள்","rules_applied":["quotes removed","hyphenation"]},{"input":"ஶ்ரீ லங்கா கிரிக்கெட்","output":"ஶ்ரீ-லங்கா-கிரிக்கெட்","rules_applied":["Grantha letter (ஶ்ரீ) preserved","hyphenation"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ta-LK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'te-IN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'TE (IN) Slugification',
    s.content = 'URL slug generation rules for te-IN',
    s.slug_rule = 'native_script',
    s.stopwords = '{"quantifier":["అన్ని","కొన్ని"]}',
    s.stopwords_count = 2,
    s.regional_additions = '[{"word":"","category":"గా","reason":"particle"},{"word":"","category":"కు","reason":"postposition"},{"word":"","category":"యొక్క","reason":"postposition"},{"word":"","category":"అన్ని","reason":"quantifier"},{"word":"","category":"కొన్ని","reason":"quantifier"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Non-Telugu characters detected (except digits)"},{"condition":"Non-standard composition","message":"Check Unicode normalization (NFC expected)"}]',
    s.examples = '[{"input":"తెలుగు భాష నేర్చుకోండి","output":"తెలుగు-భాష-నేర్చుకోండి","rules_applied":["Spaces to hyphens"]},{"input":"హైదరాబాద్ యాత్ర గైడ్","output":"హైదరాబాద్-యాత్ర-గైడ్","rules_applied":["Spaces to hyphens"]},{"input":"ఆంధ్రప్రదేశ్ వంటకాలు","output":"ఆంధ్రప్రదేశ్-వంటకాలు","rules_applied":["Spaces to hyphens"]},{"input":"విజయవాడలో ఉత్తమ రెస్టారెంట్లు","output":"విజయవాడలో-ఉత్తమ-రెస్టారెంట్లు","rules_applied":["Spaces to hyphens"]},{"input":"తిరుపతి దేవాలయం సందర్శన","output":"తిరుపతి-దేవాలయం-సందర్శన","rules_applied":["Spaces to hyphens"]},{"input":"2025లో టాప్ 10 సినిమాలు","output":"2025లో-టాప్-10-సినిమాలు","rules_applied":["Numbers preserved","spaces to hyphens"]},{"input":"తెలంగాణ రాష్ట్రంలో అత్యుత్తమ పర్యాటక ప్రదేశాలు మరియు సందర్శనీయ స్థలాలు","output":"తెలంగాణ-రాష్ట్రంలో-అత్యుత్తమ-పర్యాటక-ప్రదేశాలు-సందర్శనీయ-స్థలాలు","rules_applied":["Long title","stopword మరియు removed","truncated if >80"]},{"input":"\"బాహుబలి\" - ఒక అద్భుత చిత్రం!","output":"బాహుబలి-అద్భుత-చిత్రం","rules_applied":["Quotes/punctuation removed","stopword ఒక removed"]},{"input":"రామ్ చరణ్ \'RRR\' సినిమా","output":"రామ్-చరణ్-rrr-సినిమా","rules_applied":["Quotes removed","Latin lowercase"]},{"input":"అక్షరమాల: అ ఆ ఇ ఈ","output":"అక్షరమాల-అ-ఆ-ఇ-ఈ","rules_applied":["Colon removed","vowels preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/te-IN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'tg-TJ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'TG (TJ) Slugification',
    s.content = 'URL slug generation rules for tg-TJ',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Non-Cyrillic characters detected (except numbers)"},{"condition":"Non-standard Unicode","message":"Improper NFC normalization detected"}]',
    s.examples = '[{"input":"Хуш омадед ба Тоҷикистон","output":"хуш-омадед-тоҷикистон","rules_applied":["Lowercase + stopword (ба) removed"]},{"input":"Ахбори Тоҷикистон 2025","output":"ахбори-тоҷикистон-2025","rules_applied":["Lowercase + numbers preserved"]},{"input":"Душанбе - пойтахти Тоҷикистон","output":"душанбе-пойтахти-тоҷикистон","rules_applied":["Lowercase + punctuation removed"]},{"input":"Фарҳанги тоҷик дар замони навин","output":"фарҳанги-тоҷик-замони-навин","rules_applied":["Lowercase + stopwords (дар) removed"]},{"input":"Мусиқии мардумии тоҷикӣ","output":"мусиқии-мардумии-тоҷикӣ","rules_applied":["Lowercase + special char (ӣ) preserved"]},{"input":"10 ҷойҳои зебо дар Тоҷикистон","output":"10-ҷойҳои-зебо-тоҷикистон","rules_applied":["Numbers at start preserved + stopword (дар) removed"]},{"input":"Роҳнамои сайёҳӣ барои сафар ба Помир ва водии Фарғона дар соли 2025","output":"роҳнамои-сайёҳӣ-сафар-помир-водии-фарғона-соли-2025","rules_applied":["Long title + stopwords (барои","ба","ва","дар) removed + truncated"]},{"input":"Таърих! Фарҳанг? Оянда…","output":"таърих-фарҳанг-оянда","rules_applied":["All punctuation removed"]},{"input":"\"Шоҳнома\" - шоҳкории адабиёти тоҷик","output":"шоҳнома-шоҳкории-адабиёти-тоҷик","rules_applied":["Quotes removed + hyphen preserved"]},{"input":"Қуллаи Исмоили Сомонӣ ва кӯҳҳои Помир","output":"қуллаи-исмоили-сомонӣ-кӯҳҳои-помир","rules_applied":["Tajik-specific letters (қ","ӣ","ӯ","ҳ) preserved + stopword (ва) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/tg-TJ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'th-TH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Thai (Thailand) Slugification',
    s.content = 'URL slug generation rules for th-TH',
    s.slug_rule = 'native_script',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":"convert_thai","special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Non-Thai characters detected (except numbers)"},{"condition":"Non-standard composition","message":"Invalid Unicode normalization"}]',
    s.examples = '[{"input":"วิธีการทำอาหารไทย","output":"วิธีทำอาหารไทย","rules_applied":["Stopword removal (การ)","Thai preserved"]},{"input":"คู่มือท่องเที่ยวกรุงเทพฯ","output":"คู่มือท่องเที่ยวกรุงเทพ","rules_applied":["Special mark removed (ฯ)","Thai preserved"]},{"input":"เทคนิคการเขียนโค้ด","output":"เทคนิคเขียนโค้ด","rules_applied":["Stopword removal (การ)","Thai preserved"]},{"input":"ร้านอาหารที่ดีที่สุดในกรุงเทพ","output":"ร้านอาหารดีสุดกรุงเทพ","rules_applied":["Multiple stopwords (ที่","ใน)","Thai preserved"]},{"input":"ข่าวสาร & เหตุการณ์ ๒๐๒๕","output":"ข่าวสาร-เหตุการณ์-2025","rules_applied":["Special char removed (&)","Thai numerals converted"]},{"input":"แนะนำสถานที่ท่องเที่ยว 10 อันดับ","output":"แนะนำสถานท่องเที่ยว-10-อันดับ","rules_applied":["Stopword removal (ที่)","numbers preserved"]},{"input":"บทความเกี่ยวกับเทคโนโลยีและนวัตกรรมสำหรับการพัฒนาธุรกิจในยุคดิจิทัลปัจจุบัน","output":"บทความเกี่ยวเทคโนโลยีนวัตกรรมสำหรับพัฒนาธุรกิจยุคดิจิทัลปัจจุบัน","rules_applied":["Stopwords removed","truncate >80 chars"]},{"input":"ราคา! พิเศษ? สำหรับวันนี้","output":"ราคา-พิเศษ-สำหรับวันนี้","rules_applied":["Punctuation removed (!","?)","Thai preserved"]},{"input":"\"เคล็ดลับ\" สำหรับมือใหม่","output":"เคล็ดลับ-สำหรับมือใหม่","rules_applied":["Quotes removed","Thai preserved"]},{"input":"วิธีใช้งาน API ได้อย่างมีประสิทธิภาพ","output":"วิธีใช้งาน-api-มีประสิทธิภาพ","rules_applied":["Mixed script (API preserved)","stopwords removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/th-TH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'tk-TM'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'TK (TM) Slugification',
    s.content = 'URL slug generation rules for tk-TM',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"article":["bir","birnäçe"],"verb":["bolsa","bar"],"adverb":["ýok","has","iň"],"pronoun":["bu","şu","ol"],"conjunction":["we","hem","ýa-da","hem-de"],"preposition":["bilen","üçin"]}',
    s.stopwords_count = 16,
    s.regional_additions = '[{"word":"","category":"barada","reason":"postposition"},{"word":"","category":"hakynda","reason":"postposition"},{"word":"hem-de","category":"conjunction","reason":"Common compound conjunction in Turkmen"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed Latin scripts","message":"Verify character set is Turkmen-appropriate"}]',
    s.examples = '[{"input":"Aşgabatda Iň Gowy Restoranlar","output":"aşgabatda-gowy-restoranlar","rules_applied":["Stopword (iň) removed","ş preserved"]},{"input":"Türkmen Dilini Öwrenmegiň 10 Usuly","output":"türkmen-dilini-öwrenmegiň-10-usuly","rules_applied":["Turkmen chars preserved","number kept"]},{"input":"Çagalar üçin Gyzykly Oýunlar","output":"çagalar-gyzykly-oýunlar","rules_applied":["üçin removed (stopword)","ü preserved"]},{"input":"Sagdyn Durmuş we Iýmitlenmek Gollanmasy","output":"sagdyn-durmuş-iýmitlenmek-gollanmasy","rules_applied":["we removed","ş/ý preserved"]},{"input":"Awtoulag Satyn Almagyň Düzgünleri","output":"awtoulag-satyn-almagyň-düzgünleri","rules_applied":["ü preserved","no stopwords"]},{"input":"2025-nji Ýylda Tehnologiýa Tendensiýalary","output":"2025-nji-ýylda-tehnologiýa-tendensiýalary","rules_applied":["Year number kept","ý preserved"]},{"input":"Uzyn Makala Ady Mysaly: Türkmenistanda Sanly Özgertmeler we Geljek Mümkinçilikleri","output":"uzyn-makala-ady-mysaly-türkmenistanda-sanly-özgertmeler-geljek-mümkinçilikleri","rules_applied":["Long title truncated","we removed","colon removed"]},{"input":"Kofe, Çaý & Süýjüler!","output":"kofe-çaý-süýjüler","rules_applied":["Ampersand removed","punctuation removed","ý/ü preserved"]},{"input":"\"Ajaýyp\" Bir Gün Başlaýar","output":"ajaýyp-gün-başlaýar","rules_applied":["Quotes removed","bir removed","ý/ş preserved"]},{"input":"Iş & Şahsy Durmuş Deňagramlylygy","output":"iş-şahsy-durmuş-deňagramlylygy","rules_applied":["Ampersand removed","ş/ň preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/tk-TM.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'tl-PH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'TL (PH) Slugification',
    s.content = 'URL slug generation rules for tl-PH',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["kasi"]}',
    s.stopwords_count = 1,
    s.regional_additions = '[{"word":"","category":"po","reason":"politeness marker"},{"word":"","category":"ho","reason":"politeness marker"},{"word":"","category":"daw","reason":"hearsay marker"},{"word":"","category":"raw","reason":"hearsay marker"},{"word":"","category":"kasi","reason":"conjunction"},{"word":"","category":"pala","reason":"discourse particle"},{"word":"","category":"eh","reason":"discourse particle"},{"word":"","category":"naman","reason":"discourse particle"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed","message":"> 60% of words removed"}]',
    s.examples = '[{"input":"Paano Gumawa ng Website para sa Negosyo","output":"paano-gumawa-website-negosyo","rules_applied":["Stopwords removed: ng","para","sa"]},{"input":"Mga Tips sa Pagluluto ng Adobo","output":"tips-pagluluto-adobo","rules_applied":["Stopwords removed: mga","sa","ng"]},{"input":"Gabay sa Paghahanap ng Trabaho sa Pilipinas","output":"gabay-paghahanap-trabaho-pilipinas","rules_applied":["Stopwords removed: sa","ng","sa"]},{"input":"Ano ang Pinakamahusay na Smartphone ngayong 2024","output":"ano-pinakamahusay-smartphone-ngayong-2024","rules_applied":["Stopwords removed: ang","na"]},{"input":"Kalusugan at Kaligayahan ng Pamilya","output":"kalusugan-kaligayahan-pamilya","rules_applied":["Stopwords removed: at","ng"]},{"input":"10 Paraan para Makatipid sa Kuryente","output":"10-paraan-makatipid-kuryente","rules_applied":["Numbers preserved; Stopwords removed: para","sa"]},{"input":"Paano Magpatayo ng Bahay na Matibay at Ligtas para sa mga Pilipinong Pamilya sa Buong Bansa","output":"paano-magpatayo-bahay-matibay-ligtas-pilipinong-pamilya-buong-bansa","rules_applied":["Truncated at 80 chars; Stopwords removed: ng","na","at","para","sa","mga","sa"]},{"input":"Balita: Mga Update sa Eleksyon!","output":"balita-update-eleksyon","rules_applied":["Punctuation removed: :","!; Stopwords removed: mga","sa"]},{"input":"\"Sining ng Pagsasalita\" - Gabay para sa mga Estudyante","output":"sining-pagsasalita-gabay-estudyante","rules_applied":["Quotes removed; Dash removed; Stopwords removed: ng","para","sa","mga"]},{"input":"Kuwento ni Mang Pandoy (Buong Istorya)","output":"kuwento-mang-pandoy-buong-istorya","rules_applied":["Parentheses removed; Stopwords removed: ni"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/tl-PH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'tr-TR'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Turkish (Turkey) Slugification',
    s.content = 'URL slug generation rules for tr-TR',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"pronoun":["bu","şu","o","olan"],"article":["bir"],"adverb":["daha","çok","en"],"conjunction":["ve","veya","da","de","ya"],"preposition":["ile","için"]}',
    s.stopwords_count = 15,
    s.regional_additions = '[{"word":"","category":"olan","reason":"pronoun"},{"word":"","category":"gibi","reason":"postposition"},{"word":"","category":"kadar","reason":"postposition"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed Latin scripts","message":"Verify character set is Turkish-appropriate"}]',
    s.examples = '[{"input":"İstanbul\'un En İyi Restoranları","output":"istanbulun-iyi-restoranları","rules_applied":["Apostrophe removed","stopwords (en) removed","ı preserved"]},{"input":"Türkçe Öğrenmenin 10 Kolay Yolu","output":"türkçe-öğrenmenin-10-kolay-yolu","rules_applied":["Turkish chars preserved","number kept"]},{"input":"Çocuklar için Eğlenceli Oyunlar","output":"çocuklar-eğlenceli-oyunlar","rules_applied":["için removed (stopword)","ç preserved"]},{"input":"Başarılı Bir Kariyer İçin Öneriler","output":"başarılı-kariyer-öneriler","rules_applied":["bir/için removed","ş/ı preserved"]},{"input":"Sağlıklı Yaşam ve Beslenme Rehberi","output":"sağlıklı-yaşam-beslenme-rehberi","rules_applied":["ve removed","ğ/ş preserved"]},{"input":"2025 Yılı Teknoloji Trendleri","output":"2025-yılı-teknoloji-trendleri","rules_applied":["Year number kept","all Turkish chars preserved"]},{"input":"Uzun Bir Makale Başlığı Örneği: Türkiye\'de Dijital Dönüşüm ve Gelecek","output":"uzun-makale-başlığı-örneği-türkiyede-dijital-dönüşüm-gelecek","rules_applied":["Long title truncated","bir/ve removed","colon removed"]},{"input":"Kahve, Çay & Tatlılar!","output":"kahve-çay-tatlılar","rules_applied":["Ampersand removed","punctuation removed","ç preserved"]},{"input":"\"Güzel\" Bir Gün Başlıyor","output":"güzel-gün-başlıyor","rules_applied":["Quotes removed","bir removed","ü/ğ preserved"]},{"input":"İş & Özel Yaşam Dengesi","output":"iş-özel-yaşam-dengesi","rules_applied":["Ampersand removed","Turkish ş/ö/ı preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/tr-TR.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'uk-UA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Ukrainian (Ukraine) Slugification',
    s.content = 'URL slug generation rules for uk-UA',
    s.slug_rule = 'native_script',
    s.stopwords = '{"adverb":["як"],"conjunction":["і","та","або","але"],"preposition":["в","на","з","до","від","для"],"pronoun":["що","це","той","цей"]}',
    s.stopwords_count = 15,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for meaningful content"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Mixed scripts","message":"Avoid mixing Cyrillic with Latin (except technical terms)"},{"condition":"Apostrophe present","message":"Ukrainian apostrophe (ʼ) should be removed during processing"}]',
    s.examples = '[{"input":"Новини України 2025","output":"новини-україни-2025","rules_applied":["Lowercase","spaces→hyphens","Cyrillic preserved"]},{"input":"Як приготувати борщ","output":"приготувати-борщ","rules_applied":["Stopword \"як\" removed","lowercase"]},{"input":"Подорожі по Києву та Львову","output":"подорожі-києву-львову","rules_applied":["Stopwords \"по\"","\"та\" removed"]},{"input":"Найкращі технології для бізнесу","output":"найкращі-технології-бізнесу","rules_applied":["Stopword \"для\" removed","diacritics preserved"]},{"input":"Історія України від давнини до сучасності","output":"історія-україни-давнини-сучасності","rules_applied":["Stopwords \"від\"","\"до\" removed"]},{"input":"Інтернет-магазин №1 в Україні","output":"інтернет-магазин-1-україні","rules_applied":["Special chars removed","number preserved"]},{"input":"Довгий заголовок про культуру мистецтво історію та традиції українського народу в сучасному світі","output":"довгий-заголовок-культуру-мистецтво-історію-традиції-українського-на","rules_applied":["Truncated at 80 chars","multiple stopwords removed"]},{"input":"\"Кращі страви\": топ-10 рецептів!","output":"кращі-страви-топ-10-рецептів","rules_applied":["Quotes removed","punctuation removed"]},{"input":"Україна – незалежна держава","output":"україна-незалежна-держава","rules_applied":["Em-dash removed","spaces normalized"]},{"input":"Київ, Харків і Одеса","output":"київ-харків-одеса","rules_applied":["Comma removed","stopword \"і\" removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/uk-UA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'ur-PK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'UR (PK) Slugification',
    s.content = 'URL slug generation rules for ur-PK',
    s.slug_rule = 'native_script',
    s.stopwords = '{"adverb":["بھی"],"demonstrative":["یہ","وہ"],"conjunction":["تو"]}',
    s.stopwords_count = 4,
    s.regional_additions = '[{"word":"","category":"یہ","reason":"demonstrative"},{"word":"","category":"وہ","reason":"demonstrative"},{"word":"","category":"جو","reason":"relative"},{"word":"","category":"تو","reason":"conjunction"},{"word":"","category":"بھی","reason":"adverb"}]',
    s.script_config = '{"primary_script":"arabic","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Arabic and Latin mixed (except technical terms)"},{"condition":"Non-standard Unicode","message":"Non-NFC normalization detected"}]',
    s.examples = '[{"input":"پاکستان کی تاریخ","output":"پاکستان-تاریخ","rules_applied":["NFC","stopword کی removed","spaces to hyphens"]},{"input":"اردو زبان سیکھیں","output":"اردو-زبان-سیکھیں","rules_applied":["NFC","stopword removed","spaces to hyphens"]},{"input":"کراچی میں سیاحت","output":"کراچی-سیاحت","rules_applied":["NFC","stopword میں removed","spaces to hyphens"]},{"input":"لاہور کا قلعہ","output":"لاہور-قلعہ","rules_applied":["NFC","stopword کا removed","spaces to hyphens"]},{"input":"اسلام آباد گائیڈ","output":"اسلام-آباد-گائیڈ","rules_applied":["NFC","preserve Urdu","spaces to hyphens"]},{"input":"2025 میں پاکستان","output":"2025-پاکستان","rules_applied":["Numbers preserved","stopword میں removed"]},{"input":"پاکستان کی خوبصورت وادیوں اور پہاڑوں کا سفر نامہ اور تجربات","output":"پاکستان-خوبصورت-وادیوں-پہاڑوں-سفر-نامہ-تجربات","rules_applied":["Long title","stopwords removed","max 80 chars"]},{"input":"خوش آمدید! پاکستان میں","output":"خوش-آمدید-پاکستان","rules_applied":["Punctuation removed","stopword میں removed"]},{"input":"\"بہترین\" کھانے کی جگہیں","output":"بہترین-کھانے-جگہیں","rules_applied":["Quotes removed","stopword کی removed"]},{"input":"ٹیکنالوجی اور ای کامرس","output":"ٹیکنالوجی-ای-کامرس","rules_applied":["Stopword اور removed","preserve Urdu tech terms"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/ur-PK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'uz-UZ'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'UZ (UZ) Slugification',
    s.content = 'URL slug generation rules for uz-UZ',
    s.slug_rule = 'latin_preserve',
    s.stopwords = '{"conjunction":["va","yoki","ham","esa"],"pronoun":["bu","shu","u"],"adverb":["keyin","oldin"]}',
    s.stopwords_count = 9,
    s.regional_additions = '[{"word":"","category":"oʻzbekiston","reason":"proper noun"},{"word":"","category":"toshkent","reason":"proper noun"},{"word":"","category":"davlat","reason":"noun"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"ASCII apostrophe in output","message":"Should be normalized to modifier letter (U+02BB)"}]',
    s.examples = '[{"input":"Oʻzbekiston Tarixi","output":"oʻzbekiston-tarixi","rules_applied":["Modifier letter preserved","lowercase"]},{"input":"Toshkentning Eng Yaxshi Restoranlari","output":"toshkentning-yaxshi-restoranlari","rules_applied":["eng removed (stopword)","lowercase"]},{"input":"Bolalar uchun Oʻyin va Oʻquv","output":"bolalar-oʻyin-oʻquv","rules_applied":["uchun/va removed (stopwords)","oʻ preserved"]},{"input":"Sogʻlom Turmush Tarzi Haqida","output":"sogʻlom-turmush-tarzi","rules_applied":["haqida removed (stopword)","gʻ preserved"]},{"input":"Biznes va Iqtisodiyot Yangiliklari","output":"biznes-iqtisodiyot-yangiliklari","rules_applied":["va removed (stopword)"]},{"input":"2025 Yilgi Texnologiya Tendensiyalari","output":"2025-yilgi-texnologiya-tendensiyalari","rules_applied":["Year number kept","all chars preserved"]},{"input":"Juda Uzun Sarlavha Misoli: Oʻzbekistonda Raqamli Transformatsiya va Kelajak Rivojlanishi","output":"juda-uzun-sarlavha-misoli-oʻzbekistonda-raqamli-transformatsiya-kelajak","rules_applied":["Long title truncated at 80 chars","va removed"]},{"input":"Choy, Qahva & Shirinliklar!","output":"choy-qahva-shirinliklar","rules_applied":["Ampersand removed","punctuation removed","ch digraph preserved"]},{"input":"\"Goʻzal\" Bir Kun Boshlanmoqda","output":"goʻzal-kun-boshlanmoqda","rules_applied":["Quotes removed","bir removed (stopword)","gʻ preserved"]},{"input":"Shoʻrva & Mazzali Taomlar","output":"shoʻrva-mazzali-taomlar","rules_applied":["Ampersand removed","oʻ preserved","sh digraph preserved"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/uz-UZ.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'vi-VN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Vietnamese (Vietnam) Slugification',
    s.content = 'URL slug generation rules for vi-VN',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["của","cho","với","từ","đến","tại","trong","ngoài"],"article":["các","những","một"],"conjunction":["và","hoặc","nhưng","nếu"],"demonstrative":["này","đó","kia","đây"],"copula":["là"]}',
    s.stopwords_count = 20,
    s.regional_additions = '[]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"Too many stopwords removed","message":"> 60% of words removed - title may be too generic"}]',
    s.examples = '[{"input":"Cách Học Tiếng Anh Hiệu Quả","output":"cach-hoc-tieng-anh-hieu-qua","rules_applied":["NFD","strip marks","stopwords removed: (none)","lowercase"]},{"input":"10 Món Ăn Ngon Nhất Việt Nam","output":"10-mon-an-ngon-nhat-viet-nam","rules_applied":["NFD","strip marks","stopwords removed: của (implicit)","numbers preserved"]},{"input":"Du Lịch Đà Nẵng - Hội An 2025","output":"du-lich-da-nang-hoi-an-2025","rules_applied":["NFD","strip marks (đ→d)","punctuation removed","numbers preserved"]},{"input":"Công Nghệ AI & Machine Learning","output":"cong-nghe-ai-machine-learning","rules_applied":["NFD","strip marks","ampersand removed","stopwords removed: (none)"]},{"input":"Bí Quyết Kinh Doanh Thành Công Cho Người Mới","output":"bi-quyet-kinh-doanh-thanh-cong-nguoi-moi","rules_applied":["NFD","strip marks","stopwords removed: cho","lowercase"]},{"input":"Review Điện Thoại iPhone 15 Pro Max - Có Nên Mua?","output":"review-dien-thoai-iphone-15-pro-max-nen-mua","rules_applied":["NFD","strip marks","punctuation removed","stopwords removed: có","question mark removed"]},{"input":"Hướng Dẫn Đầu Tư Chứng Khoán Cho Người Mới Bắt Đầu Từ Con Số 0 Với Chiến Lược An Toàn","output":"huong-dan-dau-tu-chung-khoan-nguoi-moi-bat-dau-tu-con-so-0-voi","rules_applied":["NFD","strip marks","stopwords removed: cho","từ","với","truncated at 80 chars"]},{"input":"Phở Hà Nội \"Truyền Thống\" (Công Thức Bí Mật)","output":"pho-ha-noi-truyen-thong-cong-thuc-bi-mat","rules_applied":["NFD","strip marks","quotes removed","parentheses removed","stopwords removed: (none)"]},{"input":"Làm Thế Nào Để Tăng Doanh Thu? Tips & Tricks!","output":"lam-nao-tang-doanh-thu-tips-tricks","rules_applied":["NFD","strip marks","stopwords removed: thế","để","question mark removed","ampersand removed","exclamation removed"]},{"input":"Café Sài Gòn: Văn Hóa Cà Phê Vỉa Hè","output":"cafe-sai-gon-van-hoa-ca-phe-via-he","rules_applied":["NFD","strip marks (é→e","à→a","ê→e)","colon removed","stopwords removed: (none)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/vi-VN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'wo-SN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'WO (SN) Slugification',
    s.content = 'URL slug generation rules for wo-SN',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[{"word":"","category":"la","reason":"focus marker"},{"word":"","category":"ngi","reason":"presentative"},{"word":"","category":"na","reason":"verb suffix"},{"word":"","category":"dafa","reason":"emphatic"},{"word":"","category":"dinaa","reason":"future marker"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed (> 60%)","message":"May lose meaning"},{"condition":"All content was stopwords","message":"Empty slug - needs manual intervention"}]',
    s.examples = '[{"input":"Ndax jigeen yi meng ngi dem toubab?","output":"ndax-jigeen-dem-toubab","rules_applied":["Stopwords removed (yi","ngi)","diacritics stripped","lowercase"]},{"input":"Benn yoon bu baax ngir xam sa bopp","output":"benn-yoon-baax-ngir-xam-bopp","rules_applied":["Articles removed (bu","sa)","lowercase","hyphenation"]},{"input":"Wolof ak senegaal: ligeey ci kaw internet","output":"wolof-senegaal-ligeey-kaw-internet","rules_applied":["Prepositions removed (ak","ci)","special chars removed (:)"]},{"input":"Kilifa yi di wax ci xalaat bu bees","output":"kilifa-wax-xalaat-bees","rules_applied":["Articles removed (yi","bu)","verb removed (di)","preposition removed (ci)"]},{"input":"Naka landef pour am sante bu baax?","output":"naka-ndef-pour-sante-baax","rules_applied":["Stopwords removed (la","am","bu)","punctuation removed (?)","lowercase"]},{"input":"10 xalaat ngir mujj sa liggey ci 2025","output":"10-xalaat-ngir-mujj-liggey-2025","rules_applied":["Numbers preserved","possessive removed (sa)","preposition removed (ci)"]},{"input":"Dakar dafa am beneen gox bu genn nekk ci Afrik di genn ci doxalin ak xaalis yi ngi ci ekosistem ci tecnoloji bu bari","output":"dakar-beneen-gox-genn-afrik-genn-doxalin-xaalis-ekosistem-tecnoloji-bari","rules_applied":["Truncated to 80 chars","multiple stopwords removed"]},{"input":"Jokko & mbokk: tekki ci biir ker gi!","output":"jokko-mbokk-tekki-biir-ker","rules_applied":["Special chars removed (&",":","!)","articles removed (gi","ci)"]},{"input":"\"Teranga\" moo tax Senegaal genn","output":"teranga-tax-senegaal-genn","rules_applied":["Quotes removed","emphatic (moo) removed","lowercase"]},{"input":"Gaynde yi vs. Elephant yi: kan moo genn?","output":"gaynde-elephant-kan-genn","rules_applied":["Plural articles removed (yi)","punctuation removed (:","?)","emphatic (moo) removed"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/wo-SN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'xh-ZA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'XH (ZA) Slugification',
    s.content = 'URL slug generation rules for xh-ZA',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"preposition":["ku","e","kwa","nge"],"pronoun":["mna","wena","yena","thina"],"demonstrative":["lo","le","eli","aba","ezi","oku","oko"],"conjunction":["na","okanye","kodwa","ukuba","ngenxa","kunye"],"adverb":["njalo","nje"]}',
    s.stopwords_count = 23,
    s.regional_additions = '[{"word":"","category":"kunye","reason":"conjunction"},{"word":"","category":"njalo","reason":"adverb"},{"word":"","category":"ke","reason":"discourse marker"},{"word":"","category":"nje","reason":"adverb"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed","message":"Over 60% removed - verify intent preserved"}]',
    s.examples = '[{"input":"Indlela Yokufunda IsiXhosa","output":"indlela-yokufunda-isixhosa","rules_applied":["Lowercase","stopwords preserved (content words)"]},{"input":"Iindaba Zokugqibela","output":"iindaba-zokugqibela","rules_applied":["Lowercase","spaces to hyphens"]},{"input":"Amaqela e Rugby eMzantsi Afrika","output":"amaqela-rugby-emzantsi-afrika","rules_applied":["Stopword \"e\" removed","lowercase"]},{"input":"Ukutya Kwasemakhaya","output":"ukutya-kwasemakhaya","rules_applied":["Lowercase","no stopwords"]},{"input":"Imbali ka Nelson Mandela","output":"imbali-nelson-mandela","rules_applied":["Stopword \"ka\" preserved (possessive","content-bearing)"]},{"input":"Izinto Ezi-10 Onokuzenza","output":"izinto-10-onokuzenza","rules_applied":["Numbers preserved","\"ezi\" removed"]},{"input":"Ukubaluleka Kolondolozo Lweemeko Zendalo Nokugcinwa Kwamanzi Emzantsi Afrika","output":"ukubaluleka-kolondolozo-lweemeko-zendalo-nokugcinwa-kwamanzi-emzantsi-afrika","rules_applied":["Truncated at 80 chars"]},{"input":"Iincwadi & Amaphepha: Ukufunda","output":"iincwadi-amaphepha-ukufunda","rules_applied":["Ampersand and colon removed"]},{"input":"\"Uxolo\" kunye no \"Enkosi\"","output":"uxolo-enkosi","rules_applied":["Quotes removed","\"kunye\" and \"no\" removed"]},{"input":"UQongqothwane Ngumculo WeQhawe","output":"uqongqothwane-ngumculo-weqhawe","rules_applied":["Click consonants (q","qh) preserved as ASCII"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/xh-ZA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'yo-NG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'YO (NG) Slugification',
    s.content = 'URL slug generation rules for yo-NG',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"negation":["ko"]}',
    s.stopwords_count = 1,
    s.regional_additions = '[{"word":"","category":"naija","reason":"slang"},{"word":"","category":"ko","reason":"negation"},{"word":"","category":"naa","reason":"determiner"},{"word":"","category":"kan","reason":"numeral/article"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"> 60% stopwords removed","message":"Content may be too generic"},{"condition":"All tone marks stripped","message":"Verify meaning preserved"}]',
    s.examples = '[{"input":"Ọjọ Ibi Ọmọ Mi","output":"ojo-ibi-omo","rules_applied":["subdot stripped","tone stripped","stopword \"mi\" kept (possessive)"]},{"input":"Ounjẹ Yoruba ti o dara julọ","output":"ounje-yoruba-dara-julo","rules_applied":["subdot stripped","stopwords \"ti\"","\"o\" removed"]},{"input":"Ilé ẹkọ giga ni Nàìjíríà","output":"ile-eko-giga-naijiriya","rules_applied":["subdot stripped","tone stripped","stopword \"ni\" removed"]},{"input":"Ọja Nla fun Awọn Ọmọde","output":"oja-nla-awon-omode","rules_applied":["subdot stripped","stopword \"fun\" removed"]},{"input":"Àwọn Orin Fújì àti Jùjú","output":"awon-orin-fuji-juju","rules_applied":["tone stripped","stopword \"ati\" removed"]},{"input":"10 Ohun ti o yẹ ki o mọ","output":"10-ohun-ye-mo","rules_applied":["numbers kept","stopwords \"ti\"","\"o\"","\"ki\" removed"]},{"input":"Itan Pataki nipa Awọn Akọni Yoruba lati Igba Atijọ de Oni yi","output":"itan-pataki-nipa-awon-akoni-yoruba-igba-atijo-oni","rules_applied":["long title truncated","stopwords \"lati\"","\"de\"","\"yi\" removed"]},{"input":"Kilode! Ṣe o fẹ lọ si Eko?","output":"kilode-se-fe-lo-eko","rules_applied":["punctuation removed","subdot stripped","stopwords \"o\"","\"si\" removed"]},{"input":"\"Ẹ ku odun\" - Oriki Odun Tuntun","output":"e-ku-odun-oriki-odun-tuntun","rules_applied":["quotes removed","subdot stripped"]},{"input":"Ọ̀rọ̀ Àgbàyanu nipa Ọ̀pọ̀lọpọ̀ Ọ̀rọ̀","output":"oro-agbayanu-nipa-opolopo-oro","rules_applied":["multiple subdots and tones","all stripped to ASCII"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/yo-NG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'zh-CN'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Chinese (Simplified) Slugification',
    s.content = 'URL slug generation rules for zh-CN',
    s.slug_rule = 'romanized',
    s.stopwords = '{}',
    s.stopwords_count = 0,
    s.regional_additions = '[{"word":"","category":"guo","reason":"noun suffix (国)"},{"word":"","category":"ren","reason":"noun (人)"},{"word":"","category":"zhong","reason":"adjective (中)"},{"word":"","category":"gongsi","reason":"noun (公司)"},{"word":"","category":"wang","reason":"noun (网)"}]',
    s.script_config = '{"primary_script":"chinese","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Input contained no Han characters","message":"Expected Han input for zh-CN"},{"condition":"Romanization produced very short slug","message":"May indicate excessive stopword removal"},{"condition":"Too many characters romanized to same syllable","message":"Ambiguous slug"}]',
    s.examples = '[{"input":"关于我们的产品","output":"guanyu-women-chanpin","rules_applied":["Pinyin conversion","stopword removal (de 的)"]},{"input":"2025年最佳实践","output":"2025-nian-zuijia-shijian","rules_applied":["Number preserved","Pinyin conversion"]},{"input":"中国制造：质量保证","output":"zhongguo-zhizao-zhiliang-baozheng","rules_applied":["Punctuation removed","Pinyin conversion"]},{"input":"如何使用这个产品？","output":"ruhe-shiyong-zhege-chanpin","rules_applied":["Question mark removed","Pinyin conversion"]},{"input":"北京到上海的高铁","output":"beijing-shanghai-gaotie","rules_applied":["Stopwords removed (dao 到","de 的)"]},{"input":"企业级云计算解决方案2024","output":"qiyeji-yun-jisuan-jiejue-fangan-2024","rules_applied":["Number at end","full Pinyin conversion"]},{"input":"在线教育平台：免费课程资源与学习工具推荐","output":"zaixian-jiaoyu-pingtai-mianfei-kecheng-ziyuan-xuexi-gongju-tuijian","rules_applied":["Long title","punctuation removed","full conversion"]},{"input":"\"人工智能\" & 机器学习","output":"rengong-zhineng-jiqi-xuexi","rules_applied":["Quotes and ampersand removed"]},{"input":"深圳·香港·广州","output":"shenzhen-xianggang-guangzhou","rules_applied":["Middle dot removed","city names"]},{"input":"中文网址的SEO优化技巧","output":"zhongwen-wangzhi-seo-youhua-jiqiao","rules_applied":["Latin acronym preserved","Pinyin for Han"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/zh-CN.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'zh-HK'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ZH (HK) Slugification',
    s.content = 'URL slug generation rules for zh-HK',
    s.slug_rule = 'native_script',
    s.stopwords = '{"preposition":["喺"],"verb":["係"],"conjunction":["同","及"]}',
    s.stopwords_count = 4,
    s.regional_additions = '[{"word":"","category":"嘅","reason":"particle"},{"word":"","category":"係","reason":"verb"},{"word":"","category":"喺","reason":"preposition"},{"word":"","category":"同","reason":"conjunction"},{"word":"","category":"及","reason":"conjunction"}]',
    s.script_config = '{"primary_script":"chinese","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed scripts","message":"Unexpected if not technical terms"},{"condition":"Simplified characters detected","message":"Use Traditional Chinese for zh-HK"}]',
    s.examples = '[{"input":"香港旅遊指南","output":"香港旅遊指南","rules_applied":["NFC normalize","preserve Chinese"]},{"input":"中環購物攻略","output":"中環購物攻略","rules_applied":["NFC normalize","preserve Chinese"]},{"input":"港式飲茶文化","output":"港式飲茶文化","rules_applied":["NFC normalize","preserve Chinese"]},{"input":"維多利亞港夜景","output":"維多利亞港夜景","rules_applied":["NFC normalize","preserve Chinese"]},{"input":"太平山頂觀景台","output":"太平山頂觀景台","rules_applied":["NFC normalize","preserve Chinese"]},{"input":"2025年新年活動","output":"2025年新年活動","rules_applied":["Numbers preserved","Chinese kept"]},{"input":"香港迪士尼樂園遊玩攻略及交通指南完整版二零二五年更新","output":"香港迪士尼樂園遊玩攻略及交通指南完整版二零二五年更","rules_applied":["Truncated at 80 chars"]},{"input":"尖沙咀：美食、購物、景點！","output":"尖沙咀美食購物景點","rules_applied":["Punctuation removed"]},{"input":"「必試」港式奶茶推介","output":"必試港式奶茶推介","rules_applied":["Corner brackets removed"]},{"input":"長洲搶包山 vs 大澳水鄉","output":"長洲搶包山-vs-大澳水鄉","rules_applied":["Spaces to hyphens","mixed kept"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/zh-HK.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'zh-SG'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ZH (SG) Slugification',
    s.content = 'URL slug generation rules for zh-SG',
    s.slug_rule = 'romanized',
    s.stopwords = '{"verb":["是 (shi)","有 (you)"],"pronoun":["这 (zhe)","那 (na)"],"classifier":["个 (ge)"],"conjunction":["和 (he)","与 (yu)","或 (huo)"],"preposition":["在 (zai)","为 (wei)"]}',
    s.stopwords_count = 10,
    s.regional_additions = '[{"word":"啦 (la)","category":"particle","reason":"Singapore colloquial particle from Singlish"},{"word":"咯 (lo)","category":"particle","reason":"Singapore colloquial particle from Singlish"},{"word":"吗 (ma)","category":"particle","reason":"Question particle common in Singapore Chinese"},{"word":"呢 (ne)","category":"particle","reason":"Sentence-final particle"},{"word":"哦 (o)","category":"particle","reason":"Singapore colloquial acknowledgment particle"}]',
    s.script_config = '{"primary_script":"chinese","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short for SEO"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Mixed script input","message":"Verify all Han characters romanized"},{"condition":"English loanwords","message":"Retain original romanization for Singapore-specific terms"}]',
    s.examples = '[{"input":"新加坡美食指南","output":"xinjiapo-meishi-zhinan","rules_applied":["Romanize","lowercase","hyphenate"]},{"input":"最好的购物中心","output":"zuihao-gouwu-zhongxin","rules_applied":["Romanize","remove 的 (stopword)"]},{"input":"牛车水旅游攻略","output":"niucheshui-lvyou-gonglue","rules_applied":["Romanize","ü→v conversion"]},{"input":"金沙酒店住宿体验","output":"jinsha-jiudian-zhusu-tiyan","rules_applied":["Romanize","hyphenate segments"]},{"input":"圣淘沙景点推荐","output":"shengtaosha-jingdian-tuijian","rules_applied":["Romanize place name"]},{"input":"2025年新加坡活动","output":"2025-nian-xinjiapo-huodong","rules_applied":["Keep numbers","romanize text"]},{"input":"新加坡国庆节庆祝活动与美食嘉年华指南","output":"xinjiapo-guoqingjie-qingzhu-huodong-meishi-jianianhua-zhinan","rules_applied":["Romanize","remove 与 (stopword)","truncate if needed"]},{"input":"什么？最新优惠！","output":"shenme-zuixin-youhui","rules_applied":["Remove punctuation","romanize"]},{"input":"\"鱼尾狮\"的故事","output":"yuweiishi-gushi","rules_applied":["Remove quotes","remove 的 (stopword)"]},{"input":"組屋 vs 公寓","output":"zuwu-vs-gongyu","rules_applied":["Handle mixed script (Traditional 組 → Simplified)"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/zh-SG.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'zh-TH'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ZH (TH) Slugification',
    s.content = 'URL slug generation rules for zh-TH',
    s.slug_rule = 'romanized',
    s.stopwords = '{"preposition":["zai (在)"],"demonstrative":["zhe (这)","na (那)"],"classifier":["ge (个)"],"conjunction":["he (和)","yu (与)"],"verb":["shi (是)"]}',
    s.stopwords_count = 7,
    s.regional_additions = '[{"word":"tai (泰)","category":"geographic","reason":"Very common in Thai-Chinese content, rarely adds SEO value"},{"word":"mangu (曼谷)","category":"geographic","reason":"Bangkok reference, often redundant in Thailand context"},{"word":"zhongguo (中国)","category":"geographic","reason":"China reference, may not be relevant for local Thai content"}]',
    s.script_config = '{"primary_script":"thai","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short, may need manual review"},{"condition":"Slug > 60 chars","message":"Consider shortening for readability"},{"condition":"All stopwords removed","message":"Content may be too generic"},{"condition":"Romanization produced identical syllables","message":"Multiple characters may have romanized to same sound"}]',
    s.examples = '[{"input":"泰国旅游指南","output":"taiguo-lvyou-zhinan","rules_applied":["Romanization","lowercase"]},{"input":"曼谷美食推荐","output":"mangu-meishi-tuijian","rules_applied":["Romanization","lowercase"]},{"input":"泰国华人社区活动","output":"taiguo-huaren-shequ-huodong","rules_applied":["Romanization","lowercase"]},{"input":"中泰贸易合作","output":"zhongtai-maoyi-hezuo","rules_applied":["Romanization","lowercase"]},{"input":"清迈购物中心","output":"qingmai-gouwu-zhongxin","rules_applied":["Romanization","lowercase"]},{"input":"2025年泰国商业展览","output":"2025-nian-taiguo-shangye-zhanlan","rules_applied":["Numbers preserved","romanization"]},{"input":"泰国华人新年庆祝活动暨传统文化展示盛典将在曼谷举行","output":"taiguo-huaren-xinnian-qingzhu-huodong-ji-chuantong-wenhua-zhanshi","rules_applied":["Truncated at 80 chars"]},{"input":"芭提雅：海滩与夜生活","output":"batiya-haitan-yu-yeshenghuo","rules_applied":["Punctuation removed","romanization"]},{"input":"\"泰国之声\"广播节目","output":"taiguo-zhi-sheng-guangbo-jiemu","rules_applied":["Quotes removed","romanization"]},{"input":"泰铢兑换人民币","output":"taizhun-duihuan-renminbi","rules_applied":["Currency terms romanized"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/zh-TH.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'zh-TW'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'Chinese (Traditional) Slugification',
    s.content = 'URL slug generation rules for zh-TW',
    s.slug_rule = 'native_script',
    s.stopwords = '{"preposition":["在"],"adverb":["也"],"conjunction":["和"]}',
    s.stopwords_count = 3,
    s.regional_additions = '[{"word":"","category":"啦","reason":"particle"},{"word":"","category":"喔","reason":"particle"},{"word":"","category":"咧","reason":"particle"},{"word":"","category":"呢","reason":"particle"},{"word":"","category":"吧","reason":"particle"},{"word":"","category":"嗎","reason":"particle"}]',
    s.script_config = '{"primary_script":"chinese","diacritic_handling":null,"numeral_handling":null,"special_chars":null}',
    s.warnings = '[{"condition":"Slug < 2 chars","message":"Very short slug"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Simplified characters detected","message":"Use Traditional characters for zh-TW"},{"condition":"Mixed scripts (except numbers)","message":"Unusual for Chinese content"}]',
    s.examples = '[{"input":"台北旅遊指南","output":"台北旅遊指南","rules_applied":["NFC","Han preserved"]},{"input":"如何在台灣開設銀行帳戶","output":"如何台灣開設銀行帳戶","rules_applied":["Stopword 在 removed"]},{"input":"台中的美食推薦","output":"台中美食推薦","rules_applied":["Stopword 的 removed"]},{"input":"高雄港 與 愛河","output":"高雄港-愛河","rules_applied":["Stopword 與 removed","space to hyphen"]},{"input":"2024年台灣選舉新聞","output":"2024年台灣選舉新聞","rules_applied":["Numbers preserved","Han preserved"]},{"input":"全台灣最好的十大夜市排名！完整攻略與美食地圖，讓你一次吃遍北中南所有必吃小吃","output":"全台灣最好十大夜市排名完整攻略美食地圖讓一次吃遍北中南所有必吃小吃","rules_applied":["Long title truncated at 80 chars"]},{"input":"「珍珠奶茶」的歷史","output":"珍珠奶茶歷史","rules_applied":["Brackets removed","的 removed"]},{"input":"蘋果iPhone 15：台灣售價與規格","output":"蘋果iphone-15台灣售價規格","rules_applied":["Colon removed","Latin lowercase"]},{"input":"台灣高鐵（HSR）訂票教學","output":"台灣高鐵hsr訂票教學","rules_applied":["Parentheses removed","Latin lowercase"]},{"input":"阿里山日出 vs 太魯閣峽谷","output":"阿里山日出-vs-太魯閣峽谷","rules_applied":["Latin preserved lowercase"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/zh-TW.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

MERGE (s:Slugification {key: 'zu-ZA'})
SET s.node_class = 'Slugification',
    s.provenance = '{"source": "seed:locale", "version": "v0.19.0"}',
    s.display_name = 'ZU (ZA) Slugification',
    s.content = 'URL slug generation rules for zu-ZA',
    s.slug_rule = 'latin_strip',
    s.stopwords = '{"conjunction":["kanye"],"adverb":["kakhulu","njalo"]}',
    s.stopwords_count = 3,
    s.regional_additions = '[{"word":"","category":"kanye","reason":"conjunction"},{"word":"","category":"kakhulu","reason":"adverb"},{"word":"","category":"njalo","reason":"adverb"},{"word":"","category":"nje","reason":"particle"}]',
    s.script_config = 'null',
    s.warnings = '[{"condition":"Slug < 3 chars","message":"Too short"},{"condition":"Slug > 60 chars","message":"Consider shortening"},{"condition":"Too many stopwords removed (> 60%)","message":"May lose meaning"},{"condition":"Single-letter slug segments","message":"May indicate over-stripping"}]',
    s.examples = '[{"input":"Indlela Yokuthola Umsebenzi","output":"indlela-yokuthola-umsebenzi","rules_applied":["Lowercase","hyphenation"]},{"input":"Izindaba Zakamuva ZaseNingizimu Afrika","output":"izindaba-zakamuva-zaseningizimu-afrika","rules_applied":["Lowercase","spaces to hyphens"]},{"input":"Umlando Wesizwe SamaZulu","output":"umlando-wesizwe-samazulu","rules_applied":["Lowercase","hyphenation"]},{"input":"Ukudla Okumnandi Kanye Nezithelo","output":"ukudla-okumnandi-nezithelo","rules_applied":["Stopword removed (kanye)","lowercase"]},{"input":"Amathiphu Okonga Imali Futhi Uthuthuke","output":"amathiphu-okonga-imali-uthuthuke","rules_applied":["Stopword removed (futhi)","lowercase"]},{"input":"Izindlu Ezingu-10 Ezinhle KwaZulu-Natal","output":"izindlu-ezingu-10-ezinhle-kwazulu-natal","rules_applied":["Number preserved","lowercase","hyphenation"]},{"input":"Izinhlelo Zokuthuthukisa Umphakathi Wasemakhaya Kanye Nokwakha Amathuba Emisebenzi Entsha","output":"izinhlelo-zokuthuthukisa-umphakathi-wasemakhaya-nokwakha-amathuba-emisebenzi","rules_applied":["Truncated to 80 chars","stopword removed (kanye)"]},{"input":"Imidlalo: Ibhola Likanobhutshuzwayo & Rugby!","output":"imidlalo-ibhola-likanobhutshuzwayo-rugby","rules_applied":["Special chars removed (: & !)","lowercase"]},{"input":"Incwadi \"Shaka Zulu\" Nomlando Wakhe","output":"incwadi-shaka-zulu-nomlando-wakhe","rules_applied":["Quotes removed","lowercase"]},{"input":"Umculo vs Umdanso: Yini Engcono Kuwe","output":"umculo-umdanso-yini-engcono-kuwe","rules_applied":["Colon and period removed","lowercase"]}]',
    s.template_version = '2.0',
    s.source_file = '2-rules-slug/zu-ZA.md',
    s.updated_at = datetime(),
    s.created_by = 'seed:locale',
    s.created_at = coalesce(s.created_at, datetime());

// ----------------------------------------------------------------------------
// PART 3: Arcs Locale → Slugification
// ----------------------------------------------------------------------------

MATCH (l:Locale {key: 'af-ZA'})
MATCH (s:Slugification {key: 'af-ZA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-AE'})
MATCH (s:Slugification {key: 'ar-AE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-BH'})
MATCH (s:Slugification {key: 'ar-BH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-DZ'})
MATCH (s:Slugification {key: 'ar-DZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-EG'})
MATCH (s:Slugification {key: 'ar-EG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-IQ'})
MATCH (s:Slugification {key: 'ar-IQ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-JO'})
MATCH (s:Slugification {key: 'ar-JO'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-KW'})
MATCH (s:Slugification {key: 'ar-KW'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-LB'})
MATCH (s:Slugification {key: 'ar-LB'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-LY'})
MATCH (s:Slugification {key: 'ar-LY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-MA'})
MATCH (s:Slugification {key: 'ar-MA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-OM'})
MATCH (s:Slugification {key: 'ar-OM'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-QA'})
MATCH (s:Slugification {key: 'ar-QA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-SA'})
MATCH (s:Slugification {key: 'ar-SA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ar-TN'})
MATCH (s:Slugification {key: 'ar-TN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'as-IN'})
MATCH (s:Slugification {key: 'as-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'az-AZ'})
MATCH (s:Slugification {key: 'az-AZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'be-BY'})
MATCH (s:Slugification {key: 'be-BY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'bg-BG'})
MATCH (s:Slugification {key: 'bg-BG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'bn-BD'})
MATCH (s:Slugification {key: 'bn-BD'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'bn-IN'})
MATCH (s:Slugification {key: 'bn-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'bs-BA'})
MATCH (s:Slugification {key: 'bs-BA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ca-AD'})
MATCH (s:Slugification {key: 'ca-AD'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ca-ES'})
MATCH (s:Slugification {key: 'ca-ES'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ceb-PH'})
MATCH (s:Slugification {key: 'ceb-PH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'cs-CZ'})
MATCH (s:Slugification {key: 'cs-CZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'cy-GB'})
MATCH (s:Slugification {key: 'cy-GB'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'da-DK'})
MATCH (s:Slugification {key: 'da-DK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'de-AT'})
MATCH (s:Slugification {key: 'de-AT'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'de-CH'})
MATCH (s:Slugification {key: 'de-CH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'de-DE'})
MATCH (s:Slugification {key: 'de-DE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'de-LU'})
MATCH (s:Slugification {key: 'de-LU'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'el-CY'})
MATCH (s:Slugification {key: 'el-CY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'el-GR'})
MATCH (s:Slugification {key: 'el-GR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-AE'})
MATCH (s:Slugification {key: 'en-AE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-AU'})
MATCH (s:Slugification {key: 'en-AU'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-BB'})
MATCH (s:Slugification {key: 'en-BB'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-BW'})
MATCH (s:Slugification {key: 'en-BW'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-CA'})
MATCH (s:Slugification {key: 'en-CA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-CY'})
MATCH (s:Slugification {key: 'en-CY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-FJ'})
MATCH (s:Slugification {key: 'en-FJ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-GB'})
MATCH (s:Slugification {key: 'en-GB'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-GH'})
MATCH (s:Slugification {key: 'en-GH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-HK'})
MATCH (s:Slugification {key: 'en-HK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-IE'})
MATCH (s:Slugification {key: 'en-IE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-IN'})
MATCH (s:Slugification {key: 'en-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-JM'})
MATCH (s:Slugification {key: 'en-JM'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-KE'})
MATCH (s:Slugification {key: 'en-KE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-KY'})
MATCH (s:Slugification {key: 'en-KY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-MU'})
MATCH (s:Slugification {key: 'en-MU'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-MY'})
MATCH (s:Slugification {key: 'en-MY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-NG'})
MATCH (s:Slugification {key: 'en-NG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-NZ'})
MATCH (s:Slugification {key: 'en-NZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-PH'})
MATCH (s:Slugification {key: 'en-PH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-PK'})
MATCH (s:Slugification {key: 'en-PK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-SA'})
MATCH (s:Slugification {key: 'en-SA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-SG'})
MATCH (s:Slugification {key: 'en-SG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-TT'})
MATCH (s:Slugification {key: 'en-TT'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-TZ'})
MATCH (s:Slugification {key: 'en-TZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-UG'})
MATCH (s:Slugification {key: 'en-UG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-US'})
MATCH (s:Slugification {key: 'en-US'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-VN'})
MATCH (s:Slugification {key: 'en-VN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-ZA'})
MATCH (s:Slugification {key: 'en-ZA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-ZM'})
MATCH (s:Slugification {key: 'en-ZM'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'en-ZW'})
MATCH (s:Slugification {key: 'en-ZW'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-AR'})
MATCH (s:Slugification {key: 'es-AR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-BO'})
MATCH (s:Slugification {key: 'es-BO'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-CL'})
MATCH (s:Slugification {key: 'es-CL'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-CO'})
MATCH (s:Slugification {key: 'es-CO'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-CR'})
MATCH (s:Slugification {key: 'es-CR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-CU'})
MATCH (s:Slugification {key: 'es-CU'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-DO'})
MATCH (s:Slugification {key: 'es-DO'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-EC'})
MATCH (s:Slugification {key: 'es-EC'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-ES'})
MATCH (s:Slugification {key: 'es-ES'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-GT'})
MATCH (s:Slugification {key: 'es-GT'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-HN'})
MATCH (s:Slugification {key: 'es-HN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-MX'})
MATCH (s:Slugification {key: 'es-MX'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-NI'})
MATCH (s:Slugification {key: 'es-NI'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-PA'})
MATCH (s:Slugification {key: 'es-PA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-PE'})
MATCH (s:Slugification {key: 'es-PE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-PR'})
MATCH (s:Slugification {key: 'es-PR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-PY'})
MATCH (s:Slugification {key: 'es-PY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-SV'})
MATCH (s:Slugification {key: 'es-SV'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-UY'})
MATCH (s:Slugification {key: 'es-UY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'es-VE'})
MATCH (s:Slugification {key: 'es-VE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'et-EE'})
MATCH (s:Slugification {key: 'et-EE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'eu-ES'})
MATCH (s:Slugification {key: 'eu-ES'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fa-IR'})
MATCH (s:Slugification {key: 'fa-IR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fi-FI'})
MATCH (s:Slugification {key: 'fi-FI'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-BE'})
MATCH (s:Slugification {key: 'fr-BE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-BF'})
MATCH (s:Slugification {key: 'fr-BF'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-CA'})
MATCH (s:Slugification {key: 'fr-CA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-CD'})
MATCH (s:Slugification {key: 'fr-CD'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-CH'})
MATCH (s:Slugification {key: 'fr-CH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-CI'})
MATCH (s:Slugification {key: 'fr-CI'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-CM'})
MATCH (s:Slugification {key: 'fr-CM'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-DZ'})
MATCH (s:Slugification {key: 'fr-DZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-FR'})
MATCH (s:Slugification {key: 'fr-FR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-LU'})
MATCH (s:Slugification {key: 'fr-LU'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-MA'})
MATCH (s:Slugification {key: 'fr-MA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-MG'})
MATCH (s:Slugification {key: 'fr-MG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-RW'})
MATCH (s:Slugification {key: 'fr-RW'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-SN'})
MATCH (s:Slugification {key: 'fr-SN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'fr-TN'})
MATCH (s:Slugification {key: 'fr-TN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ga-IE'})
MATCH (s:Slugification {key: 'ga-IE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'gl-ES'})
MATCH (s:Slugification {key: 'gl-ES'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'gn-PY'})
MATCH (s:Slugification {key: 'gn-PY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'gu-IN'})
MATCH (s:Slugification {key: 'gu-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ha-NG'})
MATCH (s:Slugification {key: 'ha-NG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'he-IL'})
MATCH (s:Slugification {key: 'he-IL'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'hi-IN'})
MATCH (s:Slugification {key: 'hi-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'hr-HR'})
MATCH (s:Slugification {key: 'hr-HR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ht-HT'})
MATCH (s:Slugification {key: 'ht-HT'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'hu-HU'})
MATCH (s:Slugification {key: 'hu-HU'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'hy-AM'})
MATCH (s:Slugification {key: 'hy-AM'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'id-ID'})
MATCH (s:Slugification {key: 'id-ID'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ig-NG'})
MATCH (s:Slugification {key: 'ig-NG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'is-IS'})
MATCH (s:Slugification {key: 'is-IS'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'it-CH'})
MATCH (s:Slugification {key: 'it-CH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'it-IT'})
MATCH (s:Slugification {key: 'it-IT'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ja-JP'})
MATCH (s:Slugification {key: 'ja-JP'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'jv-ID'})
MATCH (s:Slugification {key: 'jv-ID'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ka-GE'})
MATCH (s:Slugification {key: 'ka-GE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'kk-KZ'})
MATCH (s:Slugification {key: 'kk-KZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'km-KH'})
MATCH (s:Slugification {key: 'km-KH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'kn-IN'})
MATCH (s:Slugification {key: 'kn-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ko-KR'})
MATCH (s:Slugification {key: 'ko-KR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ku-TR'})
MATCH (s:Slugification {key: 'ku-TR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ky-KG'})
MATCH (s:Slugification {key: 'ky-KG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ln-CD'})
MATCH (s:Slugification {key: 'ln-CD'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'lt-LT'})
MATCH (s:Slugification {key: 'lt-LT'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'lv-LV'})
MATCH (s:Slugification {key: 'lv-LV'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'mg-MG'})
MATCH (s:Slugification {key: 'mg-MG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'mi-NZ'})
MATCH (s:Slugification {key: 'mi-NZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'mk-MK'})
MATCH (s:Slugification {key: 'mk-MK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ml-IN'})
MATCH (s:Slugification {key: 'ml-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'mn-MN'})
MATCH (s:Slugification {key: 'mn-MN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'mr-IN'})
MATCH (s:Slugification {key: 'mr-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ms-BN'})
MATCH (s:Slugification {key: 'ms-BN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ms-MY'})
MATCH (s:Slugification {key: 'ms-MY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ms-SG'})
MATCH (s:Slugification {key: 'ms-SG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'mt-MT'})
MATCH (s:Slugification {key: 'mt-MT'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'my-MM'})
MATCH (s:Slugification {key: 'my-MM'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ne-NP'})
MATCH (s:Slugification {key: 'ne-NP'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'nl-BE'})
MATCH (s:Slugification {key: 'nl-BE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'nl-NL'})
MATCH (s:Slugification {key: 'nl-NL'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'no-NO'})
MATCH (s:Slugification {key: 'no-NO'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ny-MW'})
MATCH (s:Slugification {key: 'ny-MW'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'or-IN'})
MATCH (s:Slugification {key: 'or-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'pa-IN'})
MATCH (s:Slugification {key: 'pa-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'pa-PK'})
MATCH (s:Slugification {key: 'pa-PK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'pl-PL'})
MATCH (s:Slugification {key: 'pl-PL'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ps-AF'})
MATCH (s:Slugification {key: 'ps-AF'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'pt-AO'})
MATCH (s:Slugification {key: 'pt-AO'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'pt-BR'})
MATCH (s:Slugification {key: 'pt-BR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'pt-CH'})
MATCH (s:Slugification {key: 'pt-CH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'pt-MZ'})
MATCH (s:Slugification {key: 'pt-MZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'pt-PT'})
MATCH (s:Slugification {key: 'pt-PT'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'qu-PE'})
MATCH (s:Slugification {key: 'qu-PE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ro-MD'})
MATCH (s:Slugification {key: 'ro-MD'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ro-RO'})
MATCH (s:Slugification {key: 'ro-RO'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ru-BY'})
MATCH (s:Slugification {key: 'ru-BY'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ru-IL'})
MATCH (s:Slugification {key: 'ru-IL'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ru-KG'})
MATCH (s:Slugification {key: 'ru-KG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ru-KZ'})
MATCH (s:Slugification {key: 'ru-KZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ru-MD'})
MATCH (s:Slugification {key: 'ru-MD'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ru-RU'})
MATCH (s:Slugification {key: 'ru-RU'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'rw-RW'})
MATCH (s:Slugification {key: 'rw-RW'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'sd-PK'})
MATCH (s:Slugification {key: 'sd-PK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'si-LK'})
MATCH (s:Slugification {key: 'si-LK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'sk-SK'})
MATCH (s:Slugification {key: 'sk-SK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'sl-SI'})
MATCH (s:Slugification {key: 'sl-SI'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'sn-ZW'})
MATCH (s:Slugification {key: 'sn-ZW'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'so-SO'})
MATCH (s:Slugification {key: 'so-SO'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'sq-AL'})
MATCH (s:Slugification {key: 'sq-AL'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'sr-RS'})
MATCH (s:Slugification {key: 'sr-RS'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'su-ID'})
MATCH (s:Slugification {key: 'su-ID'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'sv-SE'})
MATCH (s:Slugification {key: 'sv-SE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'sw-KE'})
MATCH (s:Slugification {key: 'sw-KE'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'sw-TZ'})
MATCH (s:Slugification {key: 'sw-TZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ta-IN'})
MATCH (s:Slugification {key: 'ta-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ta-LK'})
MATCH (s:Slugification {key: 'ta-LK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'te-IN'})
MATCH (s:Slugification {key: 'te-IN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'tg-TJ'})
MATCH (s:Slugification {key: 'tg-TJ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'th-TH'})
MATCH (s:Slugification {key: 'th-TH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'tk-TM'})
MATCH (s:Slugification {key: 'tk-TM'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'tl-PH'})
MATCH (s:Slugification {key: 'tl-PH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'tr-TR'})
MATCH (s:Slugification {key: 'tr-TR'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'uk-UA'})
MATCH (s:Slugification {key: 'uk-UA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'ur-PK'})
MATCH (s:Slugification {key: 'ur-PK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'uz-UZ'})
MATCH (s:Slugification {key: 'uz-UZ'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'vi-VN'})
MATCH (s:Slugification {key: 'vi-VN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'wo-SN'})
MATCH (s:Slugification {key: 'wo-SN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'xh-ZA'})
MATCH (s:Slugification {key: 'xh-ZA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'yo-NG'})
MATCH (s:Slugification {key: 'yo-NG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'zh-CN'})
MATCH (s:Slugification {key: 'zh-CN'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'zh-HK'})
MATCH (s:Slugification {key: 'zh-HK'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'zh-SG'})
MATCH (s:Slugification {key: 'zh-SG'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'zh-TH'})
MATCH (s:Slugification {key: 'zh-TH'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'zh-TW'})
MATCH (s:Slugification {key: 'zh-TW'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

MATCH (l:Locale {key: 'zu-ZA'})
MATCH (s:Slugification {key: 'zu-ZA'})
MERGE (l)-[:HAS_SLUGIFICATION]->(s);

