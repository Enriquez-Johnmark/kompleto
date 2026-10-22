import { useMutation } from "@tanstack/react-query"
import {
  CalendarDaysIcon,
  CircleHelpIcon,
  FileCheckIcon,
  HistoryIcon,
  InboxIcon,
  LayoutDashboardIcon,
  LogOutIcon,
  SettingsIcon,
  UploadIcon,
  UsersIcon,
  type LucideIcon,
} from "lucide-react"
import { Navigate, NavLink, Outlet, useLocation } from "react-router"

import { api, ApiError } from "@/api/client"
import { useMe } from "@/app/queries"
import { Logo } from "@/components/logo"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarTrigger,
} from "@/components/ui/sidebar"
import { Skeleton } from "@/components/ui/skeleton"

type NavItem = { to: string; label: string; icon: LucideIcon }

const navigation: NavItem[] = [
  { to: "/", label: "Dashboard", icon: LayoutDashboardIcon },
  { to: "/drafts", label: "Drafts", icon: InboxIcon },
  { to: "/clients", label: "Clients", icon: UsersIcon },
  { to: "/tracker", label: "2307 tracker", icon: FileCheckIcon },
  { to: "/import", label: "Import", icon: UploadIcon },
  { to: "/calendar", label: "BIR calendar", icon: CalendarDaysIcon },
  { to: "/activity", label: "Activity", icon: HistoryIcon },
  { to: "/settings", label: "Settings", icon: SettingsIcon },
  { to: "/help", label: "Help", icon: CircleHelpIcon },
]

/** The logged-in frame: navigation beside (or, on a phone, above) the page. */
export function AppShell() {
  const me = useMe()
  // Only an ended session means logging in again; anything else (a deploy, a
  // dropped connection) is retried while the bookkeeper stays where they are.
  if (me.error instanceof ApiError && me.error.isNotLoggedIn)
    return <Navigate to="/login" replace />
  return (
    <SidebarProvider>
      <Sidebar>
        <SidebarHeader>
          <Logo className="px-2 py-1.5" />
        </SidebarHeader>
        <SidebarContent>
          <SidebarGroup>
            <SidebarGroupContent>
              <SidebarMenu>
                {navigation.map((item) => (
                  <NavMenuItem key={item.to} item={item} />
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        </SidebarContent>
        <SidebarFooter>
          <FirmMenu firmName={me.data?.firm_name} email={me.data?.email} />
        </SidebarFooter>
      </Sidebar>
      <SidebarInset>
        <header className="flex h-12 items-center gap-2 border-b px-4 md:hidden">
          <SidebarTrigger />
          <Logo />
        </header>
        <main className="mx-auto flex w-full max-w-6xl flex-col gap-6 p-4 md:p-8">
          <Outlet />
        </main>
      </SidebarInset>
    </SidebarProvider>
  )
}

function NavMenuItem({ item }: { item: NavItem }) {
  const { pathname } = useLocation()
  const isActive = item.to === "/" ? pathname === "/" : pathname.startsWith(item.to)
  const Icon = item.icon
  return (
    <SidebarMenuItem>
      <SidebarMenuButton isActive={isActive} render={<NavLink to={item.to} />}>
        <Icon />
        <span>{item.label}</span>
      </SidebarMenuButton>
    </SidebarMenuItem>
  )
}

function FirmMenu({ firmName, email }: { firmName?: string; email?: string }) {
  const logOut = useMutation({
    mutationFn: () => api("POST", "/api/logout"),
    onSettled: () => location.assign("/login"),
  })
  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <div className="flex flex-col gap-0.5 px-2 py-1.5 text-sm">
          {firmName ? (
            <span className="truncate font-medium">{firmName}</span>
          ) : (
            <Skeleton className="h-4 w-32" />
          )}
          {email ? <span className="truncate text-xs text-muted-foreground">{email}</span> : null}
        </div>
      </SidebarMenuItem>
      <SidebarMenuItem>
        <SidebarMenuButton onClick={() => logOut.mutate()} disabled={logOut.isPending}>
          <LogOutIcon />
          <span>Log out</span>
        </SidebarMenuButton>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}
