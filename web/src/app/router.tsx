import { createBrowserRouter } from "react-router"

import { AppShell } from "@/app/app-shell"
import { NotFoundPage } from "@/app/not-found-page"
import { PageLoading } from "@/app/page-loading"

/** Each page is loaded when first visited, so the first screen stays small. */
export const router = createBrowserRouter([
  {
    path: "/login",
    HydrateFallback: PageLoading,
    lazy: async () => ({ Component: (await import("@/features/login/login-page")).LoginPage }),
  },
  {
    path: "/login/:token",
    HydrateFallback: PageLoading,
    lazy: async () => ({
      Component: (await import("@/features/login/login-link-page")).LoginLinkPage,
    }),
  },
  {
    Component: AppShell,
    HydrateFallback: PageLoading,
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
      {
        path: "clients",
        lazy: async () => ({
          Component: (await import("@/features/clients/clients-page")).ClientsPage,
        }),
      },
      {
        path: "clients/new",
        lazy: async () => ({
          Component: (await import("@/features/clients/new-client-page")).NewClientPage,
        }),
      },
      {
        path: "clients/:id",
        lazy: async () => ({
          Component: (await import("@/features/clients/client-page")).ClientPage,
        }),
      },
      {
        path: "clients/:id/edit",
        lazy: async () => ({
          Component: (await import("@/features/clients/edit-client-page")).EditClientPage,
        }),
      },
      {
        path: "replies/:id",
        lazy: async () => ({
          Component: (await import("@/features/replies/reply-page")).ReplyPage,
        }),
      },
      {
        path: "import",
        lazy: async () => ({
          Component: (await import("@/features/import/import-page")).ImportPage,
        }),
      },
      {
        path: "tracker",
        lazy: async () => ({
          Component: (await import("@/features/tracker/tracker-page")).TrackerPage,
        }),
      },
      {
        path: "calendar",
        lazy: async () => ({
          Component: (await import("@/features/calendar/calendar-page")).CalendarPage,
        }),
      },
      {
        path: "activity",
        lazy: async () => ({
          Component: (await import("@/features/activity/activity-page")).ActivityPage,
        }),
      },
      { path: "*", Component: NotFoundPage },
    ],
  },
])
