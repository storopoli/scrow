"use client"

import { List } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"

export default function HistoryPage() {
  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Transaction History</h1>
      <Card>
        <CardContent className="p-6">
          <div className="text-center py-8 text-muted-foreground">
            <List className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>No transaction history available</p>
          </div>
        </CardContent>
      </Card>
    </div>
  )
} 