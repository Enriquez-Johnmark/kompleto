import { readFileSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { join } from "node:path"

import { expect, test } from "@playwright/test"

import { firstOfNextMonth, kompleto, loginLink } from "./support.ts"

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

test("h3 Payors are added from the customers who withheld tax in a sales export", async ({
  page,
}) => {
  const file = join(tmpdir(), `kompleto-sales-${run}.csv`)
  writeFileSync(
    file,
    "Customer,Invoice,Amount,CWT\n" +
      "Ayala Land,INV-1,50000,1000\n" +
      "Puregold,INV-2,3000,0\n" +
      "Meralco,INV-3,6000,120\n"
  )
  await page.goto("/clients/new")
  await page.getByLabel("Business name").fill(`Law Office ${run}`)
  await page.getByLabel("Contact's first name").fill("Tess")
  await page.getByLabel("Email").fill("tess@example.test")
  await page.getByRole("checkbox", { name: /2551Q/ }).click()
  await page.getByRole("button", { name: "Add client" }).click()
  await page.getByRole("tab", { name: "Documents" }).click()

  await page.getByRole("button", { name: "From a sales export" }).click()
  const dialog = page.getByRole("dialog", { name: "Payors from a sales export" })
  await dialog.locator("input[type=file]").setInputFiles(file)
  await expect(dialog.getByRole("checkbox", { name: "Ayala Land" })).toBeChecked()
  await expect(dialog.getByText("Puregold")).toHaveCount(0)
  await dialog.getByRole("checkbox", { name: "Meralco" }).click()
  await dialog.getByRole("button", { name: "Add 1 of 2" }).click()

  await expect(dialog).toHaveCount(0)
  await expect(page.getByRole("button", { name: "Remove Ayala Land" })).toBeVisible()
  await expect(page.getByRole("button", { name: "Remove Meralco" })).toHaveCount(0)
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

test("b5 a Xero contact export imports with the forms chosen for it", async ({ page }) => {
  const file = join(tmpdir(), `kompleto-xero-${run}.csv`)
  writeFileSync(
    file,
    "*ContactName,EmailAddress,FirstName,LastName,TaxNumber\n" +
      `Aquino Rice ${run},aquino@example.test,Mario,Aquino,123-456-789-000\n`
  )
  await page.goto("/import")
  const defaults = page.locator("[data-slot=card]", { hasText: "For files without" })
  await defaults.getByRole("checkbox", { name: "My file has no Language or Forms columns" }).click()
  await defaults.getByLabel("Language", { exact: true }).selectOption("english")
  await defaults.getByRole("checkbox", { name: /2551Q/ }).click()
  await page.locator("input[type=file]").setInputFiles(file)

  await expect(page.getByRole("cell", { name: "Mario" })).toBeVisible()
  await page.getByRole("button", { name: "Add 1 client" }).click()
  await page.getByRole("link", { name: `Aquino Rice ${run}` }).click()

  await expect(page.getByText("English").first()).toBeVisible()
  await expect(page.getByText("2551Q", { exact: true }).first()).toBeVisible()
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

test("k1 the owner invites staff, who log in without the owner's settings", async ({
  page,
  browser,
  baseURL,
}) => {
  const email = `staff-${run}@example.test`
  await page.goto("/settings")
  const staff = page.locator("[data-slot=card]", { hasText: "Staff" })
  await staff.getByLabel("Staff member's email").fill(email)
  await staff.getByRole("button", { name: "Invite" }).click()
  await expect(staff.getByText(email)).toBeVisible()

  const theirs = await browser.newContext({ baseURL, storageState: undefined })
  const theirPage = await theirs.newPage()
  await theirPage.goto(await loginLink(email))
  await theirPage.getByRole("button", { name: "Log in" }).click()
  await theirPage.goto("/settings")
  await expect(theirPage.getByText("Only the firm's owner")).toBeVisible()
  await expect(theirPage.getByRole("link", { name: "Download all data" })).toHaveCount(0)
  await expect(theirPage.getByText(email)).toBeVisible()
  await theirs.close()
})

test("l1 a visitor requests an account, which waits for the operator", async ({
  browser,
  baseURL,
}) => {
  const visitor = await browser.newContext({ baseURL, storageState: undefined })
  const page = await visitor.newPage()
  const email = `new-${run}@example.test`
  await page.goto("/login")
  await page.getByRole("link", { name: "Request an account" }).click()

  await page.getByLabel("Firm name").fill(`Cruz Bookkeeping ${run}`)
  await page.getByLabel("Your email").fill(email)
  await page.getByRole("button", { name: "Request an account" }).click()
  await expect(
    page.getByText("Accept the terms and the data processing agreement", { exact: true })
  ).toBeVisible()
  await page.getByRole("checkbox").click()
  await page.getByRole("button", { name: "Request an account" }).click()

  await expect(page.getByText("Thanks! Your request is in")).toBeVisible()
  expect(kompleto("requests")).toContain(email)
  await visitor.close()
})
