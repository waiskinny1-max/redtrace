# Sample engagement

`redtrace sample create` creates a safe demo workspace that shows the redtrace
workflow without including exploit code, payloads, scanning behavior, phishing,
credential capture, persistence, or bypass logic.

## Create sample

```bash
redtrace sample create
```

This creates:

```text
demo-redtrace/
  README.md
  .redtrace/
    engagement.yaml
    scope.yaml
    assets.yaml
    findings/F-001.yaml
    evidence/EV-001/
      original.txt
      metadata.yaml
    reports/
      report-full.md
      report-full.html
    timeline.jsonl
```

## Try the workflow

```bash
cd demo-redtrace
redtrace status
redtrace validate --strict
redtrace evidence verify-all
redtrace evidence chain
redtrace report --format markdown --profile full
redtrace export --format zip
```

The sample uses:

- `ACME Demo Corp`
- `10.10.0.0/24`
- `web01.lab.local`
- `auth.lab.local`

These are lab-safe placeholders only.
