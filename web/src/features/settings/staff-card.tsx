import { MailPlusIcon, Trash2Icon } from "lucide-react"
import { useState, type FormEvent } from "react"

import type { MemberView } from "@/api/generated/MemberView"
import { QueryError } from "@/components/query-error"
import { Alert, AlertDescription } from "@/components/ui/alert"
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Skeleton } from "@/components/ui/skeleton"
import { Spinner } from "@/components/ui/spinner"
import { toast } from "@/components/ui/toast"
import { formatDay } from "@/lib/dates"
import { useInviteStaff, useRemoveStaff, useStaff } from "@/features/settings/queries"

/** K1: the owner invites up to 5 staff, each with their own login. */
export function StaffCard() {
  const staff = useStaff()
  const invite = useInviteStaff()
  const [email, setEmail] = useState("")
  const count = staff.data ? staff.data.members.length - 1 : 0
  const full = staff.data ? count >= staff.data.max_staff : false

  function handleSubmit(event: FormEvent) {
    event.preventDefault()
    invite.mutate(
      { email },
      {
        onSuccess: ({ member, email_sent }) => {
          setEmail("")
          toast.add(
            email_sent
              ? { title: `Invitation sent to ${member.email}`, type: "success" }
              : {
                  title: `${member.email} was added, but the invitation couldn't be emailed`,
                  description: "Ask them to log in from Kompleto's login page with their email.",
                  type: "warning",
                }
          )
        },
      }
    )
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Staff</CardTitle>
        <CardDescription>
          Up to {staff.data?.max_staff ?? 5} people who log in with their own email. They do the
          daily work; only you can download all data, close the account or change staff.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        {staff.isPending ? <Skeleton className="h-16" /> : null}
        {staff.isError ? <QueryError error={staff.error} /> : null}
        {staff.data ? (
          <ul className="flex flex-col divide-y">
            {staff.data.members.map((member) => (
              <MemberRow key={member.id} member={member} />
            ))}
          </ul>
        ) : null}
        <form onSubmit={handleSubmit} className="flex gap-2">
          <Input
            type="email"
            aria-label="Staff member's email"
            placeholder="name@example.com"
            value={email}
            onChange={(event) => setEmail(event.target.value)}
            disabled={full}
          />
          <Button
            type="submit"
            variant="outline"
            disabled={invite.isPending || full || !email.trim()}
          >
            {invite.isPending ? (
              <Spinner data-icon="inline-start" />
            ) : (
              <MailPlusIcon data-icon="inline-start" />
            )}
            Invite
          </Button>
        </form>
        {full ? (
          <p className="text-sm text-muted-foreground">
            You have the most staff a firm can have. Remove someone to invite another.
          </p>
        ) : null}
        {invite.isError ? (
          <Alert variant="destructive">
            <AlertDescription>{invite.error.message}</AlertDescription>
          </Alert>
        ) : null}
      </CardContent>
    </Card>
  )
}

function MemberRow({ member }: { member: MemberView }) {
  const remove = useRemoveStaff()
  const isOwner = member.role === "owner"
  return (
    <li className="flex items-center justify-between gap-3 py-2 text-sm">
      <div className="flex min-w-0 flex-col">
        <span className="truncate font-medium">{member.email}</span>
        <span className="text-xs text-muted-foreground">Added {formatDay(member.added_at)}</span>
      </div>
      {isOwner ? (
        <Badge variant="secondary">Owner</Badge>
      ) : (
        <AlertDialog>
          <AlertDialogTrigger
            render={<Button variant="ghost" size="icon-sm" aria-label={`Remove ${member.email}`} />}
          >
            <Trash2Icon />
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>Remove {member.email}?</AlertDialogTitle>
              <AlertDialogDescription>
                They're logged out at once and can't log in again. What they did stays in the
                activity log.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Keep them</AlertDialogCancel>
              <AlertDialogAction
                variant="destructive"
                onClick={() =>
                  remove.mutate(member.id, {
                    onError: (error) =>
                      toast.add({
                        title: `${member.email} wasn't removed`,
                        description: error.message,
                        type: "error",
                      }),
                  })
                }
                disabled={remove.isPending}
              >
                Remove
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      )}
    </li>
  )
}
