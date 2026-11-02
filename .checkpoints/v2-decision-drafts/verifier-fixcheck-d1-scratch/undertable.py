import subprocess,sys
repo='/home/user/Shamble-Frontlines'
def show(rev,p): return subprocess.run(['git','-C',repo,'show',f'{rev}:{p}'],capture_output=True,text=True).stdout.split('\n')
for p in sys.argv[2:]:
    L=show(sys.argv[1],p)
    for i in range(len(L)-1):
        if L[i].lstrip().startswith('|') and L[i+1].strip() and not L[i+1].lstrip().startswith('|'):
            print(f'{p}:{i+2}: {L[i+1][:120]}')
        # also check sentence immediately before table without blank
        if L[i+1].lstrip().startswith('|') and L[i].strip() and not L[i].lstrip().startswith('|') and False:
            pass
