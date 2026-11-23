# U8 last round: netcode seat, NC-R2-1's fix (pass 3 item 1) only

Plan: (1) check snapshot hash; read advisor-u8-r2.md section 3 item 1 and :126-139; u8-build.md "## Pass 3" item 3.1.
(2) interdiff r1.sh (and any other file item 1 touches) old (.gate/u8-r3-old) vs new; isolate item 1's hunks.
(3) judge: cut log kept as whole-line prefix; judge unchanged; run INVALID; LG's own end awaited; one LG fetch per block.
(4) the coordinator's npt-judge run (.gate/u8-p3/judge.log, verdicts.json). (5) new findings; verdict.

Status: started.
