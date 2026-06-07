# Architecture

redtrace is a local-first Rust CLI. It intentionally stores engagement data in
plain YAML and JSONL files so operators and reviewers can inspect the workspace
without a database browser.

## Workspace layout

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
  timeline.jsonl
  mappings/
  reports/
```

## Design rules

1. **Local-first**: no telemetry, no cloud dependency, no remote upload.
2. **Terminal-first**: CLI now, sober TUI later.
3. **Report-first**: every object should feed a client-ready deliverable.
4. **Evidence integrity**: every evidence object stores a SHA-256 hash and can be verified later.
5. **Scope discipline**: out-of-scope assets and excluded targets are treated as validation problems.
6. **Safe public posture**: the project documents authorized work. It does not exploit, scan, persist, bypass, phish, or collect credentials.

## Main modules

| Module | Role |
|---|---|
| `engagement` | Workspace initialization and status |
| `scope` | Scope rules, exclusions, and target checks |
| `assets` | Asset inventory and scope status |
| `findings` | Finding lifecycle and evidence references |
| `evidence` | Evidence vault, hashing, verification, and chain-of-custody |
| `timeline` | JSONL activity log |
| `mappings` | ATT&CK, OWASP WSTG, and NIST CSF tags |
| `validation` | Completeness, scope, and integrity checks |
| `report` | Markdown/HTML report generation and report profiles |
| `doctor` | Local workspace health checks |
| `tui` | Future terminal operator console |

## v0.2 quality gates

`redtrace validate --strict` is intended for serious local review and CI-style
checks. It fails when critical or error-level issues exist, including evidence
hash mismatches, out-of-scope assets, missing severity, and missing
recommendations.
