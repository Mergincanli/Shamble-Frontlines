# The advisor's ruling on the glossary Retire split (verbatim)

Workflow run wf_06dbf34d-b30 (prompt `prompts/advisor-retire-split.txt`), returned 2026-10-30 about 18:21 UTC.

**Verdict: APPROVE, with conditions.** The split serves condition 9's purpose: every topic uses the glossary, and anyone can check it. I ran the script myself and got 1,335 hits. About 380 of them come from the script, not from wrong uses.

**Changes:**
1. **Kept words come out of the quotes.** The script picks up every quoted string in "Retire", including words the text keeps: "Core" (76 hits), "big" (53), "Whole ticks" (5), and "the security baseline" (2), which is the correct name. These words, and replacement words such as "authority-only" and "the largest mode's N", should move to an unquoted "Keeps:" clause. That is a format fix, listed in the edit note.
2. **Script fixes:**
   - Match whole words only, so "core" no longer hits "simulation core" or "score", and "48" no longer hits "1480".
   - Skip hits inside the entry's own term name, its "Marked by" phrases, or a longer phrase. Today "tick time" hits the term "Server tick time", and "whole tick" hits "Whole ticks".
   - Count each place in the text once. Today "48" is counted twice, 244 times under each of two entries.
   - Print every hit with its file, line and nearest id.
3. **The test for "exact":** a phrase goes there only if none of today's hits is a correct use.
   - "Core" goes nowhere: it is not retired at all.
   - "server core" and "1 CPU core" go to exact. "one core" and "per core" go to exact only if all 11 of today's hits are about server work.
   - The id-anchored phrases are exact. Everything with a context clause stays "in context".
4. **Before step 3c:** the session checks that the quoted phrases are the same before and after the edit, apart from item 1. It re-runs the baseline and records both counts in the checkpoint.

**Rule for the step-3d verifiers:** an "in context" hit is wrong when, in its sentence, it carries the term's meaning without the term's marking words. Wrong uses go back to their owner. A hit is right in three cases:
- it means something else (client cores, character models, a stated N = 48 scale point);
- it sits beside the marking words;
- it is quoted text or a dated record.

When unsure, count it wrong. Every hit gets judged, with no sampling, and each judgment records file:line, right or wrong, and one line of reason. The report shows the exact count (it must be 0), the in-context hits, how many were wrong, and how many were fixed.

My notes are in /home/user/Shamble-Frontlines/.checkpoints/v2-research/advisor-retire-split.md.
