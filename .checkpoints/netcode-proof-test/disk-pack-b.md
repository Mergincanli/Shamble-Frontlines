# DISK-PACK (b): the seven closed sets, packed one at a time (the main session, 2026-11-25 from 11:2x UTC)

His word: history.md:7942 (his message :7938-7957, 11:07 UTC): "(b), all seven, by the kept-logs rule as it stands
(CHECKPOINTS.md section 11)", one at a time, never during a run set or a build, each after its results are committed
and its ruling closed, its list committed first, the folder removed only when every hash matches; "This is my word for
fc411116c1f1 and fd4612050cd6 too"; a folder that does not match its list stays, nothing more is packed, he is told.
u4f-47-b57221ed139d stays unpacked until the -47 re-judge after Netcode Builder 2's merge.

Procedure per set (c62h-bound-brief.md:494-499; .claude/CHECKPOINTS.md §11): pgrep finds no cargo, rustc, npt-lab,
npt-judge, check.mjs or runner; free at least the folder plus 1,024 MiB (the script checks it too); `kept-logs.mjs
list`; `git add` of that one list path only (the three held DEP-HOLD files stay unstaged); `git commit -F` naming the
ruling; pushed by the notes save (never by hand); `kept-logs.mjs pack`; its lines here; exit 1 or a refusal: stop,
tell him, pack nothing more. Checked before the first: kept-logs.mjs checks only the results folder's own status
(:130-131), so the held files under spikes/netcode-proof/ do not block it; NB2's plan check names the four U4r sets only
as -57's kept sets, with -57 not run in that merge (advisor-nb2-merge.md:310-311), so no NB2 step reads them; free
13,332 MiB at 11:2x UTC.

## The sets, their results and their closed rulings
1. u4r-P2-8ee453c22882 (741 MiB; C62H-BOUND's P2 set): results/u4r/8ee453c22882/P2/ (daec788). Ruling:
   advisor-c62h-bound-p3.md:35 (RULING (b) "P2 counts, P3 counts"; advisor-c62h-bound-p3-full.json condition P1);
   C62H-BOUND closed: its report APPROVE WITH CONDITIONS on 3626a37 (advisor-c62h-bound-report.md:41), sent to him
   2026-11-17 03:14 UTC and answered by his RECS-1043 (history.md:2204-2227, :2488-2492).
2. u4r-P2-fc411116c1f1 (623; BRAWL-1's P2 re-run set): results/u4r/fc411116c1f1/P2/. Ruling:
   advisor-brawl1-sets-p0-full.md:7 ("BRAWL-1's re-run sets are complete: P3, P2 and P0 all count"); BRAWL-1 closed at
   ec90bc3 (advisor-brawl1-close-full.md:7); his word for this folder: history.md:7942.
3. u4f-20-350677f0aef2 (434; -20 at 350677f): results/u4f/350677f0aef2/pm-0001-20/ (1840e68). Ruling:
   advisor-u4f-20.md:114 (R20-Q1: the option-B FAIL is the stand-in's defect; -20 re-runs on the fix) and U4f-F1's
   close (advisor-u4f-f1-close.md:165). The ruling's "The 350677f folder stays committed as the record" (:130) is the
   results folder, which stays; only the raw logs are packed, and the archive keeps every one.
4. u4f-20-96de6acf8223 (434; -20's L1 re-run on U4f-F1's merge): results/u4f/96de6acf8223/pm-0001-20/ (46ca143).
   Ruling: advisor-u4f-f1-close.md:165 ("APPROVE WITH FINDINGS: -20's L1 verdict counts, and U4f-F1 closes").
5. u4f-22-7192607d99a1 (366; -22): results/u4f/7192607d99a1/pm-0001-22/ (2dae645). Ruling: advisor-u4f-20.md:146
   ("R20-Q3: -22's set stands").
6. u4r-P3-d66e23cd78f2 (298; C62H-BOUND's P3 set): results/u4r/d66e23cd78f2/P3/ (8ca774a). Ruling: as set 1.
7. u4r-P3-fd4612050cd6 (291; BRAWL-1's P3 re-run set): results/u4r/fd4612050cd6/P3/. Ruling:
   advisor-brawl1-sets-p3-full.md:7 ("Verdict: P3 STANDS AND COUNTS"); BRAWL-1 closed at ec90bc3; his word for this
   folder: history.md:7942.

## Done (the script's own lines, per set)
1. u4r-P2-8ee453c22882: list 18 files, 776,378,964 bytes, committed cc8180a, pushed with c843b66. pack 11:22 UTC:
   "free before: 13330 MiB"; "archive: .gate/netcode-proof-test/kept-logs/u4r-P2-8ee453c22882.tar.gz, 219643715 bytes
   from 776378964; read back: 18 of 18 files equal the list (path, bytes, sha256)"; "removed:
   .gate/netcode-proof-test/runs/u4r-P2-8ee453c22882"; "free after: 13862 MiB".
2. u4r-P2-fc411116c1f1: list 11 files, 652,405,598 bytes, committed 0472386, pushed with 4a81091. pack 11:24 UTC:
   "free before: 13861 MiB"; "archive: .gate/netcode-proof-test/kept-logs/u4r-P2-fc411116c1f1.tar.gz, 178774134 bytes
   from 652405598; read back: 11 of 11 files equal the list (path, bytes, sha256)"; "removed:
   .gate/netcode-proof-test/runs/u4r-P2-fc411116c1f1"; "free after: 14313 MiB".
3. u4f-20-350677f0aef2: list 24 files, 454,384,864 bytes, committed a98de3e, pushed with cebafd0. pack 11:25 UTC:
   "free before: 14313 MiB"; "archive: .gate/netcode-proof-test/kept-logs/u4f-20-350677f0aef2.tar.gz, 147964354 bytes
   from 454384864; read back: 24 of 24 files equal the list (path, bytes, sha256)"; "removed:
   .gate/netcode-proof-test/runs/u4f-20-350677f0aef2"; "free after: 14605 MiB".
4. u4f-20-96de6acf8223: list 24 files, 454,384,548 bytes, committed a572921, pushed with 2a93431. pack 11:26 UTC:
   "free before: 14605 MiB"; "archive: .gate/netcode-proof-test/kept-logs/u4f-20-96de6acf8223.tar.gz, 147952675 bytes
   from 454384548; read back: 24 of 24 files equal the list (path, bytes, sha256)"; "removed:
   .gate/netcode-proof-test/runs/u4f-20-96de6acf8223"; "free after: 14897 MiB".
5. u4f-22-7192607d99a1: list 16 files, 383,547,215 bytes, committed b1d445e, pushed with 0dac06c. pack 11:27 UTC:
   "free before: 14897 MiB"; "archive: .gate/netcode-proof-test/kept-logs/u4f-22-7192607d99a1.tar.gz, 122108900 bytes
   from 383547215; read back: 16 of 16 files equal the list (path, bytes, sha256)"; "removed:
   .gate/netcode-proof-test/runs/u4f-22-7192607d99a1"; "free after: 15146 MiB".
6. u4r-P3-d66e23cd78f2: list 11 files, 311,451,993 bytes, committed d26f79a, pushed with 78f7e77. pack 11:28 UTC:
   "free before: 15146 MiB"; "archive: .gate/netcode-proof-test/kept-logs/u4r-P3-d66e23cd78f2.tar.gz, 97800985 bytes
   from 311451993; read back: 11 of 11 files equal the list (path, bytes, sha256)"; "removed:
   .gate/netcode-proof-test/runs/u4r-P3-d66e23cd78f2"; "free after: 15350 MiB".
7. u4r-P3-fd4612050cd6: list 11 files, 304,793,576 bytes, committed d00d15a.
