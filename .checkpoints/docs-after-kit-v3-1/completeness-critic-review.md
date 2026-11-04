# completeness-critic review: docs-after-kit-v3-1 (head 603f53b on main 868e165)

## Plan
1. Read plan rev 2, Mustafa's answers, advisor's 16 conditions, items, evidence (done).
2. Read the full diff; map each item/answer/condition to file:line (covered/missing).
3. Cross-file contradictions: effort, trigger rule, shell list, night window, merge facts, queue order, A2/A3 naming.
4. Stale-doc greps outside the diff: "fires on pull request events", "PR-HOME", "effortLevel", "routine's default",
   "Guard test", "paused".
5. Verdict.

## Notes as I go
- Read full diff (12 files). Queue order STATUS:34-38 = ROADMAP:70 (docs pass; B3; kit-v3-2=A2; A3; EDITOR_V2; VISION_V2).
- A3 naming: advisor.md:78,149 "A3, the guard's follow-up change-set"; STATUS Open bugs "A3 ... right after kit-v3-2". OK so far.
- CLAUDE.md shell list still says "limited to git, gh, npm ci, node ... and date": omits python (guard :445) - recorded as gap in STATUS Open bugs.
- Coverage: every items-file entry, Mustafa answer, rev-2 item and advisor condition 1-16 mapped to the diff; all covered
  except the GLOSSARY isolation-flag entry (items file named it).
- Greps: no "fires on pull request events", "one run per ready PR", "routine's default"; "Guard test"/"PR-HOME"/"paused"
  hits are current or historical; effortLevel hits consistent (settings.json, REPO_MAP:76 qualified, test code).
- Night window 00:10-07:30 consistent (CLAUDE.md:116, NIGHT_SHIFT:16, REPO_MAP:75, STATUS:400, KT14, BRIEF:70).
- Merge facts consistent across STATUS header/Phase/In flight and ROADMAP. Queue order consistent STATUS:34-38/ROADMAP:70.
- A2/A3: Actions item only ever under A3. OK.
## Findings
1. major: docs/research/GLOSSARY.md:641-642 "then the default flips everywhere at once" unqualified; contradicts ADR 0002
   part 5 (:364-368), failure-mode (:1507-1511), OQ-0002-9. Items file listed it. ADR 0002:359 links to it.
2. minor: .claude/CHECKPOINTS.md:66 "advisor writes its notes with Bash" unqualified; contradicts advisor.md:148-150.
3. minor: CLAUDE.md:131-133 drops STATUS:436's "likely" hedge (day-session log only) - condition 2.
4. minor: STATUS:125-126 cites the checkpoint the last commit deletes; STATUS:79 row "building". Fix in last commit.
5. nit: STATUS:462 "[CLAUDE.md is being edited in this pass]" stale after merge.
6. nit: STATUS:125-145 Mustafa's 2026-11-04 list omits his `date` shell-list ruling.
DONE
