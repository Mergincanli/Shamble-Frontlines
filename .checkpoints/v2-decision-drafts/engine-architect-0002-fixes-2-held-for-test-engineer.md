# engine-architect's 0002 fix round 2: one block held for test-engineer (2026-11-01, the session)

The block below edits a note inside ADR 0002's "### Pass marks" section (line 1084), which is test-engineer's text
(the fix2 prompt: rows of a pass-mark table are test-engineer's). The session holds it and sends it to test-engineer
with the PM-0002-3 reading engine-architect returned. The other 24 blocks were applied.

engine-architect's return, word for word: "PM-0002-3 must then read: the same number of Wasm calls per tick at N = 12
and N = 48, with wasm-bindgen's glue calls included and no separate reading. At most 4 for the online client and for a
local match in one instance, at most 8 in two instances. Cite R1-BND-REC1 mark 2 as written (r1 2261–2263)."

### FIX F-2 (note under the pass-mark table)
FILE: docs/adr/0002-code-structure-build-toolchain.md
BEFORE:
- **PM-0002-3, "at most 4".** The draft's own per-tick list for the online client (the command in; the tick's
  received datagrams in one call; the step; the outgoing datagrams in one call; the block's pointer and length) is 5
  or 6 calls unless inputs are batched into one call, and wasm-bindgen's glue makes its own allocator call whenever
  bytes are passed in. The row gates control-API calls at 4 and reports the glue's calls; the drafter confirms the
  reading, or the design batches its inputs.
AFTER:
- **PM-0002-3, "at most 4".** The draft's first per-tick list for the online client (the command in; the tick's
  received datagrams in one call; the step; the outgoing datagrams in one call; the block's pointer and length) came
  to 5 or 6 calls, and wasm-bindgen's glue makes its own allocator call whenever bytes are passed in. Part 5 now
  batches the command and the received datagrams into one input call, and the step returns where the outgoing
  datagrams and the block lie.
END
