import { useState } from "react"

import type { PeriodView } from "@/api/generated/PeriodView"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from "@/components/ui/empty"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { DueDateDialog } from "@/features/clients/due-date-dialog"
import { formatDate } from "@/lib/dates"

type PeriodsTabProps = { clientId: string; periods: PeriodView[] }

/** B4: the next 12 months, each Period with its Due dates, Cut-off and
 *  Owed documents. */
export function PeriodsTab({ clientId, periods }: PeriodsTabProps) {
  const [showAll, setShowAll] = useState(false)
  const shown = showAll ? periods : periods.slice(0, 3)
  if (periods.length === 0) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyTitle>Nothing due in the next 12 months</EmptyTitle>
          <EmptyDescription>
            Tick the BIR forms this client files to see its deadlines.
          </EmptyDescription>
        </EmptyHeader>
      </Empty>
    )
  }
  return (
    <div className="flex flex-col gap-4">
      {shown.map((period) => (
        <Card key={period.label}>
          <CardHeader>
            <CardTitle>{period.label}</CardTitle>
            <CardDescription>
              Cut-off{" "}
              <span className="font-medium text-foreground">{formatDate(period.cut_off)}</span>
              {period.draft_dates.length > 0
                ? ` · Reminders drafted on ${period.draft_dates.map(formatDate).join(", ")}`
                : null}
            </CardDescription>
          </CardHeader>
          <CardContent className="grid gap-6 lg:grid-cols-2">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Form</TableHead>
                  <TableHead>Due</TableHead>
                  <TableHead className="w-0" />
                </TableRow>
              </TableHeader>
              <TableBody>
                {period.filings.map((filing) => (
                  <TableRow key={`${filing.form}-${filing.period_key}`}>
                    <TableCell>
                      <div className="flex flex-col">
                        <span className="font-medium">{filing.form}</span>
                        <span className="text-xs text-muted-foreground">{filing.period}</span>
                      </div>
                    </TableCell>
                    <TableCell className="whitespace-normal">
                      <div className="flex flex-wrap items-center gap-1">
                        {formatDate(filing.due)}
                        {filing.overridden ? <Badge variant="secondary">Your date</Badge> : null}
                        {filing.extension ? <Badge variant="outline">Extended</Badge> : null}
                      </div>
                    </TableCell>
                    <TableCell>
                      <DueDateDialog clientId={clientId} filing={filing} />
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
            <div className="flex flex-col gap-2">
              <h3 className="text-sm font-medium">Documents to ask for</h3>
              <ul className="flex list-disc flex-col gap-1 pl-4 text-sm">
                {period.owed.map((document) => (
                  <li key={document}>{document}</li>
                ))}
              </ul>
            </div>
          </CardContent>
        </Card>
      ))}
      {shown.length < periods.length ? (
        <Button variant="outline" className="self-center" onClick={() => setShowAll(true)}>
          Show all {periods.length} periods in the next 12 months
        </Button>
      ) : null}
    </div>
  )
}
