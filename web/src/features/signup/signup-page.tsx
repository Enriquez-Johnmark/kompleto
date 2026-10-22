import { useMutation } from "@tanstack/react-query"
import { MailCheckIcon } from "lucide-react"
import { useState, type FormEvent } from "react"
import { Link } from "react-router"

import { api, ApiError } from "@/api/client"
import type { SignupRequest } from "@/api/generated/SignupRequest"
import { CenteredLayout } from "@/components/centered-layout"
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Checkbox } from "@/components/ui/checkbox"
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { Spinner } from "@/components/ui/spinner"

/** L1: asking for an account. Kompleto opens it once the operator approves;
 *  the answer is the same whether or not the address has one already. */
export function SignupPage() {
  const [form, setForm] = useState<SignupRequest>({
    firm_name: "",
    email: "",
    accepted_terms: false,
  })
  const request = useMutation({
    mutationFn: (body: SignupRequest) => api("POST", "/api/signup", body),
  })

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    request.mutate(form)
  }

  return (
    <CenteredLayout>
      <Card>
        <CardHeader>
          <CardTitle>Request a Kompleto account</CardTitle>
          <CardDescription>
            Free for 4 weeks, no card needed. We'll email you a login link once your account is
            open.
          </CardDescription>
        </CardHeader>
        <CardContent>
          {request.isSuccess ? (
            <Alert>
              <MailCheckIcon />
              <AlertTitle>Thanks! Your request is in</AlertTitle>
              <AlertDescription>
                We'll email {form.email} when the account for {form.firm_name} is ready, usually
                within 2 working days.
              </AlertDescription>
            </Alert>
          ) : (
            <form onSubmit={handleSubmit} noValidate>
              <FieldGroup>
                <Field>
                  <FieldLabel htmlFor="firm-name">Firm name</FieldLabel>
                  <Input
                    id="firm-name"
                    autoComplete="organization"
                    maxLength={100}
                    value={form.firm_name}
                    onChange={(event) => setForm({ ...form, firm_name: event.target.value })}
                  />
                </Field>
                <Field>
                  <FieldLabel htmlFor="email">Your email</FieldLabel>
                  <Input
                    id="email"
                    type="email"
                    autoComplete="email"
                    value={form.email}
                    onChange={(event) => setForm({ ...form, email: event.target.value })}
                  />
                </Field>
                <Field orientation="horizontal">
                  <Checkbox
                    id="accept"
                    checked={form.accepted_terms}
                    onCheckedChange={(checked) => setForm({ ...form, accepted_terms: checked })}
                  />
                  <FieldLabel htmlFor="accept" className="font-normal">
                    <span>
                      I accept the{" "}
                      <Link to="/terms" className="underline underline-offset-4">
                        terms
                      </Link>{" "}
                      and the{" "}
                      <Link to="/data-processing" className="underline underline-offset-4">
                        data processing agreement
                      </Link>
                      .
                    </span>
                  </FieldLabel>
                </Field>
                {request.isError ? (
                  <Alert variant="destructive">
                    <AlertDescription>
                      {request.error.message}
                      {request.error instanceof ApiError && request.error.problems.length > 0 ? (
                        <ul className="mt-1 list-disc pl-4">
                          {request.error.problems.map((problem) => (
                            <li key={problem}>{problem}</li>
                          ))}
                        </ul>
                      ) : null}
                    </AlertDescription>
                  </Alert>
                ) : null}
                <Button type="submit" disabled={request.isPending}>
                  {request.isPending ? <Spinner data-icon="inline-start" /> : null}
                  Request an account
                </Button>
                <p className="text-center text-sm text-muted-foreground">
                  Already have one?{" "}
                  <Link to="/login" className="underline underline-offset-4">
                    Log in
                  </Link>
                </p>
              </FieldGroup>
            </form>
          )}
        </CardContent>
      </Card>
    </CenteredLayout>
  )
}
