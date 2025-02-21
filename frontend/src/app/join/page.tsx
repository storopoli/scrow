"use client"

import { Download } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"

export default function JoinPage() {
  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Join Escrow</h1>
      <Card>
        <CardContent className="p-6 space-y-4">
          <div className="space-y-2">
            <Label>Escrow ID</Label>
            <div className="flex gap-2">
              <Input placeholder="Enter escrow ID" />
              <Button>Join</Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  )
} 