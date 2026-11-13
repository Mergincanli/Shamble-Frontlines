# U4r brief rev 3: test-engineer's notes (workflow draft step, 2026-11-13)

Deliverable: /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-brief.md (rev 3).
Head read: (filled in block 1)

## Plan (blocks; a line after each when done)
- B1. Read rev 2 (ed973c9:.checkpoints/npt-u4r-brief/u4r-brief-draft.md), its check, rev 2 notes. Record its
  sections, R1-R34, Q1-Q12, D1-D12 in short here.
- B2. Read the models: u4d2-brief.md (section 0, section 4, 7.2, the verbatim paragraph), u4e-brief.md (rev 3.3 form).
- B3. Read u4-build-plan.md §5 U4r (and other sections rev 2 cites), plan.md (U7 row, C-U7-2), u4s-carries.md §U4r.
- B4. Read the advisor rulings naming U4r (grep), and the ones the task names; list each applied change.
- B5. Walk R1-R34 at the head (code reads); add rows for merged work since df8975b.
- B6. D-items: fill D1, D2, D3, D5, D6, D8, D10, D11, D12 from greps; D4, D7, D9 stay with commands.
- B7. Write the brief (Write, in parts if needed), then a self-check (cites, placeholders, change list).
- B8. Return summary through SubagentHandback.

## Block log
- B1 done. Rev 2 read whole (sections 0-11; R1-R34; Q1-Q12 all open; D1-D12). Check (16 findings) and rev 2 notes
  read (heads). Head at start ea7bf27, then fa8e335 (the main session's checkpoint commit: only .checkpoints/ changed);
  `git diff --quiet cc8595d HEAD -- spikes/netcode-proof` exit 0.
- Stamps at fa8e335: u4-build-plan.md 01ede63d… (1,092 lines); plan.md 3ca4e12a… (1,518); u4s-carries.md 8b7f165e…
  (644); history.md 88bc195f… (2,011); u4r-run.sh 349b51ad…; u4d2-brief.md 146acf18…; u4e-brief.md 0fa37e8e…;
  advisor-u4df.md 842ec092…; advisor-u4j-r2.md 7efa9410…; advisor-u4d2-m9.md f31f7876…; advisor-u4d2-r2.md
  e776f0ba…; main-run.sh 89851e77…
- B2 done. Models read: u4d2-brief.md (head :1-180; sections 3-8 :560-899: the verbatim paragraph :50-52, section 0
  :54-101, 4 :648-732, 7.2 :827-876, the ruled-questions table :882-898); u4e-brief.md headers (0 :120, 7.1 :840,
  7.2 :890).
