# Terminal demo

This demo shows the public-safe redtrace workflow. It documents authorized work;
it does not scan, exploit, persist, phish, or collect credentials.

## Generate a sample workspace

```bash
redtrace sample create
cd demo-redtrace
redtrace status
redtrace validate --strict
redtrace evidence verify-all
redtrace evidence chain
redtrace export --format zip
```

## Manual engagement flow

```bash
redtrace init "Demo Internal Assessment" --client "ACME Demo Corp" \
  --roe "Authorized internal lab assessment only. No production testing."

redtrace doctor
redtrace scope add 10.10.0.0/24 --label internal-lab
redtrace scope exclude 10.10.0.50 --reason "Production database excluded by ROE"
redtrace asset add web01.lab.local --ip 10.10.0.20 --type web

redtrace finding new "Weak access control on admin endpoint"
redtrace finding set F-001 --severity high --asset A-001 \
  --summary "Administrative function lacked expected server-side authorization." \
  --impact "Low-privileged lab users may access privileged functionality." \
  --recommendation "Enforce server-side authorization checks on privileged routes." \
  --confidence confirmed

printf 'demo evidence\n' > evidence.txt
redtrace evidence add evidence.txt --finding F-001 --asset A-001 --type terminal-output
redtrace evidence verify-all
redtrace evidence chain --out chain-of-custody.md

redtrace map attack F-001 --tactic TA0003 --technique T1078
redtrace map owasp F-001 --id WSTG-v42-ATHZ-01
redtrace map csf F-001 --function protect
redtrace timeline add "Validated finding F-001 against in-scope lab asset" --ref F-001

redtrace validate --strict
redtrace report --format markdown --profile full --out report.md
redtrace report --format html --profile full --out report.html
redtrace export --format zip --out redtrace-engagement.zip
```

## Expected export package

```text
report.md
report.html
chain-of-custody.md
hashes.txt
metadata.yaml
timeline.jsonl
evidence/
```
