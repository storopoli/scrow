"use client"

import { Settings } from "lucide-react"
import { Card, CardContent } from "@/components/ui/card"
import { Label } from "@/components/ui/label"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"

export default function SettingsPage() {
  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Settings</h1>
      <Card>
        <CardContent className="p-6 space-y-6">
          <div className="space-y-2">
            <Label>RPC URL</Label>
            <Input placeholder="http://localhost:8332" />
          </div>
          <div className="space-y-2">
            <Label>API Key</Label>
            <Input type="password" placeholder="Enter your API key" />
          </div>
          <Button>Save Settings</Button>
        </CardContent>
      </Card>
    </div>
  )
} 