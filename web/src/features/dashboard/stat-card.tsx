import { Card, CardDescription, CardHeader } from "@/components/ui/card"

type StatCardProps = { label: string; value: number }

export function StatCard({ label, value }: StatCardProps) {
  return (
    <Card size="sm">
      <CardHeader>
        <CardDescription>{label}</CardDescription>
        <p className="text-2xl font-semibold tabular-nums">{value}</p>
      </CardHeader>
    </Card>
  )
}
