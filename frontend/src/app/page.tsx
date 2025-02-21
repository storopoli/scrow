"use client"

import { Copy, Camera, Info } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Slider } from "@/components/ui/slider"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Card, CardContent } from "@/components/ui/card"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"

export default function CreateEscrowPage() {
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
            </div>

            {/* Amount and Timelock */}
            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label>Amount</Label>
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger>
                        <Info className="h-4 w-4 text-muted-foreground" />
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>The amount of Bitcoin to be held in escrow. This will be locked until both parties agree on resolution or the timelock expires.</p>
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

              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <Label>Timelock Period</Label>
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger>
                        <Info className="h-4 w-4 text-muted-foreground" />
                      </TooltipTrigger>
                      <TooltipContent className="max-w-[300px]">
                        <p>If no agreement is reached within this time, the funds will automatically return to the sender. Choose a period that gives enough time for the transaction but protects your funds.</p>
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                </div>
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
                  Time until the escrow can be refunded
                </p>
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
            <Button>Create Escrow Transaction »</Button>
          </div>
        </CardContent>
      </Card>
    </div>
  )
}

