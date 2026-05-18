import { AppShell } from "@/components/app-shell";
import { events, incidents } from "@/lib/demo-data";

export default function ServiceDetailPage({ params }: { params: { id: string } }) {
  const serviceEvents = events.filter((event) => event.service === params.id);
  const serviceIncidents = incidents.filter((incident) => incident.service === params.id);

  return (
    <AppShell>
      <h2 className="page-title">{params.id}</h2>
      <section className="grid metrics" style={{ marginTop: 14 }}>
        <div className="card"><span className="metric-label">Events tracked</span><span className="metric-value">{serviceEvents.length}</span></div>
        <div className="card"><span className="metric-label">Incidents</span><span className="metric-value critical">{serviceIncidents.length}</span></div>
        <div className="card"><span className="metric-label">p95 latency</span><span className="metric-value warning">928ms</span></div>
        <div className="card"><span className="metric-label">SLO</span><span className="metric-value ok">99.92%</span></div>
      </section>
    </AppShell>
  );
}

