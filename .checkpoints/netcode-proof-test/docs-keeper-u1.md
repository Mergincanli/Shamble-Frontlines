# docs-keeper: netcode proof test, U1 (partway) · REPO_MAP + STATUS
Scope: docs/REPO_MAP.md and docs/STATUS.md only. No ROADMAP, no docs/adr/, no docs/SPIKE_PLAN.md (PR #41). Never commit.

## Plan
1. Read sources: checkpoint, plan §0.4 / §4.1 / §12.1-12.2, spike README + tree, docs/design/netcode-proof-messages.md.
2. REPO_MAP: add spikes/ (SPIKE-SCOPE (a), no outside references; check.mjs greps), spikes/netcode-proof/ (built so far +
   not built yet per §4.1), docs/design/netcode-proof-messages.md. Run repo-map-check, fix.
3. STATUS: PR #38 lines (~3, 26, 97, 332, 744) -> merged (af8393e); lines 44-45 Mac line -> Mustafa 2026-11-06;
   add netcode proof test in flight (rev 3.2, C1-C12, five answers, PR #40, PR #41, U1 commits, next). Check line-number claims.
4. Report.

## Notes (as I go)
- [done] Sources read. Facts: PR #38 merged_at 2026-11-05T21:25:34Z (GitHub REST), merge af8393e (commit dated
  22:25:34 +0100), head 8b7cfb3, msg "On Mustafa's go ("merge #38", 2026-11-05) after the advisor's APPROVE of 8b7cfb3".
  origin/main = af8393e (ls-remote). PR #40 open, draft, head e6bfcb9. PR #41 open, not draft, needs-mustafa, head 1b4094a.
- [done] repo-map-check baseline: missing docs/design/ and spikes/ (exit 1). check.mjs refs step skips docs/, .md,
  .checkpoints/, spikes/ (check.mjs:98-111) so REPO_MAP/STATUS may name spikes/.
- Spike tree now: .cargo/config.toml (target-dir ../../.gate/netcode-proof-test/target), .gitignore, Cargo.toml/lock,
  deny.toml, rust-toolchain.toml (1.99.1, clippy, rustfmt, wasm32), rustfmt.toml, README.md, check.mjs + check.test.mjs,
  crates/ x15, marks/README.md (no marks.json yet), ops/operator/redact-check.mjs + .test.mjs. Not yet: web/, fuzz/,
  marks/marks.json, ops/{cloud-init,firewall,runs,checklists}, ops/operator/{settings.json,rules.md,PROMPT-Rn.md}, results/.
- [done] Verified: PR #41 CI gate+smoke success x2 on 1b4094a (check-runs); 8368869 gate+smoke success; advisor's
  APPROVE of 1b4094a is in-session (advisor-pr41.md:77), no PR comment or review on #41. U0 = f6886bf (checkpoint
  commit), 760fa89, d984535, 41e5565, 8368869 exist. No kit diff 0c481e1..af8393e (git diff --quiet), so A3's [verified]
  line numbers hold on main; spot-checked routine-guard.mjs:1373/1396 (ACTIONS_READ), checkpoint.mjs:30 (ESC), old
  12d7961 routine-guard.mjs:594 (GITHUB_READ), 12d7961 checkpoint.mjs:30 (%3D). Branch vs main: no .claude/CLAUDE.md/.github/tools diff.
- STATUS stale sites found: :3 tail, :25-30 Phase In flight, :31-39 queue (After A3; freeze), :40-49 Next (v2) incl.
  :44-45 Mac, :97 kit-v3-3 row, :332 follow-ups, :399-400 + :429-430 freeze headers, :744-759 kit-v3-3 risk entry
  ("once merged", "Until A3 merges", "main's checkpoint.mjs:30"), MACHINES-TIMING :318-322 (overtaken in part).
- [done] REPO_MAP block: Last checked 2026-11-06; top-level `spikes/`; docs/design/ + catalogue rows; new "spikes/"
  section (7 rows) + "Not built yet" line. Leaf clippy.toml clock ban checked in all 7. repo-map-check: "repo map OK:
  123 entries cover 102 required paths", exit 0.
- [done] STATUS edits 1-5: line 3 (date, kit-v3-3 merged, in flight = npt + #41); Phase (kit-v3-3 into Done; In flight
  = npt; queue "A3 is merged. Next: B.1"; freeze lifted by UNDER-WAY (a), U0 f6886bf); Next (v2) rewritten with his
  no-Mac words, plan §1.5/§1.6/§8.4-8.5/§9/§0.4; In flight: new npt row; kit-v3-3 row -> merged, done, none.
- [done] STATUS edits 6-10: Decisions pending entry (5 answers + no-Mac words + open #41) and MACHINES-TIMING pointer;
  follow-ups :332 -> done in kit-v3-3; freeze headers (FU-P4 block, A4 queue) -> lifted 2026-11-06; kit-v3-3 risk entry
  (merged, "closed", line numbers hold on af8393e, 12d7961's for old lines); new npt risk entry (plan §13 items 1-4, 11);
  Health: 2 rows (check.mjs [assumed], CI on 8368869 [verified]). Reflowed Phase lines to <= 120. Map check OK again.
- Noted for the caller: session's uncommitted check.mjs edits add the marks drift check -> REPO_MAP check.mjs line and
  "Not built yet" (marks.json) need updating when that lands; plan §13 risk 4 overtaken (plan not in my scope).
- DONE.
- (old plan line) STATUS block: remaining (edits 6-10: line 3, Phase done+in flight, queue/freeze, Next (v2) Mac, In flight rows, Decisions
  pending entry + MACHINES-TIMING pointer, :332, freeze headers, kit-v3-3 risk entry, Health rows).
