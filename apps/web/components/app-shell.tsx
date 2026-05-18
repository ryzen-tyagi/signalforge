"use client";

import Link from "next/link";
import type { ReactNode } from "react";
import { useState } from "react";
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
import { ingestEvent } from "@/lib/api";

const nav = [
  { href: "/", label: "Overview", icon: Gauge },
  { href: "/events", label: "Live Events", icon: Radio },
  { href: "/incidents", label: "Incidents", icon: ShieldAlert },
  { href: "/rules", label: "Alert Rules", icon: ListChecks },
  { href: "/services/payments", label: "Service Detail", icon: Activity },
];

export function AppShell({ children }: { children: ReactNode }) {
  const [sending, setSending] = useState(false);

  async function sendNotifyTest() {
    setSending(true);
    try {
      await ingestEvent({
        source: "topbar",
        service: "payments",
        severity: "critical",
        message: "Manual notification test from UI",
        attributes: { action: "notify_test" },
      });
    } finally {
      setSending(false);
    }
  }

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
            <Button
              variant="ghost"
              size="icon"
              aria-label="Search"
              onClick={() => window.dispatchEvent(new KeyboardEvent("keydown", { key: "/" }))}
              type="button"
            >
              <Search size={17} aria-hidden />
            </Button>
            <Button variant="secondary" onClick={() => window.location.assign("/events")} type="button">
              <Radio size={16} aria-hidden />
              Stream
            </Button>
            <Button onClick={sendNotifyTest} disabled={sending} type="button">
            <Bell size={16} aria-hidden />
            {sending ? "Sending" : "Notify test"}
            </Button>
          </div>
        </div>
        {children}
      </main>
    </div>
  );
}
