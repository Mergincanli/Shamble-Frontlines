"""Move the resume card's U4d-F blocks to history.md at U4d-F's merge (2026-11-12), found by their own lines, and put
the condensed text in their place; the plan files' sha256 are computed here. Run from the repo root:
python3 -I .checkpoints/netcode-proof-test/card_refresh4.py [--apply]"""
import hashlib
import sys

N = ".checkpoints/netcode-proof-test"
CARD = ".checkpoints/netcode-proof-test.md"
HIST = f"{N}/history.md"
APPLY = "--apply" in sys.argv
with open(CARD, encoding="utf-8") as fh:
    card = fh.read().split("\n")
with open(HIST, encoding="utf-8") as fh:
    hist = fh.read().split("\n")


def sha(name):
    with open(f"{N}/{name}", "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()[:8]


def only(pred, what):
    hits = [i for i, l in enumerate(card) if pred(l)]
    assert len(hits) == 1, (what, hits)
    return hits[0]


i_last = only(lambda l: l.startswith("- **Last update:** "), "last update")
assert card[i_last + 1].startswith("  ") and not card[i_last + 2].strip()
i_step = only(lambda l: l.startswith("**The current step: U4 wave 1's close"), "current step")
i_run = only(lambda l: l == "- **Running:**", "running")
i_cut = only(lambda l: l.startswith("**The three cut workflows, and what to do with each"), "cut workflows")
i_batch = only(lambda l: l.startswith("**The advisor's text batch: APPLIED"), "text batch")
i_next = only(lambda l: l.startswith("**Next steps, in order (Mustafa: \"follow the card's order"), "next steps")
i_done = only(lambda l: l.startswith("- **Done for the report of 2026-11-11 evening:**"), "done for the report")
i_wt = only(lambda l: l.startswith("  agent-a9297644e7c0354f3 (U4a), wf_f54246ff-e0d-1 (U4d)"), "housekeeping")
assert i_last < i_step < i_run < i_cut < i_batch < i_next < i_done < i_wt
assert card[i_wt + 1] == "  merged at 4d2b0ef)."

LAST = """- **Last update:** 2026-11-12 about 01:55 UTC (U4d-F merged into the spike branch: the commit "U4d-F: the estimate
  scene fix (npt-sim), merged after review round 2")""".split("\n")
STEP = f"""**The current step: U4 wave 2's start.** U4e's L3 (the carry-51 commit), then U4e's launch (L4); U4d2's brief with the
advisor. Plan files: plan.md (rev 3.3, sha256 {sha("plan.md")}…), u4-build-plan.md (rev 2 with the U4e brief ruling's
L2 edits, {sha("u4-build-plan.md")}…), u4s-carries.md (carries 1-72, {sha("u4s-carries.md")}…); the advisor's rulings: advisor-u4.md,
advisor-u4-wave1.md (with B1.4's erratum), advisor-u4j.md, advisor-u4d-close.md, advisor-u4j-r2.md,
advisor-u4e-brief.md, advisor-u4df.md.
- **Merged into the spike branch:** U4s f080551, U4b 411eeee, U4a 952f734, U4c f3f7c2f, U4d with W1-S1's npt-proto half
  39f4290 (report u4d-report.md), U4j 4d2b0ef and d50396a (u4j-build.md), and **U4d-F** (the commit "U4d-F: the
  estimate scene fix (npt-sim), merged after review round 2"; report u4df-report.md; record u4df-build.md); CI green
  on each through U4j. Main merged in at 5ca77eb (PR #49). At U4d-F's merge: check.mjs 10 steps, measured 399, bites
  446, self 52: the base counts for the next lanes (U4e's `<base>` comes from L3's counted run).""".split("\n")
RUN = """- **Running:** no agent. The saver: `node .claude/scripts/checkpoint.mjs watch netcode-proof-test --trailer
  "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>" --trailer "Claude-Session:
  https://claude.ai/code/session_0173vVo6igCNyQYBWPvhH36A"` (background, 120 minutes; restart it when it stops). Stop
  it before a merge's apply and restart it after the push.
""".split("\n")
CUT = """**Finished units, and what stays open from each** (their old blocks: history.md, "Running work, U4 wave 1 (3)" for
U4j, and "Running work, U4d-F" for U4d-F and the U4e brief's drafting):
1. **U4j: MERGED** (4d2b0ef, d50396a). Open, as routed: carries 61 (§U4e), 62 and 63 (§U4r), 64 (the next change to
   crates/judge), 65 (§U4d2, or the main session sooner); C-U6-9 to C-U6-11 (plan.md's U6 row); the FU list for PR
   #40's report (the closing checklist); one more reading only if U4e's client drops stale groups (advisor-u4j-r2.md
   B5).
2. **U4d-F, the estimate scene fix: MERGED** (2026-11-12 about 01:55 UTC). Run A (four builders; 3b's re-home;
   `aim_one_in` 40), round 1 (both seats APPROVE WITH FINDINGS: NF-M1 and TF-M1 Medium), the advisor's ruling
   (advisor-u4df.md: APPROVE WITH CONDITIONS; RM, carry 67; FX-1 and FX-2 before the merge), the fix pass (FX-1 exact
   on 954,628 sky aims; Q1-Q5 each fail it by name), round 2 (both seats APPROVE WITH FINDINGS; one Low, NC2-L1), the
   merge on M1-M9 (u4df-build.md, "Merge into the spike branch": M4 17 OK; M5 399 / 446 / 52; M7 gate PASS with
   --no-smoke). Open, as routed: carries 67 (RM), 68 (§U4e), 69 (§U4r), 70 (§U4g), 71 and 72; carry 52's last part
   (U4d2 pins only `Recording::GENERATOR` 2's recordings and names U4d-F's commit); NC2-L1 (u4df-report.md,
   follow-ups).
3. **The U4e brief: rev 3.3, approved** (advisor-u4e-brief.md: APPROVE on L1 to L4; L1 and L2 done at 3088bb5).
""".split("\n")
NEXT = """**Next steps, in order:**
1. **U4e's L3** (advisor-u4e-brief.md; carry 51's words): the main session's own commit on the merged head:
   `Scene::effect_rows()` (pub, the rows per soldier, 8 or 0 in today's scenes) and its line in u4-build-plan.md §4.1's
   npt-sim row ("`Scene::effect_rows` (an effect row's owner, the one public home: carry 51)"); then the brief's 7.2
   item 1 counted run gives `<base>`'s counts (every check.mjs step passing), filled into the brief. U4e's review reads
   its diff from the commit before the carry-51 commit (engine-architect: a new pub item, A19; rust-reviewer).
2. **L4, U4e's launch,** as the brief's section 1 says: one builder (general-purpose, opus, xhigh) in a worktree, under
   W1-TARGET's lock; at most three agents at a time, reviewers included.
3. **U4d2's builder brief,** with the advisor: carries 52, 65 and 66, N2-M2's ruling and W1-TARGET; it bases on U4d-F's
   commit and names it.
4. **U4r's brief:** the night's rev 2 at `cs/npt-u4r-brief` ed973c9 goes to the advisor after U4c, U4d2, U4e and U4j
   merge (carry 59's new sentence and carries 62, 63 and 69 to fold).""".split("\n")
for l in LAST + STEP + RUN + CUT + NEXT:
    assert len(l) <= 120, (len(l), l[:60])

moved = (["", "## Running work, U4d-F: the resume card's blocks at U4d-F's merge (moved 2026-11-12 about 01:55 UTC, by "
          "card_refresh4.py)", "From the card's header and \"The current step\" (as they were):", ""]
         + card[i_last:i_last + 2] + [""] + card[i_step:i_run]
         + ["", "From \"Running\" (as it was):", ""] + card[i_run:i_cut]
         + ["From \"The three cut workflows\" (as it was):", ""] + card[i_cut:i_batch]
         + ["From \"Next steps, in order\" (as it was):", ""] + card[i_next:i_done] + [""])
assert hist[-1] == "" and hist[-2].strip()
hist[-1:] = moved
card[i_wt + 1] = "  merged at 4d2b0ef), wf_9ce88fa3-452-1 (U4d-F, merged 2026-11-12)."
card[i_next:i_done] = NEXT
card[i_cut:i_batch] = CUT
card[i_run:i_cut] = RUN
card[i_step:i_run] = STEP
card[i_last:i_last + 2] = LAST
print(f"history.md: +{len(moved) - 1} lines; card: {len(card)} lines")
if APPLY:
    with open(HIST, "w", encoding="utf-8") as fh:
        fh.write("\n".join(hist))
    with open(CARD, "w", encoding="utf-8") as fh:
        fh.write("\n".join(card))
    print("APPLIED")
else:
    print("dry run: nothing written")
