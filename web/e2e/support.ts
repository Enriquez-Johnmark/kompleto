import { execFileSync } from "node:child_process"
import { resolve } from "node:path"

const mailpit = process.env.MAILPIT_URL ?? "http://localhost:8025"

/** The `kompleto` binary, to set up data as an operator would. */
export function kompleto(...args: string[]) {
  const repository = resolve(import.meta.dirname, "../..")
  const binary = process.env.KOMPLETO_BIN ?? resolve(repository, "target/debug/kompleto")
  return execFileSync(binary, args, {
    cwd: repository,
    encoding: "utf8",
  })
}

/** The login link in the newest email to `to`, waiting for it to arrive. */
export async function loginLink(to: string): Promise<string> {
  for (let attempt = 0; attempt < 50; attempt++) {
    const search = await fetch(
      `${mailpit}/api/v1/search?query=${encodeURIComponent(`to:${to}`)}&limit=1`
    )
    const found = (await search.json()) as { messages?: { ID: string }[] }
    const id = found.messages?.[0]?.ID
    if (id) {
      const message = (await (await fetch(`${mailpit}/api/v1/message/${id}`)).json()) as {
        Text: string
      }
      const link = message.Text.match(/https?:\/\/\S+\/login\/[0-9a-f]+/)?.[0]
      if (link) return link
    }
    await new Promise((resolve) => setTimeout(resolve, 200))
  }
  throw new Error(`no login email to ${to}`)
}

/** `YYYY-MM-DD` for the first day of next month, in Manila. */
export function firstOfNextMonth() {
  const today = new Intl.DateTimeFormat("en-CA", { timeZone: "Asia/Manila" }).format(new Date())
  const [year, month] = today.split("-").map(Number)
  const next = month === 12 ? [year + 1, 1] : [year, month + 1]
  return `${next[0]}-${String(next[1]).padStart(2, "0")}-01`
}
