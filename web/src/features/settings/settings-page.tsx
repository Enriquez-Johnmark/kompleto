import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"
import { DownloadIcon } from "lucide-react"
import { useState, type FormEvent } from "react"

import { api } from "@/api/client"
import type { Settings } from "@/api/generated/Settings"
import type { SettingsChange } from "@/api/generated/SettingsChange"
import { PageHeader } from "@/components/page-header"
import { QueryError } from "@/components/query-error"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldDescription, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Skeleton } from "@/components/ui/skeleton"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { useMe } from "@/app/queries"
import { CloseAccountCard } from "@/features/settings/close-account-card"
import { StaffCard } from "@/features/settings/staff-card"

const settingsKey = ["settings"] as const

export function SettingsPage() {
  const me = useMe()
  const isOwner = me.data?.role === "owner"
  const settings = useQuery({
    queryKey: settingsKey,
    queryFn: () => api<Settings>("GET", "/api/settings"),
  })
  return (
    <>
      <PageHeader title="Settings" />
      {settings.isPending || me.isPending ? <Skeleton className="h-96" /> : null}
      {settings.isError ? <QueryError error={settings.error} /> : null}
      {me.isError ? <QueryError error={me.error} /> : null}
      {settings.data && me.data ? (
        <div className="grid max-w-3xl gap-4">
          <FirmCard settings={settings.data} />
          <CutOffCard
            key={settings.data.cut_off_offset_days}
            days={settings.data.cut_off_offset_days}
          />
          {isOwner ? (
            <>
              <StaffCard />
              <ExportCard />
              <CloseAccountCard firmName={settings.data.firm_name} />
            </>
          ) : (
            <p className="text-sm text-muted-foreground">
              Only the firm's owner, {settings.data.login_email}, can change staff, download all
              data or close the account.
            </p>
          )}
        </div>
      ) : null}
    </>
  )
}

function FirmCard({ settings }: { settings: Settings }) {
  const { cost_usd, drafts } = settings.ai_cost
  return (
    <Card>
      <CardHeader>
        <CardTitle>{settings.firm_name}</CardTitle>
        <CardDescription>
          Owner: {settings.login_email}. Clients see your firm's name as the sender.
        </CardDescription>
      </CardHeader>
      <CardContent className="text-sm">
        AI wording this month: US${cost_usd.toFixed(4)} for {drafts}{" "}
        {drafts === 1 ? "draft" : "drafts"}.
      </CardContent>
    </Card>
  )
}

function CutOffCard({ days }: { days: number }) {
  const queryClient = useQueryClient()
  const [value, setValue] = useState(String(days))
  const save = useMutation({
    mutationFn: (change: SettingsChange) => api("PUT", "/api/settings", change),
    onSuccess: async () => {
      await queryClient.invalidateQueries()
      toast.add({ title: "Default cut-off saved", type: "success" })
    },
  })

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    save.mutate({ cut_off_offset_days: Number(value) })
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Default cut-off</CardTitle>
        <CardDescription>
          The day you want documents by, counted back from a period's earliest BIR due date.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <Field className="max-w-xs">
            <FieldLabel htmlFor="default-offset">Days before the due date</FieldLabel>
            <Input
              id="default-offset"
              type="number"
              min={0}
              max={30}
              value={value}
              onChange={(event) => setValue(event.target.value)}
            />
            <FieldDescription>0 to 30. Clients with their own setting keep it.</FieldDescription>
          </Field>
          {save.isError ? (
            <Alert variant="destructive">
              <AlertDescription>{save.error.message}</AlertDescription>
            </Alert>
          ) : null}
          <Button type="submit" className="self-start" disabled={save.isPending || value === ""}>
            {save.isPending ? <Spinner data-icon="inline-start" /> : null}
            Save
          </Button>
        </form>
      </CardContent>
    </Card>
  )
}

function ExportCard() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Your data</CardTitle>
        <CardDescription>
          One zip: spreadsheets of your clients, Payors, checklists, messages, uploads, replies and
          activity, plus the uploaded files still held.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Button
          variant="outline"
          nativeButton={false}
          render={<a href="/api/settings/export" download />}
        >
          <DownloadIcon data-icon="inline-start" />
          Download all data
        </Button>
      </CardContent>
    </Card>
  )
}
