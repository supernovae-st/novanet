'use client';

/**
 * MacropadVisualizer - Modal for Work Louder Creator Micro Configuration
 *
 * Features:
 * - 3D visualization of the macropad with interactive keys
 * - Layer tabs (Navigation, YAML, System)
 * - Key details panel with current binding info
 * - Syncs with ~/Projects/work-louder/studio-integration/configs/
 * - Export/Import JSON configs
 *
 * Keyboard shortcut: P
 */

import { memo, useState, useCallback, useEffect } from 'react';
import dynamic from 'next/dynamic';
import {
  X,
  Keyboard,
  Download,
  Upload,
  HelpCircle,
  RefreshCw,
  Settings2,
  Layers,
  Check,
  AlertCircle,
  FolderOpen,
  Usb,
  Unplug,
  Radio,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import { useUIStore } from '@/stores/uiStore';
import { useWebHID } from '@/hooks/useWebHID';
import { MacropadTutorial } from './MacropadTutorial';

// Dynamic import with SSR disabled for Three.js
const SuperNovaePad3D = dynamic(
  () => import('./SuperNovaePad3D').then((mod) => {
    console.log('[MacropadVisualizer] 3D component loaded successfully');
    return mod.default;
  }),
  {
    ssr: false,
    loading: () => (
      <div className="flex items-center justify-center h-full bg-[#0a0a12]">
        <div className="flex flex-col items-center gap-3">
          <div className="w-8 h-8 border-2 border-novanet-500/30 border-t-novanet-400 rounded-full animate-spin" />
          <p className="text-white/40 text-xs font-mono">Initializing Three.js...</p>
        </div>
      </div>
    ),
  }
);

// =============================================================================
// Types (matches work-louder-micro.json schema)
// =============================================================================

interface KeyBinding {
  key: string;
  label: string;
  action: string;
}

interface EncoderBinding {
  cw: KeyBinding;
  ccw: KeyBinding;
}

interface LayerConfig {
  id: number;
  name: string;
  color: string;
  description?: string;
  keys: Record<string, KeyBinding>;
  encoder?: EncoderBinding;
}

interface DeviceConfig {
  device: {
    name: string;
    vendorId: string;
    productId: string;
    matrix: { rows: number; cols: number };
    encoder: boolean;
  };
  layers: LayerConfig[];
}

// =============================================================================
// Config Path & Sync Status
// =============================================================================

const CONFIG_PATH = '~/Projects/work-louder/studio-integration/configs/work-louder-micro.json';

type SyncStatus = 'idle' | 'loading' | 'synced' | 'error' | 'modified';

// =============================================================================
// Default Configuration (matches work-louder-micro.json structure)
// =============================================================================

const DEFAULT_CONFIG: DeviceConfig = {
  device: {
    name: 'Work Louder Micro',
    vendorId: '0x574C',
    productId: '0xE6E3',
    matrix: { rows: 3, cols: 4 },
    encoder: true,
  },
  layers: [
    {
      id: 0,
      name: 'Navigation',
      color: '#00FFFF',
      description: 'Novanet TUI navigation mode',
      keys: {
        '0,0': { key: '1', label: 'Meta', action: 'MODE_META' },
        '0,1': { key: 'K', label: '↑', action: 'NAV_UP' },
        '0,2': { key: 'UP', label: '↑', action: 'ARROW_UP' },
        '0,3': { key: '2', label: 'Data', action: 'MODE_DATA' },
        '1,0': { key: 'H', label: '←', action: 'NAV_LEFT' },
        '1,1': { key: 'LEFT', label: '←', action: 'ARROW_LEFT' },
        '1,2': { key: 'SPACE', label: 'Toggle', action: 'TOGGLE' },
        '1,3': { key: 'L', label: '→', action: 'NAV_RIGHT' },
        '2,0': { key: '3', label: 'Overlay', action: 'MODE_OVERLAY' },
        '2,1': { key: 'J', label: '↓', action: 'NAV_DOWN' },
        '2,2': { key: 'DOWN', label: '↓', action: 'ARROW_DOWN' },
        '2,3': { key: '4', label: 'Query', action: 'MODE_QUERY' },
      },
      encoder: {
        cw: { key: 'K', label: 'Scroll Up', action: 'SCROLL_UP' },
        ccw: { key: 'J', label: 'Scroll Down', action: 'SCROLL_DOWN' },
      },
    },
    {
      id: 1,
      name: 'YAML & Overlays',
      color: '#9945FF',
      description: 'YAML scrolling and overlay navigation',
      keys: {
        '0,0': { key: 'g', label: 'First', action: 'GOTO_FIRST' },
        '0,1': { key: 'u', label: 'PgUp', action: 'PAGE_UP' },
        '0,2': { key: 'G', label: 'Last', action: 'GOTO_LAST' },
        '0,3': { key: 'G', label: 'Last', action: 'GOTO_LAST' },
        '1,0': { key: '[', label: '←Line', action: 'LINE_LEFT' },
        '1,1': { key: 'd', label: '½PgDn', action: 'HALF_PAGE_DOWN' },
        '1,2': { key: 'u', label: '½PgUp', action: 'HALF_PAGE_UP' },
        '1,3': { key: ']', label: '→Line', action: 'LINE_RIGHT' },
        '2,0': { key: '[', label: '←Sect', action: 'SECTION_PREV' },
        '2,1': { key: '/', label: 'Search', action: 'SEARCH' },
        '2,2': { key: '?', label: 'Help', action: 'HELP' },
        '2,3': { key: ']', label: '→Sect', action: 'SECTION_NEXT' },
      },
      encoder: {
        cw: { key: ']', label: 'Line Down', action: 'YAML_LINE_DOWN' },
        ccw: { key: '[', label: 'Line Up', action: 'YAML_LINE_UP' },
      },
    },
    {
      id: 2,
      name: 'System',
      color: '#FF4545',
      description: 'System controls and RGB',
      keys: {
        '0,0': { key: 'r', label: 'Refresh', action: 'REFRESH' },
        '0,1': { key: 'RGB_VAI', label: 'RGB+', action: 'RGB_BRIGHTNESS_UP' },
        '0,2': { key: 'q', label: 'Quit', action: 'QUIT' },
        '0,3': { key: '___', label: '', action: 'NONE' },
        '1,0': { key: 'RGB_TOG', label: 'Toggle', action: 'RGB_TOGGLE' },
        '1,1': { key: 'RGB_MOD', label: 'Mode', action: 'RGB_MODE_NEXT' },
        '1,2': { key: 'RGB_HUI', label: 'Hue', action: 'RGB_HUE_UP' },
        '1,3': { key: '___', label: '', action: 'NONE' },
        '2,0': { key: '___', label: '', action: 'NONE' },
        '2,1': { key: '___', label: '', action: 'NONE' },
        '2,2': { key: '___', label: '', action: 'NONE' },
        '2,3': { key: 'QK_BOOT', label: 'Boot', action: 'BOOTLOADER' },
      },
      encoder: {
        cw: { key: 'RGB_VAI', label: 'RGB+', action: 'RGB_BRIGHTNESS_UP' },
        ccw: { key: 'RGB_VAD', label: 'RGB-', action: 'RGB_BRIGHTNESS_DOWN' },
      },
    },
  ],
};

// =============================================================================
// Component
// =============================================================================

export const MacropadVisualizer = memo(function MacropadVisualizer() {
  const [config, setConfig] = useState<DeviceConfig>(DEFAULT_CONFIG);
  const [activeLayer, setActiveLayer] = useState(0);
  const [selectedKey, setSelectedKey] = useState<string | null>(null);
  const [syncStatus, setSyncStatus] = useState<SyncStatus>('idle');
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [_hoveredKey, setHoveredKey] = useState<string | null>(null);

  // Store integration
  const activeModal = useUIStore((s) => s.activeModal);
  const closeModal = useUIStore((s) => s.closeModal);

  // WebHID device connection
  const {
    status: connectionStatus,
    isSupported,
    connect: connectDevice,
    disconnect: disconnectDevice,
    deviceInfo,
  } = useWebHID();

  const isOpen = activeModal === 'macropad-configurator';

  // Load config on mount
  useEffect(() => {
    if (!isOpen) return;

    const loadConfig = async () => {
      setSyncStatus('loading');
      try {
        // Try to fetch from API route that reads the file
        const response = await fetch('/api/macropad/config');
        if (response.ok) {
          const data = await response.json();
          setConfig(data);
          setSyncStatus('synced');
        } else {
          // Fallback to default config
          console.log('[MacropadVisualizer] Using default config (API not available)');
          setConfig(DEFAULT_CONFIG);
          setSyncStatus('idle');
        }
      } catch (err) {
        console.log('[MacropadVisualizer] Config load error, using defaults:', err);
        setConfig(DEFAULT_CONFIG);
        setSyncStatus('idle');
      }
    };

    loadConfig();
  }, [isOpen]);

  // Keyboard shortcut (Escape to close)
  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        closeModal();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, closeModal]);

  const handleKeyClick = useCallback((keyId: string) => {
    setSelectedKey(keyId);
  }, []);

  const handleKeyHover = useCallback((keyId: string | null) => {
    setHoveredKey(keyId);
  }, []);

  const handleSaveConfig = useCallback(async () => {
    setSyncStatus('loading');
    try {
      const response = await fetch('/api/macropad/config', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(config),
      });
      if (response.ok) {
        setSyncStatus('synced');
      } else {
        setSyncStatus('error');
      }
    } catch (err) {
      console.error('[MacropadVisualizer] Save error:', err);
      setSyncStatus('error');
    }
  }, [config]);

  const handleReloadConfig = useCallback(async () => {
    setSyncStatus('loading');
    try {
      const response = await fetch('/api/macropad/config');
      if (response.ok) {
        const data = await response.json();
        setConfig(data);
        setSyncStatus('synced');
      } else {
        setSyncStatus('error');
      }
    } catch (err) {
      console.error('[MacropadVisualizer] Reload error:', err);
      setSyncStatus('error');
    }
  }, []);

  const handleExport = useCallback(() => {
    const blob = new Blob([JSON.stringify(config, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'work-louder-micro.json';
    a.click();
    URL.revokeObjectURL(url);
  }, [config]);

  const handleImport = useCallback(() => {
    const input = document.createElement('input');
    input.type = 'file';
    input.accept = '.json';
    input.onchange = (e) => {
      const file = (e.target as HTMLInputElement).files?.[0];
      if (file) {
        const reader = new FileReader();
        reader.onload = () => {
          try {
            const imported = JSON.parse(reader.result as string) as DeviceConfig;
            setConfig(imported);
            setSyncStatus('modified');
          } catch (err) {
            console.error('Failed to parse config:', err);
            setSyncStatus('error');
          }
        };
        reader.readAsText(file);
      }
    };
    input.click();
  }, []);

  // Get selected key info
  const currentLayer = config.layers[activeLayer];
  const selectedKeyInfo = selectedKey ? currentLayer?.keys[selectedKey] : null;
  const keyCount = Object.keys(currentLayer?.keys || {}).length;

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black/70 backdrop-blur-sm"
        onClick={closeModal}
      />

      {/* Modal - glass morphism */}
      <div className="relative w-[95vw] max-w-6xl h-[85vh] bg-[#0d0d12] border border-white/[0.12] rounded-2xl shadow-2xl shadow-black/60 flex flex-col overflow-hidden">
        {/* Header */}
        <div className="flex-shrink-0 flex items-center justify-between px-6 py-4 border-b border-white/[0.08]">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-novanet-500/15 rounded-xl border border-novanet-500/20">
              <Keyboard className="w-5 h-5 text-novanet-400" />
            </div>
            <div>
              <h2 className="text-lg font-semibold text-white/90">{config.device.name}</h2>
              <p className="text-xs text-white/40">NovaNet TUI Configuration</p>
            </div>
          </div>

          <div className="flex items-center gap-3">
            {/* WebHID connection status */}
            <div
              className={cn(
                'flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-medium border transition-colors',
                connectionStatus === 'connected' && 'bg-green-500/15 text-green-400 border-green-500/20',
                connectionStatus === 'connecting' && 'bg-blue-500/15 text-blue-400 border-blue-500/20',
                connectionStatus === 'requesting' && 'bg-blue-500/15 text-blue-400 border-blue-500/20',
                connectionStatus === 'disconnected' && 'bg-white/[0.06] text-white/50 border-white/[0.1]',
                connectionStatus === 'error' && 'bg-red-500/15 text-red-400 border-red-500/20',
                connectionStatus === 'unsupported' && 'bg-yellow-500/15 text-yellow-400 border-yellow-500/20'
              )}
            >
              {connectionStatus === 'connected' && <Usb className="w-3.5 h-3.5" />}
              {connectionStatus === 'connecting' && <RefreshCw className="w-3.5 h-3.5 animate-spin" />}
              {connectionStatus === 'requesting' && <RefreshCw className="w-3.5 h-3.5 animate-spin" />}
              {connectionStatus === 'disconnected' && <Unplug className="w-3.5 h-3.5" />}
              {connectionStatus === 'error' && <AlertCircle className="w-3.5 h-3.5" />}
              {connectionStatus === 'unsupported' && <AlertCircle className="w-3.5 h-3.5" />}
              <span>
                {connectionStatus === 'connected' && (deviceInfo?.name || 'Connected')}
                {connectionStatus === 'connecting' && 'Connecting...'}
                {connectionStatus === 'requesting' && 'Requesting...'}
                {connectionStatus === 'disconnected' && 'Not Connected'}
                {connectionStatus === 'error' && 'Error'}
                {connectionStatus === 'unsupported' && 'Unsupported'}
              </span>
            </div>

            {/* Sync status indicator */}
            <div
              className={cn(
                'flex items-center gap-2 px-3 py-1.5 rounded-full text-xs font-medium border transition-colors',
                syncStatus === 'synced' && 'bg-green-500/15 text-green-400 border-green-500/20',
                syncStatus === 'loading' && 'bg-blue-500/15 text-blue-400 border-blue-500/20',
                syncStatus === 'modified' && 'bg-yellow-500/15 text-yellow-400 border-yellow-500/20',
                syncStatus === 'error' && 'bg-red-500/15 text-red-400 border-red-500/20',
                syncStatus === 'idle' && 'bg-novanet-500/15 text-novanet-400 border-novanet-500/20'
              )}
            >
              {syncStatus === 'synced' && <Check className="w-3.5 h-3.5" />}
              {syncStatus === 'loading' && <RefreshCw className="w-3.5 h-3.5 animate-spin" />}
              {syncStatus === 'modified' && <AlertCircle className="w-3.5 h-3.5" />}
              {syncStatus === 'error' && <AlertCircle className="w-3.5 h-3.5" />}
              {syncStatus === 'idle' && <Radio className="w-3.5 h-3.5" />}
              <span>
                {syncStatus === 'synced' && 'Synced'}
                {syncStatus === 'loading' && 'Loading...'}
                {syncStatus === 'modified' && 'Modified'}
                {syncStatus === 'error' && 'Error'}
                {syncStatus === 'idle' && 'Preview Mode'}
              </span>
            </div>

            {/* Close button */}
            <button
              onClick={closeModal}
              className="p-2 rounded-lg hover:bg-white/[0.06] text-white/40 hover:text-white transition-colors"
            >
              <X className="w-5 h-5" />
            </button>
          </div>
        </div>

        {/* Layer Tabs */}
        <div className="flex-shrink-0 px-6 py-3 border-b border-white/[0.06] bg-black/20">
          <div className="flex gap-1 p-1 bg-white/[0.03] border border-white/[0.08] rounded-xl w-fit">
            {config.layers.map((layer) => (
              <button
                key={layer.id}
                onClick={() => setActiveLayer(layer.id)}
                className={cn(
                  'px-4 py-2 rounded-lg text-sm font-medium transition-all flex items-center gap-2',
                  activeLayer === layer.id
                    ? 'bg-white/[0.08] text-white'
                    : 'text-white/50 hover:text-white hover:bg-white/[0.04]'
                )}
              >
                <span
                  className="w-2.5 h-2.5 rounded-full"
                  style={{ backgroundColor: layer.color }}
                />
                <span>{layer.name}</span>
              </button>
            ))}
          </div>
        </div>

        {/* Main Content */}
        <div className="flex-1 flex min-h-0">
          {/* 3D Visualization */}
          <div className="flex-1 relative min-h-[400px]">
            <div className="absolute inset-0">
              <SuperNovaePad3D
                selectedKey={selectedKey}
                activeLayer={activeLayer}
                layers={config.layers}
                onKeyClick={handleKeyClick}
                onKeyHover={handleKeyHover}
              />
            </div>

            {/* Layer color indicator bar */}
            <div
              className="absolute bottom-0 left-0 right-0 h-1 z-10"
              style={{ backgroundColor: currentLayer?.color, opacity: 0.5 }}
            />
          </div>

          {/* Right Panel - Tutorial & Key Details */}
          <div className="w-80 flex-shrink-0 border-l border-white/[0.08] bg-black/30 flex flex-col">
            {/* Scrollable content */}
            <div className="flex-1 p-4 overflow-y-auto space-y-4">
              {/* Connection Tutorial */}
              <MacropadTutorial
                connectionStatus={connectionStatus}
                isSupported={isSupported}
                onConnect={connectDevice}
              />

              {/* Key Info */}
              <div className="flex items-center gap-2 mb-4">
                <Settings2 className="w-4 h-4 text-white/40" />
                <h3 className="text-sm font-medium text-white/80">Key Details</h3>
              </div>

              {selectedKeyInfo ? (
                <div className="space-y-4">
                  {/* Position */}
                  <div className="p-3 bg-white/[0.03] rounded-xl border border-white/[0.08]">
                    <label className="text-[10px] uppercase tracking-wider text-white/40">
                      Position
                    </label>
                    <p className="text-sm text-white font-mono mt-1">
                      Row {selectedKey?.split(',')[0]}, Col {selectedKey?.split(',')[1]}
                    </p>
                  </div>

                  {/* Current Binding */}
                  <div className="p-3 bg-white/[0.03] rounded-xl border border-white/[0.08]">
                    <label className="text-[10px] uppercase tracking-wider text-white/40">
                      Keycode
                    </label>
                    <p className="text-lg text-white font-mono mt-1">
                      {selectedKeyInfo.key || '—'}
                    </p>
                  </div>

                  {/* Label */}
                  <div className="p-3 bg-white/[0.03] rounded-xl border border-white/[0.08]">
                    <label className="text-[10px] uppercase tracking-wider text-white/40">
                      Label
                    </label>
                    <p className="text-sm text-white mt-1">{selectedKeyInfo.label || '—'}</p>
                  </div>

                  {/* Action */}
                  <div className="p-3 bg-white/[0.03] rounded-xl border border-white/[0.08]">
                    <label className="text-[10px] uppercase tracking-wider text-white/40">
                      Action
                    </label>
                    <p
                      className="text-sm font-mono mt-1"
                      style={{ color: currentLayer?.color }}
                    >
                      {selectedKeyInfo.action || 'NONE'}
                    </p>
                  </div>
                </div>
              ) : (
                <div className="text-center py-8">
                  <div className="w-12 h-12 mx-auto mb-3 rounded-full bg-white/[0.04] flex items-center justify-center border border-white/[0.08]">
                    <Keyboard className="w-6 h-6 text-white/30" />
                  </div>
                  <p className="text-sm text-white/50">Click a key to view details</p>
                  <p className="text-xs text-white/30 mt-1">
                    Hover for preview, click to select
                  </p>
                </div>
              )}

              {/* Encoder info */}
              {currentLayer?.encoder && (
                <div className="mt-4 p-3 bg-white/[0.03] rounded-xl border border-white/[0.08]">
                  <label className="text-[10px] uppercase tracking-wider text-white/40">
                    Encoder
                  </label>
                  <div className="mt-2 space-y-1 text-xs font-mono">
                    <p className="text-white/70">
                      ↻ CW: <span className="text-white">{currentLayer.encoder.cw.label}</span>
                    </p>
                    <p className="text-white/70">
                      ↺ CCW: <span className="text-white">{currentLayer.encoder.ccw.label}</span>
                    </p>
                  </div>
                </div>
              )}
            </div>

            {/* Layer Stats */}
            <div className="flex-shrink-0 p-4 border-t border-white/[0.06]">
              <div className="flex items-center gap-2 mb-3">
                <Layers className="w-4 h-4 text-white/40" />
                <h4 className="text-xs font-medium text-white/60">
                  Layer {activeLayer}: {currentLayer?.name}
                </h4>
              </div>
              <p className="text-xs text-white/40">{currentLayer?.description}</p>
              <div className="mt-3 grid grid-cols-2 gap-2 text-xs">
                <div className="p-2 bg-white/[0.03] rounded-lg">
                  <span className="text-white/40">Keys:</span>
                  <span className="text-white ml-1">{keyCount}</span>
                </div>
                <div className="p-2 bg-white/[0.03] rounded-lg">
                  <span className="text-white/40">Encoder:</span>
                  <span className="text-white ml-1">{config.device.encoder ? 'Yes' : 'No'}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="flex-shrink-0 flex items-center justify-between px-6 py-3 border-t border-white/[0.06] bg-black/25">
          <div className="flex items-center gap-3">
            <button className="px-3 py-1.5 rounded-lg text-sm text-white/50 hover:text-white hover:bg-white/[0.06] transition-colors flex items-center">
              <HelpCircle className="w-4 h-4 mr-1.5" />
              Help
            </button>

            {/* Config path indicator */}
            <div className="flex items-center gap-1.5 text-xs text-white/30">
              <FolderOpen className="w-3.5 h-3.5" />
              <span className="font-mono">{CONFIG_PATH}</span>
            </div>
          </div>

          <div className="flex items-center gap-2">
            {/* Disconnect button (when connected) */}
            {connectionStatus === 'connected' && (
              <button
                onClick={disconnectDevice}
                className="px-3 py-1.5 rounded-lg text-sm border border-red-500/30 text-red-400 hover:bg-red-500/10 transition-colors flex items-center"
                title="Disconnect device"
              >
                <Unplug className="w-4 h-4 mr-1.5" />
                Disconnect
              </button>
            )}

            <div className="w-px h-6 bg-white/[0.1]" />

            <button
              onClick={handleReloadConfig}
              className="px-3 py-1.5 rounded-lg text-sm border border-white/[0.15] text-white/70 hover:bg-white/[0.06] transition-colors flex items-center"
              title="Reload from file"
            >
              <RefreshCw className="w-4 h-4 mr-1.5" />
              Reload
            </button>
            <button
              onClick={handleImport}
              className="px-3 py-1.5 rounded-lg text-sm border border-white/[0.15] text-white/70 hover:bg-white/[0.06] transition-colors flex items-center"
            >
              <Upload className="w-4 h-4 mr-1.5" />
              Import
            </button>
            <button
              onClick={handleExport}
              className="px-3 py-1.5 rounded-lg text-sm border border-white/[0.15] text-white/70 hover:bg-white/[0.06] transition-colors flex items-center"
            >
              <Download className="w-4 h-4 mr-1.5" />
              Export
            </button>

            {/* Save button */}
            <button
              onClick={handleSaveConfig}
              disabled={syncStatus === 'loading'}
              className={cn(
                'px-4 py-1.5 rounded-lg text-sm font-medium transition-colors flex items-center gap-1.5',
                syncStatus === 'modified'
                  ? 'bg-novanet-500 text-white hover:bg-novanet-600'
                  : 'bg-white/[0.08] text-white/70 hover:bg-white/[0.12]'
              )}
            >
              <Check className="w-4 h-4" />
              Save
            </button>
          </div>
        </div>
      </div>
    </div>
  );
});

export default MacropadVisualizer;
