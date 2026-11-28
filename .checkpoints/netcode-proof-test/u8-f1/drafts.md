# U8-F1 drafts, not in the tree (the builder stopped before 1c: item 1 with the pin test passes 41)
Each block is the exact text at its file's wrap width; list-draft.test.mjs splices them into the real files in memory.

## A. PROMPT-R1.md section 6 step 2 (replaces :121-123; items 1c, 1e and 2)
<!-- A start -->
2. `ssh -F <ssh> npt-rs r1.sh collect`; copy `/var/lib/npt/results/r1.tar.gz` into `.gate\npt-results-raw\` (scp) and
   check its SHA-256 against the one printed. Put only `collect/`, `probe/presence.txt`, `ds-selftest/ws_split.txt`,
   `bringup/facts.txt`, `s14/` whole and `s14-redact.txt` into `<results>\r1\` (`<results>` is
   `.gate\npt-results\results`). Before the push, the redaction check is the pre-push hook's own: the fixed copy
   `~/npt-operator/redact-check.mjs` with `--vms ~/npt-operator/vms.txt`, run on `<results>\r1\s14`, its whole output
   saved as `<results>\r1\s14-redact.txt` in UTF-8 (UTF-16 reads as binary, a finding), its last line in step 4's
   summary. A finding, or any exit but 0, means no push: stop and tell him. Just before the push, in PowerShell,
   `git -C <results> rev-parse --git-path hooks` must print the clone's own hooks folder, the one holding pre-push;
   anything else: stop and tell him. Then push to `claude/npt-results` after the pre-push hook passes (a prompt).
<!-- A end -->

## B. R1.md section 2 step 6, the Stop B start-check line (replaces :86-87; item 2)
<!-- B start -->
   Start check 4 is run once in each shell, Bash and PowerShell: each `git -C` command must prompt, not be denied
   (PowerShell's rules match case-insensitively, so its `git -c` deny names the hooksPath key: SEC-R2-3); and, in
   PowerShell, `git -C <results> rev-parse --git-path hooks` prints the clone's own hooks folder, prompting, not denied.
<!-- B end -->

## C. R1.md section 7, a new bullet after the first (items 1c and 1e; its last sentence is item 3)
<!-- C start -->
- R1's results push (PROMPT-R1.md section 6 step 2) carries only `collect/`, `probe/presence.txt`,
  `ds-selftest/ws_split.txt`, `bringup/facts.txt`, `s14/` whole and `s14-redact.txt` (the pre-push hook's own
  redaction check, run on the s14 folder first). The s14 folder holds only the kernel's release (uname -r), the
  net.ipv4 settings, netem's settings (`tc qdisc show`), the load, the TCP counters, the congestion control's name and
  any error setting it, the message, attempt and browser-leg times, the run standings and the two documentation
  addresses. Its writers: npt-s14's kernel.txt and server-, client- and bursts-run<N>.json; r1.sh's s14 block:
  summary.txt, INVALID-s14.txt, rerun/ (npt-s14's same files and its summary), rerun-R2.txt and browser-stamps.txt;
  and r1.sh's block start, `attempts`. A change that would put anything naming the server into it (a host name, a
  routed address, a hardware address) stops and goes to Mustafa first (ops/tests/s14.test.mjs pins it).
  At R1's report the advisor applies plan.md:1256's slack-value rule from presence.txt's printed reads (ADV-U8H-2).
<!-- C end -->

## D. r1plan.test.mjs (test 5's file), a new test at its end (item 1d's list check; its last assert is item 2's)
<!-- D start -->

test("ADV-U8H-1 (b), S14-PUSH (a): one pushed list, word for word, in PROMPT-R1.md section 6 and R1.md section 7", () => {
  const list = (body) => body.replace(/\s+/g, " ").match(/`collect\/`.*?`s14-redact\.txt`/)?.[0];
  assert.equal(list(text("runs/R1.md")), "`collect/`, `probe/presence.txt`, `ds-selftest/ws_split.txt`, `bringup/facts.txt`, `s14/` whole and `s14-redact.txt`");
  assert.equal(list(text("operator/PROMPT-R1.md")), list(text("runs/R1.md")), "the same list in both files");
  assert.ok(text("operator/PROMPT-R1.md").includes("`git -C <results> rev-parse --git-path hooks`"), "SR-1: the hooks re-read just before the push");
});
<!-- D end -->
