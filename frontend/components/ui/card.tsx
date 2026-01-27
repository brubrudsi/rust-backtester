import { cn } from "@/lib/utils";

export function Card({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn("rounded-lg border border-neutral-800 bg-neutral-900/40 shadow-sm", className)}
      {...props}
    />
  );
}
