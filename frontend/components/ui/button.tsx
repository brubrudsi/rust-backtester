import { cn } from "@/lib/utils";

type Props = React.ButtonHTMLAttributes<HTMLButtonElement> & {
  variant?: "default" | "secondary";
};

export function Button({ className, variant = "default", ...props }: Props) {
  const base = "inline-flex items-center justify-center rounded-md px-4 py-2 text-sm font-medium transition border";
  const v =
    variant === "secondary"
      ? "bg-neutral-900 border-neutral-700 text-neutral-100 hover:bg-neutral-800"
      : "bg-neutral-50 border-neutral-50 text-neutral-950 hover:bg-neutral-200";
  return <button className={cn(base, v, className)} {...props} />;
}
