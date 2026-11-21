# infra-engineer: U4r-F3 review (round 1 of at most 2), 2026-11-14

Scope: the uncommitted diff to spikes/netcode-proof/ops/runs/c-u4r-runs.mjs and c-u4r-r2.mjs, against the plan
(u4r-f3-plan.md) and the advisor's ten conditions (advisor-u4r-f3-plan-full.md). Seat: runner mechanics.
Scratch: .gate/u4r-f3-review/ only (logbytes-check.mjs). No edits to spikes/, the harness or tracked files.
(No Edit tool in this agent: this file is rewritten whole with Write.)

## Plan
1. Read the diff, HEAD's copies, the working copies (line numbers).
2. Judge projection start check: place (after room("before the builds"), before mkdirSync(bin)), form (cond. 4).
3. judgeCall's one guard: what sits inside it; can anything throw out; is runs.txt always written (cond. 3).
4. runs.txt block order (cond. 5), exit lines, re-checks, results/, process groups unchanged.
5. INF-F1-5 keepLead in both scripts; re-check compare.
6. RATES-1 rates in start check and per-run floor (cond. 7).
7. P0 loss under each failure.
8. Evidence: summary.log, per-staging logs, sources.log vs working-copy sha256, helper test, python reading, basics.
   Re-run selected stagings and the helper test myself.
9. Verdict and findings.

## Progress (a line per block / finding)
- Block 1 done (diff read). Working copies sha256 99dd070e (runs) / 4e007500 (r2) = sources.log's f3/ copies = tested;
  HEAD's 18c1dcf6 / 94564f8c = sources.log's head/ = /dev/shm/u4r-f3/src/head. Harness e8516622 = sources.log;
  harness mtime 23:30:00 < first run 23:31:06; scripts' mtimes 23:16:51 / 23:17:20. diff --stat 157/34 (advisor's
  range 100-130 / 20-30; under the 250 "well past" line; card :39-40 records it).
- Block 2 (start check): runs.mjs:441-447, after room("before the builds") :440, before mkdirSync(bin) :451; nothing
  is created before it; precheck: judgeGroups [] so no line and head unchanged. Throw -> main's catch -> exit 1.
  Line form = cond. 4 (prints "10240" not "10,240": fine). B1/P2/P3 one call (bitesOf only P0).
- Block 3 (guard): judgeCall :546-579. New lines outside the try: `block = []`, block.push of HEAD's line, the heading
  templates (strings): none can throw. Inside guarded(): logBytes+judgeRoom; readFileSync+JSON.parse+setParts.
  The catch only formats e?.message: no throw path found. copyFileSync :584 depends on c.ok only. setParts tolerates
  null marks/parts/runs (list(), ?.).
- Block 4 (order): runs.txt = head (+ judge projection lines after "after the builds") ... end, [SPLIT_LINE], per
  call: on-disk line, "<label>: <HEAD's line>", parts heading + lines (ok only). Exit lines :595-597 byte-identical
  to HEAD's (diff touches none). run()/process groups unchanged (lim-P0, lim-P0-bites: groups killed).
- Block 5 (INF-F1-5): both scripts' sh() keepLead -> trimEnd; only reading() passes it (runs :413, r2 :171); start and
  now readings go through the same function; start.spike is "" (tracked() refuses any change, whole repo, both
  scripts). rev-parse unaffected. recheck-P3/P0-measured/P0-bites/r2 PASS: F3 quotes " M ...", HEAD "M ...".
- Block 6 (RATES-1): du -sb re-measured: every figure in the plan's 2.5 table matches to the byte; rates >= largest
  run per profile (P0 603.7 >= 603.63 pos-4cm). Per-run floor uses r.bytes (runBytes) :532, :465. Real disk now:
  13,212 MiB avail; P0 start needs 3,615 + 4,096 = 7,711: room. RAM 15 GiB avail vs P0 bites call 4,719 MiB.
- Block 7 (header words): INF-F2-1 sentence and INF-F2-2's three wordings equal u4r-f2-review-full.md:78, :93-97
  (Q1 (b) form for the second); cond. 2 clause exact; 302.2 + 2.3403 x 3,248.3 = 7,904 MiB = 7.7 GiB. Cond. 7 comment
  names date, five folders, du -sb, pos-4cm, spread, 55.84 MB, four ratios.
- Evidence read: summary.log 25 stagings PASS; cmp-P0 17/17 (cond. 5 bites order), cmp-P3 16/16 (no-bites order),
  parts-* 4 stagings PASS, big-plan 9/9 (no cargo, no folders), cmp-precheck 11/11, fail-* and lim-* exit lines
  HEAD's word for word, helper 6/6. Notes' "head c5cd4d9c" slip already on the card (:74); the record says 18c1dcf6.
- My re-run (23:37): `harness.mjs run cmp-P0 cmp-P3 cmp-precheck parts-P3-notjson parts-P3-empty
  parts-P0-bites-notjson parts-P0-bites-empty big-plan fail-P3 recheck-P0-bites recheck-r2 helper` -> exit 0,
  "12 staging(s) run, 0 FAIL", each STAGING line PASS; ran f3 99dd070e. No collect (kept logs untouched).
- My logBytes check (.gate/u4r-f3-review/logbytes-check.mjs, imports the /dev/shm copy, sha256 = working copy):
  logBytes = find -type f sum to the byte on P3, B1, P2, P0/measured, P0/bites; on-disk lines 518.8 / 820.5 /
  778.9 / 1477.4 / 1770.9 MiB -> 1516 / 2222 / 2125 / 3760 / 4447 MiB; missing folder throws ENOENT. exit 0.
- FINDING INF-F3-1 (Low): the guard's error line can span several physical lines: V8's JSON.parse message quotes raw
  source text with its newlines (node -e: "Unexpected token 'x', \"{\n\"a\": x\n}\" is not valid JSON"). Fix at the
  next change: collapse whitespace in the catch. Not before this commit (tested copy).
- FINDING INF-F3-2 (Low, evidence): every staging's on-disk line reads "0.0 MiB" (shim logs a few KB) and the helper
  test never calls logBytes, so the record cannot tell a working logBytes from one returning 0. Closed by my check
  above; the card/report should cite it. No code change, no re-run.
- No leftover processes from my re-run (pgrep). /dev/shm/u4r-f3 67 MB.

## Verdict
APPROVE WITH FINDINGS: INF-F3-1 Low, INF-F3-2 Low. Neither triggers a push. DONE.
