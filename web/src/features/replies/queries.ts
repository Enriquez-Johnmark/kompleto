import { useQuery } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { ReplyDetail } from "@/api/generated/ReplyDetail"

export const replyKeys = {
  detail: (id: string) => ["replies", id] as const,
}

export function useReply(id: string) {
  return useQuery({
    queryKey: replyKeys.detail(id),
    queryFn: () => api<ReplyDetail>("GET", `/api/replies/${id}`),
  })
}
