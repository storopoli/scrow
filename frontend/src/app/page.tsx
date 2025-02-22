"use client"

import { Copy, Info, Github, Check, ArrowRight, CheckCircle2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Card, CardContent } from "@/components/ui/card"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"
import { Switch } from "@/components/ui/switch"
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@/components/ui/dialog"
import { useState } from "react"
import { toast } from "sonner"
import { cn } from "@/lib/utils"
import confetti from 'canvas-confetti'

export default function CreateEscrowPage() {
  const [useThirdParty, setUseThirdParty] = useState(false)
  const [showEscrowDialog, setShowEscrowDialog] = useState(false)
  const [escrowAddress, setEscrowAddress] = useState("")
  const [fundingTxId, setFundingTxId] = useState("")
  const [copied, setCopied] = useState(false)
  const [confirmationCount, setConfirmationCount] = useState<number>(0)
  const [canGenerateUnsigned, setCanGenerateUnsigned] = useState(false)
  const [showConfirmationInput, setShowConfirmationInput] = useState(false)
  const [unsignedTx, setUnsignedTx] = useState("")
  const [showUnsignedTxDialog, setShowUnsignedTxDialog] = useState(false)
  const [txCopied, setTxCopied] = useState(false)
  const [showConfirmation, setShowConfirmation] = useState(false)

  const handleCreateEscrow = async () => {
    try {
      // This will be replaced with actual address generation
      setEscrowAddress("bc1q...") 
      setShowEscrowDialog(true)
      toast.success("Escrow address generated successfully")
    } catch (err) {
      const error = err as Error
      toast.error(`Failed to create escrow: ${error.message}`)
    }
  }

  const handleCopyAddress = () => {
    navigator.clipboard.writeText(escrowAddress)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const handleSubmitTxId = () => {
    if (!fundingTxId) {
      toast.error("Please enter the transaction ID")
      return
    }
    // Show confirmation input after txid is submitted
    setShowConfirmationInput(true)
    toast.success("Transaction ID submitted successfully")
  }

  const handleConfirmationUpdate = (value: string) => {
    const count = parseInt(value)
    setConfirmationCount(count)
    if (count >= 6) {
      setCanGenerateUnsigned(true)
      toast.success("Transaction has enough confirmations")
    } else {
      setCanGenerateUnsigned(false)
    }
  }

  const handleGenerateUnsigned = () => {
    try {
      // This will be replaced with actual unsigned tx generation
      const mockUnsignedTx = "0200000001f3a536afd4fb2baa0747ba25d96f34a4bb61f1c48b3992c9c1df8f4f8e8c4a1a0000000000feffffff0200e1f505000000001976a914b12366b839b006d2af39d5c218519ac7c3a3f8f888ac6a452f00000000001976a914b12366b839b006d2af39d5c218519ac7c3a3f8f888ac00000000"
      setUnsignedTx(mockUnsignedTx)
      setShowUnsignedTxDialog(true)
      toast.success("Unsigned transaction generated successfully")
    } catch (err) {
      toast.error("Failed to generate unsigned transaction")
    }
  }

  const handleCopyUnsignedTx = () => {
    navigator.clipboard.writeText(unsignedTx)
    setTxCopied(true)
    setTimeout(() => setTxCopied(false), 2000)
  }

  const handleProceedToSign = () => {
    setShowConfirmation(true)

    // After showing confirmation, trigger confetti and navigate
    setTimeout(() => {
      confetti({
        particleCount: 100,
        spread: 70,
        origin: { y: 0.6 },
        colors: ['#f97316', '#ffffff', '#000000'],
      })

      setTimeout(() => {
        setShowUnsignedTxDialog(false)
        setShowConfirmation(false)
        window.location.href = '/sign'
      }, 500)
    }, 1500) // Show confirmation for 1.5 seconds
  }

  return (
    <div className="max-w-3xl mx-auto relative pb-16">
      <h1 className="text-2xl font-bold mb-6">Create Escrow</h1>
      <Card>
        <CardContent className="p-6 space-y-6">
          {/* Participants Section */}
          <div className="space-y-6">
            <div className="space-y-4">
              {/* My Details */}
              <div className="space-y-4 p-4 rounded-lg border border-zinc-800">
                <div className="space-y-2">
                  <Label>My Resolution Bitcoin Address</Label>
                  <div className="flex gap-2">
                    <Input 
                      placeholder="bc1..." 
                      className="font-mono"
                    />
                    <Button variant="outline" size="icon">
                      <Copy className="h-4 w-4" />
                    </Button>
                  </div>
                </div>
              </div>

              {/* Counterparty Details */}
              <div className="space-y-4 p-4 rounded-lg border border-zinc-800">
                <div className="space-y-2">
                  <Label>Counterparty Resolution Bitcoin Address</Label>
                  <div className="flex gap-2">
                    <Input 
                      placeholder="bc1..." 
                      className="font-mono"
                    />
                    <Button variant="outline" size="icon">
                      <Copy className="h-4 w-4" />
                    </Button>
                  </div>
                </div>
              </div>

              {/* Third Party Resolution Section */}
              <div className="space-y-4 p-4 rounded-lg border border-zinc-800">
                <div className="flex items-center justify-between">
                  <h3 className="text-sm font-medium text-muted-foreground">Third Party Resolution</h3>
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger>
                        <Info className="h-4 w-4 text-muted-foreground" />
                      </TooltipTrigger>
                      <TooltipContent className="max-w-[300px]">
                        <p>A trusted third party can help resolve disputes after the timelock period.</p>
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                </div>

                <div className="flex items-center space-x-2">
                  <Switch
                    checked={useThirdParty}
                    onCheckedChange={setUseThirdParty}
                  />
                  <Label>Use Third Party Resolution</Label>
                </div>

                {useThirdParty && (
                  <div className="space-y-4 pl-4">
                    <div className="space-y-2">
                      <Label>Third Party Nostr Public Key</Label>
                      <div className="flex gap-2">
                        <Input 
                          placeholder="npub1..." 
                          className="font-mono"
                        />
                        <Button variant="outline" size="icon">
                          <Copy className="h-4 w-4" />
                        </Button>
                      </div>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>

          <div className="pt-4 flex justify-end gap-4">
            <Button variant="outline">Clear</Button>
            <Button onClick={handleCreateEscrow}>Generate Escrow Address »</Button>
          </div>
        </CardContent>
      </Card>

      {/* Escrow Address Dialog */}
      <Dialog open={showEscrowDialog} onOpenChange={setShowEscrowDialog}>
        <DialogContent className="sm:max-w-[500px]">
          <DialogHeader>
            <DialogTitle>Fund Your Escrow</DialogTitle>
            <DialogDescription>
              This is your escrow address. Please create a collaborative transaction with your escrow counterpart that funds this address as the first output in the transaction.
            </DialogDescription>
          </DialogHeader>
          
          <div className="space-y-6">
            <div className="space-y-2">
              <Label>Escrow Address</Label>
              <div className="flex gap-2">
                <Input 
                  value={escrowAddress}
                  readOnly
                  className="font-mono bg-zinc-900 text-white"
                />
                <Button 
                  variant="outline" 
                  size="icon"
                  onClick={handleCopyAddress}
                >
                  {copied ? (
                    <Check className="h-4 w-4" />
                  ) : (
                    <Copy className="h-4 w-4" />
                  )}
                </Button>
              </div>
            </div>

            <div className="space-y-2">
              <Label>Transaction ID</Label>
              <div className="flex gap-2">
                <Input 
                  value={fundingTxId}
                  onChange={(e) => setFundingTxId(e.target.value)}
                  placeholder="Enter the transaction ID that funds this address"
                  className="font-mono"
                />
                <Button 
                  onClick={handleSubmitTxId}
                >
                  Submit
                </Button>
              </div>
              <p className="text-sm text-muted-foreground">
                * The transaction must fund this address as its first output (vout 0)
              </p>
            </div>

            {showConfirmationInput && (
              <div className="space-y-2 border-t border-zinc-800 pt-4">
                <Label>Block Confirmations</Label>
                <div className="flex gap-2">
                  <Input 
                    type="number"
                    min="0"
                    value={confirmationCount}
                    onChange={(e) => handleConfirmationUpdate(e.target.value)}
                    placeholder="Enter number of confirmations"
                    className="font-mono"
                  />
                </div>
                <div className="flex items-center gap-2 mt-2">
                  <div className={`h-2 w-2 rounded-full ${confirmationCount >= 6 ? 'bg-green-500' : 'bg-orange-500'}`} />
                  <p className="text-sm text-muted-foreground">
                    {confirmationCount >= 6 
                      ? "Transaction has enough confirmations" 
                      : `Waiting for ${6 - confirmationCount} more confirmations`}
                  </p>
                </div>
              </div>
            )}

            {canGenerateUnsigned && (
              <div className="border-t border-zinc-800 pt-4">
                <Button 
                  className="w-full"
                  onClick={handleGenerateUnsigned}
                >
                  Generate Unsigned Transaction »
                </Button>
                <p className="text-sm text-muted-foreground mt-2">
                  * This will create the unsigned transaction for both parties to sign
                </p>
              </div>
            )}
          </div>
        </DialogContent>
      </Dialog>

      {/* Add new Unsigned Transaction Dialog */}
      <Dialog open={showUnsignedTxDialog} onOpenChange={setShowUnsignedTxDialog}>
        <DialogContent className="sm:max-w-[600px]">
          <DialogHeader>
            <DialogTitle>Unsigned Transaction Generated</DialogTitle>
            <DialogDescription>
              Copy this unsigned transaction and proceed to the Sign Escrow page. Both parties need to sign this transaction independently.
            </DialogDescription>
          </DialogHeader>
          
          <div className="space-y-6">
            <div className="space-y-2">
              <Label>Unsigned Transaction Hex</Label>
              <div className="bg-zinc-900 p-4 rounded-lg">
                <div className="font-mono text-sm break-all text-white">
                  {unsignedTx}
                </div>
              </div>
            </div>

            <div className="flex gap-4">
              <Button 
                variant="outline"
                className="flex-1"
                onClick={handleCopyUnsignedTx}
              >
                {txCopied ? (
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
                onClick={handleProceedToSign}
              >
                Proceed to Sign
                <ArrowRight className="w-4 h-4 ml-2" />
              </Button>
            </div>

            <div className="text-sm text-muted-foreground">
              * Save this unsigned transaction. You&apos;ll need it in the Sign Escrow page to generate your signature.
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {/* Add Confirmation Overlay */}
      {showConfirmation && (
        <div className="fixed inset-0 bg-black/80 backdrop-blur-sm flex items-center justify-center z-50">
          <div className="bg-zinc-900 p-8 rounded-lg flex flex-col items-center gap-4 animate-in fade-in zoom-in duration-300">
            <CheckCircle2 className="w-16 h-16 text-green-500 animate-bounce" />
            <div className="text-center">
              <h3 className="text-xl font-semibold text-white mb-2">
                Transaction Ready for Signing
              </h3>
              <p className="text-zinc-400">
                Redirecting you to the signature page...
              </p>
            </div>
          </div>
        </div>
      )}

      {/* GitHub link */}
      <a
        href="https://github.com/storopoli/scrow/"
        target="_blank"
        rel="noopener noreferrer"
        className="fixed bottom-4 right-4 p-2 bg-black rounded-full border border-zinc-800/40 text-zinc-400 hover:text-white transition-colors"
      >
        <Github className="w-6 h-6" />
      </a>
    </div>
  )
}

