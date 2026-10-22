import { useQuery } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { Dashboard } from "@/api/generated/Dashboard"

export const dashboardKeys = {
  all: () => ["dashboard"] as const,
}

export function useDashboard() {
  return useQuery({
    queryKey: dashboardKeys.all(),
    queryFn: () => api<Dashboard>("GET", "/api/dashboard"),
  })
}
