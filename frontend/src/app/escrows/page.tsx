"use client";

import { Bitcoin, Github } from "lucide-react";
import { Card, CardContent } from "@/components/ui/card";

export default function EscrowsPage() {
  return (
    <div className="max-w-3xl mx-auto">
      <h1 className="text-2xl font-bold mb-6">My Escrows</h1>
      <Card>
        <CardContent className="p-6">
          <div className="text-center py-8 text-muted-foreground">
            <Bitcoin className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>No active escrows found</p>
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
