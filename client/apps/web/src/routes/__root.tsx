import { createRootRoute } from '@tanstack/react-router'
import { TanStackRouterDevtoolsPanel } from '@tanstack/react-router-devtools'
import { TanStackDevtools } from '@tanstack/react-devtools'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import Layout from "@/layout/Layout.tsx"

export const Route = createRootRoute({
    component: RootComponent,
})
const queryClient = new QueryClient()
function RootComponent() {
    return (
        <>
            <QueryClientProvider client={queryClient}>
                <Layout />
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
