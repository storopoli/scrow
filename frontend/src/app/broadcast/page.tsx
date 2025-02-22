"use client";

import { Upload, Github } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { useState } from "react";
import { toast } from "sonner";

interface BroadcastResponse {
  txid: string;
}

export default function BroadcastEscrowPage() {
  const [signedTxText, setSignedTxText] = useState("14c63951b1308aafdf7fc3f78526a9768fd937a7c11700f69d9a89979e8c5845");
  const [broadcastTxid, setBroadcastTxid] = useState("");

  const handleBroadcast = async () => {
    const promise = new Promise<BroadcastResponse>((resolve) => {
      setTimeout(() => {
        // Generate a mock TXID that looks like a real Bitcoin transaction ID
        const mockTxid = "b839ef663c8ec2dbd6eee4f49691a2159006f98d4f3d329c9c59af8013e8121f";
        resolve({ txid: mockTxid });
        setBroadcastTxid(mockTxid); // Set the TXID in state
        setSignedTxText(""); // Clear the input after successful broadcast
      }, 1000);
    });

    toast.promise(promise, {
      loading: "Broadcasting transaction...",
      success: (data) =>
        `Transaction broadcasted successfully! TXID: ${data.txid}`,
      error: "Failed to broadcast transaction", // This won't be called in our mock version
    });
  };

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Broadcast Escrow</h1>
      <Card>
        <CardHeader>
          <CardTitle className="text-lg font-medium">
            Upload or paste your signed transaction to broadcast
          </CardTitle>
        </CardHeader>
        <CardContent className="p-6 space-y-6">
          <div className="space-y-2">
            <Label>Signed Transaction Data</Label>
            <Input
              placeholder="Paste your signed transaction data here"
              value={signedTxText}
              onChange={(e) => setSignedTxText(e.target.value)}
              className="font-mono"
            />
          </div>
          <div className="space-y-4 pt-4 border-t border-zinc-800">
            <Button
              className="w-full"
              disabled={!signedTxText}
              onClick={handleBroadcast}
            >
              <Upload className="w-4 h-4 mr-2" />
              Broadcast Transaction
            </Button>
            {signedTxText && (
              <div className="text-sm text-muted-foreground">
                * This action is irreversible. The transaction will be
                broadcasted to the Bitcoin network.
              </div>
            )}
          </div>

          {broadcastTxid && (
            <div className="space-y-2 pt-4 border-t border-zinc-800">
              <Label>Transaction ID</Label>
              <Input
                value={broadcastTxid}
                readOnly
                className="font-mono bg-muted"
              />
              <p className="text-sm text-muted-foreground">
                Your transaction has been successfully broadcast to the network
              </p>
            </div>
          )}
        </CardContent>
      </Card>
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