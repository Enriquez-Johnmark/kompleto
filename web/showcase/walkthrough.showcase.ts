import { test, type Page } from "@playwright/test"

import { addOverlay, OUT, pointAt, press, RECEIPT, record, say, SESSION } from "./support.ts"

const size = { width: 1280, height: 720 }

function visibleLink(page: Page, name: string) {
  return page.getByRole("link", { name }).filter({ visible: true }).first()
}

/** About two and a half minutes through a bookkeeper's week, with captions;
 *  saved as docs/showcase/walkthrough.mp4. It approves the waiting drafts
 *  and uploads a file, so it runs after the screenshots. */
test("walkthrough", async ({ browser }, testInfo) => {
  const context = await browser.newContext({
    baseURL: testInfo.project.use.baseURL,
    storageState: SESSION,
    viewport: size,
  })
  const page = await context.newPage()
  await addOverlay(page)
  const stop = await record(page, testInfo.outputPath("frames"))

  // The dashboard.
  await page.goto("/")
  await page.getByText("Lim Hardware Depot").filter({ visible: true }).first().waitFor()
  await say(page, "Kompleto chases your clients for their BIR documents, so you don't have to.")
  await say(page, "The dashboard shows what each client still owes, nearest cut-off first.")
  await pointAt(page, page.getByText("Email bounced").filter({ visible: true }).first())
  await say(page, "Bounced emails, replies and drafts waiting stand out at a glance.")

  // Drafts and approval.
  await press(page, visibleLink(page, "Drafts"))
  await page.getByText("Select all").waitFor()
  await say(
    page,
    "Every morning Kompleto drafts the reminders that are due, in Taglish or English."
  )
  const card = page.locator("[data-slot=card]", { hasText: "Dela Cruz Pharmacy" }).first()
  const linkText = await card.getByText(/\/u\/[0-9a-f]+/).textContent()
  const uploadPath = linkText?.match(/\/u\/[0-9a-f]+/)?.[0]
  if (!uploadPath) throw new Error("no upload link in Dela Cruz Pharmacy's draft")
  await pointAt(page, card.getByText(/\/u\/[0-9a-f]+/))
  await say(page, "Each one carries the client's own private upload link.")
  await pointAt(page, card.getByRole("button", { name: "Copy for Viber" }))
  await say(page, "Prefer Viber? Copy the same message into your own chat.", 4500)
  await press(page, page.getByText("Select all"))
  await press(page, page.getByRole("button", { name: /^Review \d+ selected$/ }))
  const review = page.getByRole("dialog")
  await review.waitFor()
  await say(
    page,
    "Nothing is sent until you approve it. Review them all, exactly as they'll go out."
  )
  await press(page, review.getByRole("button", { name: /^Approve these \d+$/ }))
  await page
    .getByText(/approved/)
    .first()
    .waitFor()
  await say(page, "Approved. Each email is sent once, even if the server restarts mid-send.")

  // The client's side.
  await page.goto(uploadPath)
  await page.getByRole("button", { name: "I-upload" }).waitFor()
  await say(page, "Your client opens the link on their phone. No app, no password.")
  await page.getByRole("combobox").selectOption({ index: 1 })
  await page.locator("#file").setInputFiles(RECEIPT)
  await say(page, "They choose what the file is and upload a photo or PDF.", 4500)
  await press(page, page.getByRole("button", { name: "I-upload" }))
  await page.getByText("Salamat po!").waitFor()
  await say(page, "Every file is scanned for viruses and stored encrypted.")

  // Back with the bookkeeper.
  await page.goto("/clients")
  await press(page, visibleLink(page, "Dela Cruz Pharmacy"))
  await press(page, page.getByRole("tab", { name: /^Uploads/ }))
  await say(
    page,
    "The file is on the client's page right away. Download it, then mark it received."
  )
  await press(page, page.getByRole("button", { name: "Mark received" }).first())
  await say(page, "Kompleto isn't an archive: files are deleted within 90 days.")

  await page.goto("/clients")
  await press(page, visibleLink(page, "Lito Hardware & Construction Supply"))
  await press(page, page.getByRole("tab", { name: /^Replies/ }))
  await say(page, "When a client replies, that period's reminders stop, and you get an email.")

  await press(page, visibleLink(page, "2307 tracker"))
  await page.getByRole("heading", { name: /^2307s for/ }).waitFor()
  await say(
    page,
    "The 2307 tracker shows which Payors' certificates have come in, and which haven't."
  )

  await press(page, visibleLink(page, "BIR calendar"))
  await page.getByRole("heading").first().waitFor()
  await say(page, "Deadlines for each form, moved off weekends and holidays.")

  await press(page, visibleLink(page, "Activity"))
  await page.getByRole("heading").first().waitFor()
  await say(page, "Everything done in your account is logged.")

  await press(page, visibleLink(page, "Settings"))
  await page.getByRole("heading", { name: "Settings" }).waitFor()
  await say(page, "Download all your data at any time, or close your account.")

  await page.goto("/help")
  await page.getByRole("heading", { name: "Help" }).waitFor()
  await say(page, "Kompleto never keeps books or files returns. It only chases documents.")
  await say(page, "Kompleto: approve, send, collect.", 4500)

  await stop(`${OUT}/walkthrough.mp4`)
  await context.close()
})
