// design/icons/index.ts
// Central export point for node icon system
//
// Node icons are generated from YAML (Single Source of Truth)
// UI icons use lucide-react via config/iconSystem.ts

// Generated from YAML - Single Source of Truth
export {
  NODE_ICONS,
  getNodeWebIcon,
  getNodeTerminalIcon,
  DEFAULT_NODE_ICON,
} from './nodeIcons.generated';
export type { NodeIcon } from './nodeIcons.generated';
export type { NodeType } from '@novanet/core/types';
