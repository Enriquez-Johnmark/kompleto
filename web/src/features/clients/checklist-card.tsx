import { PlusIcon, Trash2Icon, Undo2Icon } from "lucide-react"
import { useState, type FormEvent } from "react"

import type { Checklist } from "@/api/generated/Checklist"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Field, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { NativeSelect, NativeSelectOption } from "@/components/ui/native-select"
import { Separator } from "@/components/ui/separator"
import { useAddItem, useDeleteItem, useToggleItem } from "@/features/clients/queries"
import { recursLabel, recursLabels } from "@/lib/recurs"
import { cn } from "@/lib/utils"

type ChecklistCardProps = { clientId: string; checklist: Checklist }

/** B4: the documents asked for, built-in and added. */
export function ChecklistCard({ clientId, checklist }: ChecklistCardProps) {
  const toggle = useToggleItem(clientId)
  const remove = useDeleteItem(clientId)
  const error = toggle.error ?? remove.error
  return (
    <Card>
      <CardHeader>
        <CardTitle>Documents to ask for</CardTitle>
        <CardDescription>
          Kompleto asks for these in each reminder. Stop asking for any this client doesn't have.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <ul className="flex flex-col divide-y">
          {checklist.defaults.map((item) => (
            <li key={item.key} className="flex items-center justify-between gap-3 py-2">
              <div
                className={cn(
                  "flex flex-col",
                  item.removed && "text-muted-foreground line-through"
                )}
              >
                <span className="text-sm">{item.label}</span>
                <span className="text-xs text-muted-foreground no-underline">
                  {recursLabel(item.recurs)}
                </span>
              </div>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => toggle.mutate({ key: item.key, removed: item.removed })}
                disabled={toggle.isPending}
              >
                {item.removed ? <Undo2Icon data-icon="inline-start" /> : null}
                {item.removed ? "Ask again" : "Don't ask"}
              </Button>
            </li>
          ))}
          {checklist.extras.map((item) => (
            <li key={item.id} className="flex items-center justify-between gap-3 py-2">
              <div className="flex flex-col">
                <span className="text-sm">{item.label}</span>
                <span className="text-xs text-muted-foreground">
                  {recursLabel(item.recurs)} · added by you
                </span>
              </div>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={`Remove ${item.label}`}
                onClick={() => remove.mutate(item.id)}
                disabled={remove.isPending}
              >
                <Trash2Icon />
              </Button>
            </li>
          ))}
        </ul>
        {error ? (
          <Alert variant="destructive">
            <AlertDescription>{error.message}</AlertDescription>
          </Alert>
        ) : null}
        <Separator />
        <AddItemForm clientId={clientId} />
      </CardContent>
    </Card>
  )
}

function AddItemForm({ clientId }: { clientId: string }) {
  const [label, setLabel] = useState("")
  const [recurs, setRecurs] = useState("every-period")
  const add = useAddItem(clientId)

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    add.mutate({ label, recurs }, { onSuccess: () => setLabel("") })
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-3">
      <div className="flex flex-wrap items-end gap-2">
        <Field className="min-w-48 flex-1">
          <FieldLabel htmlFor={`new-item-${clientId}`}>Add a document</FieldLabel>
          <Input
            id={`new-item-${clientId}`}
            value={label}
            maxLength={120}
            placeholder="e.g. Lazada payout reports"
            onChange={(event) => setLabel(event.target.value)}
          />
        </Field>
        <NativeSelect
          aria-label="When it's owed"
          value={recurs}
          onChange={(event) => setRecurs(event.target.value)}
        >
          {Object.entries(recursLabels).map(([value, text]) => (
            <NativeSelectOption key={value} value={value}>
              {text}
            </NativeSelectOption>
          ))}
        </NativeSelect>
        <Button type="submit" variant="outline" disabled={add.isPending || !label.trim()}>
          <PlusIcon data-icon="inline-start" />
          Add
        </Button>
      </div>
      {add.isError ? (
        <Alert variant="destructive">
          <AlertDescription>{add.error.message}</AlertDescription>
        </Alert>
      ) : null}
    </form>
  )
}
