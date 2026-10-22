import { test, type Page } from "@playwright/test"

import { OUT } from "./support.ts"

async function shot(page: Page, name: string) {
  await page.waitForLoadState("networkidle")
  await page.waitForTimeout(500) // let transitions finish
  await page.screenshot({ path: `${OUT}/${name}.png` })
}

/** The Dela Cruz Pharmacy's upload link, read from its waiting Draft. */
async function uploadLink(page: Page) {
  await page.goto("/drafts")
  const text = await page
    .getByText(/\/u\/[0-9a-f]+/)
    .first()
    .textContent()
  const path = text?.match(/\/u\/[0-9a-f]+/)?.[0]
  if (!path) throw new Error("no upload link in the drafts")
  return path
}

test.describe("desktop", () => {
  test.use({ viewport: { width: 1440, height: 900 } })

  test("screenshots", async ({ page }) => {
    await page.goto("/")
    await page.getByText("Lim Hardware Depot").filter({ visible: true }).first().waitFor()
    await shot(page, "desktop-1-dashboard")

    await page.goto("/drafts")
    await page.getByText("Select all").waitFor()
    await shot(page, "desktop-2-drafts")
    await page.getByText("Select all").click()
    await page.getByRole("button", { name: /^Review \d+ selected$/ }).click()
    await page.getByRole("dialog").waitFor()
    await shot(page, "desktop-3-review-before-approving")
    await page.getByRole("button", { name: "Cancel" }).click()

    await page.goto("/clients")
    await page
      .getByRole("link", { name: "Lito Hardware & Construction Supply" })
      .filter({ visible: true })
      .first()
      .click()
    await page.getByRole("tab", { name: "Documents" }).click()
    await shot(page, "desktop-4-client-documents-and-payors")
    await page.goto("/clients")
    await page
      .getByRole("link", { name: "Bautista Bakeshop" })
      .filter({ visible: true })
      .first()
      .click()
    await page.getByRole("tab", { name: /^Uploads/ }).click()
    await shot(page, "desktop-5-client-uploads")

    for (const [path, name] of [
      ["/tracker", "desktop-6-2307-tracker"],
      ["/calendar", "desktop-7-bir-calendar"],
      ["/activity", "desktop-8-activity"],
      ["/help", "desktop-9-help"],
    ]) {
      await page.goto(path)
      await page.getByRole("heading").first().waitFor()
      await shot(page, name)
    }
  })
})

test.describe("phone", () => {
  test.use({ viewport: { width: 390, height: 844 }, deviceScaleFactor: 2, isMobile: true })

  test("screenshots", async ({ page }) => {
    await page.goto("/")
    await page.getByText("Lim Hardware Depot").filter({ visible: true }).first().waitFor()
    await shot(page, "phone-1-dashboard")
    await page.goto("/drafts")
    await page.getByText("Select all").waitFor()
    await shot(page, "phone-2-drafts")
    // What the client sees: no login, in their language.
    await page.goto(await uploadLink(page))
    await page.getByRole("button", { name: "I-upload" }).waitFor()
    await shot(page, "phone-3-client-upload-page")
  })
})
