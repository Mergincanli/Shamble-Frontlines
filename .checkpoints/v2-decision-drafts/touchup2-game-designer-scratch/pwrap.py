import sys, textwrap
sys.path.insert(0,'.')
from wrap import L
NB=' '
def pwrap(text, first, rest, protect, width=120):
    for p in protect:
        assert p in text, p
        text=text.replace(p, p.replace(' ',NB))
    w=textwrap.wrap(text,width=width,initial_indent=first,subsequent_indent=rest,break_long_words=False,break_on_hyphens=False)
    return [x.replace(NB,' ') for x in w]
