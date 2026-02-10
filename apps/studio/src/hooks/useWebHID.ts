'use client';

/**
 * useWebHID - Hook for WebHID device connection
 *
 * Provides connection to Work Louder keyboards via WebHID API.
 * Only works in Chrome/Edge browsers with HTTPS or localhost.
 *
 * Based on WebHID API spec: https://wicg.github.io/webhid/
 */

import { useState, useCallback, useEffect, useRef } from 'react';
import {
  WORK_LOUDER_VENDOR_ID,
  getDeviceByIds,
  type DeviceDefinition,
} from '@/config/macropad/devices';

// =============================================================================
// Types
// =============================================================================

export type ConnectionStatus =
  | 'disconnected'
  | 'requesting'
  | 'connecting'
  | 'connected'
  | 'error'
  | 'unsupported';

export interface WebHIDState {
  status: ConnectionStatus;
  device: HIDDevice | null;
  deviceInfo: DeviceDefinition | null;
  error: string | null;
  isSupported: boolean;
}

export interface UseWebHIDReturn extends WebHIDState {
  connect: () => Promise<boolean>;
  disconnect: () => Promise<void>;
  sendReport: (reportId: number, data: Uint8Array) => Promise<boolean>;
}

// =============================================================================
// VIA Protocol Constants
// =============================================================================

// VIA protocol uses these report IDs
const VIA_REPORT_ID = 0x00;

// VIA command types
export const VIA_COMMANDS = {
  GET_PROTOCOL_VERSION: 0x01,
  GET_KEYBOARD_VALUE: 0x02,
  SET_KEYBOARD_VALUE: 0x03,
  DYNAMIC_KEYMAP_GET_KEYCODE: 0x04,
  DYNAMIC_KEYMAP_SET_KEYCODE: 0x05,
  DYNAMIC_KEYMAP_RESET: 0x06,
  LIGHTING_SET_VALUE: 0x07,
  LIGHTING_GET_VALUE: 0x08,
  LIGHTING_SAVE: 0x09,
  EEPROM_RESET: 0x0a,
  BOOTLOADER_JUMP: 0x0b,
  DYNAMIC_KEYMAP_GET_LAYER_COUNT: 0x11,
  DYNAMIC_KEYMAP_GET_BUFFER: 0x12,
  DYNAMIC_KEYMAP_SET_BUFFER: 0x13,
} as const;

// =============================================================================
// Hook Implementation
// =============================================================================

export function useWebHID(): UseWebHIDReturn {
  const [status, setStatus] = useState<ConnectionStatus>('disconnected');
  const [device, setDevice] = useState<HIDDevice | null>(null);
  const [deviceInfo, setDeviceInfo] = useState<DeviceDefinition | null>(null);
  const [error, setError] = useState<string | null>(null);

  const deviceRef = useRef<HIDDevice | null>(null);

  // Check WebHID support
  const isSupported = typeof navigator !== 'undefined' && 'hid' in navigator;

  // Handle device connection events
  useEffect(() => {
    if (!isSupported) {
      setStatus('unsupported');
      return;
    }

    const handleConnect = (event: HIDConnectionEvent) => {
      console.log('[WebHID] Device connected:', event.device.productName);
      // Auto-reconnect if it's a Work Louder device
      if (event.device.vendorId === WORK_LOUDER_VENDOR_ID) {
        openDevice(event.device);
      }
    };

    const handleDisconnect = (event: HIDConnectionEvent) => {
      console.log('[WebHID] Device disconnected:', event.device.productName);
      if (deviceRef.current === event.device) {
        setDevice(null);
        setDeviceInfo(null);
        setStatus('disconnected');
        deviceRef.current = null;
      }
    };

    navigator.hid.addEventListener('connect', handleConnect);
    navigator.hid.addEventListener('disconnect', handleDisconnect);

    // Check for already-connected devices
    navigator.hid.getDevices().then((devices) => {
      const workLouderDevice = devices.find(
        (d) => d.vendorId === WORK_LOUDER_VENDOR_ID
      );
      if (workLouderDevice) {
        openDevice(workLouderDevice);
      }
    });

    return () => {
      navigator.hid.removeEventListener('connect', handleConnect);
      navigator.hid.removeEventListener('disconnect', handleDisconnect);
    };
  }, [isSupported]);

  // Open and setup device
  const openDevice = useCallback(async (hidDevice: HIDDevice) => {
    try {
      setStatus('connecting');

      if (!hidDevice.opened) {
        await hidDevice.open();
      }

      // Get device info
      const info = getDeviceByIds(hidDevice.vendorId, hidDevice.productId);

      // Setup input report listener
      hidDevice.addEventListener('inputreport', (event) => {
        const data = new Uint8Array(event.data.buffer);
        console.log('[WebHID] Input report:', event.reportId, data);
      });

      deviceRef.current = hidDevice;
      setDevice(hidDevice);
      setDeviceInfo(info);
      setStatus('connected');
      setError(null);

      console.log('[WebHID] Device opened:', hidDevice.productName);
      return true;
    } catch (err) {
      console.error('[WebHID] Failed to open device:', err);
      setError(err instanceof Error ? err.message : 'Failed to open device');
      setStatus('error');
      return false;
    }
  }, []);

  // Request device connection
  const connect = useCallback(async (): Promise<boolean> => {
    if (!isSupported) {
      setError('WebHID is not supported in this browser');
      setStatus('unsupported');
      return false;
    }

    try {
      setStatus('requesting');
      setError(null);

      // Request device with Work Louder vendor ID filter
      const devices = await navigator.hid.requestDevice({
        filters: [{ vendorId: WORK_LOUDER_VENDOR_ID }],
      });

      if (devices.length === 0) {
        setStatus('disconnected');
        return false;
      }

      return await openDevice(devices[0]);
    } catch (err) {
      if (err instanceof Error && err.name === 'NotFoundError') {
        // User cancelled the dialog
        setStatus('disconnected');
        return false;
      }

      console.error('[WebHID] Connection error:', err);
      setError(err instanceof Error ? err.message : 'Connection failed');
      setStatus('error');
      return false;
    }
  }, [isSupported, openDevice]);

  // Disconnect device
  const disconnect = useCallback(async (): Promise<void> => {
    if (deviceRef.current?.opened) {
      try {
        await deviceRef.current.close();
      } catch (err) {
        console.error('[WebHID] Disconnect error:', err);
      }
    }

    deviceRef.current = null;
    setDevice(null);
    setDeviceInfo(null);
    setStatus('disconnected');
    setError(null);
  }, []);

  // Send output report to device
  const sendReport = useCallback(
    async (reportId: number, data: Uint8Array): Promise<boolean> => {
      if (!deviceRef.current?.opened) {
        setError('Device not connected');
        return false;
      }

      try {
        // Cast to BufferSource for TypeScript compatibility with WebHID API
        await deviceRef.current.sendReport(reportId, data as unknown as BufferSource);
        return true;
      } catch (err) {
        console.error('[WebHID] Send report error:', err);
        setError(err instanceof Error ? err.message : 'Failed to send report');
        return false;
      }
    },
    []
  );

  return {
    status,
    device,
    deviceInfo,
    error,
    isSupported,
    connect,
    disconnect,
    sendReport,
  };
}

// =============================================================================
// VIA Protocol Helpers
// =============================================================================

/**
 * Build a VIA command packet
 */
export function buildViaCommand(command: number, ...args: number[]): Uint8Array {
  const packet = new Uint8Array(32);
  packet[0] = command;
  args.forEach((arg, i) => {
    packet[i + 1] = arg;
  });
  return packet;
}

/**
 * Get keycode at position
 */
export function buildGetKeycodeCommand(
  layer: number,
  row: number,
  col: number
): Uint8Array {
  return buildViaCommand(
    VIA_COMMANDS.DYNAMIC_KEYMAP_GET_KEYCODE,
    layer,
    row,
    col
  );
}

/**
 * Set keycode at position
 */
export function buildSetKeycodeCommand(
  layer: number,
  row: number,
  col: number,
  keycode: number
): Uint8Array {
  return buildViaCommand(
    VIA_COMMANDS.DYNAMIC_KEYMAP_SET_KEYCODE,
    layer,
    row,
    col,
    (keycode >> 8) & 0xff,
    keycode & 0xff
  );
}

export default useWebHID;
