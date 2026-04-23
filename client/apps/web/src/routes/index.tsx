import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'
import { useConfigurationStore } from '@/stores'

export const Route = createFileRoute('/')({
    component: RouteComponent,
})

function RouteComponent() {
    const navigate = useNavigate()
    const configData = useConfigurationStore((state) => state.config)
    useEffect(() => {
        if (!configData) {
            navigate({ to: '/config' })
        }
        else {
            navigate({ to: '/dashboard' })
        }
    }, [navigate, configData])

    return <div>Redirecting to dashboard...</div>
}
