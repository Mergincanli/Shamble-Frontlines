# Large scratch files (2026-11-03, the session's note)
Skeptic-6 left three build outputs here, about 31 MB in all: euclid_test.raw (22,400,386 bytes, its sweep's raw
output; the summary is euclid_ref.out), euclid_test (4,434,520 bytes, the compiled test from euclid_test.rs, rustc
1.97.0, -O) and core-rmeta-strings.txt (4,182,002 bytes, strings decoded from rustc 1.97.0's std metadata; the doc
lines it quotes are in div_euclid_f32_doc_1.97.txt, rem_euclid_f32_doc_1.97.txt, f64_euclid_docs_1.97.txt and
std-rmeta-strings-4790-4800.txt). The background saver committed them at 294b3cf before the skeptic removed them
itself; the removal is in fe0f7be. They stay in 294b3cf's history (history is never rewritten). Every prompt after
this says: no binaries and no files over 1 MB in scratch.
