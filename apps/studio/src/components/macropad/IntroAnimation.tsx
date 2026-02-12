'use client';

/**
 * IntroAnimation - Cinematic intro for "SuperNovae Pad"
 *
 * Simple, reliable animation:
 * - Staggered letter reveal with blur
 * - Text rotates out in same direction as pad
 * - Smooth transition to pad rotation
 */

import { memo, useEffect, useState } from 'react';
import { motion, AnimatePresence, type Variants } from 'framer-motion';

interface IntroAnimationProps {
  onComplete: () => void;
  duration?: number;
}

// Container variants for staggered children
const containerVariants: Variants = {
  hidden: { opacity: 0 },
  visible: {
    opacity: 1,
    transition: {
      staggerChildren: 0.05,
      delayChildren: 0.2,
    },
  },
  exit: {
    opacity: 0,
    rotateY: 90, // Rotate out in same direction as pad will rotate
    transition: {
      duration: 0.8,
      ease: 'easeInOut',
    },
  },
};

// Letter variants - NO blur for readability
const letterVariants: Variants = {
  hidden: {
    opacity: 0,
    y: 40,
    scale: 0.8,
  },
  visible: {
    opacity: 1,
    y: 0,
    scale: 1,
    transition: {
      duration: 0.5,
      ease: [0.16, 1, 0.3, 1],
    },
  },
};

export const IntroAnimation = memo(function IntroAnimation({
  onComplete,
  duration = 3000,
}: IntroAnimationProps) {
  const [isVisible, setIsVisible] = useState(true);
  const title = 'SuperNovae Pad';

  useEffect(() => {
    // Start exit after showing the text
    const exitTimer = setTimeout(() => {
      setIsVisible(false);
    }, duration - 800);

    // Call onComplete after exit animation
    const completeTimer = setTimeout(() => {
      onComplete();
    }, duration);

    return () => {
      clearTimeout(exitTimer);
      clearTimeout(completeTimer);
    };
  }, [duration, onComplete]);

  return (
    <AnimatePresence>
      {isVisible && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          transition={{ duration: 0.6 }}
          className="absolute inset-0 z-50 flex flex-col items-center justify-center pointer-events-none"
          style={{
            background: 'radial-gradient(ellipse at center, rgba(15, 23, 42, 0.9) 0%, rgba(15, 23, 42, 0.75) 100%)',
            perspective: '1000px',
          }}
        >
          {/* Glow effect */}
          <motion.div
            initial={{ opacity: 0, scale: 0.8 }}
            animate={{ opacity: 0.6, scale: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.8 }}
            className="absolute"
            style={{
              width: 500,
              height: 150,
              background: 'radial-gradient(ellipse, rgba(0, 255, 255, 0.15) 0%, transparent 70%)',
              filter: 'blur(50px)',
            }}
          />

          {/* Main title */}
          <motion.h1
            variants={containerVariants}
            initial="hidden"
            animate="visible"
            exit="exit"
            className="text-5xl sm:text-6xl md:text-7xl font-black tracking-tight whitespace-nowrap"
            style={{
              fontFamily: 'var(--font-geist-sans), system-ui, sans-serif',
              background: 'linear-gradient(135deg, #00ffff 0%, #a855f7 50%, #3b82f6 100%)',
              WebkitBackgroundClip: 'text',
              WebkitTextFillColor: 'transparent',
              backgroundClip: 'text',
              textShadow: '0 0 60px rgba(0, 255, 255, 0.4)',
              transformStyle: 'preserve-3d',
            }}
          >
            {title.split('').map((char, index) => (
              <motion.span
                key={index}
                variants={letterVariants}
                className="inline-block"
                style={{
                  textShadow: '0 0 30px rgba(0, 255, 255, 0.5)',
                }}
              >
                {char === ' ' ? '\u00A0' : char}
              </motion.span>
            ))}
          </motion.h1>

          {/* Subtitle */}
          <motion.p
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 0.5, y: 0 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.5, delay: 0.8 }}
            className="mt-4 text-sm tracking-[0.3em] uppercase text-white/50 font-medium"
          >
            Work Louder Creator Micro
          </motion.p>
        </motion.div>
      )}
    </AnimatePresence>
  );
});

export default IntroAnimation;
