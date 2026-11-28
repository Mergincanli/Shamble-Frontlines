/**
 * The map a local match is played on: `sim_wasm::LOCAL_MAP`'s value (the boundary test reads it from the Rust source,
 * boundary/sim-wasm.test.mjs). The page reads that map's file from the content bundle, and the recording reader
 * holds a recording to it. A module of its own, so that the page's import of it takes in nothing of the test-only
 * recording reader.
 */
export const LOCAL_MAP = "test-scene";
