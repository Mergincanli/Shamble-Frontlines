//! npt-host: the page host (plan E21, §10.4): a compiled-in allow-list of files from one dist directory, GET and HEAD
//! only, no listing, the CSP and other headers, COOP/COEP only under /iso/, `cert-hash.json` and `ssh-host-keys.txt`;
//! and the ACME client that `npt-cert issue` and `revoke` run (plan §8.4). Built in unit U6.

fn main() {
    eprintln!("npt-host: not built yet (unit U6)");
    std::process::exit(2);
}
