# Decisions of 2026-10-31 (verbatim)

Relayed by Mustafa in the v2 day session's chat (session_0173vVo5q59ameR6cnN9BENg), 2026-10-31, just before the
merge of PR #28 at 13:26:30 UTC (merge commit b6effde). His message, word for word:

> Mustafa: merge #28.
> - MODES-3: (a), totals; the biggest match is 24v24 = 48.
> - PREVIEW-WALL: (a), private from the first preview that reaches the test game server.
> - SEC-SPREAD: (a), server-only spread.
> - ABL-PREDICT: later, after the Feel Lab test.
> Advisor:
> - BASELINE-WHEN: (b). List it in ADR 0006 as open, with temporary positions. Research the
>   security baseline before the first online playtest, in its own fresh session.

## The questions they answer
As the session put them in its chat on 2026-10-31 (the check-in message before this one); the full wording is in each
research topic's shared decision questions and in `docs/reports/2026-10-31-v2-research.md`.

- **MODES-3 (Mustafa):** are the v2 vision's starting sizes ("two teams with 2–8 players, three teams with 6–18") totals
  or per team? Options: (a) totals, so the biggest match is 24v24 = 48; (b) per team, so it is 18v18v18 = 54.
  Recommended: (a). **Answer: (a).**
- **PREVIEW-WALL (Mustafa):** should the preview links made for each pull request sit behind a Cloudflare login on
  Mustafa's account when they connect to the test game server? Options: (a) private from the first such preview;
  (b) open; (c) decide after the security baseline is researched. Recommended: (a). **Answer: (a).**
- **SEC-SPREAD (Mustafa):** should bullet tracers on screen match the server's weapon spread exactly, or should the
  spread stay known only to the server? Options: (a) server only: tracers use a similar-looking spread and the hit
  corrects them (the research's default); (b) shared seed, so tracers match exactly. Recommended: (a).
  **Answer: (a).**
- **ABL-PREDICT (Mustafa, later):** should the player's own abilities (cost, cooldown, Banzai speed, Shield slow) react
  on screen at once, before the server confirms them? Options: (a) predict them; (b) wait for the server.
  Recommended: no answer yet; decide after a Feel Lab test that compares both ways on the Molotov and the Shield.
  **Answer: later, after the Feel Lab test** (stays open).
- **BASELINE-WHEN (the Advisor):** research the postponed security baseline before writing the infrastructure and
  operations decision draft (ADR 0006), or list it there as open? Options: (a) research it now; (b) list it as open,
  with temporary positions; (c) list it as one shared open question. Recommended: (b). **Answer: (b)**, plus: the
  security baseline is researched before the first online playtest, in its own fresh session.

Not answered in this message: NS-2 (Mustafa creates the fresh-session Night shift routine on the Routines page).

## Second message (2026-10-31, between 13:35 and 13:50 UTC, while the session planned docs-after-v2-research), word for word

> Advisor:
> - VISION-LINE: (a). I wrote that line; the clarification is right. Add it as proposed.
> - The 60 Hz margin (8.28 of 8.33 ms) is effectively zero. For the decision drafts: a 30 Hz
>   server tick is the baseline for 48 players; 60 Hz only if the netcode proof test shows at
>   least 25 % headroom on the target hardware.
>
> Mustafa + Advisor: new slice order: 1 move & shoot → 2 bots and three teams (played locally)
> → 3 online → 4 the loop. Conditions, for the decision drafts:
> 1. From slice 1, the game runs as a client plus a local server (the Rust simulation in a Web
>    Worker), talking through the same messages the network will carry. No offline-only
>    shortcuts.
> 2. Player movement and shooting are driven by inputs and can be replayed from day one, so
>    prediction slots in later without rework.
> 3. The netcode proof test (48 simulated players over WebTransport) still runs before the
>    foundation.
> Record the new order in ROADMAP and STATUS.

What "as proposed" refers to (the session's chat message before it): VISION-LINE (a) = add, in brackets, "in total:
1v1 to 4v4, and 2v2v2 to 6v6v6; MODES-3 (a), Mustafa, 2026-10-31" to VISION_V2 §3's line "two teams with 2–8 players,
and three teams with 6–18 players", and change nothing else.

A fact the session adds for the record (GLOSSARY.md §1's table, lines 62-66): at N = 48 and 60 Hz, a tick that both
encodes a snapshot and re-floods fits the 50 % server tick budget by 0.05 ms (8.28 of 8.33 ms), and its server-only
share is 0.24 ms over its own 25 % (4.41 of 4.17 ms). The session's chat message gave only the first number.
