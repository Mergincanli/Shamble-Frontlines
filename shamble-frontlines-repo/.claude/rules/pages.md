---
paths:
  - "**/index.html"
  - "**/editor.html"
---
# Rules for the two pages
- index.html (player: THREE → core → game) and editor.html (+ editor) are identical except the manifest comment, the
  "Map editor" tab (`mEditor`), the editor script tag and `bootEditor();`. Apply every CSS/DOM change to BOTH.
- The inline script is only `bootGame();` [`bootEditor();`] plus the menu tail. Logic belongs in shamble-game.js.
- War-room theme: classes, CSS tokens and keyframes are prefixed `wr-`. Never animate `transform` on a `.wr-sheet`
  (its halftone `::before` would paint over the paper); animate a wrapper instead.
- Overlay z-order: HUD 10 · pause 19 (field-manual mode 21) · title 20 · scoreboard/shop 30 · career 31 · tutorial 33 · dev 40.
- Dynamic or player-provided text goes in via `textContent`, never `innerHTML`.
