import * as React from "react"
import { Slot } from "@radix-ui/react-slot"
import { cva, type VariantProps } from "class-variance-authority"
import { Loader2 } from "lucide-react"

import { cn } from "@/lib/utils"

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-lg text-sm font-medium transition-all duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
  {
    variants: {
      variant: {
        // Primary - NovaNet brand cyan with strong glow
        default:
          "bg-primary text-primary-foreground shadow-lg shadow-primary/40 hover:bg-primary/90 hover:shadow-primary/60 hover:shadow-xl",
        // Destructive - Vivid coral with visible background
        destructive:
          "bg-red-500/20 text-red-400 border border-red-500/30 hover:bg-red-500/30 hover:border-red-500/50 hover:text-red-300",
        // Outline - High contrast border (surface-2 bg)
        outline:
          "border border-white/15 bg-[hsl(240,6%,8%)] hover:bg-[hsl(240,5%,12%)] hover:border-white/25 text-white/90 hover:text-white",
        // Secondary - Visible surface (surface-3)
        secondary:
          "bg-[hsl(240,5%,12%)] text-white/85 hover:bg-[hsl(240,5%,16%)] hover:text-white border border-white/8 hover:border-white/15",
        // Ghost - Clear hover
        ghost:
          "text-white/70 hover:bg-white/10 hover:text-white",
        // Link - Bright link
        link:
          "text-primary hover:text-primary/80 underline-offset-4 hover:underline",
        // Glass - Premium glass with inner glow
        glass:
          "bg-[hsl(240,6%,8%)] backdrop-blur-xl border border-white/12 text-white/95 hover:bg-[hsl(240,5%,12%)] hover:border-white/20 shadow-lg shadow-black/40",
        // Success - Bright emerald
        success:
          "bg-emerald-500/15 text-emerald-400 border border-emerald-500/25 hover:bg-emerald-500/25 hover:border-emerald-500/40 hover:text-emerald-300",
        // Warning - Bright amber
        warning:
          "bg-amber-500/15 text-amber-400 border border-amber-500/25 hover:bg-amber-500/25 hover:border-amber-500/40 hover:text-amber-300",
      },
      size: {
        default: "h-9 px-4 py-2",
        sm: "h-8 px-3 text-xs",
        lg: "h-11 px-6 text-base",
        xl: "h-12 px-8 text-base",
        icon: "h-9 w-9",
        "icon-sm": "h-8 w-8",
        "icon-lg": "h-10 w-10",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
)

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean
  loading?: boolean
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, asChild = false, loading = false, children, disabled, ...props }, ref) => {
    const Comp = asChild ? Slot : "button"
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, className }))}
        ref={ref}
        disabled={disabled || loading}
        {...props}
      >
        {loading ? (
          <>
            <Loader2 className="animate-spin" />
            {children}
          </>
        ) : (
          children
        )}
      </Comp>
    )
  }
)
Button.displayName = "Button"

export { Button, buttonVariants }
