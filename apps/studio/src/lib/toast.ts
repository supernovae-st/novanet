/**
 * Toast Notification System
 *
 * Wrapper around Sonner for NovaNet Visualizer.
 * Provides consistent styling and convenience methods.
 *
 * @example
 * import { toast } from '@/lib/toast';
 *
 * // Basic usage
 * toast.success('Copied to clipboard');
 * toast.error('Query failed', 'Invalid Cypher syntax');
 * toast.info('Tip: Use ⌘K for command palette');
 *
 * // With node counts
 * toast.nodeExpansion(5, 'Project');
 * toast.queryResult(150);
 */

import { toast as sonnerToast, type ExternalToast } from 'sonner';

// ============================================================================
// Toast Types
// ============================================================================

export type ToastType = 'success' | 'error' | 'info' | 'warning' | 'loading';

export interface ToastOptions extends ExternalToast {
  /** Optional description below the title */
  description?: string;
}

// ============================================================================
// Core Toast Functions
// ============================================================================

/**
 * Show a success toast
 */
function success(message: string, description?: string, options?: ToastOptions) {
  return sonnerToast.success(message, {
    description,
    ...options,
  });
}

/**
 * Show an error toast
 */
function error(message: string, description?: string, options?: ToastOptions) {
  return sonnerToast.error(message, {
    description,
    duration: 5000, // Longer duration for errors
    ...options,
  });
}

/**
 * Show an info toast
 */
function info(message: string, description?: string, options?: ToastOptions) {
  return sonnerToast.info(message, {
    description,
    ...options,
  });
}

/**
 * Show a warning toast
 */
function warning(message: string, description?: string, options?: ToastOptions) {
  return sonnerToast.warning(message, {
    description,
    duration: 4000,
    ...options,
  });
}

/**
 * Show a loading toast (returns dismiss function)
 */
function loading(message: string, description?: string, options?: ToastOptions) {
  return sonnerToast.loading(message, {
    description,
    ...options,
  });
}

/**
 * Dismiss a specific toast or all toasts
 */
function dismiss(toastId?: string | number) {
  sonnerToast.dismiss(toastId);
}

// ============================================================================
// NovaNet-Specific Toast Helpers
// ============================================================================

/**
 * Toast for clipboard copy success
 */
function copied(what?: string) {
  const message = what ? `Copied ${what}` : 'Copied to clipboard';
  return success(message);
}

/**
 * Toast for node expansion results
 */
function nodeExpansion(count: number, nodeType?: string) {
  const label = nodeType || 'nodes';
  if (count === 0) {
    return info(`No connected ${label} found`);
  }
  return success(`Added ${count} ${count === 1 ? 'node' : 'nodes'}`, nodeType ? `Expanded ${nodeType}` : undefined);
}

/**
 * Toast for query execution results
 */
function queryResult(nodeCount: number, edgeCount?: number) {
  if (nodeCount === 0 && (!edgeCount || edgeCount === 0)) {
    return info('Query returned no results');
  }

  const parts: string[] = [];
  if (nodeCount > 0) {
    parts.push(`${nodeCount} ${nodeCount === 1 ? 'node' : 'nodes'}`);
  }
  if (edgeCount && edgeCount > 0) {
    parts.push(`${edgeCount} ${edgeCount === 1 ? 'edge' : 'edges'}`);
  }

  return success(`Found ${parts.join(', ')}`);
}

/**
 * Toast for query execution start (loading state)
 */
function queryExecuting() {
  return loading('Executing query...');
}

/**
 * Toast for query error
 */
function queryError(errorMessage?: string) {
  return error(
    'Query failed',
    errorMessage || 'Check the Cypher syntax and try again'
  );
}

/**
 * Toast with promise handling
 * Automatically shows loading, then success/error based on promise result
 */
function promise<T>(
  promiseFn: Promise<T>,
  messages: {
    loading: string;
    success: string | ((data: T) => string);
    error: string | ((err: Error) => string);
  }
) {
  return sonnerToast.promise(promiseFn, messages);
}

// ============================================================================
// Export Toast API
// ============================================================================

export const toast = {
  // Core methods
  success,
  error,
  info,
  warning,
  loading,
  dismiss,
  promise,

  // NovaNet-specific helpers
  copied,
  nodeExpansion,
  queryResult,
  queryExecuting,
  queryError,

  // Direct access to sonner for advanced usage
  raw: sonnerToast,
};

export default toast;
