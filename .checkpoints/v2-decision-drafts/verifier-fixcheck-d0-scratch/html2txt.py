import sys,html,re
from html.parser import HTMLParser
class P(HTMLParser):
    def __init__(s):
        super().__init__(); s.o=[]; s.skip=0
    def handle_starttag(s,t,a):
        if t in('script','style'): s.skip+=1
        if t in('p','div','li','h1','h2','h3','h4','h5','section','br','tr','pre','summary','details','dt','dd'): s.o.append('\n')
    def handle_endtag(s,t):
        if t in('script','style'): s.skip-=1
        if t in('p','div','li','h1','h2','h3','h4','tr','pre'): s.o.append('\n')
    def handle_data(s,d):
        if not s.skip: s.o.append(d)
p=P(); p.feed(open(sys.argv[1],encoding='utf-8',errors='replace').read())
t=''.join(p.o); t=re.sub(r'[ \t]+',' ',t); t=re.sub(r'\n\s*\n+','\n',t)
open(sys.argv[2],'w').write(t)
