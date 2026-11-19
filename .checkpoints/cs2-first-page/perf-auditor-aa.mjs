// perf-auditor, cs2 review 1: an A/A run of the main session's overflow-cost.mjs, unchanged: the same built Wasm in two
// folders (two module instances), so any difference it prints is the method's own noise.   node .gate/perf-auditor-aa.mjs
process.argv[2] = "/home/user/Shamble-Frontlines/.gate/perf-auditor-pkg-a";
process.argv[3] = "/home/user/Shamble-Frontlines/.gate/perf-auditor-pkg-b";
await import("/home/user/Shamble-Frontlines/.checkpoints/cs2-first-page/overflow-cost.mjs");
