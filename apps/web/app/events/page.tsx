import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { events } from "@/lib/demo-data";

export default function EventsPage() {
  return (
    <AppShell>
      <div className="page-heading">
        <div>
          <h2 className="page-title">Live Events</h2>
          <p className="page-kicker">Streaming event feed prepared for `/api/events/stream`.</p>
        </div>
        <Badge variant="success">SSE ready</Badge>
      </div>
      <Card>
        <CardHeader>
          <CardTitle>Raw ingest stream</CardTitle>
          <CardDescription>Events are sorted by receipt time with severity normalization.</CardDescription>
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
    </AppShell>
  );
}
