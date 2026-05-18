"use client";

import { useState } from "react";
import { AppShell } from "@/components/app-shell";
import { DataTable } from "@/components/data-table";
import { SeverityBadge } from "@/components/severity-badge";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import type { Severity } from "@/lib/api";
import { useLiveData } from "@/lib/use-live-data";

export default function EventsPage() {
  const live = useLiveData();
  const [service, setService] = useState("payments");
  const [message, setMessage] = useState("Error budget burn exceeded");
  const [severity, setSeverity] = useState<Severity>("critical");
  const [submitting, setSubmitting] = useState(false);

  async function sendEvent() {
    setSubmitting(true);
    try {
      await live.submitEvent({
        source: "dashboard",
        service,
        severity,
        message,
        attributes: { submitted_from: "ui" },
      });
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <AppShell>
      <div className="page-heading">
        <div>
          <h2 className="page-title">Live Events</h2>
          <p className="page-kicker">Streaming event feed prepared for `/api/events/stream`.</p>
        </div>
        <Badge variant={live.streamState === "live" ? "success" : "warning"}>
          {live.streamState}
        </Badge>
      </div>
      <Card style={{ marginBottom: 14 }}>
        <CardHeader>
          <CardTitle>Send test event</CardTitle>
          <CardDescription>Posts to `POST /api/ingest/events`; matching rules create incidents.</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="rule-form">
            <Input value={service} onChange={(event) => setService(event.target.value)} placeholder="Service" />
            <Input value={message} onChange={(event) => setMessage(event.target.value)} placeholder="Message" />
            <select className="sf-input" value={severity} onChange={(event) => setSeverity(event.target.value as Severity)}>
              <option value="info">info</option>
              <option value="warning">warning</option>
              <option value="critical">critical</option>
            </select>
            <Button onClick={sendEvent} disabled={submitting || !service || !message} type="button">
              {submitting ? "Sending" : "Send event"}
            </Button>
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>Raw ingest stream</CardTitle>
          <CardDescription>Events are sorted by receipt time with severity normalization.</CardDescription>
        </CardHeader>
        <CardContent>
        <DataTable
          rows={live.events}
          columns={[
            { key: "received_at", label: "Time", render: (row) => live.formatTime(row.received_at) },
            { key: "service", label: "Service" },
            { key: "severity", label: "Severity", render: (row) => <SeverityBadge severity={row.severity} /> },
            { key: "message", label: "Message" },
          ]}
        />
        </CardContent>
      </Card>
    </AppShell>
  );
}
