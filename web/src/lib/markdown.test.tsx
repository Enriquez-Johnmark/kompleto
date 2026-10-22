import { render, screen } from "@testing-library/react"
import { MemoryRouter } from "react-router"
import { describe, expect, it } from "vitest"

import { Markdown, parseBlocks, slug } from "@/lib/markdown"

describe("parseBlocks", () => {
  it("splits headings, paragraphs, lists and notes", () => {
    const source = [
      "# Privacy",
      "",
      "> Draft for review.",
      "",
      "We keep files",
      "for 90 days.",
      "## 2. What we keep",
      "- Uploads, encrypted",
      "  until deleted",
      "- Replies",
      "",
      "1. First",
      "2. Second",
    ].join("\n")

    expect(parseBlocks(source)).toEqual([
      { kind: "heading", level: 1, text: "Privacy", id: "privacy" },
      { kind: "note", text: "Draft for review." },
      { kind: "paragraph", text: "We keep files for 90 days." },
      { kind: "heading", level: 2, text: "2. What we keep", id: "what-we-keep" },
      { kind: "list", ordered: false, items: ["Uploads, encrypted until deleted", "Replies"] },
      { kind: "list", ordered: true, items: ["First", "Second"] },
    ])
  })

  it("makes anchors without numbering or punctuation", () => {
    expect(slug("3. Who helps us (sub-processors)")).toBe("who-helps-us-sub-processors")
  })
})

describe("Markdown", () => {
  it("renders bold, code and safe links only", () => {
    render(
      <MemoryRouter>
        <Markdown
          source={
            "**Kompleto** sets `CLAMD_ADDR`; read [the terms](/terms), write [us](mailto:help@example.test), not [this](javascript:alert(1))."
          }
        />
      </MemoryRouter>
    )

    expect(screen.getByText("Kompleto").tagName).toBe("STRONG")
    expect(screen.getByText("CLAMD_ADDR").tagName).toBe("CODE")
    expect(screen.getByRole("link", { name: "the terms" })).toHaveAttribute("href", "/terms")
    expect(screen.getByRole("link", { name: "us" })).toHaveAttribute(
      "href",
      "mailto:help@example.test"
    )
    expect(screen.queryByRole("link", { name: "this" })).toBeNull()
  })
})
