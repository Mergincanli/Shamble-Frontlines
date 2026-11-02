SUBS = [
("| 0 lost and 0 reordered events over the 10-minute match at N = 48 |",
 "| over the 10-minute match at N = 48: 0 lost and 0 doubled events, and 0 released out of order on their entity's timeline (part 5): within each timeline (the local player's own, predicted and owner events; each remote entity's) events arrive in tick order, each remote spatial event in the first block whose delayed time reaches its tick, and own, predicted and owner events in the block of the tick that emitted them |"),
("| every event carries a per-match sequence number (a field of the spike's event section, proposed); the page checks that each block's events continue the sequence; the Worker's emitted total equals the page's received total at the end |",
 "| every event carries a per-match sequence number, its tick and its timeline (fields of the spike's event section, proposed); the page checks each sequence number arrives exactly once and, per timeline, in tick order, and logs each block's delayed time beside the events it carries; the Worker's emitted total equals the page's received total at the end (a sequence across all timelines no longer runs in order: remote events wait for the delayed time) |"),
("Bites: the page returning a block to the pool unread when two blocks arrive between frames |",
 "Bites: the page returning a block to the pool unread when two blocks arrive between frames; remote events released on arrival, into the newest block, instead of at their delayed time |"),
("| R1-BND-REC2 mark 4 (r1, 2280–2281); N = 48 (MODES-3 (a)) |",
 "| R1-BND-REC2 mark 4 (r1, 2280–2281); N = 48 (MODES-3 (a)); the draft, part 5 (the event section's timing, ADR 0001 part 7's rule); the engine review of 2026-11-02, finding 1 (ADR 0001's PM-0001-61 times the drawn result) |"),
]
