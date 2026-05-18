import { Plus } from "lucide-react";
import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { rules } from "@/lib/demo-data";

export default function RulesPage() {
  return (
    <AppShell>
      <div className="page-heading">
        <div>
          <h2 className="page-title">Alert Rules</h2>
          <p className="page-kicker">Define service filters and minimum severity for incident creation.</p>
        </div>
        <Button type="button">
          <Plus size={16} aria-hidden />
          New rule
        </Button>
      </div>
      <Card style={{ marginBottom: 14 }}>
        <CardHeader>
          <CardTitle>Quick rule draft</CardTitle>
          <CardDescription>Local UI stub for the upcoming `POST /api/rules` workflow.</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="rule-form">
            <Input placeholder="Rule name" defaultValue="Critical checkout errors" />
            <Input placeholder="Service" defaultValue="checkout-api" />
            <Button type="button">Create</Button>
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>Configured rules</CardTitle>
          <CardDescription>Rules synced from incident-service persistence.</CardDescription>
        </CardHeader>
        <CardContent>
        <DataTable
          rows={rules}
          columns={[
            { key: "name", label: "Name" },
            { key: "service", label: "Service" },
            { key: "severity", label: "Min severity", render: (row) => <Badge variant={row.severity === "critical" ? "critical" : "warning"}>{row.severity}</Badge> },
            { key: "enabled", label: "Enabled", render: (row) => <Badge variant={row.enabled ? "success" : "outline"}>{row.enabled ? "yes" : "no"}</Badge> },
          ]}
        />
        </CardContent>
      </Card>
    </AppShell>
  );
}
