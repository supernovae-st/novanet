/**
 * Card Content Presets Index
 *
 * Exports all card content presets for use with CardShell
 */

// Attractor cards (for magnetic grouping layout)
export { AttractorCardContent, type AttractorCardContentProps } from './AttractorCardContent';

// Class cards (Holographic Blueprint design)
export {
  ClassCardContent,
  type ClassNodeData,
  type ClassCardContentProps,
} from './ClassCardContent';

// Locale cards (Passport Élégant design)
export {
  LocaleCardContent,
  type LocaleNodeData,
  type LocaleCardContentProps,
} from './LocaleCardContent';

// Project cards (Premium social network style)
export {
  ProjectCardContent,
  type ProjectNodeData,
  type ProjectCardContentProps,
} from './ProjectCardContent';

// Structural cards (standard card for Page, Entity, Block, etc.)
export {
  StructuralCardContent,
  type StructuralNodeData,
  type StructuralCardContentProps,
} from './StructuralCardContent';

// Realm cards (Orbital Gateway design)
export {
  RealmOrbitalCardContent,
  type RealmNodeData,
  type RealmOrbitalCardProps,
} from './RealmOrbitalCardContent';

// Taxonomy cards (Premium TAXONOMY level design - Realm/Layer/Trait/ArcFamily)
export {
  TaxonomyCardContent,
  type TaxonomyNodeData,
  type TaxonomyCardContentProps,
  type TaxonomyVariant,
} from './TaxonomyCardContent';
