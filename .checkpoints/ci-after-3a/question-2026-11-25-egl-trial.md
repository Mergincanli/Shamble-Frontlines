# Question for Mustafa, 2026-11-25: EGL-TRIAL (Firefox needs a graphics library this container lacks)

## What the trial showed (Firefox 161.0, his TRIAL-DOWNLOAD (a); Playwright started through node, his PW-RUNS (a))
- The test lists: Chromium 14, unchanged; Firefox exactly the 13 planned (the 14 less the allocation spec).
- In Firefox: the two rule tests pass in Firefox's own WebAssembly (release 669 ms, dev 876 ms): the float vectors, the
  stream's vectors and the planted overflow with the hook's text. That is 3A-OTHER-BROWSERS' Firefox half.
- The other 11 fail, all for one reason, shown on the page itself: "This page could not start drawing: can't access
  property "getSupportedExtensions", this.gl is null". The page finds no WebGL in this headless Firefox, so it stops
  as it should, and every test that waits for blocks or for its own notice times out or reads that line instead
  (logs: `.gate/trial/firefox-1.log`, `firefox-2.log`, not committed).
- Why: headless Firefox makes WebGL through Mesa's EGL library, and this container has Mesa's GLX parts but no EGL
  (`libEGL.so.1` absent; `libegl1` and `libegl-mesa0` not installed). Playwright sets no graphics preference of its
  own for this route (its bundle passes only the user's prefs). So the cause is this container, not Playwright's route
  (no fallback under FIREFOX-SOURCE (c)) and not the game's code (the page reports a missing WebGL correctly).
- Not known: whether GitHub's runner image has EGL. Its readme and toolset list do not name it.

## The two files a container trial would need (Ubuntu's own archive, its signature checked here with the archive key
that ships in this container, `/usr/share/keyrings/ubuntu-archive-keyring.gpg`: "Good signature from "Ubuntu Archive
Automatic Signing Key (2018) <ftpmaster@ubuntu.com>"", key F6ECB3762474EDA9D21B7022871920D1991BC93C; each index's
sha256 equals its signed line)
1. `libegl1_1.7.0-1build1_amd64.deb`, version 1.7.0-1build1, from
   `http://archive.ubuntu.com/ubuntu/pool/main/libg/libglvnd/libegl1_1.7.0-1build1_amd64.deb`, 28,670 bytes, sha256
   e549f7776216f7bd3b1c216729fb40a97a562e2eb7c563cde632b4931f9a4e57 (index: noble, main; its Packages.gz 1,808,488
   bytes, sha256 e0d7e4cbb09d2aa7f9e104a1488817417bc9d85f3e5d9a21156a52ec641ae531).
2. `libegl-mesa0_25.2.8-0ubuntu0.24.04.5_amd64.deb`, version 25.2.8-0ubuntu0.24.04.5 (the same as this container's
   Mesa), from `http://archive.ubuntu.com/ubuntu/pool/main/m/mesa/libegl-mesa0_25.2.8-0ubuntu0.24.04.5_amd64.deb`,
   117,190 bytes, sha256 c04cdb8c4d29667ab9d2c3abac6bb38dc066f5cdb874d867f85f5e238d546f82 (index: noble-updates, main;
   its Packages.gz 1,842,441 bytes, sha256 3019495e081377487274cd37921d341802c6e79b20982ff8067c2e1cd789541d).
The archive serves them over plain http, as apt does; the proof is the signed index, and each file's size and sha256
are checked before use. Every package they depend on is installed here already. The route: fetched into a new folder
under `.gate/`, checked, unpacked with `dpkg-deb -x` (no apt, no root, no install script), and handed to the trial's
Firefox only, through its environment (the library folder and Mesa's EGL vendor file); nothing in the system changes.

## EGL-TRIAL: how is the Firefox drawing half tried?
- (a) Yes to the two files above for this container's trial only. CI uses the runner image's own libraries; if the
  image lacks EGL, the first GitHub run fails the same way, and that comes back to him (stop rule).
- (b) No container trial for the 11 drawing tests: ask 2 brings the text with the rule tests tried here, and the first
  GitHub run is the trial for the rest (a red is read, not re-run).
- (c) Firefox in CI runs only the rule tests for now (the configuration's Firefox entry matches `rules/` only); the
  drawing tests in Firefox come in a later pull request.
Recommendation: (a). It is small (145,860 bytes in all), from the archive the container itself uses, and it makes the
trial show whether the page draws in Firefox before he sees the text.
