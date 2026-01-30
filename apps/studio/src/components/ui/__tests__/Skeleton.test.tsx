// src/components/ui/__tests__/Skeleton.test.tsx
import React from 'react';
import { render } from '@testing-library/react';
import {
  Skeleton,
  SkeletonText,
  SkeletonCircle,
  SkeletonCard,
  ShimmerLoader,
  NodeSkeleton,
  PanelSkeleton,
} from '../Skeleton';

describe('Skeleton Components', () => {
  describe('Skeleton (base)', () => {
    it('should render with default classes', () => {
      const { container } = render(<Skeleton />);
      const skeleton = container.firstChild as HTMLElement;

      expect(skeleton).toHaveClass('animate-pulse');
      expect(skeleton).toHaveClass('rounded-md');
      expect(skeleton).toHaveClass('bg-white/10');
    });

    it('should accept custom className', () => {
      const { container } = render(<Skeleton className="w-full h-4" />);
      const skeleton = container.firstChild as HTMLElement;

      expect(skeleton).toHaveClass('w-full');
      expect(skeleton).toHaveClass('h-4');
    });

    it('should merge custom classes with defaults', () => {
      const { container } = render(<Skeleton className="custom-class" />);
      const skeleton = container.firstChild as HTMLElement;

      expect(skeleton).toHaveClass('animate-pulse');
      expect(skeleton).toHaveClass('custom-class');
    });
  });

  describe('SkeletonText', () => {
    it('should render 3 lines by default', () => {
      const { container } = render(<SkeletonText />);
      const skeletons = container.querySelectorAll('[class*="animate-pulse"]');

      expect(skeletons).toHaveLength(3);
    });

    it('should render specified number of lines', () => {
      const { container } = render(<SkeletonText lines={5} />);
      const skeletons = container.querySelectorAll('[class*="animate-pulse"]');

      expect(skeletons).toHaveLength(5);
    });

    it('should render 1 line when specified', () => {
      const { container } = render(<SkeletonText lines={1} />);
      const skeletons = container.querySelectorAll('[class*="animate-pulse"]');

      expect(skeletons).toHaveLength(1);
    });

    it('should make last line shorter (w-3/4)', () => {
      const { container } = render(<SkeletonText lines={3} />);
      const skeletons = container.querySelectorAll('[class*="animate-pulse"]');

      // First two lines should be full width
      expect(skeletons[0]).toHaveClass('w-full');
      expect(skeletons[1]).toHaveClass('w-full');
      // Last line should be 3/4 width
      expect(skeletons[2]).toHaveClass('w-3/4');
    });

    it('should apply custom className to container', () => {
      const { container } = render(<SkeletonText className="my-custom-class" />);
      const wrapper = container.firstChild as HTMLElement;

      expect(wrapper).toHaveClass('my-custom-class');
    });

    it('should have spacing between lines', () => {
      const { container } = render(<SkeletonText />);
      const wrapper = container.firstChild as HTMLElement;

      expect(wrapper).toHaveClass('space-y-2');
    });
  });

  describe('SkeletonCircle', () => {
    it('should render with md size by default', () => {
      const { container } = render(<SkeletonCircle />);
      const circle = container.firstChild as HTMLElement;

      expect(circle).toHaveClass('w-12');
      expect(circle).toHaveClass('h-12');
      expect(circle).toHaveClass('rounded-full');
    });

    it('should render sm size', () => {
      const { container } = render(<SkeletonCircle size="sm" />);
      const circle = container.firstChild as HTMLElement;

      expect(circle).toHaveClass('w-8');
      expect(circle).toHaveClass('h-8');
    });

    it('should render lg size', () => {
      const { container } = render(<SkeletonCircle size="lg" />);
      const circle = container.firstChild as HTMLElement;

      expect(circle).toHaveClass('w-16');
      expect(circle).toHaveClass('h-16');
    });

    it('should apply custom className', () => {
      const { container } = render(<SkeletonCircle className="custom-circle" />);
      const circle = container.firstChild as HTMLElement;

      expect(circle).toHaveClass('custom-circle');
      expect(circle).toHaveClass('rounded-full');
    });
  });

  describe('SkeletonCard', () => {
    it('should render with card styling', () => {
      const { container } = render(<SkeletonCard />);
      const card = container.firstChild as HTMLElement;

      expect(card).toHaveClass('rounded-xl');
      expect(card).toHaveClass('border');
      expect(card).toHaveClass('border-white/10');
      expect(card).toHaveClass('bg-white/5');
    });

    it('should contain a circle skeleton for avatar', () => {
      const { container } = render(<SkeletonCard />);
      const circles = container.querySelectorAll('.rounded-full');

      expect(circles.length).toBeGreaterThan(0);
    });

    it('should contain text skeletons', () => {
      const { container } = render(<SkeletonCard />);
      const skeletons = container.querySelectorAll('[class*="animate-pulse"]');

      // Should have multiple skeleton elements
      expect(skeletons.length).toBeGreaterThan(3);
    });

    it('should apply custom className', () => {
      const { container } = render(<SkeletonCard className="my-card" />);
      const card = container.firstChild as HTMLElement;

      expect(card).toHaveClass('my-card');
    });
  });

  describe('ShimmerLoader', () => {
    it('should render with shimmer animation', () => {
      const { container } = render(<ShimmerLoader />);
      const loader = container.firstChild as HTMLElement;

      expect(loader).toHaveClass('overflow-hidden');
      expect(loader).toHaveClass('rounded-lg');
      expect(loader).toHaveClass('bg-white/5');
    });

    it('should have shimmer animation element', () => {
      const { container } = render(<ShimmerLoader />);
      const shimmerElement = container.querySelector('[class*="shimmer"]');

      expect(shimmerElement).toBeInTheDocument();
    });

    it('should apply custom className', () => {
      const { container } = render(<ShimmerLoader className="my-shimmer" />);
      const loader = container.firstChild as HTMLElement;

      expect(loader).toHaveClass('my-shimmer');
    });
  });

  describe('NodeSkeleton', () => {
    it('should render with node-like dimensions', () => {
      const { container } = render(<NodeSkeleton />);
      const node = container.firstChild as HTMLElement;

      expect(node).toHaveClass('w-48');
      expect(node).toHaveClass('rounded-xl');
    });

    it('should have icon and title skeletons', () => {
      const { container } = render(<NodeSkeleton />);
      const skeletons = container.querySelectorAll('[class*="animate-pulse"]');

      // Should have icon (small square) + title + subtitle
      expect(skeletons.length).toBeGreaterThanOrEqual(3);
    });

    it('should apply custom className', () => {
      const { container } = render(<NodeSkeleton className="custom-node" />);
      const node = container.firstChild as HTMLElement;

      expect(node).toHaveClass('custom-node');
    });
  });

  describe('PanelSkeleton', () => {
    it('should render with panel layout', () => {
      const { container } = render(<PanelSkeleton />);
      const panel = container.firstChild as HTMLElement;

      expect(panel).toHaveClass('space-y-4');
      expect(panel).toHaveClass('p-4');
    });

    it('should contain header with avatar circle', () => {
      const { container } = render(<PanelSkeleton />);
      const circles = container.querySelectorAll('.rounded-full');

      expect(circles.length).toBeGreaterThan(0);
    });

    it('should contain multiple content sections', () => {
      const { container } = render(<PanelSkeleton />);
      const textGroups = container.querySelectorAll('.space-y-2');

      // Should have text skeleton groups
      expect(textGroups.length).toBeGreaterThan(0);
    });

    it('should apply custom className', () => {
      const { container } = render(<PanelSkeleton className="custom-panel" />);
      const panel = container.firstChild as HTMLElement;

      expect(panel).toHaveClass('custom-panel');
    });
  });

  describe('accessibility', () => {
    it('Skeleton should have correct role for screen readers', () => {
      // Skeletons are decorative, they don't need specific roles
      // but should not interfere with accessibility
      const { container } = render(<Skeleton data-testid="skeleton" />);
      const skeleton = container.firstChild as HTMLElement;

      // Should just be a div without aria attributes that would confuse
      expect(skeleton.tagName).toBe('DIV');
    });
  });
});
