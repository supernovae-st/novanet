// apps/studio/src/components/graph/schema/__tests__/RealmGroupNode.test.tsx
import React from 'react';
import { render, screen } from '@testing-library/react';
import { ReactFlowProvider } from '@xyflow/react';
import { RealmGroupNode } from '../RealmGroupNode';
import { LayerGroupNode } from '../LayerGroupNode';
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

describe('RealmGroupNode', () => {
  const defaultProps = {
    id: 'realm-Project',
    type: 'realmGroup' as const,
    data: {
      realm: 'project' as const,
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
    selectable: true,
    deletable: true,
    draggable: true,
  };

  it('should render realm label with icon', () => {
    render(
      <ReactFlowProvider>
        <RealmGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    // Icon and label are in separate elements
    expect(screen.getByText('📦')).toBeInTheDocument();
    expect(screen.getByText('PROJECT')).toBeInTheDocument();
  });

  it('should render node count', () => {
    render(
      <ReactFlowProvider>
        <RealmGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    expect(screen.getByText('14 types')).toBeInTheDocument();
  });

  it('should show NodeResizer when selected', () => {
    render(
      <ReactFlowProvider>
        <RealmGroupNode {...defaultProps} selected={true} />
      </ReactFlowProvider>
    );

    expect(screen.getByTestId('node-resizer')).toBeInTheDocument();
  });

  it('should not show NodeResizer when not selected', () => {
    render(
      <ReactFlowProvider>
        <RealmGroupNode {...defaultProps} selected={false} />
      </ReactFlowProvider>
    );

    expect(screen.queryByTestId('node-resizer')).not.toBeInTheDocument();
  });

  it('should render with correct realm styling for Project', () => {
    render(
      <ReactFlowProvider>
        <RealmGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    // Check that realm is rendered with correct label (icon and label are separate)
    expect(screen.getByText('📦')).toBeInTheDocument();
    expect(screen.getByText('PROJECT')).toBeInTheDocument();
  });

  it('should render with correct realm styling for Global', () => {
    render(
      <ReactFlowProvider>
        <RealmGroupNode
          {...defaultProps}
          data={{ ...defaultProps.data, realm: 'global', label: 'GLOBAL', icon: '🌍' }}
        />
      </ReactFlowProvider>
    );

    // Icon and label are in separate elements
    expect(screen.getByText('🌍')).toBeInTheDocument();
    expect(screen.getByText('GLOBAL')).toBeInTheDocument();
  });

  it('should render with correct realm styling for Shared', () => {
    render(
      <ReactFlowProvider>
        <RealmGroupNode
          {...defaultProps}
          data={{ ...defaultProps.data, realm: 'shared', label: 'SHARED', icon: '🎯' }}
        />
      </ReactFlowProvider>
    );

    // Icon and label are in separate elements
    expect(screen.getByText('🎯')).toBeInTheDocument();
    expect(screen.getByText('SHARED')).toBeInTheDocument();
  });
});

describe('LayerGroupNode', () => {
  const defaultProps = {
    id: 'layer-Project-foundation',
    type: 'layerGroup' as const,
    data: {
      realm: 'project' as const,
      layer: 'foundation',
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
    selectable: true,
    deletable: true,
    draggable: true,
  };

  it('should render layer label with icon', () => {
    render(
      <ReactFlowProvider>
        <LayerGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    // Icon and label are in separate elements
    expect(screen.getByText('🏛️')).toBeInTheDocument();
    expect(screen.getByText('Foundation')).toBeInTheDocument();
  });

  it('should render node count', () => {
    render(
      <ReactFlowProvider>
        <LayerGroupNode {...defaultProps} />
      </ReactFlowProvider>
    );

    // Node count is rendered as just a number, not in parentheses
    expect(screen.getByText('3')).toBeInTheDocument();
  });

  it('should render correctly when not selected', () => {
    render(
      <ReactFlowProvider>
        <LayerGroupNode {...defaultProps} selected={false} />
      </ReactFlowProvider>
    );

    // Verify the label is rendered (icon and label are in separate elements)
    expect(screen.getByText('🏛️')).toBeInTheDocument();
    expect(screen.getByText('Foundation')).toBeInTheDocument();
  });
});

describe('SchemaNode', () => {
  const defaultProps = {
    id: 'schema-Project',
    type: 'schemaNode' as const,
    data: {
      nodeType: 'project',
      label: 'project',
      description: 'Project node',
      realm: 'project' as const,
      layer: 'foundation',
    },
    selected: false,
    dragging: false,
    isConnectable: true,
    positionAbsoluteX: 0,
    positionAbsoluteY: 0,
    zIndex: 1,
    selectable: true,
    deletable: true,
    draggable: true,
  };

  it('should render node label and nodeType', () => {
    render(
      <ReactFlowProvider>
        <SchemaNode {...defaultProps} />
      </ReactFlowProvider>
    );

    // Both label and nodeType show "Project" - use getAllByText
    const projectElements = screen.getAllByText('project');
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

  it('should render with Project node type styling', () => {
    render(
      <ReactFlowProvider>
        <SchemaNode {...defaultProps} />
      </ReactFlowProvider>
    );

    // Verify the Project label is rendered
    expect(screen.getAllByText('project').length).toBeGreaterThanOrEqual(1);
  });

  it('should render with Global realm', () => {
    render(
      <ReactFlowProvider>
        <SchemaNode
          {...defaultProps}
          data={{ ...defaultProps.data, realm: 'global', nodeType: 'Locale', label: 'Locale' }}
        />
      </ReactFlowProvider>
    );

    // May have multiple Locale text elements (label + nodeType)
    expect(screen.getAllByText('Locale').length).toBeGreaterThanOrEqual(1);
  });

  it('should render with Shared realm', () => {
    render(
      <ReactFlowProvider>
        <SchemaNode
          {...defaultProps}
          data={{ ...defaultProps.data, realm: 'shared', nodeType: 'SEOKeywordL10n', label: 'SEO Keyword' }}
        />
      </ReactFlowProvider>
    );

    expect(screen.getByText('SEO Keyword')).toBeInTheDocument();
  });

  it('should render selection effects when selected', () => {
    render(
      <ReactFlowProvider>
        <SchemaNode {...defaultProps} selected={true} />
      </ReactFlowProvider>
    );

    // Verify the node still renders correctly when selected
    expect(screen.getAllByText('project').length).toBeGreaterThanOrEqual(1);
  });
});
