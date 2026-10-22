import { CheckIcon, DownloadIcon } from "lucide-react"

import type { UploadView } from "@/api/generated/UploadView"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from "@/components/ui/empty"
import { useMarkReceived } from "@/features/clients/queries"
import { formatDay } from "@/lib/dates"

type UploadsCardProps = { clientId: string; uploads: UploadView[] }

/** F3: what the Contact sent. Files are downloaded, never opened in the
 *  browser, and deleted 30 days after being marked received. */
export function UploadsCard({ clientId, uploads }: UploadsCardProps) {
  const markReceived = useMarkReceived(clientId)
  if (uploads.length === 0) {
    return (
      <Empty className="border">
        <EmptyHeader>
          <EmptyTitle>Nothing uploaded yet</EmptyTitle>
          <EmptyDescription>
            Files the client sends through their upload link or by email appear here.
          </EmptyDescription>
        </EmptyHeader>
      </Empty>
    )
  }
  return (
    <Card>
      <CardHeader>
        <CardTitle>Uploads</CardTitle>
        <CardDescription>
          Kompleto isn't an archive: each file is deleted 30 days after you mark it received, and
          after 90 days at most.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        {markReceived.isError ? (
          <Alert variant="destructive">
            <AlertDescription>{markReceived.error.message}</AlertDescription>
          </Alert>
        ) : null}
        <ul className="flex flex-col divide-y">
          {uploads.map((upload) => (
            <li key={upload.id} className="flex flex-wrap items-center justify-between gap-3 py-3">
              <div className="flex min-w-0 flex-col gap-0.5">
                <span className="text-sm font-medium">{upload.document}</span>
                {upload.detail ? (
                  <span className="text-xs text-muted-foreground">{upload.detail}</span>
                ) : null}
                <span className="text-xs text-muted-foreground">
                  {upload.period} · uploaded {formatDay(upload.uploaded_at)} ·{" "}
                  {Math.ceil(upload.size_bytes / 1024)} KB
                </span>
              </div>
              <UploadActions clientId={clientId} upload={upload} onReceived={markReceived.mutate} />
            </li>
          ))}
        </ul>
      </CardContent>
    </Card>
  )
}

type UploadActionsProps = {
  clientId: string
  upload: UploadView
  onReceived: (upload: string) => void
}

function UploadActions({ clientId, upload, onReceived }: UploadActionsProps) {
  if (upload.deleted) return <Badge variant="outline">Deleted</Badge>
  return (
    <div className="flex flex-wrap items-center gap-2">
      {upload.received_at ? (
        <Badge variant="success">Received {formatDay(upload.received_at)}</Badge>
      ) : (
        <Button variant="outline" size="sm" onClick={() => onReceived(upload.id)}>
          <CheckIcon data-icon="inline-start" />
          Mark received
        </Button>
      )}
      <Button
        variant="outline"
        size="sm"
        nativeButton={false}
        render={<a href={`/api/clients/${clientId}/uploads/${upload.id}/file`} download />}
      >
        <DownloadIcon data-icon="inline-start" />
        Download
      </Button>
      <span className="text-xs text-muted-foreground">
        Deleted by {formatDay(upload.delete_by)}
      </span>
    </div>
  )
}
