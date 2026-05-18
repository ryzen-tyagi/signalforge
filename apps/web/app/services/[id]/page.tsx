"use client";

import { AppShell } from "@/components/app-shell";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { useLiveData } from "@/lib/use-live-data";

export default function ServiceDetailPage({ params }: { params: { id: string } }) {
  const live = useLiveData();
  const serviceEvents = live.events.filter((event) => event.service === params.id);
  const serviceIncidents = live.incidents.filter((incident) => incident.service === params.id);

  return (
    <AppShell>
      <div className="page-heading">
        <div>
          <h2 className="page-title">{params.id}</h2>
          <p className="page-kicker">Service-level telemetry and incident context.</p>
        </div>
        <Badge variant="warning">watching latency</Badge>
      </div>
      <section className="grid metrics" style={{ marginTop: 14 }}>
        <Card><CardContent><span className="metric-label">Events tracked</span><div className="metric-row"><span className="metric-value">{serviceEvents.length}</span><span className="metric-trend">sample</span></div></CardContent></Card>
        <Card><CardContent><span className="metric-label">Incidents</span><div className="metric-row"><span className="metric-value critical">{serviceIncidents.length}</span><span className="metric-trend">active</span></div></CardContent></Card>
        <Card><CardContent><span className="metric-label">p95 latency</span><div className="metric-row"><span className="metric-value warning">928ms</span><span className="metric-trend">+18%</span></div></CardContent></Card>
        <Card><CardContent><span className="metric-label">SLO</span><div className="metric-row"><span className="metric-value ok">99.92%</span><span className="metric-trend">30d</span></div></CardContent></Card>
      </section>
      <section className="grid content-grid" style={{ marginTop: 14 }}>
        <Card>
          <CardHeader>
            <CardTitle>Error budget</CardTitle>
            <CardDescription>Burn-rate snapshot for the current service window.</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="service-health">
              <div className="health-row"><span>Availability</span><span>99.92%</span></div>
              <div className="bar"><div className="bar-fill" style={{ width: "92%" }} /></div>
              <div className="health-row"><span>Latency objective</span><span>82%</span></div>
              <div className="bar"><div className="bar-fill" style={{ width: "82%" }} /></div>
            </div>
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle>Routing</CardTitle>
            <CardDescription>Ownership metadata for notification jobs.</CardDescription>
          </CardHeader>
          <CardContent>
            <div className="stack">
              <div className="health-row"><span>Team</span><Badge variant="outline">platform-oncall</Badge></div>
              <div className="health-row"><span>Escalation</span><Badge variant="warning">P2</Badge></div>
              <div className="health-row"><span>Runbook</span><Badge variant="info">attached</Badge></div>
            </div>
          </CardContent>
        </Card>
      </section>
    </AppShell>
  );
}
