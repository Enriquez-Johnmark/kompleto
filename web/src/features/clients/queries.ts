import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { BirFormOption } from "@/api/generated/BirFormOption"
import type { ClientInput } from "@/api/generated/ClientInput"
import type { ClientSummary } from "@/api/generated/ClientSummary"
import type { Created } from "@/api/generated/Created"
import { dashboardKeys } from "@/features/dashboard/queries"

export const clientKeys = {
  all: () => ["clients"] as const,
  list: () => [...clientKeys.all(), "list"] as const,
  detail: (id: string) => [...clientKeys.all(), "detail", id] as const,
  birForms: () => ["bir-forms"] as const,
}

export function useClients() {
  return useQuery({
    queryKey: clientKeys.list(),
    queryFn: () => api<ClientSummary[]>("GET", "/api/clients"),
  })
}

/** The BIR forms a Client can file; they change only with a deploy. */
export function useBirForms() {
  return useQuery({
    queryKey: clientKeys.birForms(),
    queryFn: () => api<BirFormOption[]>("GET", "/api/bir-forms"),
    staleTime: Infinity,
  })
}

/** Refreshes every page that shows Clients after one changes. */
export function useRefreshClients() {
  const queryClient = useQueryClient()
  return () =>
    Promise.all([
      queryClient.invalidateQueries({ queryKey: clientKeys.all() }),
      queryClient.invalidateQueries({ queryKey: dashboardKeys.all() }),
    ])
}

export function useCreateClient() {
  const refresh = useRefreshClients()
  return useMutation({
    mutationFn: (input: ClientInput) => api<Created>("POST", "/api/clients", input),
    onSuccess: refresh,
  })
}

export function useUpdateClient(id: string) {
  const refresh = useRefreshClients()
  return useMutation({
    mutationFn: (input: ClientInput) => api("PUT", `/api/clients/${id}`, input),
    onSuccess: refresh,
  })
}
