import sys, textwrap
P='/home/user/Shamble-Frontlines/docs/adr/0005-content-abilities-assets.md'
L=open(P,encoding='utf-8').read().split('\n')
def wrap(text, first, rest, width=120):
    return textwrap.wrap(text, width=width, initial_indent=first, subsequent_indent=rest, break_long_words=False, break_on_hyphens=False)
if __name__=='__main__':
    a,b=int(sys.argv[1]),int(sys.argv[2])
    for i in range(a,b+1): print(i,len(L[i-1]),L[i-1])
