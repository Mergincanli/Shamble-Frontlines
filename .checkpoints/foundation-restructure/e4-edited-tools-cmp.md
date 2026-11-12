# E4 addendum: the head's edited tools give the same default-map dump (2026-11-11, step 9's closer)

$ cmp .gate/foundation-restructure/e6/tuning-cabana.json .gate/foundation-restructure/e4/a-cabana-1.json
exit 0: identical, 35794 bytes each

e6/tuning-cabana.json: E6's tuning-dump row, run with the head's tools/preview.js and tools/tuning-dump.js (header comments edited in 1e6f84e).
e4/a-cabana-1.json: E4's dump at A (76b4f7a) with A's copies of both tools. Default map only; the tri map was not run with the edited tools.
