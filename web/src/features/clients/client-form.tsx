import { zodResolver } from "@hookform/resolvers/zod"
import type { UseMutationResult } from "@tanstack/react-query"
import { Controller, useForm } from "react-hook-form"
import { z } from "zod"

import { ApiError } from "@/api/client"
import type { ClientInput } from "@/api/generated/ClientInput"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { NativeSelect, NativeSelectOption } from "@/components/ui/native-select"
import { Skeleton } from "@/components/ui/skeleton"
import { Spinner } from "@/components/ui/spinner"
import { useBirForms } from "@/features/clients/queries"

const schema = z.object({
  business_name: z.string().trim().min(1, "Enter the business name."),
  contact_first_name: z.string().trim().min(1, "Enter the contact's first name."),
  email: z.email("Enter an email address, e.g. nena@example.com."),
  language: z.string(),
  forms: z.array(z.string()).min(1, "Tick at least one form."),
})

export const emptyClient: ClientInput = {
  business_name: "",
  contact_first_name: "",
  email: "",
  language: "taglish",
  forms: [],
}

type ClientFormProps<TResult> = {
  defaultValues: ClientInput
  submitLabel: string
  mutation: UseMutationResult<TResult, Error, ClientInput>
  onSaved: (result: TResult) => void
}

/** B1: a Client's details. The browser checks for convenience; the API's
 *  checks are final and shown above the button. */
export function ClientForm<TResult>({
  defaultValues,
  submitLabel,
  mutation,
  onSaved,
}: ClientFormProps<TResult>) {
  const birForms = useBirForms()
  const form = useForm<ClientInput>({ resolver: zodResolver(schema), defaultValues })
  const errors = form.formState.errors

  return (
    <form
      onSubmit={form.handleSubmit((values) => mutation.mutate(values, { onSuccess: onSaved }))}
      noValidate
      className="max-w-2xl"
    >
      <FieldGroup>
        <Field data-invalid={errors.business_name ? true : undefined}>
          <FieldLabel htmlFor="business_name">Business name</FieldLabel>
          <Input
            id="business_name"
            aria-invalid={errors.business_name ? true : undefined}
            {...form.register("business_name")}
          />
          <FieldError errors={[errors.business_name]} />
        </Field>
        <div className="grid gap-6 sm:grid-cols-2">
          <Field data-invalid={errors.contact_first_name ? true : undefined}>
            <FieldLabel htmlFor="contact_first_name">Contact's first name</FieldLabel>
            <Input
              id="contact_first_name"
              aria-invalid={errors.contact_first_name ? true : undefined}
              {...form.register("contact_first_name")}
            />
            <FieldError errors={[errors.contact_first_name]} />
          </Field>
          <Field data-invalid={errors.email ? true : undefined}>
            <FieldLabel htmlFor="email">Email</FieldLabel>
            <Input
              id="email"
              type="email"
              aria-invalid={errors.email ? true : undefined}
              {...form.register("email")}
            />
            <FieldError errors={[errors.email]} />
          </Field>
        </div>
        <Field>
          <FieldLabel htmlFor="language">Reminder language</FieldLabel>
          <NativeSelect id="language" {...form.register("language")}>
            <NativeSelectOption value="taglish">Taglish</NativeSelectOption>
            <NativeSelectOption value="english">English</NativeSelectOption>
          </NativeSelect>
        </Field>
        <FieldSet data-invalid={errors.forms ? true : undefined}>
          <FieldLegend>BIR forms this client files</FieldLegend>
          <FieldDescription>Kompleto works out each form's due dates.</FieldDescription>
          {birForms.data ? (
            <Controller
              control={form.control}
              name="forms"
              render={({ field }) => (
                <div className="grid gap-3 sm:grid-cols-2">
                  {birForms.data.map((option) => (
                    <Field key={option.code} orientation="horizontal">
                      <Checkbox
                        id={`form-${option.code}`}
                        checked={field.value.includes(option.code)}
                        onCheckedChange={(checked) =>
                          field.onChange(
                            checked
                              ? [...field.value, option.code]
                              : field.value.filter((code) => code !== option.code)
                          )
                        }
                      />
                      <FieldLabel
                        htmlFor={`form-${option.code}`}
                        className="flex-col items-start gap-0.5 font-normal"
                      >
                        <span className="font-medium whitespace-nowrap">{option.code}</span>
                        <span className="text-xs text-muted-foreground">{option.name}</span>
                      </FieldLabel>
                    </Field>
                  ))}
                </div>
              )}
            />
          ) : (
            <Skeleton className="h-32" />
          )}
          <FieldError errors={[errors.forms]} />
        </FieldSet>
        {mutation.isError ? <SaveError error={mutation.error} /> : null}
        <Button type="submit" className="self-start" disabled={mutation.isPending}>
          {mutation.isPending ? <Spinner data-icon="inline-start" /> : null}
          {submitLabel}
        </Button>
      </FieldGroup>
    </form>
  )
}

function SaveError({ error }: { error: Error }) {
  const problems = error instanceof ApiError ? error.problems : []
  return (
    <Alert variant="destructive">
      <AlertTitle>{error.message}</AlertTitle>
      {problems.length > 0 ? (
        <AlertDescription>
          <ul className="list-disc pl-4">
            {problems.map((problem) => (
              <li key={problem}>{problem}</li>
            ))}
          </ul>
        </AlertDescription>
      ) : null}
    </Alert>
  )
}
