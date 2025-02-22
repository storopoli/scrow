"use client";

import Image from "next/image";
import { usePathname, useRouter } from "next/navigation";
import { navItems } from "@/config/nav";
import { ChevronLeft, ChevronRight } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { cn } from "@/lib/utils";

export function Sidebar() {
  const router = useRouter();
  const pathname = usePathname();
  const [collapsed, setCollapsed] = useState(false);

  return (
    <div className="relative flex h-screen flex-col">
      <div
        className={cn(
          "bg-gradient-to-b from-zinc-900 to-black border-r border-zinc-800/40 text-white p-4 flex flex-col flex-1",
          collapsed ? "w-[80px]" : "w-[240px]",
        )}
      >
        <div className="flex items-center gap-2 mb-8">
          <Image
            src="/SatoshiEscrowLogo 1.svg"
            alt="Scrow Logo"
            width={32}
            height={32}
            className="w-8 h-8"
          />
          <div
            className={cn(
              "text-2xl font-bold bg-gradient-to-r from-orange-500 to-orange-600 text-transparent bg-clip-text transition-opacity duration-200",
              collapsed ? "opacity-0 w-0" : "opacity-100",
            )}
          >
            scrow
          </div>
        </div>
        <nav className="space-y-2">
          {navItems.map((item) => (
            <button
              key={item.label}
              onClick={() => router.push(item.path)}
              className={cn(
                "flex items-center w-full p-3 rounded-lg transition-all duration-200",
                pathname === item.path
                  ? "bg-orange-500/10 text-orange-500"
                  : "hover:bg-orange-500/10 hover:text-orange-500",
                collapsed ? "justify-center" : "",
              )}
            >
              <item.icon className="w-5 h-5" />
              <span
                className={cn(
                  "ml-3 transition-opacity duration-200",
                  collapsed ? "opacity-0 w-0" : "opacity-100",
                )}
              >
                {item.label}
              </span>
            </button>
          ))}
        </nav>

        <div className="mt-auto pt-4 text-center">
          <p
            className={cn(
              "text-white/80 text-sm transition-opacity duration-200",
              collapsed ? "opacity-0" : "opacity-100",
            )}
          >
            Satoshi Escrow
          </p>
        </div>
      </div>

      <Button
        variant="ghost"
        size="icon"
        className="absolute -right-3 top-6 h-6 w-8 rounded-full border border-zinc-800/40 bg-white text-zinc-900 hover:bg-white hover:text-zinc-900 hover:scale-110 transition-transform z-50"
        onClick={() => setCollapsed(!collapsed)}
      >
        {collapsed ? (
          <ChevronRight className="h-4 w-4" />
        ) : (
          <ChevronLeft className="h-4 w-4" />
        )}
      </Button>
    </div>
  );
}
