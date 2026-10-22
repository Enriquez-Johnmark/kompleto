import { mkdirSync, writeFileSync } from "node:fs"

import { chromium, type FullConfig } from "@playwright/test"

import { kompleto, loginLink } from "./support.ts"

/** A new Firm for this run, logged in once (A1 allows 5 login emails an
 *  hour); every test starts from that session. */
export default async function globalSetup(config: FullConfig) {
  const { baseURL } = config.projects[0].use
  const email = `e2e-${Date.now()}@example.test`
  kompleto("firm", "add", "--name", "E2E Bookkeeping", "--email", email)
  mkdirSync("e2e/.auth", { recursive: true })
  writeFileSync("e2e/.auth/firm.json", JSON.stringify({ email }))

  const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH })
  const page = await browser.newPage({ baseURL })
  await page.goto("/login")
  await page.getByLabel("Your email").fill(email)
  await page.getByRole("button", { name: "Email me a login link" }).click()
  await page.getByText("Check your email").waitFor()
  await page.goto(await loginLink(email))
  await page.getByRole("button", { name: "Log in" }).click()
  await page.getByRole("heading", { name: "Dashboard" }).waitFor()
  await page.context().storageState({ path: "e2e/.auth/state.json" })
  await browser.close()
}
