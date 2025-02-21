"use client"

import { Download } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { toast } from "sonner"

export default function SignEscrowPage() {
  const handleSignTransaction = async () => {
    try {
      // Handle signing logic here
      toast.success("Transaction signed successfully")
    } catch (error) {
      toast.error("Failed to sign transaction")
    }
  }

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Sign Escrow</h1>
      <Card>
        <CardHeader>
          <CardTitle className="text-lg font-medium">
            Sign Raw Transaction
          </CardTitle>
        </CardHeader>
        <CardContent className="p-6 space-y-4">
          <div className="space-y-2">
            <Label>Raw Transaction</Label>
            <Input 
              placeholder="Enter raw transaction hex" 
              className="font-mono"
            />
            <p className="text-sm text-muted-foreground">
              Paste the raw transaction hex that needs to be signed
            </p>
          </div>

          <div className="pt-4 border-t border-zinc-800">
            <Button 
              className="w-full"
              onClick={handleSignTransaction}
            >
              <Download className="w-4 h-4 mr-2" />
              Sign Transaction
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  )
} 