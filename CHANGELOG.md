# Changelog

## v0.2.0

### Added

- `redtrace doctor` workspace health check.
- `redtrace validate --strict` for failing CI/local checks when critical or error-level engagement issues exist.
- Validation severity buckets: critical, error, and warning.
- `redtrace evidence chain` command for evidence chain-of-custody review.
- `redtrace evidence chain --out <file>` Markdown export.
- Report profiles: `executive`, `technical`, and `full`.
- Severity summary table in generated reports.
- Remediation roadmap sections in generated reports.
- Tests for strict validation, evidence chain export, report profiles, and doctor output.

### Changed

- Report output defaults to `report-full.md` / `report-full.html` when no explicit output path is provided.
- Validation output is now grouped and more suitable for CI and operator review.
- README now documents the v0.2 workflow and safer public positioning.

## v0.1.0

- Initial local-first red-team engagement workspace.
- Core CLI for scope, assets, findings, evidence, timeline, mappings, validation, and reports.
- SHA-256 evidence verification.
- Markdown and basic HTML reports.
