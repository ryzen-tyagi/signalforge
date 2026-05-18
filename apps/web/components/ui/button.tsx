import type { ButtonHTMLAttributes } from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";

const buttonVariants = cva("sf-button", {
  variants: {
    variant: {
      default: "sf-button-default",
      secondary: "sf-button-secondary",
      ghost: "sf-button-ghost",
      destructive: "sf-button-destructive",
    },
    size: {
      default: "sf-button-md",
      sm: "sf-button-sm",
      icon: "sf-button-icon",
    },
  },
  defaultVariants: {
    variant: "default",
    size: "default",
  },
});

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> &
  VariantProps<typeof buttonVariants>;

export function Button({ className, variant, size, ...props }: ButtonProps) {
  return (
    <button
      className={cn(buttonVariants({ variant, size }), className)}
      {...props}
    />
  );
}

