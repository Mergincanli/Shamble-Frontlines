ID = 'F14'
TITLE = 'OQ-0006-16: the TCP 443 listener claim, reworded and marked unsourced'
START, END = 953, 954
AFTER = """    QUIC (a question; no source read). A connection to an address's TCP 443 reaches one listener (no source read), so
    443 needs a front on each VM that routes by path to the room processes, or an address per room."""
WHY = """F14 holds: "Only one listener per address can hold TCP 443" (0006:953) has no source in the record; it came from my
review (review-infra-engineer.md:57, finding 7), also unsourced. As its skeptic: the claim is likely too strong as well,
since Linux's SO_REUSEPORT lets several sockets listen on one address and port (from memory; socket(7) was not read
tonight: no local man pages, and this task fetches nothing), but the kernel then picks the socket per connection, not
by path. The new wording holds either way and is all the sentence's "so" needs: each connection reaches one listener,
so either that listener routes by path or each room has its own address. Marked "(no source read)" as the verifier
asks; a read of socket(7) can drop the mark later."""
