import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { incidents } from "@/lib/demo-data";

export default function IncidentsPage() {
  return (
    <AppShell>
      <h2 className="page-title">Incidents</h2>
      <div className="card" style={{ marginTop: 14 }}>
        <DataTable
          rows={incidents}
          columns={[
            { key: "id", label: "ID" },
            { key: "title", label: "Title" },
            { key: "service", label: "Service" },
            { key: "severity", label: "Severity", render: (row) => <span className={`badge ${row.severity}`}>{row.severity}</span> },
            { key: "status", label: "Status" },
          ]}
        />
      </div>
    </AppShell>
  );
}

