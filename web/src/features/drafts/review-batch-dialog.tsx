import { CheckCheckIcon } from "lucide-react"
import { useState } from "react"

import type { BatchResult } from "@/api/generated/BatchResult"
import type { DraftView } from "@/api/generated/DraftView"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { DraftHeading } from "@/features/drafts/draft-card"
import { MessageText } from "@/features/drafts/message-text"
import { useApproveBatch } from "@/features/drafts/queries"

type ReviewBatchDialogProps = {
  drafts: DraftView[]
  /** Called once the batch is approved, to clear the selection. */
  onApproved: () => void
}

/** D3: every selected Draft in full, before any of them is approved. Each
 *  approval is bound to the text shown here. */
export function ReviewBatchDialog({ drafts, onApproved }: ReviewBatchDialogProps) {
  const [isOpen, setIsOpen] = useState(false)
  const [result, setResult] = useState<BatchResult | null>(null)
  // The drafts as they were when the review opened. A refresh of the list
  // while reading never changes the text being reviewed, or what's approved:
  // a draft edited meanwhile is refused by its hash instead.
  const [reviewed, setReviewed] = useState<DraftView[]>([])
  const approve = useApproveBatch()
  const count = isOpen ? reviewed.length : drafts.length

  function handleOpenChange(open: boolean) {
    setIsOpen(open)
    if (open) setReviewed(drafts)
    else {
      setResult(null)
      approve.reset()
    }
  }

  function handleApprove() {
    approve.mutate(reviewed, {
      onSuccess: (outcome) => {
        onApproved()
        toast.add({
          title: `${outcome.approved} approved`,
          description: "They go out within a minute.",
          type: "success",
        })
        if (outcome.refused.length === 0) setIsOpen(false)
        else setResult(outcome)
      },
    })
  }

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogTrigger render={<Button disabled={count === 0} />}>
        <CheckCheckIcon data-icon="inline-start" />
        Review {count} selected
      </DialogTrigger>
      <DialogContent className="flex max-h-[90svh] flex-col sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>Review {count === 1 ? "1 draft" : `${count} drafts`}</DialogTitle>
          <DialogDescription>
            Read each message as it will be sent. Nothing is approved until you press the button
            below.
          </DialogDescription>
        </DialogHeader>
        {result ? (
          <Alert variant="warning">
            <AlertTitle>
              {result.refused.length === 1
                ? "1 draft wasn't approved"
                : `${result.refused.length} drafts weren't approved`}
            </AlertTitle>
            <AlertDescription>
              <ul className="flex flex-col gap-1">
                {result.refused.map((refusal) => (
                  <li key={refusal.id}>
                    {refusal.client}: {refusal.reason}
                  </li>
                ))}
              </ul>
            </AlertDescription>
          </Alert>
        ) : (
          <ol className="-mx-1 flex flex-col gap-4 overflow-y-auto px-1">
            {reviewed.map((draft) => (
              <li key={draft.id} className="flex flex-col gap-2">
                <DraftHeading draft={draft} />
                <MessageText subject={draft.subject} body={draft.body} />
              </li>
            ))}
          </ol>
        )}
        {approve.isError ? (
          <Alert variant="destructive">
            <AlertDescription>{approve.error.message}</AlertDescription>
          </Alert>
        ) : null}
        <DialogFooter>
          <DialogClose render={<Button variant="outline" />}>
            {result ? "Close" : "Cancel"}
          </DialogClose>
          {result ? null : (
            <Button onClick={handleApprove} disabled={approve.isPending}>
              {approve.isPending ? <Spinner data-icon="inline-start" /> : null}
              Approve {count === 1 ? "this draft" : `these ${count}`}
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
