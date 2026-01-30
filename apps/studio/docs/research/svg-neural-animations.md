# SVG Neural Network Animation Techniques

Research document for advanced neural/electric visualization effects in SVG.

---

## 1. Branching Particle Systems (Neuron-like Forking)

### Technique: Multiple `<animateMotion>` on Shared Paths with Delays

```svg
<svg viewBox="0 0 400 200" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <!-- Main trunk path -->
    <path id="trunk" d="M50,100 Q150,100 200,100" fill="none"/>

    <!-- Branch paths (fork from trunk endpoint) -->
    <path id="branch1" d="M200,100 Q250,60 300,40" fill="none"/>
    <path id="branch2" d="M200,100 Q250,100 300,100" fill="none"/>
    <path id="branch3" d="M200,100 Q250,140 300,160" fill="none"/>

    <!-- Glowing particle -->
    <radialGradient id="particleGlow">
      <stop offset="0%" stop-color="#00d4ff" stop-opacity="1"/>
      <stop offset="50%" stop-color="#00d4ff" stop-opacity="0.5"/>
      <stop offset="100%" stop-color="#00d4ff" stop-opacity="0"/>
    </radialGradient>
  </defs>

  <!-- Main particle travels trunk, then splits into 3 -->
  <circle r="6" fill="url(#particleGlow)">
    <animateMotion dur="0.5s" repeatCount="indefinite" begin="0s;branch1Anim.end">
      <mpath href="#trunk"/>
    </animateMotion>
  </circle>

  <!-- Branch particles (spawn at fork point) -->
  <circle r="4" fill="url(#particleGlow)">
    <animateMotion id="branch1Anim" dur="0.4s" begin="0.5s" repeatCount="indefinite">
      <mpath href="#branch1"/>
    </animateMotion>
  </circle>

  <circle r="4" fill="url(#particleGlow)">
    <animateMotion dur="0.4s" begin="0.5s" repeatCount="indefinite">
      <mpath href="#branch2"/>
    </animateMotion>
  </circle>

  <circle r="4" fill="url(#particleGlow)">
    <animateMotion dur="0.4s" begin="0.5s" repeatCount="indefinite">
      <mpath href="#branch3"/>
    </animateMotion>
  </circle>
</svg>
```

### React Component Pattern

```tsx
interface BranchingParticle {
  trunkPath: string;
  branches: string[];
  trunkDuration: number;
  branchDuration: number;
  color: string;
}

const BranchingParticleSystem: React.FC<BranchingParticle> = ({
  trunkPath,
  branches,
  trunkDuration,
  branchDuration,
  color
}) => {
  const trunkId = useId();

  return (
    <g className="branching-particle-system">
      <defs>
        <path id={`trunk-${trunkId}`} d={trunkPath} fill="none"/>
        {branches.map((branch, i) => (
          <path key={i} id={`branch-${trunkId}-${i}`} d={branch} fill="none"/>
        ))}
        <radialGradient id={`glow-${trunkId}`}>
          <stop offset="0%" stopColor={color} stopOpacity="1"/>
          <stop offset="100%" stopColor={color} stopOpacity="0"/>
        </radialGradient>
      </defs>

      {/* Main particle */}
      <circle r="6" fill={`url(#glow-${trunkId})`}>
        <animateMotion
          dur={`${trunkDuration}s`}
          repeatCount="indefinite"
        >
          <mpath href={`#trunk-${trunkId}`}/>
        </animateMotion>
      </circle>

      {/* Branch particles */}
      {branches.map((_, i) => (
        <circle key={i} r="4" fill={`url(#glow-${trunkId})`} opacity="0">
          <animate
            attributeName="opacity"
            values="0;1;1;0"
            dur={`${branchDuration}s`}
            begin={`${trunkDuration}s`}
            repeatCount="indefinite"
          />
          <animateMotion
            dur={`${branchDuration}s`}
            begin={`${trunkDuration}s`}
            repeatCount="indefinite"
          >
            <mpath href={`#branch-${trunkId}-${i}`}/>
          </animateMotion>
        </circle>
      ))}
    </g>
  );
};
```

---

## 2. Electric Arc / Lightning Effects

### Technique: Animated Jagged Polylines with Glow Filters

```svg
<svg viewBox="0 0 400 200" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <!-- Electric glow filter -->
    <filter id="electricGlow" x="-50%" y="-50%" width="200%" height="200%">
      <feGaussianBlur in="SourceGraphic" stdDeviation="3" result="blur"/>
      <feColorMatrix in="blur" type="matrix"
        values="0 0 0 0 0
                0 0 0 0 0.8
                0 0 0 0 1
                0 0 0 1.5 0"/>
      <feMerge>
        <feMergeNode/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>

    <!-- Flicker animation via CSS or SMIL -->
    <style>
      .lightning {
        stroke: #00d4ff;
        stroke-width: 2;
        fill: none;
        filter: url(#electricGlow);
      }
      .lightning-flicker {
        animation: flicker 0.1s infinite;
      }
      @keyframes flicker {
        0%, 100% { opacity: 1; }
        50% { opacity: 0.3; }
        75% { opacity: 0.8; }
      }
    </style>
  </defs>

  <!-- Lightning bolt with jagged path -->
  <polyline class="lightning lightning-flicker"
    points="50,100 80,95 100,105 130,90 160,110 180,85 200,100"/>

  <!-- Animate path points for dynamic lightning -->
  <polyline class="lightning" id="dynamicLightning">
    <animate
      attributeName="points"
      dur="0.15s"
      repeatCount="indefinite"
      values="50,100 80,92 100,108 130,88 160,112 180,82 200,100;
              50,100 80,98 100,102 130,92 160,108 180,88 200,100;
              50,100 80,95 100,105 130,90 160,110 180,85 200,100"
    />
  </polyline>
</svg>
```

### JavaScript Lightning Path Generator

```typescript
/**
 * Generate a jagged lightning path between two points
 */
function generateLightningPath(
  start: { x: number; y: number },
  end: { x: number; y: number },
  segments: number = 8,
  jitter: number = 20
): string {
  const points: { x: number; y: number }[] = [start];

  const dx = (end.x - start.x) / segments;
  const dy = (end.y - start.y) / segments;

  for (let i = 1; i < segments; i++) {
    const baseX = start.x + dx * i;
    const baseY = start.y + dy * i;

    // Add perpendicular jitter
    const angle = Math.atan2(dy, dx) + Math.PI / 2;
    const offset = (Math.random() - 0.5) * 2 * jitter;

    points.push({
      x: baseX + Math.cos(angle) * offset,
      y: baseY + Math.sin(angle) * offset
    });
  }

  points.push(end);

  return `M${points.map(p => `${p.x},${p.y}`).join(' L')}`;
}

/**
 * Generate multiple lightning branches
 */
function generateLightningWithBranches(
  start: { x: number; y: number },
  end: { x: number; y: number },
  branchProbability: number = 0.3,
  branchLength: number = 30
): string[] {
  const mainPath = generateLightningPath(start, end, 10, 15);
  const paths = [mainPath];

  // Extract points from main path for branch origins
  const mainPoints = mainPath
    .replace('M', '')
    .split(' L')
    .map(p => {
      const [x, y] = p.split(',').map(Number);
      return { x, y };
    });

  // Add branches at random points
  mainPoints.slice(2, -2).forEach((point, i) => {
    if (Math.random() < branchProbability) {
      const angle = (Math.random() - 0.5) * Math.PI;
      const branchEnd = {
        x: point.x + Math.cos(angle) * branchLength,
        y: point.y + Math.sin(angle) * branchLength
      };
      paths.push(generateLightningPath(point, branchEnd, 4, 8));
    }
  });

  return paths;
}
```

---

## 3. Zigzag / Jagged Path Animations

### Technique: stroke-dasharray + stroke-dashoffset Animation

```svg
<svg viewBox="0 0 400 100" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <!-- Zigzag pattern path -->
    <path id="zigzag"
      d="M0,50 L20,30 L40,70 L60,30 L80,70 L100,30 L120,70 L140,30 L160,70 L180,30 L200,50"
      fill="none" stroke="#00d4ff" stroke-width="2"/>

    <filter id="glow">
      <feGaussianBlur stdDeviation="2" result="coloredBlur"/>
      <feMerge>
        <feMergeNode in="coloredBlur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
  </defs>

  <!-- Animated zigzag line draw -->
  <use href="#zigzag" filter="url(#glow)">
    <animate
      attributeName="stroke-dasharray"
      from="0 1000"
      to="1000 0"
      dur="1s"
      repeatCount="indefinite"
    />
  </use>

  <!-- Alternative: traveling pulse on zigzag -->
  <use href="#zigzag" stroke="#00d4ff" stroke-opacity="0.3"/>
  <use href="#zigzag" filter="url(#glow)"
    stroke-dasharray="20 180"
    stroke-linecap="round">
    <animate
      attributeName="stroke-dashoffset"
      from="0"
      to="-200"
      dur="0.5s"
      repeatCount="indefinite"
    />
  </use>
</svg>
```

### React Component: Animated Zigzag Edge

```tsx
interface ZigzagEdgeProps {
  startX: number;
  startY: number;
  endX: number;
  endY: number;
  amplitude?: number;
  frequency?: number;
  color?: string;
  animated?: boolean;
}

const ZigzagEdge: React.FC<ZigzagEdgeProps> = ({
  startX,
  startY,
  endX,
  endY,
  amplitude = 10,
  frequency = 8,
  color = '#00d4ff',
  animated = true
}) => {
  const pathId = useId();

  // Generate zigzag path
  const generateZigzag = () => {
    const dx = endX - startX;
    const dy = endY - startY;
    const length = Math.sqrt(dx * dx + dy * dy);
    const angle = Math.atan2(dy, dx);

    const segmentLength = length / frequency;
    const points: string[] = [`M${startX},${startY}`];

    for (let i = 1; i <= frequency; i++) {
      const progress = i / frequency;
      const baseX = startX + dx * progress;
      const baseY = startY + dy * progress;

      // Perpendicular offset (alternating)
      const perpAngle = angle + Math.PI / 2;
      const offset = i < frequency ? (i % 2 === 0 ? amplitude : -amplitude) : 0;

      const x = baseX + Math.cos(perpAngle) * offset;
      const y = baseY + Math.sin(perpAngle) * offset;

      points.push(`L${x},${y}`);
    }

    return points.join(' ');
  };

  const path = generateZigzag();

  return (
    <g>
      <defs>
        <filter id={`glow-${pathId}`}>
          <feGaussianBlur stdDeviation="2" result="blur"/>
          <feMerge>
            <feMergeNode in="blur"/>
            <feMergeNode in="SourceGraphic"/>
          </feMerge>
        </filter>
      </defs>

      {/* Base path (dim) */}
      <path d={path} fill="none" stroke={color} strokeOpacity="0.2" strokeWidth="1"/>

      {/* Animated pulse */}
      {animated && (
        <path
          d={path}
          fill="none"
          stroke={color}
          strokeWidth="2"
          strokeLinecap="round"
          strokeDasharray="15 85"
          filter={`url(#glow-${pathId})`}
        >
          <animate
            attributeName="stroke-dashoffset"
            from="0"
            to="-100"
            dur="0.8s"
            repeatCount="indefinite"
          />
        </path>
      )}
    </g>
  );
};
```

---

## 4. Multiple Simultaneous Particle Streams

### Technique: Staggered Animation Start Times

```svg
<svg viewBox="0 0 400 100" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <path id="mainPath" d="M50,50 Q150,20 250,50 T400,50" fill="none"/>

    <radialGradient id="particleGradient">
      <stop offset="0%" stop-color="#00d4ff"/>
      <stop offset="100%" stop-color="#00d4ff" stop-opacity="0"/>
    </radialGradient>
  </defs>

  <!-- Multiple particles with staggered starts -->
  <circle r="5" fill="url(#particleGradient)">
    <animateMotion dur="2s" repeatCount="indefinite" begin="0s">
      <mpath href="#mainPath"/>
    </animateMotion>
    <animate attributeName="r" values="5;3;5" dur="2s" repeatCount="indefinite"/>
  </circle>

  <circle r="4" fill="url(#particleGradient)">
    <animateMotion dur="2s" repeatCount="indefinite" begin="0.4s">
      <mpath href="#mainPath"/>
    </animateMotion>
  </circle>

  <circle r="3" fill="url(#particleGradient)">
    <animateMotion dur="2s" repeatCount="indefinite" begin="0.8s">
      <mpath href="#mainPath"/>
    </animateMotion>
  </circle>

  <circle r="4" fill="url(#particleGradient)">
    <animateMotion dur="2s" repeatCount="indefinite" begin="1.2s">
      <mpath href="#mainPath"/>
    </animateMotion>
  </circle>

  <circle r="3" fill="url(#particleGradient)">
    <animateMotion dur="2s" repeatCount="indefinite" begin="1.6s">
      <mpath href="#mainPath"/>
    </animateMotion>
  </circle>
</svg>
```

### React Component: Multi-Stream Particles

```tsx
interface ParticleStreamProps {
  path: string;
  particleCount?: number;
  duration?: number;
  color?: string;
  baseSize?: number;
  sizeVariation?: number;
}

const ParticleStream: React.FC<ParticleStreamProps> = ({
  path,
  particleCount = 5,
  duration = 2,
  color = '#00d4ff',
  baseSize = 4,
  sizeVariation = 2
}) => {
  const pathId = useId();
  const staggerDelay = duration / particleCount;

  return (
    <g className="particle-stream">
      <defs>
        <path id={`stream-path-${pathId}`} d={path} fill="none"/>
        <radialGradient id={`stream-gradient-${pathId}`}>
          <stop offset="0%" stopColor={color} stopOpacity="1"/>
          <stop offset="70%" stopColor={color} stopOpacity="0.4"/>
          <stop offset="100%" stopColor={color} stopOpacity="0"/>
        </radialGradient>
      </defs>

      {/* Trail/wake effect */}
      <path
        d={path}
        fill="none"
        stroke={color}
        strokeWidth="1"
        strokeOpacity="0.15"
      />

      {/* Particles */}
      {Array.from({ length: particleCount }).map((_, i) => {
        const size = baseSize + (Math.random() - 0.5) * sizeVariation;
        const delay = i * staggerDelay;

        return (
          <circle
            key={i}
            r={size}
            fill={`url(#stream-gradient-${pathId})`}
          >
            <animateMotion
              dur={`${duration}s`}
              repeatCount="indefinite"
              begin={`${delay}s`}
            >
              <mpath href={`#stream-path-${pathId}`}/>
            </animateMotion>
            {/* Pulsing size */}
            <animate
              attributeName="r"
              values={`${size};${size * 0.6};${size}`}
              dur={`${duration * 0.3}s`}
              repeatCount="indefinite"
              begin={`${delay}s`}
            />
          </circle>
        );
      })}
    </g>
  );
};
```

---

## 5. Synaptic Burst Effects at Connection Points

### Technique: Radial Burst with Scale + Opacity Animation

```svg
<svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <!-- Burst gradient -->
    <radialGradient id="burstGradient">
      <stop offset="0%" stop-color="#00d4ff" stop-opacity="1"/>
      <stop offset="50%" stop-color="#00d4ff" stop-opacity="0.5"/>
      <stop offset="100%" stop-color="#00d4ff" stop-opacity="0"/>
    </radialGradient>

    <!-- Glow filter for extra pop -->
    <filter id="burstGlow">
      <feGaussianBlur stdDeviation="4" result="blur"/>
      <feMerge>
        <feMergeNode in="blur"/>
        <feMergeNode in="SourceGraphic"/>
      </feMerge>
    </filter>
  </defs>

  <!-- Synapse node (connection point) -->
  <circle cx="100" cy="100" r="8" fill="#00d4ff"/>

  <!-- Burst rings (expanding circles) -->
  <g filter="url(#burstGlow)">
    <!-- Ring 1 -->
    <circle cx="100" cy="100" fill="none" stroke="#00d4ff" stroke-width="2">
      <animate attributeName="r" values="8;40;40" dur="1s" repeatCount="indefinite"/>
      <animate attributeName="stroke-opacity" values="1;0;0" dur="1s" repeatCount="indefinite"/>
      <animate attributeName="stroke-width" values="3;1;1" dur="1s" repeatCount="indefinite"/>
    </circle>

    <!-- Ring 2 (delayed) -->
    <circle cx="100" cy="100" fill="none" stroke="#00d4ff" stroke-width="2">
      <animate attributeName="r" values="8;40;40" dur="1s" begin="0.3s" repeatCount="indefinite"/>
      <animate attributeName="stroke-opacity" values="0.8;0;0" dur="1s" begin="0.3s" repeatCount="indefinite"/>
    </circle>

    <!-- Ring 3 (more delayed) -->
    <circle cx="100" cy="100" fill="none" stroke="#00d4ff" stroke-width="2">
      <animate attributeName="r" values="8;40;40" dur="1s" begin="0.6s" repeatCount="indefinite"/>
      <animate attributeName="stroke-opacity" values="0.6;0;0" dur="1s" begin="0.6s" repeatCount="indefinite"/>
    </circle>
  </g>

  <!-- Radial spikes (optional) -->
  <g stroke="#00d4ff" stroke-width="1" fill="none">
    <line x1="100" y1="100" x2="100" y2="60">
      <animate attributeName="y2" values="100;50;100" dur="0.5s" repeatCount="indefinite"/>
      <animate attributeName="stroke-opacity" values="0;1;0" dur="0.5s" repeatCount="indefinite"/>
    </line>
    <line x1="100" y1="100" x2="140" y2="100" transform="rotate(45 100 100)">
      <animate attributeName="x2" values="100;150;100" dur="0.5s" begin="0.1s" repeatCount="indefinite"/>
      <animate attributeName="stroke-opacity" values="0;1;0" dur="0.5s" begin="0.1s" repeatCount="indefinite"/>
    </line>
    <!-- ... more spikes at different angles -->
  </g>
</svg>
```

### React Component: Synaptic Burst

```tsx
interface SynapticBurstProps {
  x: number;
  y: number;
  color?: string;
  size?: number;
  ringCount?: number;
  duration?: number;
  active?: boolean;
}

const SynapticBurst: React.FC<SynapticBurstProps> = ({
  x,
  y,
  color = '#00d4ff',
  size = 40,
  ringCount = 3,
  duration = 1,
  active = true
}) => {
  const burstId = useId();
  const ringDelay = duration / ringCount;

  if (!active) return null;

  return (
    <g className="synaptic-burst">
      <defs>
        <filter id={`burst-glow-${burstId}`}>
          <feGaussianBlur stdDeviation="3" result="blur"/>
          <feMerge>
            <feMergeNode in="blur"/>
            <feMergeNode in="SourceGraphic"/>
          </feMerge>
        </filter>
      </defs>

      {/* Core node */}
      <circle
        cx={x}
        cy={y}
        r={size * 0.2}
        fill={color}
      >
        <animate
          attributeName="r"
          values={`${size * 0.2};${size * 0.25};${size * 0.2}`}
          dur={`${duration * 0.5}s`}
          repeatCount="indefinite"
        />
      </circle>

      {/* Expanding rings */}
      <g filter={`url(#burst-glow-${burstId})`}>
        {Array.from({ length: ringCount }).map((_, i) => (
          <circle
            key={i}
            cx={x}
            cy={y}
            fill="none"
            stroke={color}
          >
            <animate
              attributeName="r"
              values={`${size * 0.2};${size};${size}`}
              dur={`${duration}s`}
              begin={`${i * ringDelay}s`}
              repeatCount="indefinite"
            />
            <animate
              attributeName="stroke-opacity"
              values={`${1 - i * 0.2};0;0`}
              dur={`${duration}s`}
              begin={`${i * ringDelay}s`}
              repeatCount="indefinite"
            />
            <animate
              attributeName="stroke-width"
              values="3;1;1"
              dur={`${duration}s`}
              begin={`${i * ringDelay}s`}
              repeatCount="indefinite"
            />
          </circle>
        ))}
      </g>
    </g>
  );
};
```

---

## 6. Combined Neural Edge with All Effects

### Complete React Component

```tsx
interface NeuralEdgeProps {
  sourceX: number;
  sourceY: number;
  targetX: number;
  targetY: number;
  color?: string;
  intensity?: 'low' | 'medium' | 'high';
  style?: 'smooth' | 'zigzag' | 'lightning';
}

const NeuralEdge: React.FC<NeuralEdgeProps> = ({
  sourceX,
  sourceY,
  targetX,
  targetY,
  color = '#00d4ff',
  intensity = 'medium',
  style = 'smooth'
}) => {
  const edgeId = useId();

  // Intensity settings
  const intensityConfig = {
    low: { particles: 2, duration: 3, burstSize: 20 },
    medium: { particles: 4, duration: 2, burstSize: 30 },
    high: { particles: 6, duration: 1, burstSize: 40 }
  };

  const config = intensityConfig[intensity];

  // Generate path based on style
  const generatePath = () => {
    const midX = (sourceX + targetX) / 2;
    const midY = (sourceY + targetY) / 2;

    switch (style) {
      case 'zigzag':
        return generateZigzagPath(
          { x: sourceX, y: sourceY },
          { x: targetX, y: targetY },
          8
        );
      case 'lightning':
        return generateLightningPath(
          { x: sourceX, y: sourceY },
          { x: targetX, y: targetY }
        );
      default:
        // Smooth bezier curve
        return `M${sourceX},${sourceY} Q${midX},${midY - 30} ${targetX},${targetY}`;
    }
  };

  const path = generatePath();

  return (
    <g className="neural-edge">
      <defs>
        <path id={`edge-path-${edgeId}`} d={path} fill="none"/>

        <radialGradient id={`particle-${edgeId}`}>
          <stop offset="0%" stopColor={color} stopOpacity="1"/>
          <stop offset="100%" stopColor={color} stopOpacity="0"/>
        </radialGradient>

        <filter id={`glow-${edgeId}`}>
          <feGaussianBlur stdDeviation="3"/>
          <feMerge>
            <feMergeNode/>
            <feMergeNode in="SourceGraphic"/>
          </feMerge>
        </filter>
      </defs>

      {/* Base edge (dim trail) */}
      <path
        d={path}
        fill="none"
        stroke={color}
        strokeWidth="1"
        strokeOpacity="0.15"
      />

      {/* Animated particles */}
      {Array.from({ length: config.particles }).map((_, i) => (
        <circle
          key={i}
          r={4 - i * 0.5}
          fill={`url(#particle-${edgeId})`}
          filter={`url(#glow-${edgeId})`}
        >
          <animateMotion
            dur={`${config.duration}s`}
            repeatCount="indefinite"
            begin={`${(i / config.particles) * config.duration}s`}
          >
            <mpath href={`#edge-path-${edgeId}`}/>
          </animateMotion>
        </circle>
      ))}

      {/* Source burst */}
      <SynapticBurst
        x={sourceX}
        y={sourceY}
        color={color}
        size={config.burstSize * 0.5}
        duration={config.duration}
      />

      {/* Target burst (on particle arrival) */}
      <SynapticBurst
        x={targetX}
        y={targetY}
        color={color}
        size={config.burstSize}
        duration={config.duration}
      />
    </g>
  );
};

// Helper functions
function generateZigzagPath(
  start: { x: number; y: number },
  end: { x: number; y: number },
  segments: number
): string {
  const dx = (end.x - start.x) / segments;
  const dy = (end.y - start.y) / segments;
  const amplitude = 10;

  const points = [`M${start.x},${start.y}`];

  for (let i = 1; i <= segments; i++) {
    const x = start.x + dx * i;
    const y = start.y + dy * i;
    const offset = i < segments ? (i % 2 === 0 ? amplitude : -amplitude) : 0;

    // Perpendicular offset
    const angle = Math.atan2(dy, dx) + Math.PI / 2;
    const offsetX = Math.cos(angle) * offset;
    const offsetY = Math.sin(angle) * offset;

    points.push(`L${x + offsetX},${y + offsetY}`);
  }

  return points.join(' ');
}

function generateLightningPath(
  start: { x: number; y: number },
  end: { x: number; y: number }
): string {
  const segments = 10;
  const jitter = 15;
  const dx = (end.x - start.x) / segments;
  const dy = (end.y - start.y) / segments;

  const points = [`M${start.x},${start.y}`];

  for (let i = 1; i < segments; i++) {
    const x = start.x + dx * i + (Math.random() - 0.5) * jitter;
    const y = start.y + dy * i + (Math.random() - 0.5) * jitter;
    points.push(`L${x},${y}`);
  }

  points.push(`L${end.x},${end.y}`);
  return points.join(' ');
}
```

---

## 7. Performance Best Practices

### CSS vs SMIL vs JavaScript

| Approach | Pros | Cons | Use When |
|----------|------|------|----------|
| **CSS** | GPU accelerated, easy | Limited to transforms/opacity | Simple transitions |
| **SMIL** | Path morphing, built-in | Deprecated in some browsers | SVG-specific animations |
| **JavaScript** | Full control, complex logic | Manual RAF management | Dynamic/interactive |

### Performance Tips

1. **Use `will-change`** for animated elements:
```css
.animated-particle {
  will-change: transform, opacity;
}
```

2. **Prefer `transform` over position changes**:
```css
/* Good - GPU accelerated */
transform: translateX(100px);

/* Bad - triggers layout */
left: 100px;
```

3. **Batch SVG updates** in React:
```tsx
// Use refs to batch DOM updates
const particlesRef = useRef<SVGGElement>(null);

useEffect(() => {
  // Single RAF for all particles
  const animate = () => {
    particles.forEach((p, i) => {
      // Update transforms in batch
    });
    requestAnimationFrame(animate);
  };
  requestAnimationFrame(animate);
}, []);
```

4. **Limit filter usage** - blur is expensive:
```svg
<!-- Use smaller blur radius -->
<feGaussianBlur stdDeviation="2"/>  <!-- Good -->
<feGaussianBlur stdDeviation="10"/> <!-- Expensive -->
```

5. **Use `<use>` for repeated elements**:
```svg
<defs>
  <circle id="particle" r="4" fill="#00d4ff"/>
</defs>
<use href="#particle" x="10" y="10"/>
<use href="#particle" x="20" y="20"/>
```

---

## 8. CSS Keyframe Alternatives (for globals.css)

```css
/* ============================================================================
   NEURAL EDGE ANIMATIONS
   ============================================================================ */

/* Electric flicker effect */
@keyframes electric-flicker {
  0%, 100% { opacity: 1; }
  10% { opacity: 0.8; }
  20% { opacity: 1; }
  30% { opacity: 0.4; }
  40% { opacity: 1; }
  50% { opacity: 0.9; }
  60% { opacity: 0.2; }
  70% { opacity: 1; }
  80% { opacity: 0.7; }
  90% { opacity: 1; }
}

.animate-electric-flicker {
  animation: electric-flicker 0.15s infinite;
}

/* Synaptic burst expanding rings */
@keyframes synaptic-burst {
  0% {
    r: 5;
    stroke-opacity: 1;
    stroke-width: 3;
  }
  100% {
    r: 40;
    stroke-opacity: 0;
    stroke-width: 1;
  }
}

.animate-synaptic-burst {
  animation: synaptic-burst 1s ease-out infinite;
}

/* Particle pulse (size variation) */
@keyframes particle-pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(0.7);
    opacity: 0.6;
  }
}

.animate-particle-pulse {
  animation: particle-pulse 0.8s ease-in-out infinite;
}

/* Neural glow breathing */
@keyframes neural-glow {
  0%, 100% {
    filter: drop-shadow(0 0 4px var(--glow-color, #00d4ff));
  }
  50% {
    filter: drop-shadow(0 0 12px var(--glow-color, #00d4ff));
  }
}

.animate-neural-glow {
  animation: neural-glow 2s ease-in-out infinite;
}

/* Traveling spark on edge */
@keyframes traveling-spark {
  0% {
    stroke-dashoffset: 100%;
  }
  100% {
    stroke-dashoffset: 0%;
  }
}

.animate-traveling-spark {
  stroke-dasharray: 10 90;
  animation: traveling-spark 1.5s linear infinite;
}
```

---

## References

- SVG Animation Spec: https://www.w3.org/TR/SVG11/animate.html
- SMIL Animation: https://developer.mozilla.org/en-US/docs/Web/SVG/SVG_animation_with_SMIL
- CSS vs SMIL Performance: https://css-tricks.com/weighing-svg-animation-techniques-benchmarks/
- React Flow Custom Edges: https://reactflow.dev/docs/examples/edges/custom-edge/
