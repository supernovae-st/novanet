// Migration 055: Fix qr-code@fr-FR denomination_forms
// ADR-033 compliance: text, title, abbrev, url forms
// CRITICAL: In French we say "QR code" NOT "code QR"!

// First, check current state
MATCH (en:EntityNative {key: "entity:qr-code@fr-FR"})
SET en.denomination_forms = '[{"type": "text", "value": "QR code", "priority": 1}, {"type": "title", "value": "QR Code", "priority": 1}, {"type": "abbrev", "value": "QR", "priority": 1}, {"type": "url", "value": "qr-code", "priority": 1}, {"type": "plural", "value": "QR codes", "priority": 5}]',
    en.updated_at = datetime()
RETURN en.key AS fixed_key, en.denomination_forms AS new_forms;
