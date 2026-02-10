/**
 * WebHID API Type Definitions
 *
 * Based on WebHID API spec: https://wicg.github.io/webhid/
 * These types are not available in all TypeScript configurations.
 */

interface HIDDevice {
  vendorId: number;
  productId: number;
  productName: string;
  collections: readonly HIDCollectionInfo[];
  opened: boolean;
  open(): Promise<void>;
  close(): Promise<void>;
  sendReport(reportId: number, data: BufferSource): Promise<void>;
  sendFeatureReport(reportId: number, data: BufferSource): Promise<void>;
  receiveFeatureReport(reportId: number): Promise<DataView>;
  addEventListener(type: 'inputreport', listener: (event: HIDInputReportEvent) => void): void;
  removeEventListener(type: 'inputreport', listener: (event: HIDInputReportEvent) => void): void;
  oninputreport: ((event: HIDInputReportEvent) => void) | null;
}

interface HIDCollectionInfo {
  usage: number;
  usagePage: number;
  inputReports?: HIDReportInfo[];
  outputReports?: HIDReportInfo[];
  featureReports?: HIDReportInfo[];
  children?: HIDCollectionInfo[];
}

interface HIDReportInfo {
  reportId: number;
  items?: HIDReportItem[];
}

interface HIDReportItem {
  isAbsolute?: boolean;
  isArray?: boolean;
  isBufferedBytes?: boolean;
  isConstant?: boolean;
  isLinear?: boolean;
  isRange?: boolean;
  isVolatile?: boolean;
  hasNull?: boolean;
  hasPreferredState?: boolean;
  wrap?: boolean;
  usages?: number[];
  usageMinimum?: number;
  usageMaximum?: number;
  reportSize?: number;
  reportCount?: number;
  unitExponent?: number;
  unit?: string;
  logicalMinimum?: number;
  logicalMaximum?: number;
  physicalMinimum?: number;
  physicalMaximum?: number;
  strings?: string[];
}

interface HIDInputReportEvent extends Event {
  device: HIDDevice;
  reportId: number;
  data: DataView;
}

interface HIDConnectionEvent extends Event {
  device: HIDDevice;
}

interface HIDDeviceFilter {
  vendorId?: number;
  productId?: number;
  usagePage?: number;
  usage?: number;
}

interface HIDDeviceRequestOptions {
  filters: HIDDeviceFilter[];
}

interface HID extends EventTarget {
  getDevices(): Promise<HIDDevice[]>;
  requestDevice(options: HIDDeviceRequestOptions): Promise<HIDDevice[]>;
  addEventListener(type: 'connect', listener: (event: HIDConnectionEvent) => void): void;
  addEventListener(type: 'disconnect', listener: (event: HIDConnectionEvent) => void): void;
  removeEventListener(type: 'connect', listener: (event: HIDConnectionEvent) => void): void;
  removeEventListener(type: 'disconnect', listener: (event: HIDConnectionEvent) => void): void;
}

declare global {
  interface Navigator {
    hid?: HID;
  }
}

export {};
