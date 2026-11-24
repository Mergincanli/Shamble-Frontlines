//! The loopback: the transport backend of local play (ADR 0001 part 9; ADR 0002 part 6). Two queues in one process,
//! one per direction, each a fixed number of fixed-size slots allocated once: a send copies the bytes in, a receive
//! copies them out, in the order they were sent. It never loses, repeats or reorders, and after `new` it never
//! allocates. The bytes that cross it are the real encoded messages.

use crate::transport::{Channel, Delivery, LinkState, SendError, Side, Transport, TransportPair};

/// One direction's queue: a ring of slots.
struct Queue {
    bytes: Box<[u8]>,
    /// Per slot, the length of the message in it and how it travels.
    packets: Box<[(usize, Delivery)]>,
    slot_bytes: usize,
    /// The slot of the oldest message waiting, and how many wait.
    first: usize,
    waiting: usize,
}

impl Queue {
    fn new(slot_bytes: u16, slots: u16) -> Self {
        let (slot_bytes, slots) = (usize::from(slot_bytes), usize::from(slots));
        Self {
            bytes: vec![0; slot_bytes.saturating_mul(slots)].into_boxed_slice(),
            packets: vec![(0, Delivery::Datagram); slots].into_boxed_slice(),
            slot_bytes,
            first: 0,
            waiting: 0,
        }
    }

    /// The slot `steps` after `slot`, around the ring. Both are below the slot count, so the sum cannot overflow.
    fn after(&self, slot: usize, steps: usize) -> usize {
        let at = slot.wrapping_add(steps);
        if at >= self.packets.len() {
            at.wrapping_sub(self.packets.len())
        } else {
            at
        }
    }

    fn send(&mut self, delivery: Delivery, bytes: &[u8]) -> Result<(), SendError> {
        if bytes.len() > self.slot_bytes {
            return Err(SendError::TooLarge);
        }
        if self.waiting >= self.packets.len() {
            return Err(SendError::Full);
        }
        let slot = self.after(self.first, self.waiting);
        let place = self
            .bytes
            .chunks_exact_mut(self.slot_bytes.max(1))
            .nth(slot)
            .and_then(|place| place.get_mut(..bytes.len()));
        let packet = self.packets.get_mut(slot);
        // Both exist for every slot below the slot count; were one missing, nothing would be queued.
        let (Some(place), Some(packet)) = (place, packet) else {
            return Err(SendError::Full);
        };
        place.copy_from_slice(bytes);
        *packet = (bytes.len(), delivery);
        self.waiting = self.waiting.wrapping_add(1);
        Ok(())
    }

    fn receive(&mut self, out: &mut Vec<u8>) -> Option<Delivery> {
        if self.waiting == 0 {
            return None;
        }
        let (len, delivery) = *self.packets.get(self.first)?;
        let bytes = self
            .bytes
            .chunks_exact(self.slot_bytes.max(1))
            .nth(self.first)?
            .get(..len)?;
        out.clear();
        out.extend_from_slice(bytes);
        self.first = self.after(self.first, 1);
        self.waiting = self.waiting.wrapping_sub(1);
        Some(delivery)
    }
}

/// Both ends of a loopback link.
pub struct Loopback {
    to_server: Queue,
    to_client: Queue,
}

impl Loopback {
    /// A link that holds up to `slots` messages of up to `slot_bytes` bytes in each direction. Its memory is taken
    /// here, once.
    #[must_use]
    pub fn new(slot_bytes: u16, slots: u16) -> Self {
        Self {
            to_server: Queue::new(slot_bytes, slots),
            to_client: Queue::new(slot_bytes, slots),
        }
    }
}

/// One end of a loopback link: what it sends, the other end receives.
pub struct LoopbackEnd<'a> {
    outgoing: &'a mut Queue,
    incoming: &'a mut Queue,
}

impl TransportPair for Loopback {
    type End<'a> = LoopbackEnd<'a>;

    fn end(&mut self, side: Side) -> LoopbackEnd<'_> {
        let (outgoing, incoming) = match side {
            Side::Client => (&mut self.to_server, &mut self.to_client),
            Side::Server => (&mut self.to_client, &mut self.to_server),
        };
        LoopbackEnd { outgoing, incoming }
    }
}

impl Transport for LoopbackEnd<'_> {
    fn send_unreliable(&mut self, bytes: &[u8]) -> Result<(), SendError> {
        self.outgoing.send(Delivery::Datagram, bytes)
    }

    fn send_reliable(&mut self, channel: Channel, bytes: &[u8]) -> Result<(), SendError> {
        self.outgoing.send(Delivery::Reliable(channel), bytes)
    }

    fn receive(&mut self, out: &mut Vec<u8>) -> Option<Delivery> {
        self.incoming.receive(out)
    }

    fn link_state(&self) -> LinkState {
        LinkState::Open
    }

    /// The loopback has no delay to measure.
    fn rtt_us(&self) -> Option<u32> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTROL: Delivery = Delivery::Reliable(Channel::Control);

    #[test]
    fn each_direction_delivers_in_order_with_how_it_travelled() {
        let mut link = Loopback::new(8, 4);
        let mut out = vec![0xEE];
        assert_eq!(link.end(Side::Server).receive(&mut out), None);
        assert_eq!(out, [0xEE], "nothing waits: the buffer is left as it was");
        let mut client = link.end(Side::Client);
        assert_eq!(client.link_state(), LinkState::Open);
        assert_eq!(client.rtt_us(), None);
        assert_eq!(client.send_reliable(Channel::Control, &[1, 2, 3]), Ok(()));
        assert_eq!(client.send_unreliable(&[]), Ok(()));
        assert_eq!(client.send_unreliable(&[4; 8]), Ok(()));
        // What the client sent is not the client's to receive.
        assert_eq!(client.receive(&mut out), None);
        assert_eq!(link.end(Side::Server).send_unreliable(&[9]), Ok(()));
        let mut server = link.end(Side::Server);
        assert_eq!(server.receive(&mut out), Some(CONTROL));
        assert_eq!(out, [1, 2, 3]);
        assert_eq!(server.receive(&mut out), Some(Delivery::Datagram));
        assert_eq!(out, []);
        assert_eq!(server.receive(&mut out), Some(Delivery::Datagram));
        assert_eq!(out, [4; 8]);
        assert_eq!(server.receive(&mut out), None);
        assert_eq!(
            link.end(Side::Client).receive(&mut out),
            Some(Delivery::Datagram)
        );
        assert_eq!(out, [9]);
    }

    #[test]
    fn a_full_queue_and_a_message_too_long_are_refused_and_nothing_is_lost() {
        let mut link = Loopback::new(4, 2);
        let mut client = link.end(Side::Client);
        assert_eq!(client.send_unreliable(&[0; 5]), Err(SendError::TooLarge));
        assert_eq!(client.send_unreliable(&[1]), Ok(()));
        assert_eq!(client.send_unreliable(&[2]), Ok(()));
        assert_eq!(client.send_unreliable(&[3]), Err(SendError::Full));
        assert_eq!(
            client.send_reliable(Channel::Control, &[3]),
            Err(SendError::Full)
        );
        // The other direction has its own room.
        assert_eq!(link.end(Side::Server).send_unreliable(&[7]), Ok(()));
        let mut out = Vec::new();
        let mut server = link.end(Side::Server);
        assert_eq!(server.receive(&mut out), Some(Delivery::Datagram));
        assert_eq!(out, [1]);
        // One left, so one more fits again.
        assert_eq!(link.end(Side::Client).send_unreliable(&[3]), Ok(()));
        let mut server = link.end(Side::Server);
        for expected in [2, 3] {
            assert_eq!(server.receive(&mut out), Some(Delivery::Datagram));
            assert_eq!(out, [expected]);
        }
        assert_eq!(server.receive(&mut out), None);
        // A link with no room at all refuses everything and never panics.
        for mut none in [
            Loopback::new(0, 4),
            Loopback::new(4, 0),
            Loopback::new(0, 0),
        ] {
            assert!(none.end(Side::Client).send_unreliable(&[1]).is_err());
            assert!(none.end(Side::Client).send_unreliable(&[]).is_err());
            assert_eq!(none.end(Side::Server).receive(&mut out), None);
        }
    }

    /// 10,000 messages of every length go round the ring many times; the queues' memory and the receiver's buffer
    /// stay where they are.
    #[test]
    fn ten_thousand_messages_cross_without_moving_a_buffer() {
        let mut link = Loopback::new(64, 16);
        let places = (
            link.to_server.bytes.as_ptr(),
            link.to_server.packets.as_ptr(),
        );
        let mut out = Vec::with_capacity(64);
        let out_place = out.as_ptr();
        let message = |n: u32| -> Vec<u8> { n.to_le_bytes().repeat((n % 17) as usize) };
        let (mut sent, mut received) = (0u32, 0u32);
        while received < 10_000 {
            // Bursts of 1 to 16 fill the ring to its last slot and wrap it at every offset.
            for _ in 0..=sent % 16 {
                if link.end(Side::Client).send_unreliable(&message(sent)) == Ok(()) {
                    sent += 1;
                }
            }
            while link.end(Side::Server).receive(&mut out) == Some(Delivery::Datagram) {
                assert_eq!(out, message(received));
                received += 1;
            }
        }
        assert_eq!(sent, received);
        assert_eq!(
            (
                link.to_server.bytes.as_ptr(),
                link.to_server.packets.as_ptr()
            ),
            places
        );
        assert_eq!(out.as_ptr(), out_place);
    }
}
