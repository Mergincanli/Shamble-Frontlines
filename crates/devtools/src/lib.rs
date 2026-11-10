//! The replay runner, the hash comparer, the tick bench, the legacy-golden reader and the headless protocol
//! clients (ADR 0002 parts 3 and 8). Empty until its first tool.

#[cfg(test)]
mod tests {
    #[test]
    fn stub_builds() {}

    // Planted failure M1a (reverted in the next commit): a test that needs the network must fail inside the cut.
    #[test]
    fn planted_network_use() {
        std::net::TcpStream::connect(("github.com", 443)).expect("planted M1a: this test needs the network");
    }
}
