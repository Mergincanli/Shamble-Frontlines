D3B = ('equal an exact reference', 'equal a correctly rounded reference')
D3Bs = ('equal an exact reference', 'equal a rounded exact reference')
EXT = ('(PM-0004-22, extended)', '(PM-0004-22)')
Y = "yet 1.97.0's `div_euclid` gives 3"
F64 = ('as does its `f64` analogue', 'as do their `f64` analogues')
def cit(s): return ('fixcheck-d3a and skeptic-6, 2026', s + ', 2026')
CANDS = {
 'p1': [D3B, (Y, "yet 1.97.0's and the pinned 1.99.1's `div_euclid` give 3"), F64, cit('fixcheck-d3a, skeptic-6 and fixcheck-d3e')],
 'p2': [D3B, EXT, (Y, "yet 1.97.0's and the pinned 1.99.1's `div_euclid` give 3"), F64, cit('fixcheck-d3a and skeptic-6, and fixcheck-d3e from 1.99.1\'s source')],
 'p3': [D3B, (Y, "yet 1.97.0's and the pinned 1.99.1's `div_euclid` give 3"), F64, cit('fixcheck-d3a and skeptic-6, and fixcheck-d3e from 1.99.1\'s source')],
 'p4': [D3B, (Y, "yet 1.97.0's `div_euclid`, like the pinned 1.99.1's, gives 3"), cit('fixcheck-d3a, skeptic-6 and fixcheck-d3e')],
 'p5': [D3B, EXT, (Y, "yet 1.97.0's `div_euclid`, like the pinned 1.99.1's, gives 3"), cit('fixcheck-d3a and skeptic-6, and fixcheck-d3e from 1.99.1\'s source')],
 'p6': [D3B, (Y, "yet 1.97.0's `div_euclid`, like the pinned 1.99.1's, gives 3"), cit('fixcheck-d3a and skeptic-6, and fixcheck-d3e from 1.99.1\'s source')],
 'p7': [D3B, (Y, "yet 1.97.0's `div_euclid` and the pinned 1.99.1's give 3"), F64, cit('fixcheck-d3a and skeptic-6 running 1.97.0, fixcheck-d3e reading 1.99.1\'s source')],
 'p8': [D3B, EXT, (Y, "yet 1.97.0's `div_euclid` and the pinned 1.99.1's give 3"), F64, cit('fixcheck-d3a and skeptic-6 running 1.97.0, fixcheck-d3e reading 1.99.1\'s source')],
}
P = (Y, "yet 1.97.0's and the pinned 1.99.1's `div_euclid` give 3")
CANDS.update({
 'q1': [D3B, EXT, P, F64, cit("fixcheck-d3a and skeptic-6, and fixcheck-d3e from its source")],
 'q2': [D3B, EXT, P, F64, cit("fixcheck-d3a and skeptic-6, and for 1.99.1 fixcheck-d3e")],
 'q3': [D3B, P, F64, cit("fixcheck-d3a and skeptic-6, and for 1.99.1 fixcheck-d3e")],
 'q4': [D3B, EXT, P, F64, cit("fixcheck-d3a, skeptic-6 and, from 1.99.1's source, fixcheck-d3e")],
 'q5': [D3B, EXT, (Y, "yet 1.97.0's and the pinned 1.99.1's (ADR 0002) `div_euclid` give 3"), F64, cit('fixcheck-d3a, skeptic-6 and fixcheck-d3e')],
})
CANDS.update({'b_only': [D3B], 'bs_only': [D3Bs], 'q2s': [D3Bs, EXT, P, F64, cit("fixcheck-d3a and skeptic-6, and for 1.99.1 fixcheck-d3e")]})
CANDS.update({'q3s': [D3Bs, P, F64, cit("fixcheck-d3a and skeptic-6, and for 1.99.1 fixcheck-d3e")]})
