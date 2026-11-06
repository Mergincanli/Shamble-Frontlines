//! npt-log: the results log of the netcode proof test (plan E20, §6.1): JSON Lines, one record per line, a header per
//! machine, then per-tick, per-second and event records, and a footer. It is not a shipped log: it is written apart
//! from the tracing output (security F18). Record types and the writer are built in U1's second part.
