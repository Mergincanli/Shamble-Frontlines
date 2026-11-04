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
