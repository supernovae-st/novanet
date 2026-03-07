// Migration 055: Fix qr-code@fr-FR denomination_forms
// ADR-033 compliance: text, title, abbrev, url forms
// Issue: French EntityNative has corrupted/empty values

// First, check current state
MATCH (en:EntityNative {key: "entity:qr-code@fr-FR"})
SET en.denomination_forms = '[{"type": "text", "value": "code QR", "priority": 1}, {"type": "title", "value": "Code QR", "priority": 1}, {"type": "abbrev", "value": "QR", "priority": 1}, {"type": "url", "value": "code-qr", "priority": 1}, {"type": "plural", "value": "codes QR", "priority": 5}]',
    en.updated_at = datetime()
RETURN en.key AS fixed_key, en.denomination_forms AS new_forms;
