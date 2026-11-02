import sys; sys.path.insert(0, '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/touchup-infra-engineer-scratch')
from wrap import wrap
ID = 'F10'
TITLE = 'part 15, Releases: the mismatch sentence and "(part 6)"'
START, END = 532, 540
AFTER = """15. **Releases (open: OQ-0006-14).** How a new client build, content set or room binary reaches production, how two
    versions meet while rooms run, and how a bad server release is rolled back are not designed, and no topic
""" + wrap("""researched them. As things stand, the lobby sends a client only to rooms of its own build and content hash, and a
refused client reloads at most once (ADR 0001 part 11 and its failure table), but with no release model a new client
finds no room of its build until rooms on the new binary run.
OQ-0006-14 decides the model before the first online playtest. Interim position (proposed): a release is made only
while no room runs, with the client and every VM's room binary switched together, and the previous room binary stays
on each VM, so a bad server release is switched back the same way. The production deploy job is ADR 0002's (part
13, B9); routing joins by build is ADR 0001's protocol (its part 11). *Need:* VISION_V2 §6.1 (rooms and private matches
stay reachable across a release).""", '    ', '    ')
WHY = """F10 holds (part 15). ADR 0001 at 449356c: part 11 (0001:495-496) "the directory records each room's build,
protocol version and content hash, so the lobby can send a client only to rooms that match it"; its failure table
(0001:719) "a `JoinRejected` for a mismatch reloads the client once, and only if the lobby names a newer build than
its own; otherwise the client shows the reason and returns to the lobby, so a refused client never loops". So "cannot
recover" is stale. "(part 6)": this record's own part 6 (by the convention of "ADR 0002's (part 13, B9)" in the same
sentence) says routing is OQ-0006-14's, and ADR 0001's part 6 is snapshots; the routing is ADR 0001 part 11, written
"(its part 11)" because this record's part 11 is data protection. The verifier's sentence is used as written."""
