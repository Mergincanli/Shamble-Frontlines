import re,sys
D='.checkpoints/v2-decision-drafts/'
files=['touchup-test-engineer-fixes.md','touchup2-game-designer-fixes.md','touchup2-engine-architect-synthesis-fixes.md','rust-reviewer-touchup-d3a-fixes.md','test-engineer-touchup-pm22-fixes.md','touchup3-engine-architect-repin-fixes.md']
for f in files:
    t=open(D+f).read()
    for m in re.finditer(r'### FIX ([^\n]*)\nFILE: ([^\n]*)\nBEFORE:\n(.*?)\nAFTER:\n(.*?)\nEND', t, re.S):
        name,path,before,after=m.groups()
        doc=open(path).read()
        a=doc.count(after+'\n'); b=doc.count(before+'\n')
        bin_after = before in after
        print(f"{f[:38]:38} {name[:34]:34} {path.split('/')[-1][:22]:22} after_in_doc={a} before_in_doc={b}{' (before inside after)' if bin_after else ''}")
