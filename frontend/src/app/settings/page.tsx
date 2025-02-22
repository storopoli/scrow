"use client";

import { useState } from "react";
import { Settings, Github } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { toast } from "sonner";

type Network = "mainnet" | "testnet" | "signet" | "mutinynet";

interface SettingsState {
  mempoolEndpoint: string;
  network: Network;
}

export default function SettingsPage() {
  const [settings, setSettings] = useState<SettingsState>({
    mempoolEndpoint: "https://mempool.space",
    network: "testnet",
  });

  const handleSave = () => {
    // Here you would typically save to localStorage or your backend
    localStorage.setItem("settings", JSON.stringify(settings));

    toast.success("Settings saved successfully");
  };

  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">Settings</h1>
      <Card>
        <CardHeader>
          <CardTitle className="text-lg font-medium">
            Network Configuration
          </CardTitle>
        </CardHeader>
        <CardContent className="p-6 space-y-6">
          <div className="space-y-4">
            <div className="space-y-2">
              <Label>Mempool/Esplora API Endpoint</Label>
              <Input
                placeholder="Enter mempool endpoint URL"
                value={settings.mempoolEndpoint}
                onChange={(e) =>
                  setSettings({
                    ...settings,
                    mempoolEndpoint: e.target.value,
                  })
                }
              />
              <p className="text-sm text-muted-foreground">
                The endpoint used to interact with Bitcoin network
              </p>
            </div>

            <div className="space-y-2">
              <Label>Network</Label>
              <Select
                value={settings.network}
                onValueChange={(value: Network) =>
                  setSettings({
                    ...settings,
                    network: value,
                  })
                }
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="mainnet">Mainnet</SelectItem>
                  <SelectItem value="testnet">Testnet</SelectItem>
                  <SelectItem value="signet">Signet</SelectItem>
                  <SelectItem value="mutinynet">Mutinynet</SelectItem>
                </SelectContent>
              </Select>
              <p className="text-sm text-muted-foreground">
                Select which Bitcoin network to use
              </p>
            </div>

            <div className="pt-4 border-t border-zinc-800">
              <Button className="w-full" onClick={handleSave}>
                <Settings className="w-4 h-4 mr-2" />
                Save Settings
              </Button>
            </div>
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
