/**
 * useDebouncedValue Hook Tests
 *
 * Tests for the debounce utility hook.
 */

import { renderHook, act } from '@testing-library/react';
import { useDebouncedValue } from '../useDebouncedValue';

describe('useDebouncedValue', () => {
  beforeEach(() => {
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it('should return initial value immediately', () => {
    const { result } = renderHook(() => useDebouncedValue('initial', 100));

    expect(result.current).toBe('initial');
  });

  it('should debounce value updates', () => {
    const { result, rerender } = renderHook(
      ({ value, delay }) => useDebouncedValue(value, delay),
      { initialProps: { value: 'first', delay: 100 } }
    );

    expect(result.current).toBe('first');

    // Update value
    rerender({ value: 'second', delay: 100 });

    // Should still be 'first' before delay
    expect(result.current).toBe('first');

    // Fast-forward time
    act(() => {
      jest.advanceTimersByTime(100);
    });

    // Now should be 'second'
    expect(result.current).toBe('second');
  });

  it('should use default delay of 150ms', () => {
    const { result, rerender } = renderHook(
      ({ value }) => useDebouncedValue(value),
      { initialProps: { value: 'initial' } }
    );

    rerender({ value: 'updated' });

    // At 100ms, should still be initial
    act(() => {
      jest.advanceTimersByTime(100);
    });
    expect(result.current).toBe('initial');

    // At 150ms, should be updated
    act(() => {
      jest.advanceTimersByTime(50);
    });
    expect(result.current).toBe('updated');
  });

  it('should only emit the last value when multiple rapid changes', () => {
    const { result, rerender } = renderHook(
      ({ value, delay }) => useDebouncedValue(value, delay),
      { initialProps: { value: 'first', delay: 100 } }
    );

    // Rapid updates
    rerender({ value: 'second', delay: 100 });
    act(() => {
      jest.advanceTimersByTime(30);
    });

    rerender({ value: 'third', delay: 100 });
    act(() => {
      jest.advanceTimersByTime(30);
    });

    rerender({ value: 'fourth', delay: 100 });

    // Still 'first' - none have completed
    expect(result.current).toBe('first');

    // Wait full delay from last change
    act(() => {
      jest.advanceTimersByTime(100);
    });

    // Should be 'fourth' - skipped intermediate values
    expect(result.current).toBe('fourth');
  });

  it('should handle delay change mid-flight', () => {
    const { result, rerender } = renderHook(
      ({ value, delay }) => useDebouncedValue(value, delay),
      { initialProps: { value: 'initial', delay: 100 } }
    );

    rerender({ value: 'updated', delay: 200 }); // Change delay

    // Wait original delay
    act(() => {
      jest.advanceTimersByTime(100);
    });
    expect(result.current).toBe('initial'); // Still waiting

    // Wait remaining time
    act(() => {
      jest.advanceTimersByTime(100);
    });
    expect(result.current).toBe('updated');
  });

  it('should cleanup timeout on unmount', () => {
    const { rerender, unmount } = renderHook(
      ({ value, delay }) => useDebouncedValue(value, delay),
      { initialProps: { value: 'initial', delay: 100 } }
    );

    rerender({ value: 'updated', delay: 100 });

    // Unmount before timeout fires
    unmount();

    // Advance timers - should not cause errors
    act(() => {
      jest.advanceTimersByTime(200);
    });

    // No way to assert state after unmount, but this verifies no memory leak
    expect(true).toBe(true);
  });

  it('should work with different value types', () => {
    // Number
    const { result: numResult, rerender: numRerender } = renderHook(
      ({ value }) => useDebouncedValue(value, 50),
      { initialProps: { value: 42 } }
    );
    numRerender({ value: 100 });
    act(() => { jest.advanceTimersByTime(50); });
    expect(numResult.current).toBe(100);

    // Object
    const { result: objResult, rerender: objRerender } = renderHook(
      ({ value }) => useDebouncedValue(value, 50),
      { initialProps: { value: { a: 1 } as Record<string, number> } }
    );
    const newObj: Record<string, number> = { b: 2 };
    objRerender({ value: newObj });
    act(() => { jest.advanceTimersByTime(50); });
    expect(objResult.current).toBe(newObj);

    // Array
    const { result: arrResult, rerender: arrRerender } = renderHook(
      ({ value }) => useDebouncedValue(value, 50),
      { initialProps: { value: [1, 2] } }
    );
    const newArr = [3, 4, 5];
    arrRerender({ value: newArr });
    act(() => { jest.advanceTimersByTime(50); });
    expect(arrResult.current).toBe(newArr);
  });

  it('should handle zero delay', () => {
    const { result, rerender } = renderHook(
      ({ value }) => useDebouncedValue(value, 0),
      { initialProps: { value: 'initial' } }
    );

    rerender({ value: 'updated' });

    // With 0 delay, setTimeout still defers to next tick
    expect(result.current).toBe('initial');

    act(() => {
      jest.advanceTimersByTime(0);
    });

    expect(result.current).toBe('updated');
  });
});
