import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { Invite } from "@/api/generated/Invite"
import type { MemberView } from "@/api/generated/MemberView"
import type { StaffList } from "@/api/generated/StaffList"

export const staffKeys = {
  list: () => ["staff"] as const,
}

/** The Firm's owner and staff; only the owner may read it (K2). */
export function useStaff() {
  return useQuery({
    queryKey: staffKeys.list(),
    queryFn: () => api<StaffList>("GET", "/api/staff"),
  })
}

export function useInviteStaff() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (invite: Invite) => api<MemberView>("POST", "/api/staff", invite),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: staffKeys.list() }),
  })
}

export function useRemoveStaff() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: (id: string) => api("DELETE", `/api/staff/${id}`),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: staffKeys.list() }),
  })
}
