# Night shift
1. Window: work only between 00:00 and 08:30 Europe/Amsterdam (check with
   `TZ=Europe/Amsterdam date`). Outside it, stop silently: no comment, no notification.
   Start no new task after 08:15; by 08:30, report and stop.
2. Read the open issue "Night queue". If no unchecked items are left: when tonight's report
   already exists, stop silently; otherwise comment "Nothing queued tonight. Night shift
   done." and stop.
3. Lock: if a comment from tonight says "Night shift running" with no "Night shift done"
   after it and is less than 90 minutes old, another run is active: stop silently. If it is
   older, that run was cut off by a limit: take over. Otherwise comment "Night shift
   running (HH:MM)".
4. Work the unchecked items from top to bottom. Only approved work runs. An item marked
   NEEDS PLAN gets a written plan, listed under Decisions needed; then move on. Never guess
   a decision the plan doesn't cover.
5. Absolute rules (there are no approval prompts tonight): never merge a pull request;
   never push to main; never change .claude/ or CLAUDE.md; never delete branches, saves or
   data; if the gate fails and you can't fix it properly, stop that item and report it.
6. If you hit a usage or spend limit, stop cleanly. One of tonight's later start times
   continues from the queue.
7. Keep ONE report comment per night on the issue: create it, or add to it if tonight's
   exists, and end it with "Night shift done":
   - TL;DR: at most 5 lines, plain words: what got done.
   - Details (optional reading): one short paragraph per item, with PR links.
   - Decisions needed: self-contained entries (code · question · why it matters ·
     options · recommendation).
   - For Mustafa: numbered actions (Brave checks with steps, merges to approve), or
     "nothing".
   Tick the items you finished.
