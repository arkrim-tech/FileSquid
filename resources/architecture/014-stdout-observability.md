# Architecture Decision Record - FileSquid

014-stdout-observability.md

## Title

Application logs are unbuffered stdout; Promtail is not the collector

## Status

Accepted

Accepted: 2026-09-16T22:24:57Z. Revised: 2026-09-22T04:34:37Z. Superseded in part 2026-10-15 by [ADR 033](./033-activity-journal-and-live-health-monitoring.md): stdout JSON stays; the collector and external stack are replaced by FileSquid's own activity journal (100% of activity), process registry and Monitor dashboard, with optional OTLP export. Promtail, Redpanda, Loki, Grafana, Alloy and Prometheus are removed.

## Context / Requirement Reference

AGENTS requires stdout-only logging and names Promtail/Redpanda/Loki/Prometheus/Grafana. Promtail is EOL. Desktop users MUST NOT need a metrics cluster to manage a local library. The vision listed exact cloud frameworks as undecided.

## Decision

**Every FileSquid process** writes structured operational events unbuffered to **stdout** (JSON objects, RFC 3339 UTC timestamps, no private titles/pixels/keys). Applications MUST NOT write their own log files.

**Desktop/local:** stdout is enough. The Grafana/Loki/Prometheus/Redpanda compose stack is **not** required to run FileSquid.

~~**Hosted (when deployed):** a collector (Alloy) ships logs and metrics to Loki/Prometheus/Grafana.~~ **Superseded by [ADR 033](./033-activity-journal-and-live-health-monitoring.md):** hosts record activity in FileSquid's own journal and show health in the Monitor dashboard; external sinks are optional OTLP targets. **Promtail MUST NOT** be adopted (EOL).

Redaction and opaque IDs follow this ADR: no private titles, pixels, or keys in the event stream.

## Rationale

Stdout is portable. Binding the product to Promtail would freeze a dead collector. Local-first means observability infrastructure is optional.

## Assumptions

- ~~Hosted ops staff will pick Alloy vs successor at deploy time.~~ Superseded by ADR 033: operators MAY attach an OTLP sink without changing the stdout contract.
- Metric names are defined in implementation, not here.

## Alternatives Considered

- **Promtail as specified in AGENTS:** Rejected; EOL.
- **Application log files:** Forbidden.
- **Always-on local Grafana for every user:** Rejected.

## Consequences / Implications

Easier: desktop stays simple. Harder: hosted compose must be rewritten before use. AGENTS.md and `.agents/rules/logging-enforce.md` MUST match this ADR as amended by ADR 033 (no Promtail, no collector stack).

## Related Decisions / Notes

- [ADR 001](./001-shared-rust-core-with-tauri-and-axum-hosts.md), [ADR 004](./004-axum-hosted-ciphertext-mailbox.md)
- [Promtail lifecycle](https://grafana.com/docs/enterprise-logs/latest/send-data/promtail/)
