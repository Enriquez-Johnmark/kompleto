import { test, type Page } from "@playwright/test"

import { OUT, SALES_EXPORT, XERO_CONTACTS } from "./support.ts"

async function shot(page: Page, name: string) {
  await page.waitForLoadState("networkidle")
  await page.waitForTimeout(500) // let transitions finish
  await page.screenshot({ path: `${OUT}/${name}.png` })
}

/** A client's upload link, read from its first waiting Draft. */
async function uploadLink(page: Page, client: string) {
  await page.goto("/drafts")
  const text = await page
    .locator("[data-slot=card]", { hasText: client })
    .first()
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

    // M9–M13.
    await page.goto("/clients")
    await page
      .getByRole("link", { name: "Bautista Bakeshop" })
      .filter({ visible: true })
      .first()
      .click()
    await page.getByRole("tab", { name: "Schedule" }).click()
    await page.getByText("Payroll chasing", { exact: true }).waitFor()
    await shot(page, "desktop-10-payroll-chasing")

    await page.goto("/clients")
    await page
      .getByRole("link", { name: "Lito Hardware & Construction Supply" })
      .filter({ visible: true })
      .first()
      .click()
    await page.getByRole("tab", { name: "Documents" }).click()
    await page.getByRole("button", { name: "From a sales export" }).click()
    const dialog = page.getByRole("dialog", { name: "Payors from a sales export" })
    await dialog.locator("input[type=file]").setInputFiles(SALES_EXPORT)
    await dialog.getByText("Megaworld Corp").waitFor()
    await shot(page, "desktop-11-payors-from-a-sales-export")
    await dialog.getByRole("button", { name: "Cancel" }).click()

    await page.goto("/import")
    const defaults = page.locator("[data-slot=card]", { hasText: "For files without" })
    await defaults
      .getByRole("checkbox", { name: "My file has no Language or Forms columns" })
      .click()
    await defaults.getByRole("checkbox", { name: /2551Q/ }).click()
    await defaults.getByRole("checkbox", { name: /1701Q/ }).click()
    await page.locator("input[type=file]").setInputFiles(XERO_CONTACTS)
    await page.getByRole("cell", { name: "Pascual Laundry Hub" }).waitFor()
    await page.getByText(/clients? ready to add/).scrollIntoViewIfNeeded()
    await page.mouse.wheel(0, 300)
    await shot(page, "desktop-12-import-from-xero")

    await page.goto("/settings")
    await page
      .getByText(/^carlo@/)
      .first()
      .waitFor()
    await page
      .getByText(/^carlo@/)
      .first()
      .scrollIntoViewIfNeeded()
    await shot(page, "desktop-13-staff")
  })

  test("request an account", async ({ browser }, testInfo) => {
    const context = await browser.newContext({
      baseURL: testInfo.project.use.baseURL,
      storageState: undefined,
      viewport: { width: 1440, height: 900 },
    })
    const page = await context.newPage()
    await page.goto("/signup")
    await page.getByRole("button").first().waitFor()
    await shot(page, "desktop-14-request-an-account")
    await context.close()
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
    await page.goto(await uploadLink(page, "Dela Cruz Pharmacy"))
    await page.getByRole("button", { name: "I-upload" }).waitFor()
    await shot(page, "phone-3-client-upload-page")
    // The same page for a client who reads English, for the landing page.
    await page.goto(await uploadLink(page, "Dr. Reyes Dental Clinic"))
    await page.getByRole("button", { name: "Upload" }).waitFor()
    await shot(page, "phone-4-client-upload-page-english")
  })
})
