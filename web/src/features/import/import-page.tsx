import { useMutation } from "@tanstack/react-query"
import { CircleAlertIcon, DownloadIcon, FileSpreadsheetIcon } from "lucide-react"
import { useState, type ChangeEvent } from "react"
import { useNavigate } from "react-router"

import { api, ApiError, apiUpload } from "@/api/client"
import type { ImportConfirmation } from "@/api/generated/ImportConfirmation"
import type { Imported } from "@/api/generated/Imported"
import type { ImportPreview } from "@/api/generated/ImportPreview"
import { PageHeader } from "@/components/page-header"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Spinner } from "@/components/ui/spinner"
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { toast } from "@/components/ui/toast"
import { useRefreshClients } from "@/features/clients/queries"
import { BatchDefaultsCard, type BatchDefaults } from "@/features/import/batch-defaults-card"

/** B2, B5: upload a file (Kompleto's template, or a Xero or QuickBooks
 *  export), see every row checked, then add the valid ones. */
export function ImportPage() {
  const navigate = useNavigate()
  const refresh = useRefreshClients()
  const [fileName, setFileName] = useState("")
  const [defaults, setDefaults] = useState<BatchDefaults>({
    enabled: false,
    language: "taglish",
    forms: [],
  })
  const check = useMutation({
    mutationFn: (file: File) =>
      apiUpload<ImportPreview>(
        "/api/import",
        file,
        defaults.enabled
          ? [
              ["default_language", defaults.language],
              ...defaults.forms.map((form): [string, string] => ["default_forms", form]),
            ]
          : []
      ),
  })

  // A preview made with other choices would add the wrong values.
  function handleDefaults(changed: BatchDefaults) {
    setDefaults(changed)
    check.reset()
    setFileName("")
  }
  const confirm = useMutation({
    mutationFn: (confirmation: ImportConfirmation) =>
      api<Imported>("POST", "/api/import/confirm", confirmation),
    onSuccess: async ({ added }) => {
      await refresh()
      toast.add({ title: `${added} ${added === 1 ? "client" : "clients"} added`, type: "success" })
      navigate("/clients")
    },
  })

  function handleFile(event: ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0]
    if (!file) return
    setFileName(file.name)
    confirm.reset()
    check.mutate(file)
    event.target.value = ""
  }

  const preview = check.data
  return (
    <>
      <PageHeader
        title="Import clients"
        description="Upload Kompleto's template, or a client list exported from Xero or QuickBooks: .csv or .xlsx, up to 500 clients. Nothing is added until you confirm."
      >
        <Button
          variant="outline"
          nativeButton={false}
          render={<a href="/api/import/template.csv" download />}
        >
          <DownloadIcon data-icon="inline-start" />
          Download the template
        </Button>
      </PageHeader>
      <BatchDefaultsCard defaults={defaults} onChange={handleDefaults} />
      <Card>
        <CardHeader>
          <CardTitle>Choose a file</CardTitle>
          <CardDescription>
            Columns: Business name, Contact first name, Email, Language (Taglish or English) and
            Forms (e.g. "2551Q, 1701Q").
          </CardDescription>
        </CardHeader>
        <CardContent>
          <label className="flex cursor-pointer flex-col items-center gap-2 rounded-lg border border-dashed p-8 text-center hover:bg-muted/50">
            {check.isPending ? (
              <Spinner className="size-6" />
            ) : (
              <FileSpreadsheetIcon className="size-8 text-muted-foreground" />
            )}
            <span className="font-medium">{fileName || "Choose a spreadsheet"}</span>
            <span className="text-sm text-muted-foreground">CSV or Excel, up to 2 MB</span>
            <input
              type="file"
              accept=".csv,.xlsx"
              className="sr-only"
              onChange={handleFile}
              disabled={check.isPending}
            />
          </label>
        </CardContent>
      </Card>
      {check.isError ? (
        <Alert variant="destructive">
          <CircleAlertIcon />
          <AlertTitle>This file can't be imported</AlertTitle>
          <AlertDescription>{check.error.message}</AlertDescription>
        </Alert>
      ) : null}
      {preview ? (
        <PreviewResult
          preview={preview}
          isAdding={confirm.isPending}
          error={confirm.error}
          onConfirm={() => confirm.mutate({ rows: preview.rows })}
        />
      ) : null}
    </>
  )
}

type PreviewResultProps = {
  preview: ImportPreview
  isAdding: boolean
  error: Error | null
  onConfirm: () => void
}

function PreviewResult({ preview, isAdding, error, onConfirm }: PreviewResultProps) {
  const count = preview.valid.length
  return (
    <>
      {preview.invalid.length > 0 ? (
        <Alert variant="warning">
          <CircleAlertIcon />
          <AlertTitle>
            {preview.invalid.length} {preview.invalid.length === 1 ? "row" : "rows"} won't be added
          </AlertTitle>
          <AlertDescription>
            <ul className="flex flex-col gap-1">
              {preview.invalid.map((row) => (
                <li key={row.row}>
                  <span className="font-medium">Row {row.row}:</span> {row.reasons.join("; ")}
                </li>
              ))}
            </ul>
          </AlertDescription>
        </Alert>
      ) : null}
      <Card>
        <CardHeader>
          <CardTitle>
            {count} {count === 1 ? "client" : "clients"} ready to add
          </CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          {count > 0 ? (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>Row</TableHead>
                  <TableHead>Client</TableHead>
                  <TableHead className="hidden sm:table-cell">Contact</TableHead>
                  <TableHead>Forms</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {preview.valid.map((row) => (
                  <TableRow key={row.row}>
                    <TableCell className="text-muted-foreground">{row.row}</TableCell>
                    <TableCell className="font-medium whitespace-normal">
                      {row.business_name}
                    </TableCell>
                    <TableCell className="hidden sm:table-cell">
                      {row.contact_first_name} · {row.email}
                    </TableCell>
                    <TableCell className="whitespace-normal">
                      <div className="flex flex-wrap gap-1">
                        {row.forms.map((form) => (
                          <Badge key={form} variant="secondary">
                            {form}
                          </Badge>
                        ))}
                      </div>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          ) : null}
          {error ? (
            <Alert variant="destructive">
              <AlertTitle>{error.message}</AlertTitle>
              {error instanceof ApiError && error.problems.length > 0 ? (
                <AlertDescription>
                  <ul className="flex flex-col gap-1">
                    {error.problems.map((problem) => (
                      <li key={problem}>{problem}</li>
                    ))}
                  </ul>
                </AlertDescription>
              ) : null}
            </Alert>
          ) : null}
          <Button className="self-start" onClick={onConfirm} disabled={count === 0 || isAdding}>
            {isAdding ? <Spinner data-icon="inline-start" /> : null}
            Add {count} {count === 1 ? "client" : "clients"}
          </Button>
        </CardContent>
      </Card>
    </>
  )
}
