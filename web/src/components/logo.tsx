import { cn } from "@/lib/utils"

/** Kompleto's name beside a ticked box. */
export function Logo({ className }: { className?: string }) {
  return (
    <span className={cn("flex items-center gap-2 font-semibold", className)}>
      <svg viewBox="0 0 32 32" aria-hidden="true" className="size-6">
        <rect width="32" height="32" rx="8" className="fill-primary" />
        <path
          d="M9 16.5l4.5 4.5L23 11.5"
          fill="none"
          strokeWidth="3.2"
          strokeLinecap="round"
          strokeLinejoin="round"
          className="stroke-primary-foreground"
        />
      </svg>
      Kompleto
    </span>
  )
}
