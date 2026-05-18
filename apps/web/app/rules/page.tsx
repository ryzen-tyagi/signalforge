import { Plus } from "lucide-react";
import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { rules } from "@/lib/demo-data";

export default function RulesPage() {
  return (
    <AppShell>
      <div className="toolbar">
        <h2 className="page-title">Alert Rules</h2>
        <button className="button" type="button">
          <Plus size={16} aria-hidden />
          New rule
        </button>
      </div>
      <div className="card" style={{ marginTop: 14 }}>
        <DataTable
          rows={rules}
          columns={[
            { key: "name", label: "Name" },
            { key: "service", label: "Service" },
            { key: "severity", label: "Min severity", render: (row) => <span className={`badge ${row.severity}`}>{row.severity}</span> },
            { key: "enabled", label: "Enabled", render: (row) => (row.enabled ? "yes" : "no") },
          ]}
        />
      </div>
    </AppShell>
  );
}

