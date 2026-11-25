# security-auditor: the CI pull request after 3a (tier 2), the one review round
Head f876c4b (main 311c048 merged in), worktree `.gate/ci3`, read only. Texts: `draft-toolchain.diff`,
`draft-playwright-config.diff`. Memory: no local agent-memory folder in this container (none to read or write).
Line references: "diff:N" = line N of the draft diff; "draft toolchain.yml:N" / "draft config:N" = line N of the file
after the diff is applied (scratch copies in the scratchpad; the workflow copy's sha256 is 35c8d11b...d3f3).

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

## Evidence (saved as read)
E1. Hashes: toolchain.yml today 56d37fc6...9811; draft diff 6784c2d5...7271; the diff applied to a scratch copy gives
    35c8d11b...d3f3, equal to trials-main-311c048.md:20-21. Inner script (draft toolchain.yml:149-211): 0 apostrophes,
    `bash -n` ok.
E2. All new lines sit inside the single-quoted inner script (diff:19-26, 34-40) or the header comment (diff:8-10).
    Nothing new before the cut; `env -i` list (toolchain.yml:142-145), setpriv flags (:141), `permissions: contents:
    read` (:17-18), `cache-mode: none` (:37), `persist-credentials: false` (:53), the GH_TOKEN step (:93-101) untouched.
E3. Playwright 1.65.0 BiDi Firefox (node_modules/playwright-core/lib/coreBundle.js): channel table :34982-34986
    (moz-firefox, default Linux /snap/bin/firefox); BidiFirefox :45675-45758: args `--remote-debugging-port=0`
    (loopback BiDi WebSocket), `--headless`, `--profile <tmp>`; env adds MOZ_CRASHREPORTER_NO_REPORT=1 etc.
    (:45696-45702), no sandbox-off variable; non-persistent profile from mkdtemp under os.tmpdir() (:41581), never a
    user's profile. Default prefs :45448-45626: no `webgl.*`, `gfx.*`, `layers.*` or blocklist pref; one
    graphics-adjacent pref `media.sanity-test.disabled: true` (:45581, the media GFX sanity window). Test prefs that
    lower browser security: safebrowsing off (:45481-45484), `security.fileuri.strict_origin_policy: false`
    (:45603-45605, "Local documents have access to all other local documents").
E4. The 161 deb (trial copy `.gate/moz-firefox-161/`, `dpkg-deb -c`): all 126 entries root/root; `/usr/lib/firefox/`
    drwxr-xr-x, `/usr/lib/firefox/firefox` -rwxr-xr-x (ELF launcher; `/usr/bin/firefox` links to firefox.sh); no
    group- or other-writable file or folder; Firefox's code (firefox, libxul.so, omni.ja, lib*.so) under
    /usr/lib/firefox. Links: syspref.js -> /etc/firefox/syspref.js; browser/extensions, browser/plugins,
    distribution/extensions -> /usr/lib/firefox-addons/.... This is the 161 package, not the runner's 160 install.
E5. Generators (crates/devtools/vectors): stdlib only (hashlib, pathlib, re, sys; + math, struct, fractions); `--check`
    returns before any write (make_chacha8_vectors.py:82-85, make_f32_vectors.py:361-364).
E6. CUT-PATH (docs/STATUS_ARCHIVE.md:1308-1310, Low, open): PATH inside the cut is the runner's, whose first folders the
    cut's user can write, so a build script can shadow node, npx, git, sha256sum or Chrome for later lines.
    TOKEN-WORDS (:1311-1313, Low): trigger "the next CI edit, his yes" (this edit); the plan leaves it out (plan.md:46-47).
E7. No session-start prefs: no `moz:firefoxOptions` anywhere in the bundle; `kBandaidFirefoxUserPrefs = {}` (:47722,
    Juggler route only); the draft config passes no `firefoxUserPrefs`. The trial shows WebGL still refused under these
    same Playwright prefs (trials-main-311c048.md:14-16): no setting here switches off the graphics refusal.
E8. Python `-I` on 3.13.18 here: safe_path True, isolated 1, no_user_site 1, ignore_environment 1 (`-I` implies `-P`
    since 3.11, so the runner's 3.12 also keeps the script's folder off sys.path).
E9. file-url.spec.ts, trial logs: Chromium (`.gate/trial/chromium-b.log:12-13`): "did not run from a file; ... Access to
    CSS stylesheet at file:///... from origin null has been blocked by CORS policy", passes with the page's static
    notice (packages/client/index.html:11-14, `run "node serve.mjs"`). Firefox (`.gate/trial/firefox-b.log:9,34-41`):
    "the browser said: THREE.WebGPURenderer: WebGPU is not available, running under WebGL2 backend", notice replaced by
    "This page could not start drawing ... this.gl is null": the page's module scripts RAN from file:// in Firefox,
    only WebGL stopped them. Consistent with `strict_origin_policy: false` (E3); that a player's Firefox (pref true)
    would block them like Chromium is inferred, not measured.
E10. Runner image scripts (actions/runner-images, today's main branch, not the 20261116 image; read 2026-11-25 via
    WebFetch, summarised by the fetch model, data only; script order not read): install-firefox.sh: Mozilla Team PPA,
    apt install firefox, PPA removed, one locale pref (en_US) to syspref.js, no chmod. configure-environment.sh:
    `chmod -R 777 $AGENT_TOOLSDIRECTORY` (/opt/hostedtoolcache). configure-system.sh: `chmod -R 777 /usr/share` and
    `chmod -R 777 /opt`. Inferred, not measured: Chrome under /opt/google/chrome is writable by the cut's user if it
    was installed before that chmod; Firefox's code under /usr/lib/firefox is outside both trees and stays root 755;
    Firefox's loaders read /usr/share data at run time (e.g. glvnd EGL vendor JSONs in /usr/share/glvnd/egl_vendor.d,
    which name libraries to load).
E11. Fix lines tested here under `bash --norc --noprofile -euo pipefail` (scratch fix.sh): 0 apostrophes, exit 0; the
    missing-ldconfig branch prints its own line; a cache line with libEGL prints that line.

## Threat model
- New code: the image's Firefox (root-owned, Mozilla Team PPA) driven by Playwright's BiDi route; two repository
  Python scripts (stdlib, `-I -B`, no writes under `--check`); ldconfig and grep. All inside the cut: the runner's uid,
  no groups, no_new_privs, no network, `env -i`.
- Rights: none above what repository code already has inside the cut (build scripts, tests, npm). A Firefox exploit or
  a swapped Firefox gains the cut's user, which planted code already is. Firefox's userns sandbox cannot change the
  cut's netns (owned by the initial user namespace). The BiDi port listens on loopback inside the cut only.
- Unchanged: nothing new before the cut; the token, the root-owned wasm-bindgen tool (its folder and parents checked
  root-owned, :119-123), the no-cache rule, the env list (E2). Crash reports are not sent (E3) and the cut blocks any
  network; no artifact upload exists.
- Trust in a green run: CUT-PATH's class (open, Low). New members: PATH-resolved python3, ldconfig, grep. Firefox is
  called by absolute path and its own files are root-owned.

## Findings
F1 Medium (test fidelity and comment truth; not a cut finding). diff config:16-19 = draft config:10-13. "Mozilla's own
   Firefox, the build players run" is not true of the run: Playwright starts it in a fresh profile with its test prefs,
   one of them `security.fileuri.strict_origin_policy: false` (coreBundle.js:45603-45605). Control E9: Chromium stopped
   at the first file:// subresource; Firefox ran the page's scripts from file://. If the runner has WebGL, file-url.spec
   would record "the page ran from a file" for Firefox, which a player's Firefox may not do (inferred). Exploit sketch:
   none (no attacker); a wrong recorded result for OQ-0002-1.
   Fix (comment only; draft config:10-13 become):
   // `firefox` runs the same browser tests in Mozilla's own Firefox build (Mustafa's FIREFOX-SOURCE (c), 2026-11-25),
   // which Playwright drives over WebDriver BiDi (`channel: "moz-firefox"`) in a fresh profile with Playwright's own
   // test preferences, not a player's: one of them sets `security.fileuri.strict_origin_policy` to false, so what
   // file-url.spec.ts records for Firefox may differ from a player's Firefox. SHAMBLE_FIREFOX names its program, as
   // SHAMBLE_CHROMIUM names Chromium's. It leaves out the allocation spec, which reads Chromium's own heap profiler.
   Option for ask 2 (his call, changes the Firefox list of 15): add "skeleton/file-url.spec.ts" to the Firefox testIgnore.
   Follow-up (outside the fence): file-url.spec.ts:1-3 ("here for Chromium"), with the isolation.spec.ts:2-3 one.
F2 Low. diff:35 = draft toolchain.yml:206. `|| echo "libEGL.so.1: not on this runner ..."` also fires when ldconfig is
   missing from PATH or fails, and his EGL-TRIAL item 3 sends "the runner lacks it" back to him as its own question: a
   false line misroutes it. Fix (still one read-only line; no install line, no Firefox setting; tested E11):
   if ! cache=$(/usr/sbin/ldconfig -p); then echo "libEGL.so.1: not checked, ldconfig -p failed (printed, not judged)"; elif printf "%s\n" "$cache" | grep -F "libEGL.so.1 "; then :; else echo "libEGL.so.1: not in the loader cache of this runner (printed, not judged)"; fi
N1 Note (CUT-PATH, no new finding). python3, ldconfig, grep resolve through the runner PATH (draft toolchain.yml:163-165,
   206); add them, and E10 (/opt and /usr/share 777 on the image; Chrome swappable: inferred), to CUT-PATH's record.
N2 Note, default "no change". The plain-file check (diff:36 = draft :207) is enough for what it does (fail by name,
   refuse a final link). The wasm-bindgen rigour (:119-125) guards pinned bytes he approved; Firefox has no pinned
   bytes, a swap gains the cut's user nothing, and Chrome's line (:193) has no check at all, so root-ownership checks
   for Firefox alone would not be a boundary. At most one printed-not-judged line if wanted (each line changes the
   sha256 he approves): stat -c "mode %a, owner %U:%G, %n" /usr/lib/firefox/firefox /usr/lib/firefox /usr/lib /usr /
N3 Note. TOKEN-WORDS' trigger is this edit (STATUS_ARCHIVE:1311); ask 2 names it as deferred or folded in.
N4 Note. Header "every crate's release tests" (diff:10) is true today (members `crates/*` = 6, all named at diff:19).
N5 Note. His graphics rule holds (E7). `media.sanity-test.disabled` (:45581) is Playwright's default, media only.

Verdict: FINDINGS (F1 Medium, F2 Low; both non-blocking for containment). Containment PASS: nothing weakens the cut,
the root-owned tool, the token or the no-cache rule.
