import { useQuery } from "@tanstack/react-query"

import { api } from "@/api/client"
import type { Tracker } from "@/api/generated/Tracker"

export const trackerKeys = {
  all: () => ["tracker"] as const,
  quarter: (quarter: string | null) => [...trackerKeys.all(), quarter] as const,
}

/** One quarter's 2307s; without a quarter, the last one that ended. */
export function useTracker(quarter: string | null) {
  return useQuery({
    queryKey: trackerKeys.quarter(quarter),
    queryFn: () =>
      api<Tracker>(
        "GET",
        quarter ? `/api/tracker?quarter=${encodeURIComponent(quarter)}` : "/api/tracker"
      ),
  })
}
