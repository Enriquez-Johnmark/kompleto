import type { DraftView } from "@/api/generated/DraftView"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card"
import { CopyForViberButton } from "@/features/drafts/copy-for-viber-button"
import { DraftHeading } from "@/features/drafts/draft-card"
import { messageStatusLabel } from "@/lib/message-status"

const troubled = new Set(["failed", "blocked", "unconfirmed"])

/** Reminders approved in the last week, with how each is going (D7). */
export function ApprovedList({ drafts }: { drafts: DraftView[] }) {
  return (
    <section className="flex flex-col gap-3">
      <h2 className="text-lg font-semibold">Approved this week</h2>
      {drafts.map((draft) => (
        <Card key={draft.id} size="sm">
          <CardHeader className="flex flex-row flex-wrap items-start gap-3">
            <DraftHeading draft={draft} />
            <Badge
              variant={
                draft.status === "sent"
                  ? "success"
                  : troubled.has(draft.status)
                    ? "destructive"
                    : "secondary"
              }
            >
              {messageStatusLabel(draft.status)}
            </Badge>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            <p className="font-medium">{draft.subject}</p>
            {draft.problem ? (
              <Alert variant="destructive">
                <AlertDescription>Not sent: {draft.problem}</AlertDescription>
              </Alert>
            ) : null}
          </CardContent>
          <CardFooter>
            <CopyForViberButton text={draft.body} />
          </CardFooter>
        </Card>
      ))}
    </section>
  )
}
