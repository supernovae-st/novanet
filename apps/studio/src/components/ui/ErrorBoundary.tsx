'use client';

/**
 * Error Boundary Component
 *
 * Catches React errors and displays a fallback UI.
 * Critical for preventing app crashes from component errors.
 *
 * @example
 * <ErrorBoundary fallback={<div>Something went wrong</div>}>
 *   <MyComponent />
 * </ErrorBoundary>
 */

import { Component, type ReactNode, type ErrorInfo } from 'react';
import { cn } from '@/lib/utils';
import { logger } from '@/lib/logger';
import { STATUS_ICONS, ACTION_ICONS } from '@/config/iconSystem';

// Design system icons
const WarningIcon = STATUS_ICONS.warning;
const RefreshIcon = ACTION_ICONS.refresh;

export interface ErrorBoundaryProps {
  /** Child components to wrap */
  children: ReactNode;
  /** Custom fallback UI (optional) */
  fallback?: ReactNode;
  /** Callback when error occurs */
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
  /** Additional class names for wrapper */
  className?: string;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = {
      hasError: false,
      error: null,
      errorInfo: null,
    };
  }

  static getDerivedStateFromError(error: Error): Partial<ErrorBoundaryState> {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    this.setState({ errorInfo });

    // Log error for debugging
    logger.error('ErrorBoundary', 'Caught error', error);
    logger.error('ErrorBoundary', 'Component stack', errorInfo.componentStack);

    // Call optional error callback
    this.props.onError?.(error, errorInfo);
  }

  handleReset = (): void => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
    });
  };

  render(): ReactNode {
    const { hasError, error, errorInfo } = this.state;
    const { children, fallback, className } = this.props;

    if (hasError) {
      // Return custom fallback if provided
      if (fallback) {
        return fallback;
      }

      // Default error UI
      return (
        <div
          className={cn(
            'flex flex-col items-center justify-center p-8 text-center',
            'bg-red-500/10 border border-red-500/30 rounded-lg',
            className
          )}
        >
          <WarningIcon className="w-12 h-12 text-red-400 mb-4" />
          <h2 className="text-lg font-semibold text-red-400 mb-2">
            Something went wrong
          </h2>
          <p className="text-sm text-white/60 mb-4 max-w-md">
            {error?.message || 'An unexpected error occurred'}
          </p>

          {/* Show component stack in development */}
          {process.env.NODE_ENV === 'development' && errorInfo && (
            <details className="mb-4 max-w-full overflow-auto">
              <summary className="text-xs text-white/40 cursor-pointer hover:text-white/60">
                View error details
              </summary>
              <pre className="mt-2 text-xs text-left text-red-300/80 bg-black/30 p-2 rounded overflow-auto max-h-48">
                {error?.stack}
                {'\n\nComponent Stack:'}
                {errorInfo.componentStack}
              </pre>
            </details>
          )}

          <button
            onClick={this.handleReset}
            className={cn(
              'flex items-center gap-2 px-4 py-2 rounded-lg',
              'bg-red-500/20 hover:bg-red-500/30',
              'text-red-400 text-sm font-medium',
              'transition-colors'
            )}
          >
            <RefreshIcon className="w-4 h-4" />
            Try Again
          </button>
        </div>
      );
    }

    return children;
  }
}

/**
 * Chat-specific error boundary
 */
export function ChatErrorBoundary({ children }: { children: ReactNode }) {
  return (
    <ErrorBoundary
      fallback={
        <div className="flex flex-col items-center justify-center p-6 text-center">
          <WarningIcon className="w-10 h-10 text-amber-400 mb-3" />
          <h3 className="text-sm font-semibold text-white mb-1">
            Chat error
          </h3>
          <p className="text-xs text-white/60 mb-4">
            Something went wrong with the AI chat.
          </p>
          <button
            onClick={() => window.location.reload()}
            className={cn(
              'flex items-center gap-2 px-3 py-1.5 rounded-lg',
              'bg-novanet-500 hover:bg-novanet-600',
              'text-white text-xs font-medium',
              'transition-colors'
            )}
          >
            <RefreshIcon className="w-3 h-3" />
            Reload
          </button>
        </div>
      }
    >
      {children}
    </ErrorBoundary>
  );
}

/**
 * Graph-specific error boundary with connection error handling
 */
export function GraphErrorBoundary({ children }: { children: ReactNode }) {
  return (
    <ErrorBoundary
      className="h-full w-full"
      fallback={
        <div className="flex flex-col items-center justify-center h-full w-full p-8 text-center">
          <WarningIcon className="w-16 h-16 text-amber-400 mb-4" />
          <h2 className="text-xl font-semibold text-white mb-2">
            Graph visualization error
          </h2>
          <p className="text-sm text-white/60 mb-6 max-w-md">
            There was a problem rendering the graph. This might be due to invalid data
            or a rendering issue. Try refreshing the page or adjusting your filters.
          </p>
          <button
            onClick={() => window.location.reload()}
            className={cn(
              'flex items-center gap-2 px-4 py-2 rounded-lg',
              'bg-novanet-500 hover:bg-novanet-600',
              'text-white text-sm font-medium',
              'transition-colors'
            )}
          >
            <RefreshIcon className="w-4 h-4" />
            Reload Page
          </button>
        </div>
      }
    >
      {children}
    </ErrorBoundary>
  );
}
