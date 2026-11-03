import json
P = {}
P['idx5'] = [["when buying offers no weapon choice and other weapons come only from drops? |",
  'when buying offers no weapon choice and other weapons come only from drops? A lean from Mustafa (2026-11-03), still '
  'open: "leaning towards class variants, with more variants added over time. Not final." |']]
P['idx6'] = [["and what happens to the one in hand on a pickup? | Mustafa; LATER: the design session after the netcode proof test |",
  'and what happens to the one in hand on a pickup? Answered by Mustafa (2026-11-03): "a looted weapon replaces the main '
  '(class) weapon; the sidearm stays as backup. Assumed, to confirm: the class\'s own abilities stay." | Mustafa |']]
P['idx7'] = [["What does a looted gun come with, and can it be refilled? |",
  'What does a looted gun come with, and can it be refilled? Mustafa (2026-11-03): "still open (infinite, limited, or '
  'supply lines by an engineer class)." The Advisor\'s lean, as his message marks it, and not an answer: "limited but '
  'generous, refilled at the barracks and supply points." |']]
P['idx8'] = [["and does a track the player switches to catch up? | Mustafa; LATER: the design session after the netcode proof test |",
  'and does a track the player switches to catch up? Answered by Mustafa (2026-11-03): "XP from kills, assists and '
  'objective work while using that unit or weapon; fast early levels; a track that\'s behind the player\'s best levels '
  'faster." | Mustafa |']]
P['idx9'] = [['two reference machines? Still open after the stop: Mustafa\'s "Test PC: my own, in Brave" (2026-11-03) is MP, what he '
  'judges and never a measured result, so it stands in for none of ML, RD and MAC, and the netcode proof test cannot start '
  'without ML and RD (the advisor\'s reading, 2026-11-03). |',
  'two reference machines? Answered in part by Mustafa (2026-11-03): "minimum: a typical student laptop with integrated '
  'graphics; recommended: a desktop with a mid-range card (RTX 3060/4060 class); Mac: a MacBook Air, M1 or newer, for '
  'Safari. All borrowed." Until they are named, the stand-in for ML, in his words: “Until then, my laptop\'s integrated '
  'graphics (Brave set to "Power saving") stands in for the minimum, for rough checks only.” His "Test PC: my own, in '
  'Brave" (at the stop, 2026-11-03) is MP. A rough check is reported, never judged against a pass mark. MP stays what he '
  'judges, never a measured result, and nothing stands in for RD or MAC (the advisor\'s reading, 2026-11-03). A row on '
  'ML, RD or MAC is judged only on a named machine, and naming them no longer gates the netcode proof test\'s start (the '
  'advisor\'s reading, 2026-11-03). Still open: the actual machines, named by Mustafa before the look proof test. |']]
P['idx10'] = [["any rule beyond that is a design question (no research id). |",
  'any rule beyond that is a design question (no research id). Mustafa (2026-11-03): "Fragments as damage: visual only '
  'for now. The server sends one blast event, the browser draws the fragments, and the spray matches the real damage '
  'radius." |']]
P['idx11'] = [["Faction variants: full definitions that reference shared ones, or overrides of a base? |",
  'Faction variants: full definitions that reference shared ones, or overrides of a base? Faction asymmetry stays open, '
  'in Mustafa\'s words (2026-11-03): "Still open: faction asymmetry (Mustafa works on the unit list first)". |']]
P['idx12'] = [["and is the Peacekeeper's side of the triangle strong enough before Tech 1? |",
  'and is the Peacekeeper\'s side of the triangle strong enough before Tech 1? Answered for the Shield and the blade by '
  'Mustafa (2026-11-03): "a tall, slightly curved Riot Shield blocks the Banzai blade from the front, so the Warrior must '
  'flank; the Conscript\'s Molotov punishes the shield hard. LOOP_V2\'s counter triangle; numbers tuned later." The '
  'Peacekeeper\'s strength before Tech 1 is left to his "numbers tuned later" (the advisor\'s reading, 2026-11-03). |']]
P['idx13'] = [["What does spawn protection cover, and what ends it early? | Mustafa (the 3 s is the Advisor's `[DEFAULT]`; what it "
  "covers and what ends it are new rules); LATER:",
  'What does spawn protection cover, and what ends it early? Answered in part by Mustafa (2026-11-03): "3 s, against '
  'spawn killing." The Advisor\'s pencil, which Mustafa may veto: "Firing ends it early". Still open: what it covers, '
  'ADR 0005\'s options (a) damage only or (b) damage and every effect another team applies. | Mustafa (what it covers is '
  'a new rule; he may veto the Advisor\'s pencil for what ends it); LATER:']]
P['idx14'] = [["| Who sees a player's upgrade picks? | Mustafa; LATER: the design session after the netcode proof test | "
  "[ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB15 |",
  '| Who sees a player\'s upgrade picks? Answered by Mustafa (2026-11-03): "Upgrade visibility: enemies can see a '
  'player\'s build." Placing it at OQ-0005-AB15 is the advisor\'s reading (2026-11-03). How enemies see it is not said: '
  'the scoreboard or a cue on the soldier; a cue is ADR 0003\'s to draw, and the look is Mustafa\'s. | Mustafa | '
  '[ADR 0005](0005-content-abilities-assets.md) OQ-0005-AB15; [ADR 0001](0001-multiplayer-netcode.md) part 12 (the '
  'picks\' visibility class) |']]
P['idx15'] = [['and when is it registered? Answered in part by Mustafa (2026-11-03): "for the WebSocket leg, a domain I register." It '
  'is registered before the netcode proof test (the advisor\'s reading, 2026-11-03); still open: which domain, which '
  'registrar and which DNS host, and whether production uses it later, decided by Mustafa. |',
  'and when is it registered? Answered by Mustafa (2026-11-03): "shamblefrontlines.com, which I register at Cloudflare." '
  'At the stop he had written: "for the WebSocket leg, a domain I register." It is registered before the netcode proof '
  'test (the advisor\'s reading, 2026-11-03); production uses the same domain, since it is the game\'s name and ADR 0006 '
  'part 6 needs one registrable domain (the advisor\'s reading, 2026-11-03). The DNS host and the prices stay "not '
  'researched". |']]
P['idx16'] = [['What may the netcode proof test\'s VMs cost? Answered in part by Mustafa (2026-11-03): "VMs exist only during a run."; '
  'still open: the cost cap, decided by Mustafa (money). | Mustafa (money) | [ADR 0006](0006-infrastructure-operations.md) '
  'OQ-0006-20 |',
  'What may the netcode proof test\'s VMs cost? Answered by Mustafa (2026-11-03): "one cap per run, from its planned '
  'hours, written in that run\'s plan for my OK. I create and delete the VMs myself." At the stop he had written: "VMs '
  'exist only during a run." | Mustafa (money) | [ADR 0006](0006-infrastructure-operations.md) OQ-0006-20 |\n'
  '| OQ-0006-21 (COMMUNITY-SERVERS) | Could players one day host their own servers, and what must the drafts keep '
  'possible for it? Mustafa (2026-11-03), a new question, "keep the door open, don\'t design it": "players may one day '
  'host their own servers, possibly with revenue such as official private servers they rent." and "The drafts must not '
  'lock it out: the room server stays one self-contained program with a config file (players, length, rules), checks '
  'the game\'s version and content hash, and marks each match as official or community, so progression can count on '
  'official servers only." No option is weighed and no timing is set. | Mustafa | '
  '[ADR 0006](0006-infrastructure-operations.md) OQ-0006-21 |']]
for k, v in P.items():
    json.dump(v, open(f'pairs/{k}.json', 'w', encoding='utf8'), ensure_ascii=False)
print('ok', len(P))
