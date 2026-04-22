"use client"

import { TrendingUp } from "lucide-react"
import {
    Label,
    PolarGrid,
    PolarRadiusAxis,
    RadialBar,
    RadialBarChart,
} from "recharts"

import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@workspace/ui/components/card"
import {
    ChartContainer,
    type ChartConfig,
} from "@workspace/ui/components/chart"


const chartData = [
    { browser: "safari", visitors: 200, fill: "var(--color-safari)" },
]

const chartConfig = {
    visitors: {
        label: "Visitors",
    },
    safari: {
        label: "Safari",
        color: "var(--chart-2)",
    },
} satisfies ChartConfig
interface Props {
    className?: string
    num: number | string
}
export function Chart(props: Props) {
    const { num = 0 } = props
    const chartData = [
        { browser: "safari", visitors: num, fill: "var(--color-safari)" },
    ]

    const chartConfig = {
        visitors: {
            label: "Visitors",
        },
        safari: {
            label: "Safari",
            color: "var(--chart-2)",

        },
    } satisfies ChartConfig
    return (
        <div className="flex flex-col w-full">

            <ChartContainer
                config={chartConfig}
                className="mx-auto aspect-square max-h-12 w-full h-full"
            >
                <RadialBarChart
                    data={chartData}
                    startAngle={0}
                    endAngle={360 * Number(num) / 100}
                    outerRadius={24}
                    innerRadius={20}
                >
                    <PolarGrid
                        gridType="circle"
                        radialLines={false}
                        stroke="none"
                        className="first:fill-muted last:fill-background "
                        polarRadius={[24, 20]}
                    />
                    <RadialBar fill="red" dataKey="visitors" className="" background={true} cornerRadius={10} />
                    <PolarRadiusAxis tick={false} tickLine={false} axisLine={false}>
                        <Label
                            content={({ viewBox }) => {
                                if (viewBox && "cx" in viewBox && "cy" in viewBox) {
                                    return (
                                        <text
                                            x={viewBox.cx}
                                            y={viewBox.cy}
                                            textAnchor="middle"
                                            dominantBaseline="middle"
                                        >
                                            <tspan
                                                x={viewBox.cx}
                                                y={viewBox.cy}
                                                className="fill-foreground text-xs font-bold"
                                            >
                                                {Number(chartData[0].visitors).toFixed(1).toLocaleString()}%
                                            </tspan>
                                        </text>
                                    )
                                }
                            }}
                        />
                    </PolarRadiusAxis>
                </RadialBarChart>
            </ChartContainer>

        </div>
    )
}
