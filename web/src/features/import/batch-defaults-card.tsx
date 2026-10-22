import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Checkbox } from "@/components/ui/checkbox"
import { Field, FieldLabel, FieldLegend, FieldSet } from "@/components/ui/field"
import { NativeSelect, NativeSelectOption } from "@/components/ui/native-select"
import { Skeleton } from "@/components/ui/skeleton"
import { useBirForms } from "@/features/clients/queries"

export type BatchDefaults = { language: string; forms: string[] }

type BatchDefaultsCardProps = {
  defaults: BatchDefaults
  onChange: (defaults: BatchDefaults) => void
}

/** B5: the language and BIR forms for rows that have none, as in a Xero or
 *  QuickBooks export. Choose them before the file. */
export function BatchDefaultsCard({ defaults, onChange }: BatchDefaultsCardProps) {
  const birForms = useBirForms()
  return (
    <Card>
      <CardHeader>
        <CardTitle>For files without Language or Forms</CardTitle>
        <CardDescription>
          A contact list exported from Xero or QuickBooks has names and emails only. Kompleto gives
          every client in it this language and these forms; change them per client afterwards.
          Choose them before the file.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <Field className="max-w-xs">
          <FieldLabel htmlFor="default-language">Language</FieldLabel>
          <NativeSelect
            id="default-language"
            value={defaults.language}
            onChange={(event) => onChange({ ...defaults, language: event.target.value })}
          >
            <NativeSelectOption value="taglish">Taglish</NativeSelectOption>
            <NativeSelectOption value="english">English</NativeSelectOption>
          </NativeSelect>
        </Field>
        <FieldSet>
          <FieldLegend variant="label">BIR forms</FieldLegend>
          {birForms.data ? (
            <div className="grid gap-3 sm:grid-cols-3">
              {birForms.data.map((option) => (
                <Field key={option.code} orientation="horizontal">
                  <Checkbox
                    id={`default-form-${option.code}`}
                    checked={defaults.forms.includes(option.code)}
                    onCheckedChange={(checked) =>
                      onChange({
                        ...defaults,
                        forms: checked
                          ? [...defaults.forms, option.code]
                          : defaults.forms.filter((code) => code !== option.code),
                      })
                    }
                  />
                  <FieldLabel htmlFor={`default-form-${option.code}`} className="font-normal">
                    {option.code}
                  </FieldLabel>
                </Field>
              ))}
            </div>
          ) : (
            <Skeleton className="h-24" />
          )}
        </FieldSet>
      </CardContent>
    </Card>
  )
}
