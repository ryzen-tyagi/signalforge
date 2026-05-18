"use client";

import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { SeverityBadge } from "@/components/severity-badge";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { useLiveData } from "@/lib/use-live-data";

export default function OverviewPage() {
  const live = useLiveData();
  const activeIncidents = live.incidents.filter((incident) => incident.status !== "resolved");

  return (
    <AppShell>
      <div className="page-heading">
        <div>
          <h2 className="page-title">Overview</h2>
          <p className="page-kicker">Realtime service health, live ingest, and active response work.</p>
        </div>
        <div className="topbar-actions">
          <Badge variant={live.streamState === "live" ? "success" : "warning"}>
            {live.streamState}
          </Badge>
          <Button variant="secondary" onClick={live.refresh} type="button">Refresh</Button>
        </div>
      </div>
      {live.error ? <p className="page-kicker critical">{live.error}</p> : null}
      <section className="grid metrics" style={{ marginTop: 14 }}>
        {live.metrics.map((metric) => (
          <Card key={metric.label}>
            <CardContent>
              <span className="metric-label">{metric.label}</span>
              <div className="metric-row">
                <span className={`metric-value ${metric.tone}`}>{metric.value}</span>
                <span className="metric-trend">last 15m</span>
              </div>
            </CardContent>
          </Card>
        ))}
      </section>
      <section className="grid content-grid" style={{ marginTop: 14 }}>
        <Card>
          <CardHeader>
            <CardTitle>Recent events</CardTitle>
            <CardDescription>Latest normalized events from `sf.events.raw.v1`.</CardDescription>
          </CardHeader>
          <CardContent>
          <DataTable
            rows={live.events.slice(0, 8)}
            columns={[
              { key: "received_at", label: "Time", render: (row) => live.formatTime(row.received_at) },
              { key: "service", label: "Service" },
              { key: "severity", label: "Severity", render: (row) => <SeverityBadge severity={row.severity} /> },
              { key: "message", label: "Message" },
            ]}
          />
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>Open incidents</CardTitle>
            <CardDescription>Current response queue by service.</CardDescription>
          </CardHeader>
          <CardContent>
          <DataTable
            rows={activeIncidents}
            columns={[
              { key: "id", label: "ID" },
              { key: "service", label: "Service" },
              { key: "status", label: "Status", render: (row) => <Badge variant="outline">{row.status}</Badge> },
            ]}
          />
          </CardContent>
        </Card>
      </section>
    </AppShell>
  );
}
