import type { HTMLAttributes } from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "@/lib/utils";

const badgeVariants = cva("sf-badge", {
  variants: {
    variant: {
      default: "sf-badge-default",
      info: "sf-badge-info",
      warning: "sf-badge-warning",
      critical: "sf-badge-critical",
      success: "sf-badge-success",
      outline: "sf-badge-outline",
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

type BadgeProps = HTMLAttributes<HTMLSpanElement> &
  VariantProps<typeof badgeVariants>;

export function Badge({ className, variant, ...props }: BadgeProps) {
  return <span className={cn(badgeVariants({ variant }), className)} {...props} />;
}

