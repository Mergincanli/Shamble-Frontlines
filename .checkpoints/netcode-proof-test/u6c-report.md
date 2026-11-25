# U6c's report: the probe, the load, the page host and the certificate (2026-11-25)
Brief u6c-brief.md rev 1.1 (8f56f37a…; C1-C9). Base 8c81952 (U6b's merge plus Block 0 at 3b75d00; measured 807, bites
904). Builder from 2026-11-24 06:56 UTC in its own worktree. Round 1 (netcode REVISE on High NC-U6C-1; test 5 Medium;
security 5 Low), the ruling advisor-u6c-r1.md (ad72c0b8…: the fix pass FP-1 to FP-6), the fix pass (u6c-build.md
a9d45dd4…, 13:50 UTC), round 2 on the fix diff (netcode, test and security: each APPROVE WITH FINDINGS, no Critical or
High), the round-2 ruling advisor-u6c-r2.md (**APPROVE**: H2 before the counted run, H1 on Mustafa's yes), his
NPT-EDGE-READ (a) of 2026-11-25 00:07 UTC (history.md:7902-7924), the advisor's head read of the hand edits
advisor-u6c-m4b.md (**APPROVE WITH CONDITIONS C1-C6**), and the merge in the main folder (M0-M9, U4f's form).

## What was built, and its size
- **The probe** (crates/net/src/l2/probe.rs, crates/server/src/probe.rs, npt-load's probe mode): each leg sends
  1,000,000 probe datagrams a direction between the load machine and the server, through ImpairSocket, and logs every
  fate with its stamps, the leg's base round trip, the receive drops at the leg's two edges and the CPU's steal. It is
  what R1 reads for ImpairSocket's self-test and -3's L2 parts.
- **The load** (crates/bots/src/load.rs, bin/npt-load.rs): up to 48 bot clients over WebTransport or WebSocket against
  the room, READY when every client is in, per-second counts per client, DONE at the end.
- **The page host and the certificate** (crates/host: issue.rs, serve.rs): the certificate order and the static file
  server R1's browser clients load from.
- **SERIES.md:** 51 added lines (33 table rows) for the new series.
- **Size:** 24 files, 5,698 added, 199 removed (`git diff --cached --shortstat` at M8): the fix pass's 5,693 added
  (u6c-build.md, "SIZE after FP-6") plus the hand edits' net +5. Under his size stop of 6,000 added lines.
- **No change to marks.json or the judge** (Mustafa, 2026-11-15, history.md:2127): `git diff --stat 8c81952 --
  spikes/netcode-proof/marks/marks.json spikes/netcode-proof/crates/judge` printed nothing on the merged tree (M7's D10
  read). So his rule's reviewer's-read line has nothing to cover here.

## The merged tree differs from the one round 2 reviewed
The merged tree differs from the one round 2 reviewed by H2 and H1, reviewed by the advisor's head read only.
- **H2** (TE-U6C-R2-4, Medium, fixed before the counted run): crates/bots/tests/load.rs, every measured second but the
  first and the last (`&measured[1..measured.len() - 1]`), so the WebTransport four-client test no longer counts the
  second that starts after the room's close.
- **H1** (NPT-EDGE-READ (a), his yes): crates/server/src/probe.rs, each leg's start-edge read of the receive drops
  (`leg_edge`, `cpu_stat`) moved from just after the pre-touch to the leg's start; the module doc and SERIES.md:915
  follow. In the advisor's words (advisor-u6c-m4b.md): the read now lands just after the leg's zero, normally under a
  millisecond by reading, not measured. His condition "The 10 seconds of warm-up and the measured window stay fully
  counted" holds for the whole measured window and the warm-up after that first moment. ADV-M4B-1 (Low) names the
  first moment; NPT-EDGE-SLIVER goes to him (below). ADV-M4B-2 (Low, wording): "at the leg's zero" should read "just
  after the leg's zero" in the module doc, the H1c comment and SERIES.md:915.
- The hunks file .gate/u6c-merge/hunks.diff, sha256 9a3bbeb7faec8aa928460fe34643c971cd4dd5e84d4f5f30d372f3758407b846:
  load.rs +4 -3, probe.rs +7 -3, SERIES.md +1 -1; rustfmt changed nothing (m4a-fmt.log). The other 21 files are
  byte-equal to the builder's worktree (cmp 24 of 24 at M4; the advisor's own `diff -rq` at M4b).
- His sentence on the drops, word for word: "The 214 drops inside P1's window on your own machine stay recorded as
  unexplained; this answer does not cover them."

## The merge's evidence (M0-M9)
- **M0:** wip/u6c-fix.patch 0549389e…; `git diff --stat 8c81952 HEAD -- spikes/netcode-proof` empty on the
  post-MAIN-65 head; the two A2 folders copied without their `tokens` file to .gate/u6c/a2-logs/.
- **M3-M4:** `git apply --check` exit 0, then `git apply` uncommitted; 24 files cmp-equal to the worktree.
- **M4a:** H2 and H1 by Edit, each with the ruling's section 3 text; `cargo fmt --all -- --check` exit 0.
- **M4b:** the advisor's head read, APPROVE WITH CONDITIONS C1-C6; its answer 1 allowed a compile-only probe.
- **The probe** (.gate/u6c-merge/m4c-probe.log, 04:22:50-04:25:05 UTC): `check.mjs --only fmt,clippy,doc`, all 3
  PASS; its header carried C1's hashes; it used 513 MiB.
- **M5, the one counted run** (.gate/u6c-merge/m5-check.log, 04:27:42-04:48:04 UTC, `env CARGO_INCREMENTAL=0 node
  check.mjs` through main-run.sh, started only after MAIN-65's reruns and Builder 2's plan check had ended): **all 11
  steps PASS**; test 1,026.3 s: measured **861** passed, bites **958** passed, 0 failed, 0 reported a skip (807 + 54
  and 904 + 54); wasm 27,000 ticks compared, 0 mismatching (both pairs); refs 1,856 tracked files; marks 81, rows
  unchanged; self 78 of 78; redact 371 items, 0 findings. Load average 0.31 before, 2.18 after. Free 9,728 MiB at
  its start.
- **Tree hashes:** the fix-check run's pair (fix-check.log) was diff 0e3199ae… and untracked 22bda910…; M5's pair is
  diff 6c3b4dbbd5e9c9fafda5ee316eb16ce4b61de12e8db1bf5aa14483b141711ec2 and untracked
  5cb9a30097a402787e5d685bf70cb97ee816709eaeaac411f16ea7206006f33a (C1's, as in m4a-fmt.log:3). They differ by the
  hunks only, and the hashes cover spikes/netcode-proof alone (main-run.sh:41).
- **M7:** `node tools/gate.mjs --no-smoke` GATE PASS against the merge-base with main (28254a7), symbols +0 -0.
- **M8:** this commit: the 24 files, `git rm` of wip/u6c-build.patch and wip/u6c-fix.patch, u6c-build.md's merge
  section, this report and the history entry. **C9** (U8's snapshot) and **M9** (the push, the saver, CI) follow.

## The round-2 ruling, item by item (advisor-u6c-r2.md section 2)
- **Item 1 (a), the start-edge drop read:** fixed by H1 on his yes, as above.
- **Item 1 (b), npt-load ends clean with no READY:** carried after R1 (Medium; follow-up below). At R1 it gives no
  wrong verdict: U8's `load_ready` waits for READY and writes INVALID-ready without it. C9 adds R1.md's line.
- **Item 2, TE-U6C-R2-4:** fixed by H2 before the counted run, as above.
- **Item 3, FP-3's wording:** closed in substance; no plant before the merge. The builder's words (u6c-build.md, the
  fix pass's plants): P2 in shipped code "FAILS `u6c_one_probe_on_loopback…`, but at :358, `base_rtt_ns` 0 not
  525,313 (the kept socket's header is P0's), before FP-3's new assert is reached (plant-p2b.log). F-P2-HEADER: the
  e2e test already caught a carried socket by its header; FP-3's assert is a second guard, which this plant did not
  reach." So FP-3's assert is not shown to bite; F-P2's defect is caught by a test. TE-U6C-R2-2 is the follow-up.
- **Item 4, slot 41's late commands over WebTransport:** after R1 (U6d or U6e, with F-WT-CLOSE). Reported facts, read
  from the left-behind A2 logs: the room's per-client outbound queue (`second.s.out_queue_count`) was 1 for all 48
  clients in every second 0-70, slot 41 included, so the room never held sends back; the room logs no per-client
  inbound lateness, so the look cannot say whether slot 41's commands reached it late; `wt_accept` shows 48 accepted,
  0 failed handshakes, and 225 client datagrams dropped at a full session inbox, not split by client, written right
  after the room's last tick, so not from LG's 30 s tail; when they fell is not in the log. Per second (TE-U6C-R2-7):
  slot 23 sent 0 in second 0, and slot 41 ran about 1 s longer than the rest at the end. No R1 verdict depends on it;
  R1's 48-client WebTransport readings name slot 41's pattern as a known open.
- **Item 5, the in-window drops:** U8 compares exactly C-U8-6 (plan.md:1256): a drop on P1's socket between the leg's
  two reads makes ImpairSocket's self-test INVALID, cause named, to the advisor before any L2 mark counts. U8's code
  already does this; C9 brings its prose back to plan.md:1256's words and adds per-leg report lines (the rise, the up
  span's excess, their difference), reported and never judged. On this container the rule tripped: P1 rose 222, 214 of
  them in the window as two events (about 122 at measured seconds 39-40, about 93 at 57-59), with the up release side
  26-38 ms late in those seconds; the cause is not established.

## The A2 prints, corrected (NC-U6C-R2-1, TE-U6C-R2-7)
- A2's printed "inputs a client a second" minimum over WebTransport is 0 by construction: it spans the READY second and
  the 29 seconds after the room's close. The whole seconds while the room ran (1-69) give 22-31 inputs a second for all
  48 clients.
- A2's WebTransport reading prints `ws_split` as Some(all zeros), not None as the notes once said.

## Q-FP6-1 (NC-U6C-5's footer half, a design choice inside the brief)
The builder's options: (a) an event before every footer of LG's log carrying `handshake_failed_count` (a new name and
SERIES.md row), or (b) the failing line only, as built. Taken: (b) stands for R1, since no mark reads the count and R1
reads INVALID-ready; (a) rides item 1 (b)'s follow-up, which changes the same function and adds the same test. Named
in the day's message as a choice taken without him.

## Follow-ups (STATUS), each with its taker
- **U6C-1B** (Medium, item 1 (b)): `Ended::Stopped` when `over && !ready`, with an integration test (a wrong `pin_hex`:
  exit 1 and the line) and its plant; with Q-FP6-1 (a). Taker: the next unit that changes crates/bots/src/load.rs.
- **NC-U6C-R2-1** (Low): print A2's min and max over the whole seconds while the room ran. Taker: the next change to
  crates/bots/tests/load.rs.
- **NC-U6C-R2-2** (Low): `leg_series` has no end bound; bound the candidates by the loop's end, or count unattached
  series seconds. Taker: the next change-set that edits crates/net/src/l2/probe.rs or crates/server/src/probe.rs.
- **NC-U6C-R2-3** (Low): three SERIES.md row wordings (`probe_down_unindexed_count`, `probe_peer_refused_count`, the
  send and receive error rows). Taker: the next change-set that edits marks/SERIES.md.
- **TE-U6C-R2-1** (Low): the t_in sort and the tie clause are pinned by no test. Taker: the next change to
  crates/net/src/l2/probe_tests.rs.
- **TE-U6C-R2-2** (Low): the variant plant P2c (P2b plus `base` written into P1's header). Taker: the next change to
  crates/server's probe tests.
- **TE-U6C-R2-3** (Low): SEC-U6C-4's decode clause has no test (a junk socket in the e2e test). Taker: as TE-U6C-R2-2.
- **TE-U6C-R2-5** (Low): the WebSocket arm's `whole` is empty unless READY lands in LG's first second. Taker: as
  NC-U6C-R2-1.
- **TE-U6C-R2-6** (Low): `>= 5_000` has no upper bound; `(5_000..=10_000).contains(&n)`. Taker: as NC-U6C-R2-1.
- **TE-U6C-R2-7** (Low): its notes half is corrected above; its print half rides NC-U6C-R2-1.
- **TE-U6C-R2-8** (Low): FP-1's Measured extension and `probe_window_ns` have no test. Taker: as TE-U6C-R2-2.
- **SEC-U6C-R2-1** (Low, round 1's SEC-U6C-4 residual): the decode gate stops junk, not a forged probe datagram; a
  per-run tag or a fixed LG source port. Taker: the next change to the probe's wire, before any untrusted network.
- **SEC-U6C-R2-2** (Low): the pin's docs and wait_peer's error still say "first datagram". Taker: the next change-set
  that edits crates/server/src/probe.rs.
- **ADV-M4B-1** (Low): closes if he keeps NPT-EDGE-SLIVER (a); else (b)'s change takes it.
- **ADV-M4B-2** (Low, wording): rides (b) if he picks it; else the next change-set that edits probe.rs or SERIES.md.
- **The stamp tests** (Block 2, held back by C4's lever (1)): STAMP-HOME (a), his answer: this chat builds them after
  R1's run and before R1's report.
- The two security Lows are open security findings: they keep PR #40 needs-mustafa at its review (advisor.md, PR
  review step 4).

## What was not verified
- H1's effect is reasoned from the code's order, not measured; the gap after the leg's zero is unmeasured, and the
  28 ms stall figure is carried from the round-2 ruling.
- The cause of P1's two in-window drop events on this container.
- When the 225 dropped datagrams fell, and from which clients; slot 41's cause.
- Nothing here ran on a real network: R1 is the first.

## Decisions needed
- **NPT-EDGE-SLIVER: is "just after the leg starts" close enough to your "fully counted"?** Each probe leg's first
  reading of dropped packets now comes a moment after the leg starts (normally under a millisecond by reading, not
  measured). Why it matters: drops from a stall that starts in that moment are missed only if the stall also holds the
  reading back by about 28 ms. Options: (a) keep it as merged; (b) a small follow-up before R1 that reads just before
  the leg starts, with its own plan, check and one counted run, picked by the end of Sat 28 November (Amsterdam).
  Recommendation: (a). The full question: q-npt-edge-sliver.md.

## For Mustafa
1. Answer NPT-EDGE-SLIVER only if you want (b).
2. Nothing to check in Brave: nothing a player sees, hears or feels changes.
