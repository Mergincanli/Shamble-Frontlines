# security-auditor: the CI pull request after 3a (tier 2), the one review round
Head f876c4b (main 311c048 merged in), worktree `.gate/ci3`, read only. Texts: `draft-toolchain.diff`,
`draft-playwright-config.diff`. Memory: no local agent-memory folder in this container (none to read or write).

## Plan
1. Threat model: what new code or binary runs, with what rights, inside or outside the cut.
2. The image's Firefox inside the cut: planted-code path? Who can write `/usr/lib/firefox` and its parents, before and
   inside the cut; what Playwright's `moz-firefox` launch does (profile, prefs, ports, env), from the locked
   playwright-core; his rule "no Firefox setting that switches off its graphics refusal" against Playwright's own prefs.
3. Python on repository scripts inside the cut (`-I -B`, PATH, CUT-PATH).
4. Cut, root-owned wasm-bindgen tool, token, no-cache rule: anything weakened.
5. The plain-file check vs the wasm-bindgen tool's check (lines 119-123); is the same rigour needed.
6. The comments' truth (header, echo lines, config comment).
7. Findings with severity, file:line, exact fix.

## Evidence blocks (saved as read)
E1. Hashes: toolchain.yml today 56d37fc6...9811; draft diff 6784c2d5...7271; the diff applied to a scratch copy gives
    35c8d11b...d3f3, equal to trials-main-311c048.md:20-21. Inner script (draft lines 149-211): 0 apostrophes, `bash -n` ok.
E2. Diff hunks: all new lines sit inside the single-quoted inner script (diff lines 19-26, 34-40) or the header comment
    (diff 8-10). Nothing new before the cut; `env -i` list (toolchain.yml:142-145), setpriv flags (:141),
    `permissions: contents: read` (:17-18), `cache-mode: none` (:37), `persist-credentials: false` (:53), the GH_TOKEN
    step (:93-101) untouched.
E3. Playwright 1.65.0 BiDi Firefox (node_modules/playwright-core/lib/coreBundle.js): channel table :34982-34986
    (moz-firefox, default Linux /snap/bin/firefox); BidiFirefox :45675-45758: args `--remote-debugging-port=0`
    (loopback BiDi WebSocket), `--headless`, `--profile <tmp>`; env adds MOZ_CRASHREPORTER_NO_REPORT=1 etc. (:45696-45702),
    no sandbox-off variable; non-persistent profile from mkdtemp under os.tmpdir() (:41581), never a user's profile.
    Default prefs :45448-45626: no `webgl.*`, `gfx.*`, `layers.*` or blocklist pref; one graphics-adjacent pref
    `media.sanity-test.disabled: true` (:45581, the media GFX sanity window). Security-lowering test prefs include
    safebrowsing off (:45481-45484) and `security.fileuri.strict_origin_policy: false` (:45603-45605).
E4. The 161 deb (trial copy `.gate/moz-firefox-161/`, `dpkg-deb -c`): all 126 entries root/root; `/usr/lib/firefox/`
    drwxr-xr-x, `/usr/lib/firefox/firefox` -rwxr-xr-x (ELF launcher; `/usr/bin/firefox` is a link to firefox.sh); no
    group- or other-writable file or folder. Links: syspref.js -> /etc/firefox/syspref.js; browser/extensions,
    browser/plugins, distribution/extensions -> /usr/lib/firefox-addons/... This is the 161 package, not the runner's
    160 install (runner modes inferred, not measured).
E5. Generators (crates/devtools/vectors): stdlib only (hashlib, pathlib, re, sys; + math, struct, fractions); `--check`
    returns before any write (make_chacha8_vectors.py:82-85, make_f32_vectors.py:361-364).
E6. CUT-PATH (docs/STATUS_ARCHIVE.md:1308-1310, Low, open): PATH inside the cut is the runner's, whose first folders the
    cut's user can write, so a build script can shadow node, npx, git, sha256sum or Chrome for later lines.
    TOKEN-WORDS (:1311-1313, Low): trigger "the next CI edit, his yes" (this edit); the plan leaves it out (plan.md:46-47).

## Findings (one line each as found)
E7. No session-start prefs: no `moz:firefoxOptions` anywhere in the bundle; `kBandaidFirefoxUserPrefs = {}` (:47722,
    Juggler route only); the draft config passes no `firefoxUserPrefs`. The trial shows WebGL still refused under these
    same Playwright prefs (trials-main-311c048.md:14-16): no setting here switches off the graphics refusal.
E8. Python `-I` on 3.13.18 here: safe_path True, isolated 1, no_user_site 1, ignore_environment 1 (`-I` implies `-P`
    since 3.11, so the runner's 3.12 also keeps the script's folder off sys.path).
E9. Trial log `.gate/trial/firefox-b.log:9,39`: under Playwright's Firefox, file-url.spec.ts printed "the page did not
    run from a file; the browser said: THREE.WebGPURenderer: WebGPU is not available, running under WebGL2 backend" and
    the notice read "This page could not start drawing ... this.gl is null": the page's module scripts RAN from file://
    (consistent with `security.fileuri.strict_origin_policy: false`, E3); only WebGL stopped it.
E10. Runner image (actions/runner-images main, read 2026-11-25 via WebFetch, data only):
    install-firefox.sh: Mozilla Team PPA, apt install firefox, PPA removed, one locale pref (en_US) to syspref.js, no
    chmod. configure-environment.sh: `chmod -R 777 $AGENT_TOOLSDIRECTORY` (/opt/hostedtoolcache).
    configure-system.sh: `chmod -R 777 /usr/share` and `chmod -R 777 /opt`. So Chrome at /opt/google/chrome is
    writable by the cut's user (existing line 193-194, CUT-PATH's class); Firefox's code (/usr/lib/firefox: firefox,
    libxul.so, omni.ja, E4) is outside both trees and stays root 755 (inferred from the deb plus no chmod on /usr/lib;
    not measured on the runner). Firefox's loaders still read /usr/share data at run time (e.g. glvnd EGL vendor JSONs
    under /usr/share/glvnd/egl_vendor.d, which name libraries to load: inferred, not measured).
E11. Fix lines tested here under `bash --norc --noprofile -euo pipefail`: 0 apostrophes, exit 0; missing-ldconfig branch
    prints its own line; present-in-cache branch prints the cache line.

## Threat model (summary)
- New code: the image's Firefox (root-owned, Mozilla Team PPA) and Playwright's BiDi driver path; two repository Python
  scripts; ldconfig/grep. All run inside the cut as the runner's uid, no groups, no_new_privs, no network, `env -i`.
- Rights: none above what repository code already has inside the cut (build scripts, tests, npm). A Firefox exploit or
  a swapped Firefox gains the cut's user, which planted code already is. Firefox's userns sandbox cannot touch the
  cut's netns (owned by the init user namespace). Nothing new before the cut; the token, the root-owned wasm-bindgen
  tool (its folder and parents checked root-owned, :119-123), the no-cache rule and the env list are unchanged (E2).
- Trust in a green run: CUT-PATH's class (already open, Low). New members: PATH-resolved python3, ldconfig, grep.
  Firefox is called by absolute path and its files are root-owned: better than Chrome (/opt is 777).

## Findings
F1 Medium (test fidelity and comment truth, not a cut finding): draft-playwright-config.diff:16-19. "Mozilla's own
   Firefox, the build players run" is not true of the run: Playwright launches it in a fresh profile with ~150 test
   prefs, one of them `security.fileuri.strict_origin_policy: false` (coreBundle.js:45603-45605); E9 shows the page's
   scripts ran from file:// under it. If the runner has WebGL, file-url.spec.ts would record "the page ran from a
   file" for Firefox, a result a player's Firefox may not give (inferred, not measured). Fix: comment text (summary);
   follow-up for file-url.spec.ts:1-3 ("here for Chromium") with the isolation.spec.ts follow-up.
F2 Low: draft-toolchain.diff:35. `|| echo "...not on this runner"` also fires when ldconfig is missing or fails, and
   his EGL-TRIAL item 3 routes "runner lacks it" to a question: a false message misroutes it. Fix: one line (summary).
N1 Note (CUT-PATH, no new finding): python3, ldconfig, grep resolve through the runner PATH (diff:21-23,35); add them
   and E10 (/opt and /usr/share 777; Chrome swappable) to CUT-PATH's record.
N2 Note: plain-file check (diff:36) is enough for what it is (fail by name, refuse a link); same rigour as :119-123 is
   not a boundary here (no pinned bytes; a swap gains nothing; Chrome :193 has none). Optional printed line.
N3 Note: TOKEN-WORDS' trigger is this edit (STATUS_ARCHIVE:1311); name it in ask 2 as deferred or folded in.
N4 Note: header "every crate's release tests" (diff:10) true today (members crates/* = 6, all named, diff:19).
N5 Note: his graphics rule holds (E7). `media.sanity-test.disabled` (:45581) is Playwright's default, media only.
Verdict: FINDINGS (F1 Medium, F2 Low); containment PASS: nothing weakens the cut, the tool, the token or no-cache.
