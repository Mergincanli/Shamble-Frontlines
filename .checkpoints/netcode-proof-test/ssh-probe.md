# netcode-proof-test: plan item 1, the SSH reachability probe (first pass, 2026-11-05 21:30-21:33 UTC)
Probes run once each from this cloud session's container (no retries of a denial, per /root/.ccr/README.md).

| Probe | Command (shape) | Result |
|---|---|---|
| SSH client present | `command -v ssh ssh-keygen scp sftp` | all four MISSING from the image |
| Direct TCP to an SSH port | `nc -vz -w 5 github.com 22` | timed out ("Operation now in progress") |
| Direct TCP, another non-443 port | `nc -vz -w 8 1.1.1.1 53` (TCP) | timed out |
| Port 22 through the agent proxy | `curl -p -x $HTTPS_PROXY telnet://github.com:22` | timed out (no tunnel) |
| Direct TCP to 443 | `nc -vz -w 5 github.com 443` | connects, but the certificate seen is issued by "O = Anthropic, CN = Egress Gateway SDS Issuing CA (production)": port 443 is intercepted and TLS is re-terminated, so a non-TLS protocol (SSH on 443) cannot pass |
| HTTPS policy breadth | `curl https://example.com/`, `https://www.hetzner.com/` | 200, 200 (arbitrary HTTPS hosts are allowed) |
| HTTPS to the domain | `curl https://shamblefrontlines.com/` | 000; proxy log: "gateway answered 502 to CONNECT (policy denial or upstream failure)"; the domain has no address record yet, so upstream failure is the likely reading (not checked further) |
| UDP 53 to 1.1.1.1 | one DNS query by Python | a 61-byte reply (may be DNS interception; general UDP not tested) |

Proxy README (`/root/.ccr/README.md`), "Not supported through the proxy (report, do not work around)": "gRPC / HTTP/2-only
APIs, WebSocket upgrades, client-mTLS, certificate-pinned clients ..., non-443 HTTPS ports, raw-TCP databases."

**Reading:** this session cannot reach a server over SSH (no client, and no egress for SSH's TCP on any port).
What it can do: HTTPS (443) to public hosts, through the egress gateway, which re-terminates TLS. So it cannot be a
WebSocket or WebTransport client of the test servers either (WebSocket upgrades unsupported; QUIC is UDP).
Not yet done: the same probe against the real RS/LG on a run day (only if Mustafa wants it confirmed on a real VM).
