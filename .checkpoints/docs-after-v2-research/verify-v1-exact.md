V1 exact edits verification (verify-v1-exact) of cs/docs-after-v2-research e99773f vs b6effde: checks 1-6.
Unit 1 (multiset diff vs research-edits.diff): 92 vs 92 changed lines, identical multisets. OK.
Unit 5 (diff --stat scope): only STATUS, ROADMAP, REPO_MAP, VISION_V2, research/{7 files + README}, .checkpoints/. OK.
Unit 1b: script run on b6effde copies reproduces research-edits.diff and the table exactly, and --apply output is byte-identical to e99773f's 7 files (39 edits, 0 problems). OK.
Unit 2a: replace edits C17.b1 (cond 2), C17.b2 (script), C17.b3 (appendix 1071-1080), C23.b1/b3 (appendix 1415-1428 + 1596-1599), C23.b2 + rests-on (cond 4), LOG-R1/R2/R5/R6 (cond 6) checked word for word in the head diff. OK.
Unit 2b/3: all 16 shared-question notes and 12 glossary edits read in context; each note sits inside its entry (next line is '- **', blank or heading) and matches the entry's (a)/(b) reading; SEC-SPREAD entries have no (a)/(b) labels (nit).
Unit 4 (VISION_V2): diff is exactly part V's two lines replacing line 43, nothing else. OK.
Unit 6 (markdown): every added line is a continuation inside its list item/paragraph or a replaced list-item start; bold pairs balanced per paragraph; glossary anchors #the-advisories-check, #the-security-baseline, #game-server-certificate exist (GLOSSARY 690, 668, 706); lines over 120 chars were already over 120 before. OK.
Result: no problems; one nit (SEC-SPREAD '(a)' not defined in the research entries).
DONE
