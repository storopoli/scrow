"use client"

import Image from "next/image"
import { usePathname, useRouter } from "next/navigation"
import { navItems } from "@/config/nav"

export function Sidebar() {
  const router = useRouter()
  const pathname = usePathname()

  return (
    <div className="w-[240px] bg-gradient-to-b from-zinc-900 to-black border-r border-zinc-800/40 text-white p-4 space-y-4">
      <div className="flex items-center gap-2 mb-8">
        <Image 
          src="/SatoshiEscrowLogo 1.svg"
          alt="Scrow Logo"
          width={32}
          height={32}
          className="w-8 h-8"
        />
        <div className="text-2xl font-bold bg-gradient-to-r from-orange-500 to-orange-600 text-transparent bg-clip-text">
          scrow
        </div>
      </div>
      <nav className="space-y-2">
        {navItems.map((item) => (
          <button
            key={item.label}
            onClick={() => router.push(item.path)}
            className={`flex items-center w-full p-3 rounded-lg transition-all duration-200
              ${pathname === item.path 
                ? "bg-orange-500/10 text-orange-500" 
                : "hover:bg-orange-500/10 hover:text-orange-500"
              }`}
          >
            <item.icon className="w-5 h-5 mr-3" />
            {item.label}
          </button>
        ))}
      </nav>
    </div>
  )
} 