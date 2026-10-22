import { useQuery } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { ActivityLog } from "@/api/generated/ActivityLog"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Card, CardContent } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { formatDateTime } from "@/lib/dates"

/** J2: every approval, edit, send, reply and deletion lately. */
export function ActivityPage() {
  const log = useQuery({
    queryKey: ["activity"],
    queryFn: () => api<ActivityLog>("GET", "/api/activity"),
  })
  return (
    <>
      <PageHeader
        title="Activity"
        description={`What happened to your records in the last ${log.data?.days ?? 90} days, newest first.`}
      />
      {log.isPending ? <Skeleton className="h-96" /> : null}
      {log.isError ? <QueryError error={log.error} /> : null}
      {log.data ? (
        <Card>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>When</TableHead>
                  <TableHead>Who</TableHead>
                  <TableHead>What</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {log.data.entries.map((entry, index) => (
                  // Entries have no ID of their own; the list never reorders.
                  <TableRow key={`${entry.at}-${index}`}>
                    <TableCell className="text-muted-foreground tabular-nums">
                      {formatDateTime(entry.at)}
                    </TableCell>
                    <TableCell>{entry.who}</TableCell>
                    <TableCell className="whitespace-normal">{entry.what}</TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>
      ) : null}
    </>
  )
}
