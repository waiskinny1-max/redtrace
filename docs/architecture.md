# Architecture

redtrace is intentionally local-first and file-backed.

## Storage model

All engagement state lives under `.redtrace/`:

```text
.redtrace/
  engagement.yaml
  assets.yaml
  scope.yaml
  findings/
  evidence/
  reports/
  timeline.jsonl
```

The design favors transparent artifacts over an opaque database. A reviewer can
open the workspace and inspect the engagement data directly.

## Object model

- Engagement: name, client, status, rules of engagement.
- Scope: CIDR/IP/domain/wildcard-domain rules and explicit exclusions.
- Asset: hostname, IP, type, tags, notes, and scope status.
- Finding: severity, confidence, impact, recommendation, evidence, mappings.
- Evidence: copied file, original filename, stored path, SHA-256 hash.
- Timeline: JSONL event stream for chronological reconstruction.

## Integrity model

Evidence is copied into the vault at add-time and hashed with SHA-256. Later,
`redtrace evidence verify` and `redtrace validate` recompute the hash and flag
mismatches.

## Safety model

redtrace is deliberately non-executing. It records authorized assessment output;
it does not scan, exploit, persist, capture credentials, or bypass controls.
