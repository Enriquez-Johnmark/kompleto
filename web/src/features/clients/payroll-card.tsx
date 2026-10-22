import { useState, type FormEvent } from "react"

import type { Payroll } from "@/api/generated/Payroll"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldDescription, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { NativeSelect, NativeSelectOption } from "@/components/ui/native-select"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { useSetPayroll } from "@/features/clients/queries"

type PayrollCardProps = { clientId: string; payroll: Payroll }

/** P1, P2: payroll documents before each payday, and contribution
 *  receipts after each due date. Kompleto never computes pay. */
export function PayrollCard({ clientId, payroll }: PayrollCardProps) {
  const [cycle, setCycle] = useState(payroll.pay_cycle ?? "off")
  const [philhealth, setPhilhealth] = useState(payroll.philhealth_due_day.toString())
  const [pagibig, setPagibig] = useState(payroll.pagibig_due_day.toString())
  const save = useSetPayroll(clientId)
  const isOn = cycle !== "off"

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    save.mutate(
      {
        pay_cycle: isOn ? cycle : null,
        philhealth_due_day: Number(philhealth),
        pagibig_due_day: Number(pagibig),
      },
      { onSuccess: () => toast.add({ title: "Payroll chasing saved", type: "success" }) }
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Payroll chasing</CardTitle>
        <CardDescription>
          Before each payday, Kompleto asks for timesheets or DTRs, overtime and leave approvals,
          and new hires and resignations. After each month&apos;s due dates, it asks for the SSS,
          PhilHealth and Pag-IBIG payment receipts. It never computes pay or contributions.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="max-w-md">
          <FieldGroup>
            <Field>
              <FieldLabel htmlFor="pay-cycle">Paydays</FieldLabel>
              <NativeSelect
                id="pay-cycle"
                value={cycle}
                onChange={(event) => setCycle(event.target.value)}
              >
                <NativeSelectOption value="off">Off</NativeSelectOption>
                <NativeSelectOption value="twice-monthly">
                  Twice a month (15th and last day)
                </NativeSelectOption>
                <NativeSelectOption value="monthly">Monthly (last day)</NativeSelectOption>
              </NativeSelect>
            </Field>
            {isOn ? (
              <>
                <Field>
                  <FieldLabel htmlFor="philhealth-day">PhilHealth due day</FieldLabel>
                  <Input
                    id="philhealth-day"
                    type="number"
                    min={1}
                    max={31}
                    value={philhealth}
                    onChange={(event) => setPhilhealth(event.target.value)}
                  />
                  <FieldDescription>
                    Day of the following month the client pays by, e.g. 15 or 20 depending on their
                    PEN, or 25 for a household employer.
                  </FieldDescription>
                </Field>
                <Field>
                  <FieldLabel htmlFor="pagibig-day">Pag-IBIG due day</FieldLabel>
                  <Input
                    id="pagibig-day"
                    type="number"
                    min={1}
                    max={31}
                    value={pagibig}
                    onChange={(event) => setPagibig(event.target.value)}
                  />
                  <FieldDescription>
                    Day of the following month; 31 means the last day. SSS is always the last day of
                    the following month.
                  </FieldDescription>
                </Field>
              </>
            ) : null}
            {save.isError ? (
              <Alert variant="destructive">
                <AlertDescription>{save.error.message}</AlertDescription>
              </Alert>
            ) : null}
            <Button type="submit" className="self-start" disabled={save.isPending}>
              {save.isPending ? <Spinner data-icon="inline-start" /> : null}
              Save payroll chasing
            </Button>
          </FieldGroup>
        </form>
      </CardContent>
    </Card>
  )
}
