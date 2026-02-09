/**
 * Formatters - Unified formatting utilities
 *
 * Centralized formatting functions for dates, times, numbers, and sizes.
 * Use these instead of inline formatting logic.
 */

/**
 * Format a date as a time string
 * @example formatTime(new Date()) // "2:30:45 PM"
 */
export function formatTime(date: Date): string {
  return date.toLocaleTimeString('en-US', {
    hour: 'numeric',
    minute: '2-digit',
    second: '2-digit',
    hour12: true,
  });
}
