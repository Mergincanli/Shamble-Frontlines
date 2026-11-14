"""Build U4r copy 6's prompt (the fix pass of review round 1) as u4r-copy6-prompt.md: the first launch prompt word for
word (the scratchpad's u4r-launch-prompt.txt, 3,189 characters, asserted equal to the start of u4r-copy5-prompt.md),
then the main session's resume part for copy 6, then the advisor's section "The fix pass, in order" copied word for
word from advisor-u4r-r1-full.md (lines 5-17, its heading and its first and last items asserted; the file's sha256
asserted). Nothing of the ruling is retyped.
Run from the repo root:  python3 -I .checkpoints/netcode-proof-test/u4r_copy6_prompt.py"""
import hashlib

N = "/home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test"
LAUNCH = ("/tmp/claude-0/-home-user-Shamble-Frontlines/077a208e-2c05-50f2-8333-d6ac6effb7f2/scratchpad/"
          "u4r-launch-prompt.txt")
RULING = f"{N}/advisor-u4r-r1-full.md"
RULING_SHA = "7b552e8c9d6fd16dbec050f326ce162fc9d46ba2279ca6f933edb5b6660b8e8d"
WT = "/home/user/Shamble-Frontlines/.claude/worktrees/agent-a11e2354169f39acf"


def sha(path):
    with open(path, "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


with open(LAUNCH, encoding="utf-8") as fh:
    launch = fh.read()
assert len(launch) == 3189, len(launch)
with open(f"{N}/u4r-copy5-prompt.md", encoding="utf-8") as fh:
    assert fh.read().startswith(launch + "\n\n**Resume: copy 5"), "the launch prompt differs from copy 5's start"
assert sha(RULING) == RULING_SHA, sha(RULING)
with open(RULING, encoding="utf-8") as fh:
    r = fh.read().split("\n")
assert r[4] == "## The fix pass, in order" and r[17] == "" and r[18] == "## Round 2", (r[4], r[17], r[18])
assert r[6].startswith("1. THE COPY AND ITS RULES") and r[7].startswith("2. FP-0 ") and r[16].startswith("11. FP-9 ")
fix = r[4:17]
runner = sha(f"{N}/u4r-run.sh")
assert runner == "6efb91572cf06fd9ad6472c077cd39c9a59bfed8d4a3dac561992aafd5729bc8", runner
resume = f"""**Resume: copy 6, the fix pass of review round 1 (the main session, 2026-11-14 about 11:30 UTC).** You are a fresh copy. Copies 1 to 4 each ended at its context limit; copy 5 finished the build (section 6's return list, 06:58 UTC). Review round 1 (six seats) then reviewed the build's snapshot, the advisor ruled on its findings, and Mustafa chose the fix pass as ruled (his answer SIZE-U4R (a), 2026-11-14). Nothing is committed. Where this part and the text above differ, this part wins for you (the brief still wins over both).
- **Your worktree already exists:** {WT} (branch worktree-agent-a11e2354169f39acf, HEAD 2fbec7780cd04feafd8a3c67e1ebdb8889ad45ef). Its uncommitted work is the reviewed snapshot, /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/wip/u4r.patch (26 files, sha256 572c4f69435da2ad6f62d54d4bcbc47b04d6329b5493e0faf78fcecfdc4a09c2); the main session checked at 11:20 UTC that the worktree's diff equals it. Your session's folder is the main folder, not the worktree: begin every Bash command with `cd {WT} && `, and give the Read, Edit and Write tools absolute paths inside the worktree. The runner and the logs folder /home/user/Shamble-Frontlines/.gate/u4r/ stay where they are; every runner call names the worktree. Never edit a file outside the worktree, and never edit anything in the main folder.
- **The runner changed** (the ruling's item 2, FP-0, made by the main session at about 11:15 UTC): its sha256 is now {runner}, in place of the 349b51ad... above. Its only change, three lines, is in /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/u4r-run.sh.inf-u4r-4.diff: free space is read in bytes (`df -B1`), the floor is exactly 4 GiB, and the log's second line reads `free <n> MiB`. Check the sha256 before its first use and read that diff. Record that your first runner log shows the MiB line: it is FP-0's last check.
- **The start checks, in place of the first copy's:** HEAD is 2fbec77...; the brief's sha256 is 59d570b8...; the runner's is {runner[:8]}...; `git apply --check -R /home/user/Shamble-Frontlines/.checkpoints/netcode-proof-test/wip/u4r.patch` exits 0 in the worktree; and `df -h /home/user`. Write them under a new heading "## Fix pass, round 1 (copy 6)" at the end of your notes (the worktree's .checkpoints/netcode-proof-test/u4r-build.md, 1,586 lines, equal to the main folder's copy). If any does not hold, stop and report.
- **Before you change anything** (Mustafa's rule: a fresh copy reads its notes and the diff in its worktree before it changes anything): read your notes whole (1,586 lines), then the diff: `git status --short`, `git diff`, and each untracked file under spikes/ and ops/. Read again the brief's sections your items need (at least 2.4, 4, 5, 6, 7.1 and 7.3) and the code they name.
- **The advisor's rulings bind you**, in the main folder's .checkpoints/netcode-proof-test/: advisor-u4r-build1-full.md to advisor-u4r-build4-full.md, as for copy 5, and new, **advisor-u4r-r1-full.md** (sha256 {RULING_SHA}), the ruling on round 1. Its section "The fix pass, in order" is copied below word for word: it is your work. Its section "Each finding" gives each item's reasons; round 1's findings themselves (Where, Wrong, Evidence, Fix) are in u4r-review-r1-full.md (sha256 ec61dfbd38fa7dbeaed208c6f4467580c5124cf97f75780af45aa6d8a4d6db58). Where a finding's own Fix and the ruling differ, the ruling wins. Keep a block "Rulings (advisor-u4r-r1)" in your notes with each item's state as you go.
- **Your work, in this order:** the ruling's items 1 and 3 to 11 (FP-1 to FP-9), in its order. Item 2 (FP-0) is the main session's and is done (above). FP-8's end checks come last, after every edit. One plant at a time: take the file's sha256 before the plant, plant by Edit, restore by Edit, check the sha256 equal, and never stop with a plant in place.
- **Disk:** 4,784 MiB (4.67 GiB) was free at 11:15 UTC. Keep section 4 rule 6's floors as item 1 says (df first; 4 GiB before any release or wasm32 build). If a run would go below a floor, stop and report: a part-clean is the main session's, between lanes. Delete nothing (Mustafa's LEFTOVER-DELETE).
- **A failure is a result:** a check.mjs test-step failure is recorded whole under TE2-5 and never retried (item 1); a SHA-256 mismatch in FP-8 stops you, with no re-pin and no retry (item 10). If any outcome differs from what the ruling expects, stop and report: never re-run a failed suite into a pass (condition 5).
- **Your context is finite:** each earlier copy used all of its own in about an hour. Save each finished item to the notes at once. When most of it is used, stop at a clean point (no plant in place, no half edit, no run going), write the state and next steps as the earlier copies did, and report.
- **Your report** must be on disk before round 2 starts (the ruling's "Round 2"): write it at the end of your notes under "## Fix pass, round 1: report", then send it as your final message. In it: each item's state with file:line; every new test named with its file, and the counts as deltas from 526/603/76; each plant with the test that failed by name and the restore's sha256 check; the two end checks' summary lines; FP-8's two PASS lines with each log's label; FP-9's report words, its items 1 to 5; and anything left undone, with why. If you stop partway: what this copy did, and where the notes' state section is.

**The advisor's fix pass, word for word** (advisor-u4r-r1-full.md, lines 5-17):

"""
prompt = launch + "\n\n" + resume + "\n".join(fix) + "\n"
assert prompt.startswith(launch) and prompt.endswith("\n".join(fix) + "\n")
out = f"{N}/u4r-copy6-prompt.md"
with open(out, "w", encoding="utf-8") as fh:
    fh.write(prompt)
print(f"{out}: {len(prompt)} characters, {prompt.count(chr(10))} lines, sha256 {sha(out)}; launch prompt "
      f"{len(launch)} characters checked equal; the ruling's lines 5-17 copied ({len(chr(10).join(fix))} characters)")
