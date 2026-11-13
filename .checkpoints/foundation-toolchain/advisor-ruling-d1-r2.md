# Advisor ruling D1-R2 (round 2 on route A: "*", the row's wording, the panic clause, the fix pass) - notes

Seat: advisor agent, day session, run as a general agent reading main's .claude/agents/advisor.md (origin/main 2aa5e9c;
the branch changes that file, K4, so main's copy governs).
Worktree: .gate/foundation-toolchain-wt, branch claude/laughing-pasteur-fwya24, HEAD 8523d50.
Content under review: 4bccf55be7bbf54de6e114ddbc6aeadb2cab3f3e (4bccf55..8523d50 touches only .checkpoints/: checked,
`git diff --stat 4bccf55 HEAD -- . ':!.checkpoints'` is empty).
Scratch only under /home/user/Shamble-Frontlines/.gate/advisor-d1-r2/. No tracked file changed.

## Plan
1. Read the evidence first: review-r2-findings.md (table + TE/EA full texts), review-r2-verdicts.json (4 skeptic
   verdicts, exact wording), skeptic-r2-R2-D1..D4.md, the four reviewers' notes (RR/SA/TE/EA r2), session-std-share.txt.
2. Read the head's texts the drafts touch: Cargo.toml comment, docs/adr/README.md:234 row (OVERFLOW-ALL), ADR 0002
   part 4 / PM-0002-53 / Appendix A R5-RUSTLINT-REC4 (0002:1586), the report (Outcome, Plan and approval, As built,
   E-rows, OVERFLOW-* follow-ups, the decision entry :293-310), STATUS (2.2 bullet, :65-66, the 120-line cap), REPO_MAP,
   ROADMAP, STATUS_ARCHIVE overflow lines.
3. Then the candidate: fix-r2-drafts.md (A, B, C, D1-D10, E, F, G).
4. Own checks where a ruling rests on a mechanism claim: (a) re-run R2-D1's share-generics repro in scratch (opt-level
   0/1/s/z vs 2/3; workspace crate's own sum() via a dependency's unchecked copy), with and without "*"; (b) whether
   dropping "*" closes it; (c) the std-share claim; (d) column widths of draft A's lines; (e) STATUS line count.
5. Answer Q1-Q7 with exact approved texts; verdict, conditions, escalations. Severity on the one scale. Last fix pass:
   anything below High that arrives now becomes a report line or follow-up.

## Findings (one line each, as done)
- F1 read: main's advisor.md (2aa5e9c); the earlier ruling D1; review-r2-findings.md (12 items; TE/EA full texts);
  review-r2-verdicts.json (4 skeptics: R2-D1..D4 hold, Medium each; R2-D4's reviewers labelled it Low); the four skeptic
  notes; the reviewers' r2 notes (RR C1-C20; SA focus 1, SA2-1..3, durable decisions; EA C1-C14; TE blocks);
  session-std-share.txt (std supplies no unchecked shared Sum/Product at opt 0-3, s, z); fix-r2-drafts.md (A-G).
- F2 head texts read at 4bccf55: Cargo.toml:21-29 (comment :21-24, 117/115/118/54 cols); README:234 (row); ADR 0002
  :1-8 (author engine-architect; "Pass marks" table: test-engineer), part 4 :247-278, :465-471, PM-0002-53 :1319,
  Appendix A :1562 (R1-RR-OQ4), :1586 (REC4 adopted), :1617 (OQ2), :1619 (OQ4); ADR 0004 :3-5 (author rust-reviewer;
  pass marks test-engineer), part 20 :562-590, PM-0004-60 :1181; ADR 0001 :5 (author netcode-architect; pass marks
  test-engineer); the report :1-330 (Outcome :19-20, Plan and approval :39-52, As built :54-84, E1 :106, M5 :115, M7
  :117, E12 :118, Reviews :137-166, Risks :184-215, OVERFLOW-XCRATE-CI :231, follow-ups :253-273, the entry :293-310);
  STATUS :55-120 (120 lines; 2.2 bullet :95-99); STATUS_ARCHIVE :551-632, :676-678, :779-808; REPO_MAP :16, :18.
- F3 own run (scratch .gate/advisor-d1-r2/ws = git archive 4bccf55 + probe: room depends on sim and on extsum, a path
  crate outside the root with its own [workspace]; room's own #[inline(never)] sum of [200,100] (u8), sim's #[inline]
  sum called from room, extsum's own sum; values by catch_unwind; control [1,2] = 3), cargo/rustc 1.99.1, fresh target
  dir per run, `cargo test --release --offline -p room --test probe`:
  route A as shipped, CARGO_PROFILE_RELEASE_OPT_LEVEL 3 and 2: room's sum PANIC, sim's sum in room PANIC, extsum Ok(44);
  opt 1, "s", "z", 0: room's sum Ok(44), sim's sum in room Ok(44) (wrapped), extsum Ok(44). RR2-1 and K4 reproduced.
  "*" table removed, opt "s" and "z": all three PANIC. Route A + [profile.release.package.extsum] overflow-checks = true,
  "s" and "z": all three PANIC. Route A, test profile (dev): all three PANIC. Control 1+2 = 3 in every run.
  So both closing options close it (tested), the shipped opt-level 3 is checked, and dev is unaffected.
- F4 STATUS arithmetic: 120 lines at 4bccf55; draft E's two parentheticals, wrapped greedily at 120 (greedy is
  line-optimal), make the 2.2 bullet 7 lines (+2), so with only the night-shift compaction STATUS would be 121. With
  "OVERFLOW-XCRATE-CI (sooner if a crate depends on sim before 2.3)" and "OVERFLOW-PANIC-RULE (with room's first code)"
  the bullet is 6 lines (+1), and the compaction of :65-66 (117 cols; not his words: no DECISIONS.md hit) keeps 120.
- F5 comment: four lines (120/119/114/117 cols) in .gate/advisor-d1-r2/comment-final.txt, applied to a fresh archive
  of 4bccf55 (fixcheck/): `cargo build --workspace --locked --offline` exit 0, Cargo.lock byte-identical (cmp), K1b's
  release tests 1+1 passed.
- F6 owners: ADR 0002 author engine-architect, its pass marks (PM-0002-53) test-engineer (:5); ADR 0004 author
  rust-reviewer, pass marks test-engineer; ADR 0001 author netcode-architect, pass marks (PM-0001-34) test-engineer.
  D1's condition 3 named engine-architect alone for PM-0002-53: corrected here.
- Working view (before the advisor tool): Q1 keep "*" (zero reach today; both closing options tested; the ADR never
  asked for dependency checks; dropping it widens to every dependency with cost and security reach unmeasured). Q2
  approve with changes (state column: rulings D1 and D1-R2; owners incl. test-engineer; "non-member" plain; opt-level
  reopen as a trigger; PM-0002-53 home without the planted failure). Q3 (a): the panic hand-off as part of D1-R2 (tested
  fact + owners + anchor + pointer, no rule content). Q4 rule/trigger/owners/home in the row; method to follow-ups.
  Q5 D10 accurate with fixes ("directly"; the reason for the wider trigger; the reopen; "drafts"; open security
  item); (b) holds. Q6 the four lines above. Q7 fixes: D4 (five Wasm crates), D6 ("the guard" is ambiguous here), D7
  (CI-edit parenthetical), D8/F2 (in-repo path dep wording; #[inline(never)] precision; security read), D9/F3 (SA2-2
  open), E (6-line wording). R2-D4 relabelled Low -> Medium in writing.
