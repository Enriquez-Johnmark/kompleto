import { render, screen } from "@testing-library/react"
import { createMemoryRouter, RouterProvider } from "react-router"
import { expect, it } from "vitest"

import { PrivacyPage } from "@/features/legal/privacy-page"

it("shows the privacy notice, with links to the other documents, without logging in", () => {
  const router = createMemoryRouter([{ path: "/privacy", Component: PrivacyPage }], {
    initialEntries: ["/privacy"],
  })
  render(<RouterProvider router={router} />)

  expect(screen.getByRole("heading", { level: 1, name: "Privacy notice" })).toBeInTheDocument()
  expect(screen.getByRole("heading", { name: "6. Who helps us (sub-processors)" })).toHaveAttribute(
    "id",
    "who-helps-us-sub-processors"
  )
  const documents = screen.getByRole("navigation", { name: "Documents" })
  expect(documents).toHaveTextContent("HelpPrivacyTermsData processing")
  expect(screen.getByRole("link", { name: "Privacy" })).toHaveAttribute("aria-current", "page")
})
