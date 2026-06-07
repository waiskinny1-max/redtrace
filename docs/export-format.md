# redtrace export format

`redtrace export --format zip` creates a client-ready package from the current
local `.redtrace/` workspace.

The export is designed for handoff and archiving. It does not add exploitation,
scanning, credential access, persistence, or any offensive execution capability.

## Command

```bash
redtrace export --format zip --out redtrace-engagement.zip
```

If `--out` is omitted, redtrace writes the package to:

```text
.redtrace/reports/redtrace-engagement.zip
```

## Package contents

```text
report.md
report.html
chain-of-custody.md
hashes.txt
metadata.yaml
timeline.jsonl
evidence/
```

## File purposes

| File | Purpose |
|---|---|
| `report.md` | Full Markdown engagement report |
| `report.html` | Full print-friendly HTML engagement report |
| `chain-of-custody.md` | Evidence index with stored hashes and verification status |
| `hashes.txt` | Machine-readable-ish evidence hash manifest |
| `metadata.yaml` | Export timestamp, engagement name, object counts, validation status |
| `timeline.jsonl` | Chronological engagement timeline |
| `evidence/` | Copied evidence vault, including metadata files |

## Metadata example

```yaml
tool: redtrace
export_format: zip
exported_at: 2026-06-07T12:00:00Z
engagement: Demo Internal Assessment
client: ACME Demo Corp
findings: 3
evidence_items: 5
assets: 4
timeline_events: 9
validation_status: passed
```

## Validation status

The export metadata includes an inferred validation status:

| Status | Meaning |
|---|---|
| `passed` | No blocking issues or warnings were detected by the export precheck |
| `warnings` | Non-blocking completeness issues exist |
| `failed` | Blocking issues exist, such as missing severity, missing recommendation, out-of-scope assets, or hash mismatch |

This is not a replacement for:

```bash
redtrace validate --strict
```

Run strict validation before delivering any export package.
