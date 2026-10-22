import { existsSync, mkdirSync } from "node:fs"

import { chromium, type FullConfig } from "@playwright/test"

import { kompleto } from "../e2e/support.ts"
import { DEMO_DATE, FIRM_EMAIL, SESSION } from "./support.ts"

const mailpit = process.env.MAILPIT_URL ?? "http://localhost:8025"

/** Fills Kompleto with the demo Firm, unless it's already there, and logs
 *  in as it once; the session lasts 30 days. */
export default async function globalSetup(config: FullConfig) {
  const { baseURL } = config.projects[0].use
  try {
    kompleto("demo-firm", "--email", FIRM_EMAIL, "--date", DEMO_DATE)
  } catch (error) {
    const output = String((error as { stderr?: string }).stderr ?? error)
    if (!output.includes("already uses that login email")) throw error
  }
  if (existsSync(SESSION)) return
  mkdirSync("showcase/.auth", { recursive: true })
  const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH })
  const page = await browser.newPage({ baseURL })
  const earlier = await newestEmailTo(FIRM_EMAIL)
  await page.goto("/login")
  await page.getByLabel("Your email").fill(FIRM_EMAIL)
  await page.getByRole("button", { name: "Email me a login link" }).click()
  await page.getByText("Check your email").waitFor()
  await page.goto(await loginLinkAfter(FIRM_EMAIL, earlier))
  await page.getByRole("button", { name: "Log in" }).click()
  await page.getByRole("heading", { name: "Dashboard" }).waitFor()
  await page.context().storageState({ path: SESSION })
  await browser.close()
}

async function newestEmailTo(to: string): Promise<string | undefined> {
  const search = await fetch(
    `${mailpit}/api/v1/search?query=${encodeURIComponent(`to:${to}`)}&limit=1`
  )
  const found = (await search.json()) as { messages?: { ID: string }[] }
  return found.messages?.[0]?.ID
}

/** The login link in the first email to `to` newer than `earlier`; an
 *  earlier link may already have been used. */
async function loginLinkAfter(to: string, earlier: string | undefined): Promise<string> {
  for (let attempt = 0; attempt < 50; attempt++) {
    const id = await newestEmailTo(to)
    if (id && id !== earlier) {
      const message = (await (await fetch(`${mailpit}/api/v1/message/${id}`)).json()) as {
        Text: string
      }
      const link = message.Text.match(/https?:\/\/\S+\/login\/[0-9a-f]+/)?.[0]
      if (link) return link
    }
    await new Promise((resolve) => setTimeout(resolve, 200))
  }
  throw new Error(`no new login email to ${to}`)
}
