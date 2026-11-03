# Answers to the escalations, COMMUNITY-SERVERS, and the verification stop (Mustafa, 2026-11-03, about 17:31 Amsterdam, in the session's chat; word for word)

Answers (Mustafa, 2026-11-03):
- REF-MACHINES: minimum: a typical student laptop with integrated graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari. All borrowed. I name the actual machines before the look proof test; the netcode proof test doesn't measure client frames. Until then, my laptop's integrated graphics (Brave set to "Power saving") stands in for the minimum, for rough checks only.
- LOOK_V2-§8: yes, keep it as it is, with Blender for the test models only. When the asset research's export part reports, §8 says "Spline or Blender".
- SPIKE-COST: one cap per run, from its planned hours, written in that run's plan for my OK. I create and delete the VMs myself.
- DOMAIN: shamblefrontlines.com, which I register at Cloudflare.
- COMMUNITY-SERVERS (new; keep the door open, don't design it): players may one day host their own servers, possibly with revenue such as official private servers they rent. Add it as an open question to ADR 0006 and the README. The drafts must not lock it out: the room server stays one self-contained program with a config file (players, length, rules), checks the game's version and content hash, and marks each match as official or community, so progression can count on official servers only.
- Verification: after e2b, e3b and their fix round, stop. No further re-check rounds; anything left goes into the PR as notes. The spikes are the real test, and we've had ten usage cuts.
