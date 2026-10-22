import type { DashboardClient } from "@/api/generated/DashboardClient"
import { Badge } from "@/components/ui/badge"

/** The badges beside a Client's name: replies, bounces, paused. */
export function ClientFlags({ client }: { client: DashboardClient }) {
  return (
    <div className="flex flex-wrap gap-1">
      {client.replied ? <Badge variant="success">Replied</Badge> : null}
      {!client.replied && client.auto_replied ? (
        <Badge variant="secondary">Auto-reply</Badge>
      ) : null}
      {client.bouncing ? <Badge variant="destructive">Email bounced</Badge> : null}
      {!client.bouncing && client.soft_bounced ? (
        <Badge variant="warning">Temporary bounce</Badge>
      ) : null}
      {client.paused ? <Badge variant="outline">Paused</Badge> : null}
    </div>
  )
}
