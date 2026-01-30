// apps/studio/src/components/graph/schema/__tests__/ScopeGroupNode.test.tsx
import React from 'react';
import { render, screen } from '@testing-library/react';
import { ReactFlowProvider } from '@xyflow/react';
import { ScopeGroupNode } from '../ScopeGroupNode';
import { SubcategoryGroupNode } from '../SubcategoryGroupNode';
import { SchemaNode } from '../SchemaNode';

// Mock NodeResizer since it requires internal React Flow context
jest.mock('@xyflow/react', () => ({
  ...jest.requireActual('@xyflow/react'),
  NodeResizer: ({ isVisible }: { isVisible: boolean }) => (
    isVisible ? <div data-testid="node-resizer">Resizer</div> : null
  ),
  Handle: ({ type, position }: { type: string; position: string }) => (
    <div data-testid={`handle-${type}`} data-position={position} />
  ),
  Position: {
    Left: 'left',
    Right: 'right',
    Top: 'top',
    Bottom: 'bottom',
  },
}));

describe('ScopeGroupNode', () => {
  const defaultProps = {
    id: 'scope-Project',
    type: 'scopeGroup' as const,
    data: {
      scope: 'Project' as const,
      label: 'PROJECT',
      icon: '📦',
      nodeCount: 14,
    },
    selected: false,
    dragging: false,
    isConnectable: true,
    positionAbsoluteX: 0,
    positionAbsoluteY: 0,
    zIndex: 1,
  };

  it('should render scope label with icon', () => {
    render(
      <ReactFlowProvider>
        <ScopeGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    expect(screen.getByText('📦 PROJECT')).toBeInTheDocument();
  });

  it('should render node count', () => {
    render(
      <ReactFlowProvider>
        <ScopeGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    expect(screen.getByText('14 types')).toBeInTheDocument();
  });

  it('should show NodeResizer when selected', () => {
    render(
      <ReactFlowProvider>
        <ScopeGroupNode {...defaultProps} selected={true} />
      </ReactFlowProvider>
    );

    expect(screen.getByTestId('node-resizer')).toBeInTheDocument();
  });

  it('should not show NodeResizer when not selected', () => {
    render(
      <ReactFlowProvider>
        <ScopeGroupNode {...defaultProps} selected={false} />
      </ReactFlowProvider>
    );

    expect(screen.queryByTestId('node-resizer')).not.toBeInTheDocument();
  });

  it('should apply violet color for Project scope', () => {
    const { container } = render(
      <ReactFlowProvider>
        <ScopeGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    // Check that violet border class is applied
    const scopeDiv = container.querySelector('.border-violet-500\\/50');
    expect(scopeDiv).toBeInTheDocument();
  });

  it('should apply emerald color for Global scope', () => {
    const { container } = render(
      <ReactFlowProvider>
        <ScopeGroupNode
          {...defaultProps}
          data={{ ...defaultProps.data, scope: 'Global', label: 'GLOBAL', icon: '🌍' }}
        />
      </ReactFlowProvider>
    );

    const scopeDiv = container.querySelector('.border-emerald-500\\/50');
    expect(scopeDiv).toBeInTheDocument();
  });

  it('should apply amber color for Shared scope', () => {
    const { container } = render(
      <ReactFlowProvider>
        <ScopeGroupNode
          {...defaultProps}
          data={{ ...defaultProps.data, scope: 'Shared', label: 'SHARED', icon: '🎯' }}
        />
      </ReactFlowProvider>
    );

    const scopeDiv = container.querySelector('.border-amber-500\\/50');
    expect(scopeDiv).toBeInTheDocument();
  });
});

describe('SubcategoryGroupNode', () => {
  const defaultProps = {
    id: 'subcat-Project-foundation',
    type: 'subcategoryGroup' as const,
    data: {
      subcategory: 'foundation',
      label: 'Foundation',
      icon: '🏛️',
      nodeCount: 3,
    },
    selected: false,
    dragging: false,
    isConnectable: true,
    positionAbsoluteX: 0,
    positionAbsoluteY: 0,
    zIndex: 1,
  };

  it('should render subcategory label with icon', () => {
    render(
      <ReactFlowProvider>
        <SubcategoryGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    expect(screen.getByText('🏛️ Foundation')).toBeInTheDocument();
  });

  it('should render node count in parentheses', () => {
    render(
      <ReactFlowProvider>
        <SubcategoryGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    expect(screen.getByText('(3)')).toBeInTheDocument();
  });

  it('should apply dimmed style when not selected', () => {
    const { container } = render(
      <ReactFlowProvider>
        <SubcategoryGroupNode {...defaultProps} selected={false} />
      </ReactFlowProvider>
    );

    const labelSpan = container.querySelector('.text-white\\/70');
    expect(labelSpan).toBeInTheDocument();
  });
});

describe('SchemaNode', () => {
  const defaultProps = {
    id: 'schema-Project',
    type: 'schemaNode' as const,
    data: {
      nodeType: 'Project',
      label: 'Project',
      description: 'Project node',
      scope: 'Project' as const,
      subcategory: 'foundation',
    },
    selected: false,
    dragging: false,
    isConnectable: true,
    positionAbsoluteX: 0,
    positionAbsoluteY: 0,
    zIndex: 1,
  };

  it('should render node label and nodeType', () => {
    render(
      <ReactFlowProvider>
        <SchemaNode {...defaultProps} />
      </ReactFlowProvider>
    );

    // Both label and nodeType show "Project" - use getAllByText
    const projectElements = screen.getAllByText('Project');
    expect(projectElements.length).toBe(2); // label + nodeType
  });

  it('should have source and target handles', () => {
    render(
      <ReactFlowProvider>
        <SchemaNode {...defaultProps} />
      </ReactFlowProvider>
    );

    expect(screen.getByTestId('handle-target')).toBeInTheDocument();
    expect(screen.getByTestId('handle-source')).toBeInTheDocument();
  });

  it('should apply violet accent for Project scope', () => {
    const { container } = render(
      <ReactFlowProvider>
        <SchemaNode {...defaultProps} />
      </ReactFlowProvider>
    );

    const nodeDiv = container.querySelector('.border-l-violet-500');
    expect(nodeDiv).toBeInTheDocument();
  });

  it('should apply emerald accent for Global scope', () => {
    const { container } = render(
      <ReactFlowProvider>
        <SchemaNode
          {...defaultProps}
          data={{ ...defaultProps.data, scope: 'Global' }}
        />
      </ReactFlowProvider>
    );

    const nodeDiv = container.querySelector('.border-l-emerald-500');
    expect(nodeDiv).toBeInTheDocument();
  });

  it('should apply amber accent for Shared scope', () => {
    const { container } = render(
      <ReactFlowProvider>
        <SchemaNode
          {...defaultProps}
          data={{ ...defaultProps.data, scope: 'Shared' }}
        />
      </ReactFlowProvider>
    );

    const nodeDiv = container.querySelector('.border-l-amber-500');
    expect(nodeDiv).toBeInTheDocument();
  });

  it('should apply selection ring when selected', () => {
    const { container } = render(
      <ReactFlowProvider>
        <SchemaNode {...defaultProps} selected={true} />
      </ReactFlowProvider>
    );

    const selectedDiv = container.querySelector('.ring-2');
    expect(selectedDiv).toBeInTheDocument();
  });
});
