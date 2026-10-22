import { CheckIcon, PencilIcon, Trash2Icon } from "lucide-react"

import type { DraftView } from "@/api/generated/DraftView"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card"
import { Checkbox } from "@/components/ui/checkbox"
import { Spinner } from "@/components/ui/spinner"
import { CopyForViberButton } from "@/features/drafts/copy-for-viber-button"
import { EditDraftDialog } from "@/features/drafts/edit-draft-dialog"
import { MessageText } from "@/features/drafts/message-text"
import { useApprove, useDiscard } from "@/features/drafts/queries"
import { formatDate } from "@/lib/dates"

type DraftCardProps = {
  draft: DraftView
  isSelected: boolean
  onSelectedChange: (selected: boolean) => void
}

/** One Draft waiting for approval, shown in full (D2). */
export function DraftCard({ draft, isSelected, onSelectedChange }: DraftCardProps) {
  const approve = useApprove()
  const discard = useDiscard()
  const error = approve.error ?? discard.error
  return (
    <Card>
      <CardHeader className="flex flex-row flex-wrap items-start gap-3">
        <Checkbox
          checked={isSelected}
          onCheckedChange={(checked) => onSelectedChange(checked === true)}
          aria-label={`Select the draft to ${draft.client}`}
          className="mt-0.5"
        />
        <DraftHeading draft={draft} />
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        <MessageText subject={draft.subject} body={draft.body} />
        {error ? (
          <Alert variant="destructive">
            <AlertDescription>{error.message}</AlertDescription>
          </Alert>
        ) : null}
      </CardContent>
      <CardFooter className="flex flex-wrap gap-2">
        <Button onClick={() => approve.mutate(draft)} disabled={approve.isPending}>
          {approve.isPending ? (
            <Spinner data-icon="inline-start" />
          ) : (
            <CheckIcon data-icon="inline-start" />
          )}
          Approve
        </Button>
        <EditDraftDialog draft={draft}>
          <PencilIcon data-icon="inline-start" />
          Edit
        </EditDraftDialog>
        <AlertDialog>
          <AlertDialogTrigger render={<Button variant="ghost" />}>
            <Trash2Icon data-icon="inline-start" />
            Discard
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Discard this draft?</AlertDialogTitle>
              <AlertDialogDescription>
                The reminder to {draft.client} won't be sent. Later reminders still come on
                schedule.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Keep it</AlertDialogCancel>
              <AlertDialogAction variant="destructive" onClick={() => discard.mutate(draft)}>
                Discard
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
        <div className="ml-auto">
          <CopyForViberButton text={draft.body} />
        </div>
      </CardFooter>
    </Card>
  )
}

/** Who a message goes to, who wrote it and its cut-off. */
export function DraftHeading({ draft }: { draft: DraftView }) {
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-1">
      <div className="flex flex-wrap items-center gap-2">
        <span className="font-medium">{draft.client}</span>
        <Badge variant="outline">{draft.source === "ai" ? "AI" : "Template"}</Badge>
      </div>
      <p className="text-sm text-muted-foreground">
        To {draft.to}
        {draft.cut_off ? ` · Cut-off ${formatDate(draft.cut_off)}` : null}
      </p>
    </div>
  )
}
