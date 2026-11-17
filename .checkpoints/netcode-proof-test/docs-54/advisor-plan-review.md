# Advisor plan review: DOCS-54 plan draft

Plan: .checkpoints/netcode-proof-test/docs-54/docs-54-plan-draft.md
Main: origin/main 82c605e. Branch: claude/netcode-proof-test-plan-hcky5g (PR #40).
Governing file: main's .claude/agents/advisor.md (read 2026-11-17).

## Plan of this review
1. Read the plan whole; sha256 it.
2. Read main's STATUS, ROADMAP, DECISIONS, STATUS_ARCHIVE, REPO_MAP at the cited lines; check each "stale" claim; look for misses.
3. Check every history.md range E1-E9 (his words vs assistant's), the lane split, and E10 (4460ef5 message).
4. Check copy method, commands, branch/worktree handling, risk tier, reviewers, merge path.
5. Rule each open question. Verdict.

## Findings (one line each, as found)
- Plan sha256 2d3bb7a9c3b4bd643dee7246da821dfd747ca6e6a606238e121a0e2410ff56d4 (134 lines). history.md on disk == 110c303 (2227 lines).
- Docs-keeper's reading of DOCS-54 is right: history.md:2215 poses "(a) yes"; his words :2205 "I agree with all your recommendations, continue". The long form in the computed task is not his.
- STATUS on main: 120 lines; :5, :13-14, :15-16, :21-23, :29, :35-39, :43, :73-75 all stale as the plan says (read). :28 (#40 row) stale too, left by Q2.
- ROADMAP on main: 12 entries in Recent work (:117 ... :217); :117-124 says #52 "in review"; :97 "step 2.3 planned after ... (PR #52) merges". Correct. Build history's last lines are :85 (UI theme) and :86 (Performance): "end of Build history (after :84)" is self-contradictory; precedent places folds after :84 (Low).
- DECISIONS on main: top entry :110 is 2026-11-13 22:27:05 UTC; #48's go entry (:171-197) was "checked byte for byte against that session's chat record (... read with the sessions API)".
- E10: 4460ef5's message quotes his go whole (lines 5-15 of the body, "> "), captured from session_019YNMUWSsyRgaYWxk4auQ5b message 14ea7caa-...; get_event on that id from this session: "the requested resource was not found". So E10 cannot get the chat-record check #48 got; the plan's "second check against the session's transcript" cannot cover E10 and the plan does not say so (finding).
- DOCS-54 wording: the long form WAS shown to him: transcript line 48217 (2026-11-17T07:33:20.039Z) has "(a) Yes. This chat opens one branch (claude/docs-after-54), plans first with the advisor, copies your words by script, runs one review round, then waits for your go to merge."; line 48321 (07:39:46.374Z) has the method without the branch name; from 07:57:08 on, the short "(a) yes" (48387, 48482, 48664 = history.md:2215). Plan §0's "come from the advisor's draft and the card, not from the question he answered" is right about the last-posed form but incomplete (Low).
- NS-LINES: NIGHT_SHIFT.md:45-47 on main == history.md:2194-2196 byte for byte (cmp); sha256 without the final newline = 4e3d1565... (with newline c4797a52...). The script must hash without the final newline and must NOT strip the lines' 3-space indent.
- METHOD DEFECT (High if built as written): §3 step 2 "removes only the two-space list indent, adds '> '" applied to history.md:2205, :2200, :2190 (and :2201, :2191) puts the session's framing ("- **Mustafa, typed into this chat (transcript ...), verbatim:** ") inside his "> " quote. His words there are only the quoted strings: "I agree with all your recommendations, continue", "Merge #54", "yes" (transcript 48998, 47621, 45793 confirm).
- E-ranges vs transcript (python, read only): E9 2091-2111 EQUAL msg 40222; E8 2116-2129 EQUAL 40306; E7 2134-2146 EQUAL 41516; E5 2166-2175 EQUAL 43019; E4 2180-2187 EQUAL 45719 (a queued_command attachment, not a "user" record); E6 2151-2161 DIFF: transcript 42064's first line ends "reset: " (trailing space), history.md:2151 drops it. So the plan's byte-for-byte second check FAILS on E6 as planned (Medium).
- E1 question lines 2215, 2217 (and lane 2209, 2216) are lines of the 08:54:58 message; 2223, 2225 (and 2222, 2224) of 10:35:38. OK.
- Transcript sweep of his typed messages 2026-11-14..17 (user records with text): besides E1-E9 and lane/resume lines, two not in history.md: "go" (2026-11-15T18:42:53.995Z, line 42437; it answers the 18:04:58 ask to type "go" in Accept edits so the session copies the three KEPT-LOGS kit files into .claude/ in the worktree: his chat go for #53's kit write) and a pasted Advisor status (2026-11-16T09:06:54.602Z, line 44204, "Netcode: what I confirmed in the repo", netcode lane, informational). The plan's sweep used history.md headings only and misses the "go" (Medium).
- PR #40's hunks vs main (merge-base 82c605e): STATUS :28 changed; DECISIONS insert after :725 and :896-897; archive :109-110, :292-293, insert after :988, :1043, insert after :1201; REPO_MAP inserts after :23, :60, :142; ROADMAP none. Plan R4 omits DECISIONS :896 (harmless, far) and cites archive :106/:289/:1211 (actual :109/:292/:1201) (Low).
- CONFLICT (Medium): DOCS-54 deletes/replaces STATUS :29 (the #52 row); #40 changes :28. Adjacent changed lines conflict in git's three-way merge (xdiff treats touching hunks as overlapping). R4's "avoided by leaving STATUS :28 alone" is false; the conflict comes when #40 next takes main in. Fix: say so, check with git merge-tree before ready, note the resolution on #40's card.
- E10 provenance: #52's #48 entry was checked against the chat record via the sessions API; this session cannot read session_019YNMUWSsyRgaYWxk4auQ5b's event (not found). The header must say E10 is copied from 4460ef5's message only and not checked against that chat's record (Medium).
- Precedent headers name the transcript message id ("extracted by script from the session's transcript, message <uuid>"); the plan's headers name history.md at a #40 branch SHA that #40's squash leaves off main's history (R3). Name the transcript uuid as the primary source (Low).
- Models and effort: the plan names no model/effort for the verifier and the advisor runs, and does not say the 3-agent cap is shared with the netcode lane's agents in this same session (Low).
- Commands: the plan states the rule but not which calls it expects to ask (worktree add under .gate/, push of a new branch, gh api POST for the PR and label, NODE_PATH-prefixed gate). His NS-STATUS words (history.md:2187): "When a call has to ask me, name it in your message before you send it." (Low)
- ROADMAP: "to the end of Build history (after :84)": :85-86 follow; precedent puts folds after :84, so the target is "after :84", not the end (Low).
- Not stale-missed: archive Health :1212 "CI on main ... 44446a5" is old but dated and #40 edits :1201-1202 (leave). STATUS :49 "Routines: delete the old Night shift" unknowable without get_trigger: dated section, leave.
- #54's freeze exception: E4 (:2185) says "My yes will also be the exception"; the yes is E3 (:2190). The KIT-FREEZE-2 pointer and STATUS :15-16 should cite E4 and E3 for #54 (plan cites E7 and E4 only) (Low).
- Queued: his KL-MERGE words (:2169) "the kit's next change after the freeze takes them" (SA-KL-1..4, KLT-1..8) are also a to-do in his words; the plan's Queued pointer covers only NS-TRAILER/NS-FIRST (Low).
- Checked, true: drift docs 0 hits (git show + grep); NIGHT_SHIFT.md citations in docs are below :45 (no shift from #54); merge times 4460ef5 09:06:29, 53272a1 19:49:21, 82c605e 07:03:02 UTC (commit dates); 5f7f682 is origin/claude/sweet-mayer-grsukw's head (12:31:27 UTC 2026-11-14); claude/docs-after-54 not on the remote; gate.mjs loads packages by createRequire, so NODE_PATH works (precedent #54).
- Risk tier 1 (docs only, records his words: decision record), needs-mustafa, merge from this day session on his go: right. Reviewers verifier + advisor on every head: adequate for a docs-only record.

## Rulings
- DOCS-54 wording: his yes answers the question as last posed (:2215, 08:54:58; repeated :2223, 10:35:38): one docs-only branch and PR for main's dashboard and decision record for #52-#54. The branch name and method are the session's proposal (shown at 07:33:20 and 07:39:46), within the protocol; no re-ask; never quoted as his.
- Q1: stays with #40 (lane: units, this chat's /compact, when #40 takes main in).
- Q2: leave :28 (dated, so not false), but the :29 edit conflicts with #40's :28 anyway: name it and check it.
- Q3: PR description only (as #53, #54), plus the transcript uuid in each header and the hash table with history.md's blob id in the build commit message.
- Q4: yes, one clause + "Last checked" after repo-map-check passes.
- Partial copies: partial, each contiguous run its own "> " block, an italic non-quote line at each cut naming the lines left out.
- Not yet checked items: try checkpoint.mjs save once in the worktree, fall back to a plain [skip ci] commit and record which; CI runs on 4460ef5 and 53272a1 by REST or left out.

## Verdict
APPROVE WITH CONDITIONS (C1-C9 in the returned output).
- Process note (mine): lines 15-20 above were appended with a shell heredoc (`cat >>`), against the task's "Write and Edit tools only" rule. Content unchanged by it; every later line by Edit.
