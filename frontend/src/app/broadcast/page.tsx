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
  const [signedTxFile, setSignedTxFile] = useState<File | null>(null);
  const [signedTxText, setSignedTxText] = useState("");

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setSignedTxFile(file);
      const reader = new FileReader();
      reader.onload = (e) => {
        const text = e.target?.result as string;
        setSignedTxText(text);
      };
      reader.readAsText(file);
    }
  };

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
            setSignedTxFile(null);
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
          <Tabs defaultValue="upload" className="space-y-4">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="upload">Upload File</TabsTrigger>
              <TabsTrigger value="paste">Paste Signed Transaction</TabsTrigger>
            </TabsList>

            <TabsContent value="upload" className="space-y-4">
              <div className="border-2 border-dashed border-zinc-800 rounded-lg p-6 text-center">
                <Input
                  type="file"
                  accept=".psbt"
                  onChange={handleFileUpload}
                  className="hidden"
                  id="psbt-upload"
                />
                <Label
                  htmlFor="psbt-upload"
                  className="flex flex-col items-center gap-2 cursor-pointer"
                >
                  <FileUp className="h-8 w-8 text-muted-foreground" />
                  <span className="text-muted-foreground">
                    {signedTxFile
                      ? signedTxFile.name
                      : "Click to upload signed transaction file"}
                  </span>
                </Label>
              </div>
              {signedTxFile && (
                <div className="space-y-2">
                  <Label>File Content Preview</Label>
                  <div className="bg-zinc-900 p-3 rounded-lg font-mono text-sm break-all">
                    {signedTxText.slice(0, 100)}...
                  </div>
                </div>
              )}
            </TabsContent>

            <TabsContent value="paste" className="space-y-4">
              <div className="space-y-2">
                <Label>Signed Transaction Data</Label>
                <Input
                  placeholder="Paste your signed transaction data here"
                  value={signedTxText}
                  onChange={(e) => setSignedTxText(e.target.value)}
                  className="font-mono"
                />
              </div>
            </TabsContent>
          </Tabs>

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
