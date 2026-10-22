import { useMutation } from "@tanstack/react-query"
import { useState } from "react"
import { useNavigate } from "react-router"

import { api } from "@/api/client"
import type { CloseAccount } from "@/api/generated/CloseAccount"
import type { Closed } from "@/api/generated/Closed"
import { Alert, AlertDescription } from "@/components/ui/alert"
import {
  AlertDialog,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"

/** I2: closing the account, confirmed by typing the firm's name. */
export function CloseAccountCard({ firmName }: { firmName: string }) {
  const navigate = useNavigate()
  const [confirm, setConfirm] = useState("")
  const close = useMutation({
    mutationFn: (body: CloseAccount) => api<Closed>("POST", "/api/settings/close", body),
    onSuccess: (closed) => navigate("/closed", { replace: true, state: closed }),
  })
  return (
    <Card className="ring-destructive/30">
      <CardHeader>
        <CardTitle>Close your account</CardTitle>
        <CardDescription>
          Sending stops and everyone is logged out at once. All your data and files are deleted 7
          days later; until then, contact Kompleto to undo it. Download your data first if you want
          to keep it.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <AlertDialog onOpenChange={() => close.reset()}>
          <AlertDialogTrigger render={<Button variant="destructive" />}>
            Close account
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Close {firmName}'s account?</AlertDialogTitle>
              <AlertDialogDescription>
                No more reminders go out and your clients' upload links stop working.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <Field>
              <FieldLabel htmlFor="confirm-name">Type {firmName} to confirm</FieldLabel>
              <Input
                id="confirm-name"
                autoComplete="off"
                value={confirm}
                onChange={(event) => setConfirm(event.target.value)}
              />
            </Field>
            {close.isError ? (
              <Alert variant="destructive">
                <AlertDescription>{close.error.message}</AlertDescription>
              </Alert>
            ) : null}
            <AlertDialogFooter>
              <AlertDialogCancel>Keep my account</AlertDialogCancel>
              <Button
                variant="destructive"
                onClick={() => close.mutate({ confirm })}
                disabled={close.isPending || confirm.trim() === ""}
              >
                {close.isPending ? <Spinner data-icon="inline-start" /> : null}
                Close account
              </Button>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </CardContent>
    </Card>
  )
}
