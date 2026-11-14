// rust-reviewer scratch (U4c review): size_of the v1 and v2 fate-log entry and event shapes, copied field for field.
#![allow(dead_code)]
#[derive(Clone, Copy)] enum Channel { Datagram, Control, FullState }
#[derive(Clone, Copy)] enum Dir { Up = 0, Down = 1 }
#[derive(Clone, Copy)] enum Outcome { Delivered { arrival_ns: u64 }, Lost, Burst }
#[derive(Clone, Copy)] enum Carries { Datagram, Stream { channel: Channel, pn: u64, message: u64, part: u32, parts: u32 }, Ack { acked: u64, largest: u64 } }
struct FateV2 { link: u32, dir: Dir, carries: Carries, seq: u64, sent_ns: u64, len: u32, outcome: Outcome, overtaken: bool, retransmits: u32 }
struct FateV1 { link: u32, dir: Dir, channel: Channel, seq: u64, sent_ns: u64, len: u32, outcome: Outcome, overtaken: bool, retransmits: u32 }
#[derive(Clone, Copy)] struct Piece { channel: Channel, message: u64, part: u32 }
enum Body { Datagram(Vec<u8>), Piece(Piece) }
struct Packet { pn: u64, space: usize, index: u64, sent_ns: u64, body: Body }
#[derive(Clone, Copy)] struct AckInfo { ordinal: u64, largest: u64, eligible: bool, acked: u64, acked_sent_ns: u64 }
enum What { Arrive(Packet), HeldDue, Ack(AckInfo), Timer { generation: u64 } }
struct Event { at_ns: u64, key: u64, order: u8, link: u32, dir: Dir, space: usize, tie: u64, what: What }
fn main() {
    println!("FateV1 {} FateV2 {} Event {}", std::mem::size_of::<FateV1>(), std::mem::size_of::<FateV2>(), std::mem::size_of::<Event>());
    // L-c8 pre-check counts from u4c-build.md:40-42 (v1) and :80-82 (v2), P2 seed 1.
    let (v1n, v2n) = (2_000_000u64, 3_959_634u64);
    let cap = |n: u64| n.next_power_of_two();
    println!("P2 probe log: v1 {} MB (cap {}), v2 {} MB (cap {})",
        cap(v1n) * std::mem::size_of::<FateV1>() as u64 / 1_000_000, cap(v1n),
        cap(v2n) * std::mem::size_of::<FateV2>() as u64 / 1_000_000, cap(v2n));
}
