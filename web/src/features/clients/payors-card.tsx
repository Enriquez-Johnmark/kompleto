import { PlusIcon, Trash2Icon } from "lucide-react"
import { useState, type FormEvent } from "react"

import type { PayorView } from "@/api/generated/PayorView"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { useAddPayor, useRemovePayor } from "@/features/clients/queries"

type PayorsCardProps = { clientId: string; payors: PayorView[] }

/** H2: businesses that withhold tax and owe this Client a 2307. */
export function PayorsCard({ clientId, payors }: PayorsCardProps) {
  const [name, setName] = useState("")
  const add = useAddPayor(clientId)
  const remove = useRemovePayor(clientId)
  const error = add.error ?? remove.error

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    add.mutate({ name }, { onSuccess: () => setName("") })
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Payors</CardTitle>
        <CardDescription>
          Businesses that withhold tax and owe this client a BIR Form 2307 each quarter. Names only,
          no TINs.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        {payors.length > 0 ? (
          <ul className="flex flex-col divide-y">
            {payors.map((payor) => (
              <li key={payor.id} className="flex items-center justify-between gap-3 py-2 text-sm">
                {payor.name}
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={`Remove ${payor.name}`}
                  onClick={() => remove.mutate(payor.id)}
                  disabled={remove.isPending}
                >
                  <Trash2Icon />
                </Button>
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-sm text-muted-foreground">No Payors yet.</p>
        )}
        <form onSubmit={handleSubmit} className="flex gap-2">
          <Input
            aria-label="Payor's name"
            placeholder="e.g. Jollibee Foods Corp"
            value={name}
            maxLength={100}
            onChange={(event) => setName(event.target.value)}
          />
          <Button type="submit" variant="outline" disabled={add.isPending || !name.trim()}>
            <PlusIcon data-icon="inline-start" />
            Add
          </Button>
        </form>
        {error ? (
          <Alert variant="destructive">
            <AlertDescription>{error.message}</AlertDescription>
          </Alert>
        ) : null}
      </CardContent>
    </Card>
  )
}
