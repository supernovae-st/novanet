'use client';

/**
 * MacropadTutorial - Expandable tutorial steps for connecting Work Louder pad
 *
 * Shows step-by-step guide with icons for:
 * 1. Check browser compatibility (Chrome/Edge)
 * 2. Connect USB cable
 * 3. Click Connect button
 * 4. Select device in browser dialog
 * 5. Start configuring!
 */

import { memo, useState } from 'react';
import {
  ChevronDown,
  ChevronRight,
  Chrome,
  Cable,
  MousePointer,
  ListChecks,
  Sparkles,
  CheckCircle2,
  Circle,
  AlertTriangle,
  ExternalLink,
} from 'lucide-react';
import { cn } from '@/lib/utils';
import type { ConnectionStatus } from '@/hooks/useWebHID';

// =============================================================================
// Types
// =============================================================================

interface TutorialStep {
  id: string;
  title: string;
  description: string;
  icon: React.ReactNode;
  details?: string[];
  link?: { text: string; url: string };
  checkFn?: () => boolean;
}

interface MacropadTutorialProps {
  connectionStatus: ConnectionStatus;
  isSupported: boolean;
  onConnect: () => void;
}

// =============================================================================
// Tutorial Steps Definition
// =============================================================================

const getTutorialSteps = (
  isSupported: boolean,
  connectionStatus: ConnectionStatus
): TutorialStep[] => [
  {
    id: 'browser',
    title: 'Use Chrome or Edge',
    description: 'WebHID requires a Chromium-based browser',
    icon: <Chrome className="w-5 h-5" />,
    details: [
      'WebHID API only works in Chrome, Edge, or Brave',
      'Safari and Firefox are not supported',
      'Must be HTTPS or localhost',
    ],
    checkFn: () => isSupported,
  },
  {
    id: 'cable',
    title: 'Connect USB Cable',
    description: 'Plug in your Work Louder keyboard',
    icon: <Cable className="w-5 h-5" />,
    details: [
      'Use the USB-C cable that came with your device',
      'Connect directly to your computer (not through a hub)',
      'The keyboard should light up when connected',
    ],
  },
  {
    id: 'connect',
    title: 'Click Connect Button',
    description: 'Request device access from your browser',
    icon: <MousePointer className="w-5 h-5" />,
    details: [
      'Click the "Connect Device" button below',
      'Your browser will show a device picker dialog',
      'This requires a user gesture (button click)',
    ],
    checkFn: () =>
      connectionStatus === 'connected' || connectionStatus === 'requesting',
  },
  {
    id: 'select',
    title: 'Select Your Device',
    description: 'Choose "Work Louder" from the list',
    icon: <ListChecks className="w-5 h-5" />,
    details: [
      'Look for "Work Louder Creator Board" or similar',
      'If you don\'t see it, check USB connection',
      'Click "Connect" in the browser dialog',
    ],
    link: {
      text: 'Troubleshooting Guide',
      url: 'https://worklouder.cc/support',
    },
    checkFn: () => connectionStatus === 'connected',
  },
  {
    id: 'configure',
    title: 'Start Configuring!',
    description: 'Edit keybindings and sync to device',
    icon: <Sparkles className="w-5 h-5" />,
    details: [
      'Click any key on the 3D pad to select it',
      'Choose a new keycode from the palette',
      'Changes sync automatically to your device',
    ],
    checkFn: () => connectionStatus === 'connected',
  },
];

// =============================================================================
// Step Component
// =============================================================================

interface StepItemProps {
  step: TutorialStep;
  index: number;
  isExpanded: boolean;
  isComplete: boolean;
  isActive: boolean;
  onToggle: () => void;
}

const StepItem = memo(function StepItem({
  step,
  index,
  isExpanded,
  isComplete,
  isActive,
  onToggle,
}: StepItemProps) {
  return (
    <div
      className={cn(
        'border rounded-xl transition-all duration-200',
        isComplete
          ? 'border-green-500/30 bg-green-500/5'
          : isActive
            ? 'border-novanet-500/40 bg-novanet-500/5'
            : 'border-white/[0.08] bg-white/[0.02]'
      )}
    >
      {/* Step Header */}
      <button
        onClick={onToggle}
        className="w-full flex items-center gap-3 p-3 text-left"
      >
        {/* Step Number / Check */}
        <div
          className={cn(
            'w-7 h-7 rounded-full flex items-center justify-center flex-shrink-0 text-xs font-medium',
            isComplete
              ? 'bg-green-500/20 text-green-400'
              : isActive
                ? 'bg-novanet-500/20 text-novanet-400'
                : 'bg-white/[0.06] text-white/40'
          )}
        >
          {isComplete ? (
            <CheckCircle2 className="w-4 h-4" />
          ) : (
            <span>{index + 1}</span>
          )}
        </div>

        {/* Icon */}
        <div
          className={cn(
            'flex-shrink-0',
            isComplete
              ? 'text-green-400'
              : isActive
                ? 'text-novanet-400'
                : 'text-white/50'
          )}
        >
          {step.icon}
        </div>

        {/* Title & Description */}
        <div className="flex-1 min-w-0">
          <p
            className={cn(
              'text-sm font-medium',
              isComplete
                ? 'text-green-400'
                : isActive
                  ? 'text-white'
                  : 'text-white/70'
            )}
          >
            {step.title}
          </p>
          <p className="text-xs text-white/40 truncate">{step.description}</p>
        </div>

        {/* Expand Arrow */}
        <div className="text-white/30">
          {isExpanded ? (
            <ChevronDown className="w-4 h-4" />
          ) : (
            <ChevronRight className="w-4 h-4" />
          )}
        </div>
      </button>

      {/* Expanded Details */}
      {isExpanded && step.details && (
        <div className="px-3 pb-3 pt-0">
          <div className="ml-10 pl-3 border-l border-white/[0.08]">
            <ul className="space-y-1.5">
              {step.details.map((detail, i) => (
                <li
                  key={i}
                  className="flex items-start gap-2 text-xs text-white/50"
                >
                  <Circle className="w-1.5 h-1.5 mt-1.5 flex-shrink-0 fill-current" />
                  <span>{detail}</span>
                </li>
              ))}
            </ul>

            {step.link && (
              <a
                href={step.link.url}
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-1 mt-2 text-xs text-novanet-400 hover:text-novanet-300 transition-colors"
              >
                {step.link.text}
                <ExternalLink className="w-3 h-3" />
              </a>
            )}
          </div>
        </div>
      )}
    </div>
  );
});

// =============================================================================
// Main Component
// =============================================================================

export const MacropadTutorial = memo(function MacropadTutorial({
  connectionStatus,
  isSupported,
  onConnect,
}: MacropadTutorialProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const [expandedSteps, setExpandedSteps] = useState<Set<string>>(new Set(['browser']));

  const steps = getTutorialSteps(isSupported, connectionStatus);

  // Find first incomplete step
  const activeStepIndex = steps.findIndex((step) => !step.checkFn?.());

  const toggleStep = (stepId: string) => {
    setExpandedSteps((prev) => {
      const next = new Set(prev);
      if (next.has(stepId)) {
        next.delete(stepId);
      } else {
        next.add(stepId);
      }
      return next;
    });
  };

  return (
    <div className="space-y-3">
      {/* Tutorial Header */}
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="w-full flex items-center justify-between p-3 bg-gradient-to-r from-novanet-500/10 to-transparent border border-novanet-500/20 rounded-xl hover:border-novanet-500/30 transition-colors"
      >
        <div className="flex items-center gap-2">
          <div className="p-1.5 bg-novanet-500/20 rounded-lg">
            <Cable className="w-4 h-4 text-novanet-400" />
          </div>
          <div className="text-left">
            <p className="text-sm font-medium text-white">Connect Your Pad</p>
            <p className="text-[11px] text-white/40">
              {connectionStatus === 'connected'
                ? 'Device connected!'
                : 'Follow these steps to connect via WebHID'}
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2">
          {/* Status Badge */}
          <div
            className={cn(
              'px-2 py-1 rounded-md text-[10px] font-medium',
              connectionStatus === 'connected'
                ? 'bg-green-500/20 text-green-400'
                : connectionStatus === 'error'
                  ? 'bg-red-500/20 text-red-400'
                  : !isSupported
                    ? 'bg-yellow-500/20 text-yellow-400'
                    : 'bg-white/[0.06] text-white/50'
            )}
          >
            {connectionStatus === 'connected' && 'Connected'}
            {connectionStatus === 'error' && 'Error'}
            {connectionStatus === 'requesting' && 'Requesting...'}
            {connectionStatus === 'connecting' && 'Connecting...'}
            {connectionStatus === 'disconnected' && 'Not Connected'}
            {connectionStatus === 'unsupported' && 'Unsupported'}
          </div>

          {isExpanded ? (
            <ChevronDown className="w-4 h-4 text-white/40" />
          ) : (
            <ChevronRight className="w-4 h-4 text-white/40" />
          )}
        </div>
      </button>

      {/* Expanded Tutorial Content */}
      {isExpanded && (
        <div className="space-y-2">
          {/* Browser Warning */}
          {!isSupported && (
            <div className="flex items-start gap-2 p-3 bg-yellow-500/10 border border-yellow-500/20 rounded-xl">
              <AlertTriangle className="w-4 h-4 text-yellow-400 flex-shrink-0 mt-0.5" />
              <div>
                <p className="text-xs font-medium text-yellow-400">
                  Browser Not Supported
                </p>
                <p className="text-[11px] text-yellow-400/70 mt-0.5">
                  Please use Chrome, Edge, or Brave to connect your device.
                </p>
              </div>
            </div>
          )}

          {/* Steps */}
          {steps.map((step, index) => (
            <StepItem
              key={step.id}
              step={step}
              index={index}
              isExpanded={expandedSteps.has(step.id)}
              isComplete={step.checkFn?.() ?? false}
              isActive={index === activeStepIndex}
              onToggle={() => toggleStep(step.id)}
            />
          ))}

          {/* Connect Button */}
          {isSupported && connectionStatus !== 'connected' && (
            <button
              onClick={onConnect}
              disabled={
                connectionStatus === 'requesting' ||
                connectionStatus === 'connecting'
              }
              className={cn(
                'w-full py-3 rounded-xl font-medium text-sm transition-all flex items-center justify-center gap-2',
                connectionStatus === 'requesting' ||
                  connectionStatus === 'connecting'
                  ? 'bg-novanet-500/30 text-novanet-300 cursor-wait'
                  : 'bg-novanet-500 text-white hover:bg-novanet-600 active:scale-[0.98]'
              )}
            >
              <Cable className="w-4 h-4" />
              {connectionStatus === 'requesting'
                ? 'Requesting Access...'
                : connectionStatus === 'connecting'
                  ? 'Connecting...'
                  : 'Connect Device'}
            </button>
          )}
        </div>
      )}
    </div>
  );
});

export default MacropadTutorial;
