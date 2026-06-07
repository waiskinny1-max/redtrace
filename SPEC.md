# redtrace Specification

## Identity

redtrace is a terminal-first red-team engagement command center for scope
control, evidence handling, findings, ATT&CK/OWASP mapping, timelines, and
report generation.

It is not an exploit tool. It is an engagement discipline tool.

## v0.1 scope

The v0.1 release implements:

- engagement workspace initialization;
- local-first `.redtrace/` storage;
- scope rules and exclusions;
- asset inventory with scope status;
- finding lifecycle;
- evidence vault with SHA-256 verification;
- append-only timeline logging;
- basic MITRE ATT&CK / OWASP WSTG / NIST CSF tagging;
- Markdown and basic HTML report generation;
- validation checks;
- test suite.

## Storage

```text
.redtrace/
  engagement.yaml
  assets.yaml
  scope.yaml
  findings/
    F-001.yaml
  evidence/
    EV-001/
      original.txt
      metadata.yaml
  timeline.jsonl
  reports/
    report.md
```

The storage intentionally remains human-readable so that reviewers can inspect
the engagement state without a database browser.

## Safe-use statement

redtrace is designed for authorized assessments, labs, CTFs, and professional
security documentation workflows. It does not perform exploitation, credential
theft, persistence, phishing, stealth, bypass, scanning, or unauthorized access.
