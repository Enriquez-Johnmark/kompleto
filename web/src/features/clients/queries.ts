import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { BirFormOption } from "@/api/generated/BirFormOption"
import type { ClientDetail } from "@/api/generated/ClientDetail"
import type { ClientInput } from "@/api/generated/ClientInput"
import type { ClientSummary } from "@/api/generated/ClientSummary"
import type { Created } from "@/api/generated/Created"
import type { DueDateChange } from "@/api/generated/DueDateChange"
import type { ItemKey } from "@/api/generated/ItemKey"
import type { NewItem } from "@/api/generated/NewItem"
import type { NewPayor } from "@/api/generated/NewPayor"
import type { NewPayors } from "@/api/generated/NewPayors"
import type { Payroll } from "@/api/generated/Payroll"
import type { PayorsAdded } from "@/api/generated/PayorsAdded"
import type { ScheduleChange } from "@/api/generated/ScheduleChange"
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

export function useClient(id: string) {
  return useQuery({
    queryKey: clientKeys.detail(id),
    queryFn: () => api<ClientDetail>("GET", `/api/clients/${id}`),
  })
}

/** A change made from a Client's page; it refreshes the page afterwards. */
function useClientChange<TInput, TOutput = unknown>(request: (input: TInput) => Promise<TOutput>) {
  const refresh = useRefreshClients()
  return useMutation({ mutationFn: request, onSuccess: refresh })
}

export function useSetSchedule(id: string) {
  return useClientChange((change: ScheduleChange) =>
    api("PUT", `/api/clients/${id}/schedule`, change)
  )
}

/** P1, P2: turns payroll chasing on or off. */
export function useSetPayroll(id: string) {
  return useClientChange((payroll: Payroll) => api("PUT", `/api/clients/${id}/payroll`, payroll))
}

export function useSetDueDate(id: string) {
  return useClientChange((change: DueDateChange) =>
    api("PUT", `/api/clients/${id}/due-dates`, change)
  )
}

/** Stops or starts asking for a built-in document. */
export function useToggleItem(id: string) {
  return useClientChange(({ key, removed }: { key: string; removed: boolean }) =>
    api("POST", `/api/clients/${id}/checklist/${removed ? "restore" : "remove"}`, {
      key,
    } satisfies ItemKey)
  )
}

export function useAddItem(id: string) {
  return useClientChange((item: NewItem) =>
    api("POST", `/api/clients/${id}/checklist/extras`, item)
  )
}

export function useDeleteItem(id: string) {
  return useClientChange((item: string) =>
    api("DELETE", `/api/clients/${id}/checklist/extras/${item}`)
  )
}

export function useAddPayor(id: string) {
  return useClientChange((payor: NewPayor) => api("POST", `/api/clients/${id}/payors`, payor))
}

/** H3: the names ticked from a sales export. */
export function useAddPayors(id: string) {
  return useClientChange((batch: NewPayors) =>
    api<PayorsAdded>("POST", `/api/clients/${id}/payors/batch`, batch)
  )
}

export function useRemovePayor(id: string) {
  return useClientChange((payor: string) => api("DELETE", `/api/clients/${id}/payors/${payor}`))
}

/** F3: starts the Upload's 30 days before deletion. */
export function useMarkReceived(id: string) {
  return useClientChange((upload: string) =>
    api("POST", `/api/clients/${id}/uploads/${upload}/received`)
  )
}
