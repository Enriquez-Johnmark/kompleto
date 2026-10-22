import { useState } from "react"

import type { FilingView } from "@/api/generated/FilingView"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import { Field, FieldDescription, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"
import { useSetDueDate } from "@/features/clients/queries"

type DueDateDialogProps = { clientId: string; filing: FilingView }

/** B3: one filing's Due date for this Client only, e.g. an eFPS date. */
export function DueDateDialog({ clientId, filing }: DueDateDialogProps) {
  const [isOpen, setIsOpen] = useState(false)
  const [due, setDue] = useState(filing.due)
  const setDueDate = useSetDueDate(clientId)

  function save(date: string | null) {
    setDueDate.mutate(
      { form: filing.form, period: filing.period_key, due: date },
      { onSuccess: () => setIsOpen(false) }
    )
  }

  return (
    <Dialog
      open={isOpen}
      onOpenChange={(open) => {
        setIsOpen(open)
        if (open) setDue(filing.due)
        else setDueDate.reset()
      }}
    >
      <DialogTrigger render={<Button variant="ghost" size="sm" />}>Change</DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {filing.form} for {filing.period}
          </DialogTitle>
          <DialogDescription>
            A date set here applies to this client only. The cut-off and reminders move with it.
          </DialogDescription>
        </DialogHeader>
        <Field>
          <FieldLabel htmlFor={`due-${filing.form}-${filing.period_key}`}>Due date</FieldLabel>
          <Input
            id={`due-${filing.form}-${filing.period_key}`}
            type="date"
            value={due}
            onChange={(event) => setDue(event.target.value)}
          />
          {filing.extension ? (
            <FieldDescription>Extended by the BIR: {filing.extension}.</FieldDescription>
          ) : null}
        </Field>
        {setDueDate.isError ? (
          <Alert variant="destructive">
            <AlertDescription>{setDueDate.error.message}</AlertDescription>
          </Alert>
        ) : null}
        <DialogFooter>
          {filing.overridden ? (
            <Button variant="outline" onClick={() => save(null)} disabled={setDueDate.isPending}>
              Use the calendar's date
            </Button>
          ) : (
            <DialogClose render={<Button variant="outline" />}>Cancel</DialogClose>
          )}
          <Button onClick={() => save(due)} disabled={setDueDate.isPending || !due}>
            {setDueDate.isPending ? <Spinner data-icon="inline-start" /> : null}
            Save date
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
