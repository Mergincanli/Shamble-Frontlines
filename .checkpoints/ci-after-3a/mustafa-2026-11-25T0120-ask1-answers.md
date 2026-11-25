# Mustafa's answers to ask 1: FIREFOX-SOURCE (c) and TRIAL-DOWNLOAD (a)

_Copied by script from the chat's transcript (session_019YNMUUemXge56777MiUWFz), never retyped: the user message of 2026-11-25T00:07:29.708Z (uuid 4bfa53c9-eb52-4062-a15c-77e3c2854f7b), whole (1853 bytes, sha256 521e395ae056b212a7d5c9f4ddd75e873a3102c0705666cfc926603ad0a7f055). What it answers: the chat's ask 1: FIREFOX-SOURCE, which Firefox CI tests in ((a) GitHub's own Mozilla Firefox through Playwright's moz-firefox route, (b) Playwright's own Firefox 159.0 build 1572, (c) (a) with his yes in advance to fall back to (b) for a failure Playwright's route causes; recommended (c)); TRIAL-DOWNLOAD, may the chat download that Firefox into its container to try it (the deb firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb, 90,437,572 bytes, sha256 bd304b0b...cf1ed1; recommended (a) yes)_

---
For the CI Builder, from me, written with my main Advisor (25 November, 01:20 Amsterdam).

1. FIREFOX-SOURCE (which Firefox CI tests in): (c). CI tests in GitHub's own Mozilla Firefox. My yes in advance for the fallback to Playwright's own Firefox holds only for a failure that Playwright's route causes, never for one the game's page causes. One addition: if you fall back, your second ask names the tests that failed on GitHub's Firefox, with the failing lines, and says why that is Playwright's route and not the game. My main Advisor reads that before my yes to any CI text. Nothing falls back silently.
2. TRIAL-DOWNLOAD (may you download Firefox into your container to try it): (a), yes, for this one file only: firefox_161.0+build1-0ubuntu0.24.04.1~mt1_amd64.deb from the Mozilla team's package archive, 90437572 bytes, sha256 bd304b0b181c4fed449279e25d309075c3a2e85345e63d1118c5085cefcf1ed1, by the route of your plan's section 3: fetched over https into a new folder under .gate/, size and checksum checked first, unpacked without installing, no apt, no root, no install script. If a value here differs from your ask 1, stop and tell me.
3. The fallback file: Playwright's Firefox 159.0, build 1572, firefox-ubuntu-24.04.zip, 118965377 bytes, sha256 189ce3e09c61776010ab8a7cbffaa016b8b1914d3c9b50cb5ab6d0c5db4c368f. My main Advisor measured the same size and checksum over a second route today. Your fetch to measure it stands. It stays unopened unless the fallback of item 1 happens, and is deleted when the pull request is merged without it.
4. Every other file, version or checksum comes back to me first.
5. From now on, when you wait on me: save the full question as a file in your notes, push it, and name the file and the commit in your board post, next to needs-musti and a due label.

Answer in at most 6 lines. Plain words, as before.
