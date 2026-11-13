# security-auditor, review round 2, foundation 2.2 (PR #48), round-1 head 4d78ad6 -> head 4bccf55

Scope: only what changed since 4d78ad6 outside .checkpoints/ (`git diff 4d78ad6..4bccf55 -- . ':(exclude).checkpoints'`).
Memory: no agent memory folder for security-auditor in this container (none under .claude/, ~/.claude/), as in round 1.
Writes: this file; scratch copies under /home/user/Shamble-Frontlines/.gate/security-auditor-r2/.

## Plan
1. Confirm the head and the scope: worktree HEAD = 4bccf55 (or later, .checkpoints/ only); list the diff's files;
   confirm nothing under .github/, .claude/, any CLAUDE.md changed since 4d78ad6; Cargo.lock unchanged (focus 4).
2. Read the diff whole: Cargo.toml (route A), the four crates' doc comments, the register row OVERFLOW-ALL
   (docs/adr/README.md), the report (docs/reports/2026-11-12-foundation-2-2.md: Risks, findings table, decision entry
   OVERFLOW-ALL), STATUS, STATUS_ARCHIVE ("From foundation 2.2"), REPO_MAP, ROADMAP; 297d48a/8d622ac exact revert.
3. Focus 1, panic versus wrap: read ADR 0002 part 4 (:247-278) and R2-ROOM-REC2 (r2-netcode.md:2762-2772), and whatever
   part 4 says on panic = "unwind", per-connection isolation, decoders that never panic. Weigh: (a) does route A open a
   client-triggerable DoS path in room/server code that part 4's form avoided; (b) does part 4's isolation already
   cover it (catch_unwind around per-connection work; a panic inside the room's tick?); (c) is a silent wrap worse
   (wrong authoritative state: credits, health, ammo, tick counters; security reading); (d) the rule for when room
   gets code. Sources: Rust reference (integer overflow), Cargo profiles docs (overflow-checks, package overrides,
   "*"), std::panic::catch_unwind docs (UnwindSafe, abort cases), RFC 560 (integer overflow), clippy
   arithmetic_side_effects lint. Optional scratch test: catch_unwind around an overflowing room-style function in
   release under route A (behaviour, not only reading).
4. Focus 2, honesty: the report's Risks, the register row OVERFLOW-ALL, the decision entry OVERFLOW-ALL (options and
   its "no" branch), STATUS_ARCHIVE OVERFLOW-XCRATE-CI and OVERFLOW-PER-CRATE: is the consequence (every workspace
   crate panics on overflow in release, room/server/devtools included; dependencies keep the release default) stated
   truly and completely, with no claim beyond the evidence?
5. Focus 3: CUT-ESCAPE and SETPRIV-HARDEN as worded (report Risks + findings table + STATUS_ARCHIVE) vs my round-1 SA-1
   and SA-2, skeptic-r1-D4.md, skeptic-r1-D5.md, review-r1-verdicts.json.
6. Write findings one line each as found; the verdict; what I checked; what I did not verify.

## Findings and checks (one line each, as done)
