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

/**
 * Format a date as a short date string
 * @example formatDate(new Date()) // "Jan 15, 2024"
 */
export function formatDate(date: Date): string {
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
}

/**
 * Format a relative time (e.g., "2 minutes ago")
 * @example formatRelativeTime(new Date(Date.now() - 60000)) // "1 minute ago"
 */
export function formatRelativeTime(date: Date): string {
  const now = Date.now();
  const diff = now - date.getTime();

  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);

  if (seconds < 60) return 'just now';
  if (minutes === 1) return '1 minute ago';
  if (minutes < 60) return `${minutes} minutes ago`;
  if (hours === 1) return '1 hour ago';
  if (hours < 24) return `${hours} hours ago`;
  if (days === 1) return 'yesterday';
  if (days < 7) return `${days} days ago`;

  return formatDate(date);
}
