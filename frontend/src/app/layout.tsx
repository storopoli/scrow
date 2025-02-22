// app/layout.tsx
import type React from "react";
import "../app/globals.css";
import { Inter } from "next/font/google";
import { Sidebar } from "@/components/sidebar";
import { Toaster } from "sonner";
import { ClientWasmProvider } from "@/components/wasmProvider";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <ClientWasmProvider>
          <div className="flex h-screen bg-background">
            <Sidebar />
            <div className="flex-1 p-6 overflow-auto">{children}</div>
          </div>
          <Toaster theme="dark" position="top-right" />
        </ClientWasmProvider>
      </body>
    </html>
  );
}