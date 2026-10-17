# Night shift
Follow CLAUDE.md in full. Several identical night-shift routines start through the night.
Only one may work at a time; a later one continues where an earlier one stopped.

START CHECKS (in this order; "stop silently" = no comment, no notification)
1. Window: check `TZ=Europe/Amsterdam date`. Outside 00:00–07:15, stop silently.
2. Read the open issue "Night queue" and tonight's comments (tonight = since 00:00 Amsterdam).
3. Busy: if tonight's status comment says "Night shift running" and its "Last update" is
   less than 45 minutes old, another run is working: stop silently.
4. Handoff: if that status comment says "running" but its "Last update" is older than
   45 minutes, that run was cut off by a limit. Read its "Working on" and "Next" lines, check
   out its branch, check the last commit and run the gate, and continue from there. If its
   state is unclear, restart that item cleanly on the same branch and say so in the report.
5. Nothing to do: if no unchecked items are left, stop silently when tonight's report
   already exists; otherwise comment "Nothing queued tonight. Night shift done." and stop.

WHILE WORKING
6. Keep ONE status comment for tonight (create it, or take over the cut-off one) and keep
   editing it: "Night shift running · started HH:MM · Last update HH:MM · Working on: <item>,
   branch <name> · Next: <step>". Update it after every commit and at least every 30 minutes:
   it is the handoff if a limit cuts you off.
7. Work the unchecked items from top to bottom, each on its own cs/<slug> branch. Commit and
   push small steps often, so a cut-off loses little. Only approved work runs: an item marked
   NEEDS PLAN gets a written plan under Decisions needed; then move on. Never guess a
   decision the plan doesn't cover.
8. Absolute rules (there are no approval prompts tonight): never merge a pull request; never
   push to main; never change .claude/ or CLAUDE.md; never delete branches, saves or data;
   if the gate fails and you can't fix it properly, stop that item and report it.

FINISH (start no new item after 08:15; finish by 08:30, or earlier when the queue is empty)
9. Keep ONE report comment per night (create it, or extend tonight's), ending with
   "Night shift done":
   - TL;DR: at most 5 lines, plain words: what got done tonight, all runs together.
   - Details (optional reading): one short paragraph per item, with PR links.
   - Decisions needed: self-contained entries (code · question · why it matters · options ·
     recommendation).
   - For Mustafa: numbered actions (Brave checks with steps, merges to approve), or
     "nothing".
   Tick the items you finished, then edit the status comment to "Night shift done (HH:MM)".
