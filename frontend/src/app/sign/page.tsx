"use client";

import { Download, Github } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { toast } from "sonner";

export default function SignEscrowPage() {
  const handleSignTransaction = async () => {
    try {
      // Handle signing logic here
      toast.success("Transaction signed successfully");
    } catch (err) {
      const error = err as Error;
      toast.error(`Failed to sign: ${error.message}`);
    }
  };

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
              placeholder="Enter raw transaction hex"
              className="font-mono"
            />
            <p className="text-sm text-muted-foreground">
              Paste the raw transaction hex that needs to be signed
            </p>
          </div>

          <div className="pt-4 border-t border-zinc-800">
            <Button className="w-full" onClick={handleSignTransaction}>
              <Download className="w-4 h-4 mr-2" />
              Sign Transaction
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
