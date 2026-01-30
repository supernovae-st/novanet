/**
 * useDebouncedBatch Hook Tests
 *
 * Tests for the debounced batch utility hook.
 * This hook batches multiple rapid updates into a single callback.
 */

import { renderHook, act } from '@testing-library/react';
import { useDebouncedBatch } from '../useDebouncedBatch';

describe('useDebouncedBatch', () => {
  beforeEach(() => {
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it('should batch multiple rapid adds into a single callback', () => {
    const callback = jest.fn();
    const { result } = renderHook(() => useDebouncedBatch<number>(callback, 50));

    // Add multiple items rapidly
    act(() => {
      result.current.add(1);
      result.current.add(2);
      result.current.add(3);
    });

    // Callback should not have been called yet
    expect(callback).not.toHaveBeenCalled();

    // Advance timers past the delay
    act(() => {
      jest.advanceTimersByTime(50);
    });

    // Should have been called once with all items
    expect(callback).toHaveBeenCalledTimes(1);
    expect(callback).toHaveBeenCalledWith([1, 2, 3]);
  });

  it('should use default delay of 16ms (one frame at 60fps)', () => {
    const callback = jest.fn();
    const { result } = renderHook(() => useDebouncedBatch<string>(callback));

    act(() => {
      result.current.add('item');
    });

    // At 10ms, should not have fired
    act(() => {
      jest.advanceTimersByTime(10);
    });
    expect(callback).not.toHaveBeenCalled();

    // At 16ms, should have fired
    act(() => {
      jest.advanceTimersByTime(6);
    });
    expect(callback).toHaveBeenCalledWith(['item']);
  });

  it('should reset timer when new items are added', () => {
    const callback = jest.fn();
    const { result } = renderHook(() => useDebouncedBatch<number>(callback, 50));

    act(() => {
      result.current.add(1);
    });

    // Advance 30ms
    act(() => {
      jest.advanceTimersByTime(30);
    });
    expect(callback).not.toHaveBeenCalled();

    // Add another item (resets timer)
    act(() => {
      result.current.add(2);
    });

    // Advance another 30ms (60ms total, but only 30ms since last add)
    act(() => {
      jest.advanceTimersByTime(30);
    });
    expect(callback).not.toHaveBeenCalled();

    // Advance final 20ms (now 50ms since last add)
    act(() => {
      jest.advanceTimersByTime(20);
    });
    expect(callback).toHaveBeenCalledTimes(1);
    expect(callback).toHaveBeenCalledWith([1, 2]);
  });

  it('should flush immediately when flush() is called', () => {
    const callback = jest.fn();
    const { result } = renderHook(() => useDebouncedBatch<number>(callback, 100));

    act(() => {
      result.current.add(1);
      result.current.add(2);
    });

    // Flush immediately
    act(() => {
      result.current.flush();
    });

    expect(callback).toHaveBeenCalledTimes(1);
    expect(callback).toHaveBeenCalledWith([1, 2]);

    // Advancing timers should not call again
    act(() => {
      jest.advanceTimersByTime(100);
    });
    expect(callback).toHaveBeenCalledTimes(1);
  });

  it('should not call callback on flush if batch is empty', () => {
    const callback = jest.fn();
    const { result } = renderHook(() => useDebouncedBatch<number>(callback, 50));

    act(() => {
      result.current.flush();
    });

    expect(callback).not.toHaveBeenCalled();
  });

  it('should clear batch after callback is called', () => {
    const callback = jest.fn();
    const { result } = renderHook(() => useDebouncedBatch<number>(callback, 50));

    // First batch
    act(() => {
      result.current.add(1);
      result.current.add(2);
    });

    act(() => {
      jest.advanceTimersByTime(50);
    });

    expect(callback).toHaveBeenCalledWith([1, 2]);

    // Second batch
    act(() => {
      result.current.add(3);
      result.current.add(4);
    });

    act(() => {
      jest.advanceTimersByTime(50);
    });

    expect(callback).toHaveBeenCalledTimes(2);
    expect(callback).toHaveBeenLastCalledWith([3, 4]);
  });

  it('should handle different value types', () => {
    // Objects
    const objCallback = jest.fn();
    const { result: objResult } = renderHook(() =>
      useDebouncedBatch<{ id: number }>(objCallback, 50)
    );

    const obj1 = { id: 1 };
    const obj2 = { id: 2 };

    act(() => {
      objResult.current.add(obj1);
      objResult.current.add(obj2);
    });

    act(() => {
      jest.advanceTimersByTime(50);
    });

    expect(objCallback).toHaveBeenCalledWith([obj1, obj2]);
  });

  it('should use latest callback when callback changes', () => {
    const callback1 = jest.fn();
    const callback2 = jest.fn();

    const { result, rerender } = renderHook(
      ({ callback }) => useDebouncedBatch<number>(callback, 50),
      { initialProps: { callback: callback1 } }
    );

    act(() => {
      result.current.add(1);
    });

    // Change callback before timer fires
    rerender({ callback: callback2 });

    act(() => {
      jest.advanceTimersByTime(50);
    });

    // Should call the new callback, not the old one
    expect(callback1).not.toHaveBeenCalled();
    expect(callback2).toHaveBeenCalledWith([1]);
  });

  it('should cleanup timeout on unmount', () => {
    const callback = jest.fn();
    const { result, unmount } = renderHook(() =>
      useDebouncedBatch<number>(callback, 50)
    );

    act(() => {
      result.current.add(1);
    });

    // Unmount before timeout fires
    unmount();

    // Advance timers - should not cause errors and callback should not be called
    act(() => {
      jest.advanceTimersByTime(100);
    });

    expect(callback).not.toHaveBeenCalled();
  });

  it('should handle high volume of rapid additions', () => {
    const callback = jest.fn();
    const { result } = renderHook(() => useDebouncedBatch<number>(callback, 50));

    // Add 1000 items rapidly
    act(() => {
      for (let i = 0; i < 1000; i++) {
        result.current.add(i);
      }
    });

    act(() => {
      jest.advanceTimersByTime(50);
    });

    expect(callback).toHaveBeenCalledTimes(1);
    expect(callback.mock.calls[0][0]).toHaveLength(1000);
    expect(callback.mock.calls[0][0][0]).toBe(0);
    expect(callback.mock.calls[0][0][999]).toBe(999);
  });

  it('should return stable add and flush functions', () => {
    const callback = jest.fn();
    const { result, rerender } = renderHook(() =>
      useDebouncedBatch<number>(callback, 50)
    );

    const initialAdd = result.current.add;
    const initialFlush = result.current.flush;

    rerender();

    expect(result.current.add).toBe(initialAdd);
    expect(result.current.flush).toBe(initialFlush);
  });

  it('should handle delay change', () => {
    const callback = jest.fn();
    const { result, rerender } = renderHook(
      ({ delay }) => useDebouncedBatch<number>(callback, delay),
      { initialProps: { delay: 100 } }
    );

    act(() => {
      result.current.add(1);
    });

    // Change delay (this creates a new add function)
    rerender({ delay: 50 });

    // Add with new delay
    act(() => {
      result.current.add(2);
    });

    // Old timeout should still fire for first item if we wait 100ms
    // But new add resets the timer with new delay
    act(() => {
      jest.advanceTimersByTime(50);
    });

    expect(callback).toHaveBeenCalledWith([1, 2]);
  });
});
