/**
 * Keyboard utilities tests
 */

import { handleViewPresetShortcut, parseKeyCombo, matchesKeyCombo, isInputFocused } from '../keyboard';

describe('handleViewPresetShortcut', () => {
  it('maps 1 to project-structure preset', () => {
    const result = handleViewPresetShortcut('1');
    expect(result).toBe('project-structure');
  });

  it('maps 2 to generation-chain preset', () => {
    const result = handleViewPresetShortcut('2');
    expect(result).toBe('generation-chain');
  });

  it('maps 3 to locale-knowledge preset', () => {
    const result = handleViewPresetShortcut('3');
    expect(result).toBe('locale-knowledge');
  });

  it('maps 4 to concept-network preset', () => {
    const result = handleViewPresetShortcut('4');
    expect(result).toBe('concept-network');
  });

  it('maps 5 to prompts-rules preset', () => {
    const result = handleViewPresetShortcut('5');
    expect(result).toBe('prompts-rules');
  });

  it('maps 6 to seo-geo preset', () => {
    const result = handleViewPresetShortcut('6');
    expect(result).toBe('seo-geo');
  });

  it('maps 7 to high-priority preset', () => {
    const result = handleViewPresetShortcut('7');
    expect(result).toBe('high-priority');
  });

  it('maps 8 to realtime preset', () => {
    const result = handleViewPresetShortcut('8');
    expect(result).toBe('realtime');
  });

  it('maps 0 to all-nodes preset', () => {
    const result = handleViewPresetShortcut('0');
    expect(result).toBe('all-nodes');
  });

  it('returns null for non-shortcut keys', () => {
    expect(handleViewPresetShortcut('a')).toBeNull();
    expect(handleViewPresetShortcut('9')).toBeNull();
    expect(handleViewPresetShortcut('')).toBeNull();
    expect(handleViewPresetShortcut('Enter')).toBeNull();
  });

  it('returns null for special characters', () => {
    expect(handleViewPresetShortcut('!')).toBeNull();
    expect(handleViewPresetShortcut('@')).toBeNull();
    expect(handleViewPresetShortcut(' ')).toBeNull();
  });
});

describe('parseKeyCombo', () => {
  it('parses simple key', () => {
    const result = parseKeyCombo('k');
    expect(result).toEqual({
      mod: false,
      shift: false,
      alt: false,
      key: 'k',
    });
  });

  it('parses mod+key combo', () => {
    const result = parseKeyCombo('mod+k');
    expect(result).toEqual({
      mod: true,
      shift: false,
      alt: false,
      key: 'k',
    });
  });

  it('parses mod+shift+key combo', () => {
    const result = parseKeyCombo('mod+shift+k');
    expect(result).toEqual({
      mod: true,
      shift: true,
      alt: false,
      key: 'k',
    });
  });

  it('parses alt+key combo', () => {
    const result = parseKeyCombo('alt+j');
    expect(result).toEqual({
      mod: false,
      shift: false,
      alt: true,
      key: 'j',
    });
  });

  it('handles uppercase input', () => {
    const result = parseKeyCombo('MOD+SHIFT+K');
    expect(result).toEqual({
      mod: true,
      shift: true,
      alt: false,
      key: 'k',
    });
  });
});

describe('matchesKeyCombo', () => {
  const createKeyboardEvent = (overrides: Partial<KeyboardEvent> = {}): KeyboardEvent => {
    return {
      key: 'k',
      metaKey: false,
      ctrlKey: false,
      shiftKey: false,
      altKey: false,
      ...overrides,
    } as KeyboardEvent;
  };

  beforeAll(() => {
    // Mock navigator.platform for Mac
    Object.defineProperty(navigator, 'platform', {
      value: 'MacIntel',
      configurable: true,
    });
  });

  it('matches simple key press', () => {
    const event = createKeyboardEvent({ key: 'k' });
    expect(matchesKeyCombo(event, 'k')).toBe(true);
  });

  it('matches mod+key combo on Mac (metaKey)', () => {
    const event = createKeyboardEvent({ key: 'k', metaKey: true });
    expect(matchesKeyCombo(event, 'mod+k')).toBe(true);
  });

  it('does not match when mod is missing', () => {
    const event = createKeyboardEvent({ key: 'k' });
    expect(matchesKeyCombo(event, 'mod+k')).toBe(false);
  });

  it('matches shift+key combo', () => {
    const event = createKeyboardEvent({ key: 'K', shiftKey: true });
    expect(matchesKeyCombo(event, 'shift+k')).toBe(true);
  });

  it('matches special key: escape', () => {
    const event = createKeyboardEvent({ key: 'Escape' });
    expect(matchesKeyCombo(event, 'escape')).toBe(true);
  });

  it('matches special key: space', () => {
    const event = createKeyboardEvent({ key: ' ' });
    expect(matchesKeyCombo(event, 'space')).toBe(true);
  });

  it('matches special key: enter', () => {
    const event = createKeyboardEvent({ key: 'Enter' });
    expect(matchesKeyCombo(event, 'enter')).toBe(true);
  });

  it('does not match different key', () => {
    const event = createKeyboardEvent({ key: 'j' });
    expect(matchesKeyCombo(event, 'k')).toBe(false);
  });
});

describe('isInputFocused', () => {
  it('returns false when no element is focused', () => {
    // In JSDOM, document.activeElement is document.body by default
    expect(isInputFocused()).toBe(false);
  });

  it('returns true when input is focused', () => {
    const input = document.createElement('input');
    document.body.appendChild(input);
    input.focus();
    expect(isInputFocused()).toBe(true);
    document.body.removeChild(input);
  });

  it('returns true when textarea is focused', () => {
    const textarea = document.createElement('textarea');
    document.body.appendChild(textarea);
    textarea.focus();
    expect(isInputFocused()).toBe(true);
    document.body.removeChild(textarea);
  });

  it('returns true when contenteditable is focused', () => {
    const div = document.createElement('div');
    div.setAttribute('contenteditable', 'true');
    document.body.appendChild(div);
    div.focus();
    expect(isInputFocused()).toBe(true);
    document.body.removeChild(div);
  });
});
