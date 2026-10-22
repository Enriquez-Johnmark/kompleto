import { FileUpIcon, SearchIcon, UserPlusIcon } from "lucide-react"
import { Link, useSearchParams } from "react-router"

import type { ClientSummary } from "@/api/generated/ClientSummary"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent } from "@/components/ui/card"
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyTitle,
} from "@/components/ui/empty"
import { Input } from "@/components/ui/input"
import { Skeleton } from "@/components/ui/skeleton"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { useClients } from "@/features/clients/queries"

export function ClientsPage() {
  const clients = useClients()
  return (
    <>
      <PageHeader title="Clients" description="Each client's contact, language and BIR forms.">
        <Button variant="outline" render={<Link to="/import" />} nativeButton={false}>
          <FileUpIcon data-icon="inline-start" />
          Import
        </Button>
        <Button render={<Link to="/clients/new" />} nativeButton={false}>
          <UserPlusIcon data-icon="inline-start" />
          Add a client
        </Button>
      </PageHeader>
      {clients.isPending ? <Skeleton className="h-64" /> : null}
      {clients.isError ? <QueryError error={clients.error} /> : null}
      {clients.data ? <ClientList clients={clients.data} /> : null}
    </>
  )
}

function ClientList({ clients }: { clients: ClientSummary[] }) {
  // The search is in the address, so a filtered list can be reloaded.
  const [params, setParams] = useSearchParams()
  const query = params.get("q") ?? ""
  const needle = query.trim().toLowerCase()
  const shown = needle
    ? clients.filter((client) =>
        [client.business_name, client.contact_first_name, client.email].some((text) =>
          text.toLowerCase().includes(needle)
        )
      )
    : clients

  if (clients.length === 0) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyTitle>No clients yet</EmptyTitle>
          <EmptyDescription>Add them one at a time, or import a spreadsheet.</EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
          <Button render={<Link to="/clients/new" />} nativeButton={false}>
            Add a client
          </Button>
        </EmptyContent>
      </Empty>
    )
  }

  return (
    <Card>
      <CardContent className="flex flex-col gap-4">
        <div className="relative max-w-sm">
          <SearchIcon className="pointer-events-none absolute top-1/2 left-2.5 size-4 -translate-y-1/2 text-muted-foreground" />
          <Input
            type="search"
            placeholder="Search clients"
            aria-label="Search clients"
            className="pl-8"
            value={query}
            onChange={(event) =>
              setParams(event.target.value ? { q: event.target.value } : {}, { replace: true })
            }
          />
        </div>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Client</TableHead>
              <TableHead className="hidden sm:table-cell">Contact</TableHead>
              <TableHead className="hidden md:table-cell">Language</TableHead>
              <TableHead>Forms</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {shown.map((client) => (
              <TableRow key={client.id}>
                <TableCell className="whitespace-normal">
                  <Link to={`/clients/${client.id}`} className="font-medium hover:underline">
                    {client.business_name}
                  </Link>
                  {client.paused ? (
                    <Badge variant="outline" className="ml-2">
                      Paused
                    </Badge>
                  ) : null}
                </TableCell>
                <TableCell className="hidden sm:table-cell">
                  <div className="flex flex-col">
                    <span>{client.contact_first_name}</span>
                    <span className="text-xs text-muted-foreground">{client.email}</span>
                  </div>
                </TableCell>
                <TableCell className="hidden md:table-cell">
                  {client.language === "taglish" ? "Taglish" : "English"}
                </TableCell>
                <TableCell className="whitespace-normal">
                  <div className="flex flex-wrap gap-1">
                    {client.forms.map((form) => (
                      <Badge key={form} variant="secondary">
                        {form}
                      </Badge>
                    ))}
                  </div>
                </TableCell>
              </TableRow>
            ))}
            {shown.length === 0 ? (
              <TableRow>
                <TableCell colSpan={4} className="text-center text-muted-foreground">
                  No client matches "{query}".
                </TableCell>
              </TableRow>
            ) : null}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  )
}
