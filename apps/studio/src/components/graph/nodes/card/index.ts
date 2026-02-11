/**
 * Card Component System
 *
 * Unified card wrapper and content components for graph nodes.
 *
 * Architecture:
 * - CardShell: Wrapper handling borders, effects, interactions
 * - CardContent presets: Reusable content layouts (structural, attractor, etc.)
 *
 * Usage:
 * ```tsx
 * import { CardShell, StructuralCardContent } from './card';
 *
 * const MyNode = (props) => (
 *   <CardShell
 *     colors={colors}
 *     selected={selected}
 *     renderContent={(ctx) => <StructuralCardContent data={data} {...ctx} />}
 *   />
 * );
 * ```
 */

export { CardShell, type CardShellProps, type CardContext, type CardColors } from './CardShell';

// Content presets
export {
  StructuralCardContent,
  type StructuralCardContentProps,
  type StructuralNodeData,
} from './presets/StructuralCardContent';

export {
  AttractorCardContent,
  type AttractorCardContentProps,
  type AttractorNodeData,
} from './presets/AttractorCardContent';

// TODO: Add more presets as needed
// export { ProjectCardContent } from './presets/ProjectCardContent';
// export { KnowledgeCardContent } from './presets/KnowledgeCardContent';
