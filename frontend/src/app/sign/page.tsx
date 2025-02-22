"use client";

import { Download, Github } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { toast } from "sonner";
import { useState } from "react";

export default function SignEscrowPage() {
  const [signedTx, setSignedTx] = useState("");

  const handleSignTransaction = async () => {
    try {
      // Get the signed transaction from mock function
      const signed_tx_hex = mockHandleSignTransaction();
      setSignedTx(signed_tx_hex);
      toast.success("Transaction signed successfully");
    } catch (err) {
      const error = err as Error;
      toast.error(`Failed to sign: ${error.message}`);
    }
  };

  const mockHandleSignTransaction = () => {
    let signed_tx_hex = "02000000000101a7c432d45d3f1f9241e2ead6936ab3785989c2956f77e95989433c655c5741140000000000fdffffff02b81c00000000000017a9144f0a8c78f59a8d6eb08c1cc871a2bbed5b0cd73c87281c00000000000017a914e310267f38de36302f18e3e07fd2e9e93abbdc4c870247304402203b3539bb84747462f6c6dfe0e089aa9462ede97964d17c04b7d1bb40d0dc897a0220508f5c68249bd935c92133ee547ac06ed6f97a0db19f322c91853a20ccf9f1780121034cbe9dc74235d0bd9ad0f0f7ba09daf78f348a5eda6107a92118b49f17f03c4a00000000";
    return signed_tx_hex;
  }

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Sign Escrow</h1>
      <Card>
        <CardHeader>
          <CardTitle className="text-lg font-medium">
            Sign Raw Transaction
          </CardTitle>
        </CardHeader>
        <CardContent className="p-6 space-y-4">
          <div className="space-y-2">
            <Label>Raw Transaction</Label>
            <Input
              placeholder="Raw transaction hex"
              className="font-mono"
            />
            <p className="text-sm text-muted-foreground">
              Paste the raw transaction hex that needs to be signed
            </p>
          </div>

          <div className="space-y-2">
            <Label>Nsec</Label>
            <Input
              type="password"
              placeholder="Your nsec key"
              className="font-mono"
            />
            <p className="text-sm text-muted-foreground">
              Enter your <b>nsec</b> to sign the transaction
            </p>
          </div>

          <div className="pt-4 border-t border-zinc-800">
            <Button className="w-full" onClick={handleSignTransaction}>
              <Download className="w-4 h-4 mr-2" />
              Sign Transaction
            </Button>
          </div>

          {signedTx && (
            <div className="pt-4 space-y-2">
              <Label>Signed Transaction</Label>
              <Input
                value={signedTx}
                className="font-mono bg-muted"
              />
              <p className="text-sm text-muted-foreground">
                This is your signed transaction hex
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