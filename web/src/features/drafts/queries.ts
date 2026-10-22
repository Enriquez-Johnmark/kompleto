import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { BatchApproval } from "@/api/generated/BatchApproval"
import type { BatchResult } from "@/api/generated/BatchResult"
import type { DraftEdit } from "@/api/generated/DraftEdit"
import type { DraftList } from "@/api/generated/DraftList"
import type { DraftView } from "@/api/generated/DraftView"
import { dashboardKeys } from "@/features/dashboard/queries"

export const draftKeys = {
  list: () => ["drafts"] as const,
}

export function useDrafts() {
  return useQuery({
    queryKey: draftKeys.list(),
    queryFn: () => api<DraftList>("GET", "/api/drafts"),
  })
}

/** Refreshes everything that shows Drafts after one changes. */
function useRefresh() {
  const queryClient = useQueryClient()
  return () =>
    Promise.all([
      queryClient.invalidateQueries({ queryKey: draftKeys.list() }),
      queryClient.invalidateQueries({ queryKey: dashboardKeys.all() }),
    ])
}

/** D2: approves the text the bookkeeper saw, named by its hash. */
export function useApprove() {
  const refresh = useRefresh()
  return useMutation({
    mutationFn: (draft: DraftView) =>
      api("POST", `/api/drafts/${draft.id}/approve`, { hash: draft.hash }),
    onSettled: refresh,
  })
}

export function useDiscard() {
  const refresh = useRefresh()
  return useMutation({
    mutationFn: (draft: DraftView) => api("POST", `/api/drafts/${draft.id}/discard`),
    onSettled: refresh,
  })
}

export function useEditDraft() {
  const refresh = useRefresh()
  return useMutation({
    mutationFn: ({ id, edit }: { id: string; edit: DraftEdit }) =>
      api<DraftView>("PUT", `/api/drafts/${id}`, edit),
    onSuccess: refresh,
  })
}

/** D3: approves reviewed Drafts, each bound to the text the review showed. */
export function useApproveBatch() {
  const refresh = useRefresh()
  return useMutation({
    mutationFn: (drafts: DraftView[]) =>
      api<BatchResult>("POST", "/api/drafts/approve", {
        approvals: drafts.map(({ id, hash }) => ({ id, hash })),
      } satisfies BatchApproval),
    onSettled: refresh,
  })
}
