# Structure Layer Card Enhancements

**Date**: 2026-02-17
**Status**: Design Specification
**Based on**: UI research from 21st.dev component library

---

## Problem Statement

Les nodes du Structure layer (Page, Block, ContentSlot) utilisent actuellement `StructuralCardContent` qui est generique. Les Layer nodes (Output, Instructions, Structure) ont un style premium (L-corners, scanlines, badges) via `PremiumSchemaEffects` que les Class nodes n'ont pas.

**Objectif**: Unifier le style visuel tout en ajoutant des informations contextuelles specifiques a chaque type de node Structure.

---

## Research Findings

### 1. Card Variants (21st.dev)

| Variant | Use Case | Key Feature |
|---------|----------|-------------|
| `default` | Standard cards | Solid border |
| `dots` | Placeholder/empty states | `border-dashed` + dot pattern |
| `plus` | Add new item | `border-dashed` + centered + icon |
| `gradient` | Premium/featured | Gradient border + badge slot |
| `corners` | Tech/cyberpunk aesthetic | L-shape corner decorations |
| `lifted` | Depth/hierarchy | Shadow for elevation |

### 2. Status Indicators (Badge/Status Components)

```typescript
// Badge variants discovered
const badgeVariants = cva(
  'inline-flex items-center rounded-full text-xs font-medium ring-1 ring-inset',
  {
    variants: {
      variant: {
        success: 'bg-brand/10 text-brand-600 ring-brand/30',     // Published
        warning: 'bg-warning/10 text-warning-600 ring-warning/30', // Draft
        destructive: 'bg-destructive/10 text-destructive-600',     // Error
        info: 'bg-info/10 text-info-600 ring-info/30',            // Pending
        pending: 'bg-muted/50 text-muted-foreground',             // Scheduled
      },
      dot: { true: 'pl-1.5' }, // Animated dot indicator
    },
  }
);

// Animated dot indicator
<span className="relative flex h-2 w-2">
  <span className="absolute inline-flex h-full w-full animate-ping rounded-full opacity-75" />
  <span className="relative inline-flex h-2 w-2 rounded-full" />
</span>
```

### 3. Drag-Drop Ordering (@dnd-kit)

```typescript
// Sortable component pattern
<Sortable.Root
  value={items}
  onValueChange={setItems}
  orientation="vertical"
>
  <Sortable.Content>
    {items.map((item, index) => (
      <Sortable.Item key={item.id} value={item.id} asHandle>
        <div className="flex items-center gap-2">
          <GripVertical className="h-4 w-4 text-muted-foreground" />
          <span className="order-badge">{index + 1}</span>
          {item.content}
        </div>
      </Sortable.Item>
    ))}
  </Sortable.Content>
</Sortable.Root>
```

### 4. URL Preview (LinkPreview/HoverPeek)

```typescript
// Microlink API for URL screenshots
const params = encode({
  url,
  screenshot: true,
  embed: "screenshot.url",
  colorScheme: "dark",
  "viewport.width": width * 3,
  "viewport.height": height * 3,
});
const src = `https://api.microlink.io/?${params}`;
```

### 5. Tree/Hierarchy (Cards Stack)

```typescript
// Sticky card positioning for depth
<CardSticky
  index={index}
  incrementY={10}  // Vertical offset per level
  incrementZ={10}  // Z-index increment
  className="sticky"
>
  {children}
</CardSticky>
```

---

## Enhanced Card Specifications

### PageCardContent

**Purpose**: Display Page nodes with URL preview, SEO status, and block count.

```
┌─────────────────────────────────────────────┐
│ ┌─┐                              [Draft ●]  │  <- Status badge (animated dot)
│ └─┘  PAGE                                   │
├─────────────────────────────────────────────┤
│  Homepage                                   │  <- displayName
│  /qr-code-generator                         │  <- slug (with / prefix)
├─────────────────────────────────────────────┤
│  ┌─────────────────────────────────────┐    │
│  │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │    │  <- Mini wireframe preview
│  │  ▓▓▓▓▓▓▓▓▓▓   ░░░░░░░░░░░░░░░░░░░  │    │
│  │  ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░  │    │
│  └─────────────────────────────────────┘    │
├─────────────────────────────────────────────┤
│  ☰ 5 blocks   ◎ SEO: 85%   ⚡ structure    │  <- Metrics row
│  [PILLAR]                                   │  <- SEO role badge (if pillar)
└─────────────────────────────────────────────┘
```

**New Props**:
```typescript
interface PageNodeData extends StructuralNodeData {
  slug: string;
  blockCount: number;
  status: 'draft' | 'published' | 'scheduled' | 'error';
  seoScore?: number;         // 0-100
  isPillar?: boolean;        // ADR-031
  hasNative?: boolean;       // Has PageNative for current locale
}
```

**Implementation**:
```typescript
export const PageCardContent = memo(function PageCardContent({
  data,
  colors,
  selected,
  isHovered,
}: PageCardContentProps) {
  const statusConfig = {
    draft: { label: 'Draft', color: 'warning', animate: true },
    published: { label: 'Live', color: 'success', animate: false },
    scheduled: { label: 'Scheduled', color: 'info', animate: true },
    error: { label: 'Error', color: 'destructive', animate: true },
  };

  const status = statusConfig[data.status];

  return (
    <div className="px-3 py-2.5">
      {/* Header with status badge */}
      <div className="flex items-center justify-between mb-1.5">
        <div className="flex items-center gap-1.5">
          <LayerIcon layer="structure" size={20} style={{ color: colors.primary }} />
          <span className="text-[10px] font-bold uppercase tracking-wider"
            style={{ color: colors.primary }}>
            PAGE
          </span>
        </div>

        {/* Animated status badge */}
        <StatusBadge variant={status.color} animate={status.animate}>
          {status.label}
        </StatusBadge>
      </div>

      {/* Display name */}
      <h3 className="text-sm font-bold text-white truncate">{data.displayName}</h3>

      {/* Slug with URL styling */}
      <div className="flex items-center gap-1 mt-0.5">
        <Globe className="w-3 h-3 text-white/40" />
        <span className="text-[10px] font-mono text-white/60 truncate">
          /{data.slug}
        </span>
      </div>

      {/* Mini wireframe preview (optional) */}
      {selected && (
        <div className="mt-2 p-1.5 bg-black/30 rounded border border-white/10">
          <MiniWireframe blocks={data.blockCount} />
        </div>
      )}

      {/* Metrics row */}
      <div className="flex items-center gap-3 mt-2 text-[9px] text-white/50">
        <span className="flex items-center gap-1">
          <Layers className="w-3 h-3" />
          {data.blockCount} blocks
        </span>
        {data.seoScore !== undefined && (
          <span className="flex items-center gap-1">
            <TrendingUp className="w-3 h-3" />
            SEO: {data.seoScore}%
          </span>
        )}
      </div>

      {/* Pillar badge */}
      {data.isPillar && (
        <div className="mt-1.5">
          <Badge variant="outline" className="text-[8px] border-amber-500/50 text-amber-400">
            PILLAR
          </Badge>
        </div>
      )}
    </div>
  );
});
```

---

### BlockCardContent

**Purpose**: Display Block nodes with order indicator, type badge, and drag handle hint.

```
┌─────────────────────────────────────────────┐
│  ①  BLOCK              [hero]               │  <- Order number + BlockType
├─────────────────────────────────────────────┤
│  ⋮⋮  Hero Section                           │  <- Drag handle + name
│      hero-homepage-1                        │  <- Key
├─────────────────────────────────────────────┤
│  ┌───┬───┬───┐                              │
│  │ T │ D │ I │  title, description, image   │  <- Field indicators
│  └───┴───┴───┘                              │
├─────────────────────────────────────────────┤
│  [structure]        ◎ Native: fr-FR         │  <- Layer + locale status
└─────────────────────────────────────────────┘
```

**New Props**:
```typescript
interface BlockNodeData extends StructuralNodeData {
  order: number;              // Position in page (1-based)
  blockType: string;          // BlockType key (hero, pricing, etc.)
  fieldCount?: number;        // Number of fields in schema
  hasNative?: Record<string, boolean>;  // Locale -> has BlockNative
  pageKey?: string;           // Parent page reference
}
```

**Implementation**:
```typescript
export const BlockCardContent = memo(function BlockCardContent({
  data,
  colors,
  selected,
  isHovered,
}: BlockCardContentProps) {
  // Order badge with circular design
  const OrderBadge = () => (
    <div
      className="flex items-center justify-center w-5 h-5 rounded-full text-[10px] font-bold"
      style={{
        background: `${colors.primary}25`,
        border: `1.5px solid ${colors.primary}60`,
        color: colors.primary,
      }}
    >
      {data.order}
    </div>
  );

  return (
    <div className="px-3 py-2.5">
      {/* Header with order and type */}
      <div className="flex items-center justify-between mb-1.5">
        <div className="flex items-center gap-1.5">
          <OrderBadge />
          <span className="text-[10px] font-bold uppercase tracking-wider"
            style={{ color: colors.primary }}>
            BLOCK
          </span>
        </div>

        {/* BlockType badge */}
        <div
          className="px-1.5 py-0.5 rounded text-[8px] font-mono"
          style={{
            background: `${colors.secondary}20`,
            color: colors.secondary,
          }}
        >
          {data.blockType}
        </div>
      </div>

      {/* Drag handle hint + Display name */}
      <div className="flex items-center gap-1.5">
        <GripVertical
          className="w-4 h-4 text-white/30 cursor-grab"
          style={{ opacity: isHovered ? 0.6 : 0.3 }}
        />
        <h3 className="text-sm font-bold text-white truncate">{data.displayName}</h3>
      </div>

      {/* Key */}
      {data.key !== data.displayName && (
        <p className="text-[10px] font-mono truncate mt-0.5 pl-5"
          style={{ color: `${colors.primary}70` }}>
          {data.key}
        </p>
      )}

      {/* Field indicators (expanded on select) */}
      {selected && data.fieldCount && (
        <div className="mt-2 flex items-center gap-1">
          {Array.from({ length: Math.min(data.fieldCount, 6) }).map((_, i) => (
            <div
              key={i}
              className="w-4 h-4 rounded text-[8px] flex items-center justify-center"
              style={{
                background: `${colors.primary}15`,
                border: `1px solid ${colors.primary}30`,
                color: colors.primary,
              }}
            >
              {['T', 'D', 'I', 'B', 'L', 'U'][i]}
            </div>
          ))}
          {data.fieldCount > 6 && (
            <span className="text-[8px] text-white/40">+{data.fieldCount - 6}</span>
          )}
        </div>
      )}

      {/* Layer badge */}
      <div className="mt-2 flex items-center gap-2">
        <Badge variant="layer" layer="structure" size="xs" />
      </div>
    </div>
  );
});
```

---

### ContentSlotCardContent

**Purpose**: Display ContentSlot as a dashed placeholder indicating where content will be injected.

```
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                                             │
│           ┌───────────────────┐             │
│           │        +          │             │  <- Plus icon (add content)
│           └───────────────────┘             │
│                                             │
│             CONTENT SLOT                    │
│             shared-footer                   │
│                                             │
│    • • • • • • • • • • • • • • • • • •      │  <- Dot pattern
│                                             │
│       Inject: @block:footer                 │  <- @ reference preview
│                                             │
└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
```

**Design Pattern** (from 21st.dev `dots` variant):
```typescript
// Dashed border with dot pattern
const slotVariants = {
  base: [
    'relative mx-auto w-full',
    'rounded-lg border-2 border-dashed',
    'border-zinc-500/50',
    'bg-zinc-900/30',
  ],
  hover: [
    'border-primary/60',
    'bg-primary/5',
  ],
  selected: [
    'border-primary',
    'bg-primary/10',
  ],
};
```

**Implementation**:
```typescript
export const ContentSlotCardContent = memo(function ContentSlotCardContent({
  data,
  colors,
  selected,
  isHovered,
}: ContentSlotCardContentProps) {
  return (
    <div
      className={cn(
        'px-4 py-6 flex flex-col items-center justify-center',
        'border-2 border-dashed rounded-lg transition-all duration-200',
        selected ? 'border-opacity-100 bg-opacity-15' : 'border-opacity-40 bg-opacity-5',
      )}
      style={{
        borderColor: colors.primary,
        backgroundColor: `${colors.primary}${selected ? '15' : '05'}`,
      }}
    >
      {/* Plus icon circle */}
      <div
        className={cn(
          'w-10 h-10 rounded-full flex items-center justify-center mb-3',
          'border-2 border-dashed transition-all duration-200',
        )}
        style={{
          borderColor: `${colors.primary}60`,
          backgroundColor: `${colors.primary}10`,
        }}
      >
        <Plus
          className="w-5 h-5"
          style={{ color: colors.primary }}
        />
      </div>

      {/* Label */}
      <span
        className="text-[10px] font-bold uppercase tracking-wider mb-1"
        style={{ color: colors.primary }}
      >
        CONTENT SLOT
      </span>

      {/* Key */}
      <span className="text-xs font-mono text-white/70">
        {data.key}
      </span>

      {/* Dot pattern divider */}
      <div className="flex items-center gap-1 my-3 px-4">
        {Array.from({ length: 12 }).map((_, i) => (
          <div
            key={i}
            className="w-1 h-1 rounded-full"
            style={{ backgroundColor: `${colors.primary}40` }}
          />
        ))}
      </div>

      {/* @ reference preview */}
      {data.injectRef && (
        <div className="flex items-center gap-1.5 text-[10px]">
          <AtSign className="w-3 h-3" style={{ color: colors.primary }} />
          <span className="font-mono text-white/60">{data.injectRef}</span>
        </div>
      )}
    </div>
  );
});
```

---

## Pillar/Cluster Hierarchy Indicators

**Based on**: Tree component + Cards Stack from research

### Visual Hierarchy (ADR-031)

```
┌─────────────────────────────────────────────┐
│  [PILLAR]                                   │  <- Gold/amber badge
│  QR Code Generator                          │
│  └── 8 cluster pages                        │
├─────────────────────────────────────────────┤
│  ┌─ qr-instagram ─────────────────────┐     │
│  │  └─ SEO_CLUSTER_OF                 │     │  <- Cluster line
│  ├─ qr-wifi ──────────────────────────┤     │
│  └─ qr-menu ──────────────────────────┘     │
└─────────────────────────────────────────────┘
```

### Implementation (Tree Lines)

```typescript
// Vertical hierarchy lines using CSS
const HierarchyLines = ({ depth }: { depth: number }) => (
  <div
    className="absolute left-0 top-0 bottom-0"
    style={{
      width: depth * 20,
      background: `repeating-linear-gradient(
        to right,
        transparent 0,
        transparent 19px,
        var(--border) 19px,
        var(--border) 20px
      )`,
    }}
  />
);

// Depth indicator dot
const DepthIndicator = ({ depth, color }: { depth: number; color: string }) => (
  <div className="flex items-center gap-0.5">
    {Array.from({ length: depth }).map((_, i) => (
      <div
        key={i}
        className="w-1.5 h-1.5 rounded-full"
        style={{
          backgroundColor: i === depth - 1 ? color : `${color}40`,
        }}
      />
    ))}
  </div>
);
```

---

## Unified Premium Effects

Proposition: Ajouter les effets `PremiumSchemaEffects` a TOUS les nodes du graph (pas seulement les Layer nodes).

**Levels**:
1. **Schema nodes** (Realm, Layer, Class): Full effects (corners, scanlines, grid, shimmer)
2. **Structure nodes** (Page, Block): Medium effects (corners, grid)
3. **Knowledge nodes** (Term, Expression): Light effects (grid only)

```typescript
// In CardShell.tsx
export interface CardShellProps {
  // ... existing props

  /** Premium effects level */
  premiumLevel?: 'none' | 'light' | 'medium' | 'full';
}

// Usage
<CardShell
  premiumLevel="medium"  // For Page/Block
  colors={colors}
  selected={selected}
  renderContent={(ctx) => <PageCardContent data={data} {...ctx} />}
/>
```

---

## Migration Plan

### Phase 1: New Card Content Components

1. Create `PageCardContent.tsx`
2. Create `BlockCardContent.tsx`
3. Create `ContentSlotCardContent.tsx`
4. Create `StatusBadge.tsx` (shared component)
5. Create `MiniWireframe.tsx` (Page preview)

### Phase 2: Update Node Components

1. Update `StructuralNode.tsx` to use type-specific content
2. Add `premiumLevel` prop to `CardShell`
3. Integrate `PremiumSchemaEffects` at lower intensities

### Phase 3: Data Integration

1. Add `blockCount`, `seoScore`, `isPillar` to Page node data
2. Add `order`, `blockType`, `fieldCount` to Block node data
3. Add `injectRef` to ContentSlot node data

---

## File Structure

```
apps/studio/src/components/graph/nodes/card/
├── CardShell.tsx                    # Existing (add premiumLevel)
├── index.ts                         # Exports
└── presets/
    ├── StructuralCardContent.tsx    # Existing (generic fallback)
    ├── PageCardContent.tsx          # NEW
    ├── BlockCardContent.tsx         # NEW
    ├── ContentSlotCardContent.tsx   # NEW
    └── shared/
        ├── StatusBadge.tsx          # NEW
        ├── MiniWireframe.tsx        # NEW
        └── OrderBadge.tsx           # NEW
```

---

## References

- ADR-028: Page-Entity Architecture
- ADR-030: Slug Ownership
- ADR-031: SEO Pillar/Cluster Architecture
- 21st.dev Card component variants
- @dnd-kit Sortable component
- Microlink API for URL previews
