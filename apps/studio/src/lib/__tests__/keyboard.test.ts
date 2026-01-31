/**
 * Keyboard utilities tests
 */

import { parseKeyCombo, matchesKeyCombo, isInputFocused } from '../keyboard';

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
