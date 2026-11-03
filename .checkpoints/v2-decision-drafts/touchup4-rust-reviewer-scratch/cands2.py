D3B = ('equal an exact reference', 'equal a correctly rounded reference')
D3Bs = ('equal an exact reference', 'equal a rounded exact reference')
EXT = ('(PM-0004-22, extended)', '(PM-0004-22)')
CIT = 'as does its `f64` analogue (verifiers fixcheck-d3a and skeptic-6, 2026-11-03). A'
def add(s): return (CIT, CIT[:-3] + s + '. A')
CANDS = {
 'm1': [D3B, EXT, add(', and so does the pinned 1.99.1, by the same steps (verifier fixcheck-d3e, 2026-11-03)')],
 'm2': [D3Bs, EXT, add(', and so does the pinned 1.99.1, by the same steps (verifier fixcheck-d3e, 2026-11-03)')],
 'm3': [D3B, EXT, add('; so does the pin, 1.99.1 (its source: verifier fixcheck-d3e, 2026-11-03)')],
 'm4': [D3B, add('; so does the pin, 1.99.1 (verifier fixcheck-d3e, 2026-11-03)')],
 'm5': [D3B, EXT, add('; so does the pin, 1.99.1 (verifier fixcheck-d3e, 2026-11-03)')],
 'm6': [D3B, EXT, add(', and so does the pin, 1.99.1, by the same steps (verifier fixcheck-d3e, 2026-11-03)')],
 'm7': [D3Bs, add(', and so does the pin, 1.99.1 (verifier fixcheck-d3e, 2026-11-03, from its source)')],
}
