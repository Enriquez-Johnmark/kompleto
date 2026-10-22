import { CopyIcon } from "lucide-react"

import { Button } from "@/components/ui/button"
import { toast } from "@/components/ui/toast"

/** E3: copies the message text, upload link included, to paste into the
 *  bookkeeper's own Viber chat. It approves and sends nothing. */
export function CopyForViberButton({ text }: { text: string }) {
  async function handleCopy() {
    try {
      await navigator.clipboard.writeText(text)
      toast.add({ title: "Copied", description: "Paste it into your Viber chat.", type: "success" })
    } catch {
      toast.add({
        title: "Couldn't copy",
        description: "Select the text and copy it yourself.",
        type: "error",
      })
    }
  }
  return (
    <Button variant="outline" size="sm" onClick={handleCopy}>
      <CopyIcon data-icon="inline-start" />
      Copy for Viber
    </Button>
  )
}
