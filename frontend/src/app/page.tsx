"use client"

import { Copy, Camera, Info, Download, Check } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Slider } from "@/components/ui/slider"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Card, CardContent } from "@/components/ui/card"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"
import { Switch } from "@/components/ui/switch"
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from "@/components/ui/dialog"
import { useState } from "react"
import { toast } from "sonner"

export default function CreateEscrowPage() {
  const [useThirdParty, setUseThirdParty] = useState(false)
  const [includeThirdPartyAddress, setIncludeThirdPartyAddress] = useState(false)
  const [showPSBTDialog, setShowPSBTDialog] = useState(false)
  const [psbtData, setPsbtData] = useState("")
  const [copied, setCopied] = useState(false)

  const handleCreateEscrow = async () => {
    try {
      // Here you would make the API call to create the PSBT
      // For now, we'll simulate it
      const response = await fetch('/api/create-escrow', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          // Add your form data here
        }),
      })

      if (!response.ok) throw new Error('Failed to create escrow')

      const data = await response.json()
      setPsbtData(data.psbt || "cHNidP8BAJoCAAAAAljoeiG1ba8MI76OcHBFbDNvfLqlyHV5JPVFiHuyq911AAAAAAD/////g40EJ9DsZQpoqka7CwmK6kQiwHGyyng1Kgd5WdB86h0BAAAAAP////8CcKrwCAAAAAAWABTYXCtx0AYLCcmIauuBXlCZHdoSTQDh9QUAAAAAFgAUAK6pouXw+HaliN9VRuh0LR2HAI8AAAAAAAEAuwIAAAABqtc5MQGL0l+ErkALaISL4J23BurCrBgpi6vucatlb4sAAAAASEcwRAIgWPb8fGoz4bMVSNSByCbAFb0wE1qtQs1neQ2rZtKtJDsCIEoc7SYExnNbY5PltBaR3XiwDwxZQvufdRhW+qk4FX26Af7///8CgPD6AgAAAAAXqRQPuUY0IWlrgsgzryQceMF9295JNIfQ8gonAQAAABepFCnKdPigj4GZlCgYXJe12FLkBj9hh2UAAAAiAgKVg785rgpgl0etGZrd1jT6YQhVnWxc05tMIYPxq5bgf0cwRAIgdAGK1BgAl7hzMjwAFXILNoTMgSOJEEjn282bVa1nnJkCIHPTabdA4+tT3O+jOCPIBwUUylWn3ZVE8VfBZ5EyYRGMASICAtq2H/SaFNtqfQKwzR+7ePxLGDErW05U2uTbovv+9TbXSDBFAiEA9hA4swjcHahlo0hSdG8BV3KTQgjG0kRUOTzZm98iF3cCIAVuZ1pnWm0KArhbFOXikHTYolqbV2C+ooFvZhkQoAbqAQEDBAEAAAABBEdSIQKVg785rgpgl0etGZrd1jT6YQhVnWxc05tMIYPxq5bgfyEC2rYf9JoU22p9ArDNH7t4/EsYMStbTlTa5Nui+/71NtdSriIGApWDvzmuCmCXR60Zmt3WNPphCFWdbFzTm0whg/GrluB/ENkMak8AAACAAAAAgAAAAIAiBgLath/0mhTban0CsM0fu3j8SxgxK1tOVNrk26L7/vU21xDZDGpPAAAAgAAAAIABAACAAAEBIADC6wsAAAAAF6kUt/X69A49QKWkWbHbNTXyty+pIeiHIgIDCJ3BDHrG21T5EymvYXMz2ziM6tDCMfcjN50bmQMLAtxHMEQCIGLrelVhB6fHP0WsSrWh3d9vcHX7EnWWmn84Pv/3hLyyAiAMBdu3Rw2/LwhVfdNWxzJcHtMJE+mWzThAlF2xIijaXwEiAgI63ZBPPW3PWd25BrDe4jUpt/+57VDl6GFRkmhgIh8Oc0cwRAIgZfRbpZmLWaJ//hp77QFq8fH5DVSzqo90UKpfVqJRA70CIH9yRwOtHtuWaAsoS1bU/8uLCOQhkLj9YAGZ9OdqF8/iASICAn9jmXV9Lv9VoTatAsaEsYOLZVbl8bazQoKpS2tQBRCWRzBEAiBl9FulmYtZon/+GnvtAWrx8fkNVLOqj3RQql9WolEDvQIgf3JHA60e25ZoCyhLVtT/y4sI5CGQuP1gAZn052oXz+IB")
      setShowPSBTDialog(true)
    } catch (error) {
      toast.error("Failed to create escrow transaction")
    }
  }

  const handleCopyPSBT = () => {
    navigator.clipboard.writeText(psbtData)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const handleDownloadPSBT = () => {
    const blob = new Blob([psbtData], { type: 'text/plain' })
    const url = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'escrow.psbt'
    document.body.appendChild(a)
    a.click()
    window.URL.revokeObjectURL(url)
    toast.success("PSBT file downloaded successfully")
  }

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Create Escrow</h1>
      <Card>
        <CardContent className="p-6 space-y-6">
          {/* Participants Section */}
          <div className="space-y-6">
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h2 className="text-lg font-semibold">Participant Details</h2>
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger>
                      <Info className="h-4 w-4 text-muted-foreground" />
                    </TooltipTrigger>
                    <TooltipContent className="max-w-[300px]">
                      <p>Both parties need to provide their Nostr public key for communication and a Bitcoin address for fund resolution. The escrow will be locked until both parties agree or the timelock expires.</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </div>
              
              {/* My Details */}
              <div className="space-y-4 p-4 rounded-lg border border-zinc-800">
                <div className="flex items-center justify-between">
                  <h3 className="text-sm font-medium text-muted-foreground">My Details</h3>
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger>
                        <Info className="h-4 w-4 text-muted-foreground" />
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>Your Nostr key will be used to sign messages and your Bitcoin address will receive the funds if the escrow resolves in your favor.</p>
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                </div>
                <div className="space-y-2">
                  <Label>My Nostr Public Key</Label>
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
                    <Button variant="outline" size="icon">
                      <Camera className="h-4 w-4" />
                    </Button>
                  </div>
                </div>
              </div>

              {/* Counterparty Details */}
              <div className="space-y-4 p-4 rounded-lg border border-zinc-800">
                <div className="flex items-center justify-between">
                  <h3 className="text-sm font-medium text-muted-foreground">Counterparty Details</h3>
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger>
                        <Info className="h-4 w-4 text-muted-foreground" />
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>The other party's Nostr key for communication and Bitcoin address where they'll receive funds if the escrow resolves in their favor.</p>
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                </div>
                <div className="space-y-2">
                  <Label>Counterparty Nostr Public Key</Label>
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
                    <Button variant="outline" size="icon">
                      <Camera className="h-4 w-4" />
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
                        <p>A trusted third party can help resolve disputes. They can only intervene after the timelock period if both parties disagree.</p>
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
                      <Label>Timelock Period</Label>
                      <Select defaultValue="1h">
                        <SelectTrigger>
                          <SelectValue />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectItem value="1h">1 hour</SelectItem>
                          <SelectItem value="2h">2 hours</SelectItem>
                          <SelectItem value="4h">4 hours</SelectItem>
                          <SelectItem value="8h">8 hours</SelectItem>
                          <SelectItem value="24h">24 hours</SelectItem>
                        </SelectContent>
                      </Select>
                      <p className="text-sm text-muted-foreground">
                        Time before third party can intervene
                      </p>
                    </div>

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

                    <div className="flex items-center space-x-2">
                      <Switch
                        checked={includeThirdPartyAddress}
                        onCheckedChange={setIncludeThirdPartyAddress}
                      />
                      <Label>Include Resolution Address & Collateral</Label>
                    </div>

                    {includeThirdPartyAddress && (
                      <div className="space-y-4">
                        <div className="space-y-2">
                          <Label>Third Party Resolution Address</Label>
                          <div className="flex gap-2">
                            <Input 
                              placeholder="bc1..." 
                              className="font-mono"
                            />
                            <Button variant="outline" size="icon">
                              <Copy className="h-4 w-4" />
                            </Button>
                            <Button variant="outline" size="icon">
                              <Camera className="h-4 w-4" />
                            </Button>
                          </div>
                        </div>

                        <div className="space-y-2">
                          <Label>Resolution Fee</Label>
                          <div className="flex gap-2">
                            <Input placeholder="1000" />
                            <Select defaultValue="sats">
                              <SelectTrigger className="w-[100px]">
                                <SelectValue />
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value="sats">sats</SelectItem>
                                <SelectItem value="btc">BTC</SelectItem>
                                <SelectItem value="usd">USD</SelectItem>
                              </SelectContent>
                            </Select>
                          </div>
                          <p className="text-sm text-muted-foreground">
                            Amount the third party receives for resolution
                          </p>
                        </div>
                      </div>
                    )}
                  </div>
                )}
              </div>
            </div>

            {/* Amount Section */}
            <div className="grid grid-cols-1 gap-4">
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label>Amount</Label>
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger>
                        <Info className="h-4 w-4 text-muted-foreground" />
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>The amount of Bitcoin to be held in escrow. This will be locked until both parties agree on resolution.</p>
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                </div>
                <div className="flex gap-2">
                  <Input placeholder="21,000" />
                  <Select defaultValue="sats">
                    <SelectTrigger className="w-[100px]">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="sats">sats</SelectItem>
                      <SelectItem value="btc">BTC</SelectItem>
                      <SelectItem value="usd">USD</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
              </div>
            </div>
          </div>

          {/* Fee Settings Section */}
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <Label>Transaction Fee</Label>
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger>
                    <Info className="h-4 w-4 text-muted-foreground" />
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>Higher fees mean faster confirmation times</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
            </div>

            <Tabs defaultValue="recommended">
              <TabsList className="grid w-full grid-cols-3">
                <TabsTrigger value="economic">Economic</TabsTrigger>
                <TabsTrigger value="recommended">Recommended</TabsTrigger>
                <TabsTrigger value="priority">Priority</TabsTrigger>
              </TabsList>
              <TabsContent value="economic">
                <div className="py-4">
                  <Slider defaultValue={[2]} max={100} step={1} />
                  <div className="flex justify-between mt-2 text-sm text-muted-foreground">
                    <span>8 sat/vB</span>
                    <span>~30 min confirmation</span>
                  </div>
                </div>
              </TabsContent>
              <TabsContent value="recommended">
                <div className="py-4">
                  <Slider defaultValue={[16]} max={100} step={1} />
                  <div className="flex justify-between mt-2 text-sm text-muted-foreground">
                    <span>16 sat/vB</span>
                    <span>~10 min confirmation</span>
                  </div>
                </div>
              </TabsContent>
              <TabsContent value="priority">
                <div className="py-4">
                  <Slider defaultValue={[32]} max={100} step={1} />
                  <div className="flex justify-between mt-2 text-sm text-muted-foreground">
                    <span>32 sat/vB</span>
                    <span>~5 min confirmation</span>
                  </div>
                </div>
              </TabsContent>
            </Tabs>
          </div>

          <div className="pt-4 flex justify-end gap-4">
            <Button variant="outline">Clear</Button>
            <Button onClick={handleCreateEscrow}>Create Escrow Transaction »</Button>
          </div>
        </CardContent>
      </Card>

      <Dialog open={showPSBTDialog} onOpenChange={setShowPSBTDialog}>
        <DialogContent className="sm:max-w-[600px]">
          <DialogHeader>
            <DialogTitle>Escrow Transaction Created</DialogTitle>
            <DialogDescription>
              Your PSBT has been generated. Download it or copy to clipboard, then proceed to sign it.
            </DialogDescription>
          </DialogHeader>
          
          <div className="space-y-4 mt-4">
            <div className="p-4 bg-zinc-900 rounded-lg">
              <div className="font-mono text-sm break-all">
                {psbtData.slice(0, 100)}...
              </div>
            </div>

            <div className="flex gap-4">
              <Button 
                className="flex-1" 
                variant="outline"
                onClick={handleCopyPSBT}
              >
                {copied ? (
                  <>
                    <Check className="w-4 h-4 mr-2" />
                    Copied!
                  </>
                ) : (
                  <>
                    <Copy className="w-4 h-4 mr-2" />
                    Copy PSBT
                  </>
                )}
              </Button>
              
              <Button 
                className="flex-1"
                onClick={handleDownloadPSBT}
              >
                <Download className="w-4 h-4 mr-2" />
                Download PSBT
              </Button>
            </div>

            <div className="text-sm text-muted-foreground">
              * After downloading or copying the PSBT, proceed to the Sign Escrow tab to sign the transaction.
            </div>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  )
}

