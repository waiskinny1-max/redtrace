# Terminal demo

```text
$ redtrace init "Demo Internal Assessment" --roe "Authorized lab only"
initialized redtrace workspace at .redtrace/
engagement: Demo Internal Assessment

$ redtrace scope add 10.10.0.0/24 --label internal-lab
added scope rule SCOPE-001

$ redtrace scope check 10.10.0.20
IN SCOPE
10.10.0.20 matches an authorized scope rule.

$ redtrace asset add web01.lab.local --ip 10.10.0.20 --type web
added asset A-001

$ redtrace finding new "Weak access control on admin endpoint"
created finding F-001

$ redtrace evidence verify EV-001
EV-001 OK evidence.txt
stored:  9a1f...
current: 9a1f...

$ redtrace validate
redtrace validation: OK
```
