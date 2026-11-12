# U4e round 2, engine-architect seat (read only)

Started 2026-11-12. Fixed copy: /home/user/Shamble-Frontlines/.claude/worktrees/wf_0fa6595a-3e2-1 (uncommitted on 97bb9d3).
Round 1's read copy: /home/user/Shamble-Frontlines/.gate/u4e-r1. Packet: u4e-r2-packet.md.

## Plan
1. Read R1-R8 and T1 (packet), T6 (advisor-u4e-r1.md), my round-1 notes (u4e-review-arch.md: A1, A2, A4, A5, list :57-66).
2. Check the copy matches fx-hashes-end.sha256 (sha256sum -c, read only).
3. A1: room take_control's type and body (bytes encoded once on the tick with the room's Quant; full_state's counted
   encode reused; JoinAccepted and Kick likewise); the lab's send in server/tests/common/mod.rs reads the channel from
   Limits::TYPES by the type byte and no longer encodes; the bites test and its plant.
4. A2: RttSource (pub, two variants, docs, where defined, how re-exported), Decoder::new's signature, the pump's rtt_ns
   choice; the two tests; e10's ping path passes Pings.
5. A4: the scripted-input field, fill_input, no clear after the step; tick_input()'s doc; the test.
6. A5: the eight items pub(crate); with_lead pub; with_cover's test (Rewind's cover Present).
7. T6: every pub item the fix pass adds or changes, from the diff (grep `pub ` in the + lines), against §4.1's rows and
   R8's one new item; anything beyond R8's scope and bounds.
8. Hidden coupling / boundary: who owns encoded bytes, double encode anywhere else, channel rule ownership.
9. Optional runs via the runner (prefix ea).
10. Verdict and findings.

## Notes (a line after each finding)
