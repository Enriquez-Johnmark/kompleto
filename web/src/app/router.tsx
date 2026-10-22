import { createBrowserRouter } from "react-router"

import { AppShell } from "@/app/app-shell"
import { NotFoundPage } from "@/app/not-found-page"

/** Each page is loaded when first visited, so the first screen stays small. */
export const router = createBrowserRouter([
  {
    path: "/login",
    lazy: async () => ({ Component: (await import("@/features/login/login-page")).LoginPage }),
  },
  {
    path: "/login/:token",
    lazy: async () => ({
      Component: (await import("@/features/login/login-link-page")).LoginLinkPage,
    }),
  },
  {
    Component: AppShell,
    children: [
      {
        index: true,
        lazy: async () => ({
          Component: (await import("@/features/dashboard/dashboard-page")).DashboardPage,
        }),
      },
      {
        path: "drafts",
        lazy: async () => ({
          Component: (await import("@/features/drafts/drafts-page")).DraftsPage,
        }),
      },
      { path: "*", Component: NotFoundPage },
    ],
  },
])
