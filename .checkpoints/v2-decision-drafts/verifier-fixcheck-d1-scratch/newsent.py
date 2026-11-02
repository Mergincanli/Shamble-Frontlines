# Lists sentences present in HEAD's doc but not in base's doc, with HEAD line numbers.
import subprocess, re, sys
repo='/home/user/Shamble-Frontlines'
def show(rev, path):
    return subprocess.run(['git','-C',repo,'show',f'{rev}:{path}'],capture_output=True,text=True).stdout
def sentences(text):
    lines=text.split('\n')
    # build a char stream with line mapping; treat blank lines / table rows / headings as hard breaks
    out=[]  # (sentence, startline)
    buf=''; start=None
    def flush():
        nonlocal buf,start
        s=re.sub(r'\s+',' ',buf).strip()
        if s: 
            # split into sentences
            parts=re.split(r'(?<=[.!?:;])\s+(?=[A-Z(\[`*"0-9])',s)
            for p in parts:
                out.append((p.strip(),start))
        buf=''; start=None
    for i,l in enumerate(lines,1):
        if not l.strip() or l.startswith('#') or l.lstrip().startswith('|'):
            flush()
            if l.lstrip().startswith('|') or l.startswith('#'):
                # table row: split cells
                for c in l.split('|'):
                    c=c.strip()
                    if c: out.append((c,i))
            continue
        if re.match(r'\s*([-*]|\d+\.)\s',l): flush()
        if start is None: start=i
        buf+=' '+l
    flush()
    return out
base,head,path=sys.argv[1],sys.argv[2],sys.argv[3]
b=set(re.sub(r'\s+',' ',s) for s,_ in sentences(show(base,path)))
for s,ln in sentences(show(head,path)):
    if re.sub(r'\s+',' ',s) not in b:
        print(f'{ln}\t{s}')
