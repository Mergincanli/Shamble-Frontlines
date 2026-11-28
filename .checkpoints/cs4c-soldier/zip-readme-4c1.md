# Shamble v2, change-set 4c-1: the page's scene (development build)

Built from commit FINAL_HEAD of the branch `claude/ecstatic-wozniak-5cbdzs`, the head the scene's pull request is
opened at. What it shows: the test scene of 4b (the street corner and the training yard) drawn as plain grey blocks,
height read through brightness, the street in a grey of its own. No soldier yet: he comes in 4c-2.

## Open it
1. Unzip into a folder of its own.
2. In that folder, run `node serve.mjs` (Node 22.13 or newer). It prints an address such as `http://127.0.0.1:PORT/`.
3. Open that address in Brave. Opening `index.html` as a file does not run: the page says so.

## Check (about 6 minutes, Brave, then once in Firefox)
1. Open the page in a normal Brave window. **Expected:** the whole map in one view from above at an angle (the
   street corner on the left, the training yard on the right); the readout's first line names the backend ("backend:
   WebGPU (...)" on most Windows machines; "WebGL 2 fallback, because ..." is also fine, just say which); "errors: 0".
   Keep the window maximised (at least about 600 px tall): on a shorter window the black panel lies over the houses
   and the yard's back row and darkens them.
2. Look at the blocks. **Expected:** two houses on the left with the narrow alley between them, each a bright slab
   (the near side of its roof) over a darker front wall, with the slanted end of the gable on its right; the ridge
   itself does not show from this angle. A low cover wall on the grass just past the street, below them. In the yard:
   two small dark squares (the 0.55 m and 0.56 m steps, the same grey), three ledges just in front of them (the same
   light grey), a thin wall like a line, two platforms with their ramps (the ramp a little darker than its platform),
   six thin posts along the lane at the bottom right. Height reads in steps of brightness: steps darkest, then
   platforms, then ledges, then the roofs lightest; blocks within about 0.2 m of each other look the same. On every
   block the top is lighter than its sides; a tall block's wall can look as dark as a low step's top (only the tops
   carry height).
3. Look at the ground. **Expected (your STREET-GREY (b)):** two strips a little darker than the grass around them:
   the street from the left edge to the middle, between the houses and the cover wall, and the yard's lane at the
   bottom right, under the posts. The difference is small but should be plain; if you want it stronger or weaker, say
   so (it goes into the soldier's pull request).
4. Read the readout's two new lines. **Expected:** "content: maps/test-scene.json shamble-map 1 ·
   movement/soldier.json shamble-movement 1" and "scene: test-scene · 17 boxes · 6 ramps · 6 regions".
5. Press "Copy report" and paste it into the chat. **Expected:** the same rows (`content.*`, `scene.*`), "errors: 0",
   the render counts.
6. Open the same page once in Firefox. **Expected:** the same scene, greys and rows, "errors: 0"; say its backend
   line. If Brave and Firefox both say WebGPU, open the page once more in Brave with `?backend=webgl` added to the
   address (the first line then reads "backend: WebGL 2 fallback, asked for by ?backend=webgl ...") and check the
   greys look the same (the pictures made in the build machine could only show WebGL 2).

The steps are art-director's, from its review of the scene (`review-s-art-director.md` in the branch's notes).
