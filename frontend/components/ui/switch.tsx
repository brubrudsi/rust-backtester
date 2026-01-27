"use client";

import * as SwitchPrimitive from "@radix-ui/react-switch";
import { cn } from "@/lib/utils";

export function Switch({
  className,
  checked,
  onCheckedChange,
  disabled,
}: {
  className?: string;
  checked: boolean;
  onCheckedChange: (v: boolean) => void;
  disabled?: boolean;
}) {
  return (
    <SwitchPrimitive.Root
      disabled={disabled}
      checked={checked}
      onCheckedChange={onCheckedChange}
      className={cn(
        "relative inline-flex h-6 w-11 items-center rounded-full border border-neutral-700 bg-neutral-900 data-[state=checked]:bg-neutral-200 data-[state=checked]:border-neutral-200",
        disabled && "opacity-50",
        className
      )}
    >
      <SwitchPrimitive.Thumb
        className={cn(
          "block h-5 w-5 translate-x-0.5 rounded-full bg-neutral-200 data-[state=checked]:translate-x-[1.3rem] data-[state=checked]:bg-neutral-950 transition"
        )}
      />
    </SwitchPrimitive.Root>
  );
}
