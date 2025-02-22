import { Send, Download, Upload, Settings } from "lucide-react";
import type { LucideIcon } from "lucide-react";

interface NavItem {
  icon: LucideIcon;
  label: string;
  path: string;
}

export const navItems: NavItem[] = [
  { icon: Send, label: "Create Escrow", path: "/" },
  { icon: Download, label: "Sign Escrow", path: "/sign" },
  { icon: Upload, label: "Broadcast Escrow", path: "/broadcast" },
  { icon: Settings, label: "Settings", path: "/settings" },
];
