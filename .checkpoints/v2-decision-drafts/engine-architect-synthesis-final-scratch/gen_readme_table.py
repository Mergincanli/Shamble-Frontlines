"""README "Decisions the drafts ask for": re-pin every row to 604a497, update changed questions and deciders, add the
final round's new rows. One FIX block per changed row (new rows ride in the block of the row they follow).
Run: python3 -I gen_readme_table.py (from this folder). Restart-safe (lib.fix skips blocks already written)."""
import sys, re
sys.path.insert(0, '.')
from lib import *
R = 'docs/adr/README.md'
oq = {}
for r in open('oq-604a497.tsv', encoding='utf8'):
    a, ln, i, t, d = r.rstrip('\n').split('\t'); oq[i] = (a, int(ln))
FILES = {'0001': '0001-multiplayer-netcode.md', '0002': '0002-code-structure-build-toolchain.md',
         '0003': '0003-rendering.md', '0004': '0004-simulation-core.md',
         '0005': '0005-content-abilities-assets.md', '0006': '0006-infrastructure-operations.md'}

def link(i, extra=''):
    a, ln = oq[i]
    return f'[ADR {a}]({FILES[a]}) {extra}line {ln}'

def row(code, q, d, where):
    return f'| {code} | {q} | {d} | {where} |'

# question / decider changes for existing rows (README line -> (new question or None, new decider or None))
CHANGE = {
    35: ("How does the stress scene's sim load reach the 60 Hz margin run, when the wave-1 sim has no ability and effect "
         "step, economy, objectives or moments yet: (a) the sim core spike's sim inside the proof-test server, each "
         "missing part stood in by burning its ceiling as ADR 0004's S1 does (recommended; the test review's Q1), (b) a "
         "stand-in replaying measured step times, (c) wait until the real sim runs the stress scene, (d) wait until the "
         "content and ability spike's effect step has joined?", None),
    38: ("The asset-pipeline research runs before the asset spike; should it run earlier (for example beside the first "
         "spikes) so art production can start sooner, or so its slice on export checks and slot renaming answers "
         "OQ-SPIKE-1 before the look proof test?", None),
    44: ("Only the netcode proof test's place is settled (before the foundation); is SPIKE_PLAN's proposed order of the "
         "rest accepted, with the builders it names for the shared client harness (the netcode proof test) and the "
         "measurement harness's CPU side (the boundary spike)?", None),
    47: ("How are the per-hostname production certificates issued, how long do they live, and which certificate does "
         "the WebSocket fallback present on development and staging servers, since a WebSocket cannot pin a "
         "certificate hash? The netcode proof test's WebSocket leg needs the development and staging answer first, and "
         "a trusted certificate there needs a domain (OQ-0006-15).",
         "security-auditor in the security baseline, with the port plan (OQ-0006-16); the development and staging case "
         "before the netcode proof test starts (the Advisor, from a short read)"),
    50: ("The rewind cap: which value (200 ms, 250 ms, or one set from the proof test's measured honest rewind), and the "
         "victim's side?", None),
    51: ("Do shots and the active-reload tap carry their moment inside a tick at 30 Hz?",
         "for shots, the netcode proof test, then Mustafa in the Feel Lab if it fails or feels late; for the reload tap, "
         "Mustafa in the Feel Lab"),
    59: (None, "this record, from the macOS job's wall time, which infra-engineer measures in the foundation (real Safari "
               "costs runner time, not money, while the repository is public); ADR 0006 and Mustafa only if the "
               "repository stops being public"),
    98: (None, "a measurement made before the reference server is created (a small check at the start of the netcode "
               "proof test's setup), confirmed by Mustafa"),
    102: ("Will any CI runner be paid for: GPU, self-hosted reference machines, or a cache above 10 GB? (Real Safari on "
          "standard macOS runners costs nothing while the repository is public.)", None),
}
# new rows, inserted after the README line given
NEW = {
    38: [row('Look test models (OQ-SPIKE-1)',
             "Who makes the look proof test's authored models (a skinned soldier with faction colour slots, the houses, a "
             "very large unit of at least 20 parts, as glTF/GLB with named slots), and by which route, since whether "
             "Spline's export carries a skin and named slots is not researched? (the graphics review's Q2)",
             "Mustafa and the Advisor, with ASSET-RESEARCH's timing",
             '[SPIKE_PLAN](../SPIKE_PLAN.md), "Open questions"; [ADR 0003](0003-rendering.md) part 11')],
    56: [row('OQ-0001-15', "Is the local player drawn from the blend of the two newest blocks, one tick behind its "
             "prediction (33.3 ms at 30 Hz), or from the newest predicted tick?",
             'Mustafa, by feel, with netcode-architect', link('OQ-0001-15')),
         row('OQ-0001-16', 'How does each network profile split its latency and jitter over the two directions (half '
             'each way is proposed)?', 'the Advisor, before the proof test starts', link('OQ-0001-16')),
         row('OQ-0001-17', "Does a host's kick bar that player from the room for the rest of the match?",
             'Mustafa (what a host may do, with R2-ROOM-OQ5)', link('OQ-0001-17')),
         row('ROOM-CRASH (OQ-0001-18)', "When a room's simulation panics mid-match, does the match end (proposed), or "
             'does the room restore its last snapshot and continue?', 'the Advisor', link('OQ-0001-18'))],
    71: [row('OQ-0003-16', 'The look floor: which LOOK_V2 items may a quality tier switch off on the minimum laptop? '
             "(the graphics review's Q1, does every `[DECIDED]` item hold at Low; the art review's LOOK-FLOOR)",
             'Mustafa, in the look proof test, on the Low frame and the techniques\' measured costs', link('OQ-0003-16')),
         row('OQ-0003-17', 'How is VISION_V2 §6.9\'s "without dropping frames" read as a mark: how many of 600 '
             'presented frames may miss a refresh with the very large units on screen, or the p95 rows as written?',
             'the Advisor, confirmed by Mustafa at the stop review (VISION_V2 is theirs)', link('OQ-0003-17')),
         row('OQ-0003-18', 'Do the concussion shake, the halftone flashes and line boil soften for players who ask for '
             "reduced motion? (the art review's REDUCED-MOTION)", 'Mustafa, in the look proof test (parts 5.l, 5.m)',
             link('OQ-0003-18')),
         row('OQ-0003-19', "Are the three factions told apart by colour alone? (the art review's COLOUR-TELL)",
             'Mustafa, in the look proof test', link('OQ-0003-19')),
         row('OQ-0003-20', "Is the minimum laptop's budget judged after 15 minutes of play, or on a cold laptop? (the "
             "test review's HEAT-SOAK)", 'the Advisor', link('OQ-0003-20'))],
    73: [row('OQ-0004-8', 'How does an uplink steal meet defenders, and two enemy teams?',
             'Mustafa, with the game-designer', link('OQ-0004-8')),
         row('OQ-0004-9', "Is a carried tech cache's position public?", 'Mustafa', link('OQ-0004-9')),
         row('OQ-0004-10', 'Does a player who rejoins after the grace window get their player record back?',
             "Mustafa, with OQ-0005-AB10 (what a human gets on taking a bot's slot)", link('OQ-0004-10')),
         row('OQ-0004-11', "When a pool is full, which row gives way? (ADR 0005 AB-1 proposes the rule per kind; the "
             "reviews' POOL-FULL)", 'the game-designer proposes per kind; Mustafa confirms where a player could notice',
             link('OQ-0004-11')),
         row('OQ-0004-12', 'Do bots take world pickups?', 'Mustafa', link('OQ-0004-12'))],
    89: [row('OQ-0005-AB14', 'What does spawn protection cover, and what ends it early?',
             "Mustafa (the 3 s is the Advisor's `[DEFAULT]`; what it covers and what ends it are new rules)",
             link('OQ-0005-AB14')),
         row('OQ-0005-AB15', "Who sees a player's upgrade picks?", 'Mustafa', link('OQ-0005-AB15')),
         row('OQ-0005-AP1', 'Where does the automated asset step run, what does it write, and where do the model files '
             'live (which decides who can see unreleased art)?', "the asset-pipeline research, then the asset spike; "
             "the files' visibility follows Mustafa's answer to OQ-0006-18, due before the first authored model is "
             'committed', link('OQ-0005-AP1'))],
    106: [row('OQ-0006-13', "How is the production player build kept out of the preview deploy token's reach: a "
              "required reviewer on `preview`, production in a second Cloudflare account, or a token limited to the "
              'preview project?', 'the Advisor, with security-auditor; Mustafa for a second account and for the '
              'approvals (a) asks of him', link('OQ-0006-13')),
          row('OQ-0006-14', 'How do releases reach production while rooms run, and how is a bad server release rolled '
              'back?', 'the Advisor for the model; Mustafa for when players may be refused', link('OQ-0006-14')),
          row('OQ-0006-15 (DOMAIN)', 'Which domain do the client host, the lobby and the game servers use, and when is '
              'it registered?', "Mustafa (an account and a cost), after the Advisor's OQ-0006-5 read",
              link('OQ-0006-15')),
          row('OQ-0006-16', 'The port plan: which UDP and TCP ports do rooms listen on, and must the WebSocket fallback '
              'be reachable on TCP 443?', "infra-engineer with netcode-architect (ADR 0001's transport), confirmed by "
              'the Advisor', link('OQ-0006-16')),
          row('OQ-0006-17 (SPIKE-VM-ACCESS)', 'How do sessions get VMs for the spikes, the tick bench and the 3-hour '
              'soak before each playtest without holding a secret?', 'Mustafa (his account), with the Advisor',
              link('OQ-0006-17')),
          row('OQ-0006-18 (UNRELEASED-ART)', 'Must unreleased art stay unseen, or is a preview behind a login enough?',
              "Mustafa, with OQ-0005-AP1 (where model files live), before the first authored model is committed",
              link('OQ-0006-18')),
          row('OQ-0006-19', 'Is the goal budget provisional until slice 6 measures rooms per VM with the real sim? (the '
              "test review's Q2)", "Mustafa (money), on the Advisor's advice", link('OQ-0006-19')),
          row('OQ-0006-20', "What may the netcode proof test's VMs cost?", 'Mustafa (money)', link('OQ-0006-20'))],
}

L = snap(R).split('\n')
for n in range(34, 107):
    l = L[n - 1]
    assert l.startswith('| '), n
    c = [x.strip() for x in l.split('|')[1:-1]]
    assert len(c) == 4, (n, len(c))
    ids = re.findall(r'OQ-\d{4}-[A-Z]*\d+', c[0])
    q, d, w = c[1], c[2], c[3]
    if n in CHANGE:
        nq, nd = CHANGE[n]
        q = nq or q; d = nd or d
    if ids:
        a, ln = oq[ids[0]]
        w2, k = re.subn(r'(\[ADR ' + a + r'\]\([^)]*\)[^|;]*?\bline )\d+', lambda m: m.group(1) + str(ln), w)
        assert k == 1, (n, w)
        w = w2
    new = row(c[0], q, d, w)
    if n in NEW:
        new = new + '\n' + '\n'.join(NEW[n])
    if new != l:
        fid = f'README decisions row {n} ({c[0].split(" ")[0] if ids else c[0]}): re-pinned at 604a497' + \
              (', question or decider updated' if n in CHANGE else '') + (', new rows after it' if n in NEW else '')
        fix(fid, R, n, n, after=new)
log('README decisions table (rows 34-106) re-pinned at 604a497 with changed questions/deciders and new rows (gen_readme_table.py)')
