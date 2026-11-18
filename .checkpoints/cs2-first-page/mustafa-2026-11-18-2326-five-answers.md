# Mustafa, 2026-11-18: his answers to the five questions of change-set 2 (RNG-CRATE, WB-HEALTH, TOOL-SOURCE, DEPART-2, CI-WASM) and the size

_Copied by script from this session's transcript, never retyped: the user message of 2026-11-18T22:34:37.564Z (uuid d6abf351-4175-4bc2-bb81-9db74fc152fb), whole (2261 bytes, sha256 f7c64f7a527396f904fc6926d9edf8f756956293fd0d6bb0f900773f1ab75ba3)._

---
My words of 2026-11-18, 23:26 (Amsterdam time): my answers to the five questions of change-set 2 (the first page), on your plan as it stands at commit 1d79e8e4.

RNG-CRATE: (c). No random-number library now. Change-set 2 carries only the match seed. The library enters in change-set 3 beside its test vectors, and you ask me which one with that plan. For that question also read two things: the rand project's upgrade guide for 0.10 (rust-random.github.io/book/update-0.10.html), which speaks of the output of the like-named generators in chacha20, and why chacha20's releases 0.10.0 and 0.10.1 were withdrawn.

WB-HEALTH: (a). Go on with wasm-bindgen, pinned to exactly 0.2.130, with the build refusing any other version of its tool and every version bump as its own change-set. Keep what crosses it as small as the plan has it (one type, plain numbers on every tick): that keeps a later move off it cheap.

TOOL-SOURCE: (a), for this one file only: wasm-bindgen-0.2.130-x86_64-unknown-linux-musl.tar.gz, 11860397 bytes, sha256 9687f3bb551b9f66f37ed32a40d7913a00f7962db653054e56857bc306c3227f, by the route of your plan's section 7, item 5a. This is my yes to run that ready-built program on the proof you have. My rule on unreviewed code from the internet stands as it is: every other file, version or checksum comes back to me.
The Advisor downloaded the same file over another network route and read the same size, the same checksum and the same six members. Inside it the program wasm-bindgen has 8170784 bytes and sha256 ca4df773020dae87d92b9eadb2b85412bea7a9cf6318ab9999dd119fd9f46931. If your copy differs in any of these, stop and tell me. GitHub's own check could not be run there either.

DEPART-2: (a), all six. The hand-off reading at the size of 12 and of 48 soldiers is made in change-sets 4 and 5, as your section 17 says.

CI-WASM: (a). A small pull request of its own, opened right after change-set 2 is merged and merged before change-set 3's build starts; change-set 3's plan may be written beside it. Its exact text comes to me first. It runs GitHub's own check on the tool's file before the tool's first run there, and its report quotes that check's output.

The size as you plan it, about 2,900 lines with the stop line at 4,000, is fine.
