# For Mustafa, from Builder V3: change-set 3b needs your browser check and one decision (2026-11-25)

Change-set 3b ("The loop closes") is built, reviewed twice by six reviewers (all PASS), passed by the verifier, and
its code is approved by the advisor agent. The report: `docs/reports/2026-11-25-cs3b-the-loop-closes.md`.

## 1. Your browser check (about 20 minutes; Brave first, then Firefox once)
The zip `shamble-v2-03b-the-loop-closes.zip` is in the builder chat (sent with the chat's file tool, never
committed): 259,212 bytes, sha256 3fdcf9f6a1173a707cfc4f1f8620482693ba36abc6e708322b6d3628a4071f82, built from
commit 8f835d114 (its code is the approved 6abf3d2a). Its README.txt has the ten steps. In short:
1. Unzip, run `node serve.mjs` in the folder, open http://127.0.0.1:4173 in Brave, window in front. You should see
   the grey ground, a row of six boxes (W, A, S, D, Space, Shift) at the bottom left, "errors: 0" and the commit.
2. Hold W: its box lights a moment later and stays lit; let go: dark.
3. Press S, D, left Shift, right Shift, one after the other: each lights only its own box.
4. Tap Space ten times: the box flashes each time; "presses" ends with the same number made and echoed, "0 lost"
   and "0 doubled".
5. Hold D, click on another window, let go of D there, click back: D is dark.
6. Hold A, switch to another tab for 30 seconds, come back without touching the keyboard: A is dark, "last pause"
   about 30 s, "ticks while hidden" 0 or very small.
7. Cover Brave's window completely with another window for 1 minute, then bring it back.
8. Leave it 2 minutes in front, press "Copy report", paste the result into the builder chat. These should all be
   0: messages.refused, olderPosts, wrongKeys, presses.lost, presses.doubled, releases.doubled.
9. In Firefox: steps 2 to 5 once, wait 2 minutes, "Copy report", paste it.
10. Ctrl+C in PowerShell stops the server.
Anything else you see (a box that stays lit or never lights, a count above 0, a line of text over the picture) is
what the check is for: paste what you see.

## 2. One decision: RELEASE-COUNT (how releases are counted for the presses mark)
The presses mark's row (ADR 0002, PM-0002-12) says the Worker counts presses and releases "through the edge
counters". The two drafts disagree: ADR 0002 part 5 has the post carry press and release edge counters; ADR 0001
part 5 defines press counters only, and that is what is built. Releases are worked out on the page from the
presses and the held keys, with two checks (a button held again with no press; each tick's held keys against the
post that tick used). Why it matters: the report cannot call the release half met by the mark's own method. It
does not block the merge (the advisor agent's ruling).
- (a) Accept as built, and correct the row's sentence when the drafts are next edited.
- (b) Add a release counter per button to the messages: 16 bytes more per command, up to 128 more per input
  message until delta coding, about 150 to 200 lines, in change-set 3c.
- The builder recommends (a): the releases follow exactly from the held keys and the press counters.

## 3. After that
One last commit (your check's result in the report, the notes folder removed), the advisor agent's APPROVE of that
exact commit, GitHub's checks green on it; then the chat gives you the commit's full hash, your main Advisor opens
the pull request from the title and text the chat hands over, and you press Ready and Merge. Your go accepts as
still open: the bound on how late a command may run (deferred to change-set 7; no room on a network without it);
OVERFLOW-PANIC-RULE (the code keeps the rule; CI runs no lint step until the gate step 2.3); two limits of the
press pacing, each your decision when it comes due.
