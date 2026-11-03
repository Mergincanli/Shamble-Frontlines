import json
P = {}
CLAUSE = ('the machines its rows name (OQ-0003-1, answered in part: Mustafa names them before the look proof test; until '
          'then the stand-in for ML, for rough checks only); this spike\'s own plan lists its rows that run before they '
          'are named, and none of them is judged on the stand-in (the advisor\'s reading, 2026-11-03)')
P['spk1'] = [
 ['minimum laptop, and RD, the recommended desktop (ADR 0003, OQ-0003-1: the models are not named yet); MAC, a macOS '
  'machine with the stable Safari',
  'minimum laptop, RD, the recommended desktop, and MAC, the Mac for Safari, with the stable Safari'],
 ['lines 959–975); MP, Mustafa\'s PC in Brave, for what he judges only ("Test PC: my own, in Brave", 2026-11-03; it '
  'stands in for none of ML, RD and MAC: the advisor\'s reading, 2026-11-03).',
  'lines 959–975). Which machines they are is OQ-0003-1 (ADR 0003), answered in part by Mustafa (2026-11-03): '
  '"minimum: a typical student laptop with integrated graphics; recommended: a desktop with a mid-range card (RTX '
  '3060/4060 class); Mac: a MacBook Air, M1 or newer, for Safari. All borrowed."; still open: the actual machines, '
  'named by Mustafa before the look proof test. Until they are named, his laptop is the stand-in for ML, in his words: '
  '“Until then, my laptop\'s integrated graphics (Brave set to "Power saving") stands in for the minimum, for rough '
  'checks only.” MP, Mustafa\'s PC in Brave, is for what he judges only ("Test PC: my own, in Brave", 2026-11-03). A '
  'rough check is reported, never judged against a pass mark. MP stays what he judges, never a measured result, and '
  'nothing stands in for RD or MAC (the advisor\'s reading, 2026-11-03).']]
P['spk2'] = [['ML, RD and MAC named (OQ-0003-1, still open); a throwaway sim', CLAUSE + '; a throwaway sim']]
P['spk3'] = [
 ['(Mustafa, 2026-11-03; OQ-0006-5\'s development case (i) by the advisor\'s reading, and OQ-0006-15, both answered in '
  'part), the domain registered before the test starts',
  '(Mustafa, 2026-11-03; OQ-0006-5\'s development case (i) by the advisor\'s reading, answered in part), '
  'shamblefrontlines.com (OQ-0006-15, answered by Mustafa, 2026-11-03: "shamblefrontlines.com, which I register at '
  'Cloudflare."), the domain registered before the test starts'],
 ['the spike\'s run plan under its bill mark (PM-0006-18) and the cost Mustafa allows (OQ-0006-20, still open; answered '
  'in part: "VMs exist only during a run.") before the first VM.',
  'the spike\'s run plan under its bill mark (PM-0006-18), with each run\'s cost cap, from its planned hours, written in '
  'that run\'s plan for Mustafa\'s OK (OQ-0006-20, answered by Mustafa, 2026-11-03: "one cap per run, from its planned '
  'hours, written in that run\'s plan for my OK. I create and delete the VMs myself.").']]
P['spk4'] = [['the drafts approved; ML named and on hand (OQ-0003-1), MAC, and displays at 120 and 144 Hz; the',
  'the drafts approved; ' + CLAUSE + '; displays at 120 and 144 Hz; the']]
P['spk5'] = [['the drafts approved; RS orderable (OQ-0006-1), ML and RD named (OQ-0003-1), MAC; the',
  'the drafts approved; RS orderable (OQ-0006-1); ' + CLAUSE + '; the']]
P['spk6'] = [['the drafts approved; ML and RD named and on hand (OQ-0003-1), MAC; a few',
  'the drafts approved; ML, RD and MAC named and on hand (OQ-0003-1, answered in part: Mustafa names them before the '
  'look proof test); a few']]
P['spk7'] = [['Mustafa makes them himself with AI help in Blender, as blockouts with a skin, named colour slots and LOOK_V2\'s '
  'proportions (OQ-SPIKE-1,',
  'Mustafa makes them himself with AI help in Blender, where blockouts with a skin, named colour slots and LOOK_V2\'s '
  'proportions are enough (OQ-SPIKE-1,']]
P['spk8'] = [['(OQ-SPIKE-1, answered), not a session\'s; ADR 0005\'s share S', '(OQ-SPIKE-1, answered); ADR 0005\'s share S']]
P['spk9'] = [['this part\'s export, and LOOK_V2 is not edited. The part is',
  'this part\'s export, and LOOK_V2 is not edited; Mustafa (2026-11-03, LOOK_V2-§8): “yes, keep it as it is, with '
  'Blender for the test models only. When the asset research\'s export part reports, §8 says "Spline or Blender".” '
  'That edit belongs to that part\'s change-set and goes to Mustafa for the merge (the advisor\'s reading, 2026-11-03). '
  'The part is']]
P['spk10'] = [
 ['The reference machines (OQ-0003-1: ML, RD and MAC still to be named), the server location (OQ-0006-1, OQ-0006-2), '
  'the spikes\' cost (OQ-0006-20, answered in part), the domain itself (OQ-0006-15, answered in part) and the '
  'provisional goal budget (OQ-0006-19) are listed',
  'The reference machines ML, RD and MAC (OQ-0003-1, answered in part: Mustafa names them before the look proof test), '
  'the server location (OQ-0006-1, OQ-0006-2) and the provisional goal budget (OQ-0006-19) are listed'],
 ['ASSET-RESEARCH\'s timing and OQ-SPIKE-1 (below); the register records each answer.',
  'ASSET-RESEARCH\'s timing and OQ-SPIKE-1 (below); Mustafa\'s answers to the escalations, later that day, answered the '
  'spikes\' cost (OQ-0006-20) and the domain (OQ-0006-15); the register records each answer.']]
P['spk11'] = [['*Why it matters:* the look proof', '*Why it matters (as asked):* the look proof']]
P['spk12'] = [['Making the models is his own work, not a session\'s, so no spike\'s size carries it.',
  'Making the models is his own work, so no spike\'s size carries it.']]
for k, v in P.items():
    json.dump(v, open(f'pairs/{k}.json', 'w', encoding='utf8'), ensure_ascii=False)
print('ok', len(P))
