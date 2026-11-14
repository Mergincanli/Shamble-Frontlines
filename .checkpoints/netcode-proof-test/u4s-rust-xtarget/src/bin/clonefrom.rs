// Does #[derive(Clone)] give a field-wise clone_from (Vec storage reused) or `*self = source.clone()` (new storage)?
// HitHistory::push relies on clone_from (crates/sim/src/history.rs:45-58) once HitVolumes holds Vecs (U4d).
#[derive(Clone, Default)]
struct Derived {
    tick: u32,
    parts: Vec<u64>,
}

struct Manual {
    tick: u32,
    parts: Vec<u64>,
}

impl Clone for Manual {
    fn clone(&self) -> Manual {
        Manual { tick: self.tick, parts: self.parts.clone() }
    }
    fn clone_from(&mut self, source: &Manual) {
        self.tick = source.tick;
        self.parts.clone_from(&source.parts);
    }
}

fn main() {
    let src = Derived { tick: 1, parts: vec![7; 1000] };
    let mut dst = Derived { tick: 0, parts: vec![0; 1000] };
    let before = dst.parts.as_ptr();
    dst.clone_from(&src);
    println!("derived clone_from keeps the Vec's storage: {} (tick {})", before == dst.parts.as_ptr(), dst.tick);

    let src = Manual { tick: 1, parts: vec![7; 1000] };
    let mut dst = Manual { tick: 0, parts: vec![0; 1000] };
    let before = dst.parts.as_ptr();
    dst.clone_from(&src);
    println!("manual clone_from keeps the Vec's storage: {} (tick {})", before == dst.parts.as_ptr(), dst.tick);
    let _ = Derived::default().clone();
}
