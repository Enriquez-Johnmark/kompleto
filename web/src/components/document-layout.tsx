import type { ReactNode } from "react"
import { Link, NavLink, ScrollRestoration } from "react-router"

import { Logo } from "@/components/logo"
import { cn } from "@/lib/utils"

const documents = [
  { to: "/help", label: "Help" },
  { to: "/privacy", label: "Privacy" },
  { to: "/terms", label: "Terms" },
  { to: "/data-processing", label: "Data processing" },
]

/** A readable column for the help page and the legal documents, which
 *  anyone can open without logging in. */
export function DocumentLayout({ children }: { children: ReactNode }) {
  return (
    <div className="min-h-svh bg-background">
      <ScrollRestoration />
      <header className="border-b">
        <div className="mx-auto flex max-w-3xl flex-wrap items-center justify-between gap-3 px-4 py-3">
          <Link to="/" aria-label="Kompleto home">
            <Logo />
          </Link>
          <nav aria-label="Documents" className="flex flex-wrap gap-x-4 gap-y-1 text-sm">
            {documents.map((document) => (
              <NavLink
                key={document.to}
                to={document.to}
                className={({ isActive }) =>
                  cn(
                    "text-muted-foreground hover:text-foreground",
                    isActive && "font-medium text-foreground"
                  )
                }
              >
                {document.label}
              </NavLink>
            ))}
          </nav>
        </div>
      </header>
      <main className="mx-auto max-w-3xl px-4 py-8 md:py-12">{children}</main>
    </div>
  )
}
