import sys; sys.path.insert(0, '/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/touchup-infra-engineer-scratch')
from wrap import wrap
ID = 'F13 (ADR 0006 half)'
TITLE = 'OQ-0006-20: ADR 0001\'s RS-hours marked proposed, the EUR shown'
START, END = 1007, 1010
AFTER = wrap("""and logging rows about 3 h; 14.3 + 3.3 + 2.2 + 3 ≈ 23), 23 × 0.1386 = 3.19 EUR, before ADR 0001's 3-run rows
(perhaps 15 to 30 RS-hours, proposed: the infra review's guess, not summed, until the run plan sums them; 15 × 0.1386
= 2.08 to 30 × 0.1386 = 4.16 EUR) and the CX33 hours of SH, MV and the load generators (as many as
spike 2's calibration run asks); so a 5 EUR cap over everything would send the plan back to the draft at its
planning check.""", '    ', '    ')
WHY = """F13 holds for ADR 0006. "perhaps 15 to 30 RS-hours, about 2 to 4 EUR" (0006:1008) comes from my review
(review-infra-engineer.md:56, finding 13: "ADR 0001's 3-run S30 rows add perhaps 15–30 hours, about 2–4 EUR"), stated
without a sum. A sum over ADR 0001's rows was not attempted tonight (62 rows; run lengths are given per row but setup
hours and which rows share a session are not), so the hours are marked proposed with that reason, and the EUR become
shown arithmetic at R6-HOST-F15's 0.1386 EUR a CCX23-hour (15 × 0.1386 = 2.079, 30 × 0.1386 = 4.158). PM-0006-18's
planning check (0006:1188) is where the run plan's hours are summed."""
