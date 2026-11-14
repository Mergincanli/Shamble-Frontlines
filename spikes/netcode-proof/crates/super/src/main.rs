//! npt-super: the supervisor stand-in (plan E2, E16; §10.2): unprivileged, it starts, stops and kills room processes
//! through systemd template units by npt's fixed sudo wrappers, times each start to Open, and logs every exit.
//! Built in unit U6.

fn main() {
    eprintln!("npt-super: not built yet (unit U6)");
    std::process::exit(2);
}
