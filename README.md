# redtrace

Terminal-first red-team engagement tracker for scope, evidence, findings,
timelines, ATT&CK/OWASP mapping, and report generation.

redtrace is not an exploit framework. It is the local operator workspace that
keeps an authorized engagement structured, defensible, and report-ready.

## Why

Red-team work is not only execution. Professional work requires:

- scope discipline;
- rules of engagement;
- asset inventory;
- clean finding lifecycle;
- evidence integrity;
- timeline reconstruction;
- adversary-behavior mapping;
- clear remediation guidance;
- final reporting.

redtrace gives those pieces a terminal-native workflow.

## Features

- Local-first `.redtrace/` workspace
- Scope rules and explicit exclusions
- Asset inventory with scope status warnings
- Finding lifecycle with severity, confidence, evidence, and mappings
- Evidence vault with SHA-256 verification
- Append-only timeline log
- MITRE ATT&CK, OWASP WSTG, and NIST CSF tags
- Markdown and basic HTML report generation
- Validation checks for incomplete findings and evidence tampering
- Test suite and CI workflow

## Safe use

redtrace is designed for authorized security assessments, internal labs, CTFs,
purple-team exercises, and professional documentation workflows.

It does **not** perform exploitation, persistence, credential theft, phishing,
stealth, bypass, scanning, or unauthorized access.

## Installation

From source:

```bash
git clone https://github.com/waiskinny1-max/redtrace.git
cd redtrace
cargo build --release
```

Run locally:

```bash
cargo run -- init "Demo Internal Assessment"
```

After release, install with:

```bash
cargo install redtrace
```

## Quickstart

```bash
redtrace init "Demo Internal Assessment" --client "ACME Demo Corp" \
  --roe "Authorized internal lab assessment only. No production testing."

redtrace scope add 10.10.0.0/24 --label internal-lab
redtrace scope exclude 10.10.0.50 --reason "Production database excluded by ROE"
redtrace scope check 10.10.0.20

redtrace asset add web01.lab.local --ip 10.10.0.20 --type web
redtrace finding new "Weak access control on admin endpoint"
redtrace finding set F-001 --severity high --asset A-001 \
  --summary "Administrative function lacked expected server-side authorization." \
  --impact "Low-privileged users may access privileged functionality." \
  --recommendation "Enforce server-side authorization on every privileged route." \
  --confidence confirmed

echo "demo evidence" > evidence.txt
redtrace evidence add evidence.txt --finding F-001 --asset A-001 --type terminal-output \
  --note "Lab-safe reproduction notes."

redtrace map attack F-001 --tactic TA0003 --technique T1078
redtrace map owasp F-001 --id WSTG-v42-ATHZ-01
redtrace map csf F-001 --function protect
redtrace timeline add "Validated finding F-001 against in-scope lab asset" --ref F-001

redtrace validate
redtrace report --format markdown --out report.md
```

## Command map

```text
redtrace
  init
  status
  validate
  scope add|exclude|list|check
  asset add|list|show|tag|note
  finding new|list|show|set|note|close
  evidence add|list|show|verify|verify-all
  timeline add|list|export
  map attack|owasp|csf|list
  report
  tui
```

## Example validation output

```text
Validation warnings:
- Finding F-002 has no linked evidence.
- Finding F-003 has no recommendation.
- Evidence EV-004 hash mismatch.
```

## Example evidence verification

```text
EV-001 OK evidence.txt
stored:  9a1f...
current: 9a1f...
```

## Roadmap

| Version | Focus |
|---|---|
| v0.1 | Core CLI, local workspace, evidence hashing, Markdown report |
| v0.2 | Ratatui operator console |
| v0.3 | richer mapping engine and HTML report templates |
| v0.4 | package export and printable report themes |
| v0.5 | optional Git-backed audit trail |
| v1.0 | polished full operator console |

## Project stance

Good public cyber tooling should demonstrate operator discipline without shipping
harmful capability. redtrace is built around evidence, scope, reporting, and
recognized security vocabularies rather than exploitation.
