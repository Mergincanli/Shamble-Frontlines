import sys
p='/home/user/Shamble-Frontlines/.checkpoints/v2-decision-drafts/verifier-fixcheck-d0-verdicts.md'
row=open(sys.argv[1]).read().strip('\n')
s=open(p).read()
m='\n\n## Fetch audit'
i=s.index(m)
s=s[:i]+'\n'+row+s[i:]
open(p,'w').write(s)
