# redtrace architecture

redtrace is a local-first Rust CLI for documenting authorized red-team and
penetration-testing engagements. It intentionally avoids exploit execution,
scanning, credential collection, persistence, phishing, or bypass logic.

## Workspace

Each engagement is stored in a local `.redtrace/` directory:

```text
.redtrace/
  engagement.yaml
  scope.yaml
  assets.yaml
  findings/
    F-001.yaml
  evidence/
    EV-001/
      original.txt
      metadata.yaml
  reports/
  mappings/
  timeline.jsonl
```

The workspace is file-based rather than database-backed so reviewers can inspect
and version the artifacts directly.

## Core modules

| Module | Purpose |
|---|---|
| `engagement` | Engagement metadata and workspace initialization |
| `scope` | Scope rules, exclusions, and target checks |
| `assets` | Asset inventory and scope status |
| `findings` | Finding lifecycle, severity, confidence, evidence links |
| `evidence` | Evidence vault, SHA-256 hashing, chain-of-custody |
| `timeline` | JSONL chronological event log |
| `mappings` | MITRE ATT&CK, OWASP WSTG, and NIST CSF references |
| `validation` | Completeness, scope, and evidence integrity checks |
| `report` | Markdown and print-friendly HTML report generation |
| `export` | Client-ready zip package assembly |
| `sample` | Safe demo workspace generation |
| `tui` | Placeholder for future terminal operator console |

## Export pipeline

`redtrace export --format zip` stages a delivery package under
`.redtrace/reports/export-staging`, generates fresh report and evidence integrity
artifacts, zips the staged files, then removes the staging directory.

The zip contains:

```text
report.md
report.html
chain-of-custody.md
hashes.txt
metadata.yaml
timeline.jsonl
evidence/
```

## Evidence model

Evidence is copied into the local vault and hashed immediately:

```yaml
id: EV-001
finding_id: F-001
asset_id: A-001
evidence_type: terminal-output
original_filename: evidence.txt
stored_path: evidence/EV-001/original.txt
sha256: <hash>
operator_note: Lab-safe reproduction notes.
created_at: <timestamp>
```

Verification recomputes the current file hash and compares it to the stored
metadata hash.

## Validation stance

Strict validation should fail when the workspace contains blocking delivery
issues such as:

- evidence hash mismatch;
- out-of-scope or excluded asset;
- finding without severity;
- finding without recommendation.

Warnings cover non-blocking completeness gaps such as missing confidence,
summary, impact, evidence, or ROE detail.
