// rust-reviewer's round-2 scratch for change-set 4b-1 (not part of any build of the repository): json.rs string().
// Built by hand with rustc against a crates/content copy. Prints, for each input: the verdict, the bytes held at the
// peak of the parse (a counting allocator, this thread), and for a taken string its length and capacity.
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! { static HELD: Cell<(usize, usize)> = const { Cell::new((0, 0)) }; }
fn count(taken: usize, given_back: usize) {
    let _ = HELD.try_with(|held| {
        let (now, most) = held.get();
        let now = now.saturating_add(taken).saturating_sub(given_back);
        held.set((now, most.max(now)));
    });
}
struct Counting;
// SAFETY: every call passes to System unchanged; count() neither allocates nor unwinds.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 { count(l.size(), 0); unsafe { System.alloc(l) } }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 { count(l.size(), 0); unsafe { System.alloc_zeroed(l) } }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 { count(n, l.size()); unsafe { System.realloc(p, l, n) } }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) { count(0, l.size()); unsafe { System.dealloc(p, l) } }
}
#[global_allocator]
static A: Counting = Counting;

fn peak<T>(work: impl FnOnce() -> T) -> (T, usize) {
    let start = HELD.get().0;
    HELD.set((start, start));
    let r = work();
    (r, HELD.get().1.saturating_sub(start))
}

fn main() {
    use content::json::{Value, parse};
    let mb = "x".repeat(1_000_000);
    let cases: Vec<(&str, Vec<u8>)> = vec![
        ("a 1 MB string, taken", format!("{{\"a\": \"{mb}\"}}").into_bytes()),
        ("escapes, taken", b"{\"a\": \"x\\\"y\\nz\"}".to_vec()),
        ("a 524,289-byte string, taken", format!("{{\"a\": \"{}\"}}", " ".repeat(524_289)).into_bytes()),
        ("a forbidden byte first, then 1 MB, then the quote", format!("{{\"a\": \"\x01{mb}\"}}").into_bytes()),
        ("a refused escape first, then 1 MB, then the quote", format!("{{\"a\": \"\\t{mb}\"}}").into_bytes()),
        ("unterminated, 1 MB", format!("{{\"a\": \"{mb}").into_bytes()),
        ("a key of 1 MB (refused as a key after it is read)", format!("{{\"{mb}\": 1}}").into_bytes()),
        ("escaped quote at the very end", b"{\"a\": \"abc\\".to_vec()),
    ];
    for (name, bytes) in cases {
        let (verdict, held) = peak(|| parse(&bytes));
        let detail = match &verdict {
            Ok(node) => match &node.value {
                Value::Object(members) => match members.first().map(|m| &m.node.value) {
                    Some(Value::Text(t)) => format!("taken: length {} capacity {}", t.len(), t.capacity()),
                    _ => "taken".to_owned(),
                },
                _ => "taken".to_owned(),
            },
            Err(e) => format!("refused: {}", e.record().chars().take(90).collect::<String>()),
        };
        println!("{name}: {} bytes in, peak {held} bytes held; {detail}", bytes.len());
    }
}
