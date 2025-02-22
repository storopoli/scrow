/* eslint-disable @typescript-eslint/no-unused-vars */

"use client";

import { Copy, Camera, Info, Github } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Slider } from "@/components/ui/slider";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Card, CardContent } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Switch } from "@/components/ui/switch";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { useBitcoinPrice } from "@/hooks/useBitcoinPrice";
import { cn } from "@/lib/utils";

// call rust functions via wasm.<function name>
import { init } from "../lib/wasm";

import wasm from "../lib/wasm";

type FeePreset = "minimum" | "economic" | "fastest";
type NetworkType = "MutinyNet" | "Signet" | "Mainnet" | "Testnet4";
type FeeMap = Map<string, number>;

const MAINNET_HOST = "https://mempool.space";
const MUTINY_HOST = "https://mutinynet.com";
const MUTINY_FEE_ENDPOINT = "/api/v1/fees/recommended";

export default function CreateEscrowPage() {
  const [useThirdParty, setUseThirdParty] = useState(false);
  const [includeThirdPartyAddress, setIncludeThirdPartyAddress] =
    useState(false);
  const [amount, setAmount] = useState("");
  const [unit, setUnit] = useState<"sats" | "btc" | "usd">("sats");
  const [network, setNetwork] = useState<NetworkType>("MutinyNet");

  const { price, loading } = useBitcoinPrice();
  const [feeRate, setFeeRate] = useState(1);
  const [feePreset, setFeePreset] = useState<FeePreset>("economic");
  const [fees, setFees] = useState<FeeMap | null>(null);
  const [myNostrPubkey, setMyNostrPubkey] = useState("");
  const [myBitcoinAddress, setMyBitcoinAddress] = useState("");
  const [counterpartyNostrPubkey, setCounterpartyNostrPubkey] = useState("");
  const [counterpartyBitcoinAddress, setCounterpartyBitcoinAddress] =
    useState("");
  const [thirdPartyNostrPubkey, setThirdPartyNostrPubkey] = useState("");
  const [timelockPeriod, setTimelockPeriod] = useState("1h");

  useEffect(() => {
    const initAndFetchFees = async () => {
      try {
        const feesMap = await wasm.fetch_fees(
          MAINNET_HOST,
          MUTINY_FEE_ENDPOINT,
        );
        setFees(feesMap);
      } catch (error) {
        console.error("Error:", error);
      }
    };

    initAndFetchFees();
  }, []);

  const minimumFee = fees?.get("minimumFee") ?? 1;
  const economyFee = fees?.get("economyFee") ?? 2;
  const fastestFee = fees?.get("fastestFee") ?? 3;

  const handleAmountChange = (value: string) => {
    setAmount(value);
  };

  const getConversion = () => {
    if (!price?.usd || !amount || isNaN(Number(amount))) return null;

    const btcPrice = price.usd;
    const satsToBtc = 1e-8;
    const btcToSats = 1e8;

    switch (unit) {
      case "sats":
        const satsAmount = Number(amount);
        return `≈ $${(satsAmount * satsToBtc * btcPrice).toFixed(2)} USD`;
      case "btc":
        const btcAmount = Number(amount);
        return `≈ $${(btcAmount * btcPrice).toFixed(2)} USD`;
      case "usd":
        const usdAmount = Number(amount);
        return `≈ ${((usdAmount / btcPrice) * btcToSats).toFixed(0)} sats`;
    }
  };

  const handleFeePresetChange = (value: string) => {
    const preset = value as FeePreset;
    setFeePreset(preset);
    switch (preset) {
      case "minimum":
        setFeeRate(minimumFee);
        break;
      case "economic":
        setFeeRate(economyFee);
        break;
      case "fastest":
        setFeeRate(fastestFee);
        break;
    }
  };

  const handleCreateEscrow = async () => {
    try {
      // Collect all input values
      const escrowData = {
        // Network
        network,

        // Amount details
        amount: Number(amount),
        unit,

        // My details
        myNostrPubkey,
        myBitcoinAddress,

        // Counterparty details
        counterpartyNostrPubkey,
        counterpartyBitcoinAddress,

        // Fee details
        feeRate,
        feePreset,

        // Third party details (if enabled)
        useThirdParty,
        ...(useThirdParty && {
          thirdPartyNostrPubkey,
          timelockPeriod,
        }),
      };

      console.log("Escrow Data:", escrowData);

      // Call your WASM function here
      const collab_address = await wasm.create_collab_address(myNostrPubkey, counterpartyNostrPubkey, network);
      const dispute_address = await wasm.create_dispute_address(myNostrPubkey, counterpartyNostrPubkey, thirdPartyNostrPubkey || "", Number(timelockPeriod) * 6 || 6, network);

      console.log(collab_address);
      console.log(dispute_address);

      toast.success("Escrow created successfully");
    } catch (err) {
      const error = err as Error;
      toast.error(`Failed to create escrow: ${error.message}`);
    }
  };

  return (
    <div className="max-w-3xl mx-auto relative pb-16">
      <h1 className="text-2xl font-bold mb-6">Create Escrow</h1>
      <Card>
        <CardContent className="p-6 space-y-6">
          <div className="space-y-6">
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h2 className="text-lg font-semibold">Bitcoin Network</h2>
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger>
                      <Info className="h-4 w-4 text-muted-foreground" />
                    </TooltipTrigger>
                    <TooltipContent className="max-w-[300px]">
                      <p>
                        This is the Bitcoin network used for the transaction.
                      </p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </div>

              {/* Bitcoin Network Section */}
              <div className="space-y-4 p-4 rounded-lg border border-zinc-800">
                <Select
                  value={network}
                  onValueChange={(
                    value: "MutinyNet" | "Signet" | "Testnet4" | "Mainnet",
                  ) => setNetwork(value)}
                >
                  <SelectTrigger className="w-[200px]">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="MutinyNet">MutinyNet</SelectItem>
                    <SelectItem value="Signet">Signet</SelectItem>
                    <SelectItem value="Testnet4">Testnet4</SelectItem>
                    <SelectItem value="Mainnet">Mainnet</SelectItem>
                  </SelectContent>
                </Select>
              </div>

              <div className="flex items-center justify-between">
                <h2 className="text-lg font-semibold">Participant Details</h2>
                <TooltipProvider>
                  <Tooltip>
                    <TooltipTrigger>
                      <Info className="h-4 w-4 text-muted-foreground" />
                    </TooltipTrigger>
                    <TooltipContent className="max-w-[300px]">
                      <p>
                        Both parties need to provide their Nostr public key for
                        communication and a Bitcoin address for fund resolution.
                        The escrow will be locked until both parties agree or
                        the timelock expires.
                      </p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </div>

              {/* My Details */}
              <div className="space-y-4 p-4 rounded-lg border border-zinc-800">
                <div className="flex items-center justify-between">
                  <h3 className="text-sm font-medium text-muted-foreground">
                    My Details
                  </h3>
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger>
                        <Info className="h-4 w-4 text-muted-foreground" />
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>
                          Your Nostr key will be used to sign messages and your
                          Bitcoin address will receive the funds if the escrow
                          resolves in your favor.
                        </p>
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
                      value={myNostrPubkey}
                      onChange={(e) => setMyNostrPubkey(e.target.value)}
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
                      value={myBitcoinAddress}
                      onChange={(e) => setMyBitcoinAddress(e.target.value)}
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
                  <h3 className="text-sm font-medium text-muted-foreground">
                    Counterparty Details
                  </h3>
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger>
                        <Info className="h-4 w-4 text-muted-foreground" />
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>
                          The other party&apos;s Nostr key for communication and
                          Bitcoin address where they&apos;ll receive funds if
                          the escrow resolves in their favor.
                        </p>
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
                      value={counterpartyNostrPubkey}
                      onChange={(e) =>
                        setCounterpartyNostrPubkey(e.target.value)
                      }
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
                      value={counterpartyBitcoinAddress}
                      onChange={(e) =>
                        setCounterpartyBitcoinAddress(e.target.value)
                      }
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
                  <h3 className="text-sm font-medium text-muted-foreground">
                    Third Party Resolution
                  </h3>
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger>
                        <Info className="h-4 w-4 text-muted-foreground" />
                      </TooltipTrigger>
                      <TooltipContent className="max-w-[300px]">
                        <p>
                          A trusted third party can help resolve disputes. They
                          can only intervene after the timelock period, if both
                          parties disagree. The trusted third party can sign the
                          transaction with one of the participants
                        </p>
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
                      <Select
                        defaultValue="1h"
                        value={timelockPeriod}
                        onValueChange={setTimelockPeriod}
                      >
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
                          value={thirdPartyNostrPubkey}
                          onChange={(e) =>
                            setThirdPartyNostrPubkey(e.target.value)
                          }
                        />
                        <Button variant="outline" size="icon">
                          <Copy className="h-4 w-4" />
                        </Button>
                      </div>
                    </div>

                    {/*
                    <div className="flex items-center space-x-2">
                      <Switch
                        checked={includeThirdPartyAddress}
                        onCheckedChange={setIncludeThirdPartyAddress}
                      />
                      <Label>Include Resolution Address & Collateral</Label>
                    </div>
                    */}

                    {includeThirdPartyAddress && (
                      <div className="space-y-4">
                        {/*
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
                        */}
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
                          The amount of Bitcoin to be held in escrow. This will
                          be locked until both parties agree on resolution.
                          <br />
                          <br />
                          Real-time BTC/USD conversion rates are provided by
                          CoinGecko and update every minute.
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
                    onValueChange={(value: "sats" | "btc" | "usd") =>
                      setUnit(value)
                    }
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
                  <p className="text-sm text-muted-foreground">
                    Loading conversion...
                  </p>
                ) : (
                  amount && (
                    <p className="text-sm text-muted-foreground">
                      {getConversion()}
                    </p>
                  )
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
                  className={cn(minimumFee && "text-orange-500")}
                >
                  Economic
                </TabsTrigger>
                <TabsTrigger
                  value="recommended"
                  className={cn(economyFee && "text-orange-500")}
                >
                  Recommended
                </TabsTrigger>
                <TabsTrigger
                  value="priority"
                  className={cn(fastestFee && "text-orange-500")}
                >
                  Priority
                </TabsTrigger>
              </TabsList>
              <TabsContent value={feePreset}>
                <div className="py-4">
                  <div className="flex justify-between mt-2 text-sm text-muted-foreground">
                    <span>{feeRate} sat/vB</span>
                  </div>
                </div>
              </TabsContent>
            </Tabs>
          </div>

          <div className="pt-4 flex justify-end gap-4">
            <Button variant="outline">Clear</Button>
            <Button onClick={handleCreateEscrow}>
              Create Escrow Transaction »
            </Button>
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
  );
}
