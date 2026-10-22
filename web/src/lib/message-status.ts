/** How a Reminder's outbox status reads to the bookkeeper. */
export const messageStatusLabels: Record<string, string> = {
  draft: "Waiting for approval",
  approved: "Approved",
  sending: "Sending",
  sent: "Sent",
  failed: "Not sent yet",
  blocked: "Blocked",
  unconfirmed: "May have been sent",
  expired: "Expired",
  cancelled: "Cancelled",
}

export function messageStatusLabel(status: string) {
  return messageStatusLabels[status] ?? status
}
