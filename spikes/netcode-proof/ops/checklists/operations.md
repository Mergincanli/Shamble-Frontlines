# The operations checklist (Mustafa's two steps; plan.md:1224-1225, :1239-1240)

u8-brief.md rev 1.1 Block 10. Two steps, each written into the results by the laptop session as his reading.

## Step 1, during R1's browser block: the ping (PM-0006-12)
Reported, no pass number; a surprise goes to him (marks.json PM-0006-12's notes).
1. The laptop session shows LG's address (a VM address, R-21) on the laptop's screen.
2. On the laptop, in a terminal: `ping -n 100 <LG's address>`.
3. On the PC, in a terminal: the same command, typed from the laptop's screen.
4. From each, paste the "Packets" line and the "Approximate round trip times" line into the session (the PC's by
   typing them from its screen; never through chat, email, GitHub or a synced clipboard).
5. The session writes one `ping` event per machine (`mean_ns`, `max_ns`, `lost_count` out of 100) and, from him, a
   `person` event: who = Mustafa, mark = PM-0006-12, verdict = pass once he has read the numbers beside VISION_V2 §4's
   "about 100 ms", fail for a surprise, steps = the pasted lines.
LG answers only his home address's echoes (`fw-lg`); RS never answers a ping.

## Step 2, after the last run: the costs (PM-0006-18)
1. In the Hetzner console, the spike project's costs: one amount before VAT, every VM-hour and traffic block the proof
   test created, the kept Primary IPs included.
2. Beside each run's cap: R1 EUR 0.80; R2 EUR 1.90; R3 EUR 1.80; R5 EUR 3.60 (replan.md:31-34).
3. The session writes one `person` event: who = Mustafa, mark = PM-0006-18, verdict = pass if every run's cost is
   under its cap, steps = each run's cost beside its cap, as he read them.
