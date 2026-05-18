import Link from "next/link";
import type { ReactNode } from "react";
import { Activity, Bell, Gauge, ListChecks, Radio, ShieldAlert } from "lucide-react";

const nav = [
  { href: "/", label: "Overview", icon: Gauge },
  { href: "/events", label: "Live Events", icon: Radio },
  { href: "/incidents", label: "Incidents", icon: ShieldAlert },
  { href: "/rules", label: "Alert Rules", icon: ListChecks },
  { href: "/services/payments", label: "Service Detail", icon: Activity },
];

export function AppShell({ children }: { children: ReactNode }) {
  return (
    <div className="shell">
      <aside className="sidebar">
        <h1 className="brand">SignalForge</h1>
        <nav className="nav" aria-label="Primary">
          {nav.map((item) => (
            <Link href={item.href} key={item.href}>
              <item.icon size={18} aria-hidden />
              {item.label}
            </Link>
          ))}
        </nav>
      </aside>
      <main className="main">
        <div className="topbar">
          <div />
          <button className="button" type="button">
            <Bell size={16} aria-hidden />
            Notify test
          </button>
        </div>
        {children}
      </main>
    </div>
  );
}
