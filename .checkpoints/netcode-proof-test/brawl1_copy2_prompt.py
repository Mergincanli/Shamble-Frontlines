"""Build BRAWL-1 copy 2's prompt (the fix pass of review round 1) as brawl1-copy2-prompt.md, in u4r_copy6_prompt.py's
form: the first launch prompt word for word (the scratchpad's brawl1-launch-prompt.txt, 2,498 characters, copied by
script from agent a4be43b1f756e45dd's transcript; its sha256 asserted), then the main session's resume part for copy 2,
then the advisor's section "The fix pass, in order" copied word for word from advisor-brawl1-r1-full.md (lines 11-192;
its heading, its first and last steps and the next heading asserted; the file's sha256 asserted). Nothing of the ruling
is retyped. Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/brawl1_copy2_prompt.py"""
import hashlib

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
LAUNCH = ("/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/"
          "brawl1-launch-prompt.txt")
LAUNCH_SHA = "b3cb6f0e8d5b9b93b38aee5ec39fa475d58fab1a0fcde670c2b81855790079f1"
RULING = f"{N}/advisor-brawl1-r1-full.md"
RULING_SHA = "86bec63bfffbd676efbc09e09c35a952e63abc7865e89b20b5aebec11bee75c5"
RUNNER_SHA = "2f754d125b10b6bde06a980f46cb4e078a582ca277ec8828bc48e8aacb064e50"
WT = "/home/user/Shamble-Frontlines/.claude/worktrees/brawl1"
SNAP = "ee2973aa8ad8c453b5720099c27b37cb9980820da74f33ad9c78ddd7ad591dc7"
OUT = f"{N}/brawl1-copy2-prompt.md"
T4 = ("Write files with the Write and Edit tools only (no sed, no shell redirect, no python writes; never `cargo fmt` "
      "writing files: apply rustfmt's diff by Edit); an agent with no Edit tool rewrites its notes file whole with "
      "Write.")


def sha(path):
    with open(path, "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


assert sha(LAUNCH) == LAUNCH_SHA, sha(LAUNCH)
with open(LAUNCH, encoding="utf-8") as fh:
    launch = fh.read()
assert len(launch) == 2498, len(launch)
assert sha(RULING) == RULING_SHA, sha(RULING)
assert sha(f"{N}/u4r-run.sh") == RUNNER_SHA, sha(f"{N}/u4r-run.sh")
with open(RULING, encoding="utf-8") as fh:
    r = fh.read().split("\n")
assert r[10] == "## The fix pass, in order" and r[192] == "" and r[193] == "## The pins", (r[10], r[192], r[193])
assert r[12].startswith("### Step 1: 1. THE COPY AND ITS RULES") and r[182].startswith("### Step 11: 11. FP-9:")
fix = r[10:192]
resume = f"""**Resume: copy 2, the fix pass of review round 1 (the main session, 2026-11-15 about 06:45 UTC).** You are a fresh copy. Copy 1 built BRAWL-1 and reported at about 04:10 UTC; its report is the end of your notes. Review round 1 (four seats) then reviewed the build's snapshot. The advisor ruled on copy 1's points (advisor-brawl1-points-full.md: no diff change) and on the round (advisor-brawl1-r1-full.md, sha256 {RULING_SHA}). Nothing is committed. Where this part and the text above differ, this part wins for you; where the advisor's ruling gives exact words, they win over both.
- **Your worktree already exists:** {WT} (detached HEAD 352a245c3018c50843f62dcb90e91973ea56d9da). Its uncommitted work is the reviewed snapshot, the main folder's .checkpoints/netcode-proof-test/wip/brawl1.patch (10 files, sha256 {SNAP}); the main session checked at 06:40 UTC that `git diff HEAD | sha256sum` in the worktree gives that value. Your session's folder is the main folder, not the worktree: run each worktree command in a subshell, `( cd {WT} && ... )`, so your shell stays in the main folder, and give the Read, Edit and Write tools absolute paths inside the worktree. Your notes stay where copy 1 kept them, in the main folder: {N}/brawl1-build.md (388 lines). Never edit any other file outside the worktree.
- **The runner changed** (the ruling's step 2, FP-0, made by the main session and committed at 7402dac): {N}/u4r-run.sh is now sha256 {RUNNER_SHA}, in place of the 6efb9157... above. Its only change, one line, is in u4r-run.sh.te-b1-7.diff beside it: each log's header now also reads `diff <sha256>; untracked <sha256>`. Check the runner's sha256 before its first use and read that diff. Your first runner log, before any edit, must show `diff {SNAP}`: record that it does (FP-0's evidence).
- **The start checks, in place of copy 1's:** HEAD is 352a245c...; the brief's sha256 is 921c3243958a09dd5f914fb7eb247f07333558d0905b3d849d2f13222d6d2a50; the runner's is {RUNNER_SHA[:8]}...; `git diff HEAD | sha256sum` in the worktree is {SNAP[:8]}... (the ruling's step 1: if it is not, stop and report); and `df -h /home/user`. Write them under a new heading "## Fix pass, round 1 (copy 2)" at the end of your notes. If any does not hold, stop and report.
- **Before you change anything** (Mustafa's rule: a fresh copy reads its notes and the diff in its worktree before it changes anything): read your notes whole, then the diff (`git status --short`, `git diff`). Read again the brief's section 4 (its rules hold throughout: the ruling's step 1) and the code each step names.
- **The advisor's rulings bind you**, in the main folder's .checkpoints/netcode-proof-test/: advisor-brawl1-plan-full.md (conditions 1-18), advisor-brawl1-points-full.md, and new, **advisor-brawl1-r1-full.md** (sha256 {RULING_SHA}, 249 lines). Its section "The fix pass, in order" is copied below word for word: it is your work. Its sections "The pins", "The size" and "Each finding" give the reasons; round 1's findings themselves (Where, Wrong, Evidence, Fix) are in brawl1-review-r1-full.md (sha256 fc979c632a7681885c7c298af1d99b1e389533b21d19c7a41ad3d0e041973305). Where a finding's own Fix and the ruling differ, the ruling wins. Keep a block "Rulings (advisor-brawl1-r1)" in your notes with each item's state as you go.
- **Your work, in this order:** the ruling's steps 3 to 11 (FP-1 to FP-9), in its order. Step 2 (FP-0) is the main session's and is done (above). Step 1's rules hold throughout. FP-8's end checks come last, after every edit. One plant at a time (FP-7): take the file's sha256 before the plant, plant by Edit, restore by Edit, check the sha256 equal, and never stop with a plant in place.
- **Disk:** 6,566 MiB was free at 06:40 UTC (the shared target 6,442 MiB). Keep section 4 rule 6's floors: df first; 4 GiB before any release or wasm32 build, 2 GiB before any other. A floor stop is reported and never cleaned around. Delete nothing.
- **Stops** (each is reported whole and never retried): a check.mjs test-step failure (TE2-5); a SHA-256 mismatch in det-u4.mjs or the precheck regeneration (with the first differing tick); any change to a figure the full sim-only test printed in round 1 (FP-8 (4)); a non-cargo sum over 240 s; a plant among P2 to P4 that does not bite as ruled (that item stops, the rest goes on); P1 still not biting after the door-entry bar (that item stops); numstat past 1,400 changed lines against 352a245, SERIES.md included (stop: the main session tells Mustafa in three lines). Never re-run a failed suite into a pass.
- **Cuts:** the session's spend limit or a restart may cut you mid-run. Save each finished item to your notes at once. When most of your context is used, stop at a clean point (no plant in place, no half edit, no run going), write a "## State" block with the next steps, and report.
- **Your report** must be on disk before round 2 starts: write it at the end of your notes under "## Fix pass, round 1: report", then send it as your final message. In it: each step's state with file:line; every new test named with its file, and the counts as deltas from 535/613; each plant P1 to P4 with its three sha256 values, its log, each command's exit code and the failing assert's message (P1: per seed and pattern, in_building, first, and the largest distance with its slot, tick, x and y; and whether the door-entry bar was added); FP-8's results (check.mjs's summary lines, det-u4.mjs's PASS line with both pins and the non-cargo sum, the regeneration line, the short copy's times and dead figures, the full test's figures against round 1's, the two manifests' sha256, the size by numstat); FP-9's items 1 to 6; and anything left undone, with why.
- {T4}

**The advisor's fix pass, word for word** (advisor-brawl1-r1-full.md, lines 11-192):

"""
prompt = launch + "\n\n" + resume + "\n".join(fix) + "\n"
assert prompt.startswith(launch) and "\n".join(fix) in prompt
with open(OUT, "w", encoding="utf-8") as fh:
    fh.write(prompt)
print(f"{OUT}: {prompt.count(chr(10))} lines, {len(prompt)} characters, sha256 {sha(OUT)}")
