# Mustafa, 2026-11-27: a standing rule against needless guard prompts (full paths for every file a shell command writes)
Saved by script (scripts/save_msg.py) from this chat's transcript (session_019YNMUUMZBxnFZJY9gp6qxy): message
e4b7acdf-2208-4abc-b96d-9fe3ad9cd552, 2026-11-27 12:37:31 UTC, whole, 463 bytes, sha256 a014e5f89155b99e4fd4c985f567115f6eb4aa28298a85aaa45ce22ca9c5a48d. Goes into docs/DECISIONS.md, word for
word, at the next take-in of main.

> From Mustafa, 27 November (written with my Advisor). A standing rule against needless guard prompts: in a shell command, spell out every file you write with its full path. No variable, substitution or wildcard in the name of a file that is written, moved, copied or deleted. Or use the Write and Edit tools for it. Then the guard can see it is not a kit or gate file and does not ask me. This changes nothing else: writes to kit and gate files still ask me first.
