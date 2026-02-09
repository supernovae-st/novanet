// SEOQuestion nodes for fr-FR locale
// Generated from ATP export: docs/assets/keywods/fr-fr_qr/seo/paas_export_qr code.csv
// 4 parent questions + 22 child questions = 26 total

// -----------------------------------------------------------------------------
// SEOKeyword for the root term
// -----------------------------------------------------------------------------

MERGE (k:SEOKeyword {key: 'qr-code-fr'})
SET k.value = 'qr code',
    k.locale = 'fr-FR';

// -----------------------------------------------------------------------------
// Parent Question 1: Comment scanner un QR code sans application ?
// -----------------------------------------------------------------------------

MERGE (q1:SEOQuestion {
  key: 'comment-scanner-un-qr-code-sans-application',
  value: 'Comment scanner un QR code sans application ?',
  question_word: 'comment',
  is_parent: true,
  locale: 'fr-FR'
});

// Child questions for Parent 1
MERGE (q2:SEOQuestion {
  key: 'comment-scanner-un-code-qr-sans-application',
  value: 'Comment scanner un code QR sans application ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q3:SEOQuestion {
  key: 'comment-scanner-un-code-qr-avec-mon-appareil-photo',
  value: 'Comment scanner un code QR avec mon appareil photo ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q4:SEOQuestion {
  key: 'pourquoi-je-narrive-pas-a-scanner-un-qr-code-avec-mon-telephone',
  value: "Pourquoi je n'arrive pas à scanner un QR Code avec mon téléphone ?",
  question_word: 'pourquoi',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q5:SEOQuestion {
  key: 'comment-scanner-un-qr-code-sans-telecharger-dapplication',
  value: "Comment scanner un QR Code sans télécharger d'application ?",
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

// -----------------------------------------------------------------------------
// Parent Question 2: Comment scanner le QR code avec mon portable ?
// -----------------------------------------------------------------------------

MERGE (q6:SEOQuestion {
  key: 'comment-scanner-le-qr-code-avec-mon-portable',
  value: 'Comment scanner le QR code avec mon portable ?',
  question_word: 'comment',
  is_parent: true,
  locale: 'fr-FR'
});

// Child questions for Parent 2
MERGE (q7:SEOQuestion {
  key: 'comment-puis-je-scanner-un-code-qr-avec-mon-telephone-portable',
  value: 'Comment puis-je scanner un code QR avec mon téléphone portable ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q8:SEOQuestion {
  key: 'pourquoi-je-narrive-pas-a-scanner-un-qr-code-avec-mon-tel',
  value: "Pourquoi je n'arrive pas à scanner un QR code avec mon téléphone ?",
  question_word: 'pourquoi',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q9:SEOQuestion {
  key: 'ou-se-trouve-le-lecteur-de-qr-code-sur-mon-telephone',
  value: 'Où se trouve le lecteur de QR code sur mon téléphone ?',
  question_word: 'ou',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q10:SEOQuestion {
  key: 'comment-scanner-un-qr-avec-le-telephone',
  value: 'Comment scanner un QR avec le téléphone ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q11:SEOQuestion {
  key: 'comment-savoir-si-mon-telephone-peut-scanner-un-qr-code',
  value: 'Comment savoir si mon téléphone peut scanner un QR code ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q12:SEOQuestion {
  key: 'ou-se-trouve-le-scanner-sur-android',
  value: 'Où se trouve le scanner sur Android ?',
  question_word: 'ou',
  is_parent: false,
  locale: 'fr-FR'
});

// -----------------------------------------------------------------------------
// Parent Question 3: Comment faire un QR code gratuitement ?
// -----------------------------------------------------------------------------

MERGE (q13:SEOQuestion {
  key: 'comment-faire-un-qr-code-gratuitement',
  value: 'Comment faire un QR code gratuitement ?',
  question_word: 'comment',
  is_parent: true,
  locale: 'fr-FR'
});

// Child questions for Parent 3
MERGE (q14:SEOQuestion {
  key: 'quel-est-le-meilleur-qr-code-gratuit',
  value: 'Quel est le meilleur QR code gratuit ?',
  question_word: 'quel',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q15:SEOQuestion {
  key: 'comment-creer-mon-propre-code-qr',
  value: 'Comment créer mon propre code QR ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q16:SEOQuestion {
  key: 'est-ce-quun-qr-code-est-gratuit',
  value: "Est-ce qu'un QR code est gratuit ?",
  question_word: 'est-ce',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q17:SEOQuestion {
  key: 'comment-puis-je-obtenir-un-code-qr',
  value: 'Comment puis-je obtenir un code QR ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q18:SEOQuestion {
  key: 'ou-puis-je-creer-un-qr-code-gratuitement-et-en-illimite',
  value: 'Où puis-je créer un QR code gratuitement et en illimité ?',
  question_word: 'ou',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q19:SEOQuestion {
  key: 'existe-t-il-des-codes-qr-veritablement-gratuits',
  value: 'Existe-t-il des codes QR véritablement gratuits ?',
  question_word: 'existe',
  is_parent: false,
  locale: 'fr-FR'
});

// -----------------------------------------------------------------------------
// Parent Question 4: Quelle application pour lire un QR code ?
// -----------------------------------------------------------------------------

MERGE (q20:SEOQuestion {
  key: 'quelle-application-pour-lire-un-qr-code',
  value: 'Quelle application pour lire un QR code ?',
  question_word: 'quelle',
  is_parent: true,
  locale: 'fr-FR'
});

// Child questions for Parent 4
MERGE (q21:SEOQuestion {
  key: 'quel-est-le-meilleur-lecteur-de-qr-code-gratuit',
  value: 'Quel est le meilleur lecteur de QR code gratuit ?',
  question_word: 'quel',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q22:SEOQuestion {
  key: 'comment-lire-un-qr-code-avec-son-telephone-portable',
  value: 'Comment lire un QR code avec son téléphone portable ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q23:SEOQuestion {
  key: 'quelle-appli-gratuite-pour-scanner-un-qr-code',
  value: 'Quelle appli gratuite pour scanner un QR code ?',
  question_word: 'quelle',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q24:SEOQuestion {
  key: 'comment-lire-un-code-qr-avec-mon-telephone',
  value: 'Comment lire un code QR avec mon téléphone ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q25:SEOQuestion {
  key: 'comment-installer-un-lecteur-qr-gratuit-sur-mon-telephone',
  value: 'Comment installer un lecteur QR gratuit sur mon téléphone ?',
  question_word: 'comment',
  is_parent: false,
  locale: 'fr-FR'
});

MERGE (q26:SEOQuestion {
  key: 'existe-t-il-des-applications-qr-gratuites',
  value: 'Existe-t-il des applications QR gratuites ?',
  question_word: 'existe',
  is_parent: false,
  locale: 'fr-FR'
});

// -----------------------------------------------------------------------------
// Connect SEOQuestions to SEOKeyword
// -----------------------------------------------------------------------------

MATCH (k:SEOKeyword {key: 'qr-code-fr'})
MATCH (q:SEOQuestion) WHERE q.locale = 'fr-FR'
MERGE (k)-[:HAS_QUESTIONS]->(q);

// -----------------------------------------------------------------------------
// Connect parent questions to their children
// -----------------------------------------------------------------------------

// Parent 1 -> Children 1-4
MATCH (p:SEOQuestion {key: 'comment-scanner-un-qr-code-sans-application'})
MATCH (c:SEOQuestion) WHERE c.key IN [
  'comment-scanner-un-code-qr-sans-application',
  'comment-scanner-un-code-qr-avec-mon-appareil-photo',
  'pourquoi-je-narrive-pas-a-scanner-un-qr-code-avec-mon-telephone',
  'comment-scanner-un-qr-code-sans-telecharger-dapplication'
]
MERGE (p)-[:HAS_CHILD_QUESTION]->(c);

// Parent 2 -> Children 5-10
MATCH (p:SEOQuestion {key: 'comment-scanner-le-qr-code-avec-mon-portable'})
MATCH (c:SEOQuestion) WHERE c.key IN [
  'comment-puis-je-scanner-un-code-qr-avec-mon-telephone-portable',
  'pourquoi-je-narrive-pas-a-scanner-un-qr-code-avec-mon-tel',
  'ou-se-trouve-le-lecteur-de-qr-code-sur-mon-telephone',
  'comment-scanner-un-qr-avec-le-telephone',
  'comment-savoir-si-mon-telephone-peut-scanner-un-qr-code',
  'ou-se-trouve-le-scanner-sur-android'
]
MERGE (p)-[:HAS_CHILD_QUESTION]->(c);

// Parent 3 -> Children 11-16
MATCH (p:SEOQuestion {key: 'comment-faire-un-qr-code-gratuitement'})
MATCH (c:SEOQuestion) WHERE c.key IN [
  'quel-est-le-meilleur-qr-code-gratuit',
  'comment-creer-mon-propre-code-qr',
  'est-ce-quun-qr-code-est-gratuit',
  'comment-puis-je-obtenir-un-code-qr',
  'ou-puis-je-creer-un-qr-code-gratuitement-et-en-illimite',
  'existe-t-il-des-codes-qr-veritablement-gratuits'
]
MERGE (p)-[:HAS_CHILD_QUESTION]->(c);

// Parent 4 -> Children 17-22
MATCH (p:SEOQuestion {key: 'quelle-application-pour-lire-un-qr-code'})
MATCH (c:SEOQuestion) WHERE c.key IN [
  'quel-est-le-meilleur-lecteur-de-qr-code-gratuit',
  'comment-lire-un-qr-code-avec-son-telephone-portable',
  'quelle-appli-gratuite-pour-scanner-un-qr-code',
  'comment-lire-un-code-qr-avec-mon-telephone',
  'comment-installer-un-lecteur-qr-gratuit-sur-mon-telephone',
  'existe-t-il-des-applications-qr-gratuites'
]
MERGE (p)-[:HAS_CHILD_QUESTION]->(c);

// -----------------------------------------------------------------------------
// Connect SEOKeyword to Locale
// -----------------------------------------------------------------------------

MATCH (l:Locale {key: 'fr-FR'})
MATCH (k:SEOKeyword {key: 'qr-code-fr'})
MERGE (l)-[:HAS_SEO_KEYWORDS]->(k);
