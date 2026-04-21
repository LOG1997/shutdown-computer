import { Outlet, createRootRoute } from '@tanstack/react-router'
import { TanStackRouterDevtoolsPanel } from '@tanstack/react-router-devtools'
import { TanStackDevtools } from '@tanstack/react-devtools'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ThemeProvider } from "@/components/theme-provider.tsx"
import { Toaster } from "@workspace/ui/components/sonner"

export const Route = createRootRoute({
    component: RootComponent,
})
const queryClient = new QueryClient()
function RootComponent() {
    return (
        <>
            <QueryClientProvider client={queryClient}>
                <ThemeProvider>
                    <Outlet />
                    <Toaster />
                </ThemeProvider>
            </QueryClientProvider>
            <TanStackDevtools
                config={{
                    position: 'bottom-right',
                }}
                plugins={[
                    {
                        name: 'TanStack Router',
                        render: <TanStackRouterDevtoolsPanel />,
                    },
                ]}
            />
        </>
    )
}
