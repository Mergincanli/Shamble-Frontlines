//! npt-client-wasm: the client endpoint's wasm-bindgen exports only (plan E7; ADR 0002:191; u7-brief.md rev 1.1 Block
//! R3 with the advisor's C5): `Client` (`new`, `step`, `next_due_ns`, `counters`), `counter_names` and `limits`. A
//! wake is one `step` call (and a `counters` read a second); no per-entity call. The logic is engine.rs's, plain Rust
//! tested natively: these wrappers only convert. A u64 crosses as a BigInt, bytes as a Uint8Array (no js-sys). Built
//! in unit U7.

use wasm_bindgen::prelude::*;

mod engine;
mod script;

/// The browser client of one slot (engine.rs's `Core`).
#[wasm_bindgen]
pub struct Client {
    inner: engine::Core,
}

#[wasm_bindgen]
impl Client {
    /// The client of the slot `token` names (`nptj_` and 22 base64url characters), with the page's 16 B `bind`; an
    /// error for any other token or bind (its text names which, never its value).
    #[wasm_bindgen(constructor)]
    pub fn new(token: &str, bind: &[u8]) -> Result<Client, JsError> {
        engine::Core::new(token, bind)
            .map(|inner| Client { inner })
            .map_err(|r| JsError::new(r.text()))
    }

    /// One wake at `now_ns` with the host's send time of the last call's output (`prev_send_ns`, 0 before any) and
    /// the messages received since (`inbox`, C5's records); returns the outbox (C5's records).
    pub fn step(&mut self, now_ns: u64, prev_send_ns: u64, inbox: &[u8]) -> Vec<u8> {
        self.inner.step(now_ns, prev_send_ns, inbox)
    }

    /// When the next step is due, ns on the host's clock (0: at once).
    pub fn next_due_ns(&self) -> u64 {
        self.inner.next_due_ns()
    }

    /// The counters, by `counter_names()`' index.
    pub fn counters(&self) -> Vec<u32> {
        self.inner.counters().to_vec()
    }
}

/// The counters' names, by index: the one home of what `Client::counters` holds.
#[wasm_bindgen]
pub fn counter_names() -> Vec<String> {
    engine::COUNTER_NAMES
        .iter()
        .map(|n| (*n).to_owned())
        .collect()
}

/// C5's limits: the largest control message and full state (each with its length), the largest datagram, then the
/// WebSocket tags of `Datagram`, `Control` and `FullState`.
#[wasm_bindgen]
pub fn limits() -> Vec<u32> {
    engine::limits().to_vec()
}
