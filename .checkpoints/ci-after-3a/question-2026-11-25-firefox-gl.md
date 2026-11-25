# Question for Mustafa, 2026-11-25: FIREFOX-GL (Firefox gets no WebGL in this container, even with the library)

## What was tried (his EGL-TRIAL (a) of 12:15 Amsterdam)
- The two files fetched from Ubuntu's archive, sizes 28,670 and 117,190 bytes and both sha256 equal to his values
  (`sha256sum -c`: OK), unpacked with `dpkg-deb -x` into `.gate/egl/x` (libEGL.so.1, libEGL_mesa.so.0, Mesa's EGL
  vendor file), handed to Firefox only through its environment (`LD_LIBRARY_PATH`, `__EGL_VENDOR_LIBRARY_FILENAMES`).
- The Firefox run again (full limits, as CI would run it; log `.gate/trial/firefox-3.log`): the 2 rule tests pass;
  every drawing test fails as before with the page's own line "This page could not start drawing: can't access
  property "getSupportedExtensions", this.gl is null"; 7 did not run before Playwright's 5-minute stop.
- Read first (his point 4): a probe of my own (`probe-webgl.mjs` in the scratchpad, never committed) opens a blank
  page with no game code and asks Firefox for a WebGL 2 context, printing Firefox's own reason:
  - default preferences, with or without the library: "AllowWebgl2:false restricts context creation on this system."
    (Firefox's own list of systems it trusts refuses WebGL 2 on this machine);
  - with `webgl.force-enabled` (that refusal switched off), with or without the library, and also with Mesa told to
    use software rendering: "tryNativeGL () / Exhausted GL driver options. (FEATURE_FAILURE_WEBGL_EXHAUSTED_DRIVERS)".
  - This container has no graphics device at all (`/dev/dri` absent).
- So the cause is this container's graphics, not the game's page (a blank page fails the same way) and not
  Playwright's route (it sets no graphics preference). Chrome draws here only because it carries its own software
  renderer (SwiftShader); Firefox has none of its own and needs the system's OpenGL to work.
- Not known: whether Firefox gets WebGL on GitHub's runner (also no graphics card; its Mesa set is not listed).

## FIREFOX-GL: how does Firefox enter CI?
- (a) Ask 2 brings the full Firefox run (13 tests) with the read-only line he asked for (does the runner have
  libEGL.so.1), and the first GitHub run is the drawing tests' trial. If they fail there for the same reason, it comes
  back to him as its own question, with (b) as the ready option. Cost: likely one more round of CI text.
- (b) Firefox in CI runs the 2 rule tests only for now (the Firefox entry in the test configuration matches `rules/`
  only): sure to pass, and the vectors are checked in Firefox's own WebAssembly on every push; the drawing tests in
  Firefox become a follow-up that needs a graphics route for Firefox on the runner (its own question to him).
- (c) More digging here first (Firefox's graphics logs, more preferences): open-ended, and what this container lacks
  may not be what the runner lacks.
Recommendation: (a), since only the runner can say whether Firefox draws there, and his stop rule already keeps a red
run from turning into an unasked fix; (b) if he prefers a pull request that is green on its first run.
