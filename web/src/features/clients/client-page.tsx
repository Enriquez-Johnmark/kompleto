import { PencilIcon } from "lucide-react"
import { Link, useParams, useSearchParams } from "react-router"

import type { ClientDetail } from "@/api/generated/ClientDetail"
import { NotFoundPage } from "@/app/not-found-page"
import { ApiError } from "@/api/client"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Skeleton } from "@/components/ui/skeleton"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { ChecklistCard } from "@/features/clients/checklist-card"
import { PayorsCard } from "@/features/clients/payors-card"
import { PeriodsTab } from "@/features/clients/periods-tab"
import { useClient } from "@/features/clients/queries"
import { RepliesCard } from "@/features/clients/replies-card"
import { ScheduleCard } from "@/features/clients/schedule-card"
import { UploadsCard } from "@/features/clients/uploads-card"

const tabs = ["deadlines", "documents", "uploads", "replies", "schedule"] as const

/** One Client: deadlines, documents, uploads, replies and schedule (B4). */
export function ClientPage() {
  const { id = "" } = useParams()
  const client = useClient(id)
  if (client.error instanceof ApiError && client.error.status === 404) return <NotFoundPage />
  return (
    <>
      {client.isPending ? <Skeleton className="h-96" /> : null}
      {client.isError ? <QueryError error={client.error} /> : null}
      {client.data ? <ClientContent client={client.data} /> : null}
    </>
  )
}

function ClientContent({ client }: { client: ClientDetail }) {
  // The open tab is in the address, so it survives a reload.
  const [params, setParams] = useSearchParams()
  const tab = tabs.find((name) => name === params.get("tab")) ?? "deadlines"
  return (
    <>
      <PageHeader
        title={client.business_name}
        description={
          <>
            {client.contact_first_name} · {client.email} ·{" "}
            {client.language === "taglish" ? "Taglish" : "English"}
          </>
        }
      >
        <Button
          variant="outline"
          render={<Link to={`/clients/${client.id}/edit`} />}
          nativeButton={false}
        >
          <PencilIcon data-icon="inline-start" />
          Edit details
        </Button>
      </PageHeader>
      <div className="flex flex-wrap gap-1">
        {client.paused ? <Badge variant="warning">Reminders paused</Badge> : null}
        {client.forms.map((form) => (
          <Badge key={form} variant="secondary">
            {form}
          </Badge>
        ))}
      </div>
      <Tabs
        value={tab}
        onValueChange={(value) => setParams({ tab: String(value) }, { replace: true })}
      >
        <TabsList className="max-w-full overflow-x-auto">
          <TabsTrigger value="deadlines">Deadlines</TabsTrigger>
          <TabsTrigger value="documents">Documents</TabsTrigger>
          <TabsTrigger value="uploads">Uploads ({client.uploads.length})</TabsTrigger>
          <TabsTrigger value="replies">Replies ({client.replies.length})</TabsTrigger>
          <TabsTrigger value="schedule">Schedule</TabsTrigger>
        </TabsList>
        <TabsContent value="deadlines" className="pt-4">
          <PeriodsTab clientId={client.id} periods={client.periods} />
        </TabsContent>
        <TabsContent value="documents" className="grid items-start gap-4 pt-4 lg:grid-cols-2">
          <ChecklistCard clientId={client.id} checklist={client.checklist} />
          <PayorsCard clientId={client.id} payors={client.payors} />
        </TabsContent>
        <TabsContent value="uploads" className="pt-4">
          <UploadsCard clientId={client.id} uploads={client.uploads} />
        </TabsContent>
        <TabsContent value="replies" className="pt-4">
          <RepliesCard replies={client.replies} />
        </TabsContent>
        <TabsContent value="schedule" className="pt-4">
          <ScheduleCard
            key={`${client.schedule.reminder_days.join()}-${client.paused}`}
            clientId={client.id}
            schedule={client.schedule}
            paused={client.paused}
          />
        </TabsContent>
      </Tabs>
    </>
  )
}
