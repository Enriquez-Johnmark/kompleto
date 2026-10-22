import { useState, type FormEvent } from "react"

import type { Schedule } from "@/api/generated/Schedule"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Checkbox } from "@/components/ui/checkbox"
import { Field, FieldDescription, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { useSetSchedule } from "@/features/clients/queries"

type ScheduleCardProps = { clientId: string; schedule: Schedule; paused: boolean }

/** D1: this Client's cut-off, reminder days and pause. */
export function ScheduleCard({ clientId, schedule, paused }: ScheduleCardProps) {
  const [offset, setOffset] = useState(schedule.cut_off_offset_days?.toString() ?? "")
  const [days, setDays] = useState(schedule.reminder_days.join(", "))
  const [isPaused, setIsPaused] = useState(paused)
  const save = useSetSchedule(clientId)

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    save.mutate(
      {
        cut_off_offset_days: offset.trim() === "" ? null : Number(offset),
        reminder_days: days
          .split(/[\s,]+/)
          .filter(Boolean)
          .map(Number),
        paused: isPaused,
      },
      { onSuccess: () => toast.add({ title: "Schedule saved", type: "success" }) }
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Reminder schedule</CardTitle>
        <CardDescription>
          Changes apply to drafts not yet created. At most 4 messages per period.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="max-w-md">
          <FieldGroup>
            <Field>
              <FieldLabel htmlFor="cut-off-offset">
                Cut-off: days before the earliest due date
              </FieldLabel>
              <Input
                id="cut-off-offset"
                type="number"
                min={0}
                max={30}
                placeholder={`${schedule.firm_offset_days} (your default)`}
                value={offset}
                onChange={(event) => setOffset(event.target.value)}
              />
              <FieldDescription>Leave empty to use your firm's default.</FieldDescription>
            </Field>
            <Field>
              <FieldLabel htmlFor="reminder-days">Reminders: days before the cut-off</FieldLabel>
              <Input
                id="reminder-days"
                inputMode="numeric"
                value={days}
                onChange={(event) => setDays(event.target.value)}
              />
              <FieldDescription>Up to 3 days, each 1 to 14, e.g. 7, 3, 1.</FieldDescription>
            </Field>
            <Field orientation="horizontal">
              <Checkbox
                id="paused"
                checked={isPaused}
                onCheckedChange={(checked) => setIsPaused(checked === true)}
              />
              <FieldLabel htmlFor="paused" className="font-normal">
                Pause reminders for this client
              </FieldLabel>
            </Field>
            {save.isError ? (
              <Alert variant="destructive">
                <AlertDescription>{save.error.message}</AlertDescription>
              </Alert>
            ) : null}
            <Button type="submit" className="self-start" disabled={save.isPending}>
              {save.isPending ? <Spinner data-icon="inline-start" /> : null}
              Save schedule
            </Button>
          </FieldGroup>
        </form>
      </CardContent>
    </Card>
  )
}
