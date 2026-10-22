import { render, screen } from "@testing-library/react"
import type { ComponentType } from "react"
import { createMemoryRouter, RouterProvider } from "react-router"
import { expect, it } from "vitest"

import { PrivacyPage } from "@/features/legal/privacy-page"
import { TermsPage } from "@/features/legal/terms-page"

function show(path: string, Page: ComponentType) {
  const router = createMemoryRouter([{ path, Component: Page }], { initialEntries: [path] })
  render(<RouterProvider router={router} />)
}

it.each([
  ["/privacy", PrivacyPage, "Privacy notice", "Privacy"],
  ["/terms", TermsPage, "Terms of service", "Terms"],
])("%s opens without logging in, marked as a draft", (path, Page, title, link) => {
  show(path, Page)

  expect(screen.getByRole("heading", { level: 1, name: title })).toBeInTheDocument()
  expect(screen.getByText("Draft for review, not yet in force.")).toBeInTheDocument()
  const documents = screen.getByRole("navigation", { name: "Documents" })
  expect(documents).toHaveTextContent("HelpPrivacyTermsData processing")
  expect(screen.getByRole("link", { name: link })).toHaveAttribute("aria-current", "page")
})

it("gives the privacy notice's sections anchors", () => {
  show("/privacy", PrivacyPage)

  expect(screen.getByRole("heading", { name: "6. Who helps us (sub-processors)" })).toHaveAttribute(
    "id",
    "who-helps-us-sub-processors"
  )
})
