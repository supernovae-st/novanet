/**
 * Types for SuperNovae Pad 3D Configurator
 */

export type NodeLayer =
  | 'config'
  | 'locale'
  | 'geography'
  | 'knowledge'
  | 'foundation'
  | 'structure'
  | 'semantic'
  | 'instruction'
  | 'output';

export interface KeyBinding {
  id: string;
  layer: NodeLayer;
  action: string;
  label?: string;
}

export interface VisualProps {
  color: string;
  icon: string;
}

export interface KeycapProps {
  position: [number, number, number];
  binding: KeyBinding;
  onPress: () => void;
  index: number;
}

export interface RotaryEncoderProps {
  position: [number, number, number];
  size: 'large' | 'small';
  onChange: (delta: number) => void;
  onClick?: () => void;
}

export interface SuperNovaePadModalProps {
  trigger?: React.ReactNode;
  defaultOpen?: boolean;
}

export interface SuperNovaePad3DProps {
  bindings?: KeyBinding[];
  onKeyPress?: (binding: KeyBinding) => void;
  onEncoderChange?: (encoderId: string, delta: number) => void;
}
