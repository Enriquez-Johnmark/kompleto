import { readFileSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"

import { expect, test } from "@playwright/test"

import { firstOfNextMonth, kompleto } from "./support.ts"

// Names unique to this run, so the tests can share one Firm.
const run = Date.now().toString(36)

test("a1 a logged-in bookkeeper sees their firm and the dashboard", async ({ page }) => {
  const { email } = JSON.parse(readFileSync("e2e/.auth/firm.json", "utf8")) as { email: string }
  await page.goto("/")
  await expect(page.getByRole("heading", { name: "Dashboard" })).toBeVisible()
  await expect(page.getByText(email)).toBeVisible()
})

test("b1 a client added through the form gets its deadlines", async ({ page }) => {
  await page.goto("/clients/new")
  await page.getByLabel("Business name").fill(`Nena's Store ${run}`)
  await page.getByLabel("Contact's first name").fill("Nena")
  await page.getByLabel("Email").fill("nena@example.test")
  await page.getByRole("checkbox", { name: /2551Q/ }).click()
  await page.getByRole("button", { name: "Add client" }).click()

  await expect(page.getByRole("heading", { name: `Nena's Store ${run}` })).toBeVisible()
  await expect(page.getByText("Documents to ask for").first()).toBeVisible()
  await expect(page.getByText(/^Cut-off/).first()).toBeVisible()
})

test("h2 a Payor added on the client page is listed", async ({ page }) => {
  await page.goto("/clients/new")
  await page.getByLabel("Business name").fill(`Dental Clinic ${run}`)
  await page.getByLabel("Contact's first name").fill("Mike")
  await page.getByLabel("Email").fill("mike@example.test")
  await page.getByRole("checkbox", { name: /2551Q/ }).click()
  await page.getByRole("button", { name: "Add client" }).click()
  await page.getByRole("tab", { name: "Documents" }).click()

  await page.getByLabel("Payor's name").fill("Jollibee Foods Corp")
  await page.getByRole("button", { name: "Add" }).last().click()

  await expect(page.getByText("Jollibee Foods Corp")).toBeVisible()
  await expect(page.getByLabel("Payor's name")).toHaveValue("")
  await expect(page.getByRole("alert")).toHaveCount(0)
})

test("b1 a form with problems shows every reason and adds nothing", async ({ page }) => {
  await page.goto("/clients/new")
  await page.getByRole("button", { name: "Add client" }).click()

  await expect(page.getByText("Enter the business name.")).toBeVisible()
  await expect(page.getByText("Tick at least one form.")).toBeVisible()
  await expect(page).toHaveURL(/\/clients\/new$/)
})

test("b2 an import is previewed and only confirming adds the valid rows", async ({ page }) => {
  const file = join(tmpdir(), `kompleto-${run}.csv`)
  writeFileSync(
    file,
    "Business name,Contact first name,Email,Language,Forms\n" +
      `Lito Hardware ${run},Lito,lito@example.test,English,2550Q\n` +
      ",Ramon,ramon@example.test,English,2550Q\n"
  )
  await page.goto("/import")
  await page.locator("input[type=file]").setInputFiles(file)

  await expect(page.getByText("1 row won't be added")).toBeVisible()
  await expect(page.getByText("Row 3: Business name: is required")).toBeVisible()
  await page.getByRole("button", { name: "Add 1 client" }).click()

  await expect(page).toHaveURL(/\/clients$/)
  await expect(page.getByRole("link", { name: `Lito Hardware ${run}` })).toBeVisible()
})

test("d3 a batch shows every draft in full before any is approved", async ({ page }) => {
  // An employer files 1601-C monthly, so next month's first day drafts a reminder.
  await page.goto("/clients/new")
  await page.getByLabel("Business name").fill(`Ben Bakery ${run}`)
  await page.getByLabel("Contact's first name").fill("Ben")
  await page.getByLabel("Email").fill("ben@example.test")
  await page.getByRole("checkbox", { name: /1601-C/ }).click()
  await page.getByRole("button", { name: "Add client" }).click()
  await expect(page.getByRole("heading", { name: `Ben Bakery ${run}` })).toBeVisible()
  kompleto("dev-schedule", "--date", firstOfNextMonth())

  await page.goto("/drafts")
  const card = page.locator("[data-slot=card]", { hasText: `Ben Bakery ${run}` }).first()
  await card.getByRole("checkbox").click()
  await page.getByRole("button", { name: /Review 1 selected/ }).click()
  const review = page.getByRole("dialog")
  await expect(review.getByText("ben@example.test")).toBeVisible()
  await expect(review.getByText(/\/u\/[0-9a-f]+/)).toBeVisible()
  await review.getByRole("button", { name: "Approve this draft" }).click()

  await expect(page.getByText("1 approved")).toBeVisible()
  await expect(
    page.locator("section", { hasText: "Approved this week" }).getByText(`Ben Bakery ${run}`)
  ).toBeVisible()
})

test("f1 the upload page asks for a note only for something else", async ({ page }) => {
  await page.goto("/clients/new")
  await page.getByLabel("Business name").fill(`Cora Cafe ${run}`)
  await page.getByLabel("Contact's first name").fill("Cora")
  await page.getByLabel("Email").fill("cora@example.test")
  await page.getByRole("checkbox", { name: /1601-C/ }).click()
  await page.getByRole("button", { name: "Add client" }).click()
  await expect(page.getByRole("heading", { name: `Cora Cafe ${run}` })).toBeVisible()
  kompleto("dev-schedule", "--date", firstOfNextMonth())
  await page.goto("/drafts")
  const card = page.locator("[data-slot=card]", { hasText: `Cora Cafe ${run}` }).first()
  const link = (await card.getByText(/\/u\/[0-9a-f]+/).textContent())?.match(/\/u\/[0-9a-f]+/)

  await page.goto(link![0])
  const note = page.getByLabel(/^(Kung iba pa|If something else)/)
  await expect(note).toBeHidden()
  await page.getByRole("combobox").selectOption("other")

  await expect(note).toBeVisible()
})
