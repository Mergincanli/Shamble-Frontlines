# Verifier: kit-v3-4 fix pass (9f0c6a2, 198c075), head 198c075, 2026-11-09

Role read from origin/main (`.claude/agents/verifier.md`, `CLAUDE.md`). Read-only; scratch under `.gate/verifier-fixpass/`.

## Plan
1. His words byte for byte: CHECKPOINTS.md §10 quote vs `mustafa-2026-11-08-disk-hygiene.md`; every A4 quote in
   advisor.md vs `facts.md` §0 (wrapped text joined).
2. Ruling's "fix now" list vs the diff, file by file; no file outside the list (CLAUDE.md, SKILL.md,
   security-auditor.md, hooks, settings, CI, gate untouched).
3. Every changed sentence has a `kit-draft.md` §8 row; looser/stricter/same right (only IE-2-SCOPE looser than draft).
4. Kit tests (1034, 127, 27); `node tools/gate.mjs origin/main --no-smoke`; repo-map check.
5. L1 line numbers at this head.
6. Added lines over 120 characters outside table rows.

## Results
Scratch: `.gate/verifier-fixpass/` (`tests.txt`, `quotes.py`, `wdiff.py`, `pid.sh` + `pid1/`, `pid2.sh` + `pid2/`).

### 1. His words byte for byte: PASS
- `quotes.py` joins wrapped quote lines per numbered item. advisor.md's four A4 quotes (:48-51 item 1, :208 item 5,
  :241-242 item 2, :260 item 3) each equal facts.md §0's text (STATUS:547-562 copy): 4/4 OK. No `>` line in the diff.
- CHECKPOINTS.md §10 "His words" (277 characters, joined) is the exact tail of his message in
  `mustafa-2026-11-08-disk-hygiene.md` (after "add a "disk hygiene" note to CHECKPOINTS.md: "). The diff does not
  touch the quote or the heading.

### 2. Fix list vs the diff: PASS (one ruled text carries a defect: H1)
- Files changed c3cf647..198c075: advisor.md, CHECKPOINTS.md, STATUS.md, the vfx report, kit-draft.md, fix-pass.md,
  `.checkpoints/kit-v3-4.md`. CLAUDE.md, SKILL.md, security-auditor.md, hooks, settings, `.github/`, gate files:
  untouched.
- `wdiff.py` (word-level diff, re-wraps ignored): every change maps to a ruled item; no other word changed.
  SA-2 a/b/c (:54, :127; tier-0 clause; tier-2 bracket, the ruling's own words), IE-1, IE-2 (patch-id; the later-merge
  test, ruling's words), IE-3 (both), IE-4, IE-5, IE-6, IE-7, SA-3, SA-4 (×3), SA-6, SA-7: all present, as ruled.
  §10: IE-8, IE-9, IE-10 present; plus the main session's "on its branch until it merges" (the spike is only on
  `origin/claude/netcode-proof-test-plan-hcky5g`; its `.cargo/config.toml` there: `target-dir =
  "../../.gate/netcode-proof-test/target"`, so the root command's `--target-dir` matches).
- F1-F4 are in STATUS ("From kit-v3-4"), as the ruling's "STATUS follow-ups"; F3's lines spot-checked: STATUS:116
  cites README.md:160 (OQ-0006-21 is :161 now), :484 cites :202 (R6-TEL-OQ2 is :203 now). True.

### 3. kit-draft.md §8 rows: every change has a row; verdicts right except row 76 (H1); quotes in rows 80, 82 (L2)
- Rows 64-82 cover all 19 advisor.md changes and the three §10 bullets; rows 56-63 add M1's §10 rows and L2's
  tier-1 Check row.
- Looser rows: 68 (IE-3), 76 (IE-2) and 77 (IE-2-SCOPE), not only 77. Row 68 is the ruling's IE-3 text and within his
  item 3 (the drafted "STATUS and ROADMAP updated" could never hold for an exact undo); acceptable, the advisor to
  confirm. Row 76 is wrong: see H1.
- Rows 64-67, 69-75, 78-79, 81-82 stricter, 72-73 and 80 same: agreed (72: no seat gains a power; he opens it).

### 4. Tests and checks: PASS
- routine-guard 1034/1034, settings 127/127, checkpoint 27/27 (main's counts).
- `node tools/gate.mjs origin/main --no-smoke` (base 626df67 = merge-base): GATE PASS, symbols +0 −0, 5a skipped.
- `repo-map-check.mjs`: OK, 114 entries cover 101 required paths.

### 5. L1 line numbers at 198c075: PASS
- STATUS:528 `README.md:184` = R2-ROOM-OQ7; STATUS:539 `README:104` = OQ-0003-15 (head's README).
- vfx :73 "README:103 at 69ea88e" (OQ-0003-15 there); :78 "README:202 at 69ea88e" (R6-TEL-OQ2 there), :69 AB3 and :66
  AB5 (same at both); :83 "README:103 at 69ea88e"; :88 "README:184" = R2-ROOM-OQ7 at head.

### 6. Long lines: PASS
- No added line over 120 characters (Python `len`, characters) outside table rows, in any of the 7 files.

## Findings
- **H1 (High): the "exact undo" check passes a revert that changes whitespace.** `.claude/agents/advisor.md:285-286`
  compares `git patch-id --stable`, which strips all whitespace before hashing. Reproduced (`pid.sh`, git 2.43.0): a
  "revert" that turns `"Bash(rm -rf:*)"` into `"Bash(rm-rf:*)"` has the same `--stable` id as the true inverse
  (50afd389efe7 both) while its tree differs; `--verbatim` tells them apart (e4ba5b5aef62 vs 1f6ed17af5c4). His words
  are "the advisor checks it's an exact undo" (REVERT-EXCEPTION (a)); a whitespace change in YAML, a settings pattern
  or a shell string is not one, so this loosens his rule (CLAUDE.md:109). Row 76 ("same as his words") and IE-2's
  "exact on content" (`infra-engineer.md:65`) do not hold. Fix: `--verbatim` in place of `--stable` on both sides
  (`git diff <merge-sha> <merge-sha>^1 | git patch-id --verbatim` equals `git diff origin/main...<headSHA> | git
  patch-id --verbatim`). `pid2.sh` shows `--verbatim` still ignores line offsets from a later STATUS merge (equal ids,
  plain diffs unequal). The two flags cannot be combined (git: "incompatible"); `--verbatim` is order-sensitive, which
  is fine here (both sides are `git diff` over the same paths, sorted the same way). Row 76's verdict becomes "same as
  his words" only after the fix.
- **L1 (Low): the tier-2 bracket lists the guard's blind spots incompletely.** `.claude/agents/advisor.md:76-77` names
  "a renamed-away path or `CLAUDE.local.md`"; `protectedRel` (`routine-guard.mjs:32-43`) does not protect `AGENTS.md`
  either, which the new tier-0 clause (:63-64) makes risk tier 2. The "read the `--no-renames` list yourself" covers
  it in practice. Fix: "a renamed-away path, `CLAUDE.local.md` or `AGENTS.md`" (rides the H1 push).
- **L2 (Low): rows 80 and 82 quote words that are not in the file.** `kit-draft.md` row 80's After is "Today: from
  `spikes/netcode-proof/`, `cargo clean` ..." (landed: "For the netcode proof test's spike (`spikes/netcode-proof/`, on
  its branch until it merges): from its folder, `cargo clean` ..."); row 82's "today each copy keeps its own" (landed:
  "The netcode spike keeps one per copy ..."). The verdicts stand; the report carries the table. Fix: copy the landed
  words into both cells.
- **L3 (Low): SA-3's trigger test omits the third "main breaks" case.** `advisor.md:282-284` accepts a red run or
  Mustafa's own words; "a merged kit change stops sessions working" (:264) qualifies only through his report, so a
  session that sees it must plan first. Strict side. Fix: name it in the report, or add "or the session's quoted
  refusal or error" (next kit pass, with F4).

Not verified: IE-8's git worktree claims and IE-9's Cargo claims (infra-engineer's runs; not re-run here); 5a smoke
(`--no-smoke`, as asked).

## Verdict: FAIL (one High, H1; everything else PASS)
The fix pass carries out the ruling exactly and adds nothing outside it; his words are intact; tests, gate and repo
map pass; L1's numbers hold. The FAIL is H1, a defect in the ruled IE-2 text itself: one word (`--verbatim`) on two
places of one line, plus row 76. Per the ruling §4, a High in the new text means a second round on what changed
(infra-engineer, IE-2's owner) or the advisor's ruling on the one-word fix.
