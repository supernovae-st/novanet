import * as React from "react"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@/lib/utils"

const cardVariants = cva(
  "rounded-xl border text-card-foreground transition-all duration-200",
  {
    variants: {
      variant: {
        // Default - Surface-1 (5% lightness)
        default:
          "bg-[hsl(240,8%,5%)] border-white/12 hover:bg-[hsl(240,6%,8%)] hover:border-white/18",
        // Glass - Surface-2 with backdrop blur
        glass:
          "bg-[hsl(240,6%,8%)] backdrop-blur-xl border-white/12 shadow-lg shadow-black/40",
        // Floating - Elevated with premium shadow
        floating:
          "bg-[hsl(240,6%,8%)] backdrop-blur-xl border-white/15 rounded-2xl shadow-2xl shadow-black/50 ring-1 ring-white/4 ring-inset",
        // Outline - Transparent with visible border
        outline:
          "bg-transparent border-white/15 hover:border-white/25 hover:bg-white/5",
        // Solid - Opaque surface-1
        solid:
          "bg-[hsl(240,8%,5%)] border-white/12",
        // Interactive - Clear hover feedback (surface-1 → surface-3)
        interactive:
          "bg-[hsl(240,8%,5%)] border-white/12 hover:bg-[hsl(240,5%,12%)] hover:border-white/20 hover:shadow-lg hover:shadow-black/40 cursor-pointer",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
)

export interface CardProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof cardVariants> {}

const Card = React.forwardRef<HTMLDivElement, CardProps>(
  ({ className, variant, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(cardVariants({ variant, className }))}
      {...props}
    />
  )
)
Card.displayName = "Card"

const CardHeader = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("flex flex-col space-y-1.5 p-5", className)}
    {...props}
  />
))
CardHeader.displayName = "CardHeader"

const CardTitle = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("text-lg font-semibold text-white", className)}
    {...props}
  />
))
CardTitle.displayName = "CardTitle"

const CardDescription = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn("text-sm text-white/65", className)}
    {...props}
  />
))
CardDescription.displayName = "CardDescription"

const CardContent = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div ref={ref} className={cn("p-5 pt-0", className)} {...props} />
))
CardContent.displayName = "CardContent"

const CardFooter = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn(
      "flex items-center p-5 pt-0",
      "border-t border-white/[0.06] mt-4 pt-4",
      className
    )}
    {...props}
  />
))
CardFooter.displayName = "CardFooter"

export { Card, CardHeader, CardFooter, CardTitle, CardDescription, CardContent, cardVariants }
