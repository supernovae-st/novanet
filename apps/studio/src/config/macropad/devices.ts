/**
 * Work Louder Device Definitions
 *
 * Source of truth for supported Work Louder devices.
 * Based on ~/Projects/work-louder/studio-integration/configs/
 */

// WebHID API type (not available in all TS configs)
interface HIDDevice {
  vendorId: number;
  productId: number;
  productName: string;
  collections: readonly { usage: number; usagePage: number }[];
  opened: boolean;
  open(): Promise<void>;
  close(): Promise<void>;
  sendReport(reportId: number, data: BufferSource): Promise<void>;
  sendFeatureReport(reportId: number, data: BufferSource): Promise<void>;
  receiveFeatureReport(reportId: number): Promise<DataView>;
  addEventListener(type: string, listener: EventListener): void;
  removeEventListener(type: string, listener: EventListener): void;
}

export interface DeviceDefinition {
  name: string;
  vendorId: number;
  productId: number;
  matrix: { rows: number; cols: number };
  encoders: number;
  defaultLayers: Record<number, string>;
}

// Work Louder vendor ID
export const WORK_LOUDER_VENDOR_ID = 0x574c;

// Device definitions
export const DEVICES: Record<string, DeviceDefinition> = {
  creator_board: {
    name: 'Creator Board',
    vendorId: 0x574c,
    productId: 0xe6e3,
    matrix: { rows: 3, cols: 4 },
    encoders: 1,
    defaultLayers: {
      0: 'Navigation',
      1: 'YAML & Overlays',
      2: 'System',
      3: 'Custom',
    },
  },
  creator_micro: {
    name: 'Creator Micro',
    vendorId: 0x574c,
    productId: 0x1001,
    matrix: { rows: 2, cols: 3 },
    encoders: 1,
    defaultLayers: {
      0: 'Base',
      1: 'Function',
    },
  },
  loop: {
    name: 'Loop',
    vendorId: 0x574c,
    productId: 0x1002,
    matrix: { rows: 1, cols: 12 },
    encoders: 0,
    defaultLayers: {
      0: 'Base',
      1: 'Alt',
    },
  },
};

// NovaNet TUI preset keycodes
export const NOVANET_PRESETS = {
  navigation: ['KC_ESC', 'KC_UP', 'KC_ENT', 'KC_LEFT', 'KC_DOWN', 'KC_RGHT'],
  media: ['KC_MPLY', 'KC_VOLU', 'KC_MNXT', 'KC_MSTP', 'KC_VOLD', 'KC_MPRV'],
  system: ['G(KC_TAB)', 'G(KC_UP)', 'G(KC_ENT)', 'G(KC_LEFT)', 'G(KC_DOWN)', 'G(KC_RGHT)'],
};

// Default device for NovaNet Studio
export const DEFAULT_DEVICE = DEVICES.creator_board;

// Get device by vendor/product ID
export function getDeviceByIds(vendorId: number, productId: number): DeviceDefinition | null {
  return (
    Object.values(DEVICES).find(
      (d) => d.vendorId === vendorId && d.productId === productId
    ) ?? null
  );
}

// Check if a HID device is a Work Louder device
export function isWorkLouderDevice(device: HIDDevice): boolean {
  return device.vendorId === WORK_LOUDER_VENDOR_ID;
}
