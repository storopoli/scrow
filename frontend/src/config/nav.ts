import { Bitcoin, Send, Download, List, Database, Settings } from "lucide-react"
import type { LucideIcon } from "lucide-react"

interface NavItem {
  icon: LucideIcon
  label: string
  path: string
}

export const navItems: NavItem[] = [
  { icon: Bitcoin, label: "Escrows", path: "/escrows" },
  { icon: Send, label: "Create Escrow", path: "/" },
  { icon: Download, label: "Join Escrow", path: "/join" },
  { icon: List, label: "History", path: "/history" },
  { icon: Database, label: "UTXOs", path: "/utxos" },
  { icon: Settings, label: "Settings", path: "/settings" },
] 