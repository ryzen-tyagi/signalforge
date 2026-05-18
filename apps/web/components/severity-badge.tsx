import { Badge } from "@/components/ui/badge";
import type { Severity } from "@/lib/api";

export function SeverityBadge({ severity }: { severity: Severity }) {
  return (
    <Badge variant={severity === "critical" ? "critical" : severity === "warning" ? "warning" : "info"}>
      {severity}
    </Badge>
  );
}
