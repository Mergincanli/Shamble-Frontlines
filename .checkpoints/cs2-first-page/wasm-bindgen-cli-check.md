# The wasm-bindgen tool's check (plan section 7, item 5a): what the route measured

_The output of `bash .checkpoints/cs2-first-page/tool-route.sh`, run by the main session; its own first and last lines are the clock's (`date -u`). Every value equals Mustafa's of 2026-11-18, 22:34:37 UTC (TOOL-SOURCE (a), `mustafa-2026-11-18-2326-five-answers.md`): the archive's 11860397 bytes and sha256 9687f3bb...227f, its six members, the program's 8170784 bytes and sha256 ca4df773...6931. The program was not run by the route._

```
Wed Nov 18 23:09:31 UTC 2026
== 1. a new folder, and the download
final address: https://release-assets.githubusercontent.com/github-production-release-asset/114685405/ac3e940f-cd35-4c83-83e6-e4ccfff00e5b?(query left out)
status: 200, redirects: 1
== 2. size, then sha256, before any tar call
size: 11860397
.gate/cs2-first-page/tool-download/wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz: OK
== 3. the member list
drwxr-xr-x runner/runner     0 2026-10-30 06:10 wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/
-rw-r--r-- runner/runner 10847 2026-10-30 06:10 wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/LICENSE-APACHE
-rw-r--r-- runner/runner  1057 2026-10-30 06:10 wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/LICENSE-MIT
-rwxr-xr-x runner/runner 5795136 2026-10-30 06:10 wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/wasm2es6js
-rwxr-xr-x runner/runner 8170784 2026-10-30 06:10 wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/wasm-bindgen
-rw-r--r-- runner/runner    5549 2026-10-30 06:10 wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/README.md
-rwxr-xr-x runner/runner 19316480 2026-10-30 06:10 wasm-bindgen-0.2.130-x86_64-unknown-linux-musl/wasm-bindgen-test-runner
members: 7 (the folder and six files)
== 4. only the program, into a new empty folder
== 5. the program, before any run
size: 8170784
.gate/tools/wasm-bindgen-0.2.130/wasm-bindgen: OK
  Class:                             ELF64
  Type:                              DYN (Position-Independent Executable file)
  Machine:                           Advanced Micro Devices X86-64
no interpreter: a static program
mode 555, owner 0:0, .gate/tools/wasm-bindgen-0.2.130/wasm-bindgen
mode 555, owner 0:0, .gate/tools/wasm-bindgen-0.2.130
== done: the program is in place, checked and not run
Wed Nov 18 23:09:33 UTC 2026
```
