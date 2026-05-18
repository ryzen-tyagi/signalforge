import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { events, incidents, metrics } from "@/lib/demo-data";

export default function OverviewPage() {
  return (
    <AppShell>
      <h2 className="page-title">Overview</h2>
      <section className="grid metrics" style={{ marginTop: 14 }}>
        {metrics.map((metric) => (
          <div className="card" key={metric.label}>
            <span className="metric-label">{metric.label}</span>
            <span className={`metric-value ${metric.tone}`}>{metric.value}</span>
          </div>
        ))}
      </section>
      <section className="grid" style={{ gridTemplateColumns: "1.3fr 1fr", marginTop: 14 }}>
        <div className="card">
          <div className="toolbar">
            <h3>Recent events</h3>
          </div>
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
        <div className="card">
          <h3>Open incidents</h3>
          <DataTable
            rows={incidents.filter((incident) => incident.status !== "resolved")}
            columns={[
              { key: "id", label: "ID" },
              { key: "service", label: "Service" },
              { key: "status", label: "Status" },
            ]}
          />
        </div>
      </section>
    </AppShell>
  );
}

