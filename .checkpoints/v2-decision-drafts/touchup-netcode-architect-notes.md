# touchup-netcode-architect notes

- 21:12 started; output file created; next: read fixcheck-d1 findings F1, F2, F13.
- 21:20 F2 checked: holds (part 4 never predicts picks/own abilities, 0001:191-193; PM-0001-60 0001:1428 marks at most one correction at the first snapshot). F1 checked: holds, but exact figure is 4.69 ms (16.67-9.18-2.80; unrounded 16.6667-8.3333-3.6435=4.6899), 3.69 re-flood; F1's 4.70 is a sum of rounded parts. Coordinator asked: keep line counts equal to BEFORE (rewrap). Next: write F2 block, then F1, then F13.
- 21:27 F2 FIX block written (lines 189-195 rewrapped, 7 lines -> 7; adds "after the first snapshot that carries it", PM-0001-60's words). Next: F1 block.
