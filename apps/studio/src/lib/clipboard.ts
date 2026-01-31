/**
 * Cross-platform clipboard utility
 * Adapted from Nika Studio pattern
 */

import { logger } from '@/lib/logger';

export interface ClipboardProvider {
  writeText(text: string): Promise<void>;
  readText(): Promise<string>;
}

/**
 * Web clipboard implementation using navigator.clipboard API
 */
class WebClipboardProvider implements ClipboardProvider {
  async writeText(text: string): Promise<void> {
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      // Fallback for older browsers
      this.fallbackCopy(text);
    }
  }

  async readText(): Promise<string> {
    try {
      return await navigator.clipboard.readText();
    } catch {
      logger.warn('Clipboard', 'Read not available');
      return '';
    }
  }

  private fallbackCopy(text: string): void {
    const textarea = document.createElement('textarea');
    textarea.value = text;
    textarea.style.position = 'fixed';
    textarea.style.left = '-9999px';
    textarea.style.top = '-9999px';
    document.body.appendChild(textarea);
    textarea.focus();
    textarea.select();
    try {
      document.execCommand('copy');
    } catch (err) {
      logger.error('Clipboard', 'Fallback copy failed', err);
    }
    document.body.removeChild(textarea);
  }
}

/**
 * Create clipboard provider based on environment
 */
function createClipboardProvider(): ClipboardProvider {
  // Could add Tauri/Electron support here
  return new WebClipboardProvider();
}

export const clipboard = createClipboardProvider();

/**
 * Copy text to clipboard with optional formatting
 */
export async function copyToClipboard(text: string): Promise<boolean> {
  try {
    await clipboard.writeText(text);
    return true;
  } catch {
    return false;
  }
}

/**
 * Copy node properties to clipboard
 */
export async function copyNodeProperties(
  properties: Record<string, unknown>,
  format: 'json' | 'typescript' | 'yaml' = 'json'
): Promise<boolean> {
  let text: string;

  switch (format) {
    case 'typescript':
      text = formatAsTypeScript(properties);
      break;
    case 'yaml':
      text = formatAsYaml(properties);
      break;
    case 'json':
    default:
      text = JSON.stringify(properties, null, 2);
  }

  return copyToClipboard(text);
}

/**
 * Format properties as TypeScript interface
 */
function formatAsTypeScript(properties: Record<string, unknown>): string {
  const lines = ['interface NodeProperties {'];

  for (const [key, value] of Object.entries(properties)) {
    const type = getTypeScriptType(value);
    lines.push(`  ${key}: ${type};`);
  }

  lines.push('}');
  return lines.join('\n');
}

/**
 * Format properties as YAML
 *
 * Supports:
 * - Nested objects
 * - Arrays (both primitive and object items)
 * - Multi-line strings (block scalar)
 * - Special characters (quoted)
 */
function formatAsYaml(properties: Record<string, unknown>, indent = 0): string {
  const lines: string[] = [];
  const prefix = '  '.repeat(indent);

  for (const [key, value] of Object.entries(properties)) {
    if (value === null || value === undefined) {
      lines.push(`${prefix}${key}: null`);
    } else if (typeof value === 'object' && !Array.isArray(value)) {
      lines.push(`${prefix}${key}:`);
      lines.push(formatAsYaml(value as Record<string, unknown>, indent + 1));
    } else if (Array.isArray(value)) {
      if (value.length === 0) {
        lines.push(`${prefix}${key}: []`);
      } else {
        lines.push(`${prefix}${key}:`);
        value.forEach((item) => {
          if (item !== null && typeof item === 'object') {
            lines.push(`${prefix}  -`);
            lines.push(formatAsYaml(item as Record<string, unknown>, indent + 2));
          } else {
            lines.push(`${prefix}  - ${formatYamlValue(item, indent + 1)}`);
          }
        });
      }
    } else {
      const formatted = formatYamlValue(value, indent);
      lines.push(`${prefix}${key}: ${formatted}`);
    }
  }

  return lines.join('\n');
}

/**
 * Format a value for YAML output
 *
 * Handles:
 * - Multi-line strings with block scalar (|)
 * - Strings with special characters (quoted)
 * - Numbers, booleans, null
 */
function formatYamlValue(value: unknown, indent = 0): string {
  if (typeof value === 'string') {
    // Multi-line strings use block scalar
    if (value.includes('\n')) {
      const prefix = '  '.repeat(indent + 1);
      const lines = value.split('\n');
      return `|\n${lines.map((line) => `${prefix}${line}`).join('\n')}`;
    }
    // Quote strings with special characters
    if (
      value.includes(':') ||
      value.includes('#') ||
      value.includes("'") ||
      value.includes('"') ||
      value.startsWith(' ') ||
      value.endsWith(' ') ||
      value === '' ||
      // YAML reserved words
      /^(true|false|null|yes|no|on|off)$/i.test(value)
    ) {
      // Use single quotes, escape single quotes by doubling
      return `'${value.replace(/'/g, "''")}'`;
    }
    return value;
  }
  if (typeof value === 'boolean') {
    return value ? 'true' : 'false';
  }
  if (value === null || value === undefined) {
    return 'null';
  }
  return String(value);
}

function getTypeScriptType(value: unknown): string {
  if (value === null || value === undefined) return 'null';
  if (Array.isArray(value)) {
    if (value.length === 0) return 'unknown[]';
    const itemType = getTypeScriptType(value[0]);
    return `${itemType}[]`;
  }
  if (typeof value === 'object') return 'Record<string, unknown>';
  if (typeof value === 'number') return 'number';
  if (typeof value === 'boolean') return 'boolean';
  return 'string';
}
