
import { createFileRoute } from '@tanstack/react-router'
import { Badge } from "@workspace/ui/components/badge"
import { Button } from "@workspace/ui/components/button"
import {
    Card,
    CardAction,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@workspace/ui/components/card"
import { useQuery } from "@tanstack/react-query"
import { getDeviceStatus, getDeviceInfo } from '@/apis'
import { useConfigurationStore } from '@/stores'
import OsCard from "./components/OsCard/-index"
import CpuCard from "./components/CpuCard/-index"

export const Route = createFileRoute('/dashboard/')({
    component: Dashboard,
})
function Dashboard() {
    const configData = useConfigurationStore((state) => state.config)
    const { protocol } = window.location
    const baseUrl = protocol + "//" + configData?.host + ":" + configData?.port
    const { data, isLoading, error, refetch, isSuccess, isError } = useQuery({
        queryKey: ['deviceStatus', baseUrl],
        queryFn: async () => {
            if (!baseUrl) throw new Error("No URL provided")
            const response = await getDeviceInfo({ config: { baseUrl } })
            console.log('response:', response);
            return response.data
        },
        // 只有当 queryUrl 存在时才启用查询
        enabled: !!baseUrl,
        // 可选：配置重试次数等
        retry: 1,
        refetchInterval: 5000,
    })
    return (
        <div>
            <OsCard data={data?.os} isLoading={isLoading} className='h-32' />
            <CpuCard data={data?.cpu} isLoading={isLoading} className='h-32' />
        </div>
    )
}
