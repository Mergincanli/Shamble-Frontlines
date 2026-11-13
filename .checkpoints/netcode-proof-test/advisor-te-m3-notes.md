# Advisor: TE-M3 plan review (te-m3-plan.md rev 1), notes

## Plan
1. Read the plan, the runner, main-run.sh (done).
2. Read the logs (.gate/te-m3/: loop.log, one-h6.log, one-h6-1-4.log, build logs).
3. Read the rulings: advisor-u4d2-r1.md TE-M3 block (:287-310), advisor-u4j-r2.md B2.3.
4. Read the code at HEAD: delay.rs (pump, shutdown block, life/polls counting, current()), delay_tests.rs (the test,
   `until`, NeverShut), delay_clock.rs.
5. Check U3c's record (u3c-fix2-c2b.md:74; b2e879b's delay_tests.rs around :2002).
6. Check the cause step by step against the code; look for alternatives (e.g. pump double-call, a defect hidden).
7. Judge the change (smallest, hides nothing?), the proof P1-P6, the bound.
8. Rule on TE-Q1..TE-Q4; write advisor-te-m3.md.

## Blocks done
- Block 1 (plan, runner, main-run.sh read): done.
