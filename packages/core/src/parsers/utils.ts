// novanet-core/src/parsers/utils.ts
// Shared utilities for markdown parsing

/**
 * Generic table row type for parsed markdown tables
 */
export interface TableRow {
  [key: string]: string;
}

/**
 * Field-value table row (for simpler two-column tables)
 */
export interface FieldValueRow {
  field: string;
  value: string;
}

/**
 * Parse a markdown table from a section of content.
 * Handles markdown tables with format:
 * | Header1 | Header2 |
 * |---------|---------|
 * | Value1  | Value2  |
 *
 * @param section - The markdown section containing a table
 * @returns Array of row objects with headers as keys
 */
export function parseMarkdownTable(section: string): TableRow[] {
  const lines = section.split('\n').filter(l => l.trim().startsWith('|'));
  if (lines.length < 3) return []; // Need header, separator, and at least one data row

  // First line is headers
  const headerLine = lines[0];
  const headers = headerLine
    .split('|')
    .map(h => h.trim().toLowerCase().replace(/\s+/g, '_'))
    .filter(h => h.length > 0);

  // Skip separator line (line with ---)
  // Data lines are everything after the separator
  const dataLines = lines.slice(2).filter(l => !l.includes('---'));

  const rows: TableRow[] = [];

  for (const line of dataLines) {
    const cells = line.split('|').slice(1); // Remove first empty element from split
    const row: TableRow = {};
    headers.forEach((h, idx) => {
      row[h] = (cells[idx] || '').trim();
    });
    // Only add row if it has actual content (not empty row)
    if (Object.values(row).some(v => v.length > 0)) {
      rows.push(row);
    }
  }
  return rows;
}

/**
 * Parse a markdown table with Field | Value format.
 * Handles tables like:
 * | Field | Value |
 * |-------|-------|
 * | script_code | Latn |
 *
 * @param section - The markdown section containing a field-value table
 * @returns Array of field-value pairs
 */
export function parseFieldValueTable(section: string): FieldValueRow[] {
  const lines = section.split('\n').filter(l => l.trim().startsWith('|'));
  const rows: FieldValueRow[] = [];

  for (const line of lines) {
    // Skip header and separator lines
    if (line.includes('---') || line.toLowerCase().includes('| field |')) {
      continue;
    }

    const cells = line.split('|').map(c => c.trim()).filter(Boolean);
    if (cells.length >= 2) {
      rows.push({
        field: cells[0].toLowerCase().replace(/\s+/g, '_'),
        value: cells[1],
      });
    }
  }
  return rows;
}

/**
 * Find a value in parsed field-value table rows by field name.
 *
 * @param rows - Array of field-value rows
 * @param field - Field name to find
 * @returns The value for that field, or empty string
 */
export function findValue(rows: FieldValueRow[], field: string): string {
  const row = rows.find(r => r.field === field.toLowerCase());
  return row?.value || '';
}

/**
 * Extract a specific section from markdown by header pattern.
 * Section ends at the next ### header or at end of content.
 *
 * @param content - Full markdown content
 * @param pattern - RegExp to match section header
 * @returns The matched section content, or empty string
 */
export function extractSection(content: string, pattern: RegExp): string {
  const match = content.match(pattern);
  return match ? match[0] : '';
}

/**
 * Extract a numeric score from content using a regex pattern.
 *
 * @param content - Content to search
 * @param pattern - RegExp with capture group for the number
 * @returns Parsed integer, or 0 if not found
 */
export function extractScore(content: string, pattern: RegExp): number {
  const match = content.match(pattern);
  return match ? parseInt(match[1], 10) : 0;
}

/**
 * Extract a string value from content using a regex pattern.
 *
 * @param content - Content to search
 * @param pattern - RegExp with capture group for the string
 * @returns Trimmed string value, or empty string if not found
 */
export function extractString(content: string, pattern: RegExp): string {
  const match = content.match(pattern);
  return match ? match[1].trim() : '';
}

/**
 * Extract a numeric value from a pattern match in content.
 * Handles various formats: "68.2 million", "93%", "159.9 billion EUR"
 *
 * @param content - Content to search
 * @param pattern - RegExp with capture group for the number
 * @returns Parsed float, or 0 if not found
 */
export function extractNumber(content: string, pattern: RegExp): number {
  const match = content.match(pattern);
  if (!match) return 0;
  // Remove commas and spaces, parse as float
  return parseFloat(match[1].replace(/[,\s]/g, ''));
}

/**
 * Parse a single markdown table row into cells.
 *
 * @param line - A table row line starting with |
 * @returns Array of trimmed cell values
 */
export function parseTableRow(line: string): string[] {
  return line
    .split('|')
    .map(cell => cell.trim())
    .filter(Boolean);
}

/**
 * Check if a line is a table separator (e.g., |---|---|---|)
 */
export function isTableSeparator(line: string): boolean {
  return line.includes('---');
}

/**
 * Check if a line is a table header row.
 */
export function isTableHeader(line: string): boolean {
  const lowerLine = line.toLowerCase();
  return (
    lowerLine.includes('| expression |') ||
    lowerLine.includes('| field |') ||
    lowerLine.includes('| header') ||
    (lowerLine.startsWith('|') && lowerLine.includes('register'))
  );
}
