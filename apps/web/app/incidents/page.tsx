"use client";

import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import type { IncidentStatus } from "@/lib/api";
import { useLiveData } from "@/lib/use-live-data";

export default function IncidentsPage() {
  const live = useLiveData();
  const activeCount = live.incidents.filter((incident) => incident.status !== "resolved").length;
  const statusVariant = (status: IncidentStatus) => status === "resolved" ? "success" : "outline";

  return (
    <AppShell>
      <div className="page-heading">
        <div>
          <h2 className="page-title">Incidents</h2>
          <p className="page-kicker">Track ownership, status, and severity for response work.</p>
        </div>
        <Badge variant={activeCount > 0 ? "critical" : "success"}>{activeCount} active</Badge>
      </div>
      <Card>
        <CardHeader>
          <CardTitle>Response queue</CardTitle>
          <CardDescription>Open and acknowledged incidents from the incident service.</CardDescription>
        </CardHeader>
        <CardContent>
        <DataTable
          rows={live.incidents}
          columns={[
            { key: "id", label: "ID" },
            { key: "title", label: "Title" },
            { key: "service", label: "Service" },
            { key: "severity", label: "Severity", render: (row) => <Badge variant={row.severity === "critical" ? "critical" : "warning"}>{row.severity}</Badge> },
            { key: "status", label: "Status", render: (row) => <Badge variant={statusVariant(row.status)}>{row.status}</Badge> },
            {
              key: "created_at",
              label: "Actions",
              render: (row) => (
                <div className="topbar-actions">
                  <Button size="sm" variant="secondary" onClick={() => live.setIncidentStatus(row.id, "acknowledged")} type="button">Ack</Button>
                  <Button size="sm" variant="ghost" onClick={() => live.setIncidentStatus(row.id, "resolved")} type="button">Resolve</Button>
                </div>
              ),
            },
          ]}
        />
        </CardContent>
      </Card>
    </AppShell>
  );
}
