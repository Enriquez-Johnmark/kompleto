import { ClockAlertIcon, FileUpIcon, UserPlusIcon } from "lucide-react"
import { Link } from "react-router"

import type { Dashboard } from "@/api/generated/Dashboard"
import type { DashboardClient } from "@/api/generated/DashboardClient"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/empty"
import { Skeleton } from "@/components/ui/skeleton"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { ClientFlags } from "@/features/dashboard/client-flags"
import { useDashboard } from "@/features/dashboard/queries"
import { StatCard } from "@/features/dashboard/stat-card"
import { formatDate, formatDay } from "@/lib/dates"
import { messageStatusLabel } from "@/lib/message-status"

/** H1: every Client with what's still missing, nearest cut-off first. */
export function DashboardPage() {
  const dashboard = useDashboard()
  return (
    <>
      <PageHeader
        title="Dashboard"
        description="What each client still owes, nearest cut-off first."
      />
      {dashboard.isPending ? <DashboardSkeleton /> : null}
      {dashboard.isError ? <QueryError error={dashboard.error} /> : null}
      {dashboard.data ? <DashboardContent dashboard={dashboard.data} /> : null}
    </>
  )
}

function DashboardContent({ dashboard }: { dashboard: Dashboard }) {
  const { clients, held } = dashboard
  if (clients.length === 0) return <NoClients />
  const draftsWaiting = clients.reduce((total, client) => total + client.drafts_waiting, 0)
  return (
    <>
      {held > 0 ? (
        <Alert variant="warning">
          <ClockAlertIcon />
          <AlertTitle>
            {held === 1
              ? "1 approved reminder hasn't gone out yet"
              : `${held} approved reminders haven't gone out yet`}
          </AlertTitle>
          <AlertDescription>
            Kompleto sends at most 90 reminders a day. The rest go out as the allowance frees up,
            nearest cut-off first.
          </AlertDescription>
        </Alert>
      ) : null}
      <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
        <StatCard label="Clients" value={clients.length} />
        <StatCard
          label="Still owe documents"
          value={clients.filter((c) => c.missing.length > 0).length}
        />
        <StatCard label="Drafts to approve" value={draftsWaiting} />
        <StatCard label="Replied" value={clients.filter((c) => c.replied).length} />
      </div>
      {draftsWaiting > 0 ? (
        <Button className="self-start" render={<Link to="/drafts" />} nativeButton={false}>
          Review {draftsWaiting} {draftsWaiting === 1 ? "draft" : "drafts"}
        </Button>
      ) : null}
      <Card className="hidden md:flex">
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Client</TableHead>
                <TableHead>Next cut-off</TableHead>
                <TableHead>Still missing</TableHead>
                <TableHead>Last message</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {clients.map((client) => (
                <TableRow key={client.id} className="[&>td]:align-top">
                  <TableCell className="flex flex-col gap-1 whitespace-normal">
                    <ClientLink client={client} />
                    <ClientFlags client={client} />
                  </TableCell>
                  <TableCell>
                    <CutOff client={client} />
                  </TableCell>
                  <TableCell className="whitespace-normal">
                    <Missing client={client} />
                  </TableCell>
                  <TableCell>
                    <LastMessage client={client} />
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
      <div className="flex flex-col gap-3 md:hidden">
        {clients.map((client) => (
          <Card key={client.id} size="sm">
            <CardContent className="flex flex-col gap-3">
              <div className="flex flex-col gap-1">
                <ClientLink client={client} />
                <ClientFlags client={client} />
              </div>
              <CutOff client={client} />
              <Missing client={client} />
              <LastMessage client={client} />
            </CardContent>
          </Card>
        ))}
      </div>
    </>
  )
}

function ClientLink({ client }: { client: DashboardClient }) {
  return (
    <Link to={`/clients/${client.id}`} className="font-medium hover:underline">
      {client.business_name}
    </Link>
  )
}

function CutOff({ client }: { client: DashboardClient }) {
  if (!client.cut_off) return <span className="text-muted-foreground">Nothing due</span>
  return (
    <div className="flex flex-col">
      <span>{formatDate(client.cut_off)}</span>
      <span className="text-xs text-muted-foreground">{client.period}</span>
    </div>
  )
}

function Missing({ client }: { client: DashboardClient }) {
  if (!client.cut_off) return null
  if (client.missing.length === 0) {
    return <span className="text-success">Everything received</span>
  }
  return (
    <ul className="flex flex-col gap-0.5">
      {client.missing.map((document) => (
        <li key={document}>{document}</li>
      ))}
    </ul>
  )
}

function LastMessage({ client }: { client: DashboardClient }) {
  const last = client.last_message
  return (
    <div className="flex flex-col gap-0.5">
      {last ? (
        <span>
          {messageStatusLabel(last.status)}{" "}
          <span className="text-muted-foreground">{formatDay(last.at)}</span>
        </span>
      ) : (
        <span className="text-muted-foreground">No messages yet</span>
      )}
      {client.drafts_waiting > 0 ? (
        <Link to="/drafts" className="text-xs text-primary hover:underline">
          {client.drafts_waiting} {client.drafts_waiting === 1 ? "draft" : "drafts"} waiting
        </Link>
      ) : null}
    </div>
  )
}

function NoClients() {
  return (
    <Empty className="border">
      <EmptyHeader>
        <EmptyTitle>No clients yet</EmptyTitle>
        <EmptyDescription>
          Add your clients and the BIR forms each one files. Kompleto works out what they owe and
          when.
        </EmptyDescription>
      </EmptyHeader>
      <EmptyContent className="flex-row justify-center">
        <Button render={<Link to="/clients/new" />} nativeButton={false}>
          <UserPlusIcon data-icon="inline-start" />
          Add a client
        </Button>
        <Button variant="outline" render={<Link to="/import" />} nativeButton={false}>
          <FileUpIcon data-icon="inline-start" />
          Import a spreadsheet
        </Button>
      </EmptyContent>
    </Empty>
  )
}

function DashboardSkeleton() {
  return (
    <div className="flex flex-col gap-3">
      <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
        {[0, 1, 2, 3].map((i) => (
          <Skeleton key={i} className="h-20" />
        ))}
      </div>
      <Skeleton className="h-64" />
    </div>
  )
}
