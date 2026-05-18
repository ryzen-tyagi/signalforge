export type Severity = "info" | "warning" | "critical";
export type IncidentStatus = "open" | "acknowledged" | "resolved";

export type LiveEvent = {
  id: string;
  tenant_id: string;
  source: string;
  service: string;
  severity: Severity;
  message: string;
  attributes: Record<string, unknown>;
  received_at: string;
};

export type Incident = {
  id: string;
  title: string;
  service: string;
  severity: Severity;
  status: IncidentStatus;
  created_at: string;
};

export type Rule = {
  id: string;
  name: string;
  service: string | null;
  min_severity: Severity;
  enabled: boolean;
};

export type CreateRuleInput = {
  name: string;
  service?: string | null;
  min_severity: Severity;
};

export type IngestEventInput = {
  source: string;
  service: string;
  severity: Severity;
  message: string;
  attributes?: Record<string, unknown>;
};

export const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://127.0.0.1:8080";

async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`, {
    ...init,
    headers: {
      "Content-Type": "application/json",
      ...init?.headers,
    },
  });

  if (!response.ok) {
    throw new Error(`${init?.method ?? "GET"} ${path} failed: ${response.status}`);
  }

  return response.json() as Promise<T>;
}

export function listEvents() {
  return apiFetch<LiveEvent[]>("/api/events/recent");
}

export function ingestEvent(input: IngestEventInput) {
  return apiFetch<LiveEvent>("/api/ingest/events", {
    method: "POST",
    body: JSON.stringify({ ...input, attributes: input.attributes ?? {} }),
  });
}

export function listIncidents() {
  return apiFetch<Incident[]>("/api/incidents");
}

export function updateIncident(id: string, status: IncidentStatus) {
  return apiFetch<Incident>(`/api/incidents/${id}`, {
    method: "PATCH",
    body: JSON.stringify({ status }),
  });
}

export function listRules() {
  return apiFetch<Rule[]>("/api/rules");
}

export function createRule(input: CreateRuleInput) {
  return apiFetch<Rule>("/api/rules", {
    method: "POST",
    body: JSON.stringify(input),
  });
}

export function updateRule(id: string, input: Partial<CreateRuleInput> & { enabled?: boolean }) {
  return apiFetch<Rule>(`/api/rules/${id}`, {
    method: "PATCH",
    body: JSON.stringify(input),
  });
}

export function formatTime(value: string) {
  return new Intl.DateTimeFormat("en", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  }).format(new Date(value));
}

