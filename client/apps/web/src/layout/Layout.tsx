import { ThemeProvider } from "@/components/theme-provider.tsx"
import { Toaster } from "@workspace/ui/components/sonner"
import { Outlet } from '@tanstack/react-router'

export default function Layout() {
    return (
        <>
            <ThemeProvider>
                <Outlet />
                <Toaster />
            </ThemeProvider>
        </>
    )
}
