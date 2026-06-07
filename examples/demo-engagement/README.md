# Demo engagement

This folder documents the intended safe demo flow for redtrace.

Generate a working sample with:

```bash
redtrace sample create --path demo-redtrace
cd demo-redtrace
redtrace validate --strict
redtrace export --format zip
```

The generated workspace uses lab-safe placeholder data only.
