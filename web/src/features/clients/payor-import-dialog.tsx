import { useMutation } from "@tanstack/react-query"
import { FileSpreadsheetIcon } from "lucide-react"
import { useState, type ChangeEvent } from "react"

import { apiUpload } from "@/api/client"
import type { PayorNames } from "@/api/generated/PayorNames"
import type { PayorView } from "@/api/generated/PayorView"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
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
import { Field, FieldLabel } from "@/components/ui/field"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { useAddPayors } from "@/features/clients/queries"

type PayorImportDialogProps = { clientId: string; payors: PayorView[] }

/** H3: the customers who withheld tax in a sales export, to tick and add
 *  as Payors. Kompleto reads names only. */
export function PayorImportDialog({ clientId, payors }: PayorImportDialogProps) {
  const [isOpen, setIsOpen] = useState(false)
  const [everyRow, setEveryRow] = useState(false)
  const [ticked, setTicked] = useState<string[]>([])
  const read = useMutation({
    mutationFn: (file: File) =>
      apiUpload<PayorNames>("/api/payors/import", file, everyRow ? [["every_row", "true"]] : []),
  })
  const add = useAddPayors(clientId)
  const listed = new Set(payors.map((payor) => payor.name.toLowerCase()))
  const names = read.data?.names ?? []
  const fresh = names.filter((name) => !listed.has(name.toLowerCase()))
  // Only names on screen and not yet listed: a list thrown away, or names
  // added a moment ago, are never sent.
  const selected = fresh.filter((name) => ticked.includes(name))
  const error = read.error ?? add.error

  function handleOpenChange(open: boolean) {
    setIsOpen(open)
    if (!open) {
      read.reset()
      add.reset()
    }
  }

  function handleFile(event: ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0]
    event.target.value = ""
    if (!file) return
    read.mutate(file, {
      onSuccess: ({ names }) => setTicked(names.filter((name) => !listed.has(name.toLowerCase()))),
    })
  }

  function handleEveryRow(checked: boolean) {
    setEveryRow(checked)
    // The names came from the other choice.
    read.reset()
  }

  function handleAdd() {
    add.mutate(
      { names: selected },
      {
        onSuccess: ({ added, refused }) => {
          toast.add({ title: `${added} Payor${added === 1 ? "" : "s"} added`, type: "success" })
          if (refused.length === 0) handleOpenChange(false)
        },
      }
    )
  }

  return (
    <Dialog open={isOpen} onOpenChange={handleOpenChange}>
      <DialogTrigger render={<Button variant="outline" />}>
        <FileSpreadsheetIcon data-icon="inline-start" />
        From a sales export
      </DialogTrigger>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>Payors from a sales export</DialogTitle>
          <DialogDescription>
            Upload one quarter&apos;s sales, invoice or account transactions (.csv or .xlsx) from
            Xero, QuickBooks or a spreadsheet. Kompleto lists the customers on rows that show tax
            withheld, and reads nothing but their names.
          </DialogDescription>
        </DialogHeader>
        <Field orientation="horizontal">
          <Checkbox id="every-row" checked={everyRow} onCheckedChange={handleEveryRow} />
          <FieldLabel htmlFor="every-row" className="font-normal">
            Every row is withholding (a report of the CWT account only)
          </FieldLabel>
        </Field>
        <label className="flex cursor-pointer items-center justify-center gap-2 rounded-lg border border-dashed p-6 text-sm font-medium hover:bg-muted/50">
          {read.isPending ? <Spinner /> : <FileSpreadsheetIcon className="text-muted-foreground" />}
          Choose a spreadsheet
          <input
            type="file"
            accept=".csv,.xlsx"
            className="sr-only"
            onChange={handleFile}
            disabled={read.isPending}
          />
        </label>
        {read.data ? (
          names.length === 0 ? (
            <p className="text-sm text-muted-foreground">No customer in this file withheld tax.</p>
          ) : (
            <ul className="flex max-h-72 flex-col gap-2 overflow-y-auto">
              {names.map((name, index) => {
                const isListed = listed.has(name.toLowerCase())
                const id = `payor-name-${index}`
                return (
                  <li key={name}>
                    <Field orientation="horizontal">
                      <Checkbox
                        id={id}
                        checked={isListed || ticked.includes(name)}
                        disabled={isListed}
                        onCheckedChange={(checked) =>
                          setTicked(checked ? [...ticked, name] : ticked.filter((n) => n !== name))
                        }
                      />
                      <FieldLabel htmlFor={id} className="font-normal">
                        {name}
                        {isListed ? (
                          <span className="text-muted-foreground"> (already listed)</span>
                        ) : null}
                      </FieldLabel>
                    </Field>
                  </li>
                )
              })}
            </ul>
          )
        ) : null}
        {error ? (
          <Alert variant="destructive">
            <AlertDescription>{error.message}</AlertDescription>
          </Alert>
        ) : null}
        {add.data && add.data.refused.length > 0 ? (
          <Alert variant="destructive">
            <AlertDescription>Not added: {add.data.refused.join("; ")}</AlertDescription>
          </Alert>
        ) : null}
        <DialogFooter>
          <DialogClose render={<Button variant="outline" />}>Cancel</DialogClose>
          <Button onClick={handleAdd} disabled={add.isPending || selected.length === 0}>
            Add {selected.length} of {fresh.length}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
