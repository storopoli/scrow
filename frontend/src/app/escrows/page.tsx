"use client";

import { Bitcoin } from "lucide-react";
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
    </div>
  );
}
