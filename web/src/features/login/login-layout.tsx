import type { ReactNode } from "react"

import { Logo } from "@/components/logo"

/** The centred card the login pages sit in. */
export function LoginLayout({ children }: { children: ReactNode }) {
  return (
    <div className="flex min-h-svh flex-col items-center justify-center gap-6 bg-muted p-4">
      <Logo className="text-lg" />
      <div className="w-full max-w-sm">{children}</div>
    </div>
  )
}
