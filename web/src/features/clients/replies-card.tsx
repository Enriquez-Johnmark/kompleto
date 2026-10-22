import { Link } from "react-router"

import type { ReplySummary } from "@/api/generated/ReplySummary"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from "@/components/ui/empty"
import { formatDay } from "@/lib/dates"

/** G1: what the Contact wrote back. */
export function RepliesCard({ replies }: { replies: ReplySummary[] }) {
  if (replies.length === 0) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyTitle>No replies yet</EmptyTitle>
          <EmptyDescription>
            When the client answers a reminder, their reply appears here and their reminders for
            that period stop.
          </EmptyDescription>
        </EmptyHeader>
      </Empty>
    )
  }
  return (
    <Card>
      <CardHeader>
        <CardTitle>Replies</CardTitle>
      </CardHeader>
      <CardContent>
        <ul className="flex flex-col divide-y">
          {replies.map((reply) => (
            <li key={reply.id} className="flex flex-wrap items-center justify-between gap-2 py-3">
              <Link to={`/replies/${reply.id}`} className="text-sm font-medium hover:underline">
                {reply.subject || "(no subject)"}
              </Link>
              <span className="flex items-center gap-2 text-xs text-muted-foreground">
                {reply.automatic ? <Badge variant="secondary">Automatic</Badge> : null}
                {reply.period} · {formatDay(reply.received_at)}
              </span>
            </li>
          ))}
        </ul>
      </CardContent>
    </Card>
  )
}
