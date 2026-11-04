import sys,re
def block(p):
    t=open(p,encoding='utf-8').read()
    i=t.index('## Saving your progress'); j=t.index('```',i)
    lines=[l.strip() for l in t[i:j].splitlines()]
    return ' '.join(l for l in lines if l)
b=block(sys.argv[1]); h=block(sys.argv[2])
ins="Trust the blocks they mark done: never reread, recheck or redo them (a later block may still read a file it needs). "
assert h.count(ins)==1
h2=h.replace(ins,'')
end_new="then give your answer from your notes alone, without opening any file again. Never commit or push."
end_old="then give your answer. Never commit or push."
assert h2.count(end_new)==1
h2=h2.replace(end_new,end_old)
print("other sentences identical:", h2==b)
# sentence lists
sb=re.split(r'(?<=[.:;])\s+',b); sh=re.split(r'(?<=[.:;])\s+',h)
print("base sentences",len(sb),"head",len(sh))
print("in base not head:",[s for s in sb if s not in sh])
print("in head not base:",[s for s in sh if s not in sb])
