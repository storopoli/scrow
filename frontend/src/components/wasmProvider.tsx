// components/ClientWasmProvider.tsx
"use client";

import { useEffect, useState } from "react";
import { init } from "../lib/wasm";

export function ClientWasmProvider({ children }: { children: React.ReactNode }) {
  const [wasmLoaded, setWasmLoaded] = useState(false);

  useEffect(() => {
    init().then(() => setWasmLoaded(true));
    console.log("WASM is active!");
  }, []);

  if (!wasmLoaded) return <div>Loading WASM...</div>;
  return <>{children}</>;
}