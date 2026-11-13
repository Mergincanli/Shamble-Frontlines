//! The wasm-bindgen bindings the Worker loads, and nothing else (ADR 0002 parts 3 and 5). Empty, with no
//! bindings, until the boundary spike fixes its exports (OQ-0002-3); wasm-bindgen's by-day reading (OQ-0002-7)
//! comes first.

#[cfg(test)]
mod tests {
    #[test]
    fn stub_builds() {}
}
