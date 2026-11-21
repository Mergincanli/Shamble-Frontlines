"""Move the resume card's U4j blocks to history.md at U4j's merge (2026-11-11), by line range, and put the condensed
text in their place. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/card_refresh3.py [--apply]"""
import sys

CARD = ".checkpoints/netcode-proof-test.md"
HIST = ".checkpoints/netcode-proof-test/history.md"
APPLY = "--apply" in sys.argv
with open(CARD, encoding="utf-8") as fh:
    card = fh.read().split("\n")
with open(HIST, encoding="utf-8") as fh:
    hist = fh.read().split("\n")

assert card[32].startswith("1. **U4j, the judge and marks change** (task wgj6zbosy"), card[32][:50]
assert card[62].startswith("2. **The U4e builder brief, rev 3**"), card[62][:50]
assert card[103].startswith("1. U4j: round 2 done, no round 3"), card[103][:50]
assert card[111].startswith("2. The U4e brief:"), card[111][:50]
block, step = card[32:62], card[103:111]

NEW_BLOCK = """1. **U4j, the judge and marks change: MERGED** (4d2b0ef, the merge; d50396a, round 2's wording; pushed together;
   run wf_0906017a-a9b finished). Round 1 (three REVISE; F1 High), the fix pass (FX-1 to FX-14), round 2 (three
   APPROVE WITH FINDINGS; F1 closed by all three; no round 3), the advisor's round-2 ruling (advisor-u4j-r2.md:
   APPROVE on M1-M9), then the merge: M1 66 OK, M2 true, M3 64 files (the snapshot equal to 4ad80a9's), M4 64 OK, M5
   and M8.2 measured 396, bites 443, self 52, M6 no failure, M7 gate PASS with --no-smoke (this container's Chromium
   does not match tools/'s Playwright). The record: u4j-build.md, "Merge into the spike branch" and "Readings, as
   ruled". The old block: history.md, "Running work, U4 wave 1 (3)".
   - Open from U4j, as routed: carries 61 (§U4e), 62 and 63 (§U4r), 64 (the next change to crates/judge), 65
     (§U4d2, or the main session sooner); C-U6-9 to C-U6-11 (plan.md's U6 row); the FU list for PR #40's report (the
     closing checklist); TE2-5 (b) and U4c's L-c8 judged re-run (next steps); one more reading only if U4e's client
     drops stale groups (advisor-u4j-r2.md B5).""".split("\n")
NEW_STEP = """1. **U4j: MERGED** (4d2b0ef, d50396a; about 20:10 UTC), M1-M9 met. Next, M9's last part: **U4c's L-c8 judged re-run
   on U4j** (u4-build-plan.md §5 U4c, :591-593: -3's five L1 parts re-run on the merged build and re-judged PASS, its
   four bites failing as before, and for P2, B1 and P4 seed 1 the SHA-256 of the v1 tag plus the v2 log's datagram
   entries equal to U2's 913db8e9, feabf2f0 and b8a04f2f). Then **TE2-5 (b)** (advisor-u4j-r2.md B2.3), before the
   next merge into the spike branch: the bites build's workspace tests ten times (`cargo test --locked --workspace
   --all-features --no-fail-fast` through main-run.sh), each run's whole output under .gate/te2-5/; any failure's
   name and panic into the notes, tagged TE2-5; none in ten is recorded, and the rule stays.""".split("\n")
for l in NEW_BLOCK + NEW_STEP:
    assert len(l) <= 120, l[:60]

assert hist[-1] == "" and hist[-2].strip()
moved = (["", "## Running work, U4 wave 1 (3): the resume card's U4j blocks at U4j's merge (moved 2026-11-11 about "
          "20:15 UTC, by card_refresh3.py)", "From \"The three cut workflows\", item 1 (card lines 33-62):", ""]
         + block + ["", "From \"Next steps, in order\", item 1 (card lines 104-111):", ""] + step + [""])
hist[-1:] = moved
card[103:111] = NEW_STEP
card[32:62] = NEW_BLOCK
print(f"history.md: +{len(moved) - 1} lines; card: block {len(block)} -> {len(NEW_BLOCK)} lines, step {len(step)} -> "
      f"{len(NEW_STEP)} lines")
if APPLY:
    with open(HIST, "w", encoding="utf-8") as fh:
        fh.write("\n".join(hist))
    with open(CARD, "w", encoding="utf-8") as fh:
        fh.write("\n".join(card))
    print("APPLIED")
else:
    print("dry run: nothing written")
