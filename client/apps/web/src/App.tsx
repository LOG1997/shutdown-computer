import { ShutDownForm } from '@/components/form'
import { Toaster } from "@workspace/ui/components/sonner"
export function App() {
    return (
        <div className="flex min-h-svh p-6">

            <ShutDownForm />

            <Toaster />
        </div >
    )
}
