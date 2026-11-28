# Advisor: U8-F1's brief check (u8-f1-brief.md), 2026-11-26 14:50 UTC window

Read: his RESULTS-PUSH (b) and S14-PUSH (a), word for word at history.md:8068-8106; u8-f1-brief.md (90 lines); my
advisor-u8-head.md; r1.sh's s14 block (ops/runs/r1.sh:715-740) for what it writes into the s14 folder.

## Verdict: APPROVE WITH CONDITIONS (C1-C7, each to be applied word for word)
The brief follows the head read and both of his answers:
- Base: a fresh worktree from the branch head. U8's builder's own worktree holds the 43 files untracked, so its
  fast-forward is refused; ffcheck found 44 ADDED paths present.
- Size: 6,230 less 6,189 is 41.
- Item 1c's list: s14/ whole and s14-redact.txt, as his S14-PUSH (a) asks.
- Item 1e's always-built part meets his first condition.
- Item 2: the right command; the deny at settings.json:55 is why `config --get` cannot be used.
Four points need a condition: how the 41 are counted, where the pin test sits in the carry order, whether item 2's
proof is a committed test, and whether the policy strings fit "only numbers". The round's scope also needs widening.

## The rulings asked
- **The 41-line budget:** right, if counted net (C1). U8's 6,189 is its files' total lines, and the M5 fix counted net
  (+19, with r1.sh's changed line as 0). A gross added column would carry items that do not grow U8.
- **Item 1e's pin test against item 2's line: the pin test comes first, and is never carried (C2).** His second
  condition says a later change that would put anything naming the server into the folder "stops and comes to me
  first". The R1.md sentence states that rule; only the pin test makes such a change stop. So the test is part of his
  condition, like item 1's other parts, not a Low.
- **Item 2's four-way proof may be a scratch run (C3).** My head read asked the builder to "prove it on a throwaway
  repository". A committed test would rerun the proof only on this container's git, while the line runs on his
  laptop's Git for Windows. So the evidence that counts is the scratch proof recorded once here, plus one read on his
  laptop at Stop B. The PROMPT-R1.md line therefore always ships, and only item 2's assert in test 5 can be carried.
- **Item 1's prints: the two policy strings are not inside "Only numbers leave the server" as written (C4).** A
  policy is a name ("SCHED_OTHER 0"). Printed as Linux's policy number and its priority, it becomes numbers and keeps
  everything the slack rule (item 3) needs. presence.txt's existing verdict lines and bringup/facts.txt are summaries
  written on RS that name no server; they stay under "the push stays summaries".
- **Item 1c's list:** right. s14/ whole is his S14-PUSH (a), and s14-redact.txt sits beside it in the pushed set, so
  the folder stays as npt-s14 and r1.sh write it.
- **Item 1e against his two conditions.**
  - The first, "the redaction check runs on the s14 folder itself before the push, and its result is in the run's
    record", is met, once the command is the hook's own (C5).
  - The second is met by the R1.md sentence and the pin test together. Both must cover every writer into the folder.
    r1.sh's s14 block adds summary.txt, INVALID-s14.txt, rerun/, rerun-R2.txt and browser-stamps.txt beside npt-s14's
    files (r1.sh:720-739), which the brief's list omits (C6).
- **Does item 1 fit 41 lines?** By my count, about 40 with the pin test:
  - 1a about 4, 1b about 6, 1c about 2, 1d about 10, 1e's text about 6, the pin test about 12;
  - item 2's line about 1-2, item 3's clause about 1.
  - So it fits only if the fixtures are kept small (extend the existing P1 fixture rather than adding a new one). The
    stop in C2 covers the case where it does not.
- **The round:** security-auditor at xhigh, one read, but it must also cover the code that decides what leaves the
  server (C7).

## Conditions (paste into u8-f1-brief.md as written)
**C1, section 0.** "The 41 are net: for the files U8-F1 touches, the added column less the removed column of `git
diff --numstat <base>`, the way U8's 6,189 and the M5 fix's +19 were counted."

**C2, section 0, the carry order.**
- "Item 1e's pin test is part of his S14-PUSH condition and is never carried. Before an item would pass 41, carry the
  rest, last first: item 3, then item 2's test 5 assert. Item 2's PROMPT-R1.md line is never carried. If item 1,
  with the pin test, would pass 41 alone, stop before it does and report what grew, why, and the cheaper option (his
  stop)."
- This replaces section 0's carry sentences and section 1's line "The pin test, carried before item 2 if the lines run
  out".

**C3, section 1 item 2.** "The four-way proof is a scratch run: a throwaway repository under your worktree's .gate/
(never this repository), with each of (i) to (iv) and the printed path recorded in u8-f1-build.md. It is not counted
in the 41 and not committed. The committed part is the PROMPT-R1.md line, plus R1.md's Stop B start-check line
extended with: 'and, in PowerShell, `git -C <results> rev-parse --git-path hooks` prints the clone's own hooks
folder, prompting, not denied' (the laptop's git is not this container's)."

**C4, section 1 item 1a and 1b.**
- "Every value printed is an integer.
- The two policy reads print as Linux's policy number and the priority: SCHED_OTHER 0, SCHED_FIFO 1, SCHED_RR 2,
  SCHED_BATCH 3, SCHED_IDLE 5, SCHED_DEADLINE 6, as `P1 <name> <policy number> <priority>`; an unread or unknown
  policy prints `-1 -1`.
- A field whose value is not an integer is not printed; one line `P1 non_integer_fields <count>` (in 1b,
  `ws_split non_integer_fields <count>`) gives how many.
- No new line holds the word INVALID (the probe block greps presence.txt for it)."
- This replaces item 1a's "Numbers and the two policy strings only".

**C5, section 1 item 1e, first condition.** "The redaction check is the pre-push hook's own: the fixed copy
`~/npt-operator/redact-check.mjs` with `--vms ~/npt-operator/vms.txt`, run on `<results>\r1\s14`, its whole output
saved as `<results>\r1\s14-redact.txt`. A finding, or any exit but 0, means no push: stop and tell him."

**C6, section 1 item 1e, second condition.** "The R1.md sentence and the pin test name every writer into the folder:
- npt-s14's kernel.txt and server-, client- and bursts-run<N>.json;
- r1.sh's s14 block: summary.txt, INVALID-s14.txt, rerun/ (npt-s14's same files and its summary), rerun-R2.txt and
  browser-stamps.txt.
Any other file, writer, key or address fails the pin test by name, with the message that such a change goes to
Mustafa first."

**C7, section 5.** "security-auditor's one read also covers items 1a and 1b's print code and item 1e's pin test (what
leaves the server), beside the new PROMPT-R1.md and R1.md lines."

## Decisions needed
None. These are choices inside his two answers; none moves a mark, money or main.

## For Mustafa
Nothing. In the day's message, his to reverse: the policy reads leave the server as numbers (C4); facts.txt and
presence.txt's verdict lines go as summaries.

DONE
