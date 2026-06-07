# Terminal demo

This demo shows the v0.2 operator workflow.

```bash
redtrace init "Demo Internal Assessment" --client "ACME Demo Corp" \
  --roe "Authorized internal lab assessment only."

redtrace doctor
redtrace scope add 10.10.0.0/24 --label internal-lab
redtrace asset add web01.lab.local --ip 10.10.0.20 --type web

redtrace finding new "Weak access control on admin endpoint"
redtrace finding set F-001 --severity high --asset A-001 \
  --summary "Administrative function lacked server-side authorization." \
  --impact "Low-privileged users may access privileged functionality." \
  --recommendation "Enforce authorization checks on privileged routes." \
  --confidence confirmed

echo "demo evidence" > evidence.txt
redtrace evidence add evidence.txt --finding F-001 --asset A-001 --type terminal-output
redtrace evidence verify-all
redtrace evidence chain --out chain-of-custody.md

redtrace map attack F-001 --tactic TA0003 --technique T1078
redtrace map owasp F-001 --id WSTG-v42-ATHZ-01
redtrace timeline add "Validated finding F-001 against in-scope lab asset" --ref F-001

redtrace validate --strict
redtrace report --format markdown --profile full --out report.md
redtrace report --format markdown --profile executive --out executive.md
```

Expected validation output for incomplete work:

```text
Validation Report

ERROR
  - Finding F-001 has no recommendation.

WARNING
  - Finding F-001 has no linked evidence.

Summary: 0 critical, 1 errors, 1 warnings
Result: FAILED
```

Expected evidence-chain output:

```text
Evidence Chain

EV-001  OK      evidence.txt                 finding=F-001 asset=A-001

evidence files: 1
failed verification: 0
```
