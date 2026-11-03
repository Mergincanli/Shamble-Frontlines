// d3d: which Hasher methods does std's Hash feed for a slice, a Vec and a derived struct? (local rustc, no fetch)
use std::hash::{Hash, Hasher};
#[derive(Default)]
struct Log { calls: Vec<String> }
impl Hasher for Log {
    fn finish(&self) -> u64 { 0 }
    fn write(&mut self, b: &[u8]) { self.calls.push(format!("write({} bytes)", b.len())); }
    fn write_u32(&mut self, i: u32) { self.calls.push(format!("write_u32({})", i)); }
    fn write_u64(&mut self, i: u64) { self.calls.push(format!("write_u64({})", i)); }
    fn write_usize(&mut self, i: usize) { self.calls.push(format!("write_usize({})", i)); }
}
#[derive(Hash)]
struct S { a: u32, v: Vec<u32>, s: [u32; 2] }
fn show<T: Hash + ?Sized>(name: &str, t: &T) {
    let mut h = Log::default();
    t.hash(&mut h);
    println!("{:<28} {}", name, h.calls.join(", "));
}
fn main() {
    show("&[u32] len 3", &[7u32, 8, 9][..]);
    show("Vec<u32> len 3", &vec![7u32, 8, 9]);
    show("[u32; 2] (array)", &[7u32, 8]);
    show("derive S{a,v:len2,s:[_;2]}", &S { a: 1, v: vec![5, 6], s: [3, 4] });
    show("usize 5", &5usize);
    show("str \"ab\"", "ab");
    println!("size_of::<usize>() = {} on this target", std::mem::size_of::<usize>());
}
