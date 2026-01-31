import * as React from "react"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@/lib/utils"
import { glassClasses, opacity } from "@/design/tokens"

const cardVariants = cva(
  "rounded-xl border text-card-foreground transition-all duration-200",
  {
    variants: {
      variant: {
        // Default - Surface-1 (5% lightness) with token-based borders
        default:
          `bg-[hsl(240,8%,5%)] border-${opacity.border.medium} hover:bg-[hsl(240,6%,8%)] hover:border-${opacity.border.heavy}`,
        // Glass - Surface-2 with backdrop blur
        glass: glassClasses.light,
        // Floating - Elevated with premium shadow
        floating: glassClasses.floating,
        // Outline - Transparent with visible border (uses strong border token)
        outline:
          `bg-transparent border-${opacity.border.strong} hover:border-${opacity.border.heavy} hover:bg-${opacity.bg.light}`,
        // Solid - Opaque surface-1 with token-based border
        solid:
          `bg-[hsl(240,8%,5%)] border-${opacity.border.medium}`,
        // Interactive - Clear hover feedback (surface-1 → surface-3)
        interactive:
          `bg-[hsl(240,8%,5%)] border-${opacity.border.medium} hover:bg-[hsl(240,5%,12%)] hover:border-${opacity.border.heavy} hover:shadow-lg hover:shadow-black/40 cursor-pointer`,
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
      `border-t border-${opacity.border.subtle} mt-4 pt-4`,
      className
    )}
    {...props}
  />
))
CardFooter.displayName = "CardFooter"

export { Card, CardHeader, CardFooter, CardTitle, CardDescription, CardContent, cardVariants }
