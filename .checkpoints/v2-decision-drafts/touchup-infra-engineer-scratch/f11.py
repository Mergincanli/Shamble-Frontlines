import sys; sys.path.insert(0, '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/touchup-infra-engineer-scratch')
from wrap import wrap
ID = 'F11'
TITLE = 'part 6, the supervisor\'s link to the lobby: one port per firewall range'
START, END = 270, 271
AFTER = wrap("""the overrides (ADR 0001's supervisor calls); the supervisor answers with the host and a port from each firewall
range (part 5); it reports Closed, an abnormal exit and draining. The link is authenticated (how: the security""", '   ', '   ')
WHY = """F11 holds. ADR 0001 part 10 (0001:402-404): "each room process listens on one UDP port for WebTransport and one
TCP port for the WebSocket fallback, taken from the VM's two firewall ranges (ADR 0006 part 5 ...), and the lobby hands
the client the host and both ports"; its catalogue (0001:643) "the host and the room's ports"; this record's part 5
(0006:224-225) "one UDP range for WebTransport and one TCP range for the WebSocket fallback". The verifier's wording
("each of the firewall's two ranges") does not fit lines 270-271 without a third line; "each firewall range" does,
and part 5 gives a game VM exactly those two ranges."""
