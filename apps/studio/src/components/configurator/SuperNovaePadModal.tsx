'use client';

/**
 * SuperNovaePadModal - Modal wrapper for the 3D pad configurator
 *
 * Features:
 * - Uses existing Modal compound component
 * - Animated backdrop blur
 * - Trigger button with keyboard icon
 * - Full 3D canvas inside modal
 */

import { useState, useCallback } from 'react';
import { Keyboard } from 'lucide-react';
import { Modal } from '@/components/ui/Modal';
import { cn } from '@/lib/utils';
import { SuperNovaePad3D } from './SuperNovaePad3D';
import type { SuperNovaePadModalProps, KeyBinding } from './types';

export function SuperNovaePadModal({
  trigger,
  defaultOpen = false,
}: SuperNovaePadModalProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);

  const handleClose = useCallback(() => {
    setIsOpen(false);
  }, []);

  const handleKeyPress = useCallback((binding: KeyBinding) => {
    console.log('[SuperNovaePadModal] Key pressed:', binding);
    // TODO: Dispatch action based on binding
  }, []);

  const handleEncoderChange = useCallback((encoderId: string, delta: number) => {
    console.log('[SuperNovaePadModal] Encoder changed:', encoderId, delta);
    // TODO: Dispatch action based on encoder
  }, []);

  return (
    <>
      {/* Trigger button */}
      {trigger ?? (
        <button
          onClick={() => setIsOpen(true)}
          className={cn(
            'flex items-center gap-2 px-4 py-2',
            'bg-blue-600 hover:bg-blue-700',
            'text-white text-sm font-medium',
            'rounded-lg transition-colors',
            'focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2',
            'focus:ring-offset-slate-900'
          )}
        >
          <Keyboard className="w-4 h-4" />
          Configure Pad
        </button>
      )}

      {/* Modal */}
      <Modal.Root
        isOpen={isOpen}
        onClose={handleClose}
        closeOnOutsideClick
        closeOnEscape
      >
        <Modal.Content
          size="xl"
          className="h-[600px] bg-slate-900/95 backdrop-blur-sm"
          ariaLabel="SuperNovae Pad Configuration"
        >
          <Modal.Header titleId="pad-modal-title">
            <Modal.Title id="pad-modal-title" icon={<Keyboard className="w-4 h-4 text-blue-400" />}>
              SuperNovae Pad - NovaNet
            </Modal.Title>
          </Modal.Header>

          <Modal.Body className="p-0 h-[520px]" maxHeight="none">
            <div className="w-full h-full">
              <SuperNovaePad3D
                onKeyPress={handleKeyPress}
                onEncoderChange={handleEncoderChange}
              />
            </div>
          </Modal.Body>
        </Modal.Content>
      </Modal.Root>
    </>
  );
}
