# test-engineer: check of U4f's builder brief rev 3 (2026-11-17)

Brief: N/u4f-brief.md (N = .checkpoints/netcode-proof-test). Read only. Branch head at start: 42e0912.
Scripts (read-only, print only): N/u4f-r3/te-*.py
(No Edit tool in this agent: this file is rewritten whole with Write at each save.)

## Plan (blocks; a line after each)
- B1 Read the brief whole (sections 0-12, NA table).
- B2 Read plan §5 U4f (u4-build-plan.md:805-816) and every plan section the brief cites for tests, bites, runs; check (1) counts add up.
- B3 My night findings TE-1..TE-24 (git show 34a2e55:...test-engineer-u4f-check.md) vs rev 3 (2).
- B4 Placeholders: anchors found once, first/last words (script) (3).
- B5 Recheck rows vs code at HEAD (>=10; C62H-BOUND judge figures, PM-0001-17 checks, l2 delay test) (4).
- B6 Judged runs -47, -20, -21 parts/twins, -22: precondition, bar, bites; JDG-ONLY-1 interim rule, ADV-CBH-1 lesson (5).
- B7 13:35 rule: marks.json/SERIES.md/judge changes listed, bars routed, reviewer read named (6).
- B8 Disk/memory vs KEPT-LOGS §11 and 5,514 MiB; D-items' commands runnable (7).
- B9 Missing/untestable/contradiction/scope creep (8). Findings with severity and fix.

## Progress
- B1 done: brief read whole (1,408 lines; sections 0-12, NA table). Next B2.
- B2 (part): plan §5 U4f :805-816, §4.10 :360-390, §4.11-4.14 :392-476, §5 preamble :507-512, §7 :909-932, §9 :958-968 read. Carries §U4f :507-540 read (46 :508-511; 80 :512-536; 86 :537-539; U4g header :541).
- B3 (part): night check read (34a2e55): TE-1 High; Medium TE-2..11,13,15,16,23; Low TE-12,14,17..22,24 (24 total).

## Findings (one line each as found)
