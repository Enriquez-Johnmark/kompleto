import { useQuery } from "@tanstack/react-query"
import { ShieldAlertIcon } from "lucide-react"
import { Link, useParams } from "react-router"

import { api } from "@/api/client"
import type { ReplyDetail } from "@/api/generated/ReplyDetail"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { formatDateTime } from "@/lib/dates"

/** G1: one Reply. The Contact's text is untrusted and shown as plain text. */
export function ReplyPage() {
  const { id = "" } = useParams()
  const reply = useQuery({
    queryKey: ["replies", id],
    queryFn: () => api<ReplyDetail>("GET", `/api/replies/${id}`),
  })
  return (
    <>
      {reply.isPending ? <Skeleton className="h-64" /> : null}
      {reply.isError ? <QueryError error={reply.error} /> : null}
      {reply.data ? <ReplyContent reply={reply.data} /> : null}
    </>
  )
}

function ReplyContent({ reply }: { reply: ReplyDetail }) {
  return (
    <>
      <PageHeader
        title={`Reply from ${reply.business_name}`}
        description={`About ${reply.period} · ${formatDateTime(reply.received_at)}`}
      />
      {reply.signed ? null : (
        <Alert variant="warning">
          <ShieldAlertIcon />
          <AlertTitle>Matched by the sender's address only</AlertTitle>
          <AlertDescription>
            Anyone can put an address in From, so this reply didn't stop the reminders. Check with
            the client if it looks unusual.
          </AlertDescription>
        </Alert>
      )}
      <Card>
        <CardHeader className="flex flex-row flex-wrap items-center gap-2">
          <CardTitle>{reply.subject ?? "(no subject)"}</CardTitle>
          {reply.automatic ? <Badge variant="secondary">Automatic reply</Badge> : null}
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          <p className="text-sm text-muted-foreground">From {reply.from}</p>
          {reply.text === null ? (
            <p className="text-sm text-muted-foreground">
              The text was deleted after 90 days; only the date is kept.
            </p>
          ) : (
            <p className="text-sm break-words whitespace-pre-wrap">{reply.text}</p>
          )}
          {reply.attachments > 0 ? (
            <p className="text-sm">
              {reply.attachments} {reply.attachments === 1 ? "attachment was" : "attachments were"}{" "}
              kept as uploads.{" "}
              <Link
                to={`/clients/${reply.client_id}?tab=uploads`}
                className="text-primary underline"
              >
                See them
              </Link>
            </p>
          ) : null}
          <Link to={`/clients/${reply.client_id}`} className="text-sm text-primary hover:underline">
            Back to {reply.business_name}
          </Link>
        </CardContent>
      </Card>
    </>
  )
}
