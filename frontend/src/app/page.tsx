"use client"

import { Copy, Camera, Info, Github } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Slider } from "@/components/ui/slider"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Card, CardContent } from "@/components/ui/card"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"
import { Switch } from "@/components/ui/switch"
import { useState } from "react"
import { toast } from "sonner"
import { useBitcoinPrice } from "@/hooks/useBitcoinPrice"
import { cn } from "@/lib/utils"

export default function CreateEscrowPage() {
  const [useThirdParty, setUseThirdParty] = useState(false)
  const [includeThirdPartyAddress, setIncludeThirdPartyAddress] = useState(false)
  const [amount, setAmount] = useState("")
  const [unit, setUnit] = useState<"sats" | "btc" | "usd">("sats")
  const { price, loading } = useBitcoinPrice()
  const [feeRate, setFeeRate] = useState(16)
  const [feePreset, setFeePreset] = useState<"economic" | "recommended" | "priority">("recommended")

  const handleCreateEscrow = async () => {
    try {
      // Handle signing logic here
      toast.success("Escrow created successfully")
    } catch (err) {
      const error = err as Error
      toast.error(`Failed to create escrow: ${error.message}`)
    }
  }

  const handleAmountChange = (value: string) => {
    setAmount(value)
  }

  const getConversion = () => {
    if (!price?.usd || !amount || isNaN(Number(amount))) return null

    const btcPrice = price.usd
    const satsToBtc = 1e-8
    const btcToSats = 1e8

    switch (unit) {
      case "sats":
        const satsAmount = Number(amount)
        return `≈ $${(satsAmount * satsToBtc * btcPrice).toFixed(2)} USD`
      case "btc":
        const btcAmount = Number(amount)
        return `≈ $${(btcAmount * btcPrice).toFixed(2)} USD`
      case "usd":
        const usdAmount = Number(amount)
        return `≈ ${((usdAmount / btcPrice) * btcToSats).toFixed(0)} sats`
    }
  }

  const handleFeeChange = (value: number[]) => {
    const newFee = value[0]
    setFeeRate(newFee)
    
    if (newFee < 24) {
      setFeePreset("economic")
    } else if (newFee <= 32) {
      setFeePreset("recommended")
    } else {
      setFeePreset("priority")
    }
  }

  const handleFeePresetChange = (preset: "economic" | "recommended" | "priority") => {
    setFeePreset(preset)
    switch (preset) {
      case "economic":
        setFeeRate(16)
        break
      case "recommended":
        setFeeRate(28)
        break
      case "priority":
        setFeeRate(40)
        break
    }
  }

  const getConfirmationTime = (fee: number) => {
    if (fee < 24) return "~30 min"
    if (fee <= 32) return "~10 min"
    return "< 5 min"
  }

  return (
    <div className="max-w-3xl mx-auto relative pb-16">
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
                        <p>The other party&apos;s Nostr key for communication and Bitcoin address where they&apos;ll receive funds if the escrow resolves in their favor.</p>
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
                      <TooltipContent className="max-w-[300px]">
                        <p>
                          The amount of Bitcoin to be held in escrow. This will be locked until both parties agree on resolution.
                          <br /><br />
                          Real-time BTC/USD conversion rates are provided by CoinGecko and update every minute.
                        </p>
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                </div>
                <div className="flex gap-2">
                  <Input 
                    placeholder="21,000" 
                    value={amount}
                    onChange={(e) => handleAmountChange(e.target.value)}
                  />
                  <Select 
                    value={unit} 
                    onValueChange={(value: "sats" | "btc" | "usd") => setUnit(value)}
                  >
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
                {loading ? (
                  <p className="text-sm text-muted-foreground">Loading conversion...</p>
                ) : (
                  amount && <p className="text-sm text-muted-foreground">{getConversion()}</p>
                )}
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

            <Tabs value={feePreset} onValueChange={handleFeePresetChange}>
              <TabsList className="grid w-full grid-cols-3">
                <TabsTrigger 
                  value="economic"
                  className={cn(feePreset === "economic" && "text-orange-500")}
                >
                  Economic
                </TabsTrigger>
                <TabsTrigger 
                  value="recommended"
                  className={cn(feePreset === "recommended" && "text-orange-500")}
                >
                  Recommended
                </TabsTrigger>
                <TabsTrigger 
                  value="priority"
                  className={cn(feePreset === "priority" && "text-orange-500")}
                >
                  Priority
                </TabsTrigger>
              </TabsList>
              <TabsContent value={feePreset}>
                <div className="py-4">
                  <Slider 
                    value={[feeRate]} 
                    onValueChange={handleFeeChange}
                    min={1}
                    max={50}
                    step={1}
                    className="my-4"
                  />
                  <div className="flex justify-between mt-2 text-sm text-muted-foreground">
                    <span>{feeRate} sat/vB</span>
                    <span>{getConfirmationTime(feeRate)}</span>
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

      {/* Add GitHub link with circular background */}
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

