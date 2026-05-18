import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { events, incidents, metrics } from "@/lib/demo-data";

export default function OverviewPage() {
  return (
    <AppShell>
      <div className="page-heading">
        <div>
          <h2 className="page-title">Overview</h2>
          <p className="page-kicker">Realtime service health, live ingest, and active response work.</p>
        </div>
        <Badge variant="success">all systems ingesting</Badge>
      </div>
      <section className="grid metrics" style={{ marginTop: 14 }}>
        {metrics.map((metric) => (
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
            rows={events}
            columns={[
              { key: "time", label: "Time" },
              { key: "service", label: "Service" },
              { key: "severity", label: "Severity", render: (row) => <Badge variant={row.severity === "critical" ? "critical" : row.severity === "warning" ? "warning" : "info"}>{row.severity}</Badge> },
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
            rows={incidents.filter((incident) => incident.status !== "resolved")}
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
