Mustafa's words for A3 (kit-v3-3), word for word, as given in the A2 day session (session_0173vVo7FcYUpbWgRW8oQyHU).

2026-11-04, with TEST-RUN B2's result (the A2 item that became A3's third point):
"A2: the guard refused actions_get, a read-only GitHub Actions lookup the PR Advisor needs for CI details; consider
allowing read-only Actions tools for pr-advisor, with security-auditor's review."

2026-11-04, his answers (the A3 scope):
"KIT-FOLLOWUPS: (a), a small A3 right after A2 for the guard's follow-ups: ask before kit and gate edits in day
sessions; the sk- key after %20, %2C or %3A; and read-only GitHub Actions tools (such as actions_get) for pr-advisor,
which B2 showed it needs for CI details. security-auditor reviews A3. GitHub secret scanning and push protection are
now on."

2026-11-04 (~22:45 UTC), after PR #37's merge, two additions, word for word:
"After the merge, two additions:
1. A3 also gets the worktree rule from the freeze, which never reached the rules: never switch a session's main
folder to another branch, a PR's head or an old commit; use a separate worktree (git worktree add). Why: switching
loads that branch's .claude/settings.json and hooks into the running session. That froze one session, and it would
let a PR's settings act on a day session.
2. The three deferred points (and anything else in your scratchpad queue file) live only in /tmp, which is lost with
the session. Record them in the repo, in STATUS's follow-ups, alongside A3's plan."

2026-11-05 (~10:30 UTC), his answers to WT-NIGHT and PC-SETUP, word for word:
"WT-NIGHT: (a). The exception holds only when the branch's rules, CI and gate files match main's.
PC-SETUP: 1. Permission mode: [Auto]. 2. Mods: [no mods installed but I have been thinking about it]. 3. Enterprise login (my university account). 4. Claude Code runs in Windows itself (PowerShell), not WSL"

2026-11-05 (~16:10 UTC), his scope stop for A3, during fix pass 3, word for word:
"Scope stop for A3 (Mustafa): the guard will never parse all of bash, so stop teaching it new shell syntax. Instead: in routines, R-PLAIN refuses any line that isn't plain; in day sessions, any line the guard can't fully read is an ask, not a parse. Drop Day C's parser additions (comments, line continuations, $'…' quoting, here-document markers, >&file). Keep the git clone fix, the check that no routine line got looser, and the full gate. Finish fix pass 3 with that, then one final review (pass 4), and apply the review budget now: anything pass 4 finds below Critical or High becomes a STATUS follow-up, not another pass. After A3, the kit is frozen until the netcode proof test is under way."
