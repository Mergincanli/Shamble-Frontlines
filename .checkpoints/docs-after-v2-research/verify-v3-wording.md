V3 decision wording check (MODES-3..SLICE-ORDER, advisor exact texts, reporting rule, PR #31 quotes) on e99773f
- unit 2 (advisor exact texts in docs): conditions 7, 8, 9, 11.2, 11.4, 11.5, 12 match head e99773f word for word after whitespace/bold normalisation (scratch script cmp.py). OK.
- unit 1a (docs diff wording): STATUS, ROADMAP, VISION_V2, REPO_MAP, research README and the research notes read; quotes of MODES-3, PREVIEW-WALL, SEC-SPREAD, BASELINE-WHEN, VISION-LINE, TICK-30, SLICE-ORDER + 3 conditions match decisions file; ABL-PREDICT stays open; "the foundation" in Advisor's words.
- unit 3 (issue #16 live): condition 13 text present verbatim; the three NEW sentences in PR body are in the live body; OLD ones gone; old/new match issue16-diff.md. OK.
- unit 4 (PR #31 live, head e99773f): both 2026-10-31 quote blocks byte-exact vs decisions file; as-proposed line present; issue #16 old/new present. OK.
- unit 5 (reporting rule in living docs): no comma decimals in added lines; codes explained on their line except STATUS.md:217 "under NS-2 (c)" (explained 11 lines up at :206) -> nit. C17/C23 ids and R2-BUD-REC6 unexplained but inside the advisor's mandated exact texts (conds 7, 11) -> not a finding. KIT31-STATE pre-existing.
- unit 6 (PR body extra wording): FOUNDATION question's option (a) glosses phase 1 as "restructure, toolchain and CI gate v2", but V2_PHASE0_BRIEF.md:22 adds "then the first playable slice" -> unclear.
DONE
