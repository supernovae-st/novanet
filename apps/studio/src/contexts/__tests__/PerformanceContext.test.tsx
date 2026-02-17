import { render, screen } from '@testing-library/react';
import { PerformanceProvider, usePerformance } from '../PerformanceContext';

const TestComponent = () => {
  const { tier, config } = usePerformance();
  return (
    <div>
      <div data-testid="tier">{tier}</div>
      <div data-testid="techCorners">{config.effects.techCorners.toString()}</div>
    </div>
  );
};

describe('PerformanceContext', () => {
  it('provides default ULTRA tier when node count is low', () => {
    render(
      <PerformanceProvider nodeCount={10}>
        <TestComponent />
      </PerformanceProvider>
    );
    expect(screen.getByTestId('tier')).toHaveTextContent('ULTRA');
    expect(screen.getByTestId('techCorners')).toHaveTextContent('true');
  });

  it('provides HIGH tier when node count is 21-50', () => {
    render(
      <PerformanceProvider nodeCount={35}>
        <TestComponent />
      </PerformanceProvider>
    );
    expect(screen.getByTestId('tier')).toHaveTextContent('HIGH');
  });

  it('provides MEDIUM tier when node count is 51-100', () => {
    render(
      <PerformanceProvider nodeCount={75}>
        <TestComponent />
      </PerformanceProvider>
    );
    expect(screen.getByTestId('tier')).toHaveTextContent('MEDIUM');
  });

  it('provides LOW tier when node count exceeds 100', () => {
    render(
      <PerformanceProvider nodeCount={150}>
        <TestComponent />
      </PerformanceProvider>
    );
    expect(screen.getByTestId('tier')).toHaveTextContent('LOW');
    expect(screen.getByTestId('techCorners')).toHaveTextContent('true');
  });

  it('provides MINIMAL tier when node count exceeds 200', () => {
    render(
      <PerformanceProvider nodeCount={300}>
        <TestComponent />
      </PerformanceProvider>
    );
    expect(screen.getByTestId('tier')).toHaveTextContent('MINIMAL');
    expect(screen.getByTestId('techCorners')).toHaveTextContent('false');
  });

  it('allows tier override', () => {
    render(
      <PerformanceProvider nodeCount={10} overrideTier="MINIMAL">
        <TestComponent />
      </PerformanceProvider>
    );
    expect(screen.getByTestId('tier')).toHaveTextContent('MINIMAL');
  });

  it('throws error when usePerformance is used outside provider', () => {
    // Suppress console.error for this test
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});

    expect(() => {
      render(<TestComponent />);
    }).toThrow('usePerformance must be used within PerformanceProvider');

    consoleSpy.mockRestore();
  });
});
