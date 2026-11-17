# test-engineer: check of U4f's builder brief rev 3 (2026-11-17)

Brief: N/u4f-brief.md (N = .checkpoints/netcode-proof-test). Read only. Branch head at start: 42e0912.
Scripts (read-only, print only): N/u4f-r3/te_anchors.py (and te-*.py to come).
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
- B1 done: brief read whole (1,408 lines; sections 0-12, NA table).
- B2 done: plan §5 U4f :805-816, §4.10 :360-390, §4.11-4.14 :392-476, §5 preamble :507-512, §7 :909-932, §9 :958-968; carries §U4f :507-540. Counts recomputed: 2.10 rows sum to measured +102 (f30 excluded), bites +117 (102 + f30 + 14 unit proofs); neg rows sum 61; 5.1: 36 measured (6 + 24 + 3 + 3) + 12 twins + 17 bites = 65; 5.4 memory 4,801 / 1,927 / 6,426 MiB re-derived from 302.2 + 2.3403 x MiB (c-u4r-runs.mjs:177); 5.5 disk 3,596 / 3,667 re-derived; room 5,514 - 4,096 = 1,418. All add up. Every §5 U4f build item, mark, bite and the -22 load report present.
- B3 (part): night check read (34a2e55): TE-1 High; Medium TE-2..11,13,15,16,23; Low TE-12,14,17..22,24 (24 total). Walk below.
- B4 done: te_anchors.py run. Every stamp equal to 0.2's. P1-P23, P25-P36: start found once, spans and stated lines equal, entry numbers right (P1: 46, 80, 86; P3-P13 each), P14's line before equal. **P24: "Write files with the" found twice in df-ruling (:313 inside a quoted rule, and :462)**; the drafter's script matches once only because its anchor ends in a line break. -> TE3-2.
- B5 (part) rows checked at HEAD, all hold: 1 (input.rs:10-13, :61, :64-66, :68-71, :80-125, :82, :89, :98, :110, :127-151, :155), 2 (room.rs:361-369, :367), 6 (room.rs:744-801, :783-788, :791, :795), 11 (pump.rs:16, :24, :26, :372-480, :387-404, :449-457, :458-472, :500-516, :510), 19 (scenario.rs:70, :134, :215, :227, :458, :1370, :1447), 22 (c-u4r-runs.mjs:133, :138-154, :167, :172, :177, :179, :224-231), 31 (:1270, :2212, :2267), 36 (figures.rs:538, :579, :677, :689, :818-826), 37 (marks.json:3972-3982, :3983-4005, :4036-4039), 38 (SERIES.md:291, :292, :322, :378), 40 (scenario.rs:2795-2833, :2904, :2954, :2970), 44 (room.rs:689-695, :739), 45 (scenario.rs:70, :995-1006). SERIES rows of 2.7 all at the cited lines (:21, :33-34, :40-42, :61, :63, :93, :101, :128, :144-145, :149, :155-156, :158-159, :170, :245, :324, :344, :346, :385, :415, :416, :429, :445).
- B6 (part): marks.json -20/-21/-22/-47 read. judge eval.rs:1386-1502: a bite is Shown when any plain check of its matched parts fails on a valid run (:1475-1478); gaps aside (eval.rs:1085). -21's count checks and -47's 1,000-connects check are plain (no "setup"): they fail by length on every short run. -> TE3-1. Twin figure figures.rs:191-214 = 5.4's rule (profile, seed, bite, role twin, same set).
- B7 done: 5.6 B1-B7 complete for the options asked; rev 2 cited only :2127 (34a2e55 draft :124, :695), rev 3 carries :2125 (P15) and :2127 (P16) in 1 item 5, 4, 5.6, 12 items 6 and 8.

## Findings (one line each as found)
- TE3-1 Medium: D12 "each bite failing its check by name, every other check passing" cannot hold on 5.3's short runs (count checks fail by length); a short bitten run's bite verdict is "shown" by the count check alone (eval.rs:1475-1478), so only a by-name reading of the bite's own check against the clean short run's same check proves anything.
- TE3-2 Medium: P24's anchor not unique as worded (df-ruling :313 and :462).
- TE3-3 Medium: f16's "equal fate logs on every flow": L1's canonical fate log includes each datagram's size (l1/src/lib.rs:353-362, :385, :1146); twins' client datagrams differ in size by design (kinds 8, 9 header only: plan :377-379; kinds 1-6 one fewer resent command under U4F-Q13 (b); kind 11 another message) -> fate_log_sha256 differs on a correct build. Loss/delay/reorder draws are size-independent (:1125-1138, :1188), so "fates equal" (outcome, arrival, index, send time per flow entry, size excluded) is what can hold. Plan §4.10 :389-390 says "fate logs equal": route the reading.
- TE3-4 Medium: f16 (48 short runs), f18, f28 ("a short run of each suite at 5.3's lengths": 48 clients, 30 s; the fast clock 80 s) and f29 (1,000 connects, about 52 s or more) are lab tests in check.mjs's `cargo test --workspace` (debug profile, both builds: check.mjs:2052-2057), with no client count, length or time budget; the lab's existing short tests use 16 clients, 6 s + 4 s (scenario.rs:2842-2843). f28 conflates the cargo test with 5.3's release runs.
- TE3-5 Low: f8 and f32 "property tests" with no case count or generator; npt-room and npt-client have no RNG dependency (room/Cargo.toml:17-21; client/Cargo.toml:17-22) and a manifest change is a stop; the project's pattern is a hand-rolled splitmix64 (bots/src/lib.rs:114; lab/src/scenario.rs:94-95).
