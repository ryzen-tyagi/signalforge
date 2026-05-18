import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { incidents } from "@/lib/demo-data";

export default function IncidentsPage() {
  return (
    <AppShell>
      <div className="page-heading">
        <div>
          <h2 className="page-title">Incidents</h2>
          <p className="page-kicker">Track ownership, status, and severity for response work.</p>
        </div>
        <Badge variant="critical">2 active</Badge>
      </div>
      <Card>
        <CardHeader>
          <CardTitle>Response queue</CardTitle>
          <CardDescription>Open and acknowledged incidents from the incident service.</CardDescription>
        </CardHeader>
        <CardContent>
        <DataTable
          rows={incidents}
          columns={[
            { key: "id", label: "ID" },
            { key: "title", label: "Title" },
            { key: "service", label: "Service" },
            { key: "severity", label: "Severity", render: (row) => <Badge variant={row.severity === "critical" ? "critical" : "warning"}>{row.severity}</Badge> },
            { key: "status", label: "Status", render: (row) => <Badge variant={row.status === "resolved" ? "success" : "outline"}>{row.status}</Badge> },
          ]}
        />
        </CardContent>
      </Card>
    </AppShell>
  );
}
