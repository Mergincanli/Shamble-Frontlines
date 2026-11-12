# The two console lines for Mustafa's saves backup (plan 1.8 step 1; for security-auditor to read first)

Both run in Brave's DevTools Console (F12, Console; type `allow pasting` first if Brave asks) on the page whose saves
they touch. They read and write only that page's localStorage (the game's KVStore backend under `file://`), make no
network request, and delete nothing.

## 1. Backup (copy every saved key as one text)
Paste on your current `index.html` (and again on your current `editor.html`), then paste the clipboard into a new text
file outside the repository and save it:

```js
(()=>{const o={};for(const k of Object.keys(localStorage).sort())o[k]=localStorage.getItem(k);copy(JSON.stringify(o));return Object.keys(o).length+' keys copied: paste them into a text file now'})()
```
Expected: the console answers `"<n> keys copied: paste them into a text file now"`; the text file starts with `{` and
holds your keys (`shamble_prog`, `vmap:…` for each map). `copy()` is the DevTools console's own clipboard helper.

## 2. Restore (put back only keys that are missing; never overwrites)
Only if a save is missing after the move: open the page (`legacy/index.html` or `legacy/editor.html`), type the line
below without pressing Enter, replace `PASTE_HERE` with the whole text of your backup file, then press Enter:

```js
((o)=>{let n=0,s=0;for(const k of Object.keys(o)){if(localStorage.getItem(k)===null){localStorage.setItem(k,String(o[k]));n++}else s++}return n+' keys restored, '+s+' already there (left as they are)'})(PASTE_HERE)
```
Expected: `"<n> keys restored, <m> already there (left as they are)"`. Reload the page to see the restored saves.

## Proof (headless Chromium 1194, `file://`): `saves-lines-test.mjs`, results in `saves-lines-result.json`.
