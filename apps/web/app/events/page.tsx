import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { events } from "@/lib/demo-data";

export default function EventsPage() {
  return (
    <AppShell>
      <div className="toolbar">
        <h2 className="page-title">Live Events</h2>
        <span className="badge ok">SSE ready</span>
      </div>
      <div className="card" style={{ marginTop: 14 }}>
        <DataTable
          rows={events}
          columns={[
            { key: "time", label: "Time" },
            { key: "service", label: "Service" },
            { key: "severity", label: "Severity", render: (row) => <span className={`badge ${row.severity}`}>{row.severity}</span> },
            { key: "message", label: "Message" },
          ]}
        />
      </div>
    </AppShell>
  );
}

