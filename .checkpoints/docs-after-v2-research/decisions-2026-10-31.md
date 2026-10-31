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
