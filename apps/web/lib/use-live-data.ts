"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import {
  API_BASE_URL,
  type CreateRuleInput,
  type Incident,
  type IncidentStatus,
  type IngestEventInput,
  type LiveEvent,
  type Rule,
  createRule,
  formatTime,
  ingestEvent,
  listEvents,
  listIncidents,
  listRules,
  updateIncident,
  updateRule,
} from "@/lib/api";

type LoadState = "idle" | "loading" | "ready" | "error";

export function useLiveData() {
  const [events, setEvents] = useState<LiveEvent[]>([]);
  const [incidents, setIncidents] = useState<Incident[]>([]);
  const [rules, setRules] = useState<Rule[]>([]);
  const [state, setState] = useState<LoadState>("idle");
  const [streamState, setStreamState] = useState<"connecting" | "live" | "offline">("connecting");
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setState("loading");
    try {
      const [nextEvents, nextIncidents, nextRules] = await Promise.all([
        listEvents(),
        listIncidents(),
        listRules(),
      ]);
      setEvents(nextEvents);
      setIncidents(nextIncidents);
      setRules(nextRules);
      setError(null);
      setState("ready");
    } catch (err) {
      setError(err instanceof Error ? err.message : "Unable to load live data");
      setState("error");
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  useEffect(() => {
    const source = new EventSource(`${API_BASE_URL}/api/events/stream`);

    source.addEventListener("open", () => setStreamState("live"));
    source.addEventListener("error", () => setStreamState("offline"));
    source.addEventListener("event.received", (message) => {
      const event = JSON.parse((message as MessageEvent).data) as LiveEvent;
      setEvents((current) => [event, ...current.filter((item) => item.id !== event.id)].slice(0, 100));
      void listIncidents().then(setIncidents).catch(() => undefined);
    });

    return () => source.close();
  }, []);

  const submitEvent = useCallback(async (input: IngestEventInput) => {
    const event = await ingestEvent(input);
    setEvents((current) => [event, ...current.filter((item) => item.id !== event.id)].slice(0, 100));
    const nextIncidents = await listIncidents();
    setIncidents(nextIncidents);
  }, []);

  const submitRule = useCallback(async (input: CreateRuleInput) => {
    const rule = await createRule(input);
    setRules((current) => [rule, ...current.filter((item) => item.id !== rule.id)]);
  }, []);

  const toggleRule = useCallback(async (rule: Rule) => {
    const updated = await updateRule(rule.id, { enabled: !rule.enabled });
    setRules((current) => current.map((item) => (item.id === updated.id ? updated : item)));
  }, []);

  const setIncidentStatus = useCallback(async (id: string, status: IncidentStatus) => {
    const updated = await updateIncident(id, status);
    setIncidents((current) => current.map((item) => (item.id === updated.id ? updated : item)));
  }, []);

  const metrics = useMemo(() => {
    const open = incidents.filter((incident) => incident.status !== "resolved").length;
    const critical = events.filter((event) => event.severity === "critical").length;
    const services = new Set(events.map((event) => event.service)).size;
    return [
      { label: "Open incidents", value: String(open), tone: open > 0 ? "critical" : "ok" },
      { label: "Live events", value: String(events.length), tone: critical > 0 ? "warning" : "ok" },
      { label: "Observed services", value: String(services), tone: "ok" },
      { label: "Active rules", value: String(rules.filter((rule) => rule.enabled).length), tone: "ok" },
    ];
  }, [events, incidents, rules]);

  return {
    events,
    incidents,
    rules,
    metrics,
    state,
    streamState,
    error,
    refresh,
    submitEvent,
    submitRule,
    toggleRule,
    setIncidentStatus,
    formatTime,
  };
}

