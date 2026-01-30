'use client';

/**
 * SchemaErrorBoundary - Error boundary for schema mode layout failures
 *
 * Features:
 * - Catches errors during schema graph rendering
 * - Shows error message with stack trace preview
 * - Retry button to recover from errors
 * - Glassmorphism styling matching NovaNet design
 *
 * Usage:
 * ```tsx
 * <SchemaErrorBoundary>
 *   <SchemaGraphContent />
 * </SchemaErrorBoundary>
 * ```
 */

import { Component, type ReactNode, type ErrorInfo } from 'react';
import { AlertTriangle, RefreshCw } from 'lucide-react';
import { cn } from '@/lib/utils';

/**
 * Props for SchemaErrorBoundary
 */
interface SchemaErrorBoundaryProps {
  /** Child components to render */
  children: ReactNode;
  /** Optional custom fallback UI */
  fallback?: ReactNode;
  /** Optional callback when error occurs */
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
}

/**
 * State for SchemaErrorBoundary
 */
interface SchemaErrorBoundaryState {
  /** Whether an error has been caught */
  hasError: boolean;
  /** The caught error object */
  error: Error | null;
  /** Error info with component stack */
  errorInfo: ErrorInfo | null;
}

/**
 * SchemaErrorBoundary - React error boundary for schema layout
 *
 * Catches errors during ELK layout or React Flow rendering and provides
 * a user-friendly error display with retry functionality.
 */
export class SchemaErrorBoundary extends Component<
  SchemaErrorBoundaryProps,
  SchemaErrorBoundaryState
> {
  constructor(props: SchemaErrorBoundaryProps) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null,
    };
  }

  /**
   * Update state when error is caught
   */
  static getDerivedStateFromError(error: Error): Partial<SchemaErrorBoundaryState> {
    return {
      hasError: true,
      error,
    };
  }

  /**
   * Log error details and notify callback
   */
  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    // Log to console for debugging
    console.error('[SchemaErrorBoundary] Layout error caught:', error);
    console.error('[SchemaErrorBoundary] Component stack:', errorInfo.componentStack);

    // Update state with error info
    this.setState({ errorInfo });

    // Notify callback if provided
    this.props.onError?.(error, errorInfo);
  }

  /**
   * Reset error state to retry rendering
   */
  handleRetry = (): void => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
    });
  };

  /**
   * Get a preview of the stack trace (first 5 lines)
   */
  getStackPreview(): string {
    const { error, errorInfo } = this.state;

    if (errorInfo?.componentStack) {
      // Get first 5 lines of component stack
      const lines = errorInfo.componentStack.trim().split('\n').slice(0, 5);
      return lines.join('\n');
    }

    if (error?.stack) {
      // Get first 5 lines of error stack
      const lines = error.stack.split('\n').slice(0, 5);
      return lines.join('\n');
    }

    return 'No stack trace available';
  }

  render(): ReactNode {
    const { hasError, error } = this.state;
    const { children, fallback } = this.props;

    // Render children if no error
    if (!hasError) {
      return children;
    }

    // Use custom fallback if provided
    if (fallback) {
      return fallback;
    }

    // Default error UI with glassmorphism styling
    return (
      <div
        className={cn(
          'flex flex-col items-center justify-center',
          'h-full w-full p-8',
          'text-center'
        )}
        data-testid="schema-error-boundary"
        role="alert"
        aria-live="assertive"
      >
        {/* Error card with glassmorphism */}
        <div
          className={cn(
            'max-w-lg w-full p-6 rounded-xl',
            'bg-black/70 backdrop-blur-xl',
            'border border-white/[0.08]',
            'shadow-2xl'
          )}
        >
          {/* Error icon and title */}
          <div className="flex items-center justify-center gap-3 mb-4">
            <div
              className={cn(
                'p-2 rounded-lg',
                'bg-red-500/20',
                'border border-red-500/30'
              )}
            >
              <AlertTriangle className="w-6 h-6 text-red-400" aria-hidden="true" />
            </div>
            <h2 className="text-lg font-semibold text-red-400">
              Schema Layout Error
            </h2>
          </div>

          {/* Error message */}
          <p className="text-white/70 text-sm mb-4">
            Failed to render the schema graph. This may be due to an ELK layout
            failure or a rendering issue.
          </p>

          {/* Error details */}
          {error && (
            <div className="mb-4">
              <p className="text-xs text-white/50 mb-2 uppercase tracking-wider">
                Error Message
              </p>
              <div
                className={cn(
                  'p-3 rounded-lg',
                  'bg-black/50',
                  'border border-white/[0.05]',
                  'text-left'
                )}
              >
                <code className="text-xs text-red-300 break-all">
                  {error.message || 'Unknown error'}
                </code>
              </div>
            </div>
          )}

          {/* Stack trace preview */}
          <div className="mb-6">
            <p className="text-xs text-white/50 mb-2 uppercase tracking-wider">
              Stack Trace Preview
            </p>
            <div
              className={cn(
                'p-3 rounded-lg',
                'bg-black/50',
                'border border-white/[0.05]',
                'text-left overflow-auto max-h-32'
              )}
            >
              <pre className="text-xs text-white/40 font-mono whitespace-pre-wrap">
                {this.getStackPreview()}
              </pre>
            </div>
          </div>

          {/* Retry button */}
          <button
            onClick={this.handleRetry}
            className={cn(
              'flex items-center justify-center gap-2',
              'w-full px-4 py-3 rounded-lg',
              'bg-white/10 hover:bg-white/15',
              'border border-white/[0.08] hover:border-white/[0.15]',
              'text-white/90 text-sm font-medium',
              'transition-all duration-200',
              'focus:outline-none focus:ring-2 focus:ring-white/20 focus:ring-offset-2 focus:ring-offset-black'
            )}
            aria-label="Retry rendering schema graph"
          >
            <RefreshCw className="w-4 h-4" aria-hidden="true" />
            Retry
          </button>

          {/* Help text */}
          <p className="mt-4 text-xs text-white/40">
            If the error persists, try refreshing the page or check the console
            for more details.
          </p>
        </div>
      </div>
    );
  }
}
