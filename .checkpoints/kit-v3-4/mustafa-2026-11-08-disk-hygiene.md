# Mustafa's disk-hygiene item, 2026-11-08, word for word

Source: his chat message in the day session (session_0173vVo7FcYUpbWgRW8oQyHU), 2026-11-08 about 08:35 UTC, while
A4 (PR #44) waited on his TIER0-SCOPE and REVERT-EXCEPTION answers. A queue item; `.claude/CHECKPOINTS.md` is a kit
file, so the edit asks him in a day session and needs his go.

```text
Small docs item for the queue (Mustafa): add a "disk hygiene" note to CHECKPOINTS.md: clean finished units' build output with cargo clean --target-dir <dir>, and remove merged agent copies with git worktree remove (never --force; git refuses if a copy has unsaved work). Consider one shared Cargo target folder for all copies. No rule change: rm -rf stays denied.
```
