// src/components/graph/nodes/__tests__/NodeTooltip.test.tsx
import React from 'react';
import { render, screen } from '@testing-library/react';
import { NodeTooltip } from '../NodeTooltip';

// Mock NODE_TYPE_CONFIG
jest.mock('@/config/nodeTypes', () => ({
  NODE_TYPE_CONFIG: {
    Project: { label: 'Project', icon: '📁', color: '#3b82f6' },
    Concept: { label: 'Concept', icon: '💡', color: '#8b5cf6' },
    Block: { label: 'Block', icon: '📝', color: '#10b981' },
    Locale: { label: 'Locale', icon: '🌐', color: '#f59e0b' },
  },
}));

describe('NodeTooltip', () => {
  describe('visibility', () => {
    it('should not render when visible is false', () => {
      render(
        <NodeTooltip
          visible={false}
          nodeType="Project"
          nodeKey="test-project"
        />
      );

      expect(screen.queryByText('Project')).not.toBeInTheDocument();
    });

    it('should render when visible is true', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Project"
          nodeKey="test-project"
        />
      );

      expect(screen.getByText('Project')).toBeInTheDocument();
    });
  });

  describe('content display', () => {
    it('should display the node type label', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Concept"
          nodeKey="my-concept"
        />
      );

      expect(screen.getByText('Concept')).toBeInTheDocument();
    });

    it('should display the node key', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Project"
          nodeKey="unique-key-123"
        />
      );

      expect(screen.getByText('unique-key-123')).toBeInTheDocument();
    });

    it('should display displayName with key in parentheses when different', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Block"
          nodeKey="block-001"
          displayName="Welcome Message"
        />
      );

      expect(screen.getByText('Welcome Message')).toBeInTheDocument();
      expect(screen.getByText('(block-001)')).toBeInTheDocument();
    });

    it('should only show key when displayName equals nodeKey', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Locale"
          nodeKey="fr-FR"
          displayName="fr-FR"
        />
      );

      // Should only appear once (not duplicated)
      const frFRElements = screen.getAllByText('fr-FR');
      expect(frFRElements).toHaveLength(1);
    });
  });

  describe('styling', () => {
    it('should apply custom color when provided', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Project"
          nodeKey="test"
          color="#ff0000"
        />
      );

      // Portal renders to document.body
      const coloredSpan = document.body.querySelector('[style*="color: rgb(255, 0, 0)"]');
      expect(coloredSpan).toBeInTheDocument();
    });

    it('should use NODE_TYPE_CONFIG color when no custom color', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Concept"
          nodeKey="test"
        />
      );

      // Concept has color #8b5cf6 - portal renders to document.body
      const coloredSpan = document.body.querySelector('[style*="color"]');
      expect(coloredSpan).toHaveStyle({ color: '#8b5cf6' });
    });

    it('should apply custom className', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Project"
          nodeKey="test"
          className="custom-tooltip-class"
        />
      );

      const tooltip = document.body.querySelector('.custom-tooltip-class');
      expect(tooltip).toBeInTheDocument();
    });

    it('should have tooltip animation class', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Project"
          nodeKey="test"
        />
      );

      const tooltip = document.body.querySelector('.animate-tooltip-fade-in');
      expect(tooltip).toBeInTheDocument();
    });

    it('should have fixed positioning classes for center pill placement', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Project"
          nodeKey="test"
        />
      );

      const tooltip = document.body.querySelector('.node-tooltip');
      expect(tooltip).toHaveClass('fixed');
      expect(tooltip).toHaveClass('left-1/2');
      expect(tooltip).toHaveClass('-translate-x-1/2');
      expect(tooltip).toHaveClass('bottom-20');
    });
  });

  describe('fallback behavior', () => {
    it('should use Project config for unknown node types', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType={'UnknownType' as never}
          nodeKey="test"
        />
      );

      // Should fallback to Project config
      expect(screen.getByText('Project')).toBeInTheDocument();
    });
  });

  describe('icon display', () => {
    it('should display the category icon from config', () => {
      render(
        <NodeTooltip
          visible={true}
          nodeType="Locale"
          nodeKey="en-US"
        />
      );

      // Category icon is now an SVG (CategoryIcon component using Lucide icons)
      // Portal renders to document.body, so query there
      const svg = document.body.querySelector('svg');
      expect(svg).toBeInTheDocument();
    });
  });
});
