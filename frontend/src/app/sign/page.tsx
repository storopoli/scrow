"use client"

import { Copy, Check, ArrowRight } from "lucide-react"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Label } from "@/components/ui/label"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useState } from "react"
import { toast } from "sonner"

interface SignatureState {
  unsignedTx: string
  nsec: string
  signature: string
  copied: boolean
}

export default function SignEscrowPage() {
  // States for User 1
  const [user1, setUser1] = useState<SignatureState>({
    unsignedTx: "",
    nsec: "",
    signature: "",
    copied: false
  })

  // States for User 2
  const [user2, setUser2] = useState<SignatureState>({
    unsignedTx: "",
    nsec: "",
    signature: "",
    copied: false
  })

  // States for combined signatures
  const [combinedTx, setCombinedTx] = useState("")
  const [combinedCopied, setCombinedCopied] = useState(false)

  const handleGenerateSignature = (user: "user1" | "user2") => {
    const currentUser = user === "user1" ? user1 : user2
    const setUser = user === "user1" ? setUser1 : setUser2

    if (!currentUser.unsignedTx || !currentUser.nsec) {
      toast.error("Please fill in all fields")
      return
    }

    try {
      // This will be replaced with actual signature generation
      const mockSignature = "304402..." + (user === "user1" ? "1" : "2")
      setUser({ ...currentUser, signature: mockSignature })
      toast.success("Signature generated successfully")
    } catch (err) {
      toast.error("Failed to generate signature")
    }
  }

  const handleCopySignature = (user: "user1" | "user2") => {
    const currentUser = user === "user1" ? user1 : user2
    const setUser = user === "user1" ? setUser1 : setUser2

    navigator.clipboard.writeText(currentUser.signature)
    setUser({ ...currentUser, copied: true })
    setTimeout(() => setUser({ ...currentUser, copied: false }), 2000)
  }

  const handleCombineSignatures = () => {
    if (!user1.signature || !user2.signature) {
      toast.error("Both signatures are required")
      return
    }

    try {
      // This will be replaced with actual signature combination
      const mockCombinedTx = "02000000..." + user1.signature + user2.signature
      setCombinedTx(mockCombinedTx)
      toast.success("Signatures combined successfully")
    } catch (err) {
      toast.error("Failed to combine signatures")
    }
  }

  const handleCopyCombined = () => {
    navigator.clipboard.writeText(combinedTx)
    setCombinedCopied(true)
    setTimeout(() => setCombinedCopied(false), 2000)
  }

  return (
    <div className="max-w-5xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Sign Escrow</h1>

      <div className="grid grid-cols-2 gap-6 mb-6">
        {/* User 1 Signature Generation */}
        <Card>
          <CardHeader>
            <CardTitle>User 1 Signature</CardTitle>
            <CardDescription>Generate signature for the first party</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label>Unsigned Transaction</Label>
              <Input 
                placeholder="Paste unsigned transaction hex"
                value={user1.unsignedTx}
                onChange={(e) => setUser1({ ...user1, unsignedTx: e.target.value })}
                className="font-mono"
              />
            </div>

            <div className="space-y-2">
              <Label>Nostr Private Key (nsec)</Label>
              <Input 
                type="password"
                placeholder="Enter your nsec"
                value={user1.nsec}
                onChange={(e) => setUser1({ ...user1, nsec: e.target.value })}
                className="font-mono"
              />
            </div>

            <Button 
              className="w-full"
              onClick={() => handleGenerateSignature("user1")}
            >
              Generate Signature
            </Button>

            {user1.signature && (
              <div className="space-y-2 pt-4 border-t border-zinc-800">
                <Label>Generated Signature</Label>
                <div className="bg-zinc-900 p-3 rounded-lg font-mono text-sm break-all text-white">
                  {user1.signature}
                </div>
                <Button
                  variant="outline"
                  className="w-full"
                  onClick={() => handleCopySignature("user1")}
                >
                  {user1.copied ? (
                    <>
                      <Check className="w-4 h-4 mr-2" />
                      Copied!
                    </>
                  ) : (
                    <>
                      <Copy className="w-4 h-4 mr-2" />
                      Copy Signature
                    </>
                  )}
                </Button>
              </div>
            )}
          </CardContent>
        </Card>

        {/* User 2 Signature Generation - Similar structure */}
        <Card>
          <CardHeader>
            <CardTitle>User 2 Signature</CardTitle>
            <CardDescription>Generate signature for the second party</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            {/* Similar content as User 1, but with user2 state */}
            <div className="space-y-2">
              <Label>Unsigned Transaction</Label>
              <Input 
                placeholder="Paste unsigned transaction hex"
                value={user2.unsignedTx}
                onChange={(e) => setUser2({ ...user2, unsignedTx: e.target.value })}
                className="font-mono"
              />
            </div>

            <div className="space-y-2">
              <Label>Nostr Private Key (nsec)</Label>
              <Input 
                type="password"
                placeholder="Enter your nsec"
                value={user2.nsec}
                onChange={(e) => setUser2({ ...user2, nsec: e.target.value })}
                className="font-mono"
              />
            </div>

            <Button 
              className="w-full"
              onClick={() => handleGenerateSignature("user2")}
            >
              Generate Signature
            </Button>

            {user2.signature && (
              <div className="space-y-2 pt-4 border-t border-zinc-800">
                <Label>Generated Signature</Label>
                <div className="bg-zinc-900 p-3 rounded-lg font-mono text-sm break-all text-white">
                  {user2.signature}
                </div>
                <Button
                  variant="outline"
                  className="w-full"
                  onClick={() => handleCopySignature("user2")}
                >
                  {user2.copied ? (
                    <>
                      <Check className="w-4 h-4 mr-2" />
                      Copied!
                    </>
                  ) : (
                    <>
                      <Copy className="w-4 h-4 mr-2" />
                      Copy Signature
                    </>
                  )}
                </Button>
              </div>
            )}
          </CardContent>
        </Card>
      </div>

      {/* Signature Combination Section */}
      {(user1.signature || user2.signature) && (
        <Card>
          <CardHeader>
            <CardTitle>Combine Signatures</CardTitle>
            <CardDescription>
              Combine both signatures to create the final transaction
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <Button 
              className="w-full"
              onClick={handleCombineSignatures}
              disabled={!user1.signature || !user2.signature}
            >
              Combine Signatures
            </Button>

            {combinedTx && (
              <div className="space-y-4 pt-4 border-t border-zinc-800">
                <div className="space-y-2">
                  <Label>Combined Transaction</Label>
                  <div className="bg-zinc-900 p-3 rounded-lg font-mono text-sm break-all text-white">
                    {combinedTx}
                  </div>
                </div>

                <div className="flex gap-4">
                  <Button
                    variant="outline"
                    className="flex-1"
                    onClick={handleCopyCombined}
                  >
                    {combinedCopied ? (
                      <>
                        <Check className="w-4 h-4 mr-2" />
                        Copied!
                      </>
                    ) : (
                      <>
                        <Copy className="w-4 h-4 mr-2" />
                        Copy Transaction
                      </>
                    )}
                  </Button>

                  <Button 
                    className="flex-1"
                    onClick={() => {
                      window.location.href = '/broadcast'
                    }}
                  >
                    Proceed to Broadcast
                    <ArrowRight className="w-4 h-4 ml-2" />
                  </Button>
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  )
} 