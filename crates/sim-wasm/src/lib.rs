//! The wasm-bindgen bindings the Worker loads, and nothing else (ADR 0002 parts 3 and 5). Empty, with no
//! bindings, until the boundary spike fixes its exports (OQ-0002-3) and wasm-bindgen's by-day reading is done
//! (OQ-0002-7).

#[cfg(test)]
mod tests {
    #[test]
    fn stub_builds() {}
}
