import { describe, expect, it } from "vitest"

import { cn } from "@/lib/utils"

describe("cn", () => {
  it("lets a later Tailwind class override an earlier one", () => {
    expect(cn("px-2 text-sm", false && "hidden", "px-4")).toBe("text-sm px-4")
  })
})
