"use client";

import { Upload, FileUp, Github } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useState } from "react";
import { toast } from "sonner";

interface BroadcastResponse {
  txid: string;
}

export default function BroadcastEscrowPage() {
  const [signedTxText, setSignedTxText] = useState("");

  const handleBroadcast = async () => {
    const promise = new Promise<BroadcastResponse>((resolve, reject) => {
      setTimeout(() => {
        fetch("/api/broadcast", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ psbt: signedTxText }),
        })
          .then((response) => {
            if (!response.ok)
              throw new Error("Failed to broadcast transaction");
            return response.json();
          })
          .then((data: BroadcastResponse) => {
            setSignedTxText("");
            resolve(data);
          })
          .catch((error) => reject(error));
      }, 1000);
    });

    toast.promise(promise, {
      loading: "Broadcasting transaction...",
      success: (data) =>
        `Transaction broadcasted successfully! TXID: ${data.txid}`,
      error: (error: Error) =>
        `Error: ${error.message || "Failed to broadcast transaction"}`,
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
