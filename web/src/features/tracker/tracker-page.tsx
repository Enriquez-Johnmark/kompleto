import { useQueryClient } from "@tanstack/react-query"
import { ChevronLeftIcon, ChevronRightIcon, PlusIcon } from "lucide-react"
import { Link, useSearchParams } from "react-router"

import type { TrackedClient } from "@/api/generated/TrackedClient"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from "@/components/ui/empty"
import { Skeleton } from "@/components/ui/skeleton"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { useAddPayor } from "@/features/clients/queries"
import { trackerKeys, useTracker } from "@/features/tracker/queries"

const statusBadge = {
  received: { label: "Received", variant: "success" },
  uploaded: { label: "Uploaded, to check", variant: "warning" },
  missing: { label: "Missing", variant: "destructive" },
} as const

/** H2: each Client's expected 2307s for one quarter. */
export function TrackerPage() {
  const [params, setParams] = useSearchParams()
  const quarter = params.get("quarter")
  const tracker = useTracker(quarter)
  const data = tracker.data
  return (
    <>
      <PageHeader
        title={data ? `2307s for ${data.quarter}` : "2307 tracker"}
        description="Each client's Payors and whether their BIR Form 2307 has arrived."
      >
        {data ? (
          <>
            <Button variant="outline" onClick={() => setParams({ quarter: data.earlier })}>
              <ChevronLeftIcon data-icon="inline-start" />
              Earlier
            </Button>
            <Button
              variant="outline"
              disabled={!data.later}
              onClick={() => data.later && setParams({ quarter: data.later })}
            >
              Later
              <ChevronRightIcon data-icon="inline-end" />
            </Button>
          </>
        ) : null}
      </PageHeader>
      {tracker.isPending ? <Skeleton className="h-64" /> : null}
      {tracker.isError ? <QueryError error={tracker.error} /> : null}
      {data ? <TrackerTable clients={data.clients} /> : null}
    </>
  )
}

function TrackerTable({ clients }: { clients: TrackedClient[] }) {
  if (clients.length === 0) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyTitle>No clients yet</EmptyTitle>
          <EmptyDescription>List each client's Payors on the client's page.</EmptyDescription>
        </EmptyHeader>
      </Empty>
    )
  }
  const counts = { received: 0, uploaded: 0, missing: 0 }
  for (const client of clients) {
    for (const payor of client.payors) {
      if (payor.status in counts) counts[payor.status as keyof typeof counts] += 1
    }
  }
  return (
    <>
      <div className="flex flex-wrap gap-2">
        <Badge variant="success">{counts.received} received</Badge>
        <Badge variant="warning">{counts.uploaded} to check</Badge>
        <Badge variant="destructive">{counts.missing} missing</Badge>
      </div>
      <Card>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Client</TableHead>
                <TableHead>Payors</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {clients.map((client) => (
                <TableRow key={client.id} className="[&>td]:align-top">
                  <TableCell className="whitespace-normal">
                    <Link
                      to={`/clients/${client.id}?tab=documents`}
                      className="font-medium hover:underline"
                    >
                      {client.business_name}
                    </Link>
                  </TableCell>
                  <TableCell className="whitespace-normal">
                    <PayorList client={client} />
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </>
  )
}

function PayorList({ client }: { client: TrackedClient }) {
  const queryClient = useQueryClient()
  const add = useAddPayor(client.id)
  function handleAdd(name: string) {
    add.mutate(
      { name },
      { onSuccess: () => queryClient.invalidateQueries({ queryKey: trackerKeys.all() }) }
    )
  }
  if (client.payors.length === 0 && client.unexpected.length === 0) {
    return <span className="text-muted-foreground">No Payors listed</span>
  }
  return (
    <ul className="flex flex-col gap-1.5">
      {client.payors.map((payor) => {
        const badge = statusBadge[payor.status as keyof typeof statusBadge]
        return (
          <li key={payor.name} className="flex flex-wrap items-center gap-2">
            {payor.name}
            {badge ? <Badge variant={badge.variant}>{badge.label}</Badge> : null}
          </li>
        )
      })}
      {client.unexpected.map((name) => (
        <li key={name} className="flex flex-wrap items-center gap-2">
          {name}
          <Badge variant="outline">Not on the list</Badge>
          <Button
            variant="ghost"
            size="xs"
            onClick={() => handleAdd(name)}
            disabled={add.isPending}
          >
            <PlusIcon data-icon="inline-start" />
            Add to list
          </Button>
        </li>
      ))}
      {add.isError ? (
        <li>
          <Alert variant="destructive">
            <AlertDescription>{add.error.message}</AlertDescription>
          </Alert>
        </li>
      ) : null}
    </ul>
  )
}
