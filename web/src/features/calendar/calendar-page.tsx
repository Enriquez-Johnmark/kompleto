import { useQuery } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { CalendarEntry } from "@/api/generated/CalendarEntry"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Skeleton } from "@/components/ui/skeleton"
import { formatDate } from "@/lib/dates"

const monthFormat = new Intl.DateTimeFormat("en-GB", {
  month: "long",
  year: "numeric",
  timeZone: "UTC",
})

/** "2026-11-05" → "November 2026". */
function monthOf(isoDate: string) {
  const [year, month] = isoDate.split("-").map(Number)
  return monthFormat.format(new Date(Date.UTC(year, month - 1, 1)))
}

/** B3: every BIR form's due date in the next 12 months. */
export function CalendarPage() {
  const calendar = useQuery({
    queryKey: ["calendar"],
    queryFn: () => api<CalendarEntry[]>("GET", "/api/calendar"),
    staleTime: 60 * 60 * 1000,
  })
  const months = new Map<string, CalendarEntry[]>()
  for (const entry of calendar.data ?? []) {
    const month = monthOf(entry.due)
    months.set(month, [...(months.get(month) ?? []), entry])
  }
  return (
    <>
      <PageHeader
        title="BIR calendar"
        description="Manual and eBIRForms due dates for the next 12 months, with weekends, holidays and BIR extensions applied. Set a client's own date on its page."
      />
      {calendar.isPending ? <Skeleton className="h-96" /> : null}
      {calendar.isError ? <QueryError error={calendar.error} /> : null}
      <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
        {[...months].map(([month, entries]) => (
          <Card key={month} size="sm">
            <CardHeader>
              <CardTitle>{month}</CardTitle>
            </CardHeader>
            <CardContent>
              <ul className="flex flex-col divide-y">
                {entries.map((entry) => (
                  <li key={`${entry.form}-${entry.period}`} className="flex flex-col gap-0.5 py-2">
                    <div className="flex items-center justify-between gap-2">
                      <span className="font-medium">{entry.form}</span>
                      <span className="text-sm tabular-nums">{formatDate(entry.due)}</span>
                    </div>
                    <div className="flex flex-wrap items-center gap-1 text-xs text-muted-foreground">
                      {entry.period}
                      {entry.extension ? (
                        <Badge variant="outline">Extended: {entry.extension}</Badge>
                      ) : null}
                      {entry.moved_from ? (
                        <span>· moved from {formatDate(entry.moved_from)}</span>
                      ) : null}
                    </div>
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>
        ))}
      </div>
    </>
  )
}
