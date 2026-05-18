"use client";

import { Plus } from "lucide-react";
import { useState } from "react";
import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import type { Severity } from "@/lib/api";
import { useLiveData } from "@/lib/use-live-data";

export default function RulesPage() {
  const live = useLiveData();
  const [name, setName] = useState("Critical checkout errors");
  const [service, setService] = useState("checkout-api");
  const [minSeverity, setMinSeverity] = useState<Severity>("critical");
  const [submitting, setSubmitting] = useState(false);

  async function submitRule() {
    setSubmitting(true);
    try {
      await live.submitRule({
        name,
        service: service || null,
        min_severity: minSeverity,
      });
    } finally {
      setSubmitting(false);
    }
  }

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
            <Input placeholder="Rule name" value={name} onChange={(event) => setName(event.target.value)} />
            <Input placeholder="Service" value={service} onChange={(event) => setService(event.target.value)} />
            <select className="sf-input" value={minSeverity} onChange={(event) => setMinSeverity(event.target.value as Severity)}>
              <option value="info">info</option>
              <option value="warning">warning</option>
              <option value="critical">critical</option>
            </select>
            <Button onClick={submitRule} disabled={submitting || !name} type="button">
              {submitting ? "Creating" : "Create"}
            </Button>
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
          rows={live.rules}
          columns={[
            { key: "name", label: "Name" },
            { key: "service", label: "Service", render: (row) => row.service ?? "all" },
            { key: "min_severity", label: "Min severity", render: (row) => <Badge variant={row.min_severity === "critical" ? "critical" : row.min_severity === "warning" ? "warning" : "info"}>{row.min_severity}</Badge> },
            { key: "enabled", label: "Enabled", render: (row) => <Badge variant={row.enabled ? "success" : "outline"}>{row.enabled ? "yes" : "no"}</Badge> },
            {
              key: "id",
              label: "Actions",
              render: (row) => (
                <Button size="sm" variant="secondary" onClick={() => live.toggleRule(row)} type="button">
                  {row.enabled ? "Disable" : "Enable"}
                </Button>
              ),
            },
          ]}
        />
        </CardContent>
      </Card>
    </AppShell>
  );
}
