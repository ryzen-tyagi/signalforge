export const metrics = [
  { label: "Open incidents", value: "12", tone: "critical" },
  { label: "Live events/min", value: "428", tone: "warning" },
  { label: "Healthy services", value: "31", tone: "ok" },
  { label: "Active rules", value: "18", tone: "ok" },
];

export const events = [
  { id: "evt_01", service: "payments", severity: "critical", message: "Error budget burn exceeded", time: "16:14:22" },
  { id: "evt_02", service: "checkout-api", severity: "warning", message: "p95 latency above 900ms", time: "16:13:58" },
  { id: "evt_03", service: "webhook-worker", severity: "info", message: "Queue depth recovered", time: "16:13:41" },
];

export const incidents = [
  { id: "inc_217", title: "Payments elevated failures", service: "payments", severity: "critical", status: "open" },
  { id: "inc_216", title: "Checkout latency spike", service: "checkout-api", severity: "warning", status: "acknowledged" },
  { id: "inc_215", title: "Webhook retry backlog", service: "webhook-worker", severity: "warning", status: "resolved" },
];

export const rules = [
  { id: "rule_01", name: "Critical payment events", service: "payments", severity: "critical", enabled: true },
  { id: "rule_02", name: "Checkout latency", service: "checkout-api", severity: "warning", enabled: true },
  { id: "rule_03", name: "Worker queue growth", service: "webhook-worker", severity: "warning", enabled: false },
];

