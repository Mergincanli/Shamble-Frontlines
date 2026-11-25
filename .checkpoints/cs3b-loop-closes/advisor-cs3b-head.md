# Advisor: review of cs3b's pushed head (2026-11-25)

Head 09b0043d81894c2b4802d38aa15fec77c9426f3a; final code 6abf3d2a3d347ac8b2d5a169cbe5ac0aabc3d9b1; base main 28254a73ab226379373e380eeee64336e58ef916.
Seat: day session, advisor agent (called by the main session). Only write: this file.

## Plan
1. Diff file list and modes vs base (risk tier; kit/gate/.github untouched?; symlinks/submodules); code-free tail 6abf3d2a..09b0043d.
2. Read report, card, plan s5, my start notes (C1..C8), DECISIONS + decisions-5.mjs --check, his three words files.
3. Reviewers' notes round 1 and 2, verifier, docs-keeper; what changed after round 2 (6380df45, 6803671d, a85b9dbb, 6abf3d2a).
4. Run the gate and the tests myself on the head (outputs to scratchpad only; no tracked file touched).
5. CI on 6abf3d2a and later heads via gh api GET.
6. Judge points 1..9; spot-check claims (counts, quotes, line numbers).
7. Verdict + findings + merge conditions.

## Findings (one line each, as found)
- F0 setup: worktree HEAD = 09b0043d, clean; merge-base = 28254a73 = origin/main. Diff: 122 files (84 outside .checkpoints), all plain 100644 add/modify; no .claude/, CLAUDE.md, .github/, tools/, legacy/, docs/adr/ file. Tail 6abf3d2a..09b0043d: docs/ + notes only (code stat empty). Risk tier 1 (crates, packages, page) + DECISIONS (decision doc) -> needs-mustafa.
- F1 sizes: code incl. generated 7,027 added (76 files); minus generated types 66, f32-expected 39, Cargo.lock 13 = 6,909 (matches report). Docs 577 added.
- F2 CI (gh api GET): gate/smoke/toolchain success on 6abf3d2a (toolchain 03:08:37-03:13:35) and on 09b0043d (03:36:31-03:41:29).
- F3 his words: decisions-5.mjs --check exit 0; the three saved messages equal the transcript's user messages byte for byte (4c185bad 2601 B, 172046ad 510 B, fad04f54 1043 B). No other message of his in the transcript after 21:48:49 (only meta, task notices, /compact).
- F4 ADR quotes in RELEASE-COUNT hold: ADR 0002:319 "press and release edge counters kept as running totals (their exact fields are ADR 0001's)"; ADR 0001:226 "a press counter per button". ADRs Proposed -> changing the row is his.
- F5 my runs (scratch target dir, nothing in the worktree): gate checks 1-4, 5b-5d PASS, 5a smoke cannot start here (tools' Playwright wants chromium_headless_shell-1194, /opt has 1243; legacy/ and tools/ unchanged by the branch; CI smoke green on 09b0043d). cargo debug workspace 114 passed; release -p sim -p protocol 19, 8, 17, 5, 1; decoder run 6.5 s: per type 10 M + 10 M, 0 panics, largest Join 1024/1024, JoinAccepted 64/64, others 0, plants at inputs 17 and 0; release room, client-endpoint, sim-wasm, devtools pass; fmt and clippy -D warnings clean.
- F6 CI log of 09b0043d (toolchain job 138959677883): client 85, sim-worker 46, types 7, net 1, boundary 18, Wasm 53,813/21,994/18,464 sha256 5ebae154...1262, 16 browser tests 3.3 min; presses 10,000/10,000 0 lost 0 doubled, 1 first copy overtaken, 0 older, 0 wrong keys.
- F7 C5: Cargo.lock diff = path edges only; all six manifests only in 1ee3e918; sha256 of all six at head = dependency-check.md's. No package.json change.
- F8 C3: build plan's three old/new pairs equal the diff word for word; ADR untouched; messages.rs:5-8 updated (OQ-0001-6 to Slice 3); "queue picks neither" in report:163.
- F9 C4: chat's message 2026-11-24T23:57:49Z (three lines: size 3,723 at 2.0x, ladder moves L1 and recording to 3c, what stays) before steps 4-8; nothing of 7/8 built. Report says "about 23:56" (Low, fine).
- F10 C6: timed on clean copy 00:39-00:44 before eb0d2314 (first push with browser specs, 00:45); three code pushes before it (1ee3e918, 5c582ddd, d117fca5) touched no e2e spec. His words "before the first push, as you said" met in purpose, not letter; report/DECISIONS say "first evidence push" (Low: say it plainly).
- F11 round-2 fix 6abf3d2a: queue.ran() now uses the queue's own last-taken sequence; only call site lib.rs:269 passed next.sequence = the taken one -> identical behaviour; debug_assert only. wrongKeys check: postedHeld per post vs echo held; Worker host clears held on hidden, page fills postedHeld 0 on hidden: consistent. Within budget; no reviewer needed.
- F12 HIGH: pr-body.md "The presses mark ... by its own method ... 10,000 presses and releases ... reordered ..., 0 lost and 0 doubled" calls the release half met by the mark's method; test-engineer's ruling (review2 summary): "Medium, High if the report calls the release half met". Report:86 heading says "by its own method" too (hedged by "What is not met"). Fix the PR text and that heading.
- F13 MEDIUM: OVERFLOW-PANIC-RULE (his go for #48 item 5: accepted open, "due with the first code in the room crate") came due with 3b; report silent; STATUS pointer now files it "for the gate step (2.3)". Substance met (room/client-endpoint deny arithmetic_side_effects outside tests; design block 2 rule; my clippy clean), CI has no lint step. Fix: name it in the report and For Mustafa, keep its due point visible.
- F14 MEDIUM: RELEASE-COUNT text: "No planned system acts on a release by itself" vs charged shots (ADR 0005 AB-3 "charge curve", ADR 0001:265 "charged shots", UNITS:170-171 Embar/Torque Bow fire on release). Releases are derivable from held bits + paced press counters (test-engineer A2), so (a) still stands, but the stated reason is wrong. Cost "16 more bytes each way": per command 16 B, an Input carries up to 8 whole commands: up to +128 B per Input (MAX_INPUT_BYTES 292 -> 420).
- F15 MEDIUM: plan s5 evidence and s9 row PM-0002-5 ("3b: one 10-minute run in Chromium here, from a script of its own ..., in the report"): not in the report's Evidence, Not verified or deviations; no such script tracked. Fix: run once or list as not done + follow-up.
- F16 point 6: pacer bound 255 (press.rs MAX_BACKLOG; only > 30/s on one key) and outage 256 (one-byte counters; needs a network) cannot be reached by a person in 3b -> follow-ups right, no decision before merge. But the option he chose (plan :303, his "Yes to all seven", DECISIONS:728/741 as the thread's text) reads "It is applied one tick (33 ms) later; nothing is ever lost." Report:182 says "His answer was about taps within one tick"; archive 3B-OUTAGE-256 "in its reading no rule of his is loosened". Fix: quote the option, mark both his decisions when due. MEDIUM.
- F17 point 4 (M1): deferral is order of work inside his own decision (Slice 3 picks timing; cs7 builds both) -> mine, approved with its written conditions (archive 3B-LATE-BOUND). Security finding left open -> PR needs-mustafa; For Mustafa (report:239-247, pr-body) does not list it, nor OVERFLOW-PANIC-RULE, nor the two press limits. MEDIUM: one line under For Mustafa item 3.
- F18 point 8: README (make-zip.py) exercises W, Space, A only; S, D, both Shifts never pressed; no focus-loss step (verifier: boot-no-blur not caught anywhere; "Mustafa's Brave steps cover the hidden tab ..., not a focus loss"). LOW: add two steps. Version check present (step 1, commit in readout).
- F19 LOW: DECISIONS' first new line is the chat's note about the board add-on (labelled "not a decision"); optional move to STATUS.
- F20 spot-checks: #65 merged_at 2026-11-24T22:40:39Z head 1624f084 (3a note right); 6abf3d2a CI log: loop 1,814 ticks 60.6 s, 138 presses, key-to-tick 238.1/257.8 ms, 502 delayed, 46 twice, 1 overtaken, 47 ignored, 16 passed (3.4m): all as the report. ROADMAP 2.2 entry moved (1 copy). repo-map-check OK 193/109.
- VERDICT: REVISE 09b0043d (text only; code 6abf3d2a approved). Findings: High F12; Medium F13, F14, F15, F16, F17; Low F10, F18, F19.
DONE

# Block 2: review of the changes 09b0043d..8f835d11 (2026-11-25)
Head 8f835d114dcbf03c80cb7227c9cf189539dc6d09 (= pushed branch head). Two commits 6fade7cd, 8f835d11; 10 files, docs + notes only; code stat vs 6abf3d2a empty.
## Plan
1. Read the diff of the report, STATUS, archive, DECISIONS, pr-body, make-zip, card; check H-1, M-2..M-6, L-1..L-3 each.
2. decisions-5.mjs --check; repo map check; STATUS line count; quotes of his/the plan's words against their sources.
3. ten-minutes.mjs: read it (what it runs, where it writes), the raw line in the card; plausibility of 18,089 ticks in 603.1 s.
4. CI on 8f835d11; anything new wrong in what changed.
## Findings (block 2)
