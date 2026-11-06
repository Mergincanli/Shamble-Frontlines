Task: advisor PR review of #41 (claude/npt-readings, R-PR of the netcode proof test), head 2ccc184b55f62430ece8f47afcfe8c058816f863, base af8393e9d40f3bd382e77f86e6df2612785c706c. Day session, verdict only.

## Plan (blocks)
B1. Read advisor-verdict.md (R-PR bullets, exact words, Mustafa answers as recorded).
B2. Read the diff (stat + word diff) of the 5 files; file list check (no .claude/CLAUDE.md/.github/gate files).
B3. Check each R-PR bullet at its anchor, exact words (scripted compare).
B4. Check nothing else changed / no mark moved; pointers on overtaken lines accurate, add no rule.
B5. Mustafa's answers quoted exactly; rule on the "applies it" sentence.
B6. CI check runs on head (GitHub tools); PR state; no .checkpoints in PR.
B7. Verdict written.

## Notes
- full head SHA 2ccc184b55f62430ece8f47afcfe8c058816f863; merge-base = af8393e (main at PR open). 5 files, +96 -34.
- B1 DONE: verdict has 15 R-PR bullets (R-1/18 ports 0006; R-5 part 13; R-6/9 PM-0001-3; R-7 PM-0001-28; R-10 OQ-0001-9 cover; R-12 bites rows 0001/0006/0004; R-13 SPIKE_PLAN:53-55; R-14 SPIKE_PLAN item 1; R-15 PM-0004-67; R-16 PM-0001-59; R-17 PM-0001-23/-24; R-25 0001:1444-1445; R-26 0006:1314-1316; R-28 checklist; FIXED-IP SPIKE_PLAN:64-66,173 with <date>). Watch: R-12 "digest F proposals, each written into its own row" (is it just placed text?); FIXED-IP <date> should become 2026-11-06. Next B2.
- B2a (ADR 0001 diff read): R-5, R-25, R-16, R-17 (both rows), R-7, R-28, R-12 bites (-18,-36,-55, "Memory and queue marks bite over a full S30" in bites rule ~1527) at anchors, words match on read.
- F1 (0001 PM-0001-3, line ~1537): R-9's sentence spliced into the existing cell, not verbatim: "the injected delay in each direction set per direction to (the profile's RTT − the path's measured base RTT) ÷ 2" -- meaning = my reading; redundant "per direction". Cadence column "every L2 session" -> "every L2 block" (matches my R-6 words "at the start of every L2 block"). 100,000 -> 1,000,000 per R-6. OK.
- F2 (0001 PM-0001-3): NEW Bites list not in verdict words ("full round trip applied each way fails the mean-delay band; loss drawn once per round trip...; normal jitter...; RNG seeded from clock...; on L2 delay not net of base RTT fails MP's 100 ± 5 ms"), tagged advisor's reading. Presumably digest F proposals (R-12: "each written into its own row"). MUST check vs test-engineer-digest.md F.
- F3 (0001 OQ-0001-9 ~1167): extra lead-in sentence not mine: "The cover half is answered by the Advisor's rule (2026-11-06), which Mustafa may overrule at the close." Check OQ-0001-9's decider/status and README index.
- F4 (0001 ~1411): pointer on MACHINES-TIMING quote: "(overtaken by Mustafa's answer of 2026-11-06, SPIKE_PLAN's Machines: no Mac is available, and the Safari rows wait for one)" -- "the Safari rows" vs his "the timed Safari rows" (+ runner-fail case). Check context.
- F5 (0001 checklist ~1664): pointer "The profile changes from the overlay (...), and the cap is set as the advisor's reading below says" + my words repeat "The profile changes from the overlay." Redundant, no new rule.
- 0001 ~1473 pointer "An explicit reorder step joins the profiles (part 13, ...)" check context; ~427 pointer to OQ-0006-16 OK.
- B2b (other 4 files' diff read): R-13, R-14, FIXED-IP x2 (date filled 2026-11-06), R-15, R-12 (-67, -11, -26, -14), R-26, R-1/18 ports (0006 + README row) at anchors; Mac quote + applies-it sentence at SPIKE_PLAN ~57-60; OQ-0006-19 answered (a) in 0006 x3 + README.
- F6 (SPIKE_PLAN ~74-77): ROUTE line is a paraphrase, not a quote: "ROUTE (Mustafa, 2026-11-06): the hands-on server work runs through cloud-init at creation and Claude Code on his laptop over SSH, with a key made on the laptop; this cloud session cannot reach the servers over SSH." Check vs the plan's ROUTE option (a).
- F7 (SPIKE_PLAN ~57-60): applies-it sentence cites "condition (i), (ii) or (iii)"; R-13's new words say "two conditions" unnumbered. Check (i)-(iii) defined in Machines.
- F8 (0006 ~1337): gloss "(ADR 0001's 30-minute soak, its key's Soaks)" added to my R-12 sentence; check accurate.
- F9 (0006 OQ-0006-16 lead + README): lead "Answered in part ... by infra-engineer and netcode-architect, confirmed by the Advisor" = R-1 ruling; OK; 443 sentence repeated (harmless).
- Next: B3 scripted exact-word compare; check digest F; plan ROUTE/FIXED-IP; (i)-(iii).
- F2 RESOLVED: PM-0001-3's 5 bites = test-engineer-digest.md:227 REC (a)-(e) verbatim (digest F's only other proposal, -18, is covered by my R-12 words). Matches R-12's "digest F proposals, each written into its own row". OK.
- F6 checked: plan.md:1333-1334 ROUTE (a) = "cloud-init builds each VM, and Claude Code on your laptop runs the sessions over SSH, with a key made there, a fixed settings file of its own, and your prompts as the last wall". PR's ROUTE line condenses it, drops "a fixed settings file of its own, and your prompts as the last wall"; no quote marks, untagged; adds the true fact (plan §1.1, ssh-probe) that this cloud session can't SSH. Minor, non-blocking (suggest full option text).
- FIXED-IP (a) plan.md:173 "keep the two addresses; delete them at the close"; PR uses my FIXED-IP line with date 2026-11-06 at both anchors. OK. DENSITY-FIGURE (a) wording "a lower bound, provisional until slice 6" = plan.md:174. OK.
- F7 RESOLVED: SPIKE_PLAN:50-53 defines (i) Safari >= 26.4, (ii) real Safari via safaridriver, (iii) WT session to RS + WS session on the domain. Reference is correct.
- B5 ruling (applies-it): fair. His MACHINES-TIMING branch 2 ("If they can't, I borrow the Mac before") is impossible now (no Mac) and his new words say nothing in this test waits on one; so runner-fail => all Safari rows pending until the Mac, which is the sentence. "that Mac" = the Mac he names, at the latest the look proof test. Note (info, not condition): under runner-fail, PM-0001-28's library pick and the count rows go ahead without Safari until then (C8: migration step 1 waits on ML/RD only).
- F4 RESOLVED: 0001:1411 and SPIKE_PLAN:172 pointers sit on the "if they can't, borrow before" branch, where indeed every Safari row waits; accurate, no rule. 0001:1473 reorder pointer accurate. F8 RESOLVED: S30 = key's Soaks (0001:1474), 30 min measured; gloss accurate.
- B2 DONE (all hunks classified: 0001 15 hunks, SPIKE_PLAN 5, 0004 1, 0006 9, README 3; each = my words, Mustafa's answer, digest F bites, or pointer). Next B3 script compare.
- F10 (README index, lines 16, 21, 23-24): open-question counts NOT updated: 0006 row still "18 open" and doesn't list -19 answered by Mustafa (2026-11-06) or -16 answered in part; 0001 row doesn't list -9 answered in part; Total line "98 of them open" not updated. Convention (README:23-28) does track these. Register now internally inconsistent. Low severity; decide fix-now vs follow-up.
- OQ-0006-19 answer text "(a), a lower bound, provisional until slice 6" = plan.md:174 and the option (0006:1103-1106). OK.
- Scope: plan.md:1149-1151 lists R-PR = R-1,5,6,7,9,10,12-17,25,26,28 + FIXED-IP; plan.md:174 adds OQ-0006-19. PR also adds Mustafa's Mac words + applies-it sentence and a ROUTE line: beyond the listed scope, but they record his own answers and correct SPIKE_PLAN/0001's now-false "borrow before" branch. Acceptable; say so.
- Mac words: match checkpoint .checkpoints/netcode-proof-test.md:102-104 exactly; I cannot see his chat (not verified at source).
- B4 mostly done via read (pointers ok). Next B3 script.
- B3 DONE (own script advisor-pr41-check.py, git show of base/head, whitespace-normalised): every sentence of the 15 bullets present at head, absent at base, except by design: R-9's sentence spliced (F1); R-12's "Bites added" list distributed as "Bites: X (tag)" into each own row (5 greps, 1 each, right rows); "digest F" sentence applied as PM-0001-3's 5 bites. Tags 2026-11-06: 0001 17, 0004 2, 0006 6, SP 4, README 2; none at base. Table rows: pipe counts unchanged, no new rows; prose lines <=120, no trailing ws. README OQ-0006-19 row renamed "OQ-0006-19 (DENSITY-FIGURE)" (style: other named rows read "NAME (OQ-…)"; trivial).
- Next B6 CI.
