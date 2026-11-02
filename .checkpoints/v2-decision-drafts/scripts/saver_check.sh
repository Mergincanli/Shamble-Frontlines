#!/usr/bin/env bash
# Mustafa's rule 3 (2026-11-02): at every checkpoint update, check that the background saver runs; restart it if not.
# Prints "saver running" (exit 0) or "saver NOT running" (exit 1); the session restarts it with run_in_background.
if pgrep -f "checkpoint.mjs watch v2-decision-drafts" >/dev/null; then echo "saver running"; exit 0; else echo "saver NOT running"; exit 1; fi
