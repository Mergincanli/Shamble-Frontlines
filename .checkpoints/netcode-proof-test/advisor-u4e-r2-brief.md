# The advisor's ruling on U4e's round 2: the brief (the main session, 2026-11-12 about 16:40 UTC)

Kept so a restarted advisor can take the same brief: if advisor-u4e-r2.md is incomplete and no notice came back, start
after its last complete block.

**Your role:** read `cd /home/user/Shamble-Frontlines && git show origin/main:.claude/agents/advisor.md` first and work
as it says. You rule on round 2 of U4e under your round-1 ruling's outcome rule (advisor-u4e-r1.md, R8, :431-434) and
CLAUDE.md's review budget. Write your ruling to `.checkpoints/netcode-proof-test/advisor-u4e-r2.md`, one block at a
time as you finish each, so that a cut run leaves every finished block on disk. Write files with the Write and Edit
tools only (no sed, no shell redirect, no python writes; never `cargo fmt` writing files: apply rustfmt's diff by
Edit); an agent with no Edit tool rewrites its notes file whole with Write. Run nothing that builds or touches a copy;
reading, git and sha256 are fine.

**What round 2 is:** the review on what changed, the fix pass FX-1 to FX-13 your R8 set, by the five seats R8 named,
in batches of at most two (the U4d2 brief check's L5): netcode-architect and security-auditor, then engine-architect
and rust-reviewer (read only), then the test-engineer alone (plants by Edit in the copy, one counted check.mjs).

**Evidence (read whole; all paths under /home/user/Shamble-Frontlines):**
- The packet every seat started from: `.checkpoints/netcode-proof-test/u4e-r2-packet.md` (sha256 ecf4394a…; cut by
  review_packet.py from u4e-r2-packet.json: round 1's tree `.gate/u4e-r1` against the fixed copy, your R1 to R8 and T1,
  the carries it holds and the ids of those it left out). It was cut before today's change to review_packet.py's name
  check (below) and stays as it is.
- The five seats' notes, each APPROVE WITH FINDINGS:
  - netcode-architect `u4e-review-r2-netcode.md`: NC2-M1 Medium (not before the merge); NC2-L1 Low;
  - security-auditor `u4e-review-r2-security.md`: SEC-U4E-R2-1 to SEC-U4E-R2-4 Low;
  - engine-architect `u4e-review-r2-arch.md`: A-R2-1 and A-R2-2 Low;
  - rust-reviewer `u4e-review-r2-rust.md`: RR2-1 to RR2-3 Low;
  - test-engineer `u4e-review-r2-test.md`: TE2-L1 and TE2-L2 Low. Every FX's test fails by name under its plant
    (logs `.gate/u4e/r2-te-*.log`); its own edge plants too. Its counted check.mjs run
    (`.gate/u4e/r2-te-check-counted.log`): all 10 steps PASS, measured 473, bites 541, self 52 of 52, the fix pass's
    counts. Its one deviation: that run in the background with a 1,500,000 ms limit, not 600,000 ms (its round-1 run
    took 590 s); the runner, the lock and the command were the brief's.
- The copy: `.claude/worktrees/wf_0fa6595a-3e2-1` (97bb9d3 plus the uncommitted fix). After round 2 the main session
  checked its 23 files against `.gate/u4e/fx-hashes-end.sha256`: 23 OK (no plant left in place). Its snapshot on the
  spike branch: `wip/u4e.patch` (4300944). The builder's record (its claims, not evidence): the copy's
  `.checkpoints/netcode-proof-test/u4e-build.md`, "Fix pass, round 1".

**What changed around it since your round-1 ruling:**
- Main merged into the spike branch at bfda3cd (PRs #50 and #51: the legacy game into legacy/; the gate files and
  CLAUDE.md as main's; CI green on bfda3cd). spikes/ did not move: `git diff --quiet 97bb9d3 HEAD -- spikes/` is true at
  ab48790, so R9's "Checked now" premise holds.
- U4d2's build runs beside U4e (your U4d2 brief check, L5), in `.claude/worktrees/u4d2-build` at 51520b4 on `<base>`
  bfda3cd; its brief's 1.2 sentence covers U4e merging first.
- u4-build-plan.md moved at 85c08df (3f58ff0d…; your U4d2 brief check's L2): only lines below §4.11 moved and §5
  U4d2's and the Lanes' text changed; §4.1 (T6's rows) did not move. u4s-carries.md is unchanged (cdab0240…).
- Mustafa's words of 2026-11-12 (history.md, the entry under "## Decision: Mustafa, 2026-11-12 about 15:32 UTC" at
  its end): review_packet.py lets pass only the exact paths on its ALLOWED list (advisor-u4e-brief.md alone; a name is
  added only for a file the advisor agent wrote as its ruling, each addition noted there), and "The packets already cut
  stay as they are. Do not re-cut them and do not restart a review." This ruling will be the first file a later packet
  may need; you need not do anything for that.

**What I ask you to rule on:**
1. **NC2-M1 (Medium):** a follow-up or a carry (R8), with its words and its home if a carry. The netcode seat's route:
   a carry with C-U6-12 and the S20 and -19 run briefs (the trap injector spaces each trap at least
   `FULL_STATE_ACK_TICKS` plus the rejoin after the client's last full state, or the limit's form is revisited before
   those runs), the client's resync path re-sends after the window, and the catalogue's Resync row
   (docs/design/netcode-proof-messages.md:90) gains the limit and `resyncs_limited`. SEC-U4E-R2-2 (Low) names the same
   row; SEC-U4E-R2-1 (Low) is the join-replacement edge of the same limit.
2. **The Lows** (none marked before the merge): under R8 each becomes a follow-up the main session records. Rule
   whether any belongs in a carry text instead, above all RR2-3 (T1's marks/README.md §1 sentence says "once" while the
   client remembers 32 bad-header ticks; its route: one line in U4r's carry 73 or the report) and TE2-L1 with NC2-L1
   (case 3's and case 2's forms untested; route: an endpoint test at the next npt-client edit, U4f's carry 80, or U4r's
   contract test). T1 to T6 are your words, copied by script in the merge commit: if any of them changes, give its new
   words in full.
3. **The merge:** confirm R9's M1 to M9 and T1 to T6 stand as written after round 2 and today's main merge, or say
   what changes (the expected counts after the apply, from the fix pass and round 2's counted run: 473 / 541 / 52 of
   52, 10 steps; the gate now runs main's 2.1b layout).
4. **The test seat's deviation** (the counted run's 1,500,000 ms limit): accept it or not.

For each item give APPROVE, REVISE or ESCALATE where one fits, the words wherever a text is to be written, and at the
end two lists: what must be done before the merge, and what goes to Mustafa (if anything), with the reason.
