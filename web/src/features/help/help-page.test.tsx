import { render, screen, within } from "@testing-library/react"
import { createMemoryRouter, RouterProvider } from "react-router"
import { expect, it } from "vitest"

import { HelpPage } from "@/features/help/help-page"

it("opens without logging in, with contents linking to each section", () => {
  const router = createMemoryRouter([{ path: "/help", Component: HelpPage }], {
    initialEntries: ["/help"],
  })
  render(<RouterProvider router={router} />)

  expect(screen.getByRole("heading", { level: 1, name: "Help" })).toBeInTheDocument()
  const contents = within(screen.getByRole("navigation", { name: "On this page" }))
  expect(contents.getByRole("link", { name: "Approving drafts" })).toHaveAttribute(
    "href",
    "#approving-drafts"
  )
  expect(screen.getByRole("heading", { level: 2, name: "Approving drafts" })).toHaveAttribute(
    "id",
    "approving-drafts"
  )
})
