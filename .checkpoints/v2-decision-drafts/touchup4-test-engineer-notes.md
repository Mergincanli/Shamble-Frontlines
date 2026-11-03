# touchup4 test-engineer notes

- 08:57 started: D3E-2, D3E-3 (PM-0003-2 in docs/adr/0003-rendering.md), D3C-1 (ADR 0001 netcode checklist step 7 wrap). Next: read verdicts + excerpts.
- 09:05 checked D3E-2/D3E-3 against W1 excerpts (webgpu-excerpt.raw.txt 11384-11388, excerpt2 11126-11128, 11267-11276; destroy 2327-2329): both hold. D3E-2 refinement: the scope alone proves the plant errs, not hook vs browser; add a bare listener. D3E-3: close count before teardown (covers a mid-LS destroy, which 'unknown' only would miss). One combined FIX (same line 1136). D3C-1: 0001:1518 is 156 chars (step 7 1517-1520); also 1526 (152) and 1528 (136) in steps 10-11, same author commit d1f7e13: extra FIX D3C-1b. Next: compose FIX D3E-2+D3E-3.
