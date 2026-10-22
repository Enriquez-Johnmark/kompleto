import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Checkbox } from "@/components/ui/checkbox"
import { Field, FieldLabel, FieldLegend, FieldSet } from "@/components/ui/field"
import { NativeSelect, NativeSelectOption } from "@/components/ui/native-select"
import { Skeleton } from "@/components/ui/skeleton"
import { useBirForms } from "@/features/clients/queries"

export type BatchDefaults = { enabled: boolean; language: string; forms: string[] }

type BatchDefaultsCardProps = {
  defaults: BatchDefaults
  onChange: (defaults: BatchDefaults) => void
}

/** B5: for a file without Language and Forms columns, as Xero and QuickBooks
 *  export, the language and BIR forms every client in it gets. */
export function BatchDefaultsCard({ defaults, onChange }: BatchDefaultsCardProps) {
  const birForms = useBirForms()
  return (
    <Card>
      <CardHeader>
        <CardTitle>For files without Language or Forms</CardTitle>
        <CardDescription>
          A contact list exported from Xero or QuickBooks has names and emails only. Kompleto can
          give every client in it the language and forms you choose here; change any client
          afterwards.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <Field orientation="horizontal">
          <Checkbox
            id="use-defaults"
            checked={defaults.enabled}
            onCheckedChange={(enabled) => onChange({ ...defaults, enabled })}
          />
          <FieldLabel htmlFor="use-defaults" className="font-normal">
            My file has no Language or Forms columns
          </FieldLabel>
        </Field>
        {defaults.enabled ? (
          <>
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
          </>
        ) : null}
      </CardContent>
    </Card>
  )
}
