import * as React from "react"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@/lib/utils"

const inputVariants = cva(
  // Base: no focus ring, no outline - clean design
  "flex w-full rounded-lg text-sm transition-all duration-200 file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-white/40 outline-none ring-0 focus:outline-none focus:ring-0 disabled:cursor-not-allowed disabled:opacity-50",
  {
    variants: {
      variant: {
        // Default - Linear-dark with subtle focus state
        default:
          "bg-[#0d0d12] border border-white/10 text-white/90 focus:border-white/20 focus:bg-[#111118]",
        // Ghost - Transparent until interaction
        ghost:
          "bg-transparent border-transparent text-white/90 hover:bg-white/5 focus:bg-white/8 focus:border-white/10",
        // Filled - Visible surface background
        filled:
          "bg-[#111118] border-transparent text-white/90 focus:bg-[#16161f] focus:border-white/10",
      },
      inputSize: {
        default: "h-10 px-4 py-2",
        sm: "h-8 px-3 py-1.5 text-xs",
        lg: "h-12 px-5 py-3",
      },
    },
    defaultVariants: {
      variant: "default",
      inputSize: "default",
    },
  }
)

export interface InputProps
  extends Omit<React.InputHTMLAttributes<HTMLInputElement>, "size">,
    VariantProps<typeof inputVariants> {}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({ className, type, variant, inputSize, ...props }, ref) => {
    return (
      <input
        type={type}
        className={cn(inputVariants({ variant, inputSize, className }))}
        ref={ref}
        {...props}
      />
    )
  }
)
Input.displayName = "Input"

export { Input, inputVariants }
