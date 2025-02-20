"use client"

import { Bitcoin, Send, Download, List, Database, Settings, Copy, Camera, Info } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Slider } from "@/components/ui/slider"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Card, CardContent } from "@/components/ui/card"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip"

export default function EscrowInterface() {
  return (
    <div className="flex h-screen bg-background">
      {/* Sidebar */}
      <div className="w-[240px] bg-gradient-to-b from-zinc-900 to-black border-r border-zinc-800/40 text-white p-4 space-y-4">
        <div className="text-2xl font-bold mb-8 bg-gradient-to-r from-orange-500 to-orange-600 text-transparent bg-clip-text">
          scrow
        </div>
        <nav className="space-y-2">
          {[
            { icon: Bitcoin, label: "Escrows" },
            { icon: Send, label: "Create Escrow" },
            { icon: Download, label: "Join Escrow" },
            { icon: List, label: "History" },
            { icon: Database, label: "UTXOs" },
            { icon: Settings, label: "Settings" },
          ].map((item) => (
            <button
              key={item.label}
              className="flex items-center w-full p-3 rounded-lg hover:bg-orange-500/10 hover:text-orange-500 transition-all duration-200"
            >
              <item.icon className="w-5 h-5 mr-3" />
              {item.label}
            </button>
          ))}
        </nav>
      </div>

      {/* Main Content */}
      <div className="flex-1 p-6 overflow-auto">
        <div className="max-w-3xl mx-auto">
          <h1 className="text-2xl font-bold mb-6">Create Escrow</h1>

          <Card>
            <CardContent className="p-6 space-y-6">
              {/* Escrow Details */}
              <div className="space-y-4">
                <div className="space-y-2">
                  <Label>Counterparty Bitcoin Address</Label>
                  <div className="flex gap-2">
                    <Input placeholder="bc1qe2jmxmuytvcyzack9n9ng2sjjzr2sdryyae75zr4euf1392wkfqsuntdsd" />
                    <Button variant="outline" size="icon">
                      <Copy className="h-4 w-4" />
                    </Button>
                    <Button variant="outline" size="icon">
                      <Camera className="h-4 w-4" />
                    </Button>
                  </div>
                </div>

                <div className="space-y-2">
                  <Label>Description</Label>
                  <Input placeholder="Enter escrow description" />
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label>Amount</Label>
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
                    <Label>Timelock Period</Label>
                    <Select defaultValue="24h">
                      <SelectTrigger>
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="24h">24 hours</SelectItem>
                        <SelectItem value="48h">48 hours</SelectItem>
                        <SelectItem value="72h">72 hours</SelectItem>
                        <SelectItem value="1w">1 week</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                </div>
              </div>

              {/* Fee Settings */}
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
      </div>
    </div>
  )
}

