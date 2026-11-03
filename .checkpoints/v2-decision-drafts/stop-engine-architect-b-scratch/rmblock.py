import sys, re, pathlib
p = pathlib.Path('/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/stop-engine-architect-b-fixes.md')
t = p.read_text(encoding='utf8')
for bid in sys.argv[1:]:
    m = re.search(r'^### (?:FIX|NOTE) ' + re.escape(bid) + r'\n.*?^END\n\n', t, re.S | re.M)
    assert m, bid
    t = t[:m.start()] + t[m.end():]
    print('removed', bid)
p.write_text(t, encoding='utf8')
