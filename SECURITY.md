# Security Policy

## Safe-use boundary

redtrace is designed for authorized security assessments, internal labs, CTFs,
purple-team exercises, and professional documentation workflows.

redtrace does **not** perform exploitation, persistence, credential theft,
phishing, stealth, bypass, scanning, or unauthorized access.

## Reporting security issues in redtrace

Open a private security advisory or contact the maintainer directly if you find:

- evidence integrity bypasses;
- unsafe file handling;
- path traversal risks;
- report-generation injection risks;
- accidental disclosure risks.

Do not submit exploit payloads, real client evidence, real target data, or
private engagement reports in public issues.

## Data model

redtrace is local-first:

- no telemetry;
- no cloud account;
- no network calls by default;
- evidence is copied into the local `.redtrace/evidence/` vault;
- SHA-256 hashes are stored and can be verified later.
