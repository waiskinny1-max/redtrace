#!/usr/bin/env bash
set -euo pipefail

rm -rf demo-engagement
mkdir demo-engagement
cd demo-engagement

redtrace init "Demo Internal Assessment" --client "ACME Demo Corp" --roe "Authorized internal lab assessment only."
redtrace scope add 10.10.0.0/24 --label internal-lab
redtrace asset add web01.lab.local --ip 10.10.0.20 --type web

echo "demo evidence" > evidence.txt

redtrace finding new "Weak access control on admin endpoint"
redtrace finding set F-001 --severity high --asset A-001 \
  --summary "Administrative function lacked expected server-side authorization." \
  --impact "Low-privileged users may access privileged functionality." \
  --recommendation "Enforce server-side authorization checks on privileged routes." \
  --confidence confirmed
redtrace evidence add evidence.txt --finding F-001 --asset A-001 --type terminal-output
redtrace timeline add "Validated finding F-001 against in-scope lab asset" --ref F-001
redtrace map attack F-001 --tactic TA0003 --technique T1078
redtrace map owasp F-001 --id WSTG-v42-ATHZ-01
redtrace report --format markdown --out report.md
redtrace status
