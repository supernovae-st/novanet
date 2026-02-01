/**
 * Monaco Editor Configuration - Matrix Terminal Theme
 *
 * Custom theme and Cypher language definition for the CypherEditorModal.
 * Designed to match the Matrix Terminal aesthetic with emerald/cyan/amber colors.
 */

import type { editor, languages } from 'monaco-editor';

// =============================================================================
// Matrix Terminal Theme
// =============================================================================

export const MATRIX_THEME_NAME = 'matrix-terminal';

export const matrixTheme: editor.IStandaloneThemeData = {
  base: 'vs-dark',
  inherit: true,
  rules: [
    // Base text - emerald glow
    { token: '', foreground: '34d399' },

    // Keywords (MATCH, RETURN, WHERE, etc.) - cyan
    { token: 'keyword', foreground: '22d3ee', fontStyle: 'bold' },
    { token: 'keyword.cypher', foreground: '22d3ee', fontStyle: 'bold' },

    // Labels (:Project, :Locale) - bright emerald
    { token: 'type', foreground: '34d399' },
    { token: 'type.identifier', foreground: '34d399' },
    { token: 'label.cypher', foreground: '34d399' },

    // Variables ($param) - amber
    { token: 'variable', foreground: 'fbbf24' },
    { token: 'variable.cypher', foreground: 'fbbf24' },
    { token: 'parameter.cypher', foreground: 'fbbf24' },

    // Strings - orange
    { token: 'string', foreground: 'fb923c' },
    { token: 'string.cypher', foreground: 'fb923c' },

    // Numbers - purple
    { token: 'number', foreground: 'c084fc' },
    { token: 'number.cypher', foreground: 'c084fc' },

    // Comments - muted gray
    { token: 'comment', foreground: '4b5563', fontStyle: 'italic' },
    { token: 'comment.cypher', foreground: '4b5563', fontStyle: 'italic' },

    // Operators - white
    { token: 'operator', foreground: 'f9fafb' },
    { token: 'delimiter', foreground: '6b7280' },

    // Properties - light emerald
    { token: 'property', foreground: '6ee7b7' },
    { token: 'property.cypher', foreground: '6ee7b7' },

    // Functions - cyan
    { token: 'function', foreground: '22d3ee' },
  ],
  colors: {
    // Editor background - near black with slight green tint
    'editor.background': '#0a0a0f',
    'editor.foreground': '#34d399',

    // Selection - emerald with transparency
    'editor.selectionBackground': '#34d39930',
    'editor.selectionHighlightBackground': '#34d39920',
    'editor.inactiveSelectionBackground': '#34d39915',

    // Line highlight
    'editor.lineHighlightBackground': '#34d39908',
    'editor.lineHighlightBorder': '#34d39920',

    // Cursor - bright emerald
    'editorCursor.foreground': '#34d399',
    'editorCursor.background': '#0a0a0f',

    // Line numbers
    'editorLineNumber.foreground': '#34d39940',
    'editorLineNumber.activeForeground': '#34d399',

    // Gutter
    'editorGutter.background': '#0a0a0f',

    // Scrollbar
    'scrollbarSlider.background': '#34d39920',
    'scrollbarSlider.hoverBackground': '#34d39940',
    'scrollbarSlider.activeBackground': '#34d39960',

    // Widget (autocomplete, etc.)
    'editorWidget.background': '#0d0d12',
    'editorWidget.border': '#34d39930',
    'editorSuggestWidget.background': '#0d0d12',
    'editorSuggestWidget.border': '#34d39930',
    'editorSuggestWidget.selectedBackground': '#34d39920',

    // Indent guides
    'editorIndentGuide.background': '#34d39915',
    'editorIndentGuide.activeBackground': '#34d39930',

    // Bracket matching
    'editorBracketMatch.background': '#34d39930',
    'editorBracketMatch.border': '#34d39960',

    // Whitespace
    'editorWhitespace.foreground': '#34d39920',
  },
};

// =============================================================================
// Cypher Language Definition
// =============================================================================

export const CYPHER_LANGUAGE_ID = 'cypher';

export const cypherLanguageConfig: languages.LanguageConfiguration = {
  comments: {
    lineComment: '//',
    blockComment: ['/*', '*/'] as [string, string],
  },
  brackets: [
    ['{', '}'],
    ['[', ']'],
    ['(', ')'],
  ],
  autoClosingPairs: [
    { open: '{', close: '}' },
    { open: '[', close: ']' },
    { open: '(', close: ')' },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
    { open: '`', close: '`' },
  ],
  surroundingPairs: [
    { open: '{', close: '}' },
    { open: '[', close: ']' },
    { open: '(', close: ')' },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
    { open: '`', close: '`' },
  ],
};

export const cypherTokensProvider: languages.IMonarchLanguage = {
  defaultToken: '',
  tokenPostfix: '.cypher',

  keywords: [
    'MATCH', 'OPTIONAL', 'RETURN', 'WITH', 'WHERE', 'ORDER', 'BY', 'SKIP', 'LIMIT',
    'CREATE', 'MERGE', 'DELETE', 'DETACH', 'REMOVE', 'SET', 'ON',
    'CALL', 'YIELD', 'UNWIND', 'FOREACH', 'LOAD', 'CSV', 'FROM',
    'AS', 'DISTINCT', 'UNION', 'ALL',
    'AND', 'OR', 'XOR', 'NOT', 'IN', 'STARTS', 'ENDS', 'CONTAINS',
    'IS', 'NULL', 'TRUE', 'FALSE',
    'ASC', 'DESC', 'ASCENDING', 'DESCENDING',
    'CASE', 'WHEN', 'THEN', 'ELSE', 'END',
    'EXISTS', 'COUNT', 'COLLECT', 'SUM', 'AVG', 'MIN', 'MAX',
  ],

  operators: [
    '=', '>', '<', '!', '~', '?', ':',
    '==', '<=', '>=', '!=', '<>',
    '+', '-', '*', '/', '%', '^',
    '+=', '-=', '*=', '/=',
    '.', '..', '|', '&',
  ],

  symbols: /[=><!~?:&|+\-*/^%]+/,

  escapes: /\\(?:[bfnrt\\"']|u[0-9A-Fa-f]{4})/,

  tokenizer: {
    root: [
      // Whitespace
      { include: '@whitespace' },

      // Labels (:Label)
      [/:([A-Z][a-zA-Z0-9_]*)/, 'label.cypher'],

      // Parameters ($param)
      [/\$[a-zA-Z_][a-zA-Z0-9_]*/, 'parameter.cypher'],

      // Properties (.property)
      [/\.([a-zA-Z_][a-zA-Z0-9_]*)/, 'property.cypher'],

      // Keywords
      [/[a-zA-Z_]\w*/, {
        cases: {
          '@keywords': 'keyword.cypher',
          '@default': 'identifier',
        },
      }],

      // Numbers
      [/\d*\.\d+([eE][-+]?\d+)?/, 'number.float.cypher'],
      [/\d+/, 'number.cypher'],

      // Strings
      [/"([^"\\]|\\.)*$/, 'string.invalid'],
      [/'([^'\\]|\\.)*$/, 'string.invalid'],
      [/"/, 'string', '@string_double'],
      [/'/, 'string', '@string_single'],

      // Delimiters
      [/[{}()[\]]/, '@brackets'],
      [/[<>](?!@symbols)/, '@brackets'],
      [/@symbols/, {
        cases: {
          '@operators': 'operator',
          '@default': '',
        },
      }],

      // Delimiter
      [/[;,.]/, 'delimiter'],
    ],

    whitespace: [
      [/[ \t\r\n]+/, 'white'],
      [/\/\*/, 'comment', '@comment'],
      [/\/\/.*$/, 'comment'],
    ],

    comment: [
      [/[^/*]+/, 'comment'],
      [/\/\*/, 'comment', '@push'],
      [/\*\//, 'comment', '@pop'],
      [/[/*]/, 'comment'],
    ],

    string_double: [
      [/[^\\"]+/, 'string'],
      [/@escapes/, 'string.escape'],
      [/\\./, 'string.escape.invalid'],
      [/"/, 'string', '@pop'],
    ],

    string_single: [
      [/[^\\']+/, 'string'],
      [/@escapes/, 'string.escape'],
      [/\\./, 'string.escape.invalid'],
      [/'/, 'string', '@pop'],
    ],
  },
};

// =============================================================================
// Editor Options
// =============================================================================

export const matrixEditorOptions: editor.IStandaloneEditorConstructionOptions = {
  fontSize: 14,
  fontFamily: "'JetBrains Mono', 'Fira Code', 'SF Mono', Consolas, monospace",
  fontLigatures: true,
  lineNumbers: 'on',
  renderLineHighlight: 'gutter',
  minimap: { enabled: false },
  scrollbar: {
    vertical: 'auto',
    horizontal: 'auto',
    verticalScrollbarSize: 8,
    horizontalScrollbarSize: 8,
  },
  glyphMargin: false,
  folding: false,
  lineDecorationsWidth: 8,
  lineNumbersMinChars: 3,
  cursorBlinking: 'phase',
  cursorStyle: 'block',
  cursorWidth: 2,
  smoothScrolling: true,
  contextmenu: false,
  quickSuggestions: false,
  suggestOnTriggerCharacters: false,
  wordWrap: 'on',
  wrappingIndent: 'indent',
  padding: { top: 16, bottom: 16 },
  overviewRulerBorder: false,
  overviewRulerLanes: 0,
  hideCursorInOverviewRuler: true,
  scrollBeyondLastLine: false,
  automaticLayout: true,
};
