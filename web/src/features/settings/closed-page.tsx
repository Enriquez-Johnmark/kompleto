import { useLocation } from "react-router"

import type { Closed } from "@/api/generated/Closed"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { LoginLayout } from "@/features/login/login-layout"
import { formatDate } from "@/lib/dates"

/** Shown once, right after the account is closed. */
export function ClosedPage() {
  const closed = useLocation().state as Closed | null
  return (
    <LoginLayout>
      <Card>
        <CardHeader>
          <CardTitle>Your account is closed</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-3 text-sm">
          <p>Nothing more will be sent, and your clients' upload links no longer work.</p>
          {closed ? (
            <p>
              All your data and files will be deleted on {formatDate(closed.erase_on)}. To undo this
              before then,{" "}
              {closed.support_email ? (
                <>
                  email{" "}
                  <a href={`mailto:${closed.support_email}`} className="text-primary underline">
                    {closed.support_email}
                  </a>
                </>
              ) : (
                "contact Kompleto"
              )}
              .
            </p>
          ) : null}
        </CardContent>
      </Card>
    </LoginLayout>
  )
}
