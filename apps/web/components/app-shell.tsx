import Link from "next/link";
import type { ReactNode } from "react";
import {
  Activity,
  Bell,
  Command,
  Gauge,
  ListChecks,
  Radio,
  Search,
  ShieldAlert,
} from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";

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
        <div className="brand-block">
          <div className="brand-mark">
            <Command size={18} aria-hidden />
          </div>
          <div>
            <h1 className="brand-title">SignalForge</h1>
            <p className="brand-subtitle">Incident command</p>
          </div>
        </div>
        <nav className="nav" aria-label="Primary">
          {nav.map((item) => (
            <Link className="nav-link" href={item.href} key={item.href}>
              <item.icon size={18} aria-hidden />
              {item.label}
            </Link>
          ))}
        </nav>
        <div className="sidebar-footer">
          <Card>
            <CardContent>
              <div className="health-row">
                <span>Ingest pipeline</span>
                <Badge variant="success">online</Badge>
              </div>
              <div className="bar" style={{ marginTop: 10 }}>
                <div className="bar-fill" style={{ width: "78%" }} />
              </div>
            </CardContent>
          </Card>
        </div>
      </aside>
      <main className="main">
        <div className="topbar">
          <div className="topbar-meta">
            <Badge variant="outline">prod</Badge>
            <span>Redpanda / Postgres / Redis</span>
          </div>
          <div className="topbar-actions">
            <Button variant="ghost" size="icon" aria-label="Search">
              <Search size={17} aria-hidden />
            </Button>
            <Button variant="secondary" type="button">
              <Radio size={16} aria-hidden />
              Stream
            </Button>
            <Button type="button">
            <Bell size={16} aria-hidden />
            Notify test
            </Button>
          </div>
        </div>
        {children}
      </main>
    </div>
  );
}
