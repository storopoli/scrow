"use client"

import { Upload, Check, Copy } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { useState } from "react"
import { toast } from "sonner"

interface BroadcastResponse {
  txid: string;
}

export default function BroadcastEscrowPage() {
  const [signedTx, setSignedTx] = useState("")
  const [txid, setTxid] = useState("")
  const [copied, setCopied] = useState(false)

  const handleBroadcast = async () => {
    if (!signedTx) {
      toast.error("Please enter the signed transaction")
      return
    }

    const promise = new Promise<BroadcastResponse>((resolve, reject) => {
      setTimeout(() => {
        // This will be replaced with actual broadcasting logic
        if (signedTx.length < 10) {
          reject(new Error("Invalid transaction format"))
          return
        }
        
        const mockTxid = "abc123..."
        setTxid(mockTxid)
        resolve({ txid: mockTxid })
      }, 1000)
    })

    toast.promise(promise, {
      loading: 'Broadcasting transaction...',
      success: (data) => `Transaction broadcasted successfully! TXID: ${data.txid}`,
      error: (error: Error) => `Error: ${error.message || 'Failed to broadcast transaction'}`
    })
  }

  const handleCopyTxid = () => {
    navigator.clipboard.writeText(txid)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Broadcast Escrow</h1>
      <Card>
        <CardHeader>
          <CardTitle>Broadcast Transaction</CardTitle>
          <CardDescription>
            Broadcast your signed escrow transaction to the Bitcoin network
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="space-y-2">
            <Label>Signed Transaction</Label>
            <Input
              placeholder="Paste your signed transaction hex"
              value={signedTx}
              onChange={(e) => setSignedTx(e.target.value)}
              className="font-mono"
            />
            <p className="text-sm text-muted-foreground">
              * This should be the fully signed transaction from the previous step
            </p>
          </div>

          <Button
            className="w-full"
            disabled={!signedTx}
            onClick={handleBroadcast}
          >
            <Upload className="w-4 h-4 mr-2" />
            Broadcast Transaction
          </Button>

          {txid && (
            <div className="space-y-4 pt-4 border-t border-zinc-800">
              <div className="space-y-2">
                <Label>Transaction ID</Label>
                <div className="flex gap-2">
                  <Input 
                    value={txid}
                    readOnly
                    className="font-mono bg-zinc-900 text-white"
                  />
                  <Button 
                    variant="outline" 
                    size="icon"
                    onClick={handleCopyTxid}
                  >
                    {copied ? (
                      <Check className="h-4 w-4" />
                    ) : (
                      <Copy className="h-4 w-4" />
                    )}
                  </Button>
                </div>
              </div>

              <div className="bg-zinc-900 rounded-lg p-4 text-sm text-muted-foreground">
                <p className="font-medium text-white mb-2">Next Steps:</p>
                <ul className="list-disc list-inside space-y-1">
                  <li>Transaction has been broadcasted to the network</li>
                  <li>Wait for at least 1 confirmation before considering it final</li>
                  <li>You can track the transaction status using the TXID above</li>
                </ul>
              </div>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
} 