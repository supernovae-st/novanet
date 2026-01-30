import { render, screen } from '@testing-library/react';
import { GlowingBorder, GlowingBorderState } from '../GlowingBorder';

describe('GlowingBorder', () => {
  describe('basic rendering', () => {
    it('renders children correctly', () => {
      render(
        <GlowingBorder color="#8b5cf6">
          <div data-testid="content">Test Content</div>
        </GlowingBorder>
      );
      expect(screen.getByTestId('content')).toBeInTheDocument();
      expect(screen.getByText('Test Content')).toBeInTheDocument();
    });

    it('applies custom className', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" className="custom-class">
          Content
        </GlowingBorder>
      );
      expect(container.firstChild).toHaveClass('custom-class');
    });

    it('applies border radius style', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" borderRadius={20}>
          Content
        </GlowingBorder>
      );
      const wrapper = container.firstChild as HTMLElement;
      expect(wrapper.style.borderRadius).toBe('20px');
    });
  });

  describe('state variants', () => {
    it('defaults to idle state', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6">
          Content
        </GlowingBorder>
      );
      // Idle state should not have state-specific animation classes
      const wrapper = container.firstChild as HTMLElement;
      expect(wrapper).not.toHaveClass('animate-pulse-fast');
      expect(wrapper).not.toHaveClass('animate-pulse-slow');
      expect(wrapper).not.toHaveClass('animate-shake');
      expect(wrapper).not.toHaveClass('animate-success-flash');
    });

    it('renders running state with pulse-fast animation', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" state="running">
          Content
        </GlowingBorder>
      );
      expect(container.querySelector('.animate-pulse-fast')).toBeInTheDocument();
    });

    it('renders pending state with pulse-slow animation', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" state="pending">
          Content
        </GlowingBorder>
      );
      expect(container.querySelector('.animate-pulse-slow')).toBeInTheDocument();
    });

    it('renders error state with shake animation', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" state="error">
          Content
        </GlowingBorder>
      );
      expect(container.querySelector('.animate-shake')).toBeInTheDocument();
    });

    it('renders success state with success-flash animation', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" state="success">
          Content
        </GlowingBorder>
      );
      expect(container.querySelector('.animate-success-flash')).toBeInTheDocument();
    });

    it('renders partial state without special animation', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" state="partial">
          Content
        </GlowingBorder>
      );
      // Partial state uses the color but no specific animation class
      const wrapper = container.firstChild as HTMLElement;
      expect(wrapper).not.toHaveClass('animate-pulse-fast');
      expect(wrapper).not.toHaveClass('animate-pulse-slow');
      expect(wrapper).not.toHaveClass('animate-shake');
      expect(wrapper).not.toHaveClass('animate-success-flash');
    });
  });

  describe('state colors', () => {
    // Test that each state type is valid
    const validStates: GlowingBorderState[] = ['idle', 'running', 'pending', 'error', 'success', 'partial'];

    validStates.forEach((state) => {
      it(`accepts "${state}" as a valid state`, () => {
        const { container } = render(
          <GlowingBorder color="#8b5cf6" state={state}>
            Content
          </GlowingBorder>
        );
        expect(container.firstChild).toBeInTheDocument();
      });
    });
  });

  describe('interactive states', () => {
    it('applies breathing animation when selected', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" isSelected>
          Content
        </GlowingBorder>
      );
      expect(container.querySelector('.animate-breathing')).toBeInTheDocument();
    });

    it('does not apply breathing animation when state animation is active', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" isSelected state="running">
          Content
        </GlowingBorder>
      );
      // When state has an animation, breathing should be suppressed
      expect(container.querySelector('.animate-breathing')).not.toBeInTheDocument();
      expect(container.querySelector('.animate-pulse-fast')).toBeInTheDocument();
    });

    it('shows animated beam when animated prop is true', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" animated>
          Content
        </GlowingBorder>
      );
      // Check for the animated beam container (has conic-gradient animation)
      expect(container.querySelector('.animate-shine-beam, .animate-spin-slow')).toBeInTheDocument();
    });

    it('shows animated beam when hovered', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" isHovered>
          Content
        </GlowingBorder>
      );
      expect(container.querySelector('.animate-shine-beam, .animate-spin-slow')).toBeInTheDocument();
    });

    it('shows animated beam for active states (running/pending)', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" state="running">
          Content
        </GlowingBorder>
      );
      expect(container.querySelector('.animate-shine-beam, .animate-spin-slow')).toBeInTheDocument();
    });
  });

  describe('secondary color', () => {
    it('uses primary color when secondary not provided', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6">
          Content
        </GlowingBorder>
      );
      expect(container.firstChild).toBeInTheDocument();
    });

    it('accepts secondary color prop', () => {
      const { container } = render(
        <GlowingBorder color="#8b5cf6" colorSecondary="#a78bfa">
          Content
        </GlowingBorder>
      );
      expect(container.firstChild).toBeInTheDocument();
    });
  });
});
