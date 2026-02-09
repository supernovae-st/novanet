// src/lib/__tests__/toast.test.ts
import { toast } from '../toast';
import { toast as sonnerToast } from 'sonner';

// Mock sonner
jest.mock('sonner', () => ({
  toast: {
    success: jest.fn(() => 'toast-id-success'),
    error: jest.fn(() => 'toast-id-error'),
    info: jest.fn(() => 'toast-id-info'),
    warning: jest.fn(() => 'toast-id-warning'),
    loading: jest.fn(() => 'toast-id-loading'),
    dismiss: jest.fn(),
    promise: jest.fn(),
  },
}));

describe('toast', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe('core methods', () => {
    describe('success', () => {
      it('should call sonner success with message', () => {
        toast.success('Operation completed');

        expect(sonnerToast.success).toHaveBeenCalledWith('Operation completed', {
          description: undefined,
        });
      });

      it('should include description when provided', () => {
        toast.success('Operation completed', 'All items processed');

        expect(sonnerToast.success).toHaveBeenCalledWith('Operation completed', {
          description: 'All items processed',
        });
      });

      it('should pass additional options', () => {
        toast.success('Done', undefined, { duration: 3000 });

        expect(sonnerToast.success).toHaveBeenCalledWith('Done', {
          description: undefined,
          duration: 3000,
        });
      });
    });

    describe('error', () => {
      it('should call sonner error with message and 5s duration', () => {
        toast.error('Something went wrong');

        expect(sonnerToast.error).toHaveBeenCalledWith('Something went wrong', {
          description: undefined,
          duration: 5000,
        });
      });

      it('should include description when provided', () => {
        toast.error('Query failed', 'Invalid syntax');

        expect(sonnerToast.error).toHaveBeenCalledWith('Query failed', {
          description: 'Invalid syntax',
          duration: 5000,
        });
      });
    });

    describe('info', () => {
      it('should call sonner info with message', () => {
        toast.info('Tip: Use keyboard shortcuts');

        expect(sonnerToast.info).toHaveBeenCalledWith('Tip: Use keyboard shortcuts', {
          description: undefined,
        });
      });
    });

    describe('warning', () => {
      it('should call sonner warning with message and 4s duration', () => {
        toast.warning('Connection unstable');

        expect(sonnerToast.warning).toHaveBeenCalledWith('Connection unstable', {
          description: undefined,
          duration: 4000,
        });
      });
    });

    describe('loading', () => {
      it('should call sonner loading with message', () => {
        toast.loading('Processing...');

        expect(sonnerToast.loading).toHaveBeenCalledWith('Processing...', {
          description: undefined,
        });
      });

      it('should return toast id for later dismissal', () => {
        const result = toast.loading('Loading...');

        expect(result).toBe('toast-id-loading');
      });
    });

    describe('dismiss', () => {
      it('should call sonner dismiss without id', () => {
        toast.dismiss();

        expect(sonnerToast.dismiss).toHaveBeenCalledWith(undefined);
      });

      it('should call sonner dismiss with specific id', () => {
        toast.dismiss('toast-123');

        expect(sonnerToast.dismiss).toHaveBeenCalledWith('toast-123');
      });
    });
  });

  describe('NovaNet-specific helpers', () => {
    describe('copied', () => {
      it('should show generic copy message without argument', () => {
        toast.copied();

        expect(sonnerToast.success).toHaveBeenCalledWith('Copied to clipboard', {
          description: undefined,
        });
      });

      it('should show specific copy message with argument', () => {
        toast.copied('node ID');

        expect(sonnerToast.success).toHaveBeenCalledWith('Copied node ID', {
          description: undefined,
        });
      });
    });

    describe('nodeExpansion', () => {
      it('should show info when count is 0', () => {
        toast.nodeExpansion(0);

        expect(sonnerToast.info).toHaveBeenCalledWith('No connected nodes found', {
          description: undefined,
        });
      });

      it('should show info with nodeType when count is 0', () => {
        toast.nodeExpansion(0, 'Entity');

        expect(sonnerToast.info).toHaveBeenCalledWith('No connected Entity found', {
          description: undefined,
        });
      });

      it('should show singular node message for count of 1', () => {
        toast.nodeExpansion(1);

        expect(sonnerToast.success).toHaveBeenCalledWith('Added 1 node', {
          description: undefined,
        });
      });

      it('should show plural nodes message for count > 1', () => {
        toast.nodeExpansion(5);

        expect(sonnerToast.success).toHaveBeenCalledWith('Added 5 nodes', {
          description: undefined,
        });
      });

      it('should include nodeType in description', () => {
        toast.nodeExpansion(3, 'Translation');

        expect(sonnerToast.success).toHaveBeenCalledWith('Added 3 nodes', {
          description: 'Expanded Translation',
        });
      });
    });

    describe('queryResult', () => {
      it('should show info when no results', () => {
        toast.queryResult(0);

        expect(sonnerToast.info).toHaveBeenCalledWith('Query returned no results', {
          description: undefined,
        });
      });

      it('should show info when both counts are 0', () => {
        toast.queryResult(0, 0);

        expect(sonnerToast.info).toHaveBeenCalledWith('Query returned no results', {
          description: undefined,
        });
      });

      it('should show singular node count', () => {
        toast.queryResult(1);

        expect(sonnerToast.success).toHaveBeenCalledWith('Found 1 node', {
          description: undefined,
        });
      });

      it('should show plural node count', () => {
        toast.queryResult(50);

        expect(sonnerToast.success).toHaveBeenCalledWith('Found 50 nodes', {
          description: undefined,
        });
      });

      it('should show both nodes and edges', () => {
        toast.queryResult(10, 5);

        expect(sonnerToast.success).toHaveBeenCalledWith('Found 10 nodes, 5 edges', {
          description: undefined,
        });
      });

      it('should show singular edge count', () => {
        toast.queryResult(2, 1);

        expect(sonnerToast.success).toHaveBeenCalledWith('Found 2 nodes, 1 edge', {
          description: undefined,
        });
      });

      it('should handle only edges (0 nodes but edges present)', () => {
        toast.queryResult(0, 5);

        // When nodeCount is 0 but edges exist, show only edges
        expect(sonnerToast.success).toHaveBeenCalledWith('Found 5 edges', {
          description: undefined,
        });
      });
    });

    describe('queryExecuting', () => {
      it('should show loading toast', () => {
        toast.queryExecuting();

        expect(sonnerToast.loading).toHaveBeenCalledWith('Executing query...', {
          description: undefined,
        });
      });
    });

    describe('queryError', () => {
      it('should show error with default message', () => {
        toast.queryError();

        expect(sonnerToast.error).toHaveBeenCalledWith('Query failed', {
          description: 'Check the Cypher syntax and try again',
          duration: 5000,
        });
      });

      it('should show error with custom message', () => {
        toast.queryError('Invalid node label');

        expect(sonnerToast.error).toHaveBeenCalledWith('Query failed', {
          description: 'Invalid node label',
          duration: 5000,
        });
      });
    });

  });

  describe('promise', () => {
    it('should pass promise to sonner.promise', () => {
      const promise = Promise.resolve('data');
      const messages = {
        loading: 'Loading...',
        success: 'Done!',
        error: 'Failed!',
      };

      toast.promise(promise, messages);

      expect(sonnerToast.promise).toHaveBeenCalledWith(promise, messages);
    });
  });

  describe('raw access', () => {
    it('should expose sonner toast for advanced usage', () => {
      expect(toast.raw).toBe(sonnerToast);
    });
  });
});
