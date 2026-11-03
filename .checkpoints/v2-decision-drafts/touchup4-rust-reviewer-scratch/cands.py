D3B = ('equal an exact reference', 'equal a correctly rounded reference')
D3Bs = ('equal an exact reference', 'equal a rounded exact reference')
CIT = 'as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03). A'
CANDS = {
 'n1': [D3B, (CIT, 'as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03); so does the pin, 1.99.1, by the same steps (verifier fixcheck-d3e, 2026-11-03). A')],
 'n2': [D3B, ('(PM-0004-22, extended)', '(PM-0004-22)'), (CIT, 'as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03); so does the pin, 1.99.1 (verifier fixcheck-d3e, 2026-11-03). A')],
 'n3': [D3B, ('yet 1.97.0\'s `div_euclid` gives 3', 'yet 1.97.0\'s and 1.99.1\'s `div_euclid` give 3'), ('fixcheck-d3a and skeptic-6, 2026', 'fixcheck-d3a, skeptic-6 and fixcheck-d3e, 2026')],
 'n4': [D3B, ('yet 1.97.0\'s `div_euclid` gives 3', 'yet 1.97.0\'s `div_euclid`, like the pin 1.99.1\'s, gives 3'), ('fixcheck-d3a and skeptic-6, 2026', 'fixcheck-d3a and skeptic-6, and for 1.99.1\'s source fixcheck-d3e, 2026')],
 'n5': [D3B, ('yet 1.97.0\'s `div_euclid` gives 3', 'yet 1.97.0\'s `div_euclid`, like the pin\'s (1.99.1), gives 3'), ('fixcheck-d3a and skeptic-6, 2026', 'fixcheck-d3a and skeptic-6; fixcheck-d3e read 1.99.1\'s source; 2026')],
}
