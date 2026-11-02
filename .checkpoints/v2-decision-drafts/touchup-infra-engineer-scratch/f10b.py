ID = 'F10 (part 6)'
TITLE = 'part 6, the supervisor\'s link: who owns routing joins by build'
START, END = 273, 273
AFTER = """   build is ADR 0001 part 11's, the release model OQ-0006-14's. *Ids:* R2-ROOM-OQ9, R2-XT-OQ4, R5-XT-OQ3, R6-XT-OQ1."""
WHY = """F10, one more place with the same stale reading (not in the verifier's list; mine to flag): part 6 ends "routing
joins by build is OQ-0006-14's" (0006:272-273), but ADR 0001 now routes joins by build in its protocol (part 11,
0001:495-498: "needed if releases run live, harmless in a release window: how releases reach running rooms is
OQ-0006-14"; also 0001:1628-1629), and part 15 after the fix above says so. Without this, the record says two things."""
