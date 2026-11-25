# Mustafa's answer EGL-TRIAL (a)

_Copied by script from the chat's transcript (session_019YNMUUemXge56777MiUWFz), never retyped: the user message of 2026-11-25T11:07:23.505Z (uuid bea1027f-bf08-4dae-aa6a-1fdb6572e64a), whole (1795 bytes, sha256 6450235a15ada4aa5c83b1d6d01fc7a41b27defe9249fce8651eca294dcbab3e). What it answers: the chat's question EGL-TRIAL (question-2026-11-25-egl-trial.md at 1b64214): how the Firefox drawing tests are tried; (a) yes to libegl1 and libegl-mesa0 for the container's trial only, (b) the first GitHub run as their trial, (c) Firefox in CI runs only the rule tests; recommended (a)_

---
For the CI Builder, from me, written with my main Advisor (25 November, 12:15 Amsterdam).

1. EGL-TRIAL (how the Firefox drawing tests are tried in your container): (a), yes, for these two files only and for this container's trial only: libegl1_1.7.0-1build1_amd64.deb, 28670 bytes, sha256 e549f7776216f7bd3b1c216729fb40a97a562e2eb7c563cde632b4931f9a4e57, and libegl-mesa0_25.2.8-0ubuntu0.24.04.5_amd64.deb, 117190 bytes, sha256 c04cdb8c4d29667ab9d2c3abac6bb38dc066f5cdb874d867f85f5e238d546f82, from the two addresses in your question. My main Advisor measured the same sizes and checksums over a second route and deleted its copies.
2. The route is the one in your question: fetched into a new folder under .gate/, size and checksum checked before use, unpacked with dpkg-deb -x, no apt, no root, no install script, handed only to the trial's Firefox through its environment. Nothing in the system changes, nothing of it is committed, and the folder is deleted when the pull request is merged. If a value differs, stop and tell me.
3. This answers your container only. Whether GitHub's runner has this library is still unknown. So in ask 2: say which Firefox tests passed here with the two files and which did not; add one read-only line to the CI text that prints whether the runner has libEGL.so.1, so the first GitHub run says it in words; and bring no install line for the runner. If the runner lacks the library, that comes back to me as its own question with options.
4. A Firefox test that still fails with the library in place: read it first. If the cause is the game's page, rule 4 of my opening block holds: stop, tell me in three lines with options, and fix nothing.
5. Every other file, version or checksum comes back to me first.

Answer in at most 6 lines. Plain words, as before.
