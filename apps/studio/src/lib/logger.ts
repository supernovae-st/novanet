/**
 * Centralized Logger Utility
 *
 * Features:
 * - Log levels: debug, info, warn, error
 * - Silent in production (except errors)
 * - Colored output in development
 * - Context prefixes for easy filtering
 * - Debug mode toggleable via localStorage
 *
 * Usage:
 *   import { logger } from '@/lib/logger';
 *   logger.info('Neo4j', 'Connection established');
 *   logger.error('API', 'Request failed', error);
 *   logger.debug('Graph', 'Rendering nodes', { count: 100 });
 */

type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface LoggerConfig {
  /** Minimum log level to display */
  minLevel: LogLevel;
  /** Enable colored output */
  colors: boolean;
  /** Enable timestamps */
  timestamps: boolean;
}

const LOG_LEVELS: Record<LogLevel, number> = {
  debug: 0,
  info: 1,
  warn: 2,
  error: 3,
};

const LOG_COLORS: Record<LogLevel, string> = {
  debug: '\x1b[36m', // Cyan
  info: '\x1b[32m',  // Green
  warn: '\x1b[33m',  // Yellow
  error: '\x1b[31m', // Red
};

const RESET = '\x1b[0m';

/**
 * Check if we're in a browser environment
 */
const isBrowser = typeof window !== 'undefined';

/**
 * Check if debug mode is enabled via localStorage
 */
function isDebugEnabled(): boolean {
  if (!isBrowser) return process.env.NODE_ENV === 'development';
  try {
    return localStorage.getItem('novanet:debug') === 'true';
  } catch {
    return false;
  }
}

/**
 * Get default config based on environment
 */
function getDefaultConfig(): LoggerConfig {
  const isDev = process.env.NODE_ENV === 'development';
  const debugEnabled = isDebugEnabled();

  return {
    minLevel: isDev || debugEnabled ? 'debug' : 'warn',
    colors: !isBrowser, // Colors only in Node.js (terminal)
    timestamps: !isBrowser,
  };
}

/**
 * Format log message with optional color and timestamp
 */
function formatMessage(
  level: LogLevel,
  context: string,
  message: string,
  config: LoggerConfig
): string {
  const parts: string[] = [];

  // Timestamp (server-side only)
  if (config.timestamps) {
    const time = new Date().toISOString().slice(11, 23);
    parts.push(`[${time}]`);
  }

  // Level with color
  const levelStr = level.toUpperCase().padEnd(5);
  if (config.colors) {
    parts.push(`${LOG_COLORS[level]}${levelStr}${RESET}`);
  } else {
    parts.push(levelStr);
  }

  // Context
  if (context) {
    if (config.colors) {
      parts.push(`\x1b[90m[${context}]${RESET}`);
    } else {
      parts.push(`[${context}]`);
    }
  }

  // Message
  parts.push(message);

  return parts.join(' ');
}

/**
 * Create a logger instance
 */
function createLogger() {
  const config = getDefaultConfig();

  const log = (level: LogLevel, context: string, message: string, ...args: unknown[]) => {
    // Check if this level should be logged
    if (LOG_LEVELS[level] < LOG_LEVELS[config.minLevel]) {
      return;
    }

    const formattedMessage = formatMessage(level, context, message, config);

    // Use appropriate console method
    switch (level) {
      case 'debug':
        console.debug(formattedMessage, ...args);
        break;
      case 'info':
        console.info(formattedMessage, ...args);
        break;
      case 'warn':
        console.warn(formattedMessage, ...args);
        break;
      case 'error':
        console.error(formattedMessage, ...args);
        break;
    }
  };

  return {
    /**
     * Debug level - verbose development info
     * Hidden in production unless debug mode enabled
     */
    debug: (context: string, message: string, ...args: unknown[]) =>
      log('debug', context, message, ...args),

    /**
     * Info level - general operational info
     * Hidden in production
     */
    info: (context: string, message: string, ...args: unknown[]) =>
      log('info', context, message, ...args),

    /**
     * Warn level - potential issues
     * Always shown
     */
    warn: (context: string, message: string, ...args: unknown[]) =>
      log('warn', context, message, ...args),

    /**
     * Error level - errors and failures
     * Always shown
     */
    error: (context: string, message: string, ...args: unknown[]) =>
      log('error', context, message, ...args),

    /**
     * Enable debug mode (persisted in localStorage)
     */
    enableDebug: () => {
      if (isBrowser) {
        localStorage.setItem('novanet:debug', 'true');
        console.info('[Logger] Debug mode enabled. Refresh to see debug logs.');
      }
    },

    /**
     * Disable debug mode
     */
    disableDebug: () => {
      if (isBrowser) {
        localStorage.removeItem('novanet:debug');
        console.info('[Logger] Debug mode disabled.');
      }
    },
  };
}

/**
 * Singleton logger instance
 */
export const logger = createLogger();

/**
 * Re-export for convenience
 */
export default logger;
