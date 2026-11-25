//! The panic hook's own test (`sim_wasm::panic`), in a test binary of its own: it installs the process's panic
//! hook, and a test that panics beside it would race it. Nothing else may join this file.

use sim_wasm::panic::{BUFFER_BYTES, HEADER_BYTES, buffer_bytes, panic_buffer, start};

#[test]
fn the_hook_writes_a_panic_s_place_and_message_into_the_module_s_buffer() {
    // Before the hook is installed the buffer says nothing.
    assert_eq!(buffer_bytes(), [0; BUFFER_BYTES]);
    start();
    assert!(std::panic::catch_unwind(|| panic!("planted for the hook's test: {}", 7)).is_err());
    let _ = std::panic::take_hook();
    // What a host reads: the text's length, the room for text, then that many bytes of text.
    let bytes = buffer_bytes();
    let word = |at: usize| {
        u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]) as usize
    };
    let text = std::str::from_utf8(&bytes[HEADER_BYTES..HEADER_BYTES + word(0)])
        .expect("whole characters");
    assert_eq!(word(4), BUFFER_BYTES - HEADER_BYTES);
    assert!(
        text.starts_with("panicked at crates/sim-wasm/tests/panic_hook.rs:"),
        "{text}"
    );
    assert!(
        text.ends_with(":\nplanted for the hook's test: 7"),
        "{text}"
    );
    assert!(!panic_buffer().is_null());
}
