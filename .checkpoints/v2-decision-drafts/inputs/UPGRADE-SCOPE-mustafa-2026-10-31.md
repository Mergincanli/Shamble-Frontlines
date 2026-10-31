# UPGRADE-SCOPE: Mustafa's update in the session's chat, 2026-10-31 (verbatim)

Received in session_0173vVo6CEsV7ckERUa6TtEd at about 18:45 Amsterdam, typed by Mustafa (a user message, not an agent
report). It replaces the drafts brief's upgrade note (brief §1 "UPGRADE-SCOPE" and §3's ADR 0005 sentence on in-match
upgrades), as its first line says.

> Update from Mustafa (2026-10-31). It replaces any earlier upgrade note from today.
>
> UPGRADE-SCOPE, a settled input for ADR 0005 (record it in STATUS with today's decisions):
> - Every unit and every weapon has its own roguelike upgrade track (pick one of three per level), owned by the player.
> - Tracks survive death and switching (switch back and the track is still there). They reset at the start of every match.
> - A class always comes with its own weapon: no weapon choice when buying.
> - Other weapons come only from killed enemies' drops; dropped weapons are world pickups.
> - Progress belongs to the weapon type, not the physical gun: when a gun is lost, its track waits for the next gun of that type.
> - In Unreal terms, the tracks live on the PlayerState, not the Pawn.
>
> Still open; list them as open questions, don't decide them: carry slots, ammo for looted weapons, and how fast tracks level (including catch-up when switching).
