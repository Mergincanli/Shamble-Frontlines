# touchup-infra-engineer fixes (ADR 0006, after fixcheck-d1)

Every BEFORE below was copied by script (touchup-infra-engineer-scratch/mkblock.py: whole lines, by line range) from
docs/adr/0006-infrastructure-operations.md as it is on disk at 449356c, and occurs exactly once there; no two blocks
share a line, none touches "### Pass marks", and every AFTER keeps its BEFORE's line count (the coordinator's
request: README points into the open questions by line number), with no line over 120 characters.

F10 holds (part 15). ADR 0001 at 449356c: part 11 (0001:495-496) "the directory records each room's build,
protocol version and content hash, so the lobby can send a client only to rooms that match it"; its failure table
(0001:719) "a `JoinRejected` for a mismatch reloads the client once, and only if the lobby names a newer build than
its own; otherwise the client shows the reason and returns to the lobby, so a refused client never loops". So "cannot
recover" is stale. "(part 6)": this record's own part 6 (by the convention of "ADR 0002's (part 13, B9)" in the same
sentence) says routing is OQ-0006-14's, and ADR 0001's part 6 is snapshots; the routing is ADR 0001 part 11, written
"(its part 11)" because this record's part 11 is data protection. The verifier's sentence is used as written.

### FIX F10 (part 15, Releases: the mismatch sentence and "(part 6)"; lines 532-540 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
15. **Releases (open: OQ-0006-14).** How a new client build, content set or room binary reaches production, how two
    versions meet while rooms run, and how a bad server release is rolled back are not designed, and no topic
    researched them. As things stand, every room still on the old binary refuses a new client (ADR 0001's `JoinRejected`
    on a version or content mismatch), and the client's reload fetches the same new bundle, so it cannot recover.
    OQ-0006-14 decides the model before the first online playtest. Interim position (proposed): a release is made only
    while no room runs, with the client and every VM's room binary switched together, and the previous room binary stays
    on each VM, so a bad server release is switched back the same way. The production deploy job is ADR 0002's (part
    13, B9); routing joins by build is ADR 0001's protocol (part 6). *Need:* VISION_V2 §6.1 (rooms and private matches
    stay reachable across a release).
AFTER:
15. **Releases (open: OQ-0006-14).** How a new client build, content set or room binary reaches production, how two
    versions meet while rooms run, and how a bad server release is rolled back are not designed, and no topic
    researched them. As things stand, the lobby sends a client only to rooms of its own build and content hash, and a
    refused client reloads at most once (ADR 0001 part 11 and its failure table), but with no release model a new client
    finds no room of its build until rooms on the new binary run. OQ-0006-14 decides the model before the first online
    playtest. Interim position (proposed): a release is made only while no room runs, with the client and every VM's
    room binary switched together, and the previous room binary stays on each VM, so a bad server release is switched
    back the same way. The production deploy job is ADR 0002's (part 13, B9); routing joins by build is ADR 0001's
    protocol (its part 11). *Need:* VISION_V2 §6.1 (rooms and private matches stay reachable across a release).
END

F11 holds. ADR 0001 part 10 (0001:402-404): "each room process listens on one UDP port for WebTransport and one
TCP port for the WebSocket fallback, taken from the VM's two firewall ranges (ADR 0006 part 5 ...), and the lobby hands
the client the host and both ports"; its catalogue (0001:643) "the host and the room's ports"; this record's part 5
(0006:224-225) "one UDP range for WebTransport and one TCP range for the WebSocket fallback". The verifier's wording
("each of the firewall's two ranges") does not fit lines 270-271 without a third line; "each firewall range" does,
and part 5 gives a game VM exactly those two ranges.

### FIX F11 (part 6, the supervisor's link to the lobby: one port per firewall range; lines 270-271 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   the overrides (ADR 0001's supervisor calls); the supervisor answers with the host and a port from the firewall's
   range (part 5); it reports Closed, an abnormal exit and draining. The link is authenticated (how: the security
AFTER:
   the overrides (ADR 0001's supervisor calls); the supervisor answers with the host and a port from each firewall range
   (part 5); it reports Closed, an abnormal exit and draining. The link is authenticated (how: the security
END

F10, one more place with the same stale reading (not in the verifier's list; mine to flag): part 6 ends "routing
joins by build is OQ-0006-14's" (0006:272-273), but ADR 0001 now routes joins by build in its protocol (part 11,
0001:495-498: "needed if releases run live, harmless in a release window: how releases reach running rooms is
OQ-0006-14"; also 0001:1628-1629), and part 15 after the fix above says so. Without this, the record says two things.

### FIX F10 (part 6) (part 6, the supervisor's link: who owns routing joins by build; lines 273-273 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
   build is OQ-0006-14's. *Ids:* R2-ROOM-OQ9, R2-XT-OQ4, R5-XT-OQ3, R6-XT-OQ1.
AFTER:
   build is ADR 0001 part 11's, the release model OQ-0006-14's. *Ids:* R2-ROOM-OQ9, R2-XT-OQ4, R5-XT-OQ3, R6-XT-OQ1.
END

F10 holds (OQ-0006-14). "its reload fetches the same new bundle (ADR 0001's failure table)" (0006:918) is the old
row; the failure table now reloads at most once and only for a newer build (0001:719), and the lobby sends a client
only to rooms of its build (0001:495-496, 1628-1629). "sends" for the verifier's "can route": fewer words, same claim.

### FIX F10 (OQ-0006-14, why it matters) (OQ-0006-14: the lobby routes by build, the reload no longer loops; lines 918-918 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    client with `JoinRejected`, and its reload fetches the same new bundle (ADR 0001's failure table), so from the
AFTER:
    client with `JoinRejected`, and the lobby sends it only to rooms of its build (ADR 0001 part 11), so from the
END

F10 holds (option (b)). "(a field ADR 0001's directory would add)" (0006:925): ADR 0001 part 11 already records each
room's build, protocol version and content hash and sends a client only to matching rooms (0001:495-496).

### FIX F10 (OQ-0006-14, option b) (OQ-0006-14 option (b): the directory field exists in ADR 0001; lines 925-925 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    a client only to rooms of its own build (a field ADR 0001's directory would add), old rooms finishing, then the
AFTER:
    a client only to rooms of its own build (as ADR 0001 part 11's directory does), old rooms finishing, then the
END

F13 holds for ADR 0006. "perhaps 15 to 30 RS-hours, about 2 to 4 EUR" (0006:1008) comes from my review
(review-infra-engineer.md:56, finding 13: "ADR 0001's 3-run S30 rows add perhaps 15–30 hours, about 2–4 EUR"), stated
without a sum. A sum over ADR 0001's rows was not attempted tonight (62 rows; run lengths are given per row but setup
hours and which rows share a session are not), so the hours are marked proposed with that reason, and the EUR become
shown arithmetic at R6-HOST-F15's 0.1386 EUR a CCX23-hour (15 × 0.1386 = 2.079, 30 × 0.1386 = 4.158). PM-0006-18's
planning check (0006:1188) is where the run plan's hours are summed.

### FIX F13 (ADR 0006 half) (OQ-0006-20: ADR 0001's RS-hours marked proposed, the EUR shown; lines 1007-1010 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    and logging rows about 3 h; 14.3 + 3.3 + 2.2 + 3 ≈ 23), 23 × 0.1386 = 3.19 EUR, before ADR 0001's 3-run rows
    (perhaps 15 to 30 RS-hours, about 2 to 4 EUR) and the CX33 hours of SH, MV and the load generators (as many as
    spike 2's calibration run asks); so a 5 EUR cap over everything would send the plan back to the draft at its
    planning check.
AFTER:
    and logging rows about 3 h; 14.3 + 3.3 + 2.2 + 3 ≈ 23), 23 × 0.1386 = 3.19 EUR, before ADR 0001's 3-run rows
    (perhaps 15 to 30 RS-hours, proposed: the infra review's guess, which the run plan replaces with a sum;
    15 to 30 × 0.1386 = 2.08 to 4.16 EUR) and the CX33 hours of SH, MV and the load generators (as many as spike 2's
    calibration run asks); so a 5 EUR cap over everything would send the plan back to the draft at its planning check.
END

F14 holds: "Only one listener per address can hold TCP 443" (0006:953) has no source in the record; it came from my
review (review-infra-engineer.md:57, finding 7), also unsourced. As its skeptic: the claim is likely too strong as well,
since Linux's SO_REUSEPORT lets several sockets listen on one address and port (from memory; socket(7) was not read
tonight: no local man pages, and this task fetches nothing), but the kernel then picks the socket per connection, not
by path. The new wording holds either way and is all the sentence's "so" needs: each connection reaches one listener,
so either that listener routes by path or each room has its own address. Marked "(no source read)" as the verifier
asks; a read of socket(7) can drop the mark later.

### FIX F14 (OQ-0006-16: the TCP 443 listener claim, reworded and marked unsourced; lines 953-954 on disk now)
FILE: docs/adr/0006-infrastructure-operations.md
BEFORE:
    QUIC (a question; no source read). Only one listener per address can hold TCP 443, so 443 needs a front on each VM
    that routes by path to the room processes, or an address per room.
AFTER:
    QUIC (a question; no source read). A connection to an address's TCP 443 reaches one listener (no source read), so
    443 needs a front on each VM that routes by path to the room processes, or an address per room.
END

Checks (21:46): apply_fixes.py on a scratch copy applies all 7 blocks, 1485 -> 1485 lines, every heading and OQ line
at its old number; glossary_check_drafts.py --skip-records on scratch copies: exact 0, in context 98, before and after,
hit lists identical; no block sits beside a table.
