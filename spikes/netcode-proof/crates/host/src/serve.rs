//! The page host, trimmed (u6c-brief.md Block 5, C1 (e); plan.md:1117-1128): `npt-host serve <settings.json>`, U8's
//! host.json (`bind`, `cert_chain`, `cert_key`, `dist`, `hash_template`, `host_keys`), run until stopped.
//!
//! - **TLS:** 1.3 only, ALPN `http/1.1`, ring chosen for this configuration with `builder_with_provider` (no process
//!   default), the production chain and key read once at start under a size cap.
//! - **Files:** a compiled-in allow-list of U7's seven dist names (u7-brief.md:66-68), each read into memory once at
//!   start (a missing one is exit 2), never a path built from the request; `/npt/cert-hash-NN.json` for NN 00 to 31 from
//!   the settings' template (no plain `/npt/cert-hash.json`: Q-U6C-3) and `/npt/ssh-host-keys.txt`, each read per
//!   request under its cap (1 KiB, 4 KiB) and served verbatim with `Cache-Control: no-store`: absent 404, oversized 500,
//!   never cut short.
//! - **HTTP:** one request a connection, then close; httparse over one 8 KiB buffer and 32 header slots a connection, a
//!   request that does not fit closed (Q-U6C-4 (a)); GET and HEAD only (405); the request target's path up to the first
//!   `?` matched exactly, the query never read or logged (C1 (e)); anything else 404. Every answer carries plan.md's CSP
//!   word for word, `X-Content-Type-Options: nosniff` and `Referrer-Policy: no-referrer`.
//! - **Logs:** none per request (no client string anywhere: E22); one stderr line at start and one on a fatal error.

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use serde::Deserialize;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_rustls::rustls;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};

/// plan.md:1123-1126, word for word.
pub(crate) const CSP: &str = "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; worker-src 'self'; \
    connect-src 'self' https://rs.nt.shamblefrontlines.com:* wss://rs.nt.shamblefrontlines.com:* \
    https://rs-b.nt.shamblefrontlines.com:* wss://rs-b.nt.shamblefrontlines.com:*; style-src 'self'; img-src 'self'; \
    base-uri 'none'; form-action 'none'; frame-ancestors 'none'";
const JS: &str = "text/javascript; charset=utf-8";
/// U7's dist names (u7-brief.md:66-68): (path, file in `dist`, type).
pub(crate) const FILES: [(&str, &str, &str); 7] = [
    ("/", "index.html", "text/html; charset=utf-8"),
    ("/main.js", "main.js", JS),
    ("/worker.js", "worker.js", JS),
    ("/lib.js", "lib.js", JS),
    ("/hooks.js", "hooks.js", JS),
    ("/npt_client_wasm.js", "npt_client_wasm.js", JS),
    (
        "/npt_client_wasm_bg.wasm",
        "npt_client_wasm_bg.wasm",
        "application/wasm",
    ),
];
const SETTINGS_CAP: u64 = 64 * 1024;
const PEM_CAP: u64 = 64 * 1024;
/// A dist file's cap (the Wasm is the largest).
const FILE_CAP: u64 = 32 * 1024 * 1024;
const HASH_CAP: u64 = 1024;
const HOST_KEYS_CAP: u64 = 4 * 1024;
const REQUEST_BYTES: usize = 8 * 1024;
const HEADERS: usize = 32;

/// The host's settings (U8's ops/systemd/host.json; SEC-U6AD-2's rules).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Settings {
    pub(crate) bind: String,
    pub(crate) cert_chain: String,
    pub(crate) cert_key: String,
    pub(crate) dist: String,
    /// The hash files' path with `NN` once for the room (`/var/lib/npt/hash/room-NN.json`).
    pub(crate) hash_template: String,
    pub(crate) host_keys: String,
}

/// What the host serves: the dist files in memory, the per-request files' paths.
pub(crate) struct Host {
    files: Vec<(&'static str, &'static str, Vec<u8>)>,
    hash_template: String,
    host_keys: String,
}

/// The file at `path` through `take(cap + 1)`; `Ok(None)` above `cap`. std's read (the workspace's tokio has no `fs`):
/// the per-request files are at most 4 KiB.
pub(crate) fn capped(path: &str, cap: u64) -> io::Result<Option<Vec<u8>>> {
    use std::io::Read;
    let mut v = Vec::new();
    std::fs::File::open(path)?
        .take(cap + 1)
        .read_to_end(&mut v)?;
    Ok((v.len() as u64 <= cap).then_some(v))
}

fn std_capped(path: &str, cap: u64) -> Result<Vec<u8>, String> {
    capped(path, cap)
        .map_err(|e| format!("{path}: {e}"))?
        .ok_or_else(|| format!("{path} is above {cap} bytes"))
}

/// The settings checked and the start's reads: the TLS configuration (TLS 1.3, ALPN `http/1.1`, ring) and the files.
/// Every refusal is exit 2's.
pub(crate) fn prepare(s: &Settings) -> Result<(SocketAddr, rustls::ServerConfig, Host), String> {
    let texts = [
        &s.bind,
        &s.cert_chain,
        &s.cert_key,
        &s.dist,
        &s.hash_template,
        &s.host_keys,
    ];
    if texts
        .iter()
        .any(|t| t.is_empty() || t.len() > 4_096 || t.chars().any(char::is_control))
    {
        return Err("a setting is empty, above 4,096 B or holds a control character".into());
    }
    if s.hash_template.matches("NN").count() != 1 {
        return Err("hash_template holds NN once".into());
    }
    let bind = s
        .bind
        .parse()
        .map_err(|_| "bind is not an address and port".to_owned())?;
    let chain: Vec<CertificateDer<'static>> =
        CertificateDer::pem_slice_iter(&std_capped(&s.cert_chain, PEM_CAP)?)
            .collect::<Result<_, _>>()
            .map_err(|_| "the chain's PEM".to_owned())?;
    let key = PrivateKeyDer::from_pem_slice(&std_capped(&s.cert_key, PEM_CAP)?)
        .map_err(|_| "the key's PEM".to_owned())?;
    let mut tls = rustls::ServerConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_protocol_versions(&[&rustls::version::TLS13])
    .map_err(|e| e.to_string())?
    .with_no_client_auth()
    .with_single_cert(chain, key)
    .map_err(|e| format!("the certificate: {e}"))?;
    tls.alpn_protocols = vec![b"http/1.1".to_vec()];
    let mut files = Vec::new();
    for (path, name, kind) in FILES {
        files.push((
            path,
            kind,
            std_capped(&format!("{}/{name}", s.dist), FILE_CAP)?,
        ));
    }
    let host = Host {
        files,
        hash_template: s.hash_template.clone(),
        host_keys: s.host_keys.clone(),
    };
    Ok((bind, tls, host))
}

/// One answer: the status, the headers every answer carries, the type and `no-store` when given, the body's length, and
/// the body but for HEAD.
fn answer(status: &str, kind: Option<&str>, no_store: bool, body: &[u8], head: bool) -> Vec<u8> {
    let mut h = format!(
        "HTTP/1.1 {status}\r\nContent-Security-Policy: {CSP}\r\nX-Content-Type-Options: nosniff\r\n\
         Referrer-Policy: no-referrer\r\nContent-Length: {}\r\nConnection: close\r\n",
        body.len()
    );
    if let Some(k) = kind {
        h.push_str(&format!("Content-Type: {k}\r\n"));
    }
    if no_store {
        h.push_str("Cache-Control: no-store\r\n");
    }
    if status.starts_with("405") {
        h.push_str("Allow: GET, HEAD\r\n");
    }
    h.push_str("\r\n");
    let mut out = h.into_bytes();
    if !head {
        out.extend_from_slice(body);
    }
    out
}

/// A per-request file, verbatim with `no-store`: absent 404, unreadable or oversized 500.
fn per_request(path: &str, cap: u64, kind: &str, head: bool) -> Vec<u8> {
    match capped(path, cap) {
        Ok(Some(b)) => answer("200 OK", Some(kind), true, &b, head),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            answer("404 Not Found", None, false, b"", head)
        }
        _ => answer("500 Internal Server Error", None, false, b"", head),
    }
}

impl Host {
    /// The answer to one request: `target`'s path up to the first `?` (the query never read), matched exactly.
    pub(crate) fn respond(&self, method: &str, target: &str) -> Vec<u8> {
        let head = method == "HEAD";
        if method != "GET" && !head {
            return answer("405 Method Not Allowed", None, false, b"", false);
        }
        let path = target.split('?').next().unwrap_or_default();
        if let Some((_, kind, body)) = self.files.iter().find(|f| f.0 == path) {
            return answer("200 OK", Some(kind), false, body, head);
        }
        if path == "/npt/ssh-host-keys.txt" {
            let text = "text/plain; charset=utf-8";
            return per_request(&self.host_keys, HOST_KEYS_CAP, text, head);
        }
        let nn = path
            .strip_prefix("/npt/cert-hash-")
            .and_then(|r| r.strip_suffix(".json"))
            .filter(|n| {
                n.len() == 2
                    && n.bytes().all(|b| b.is_ascii_digit())
                    && n.parse::<u8>().is_ok_and(|v| v <= 31)
            });
        match nn {
            Some(n) => {
                let file = self.hash_template.replace("NN", n);
                per_request(&file, HASH_CAP, "application/json", head)
            }
            None => answer("404 Not Found", None, false, b"", head),
        }
    }
}

/// One connection's request: read into one 8 KiB buffer until httparse has it whole (32 header slots); a request
/// that does not fit, or does not parse, is closed with no answer.
pub(crate) async fn connection<S: AsyncRead + AsyncWrite + Unpin>(mut s: S, host: &Host) {
    let mut buf = vec![0u8; REQUEST_BYTES];
    let mut n = 0;
    let (method, target) = loop {
        if n == buf.len() {
            return;
        }
        match s.read(&mut buf[n..]).await {
            Ok(0) | Err(_) => return,
            Ok(k) => n += k,
        }
        let mut headers = [httparse::EMPTY_HEADER; HEADERS];
        let mut req = httparse::Request::new(&mut headers);
        match req.parse(&buf[..n]) {
            Ok(httparse::Status::Complete(_)) => {
                break (
                    req.method.unwrap_or_default().to_owned(),
                    req.path.unwrap_or_default().to_owned(),
                );
            }
            Ok(httparse::Status::Partial) => {}
            Err(_) => return,
        }
    };
    let out = host.respond(&method, &target);
    if s.write_all(&out).await.is_ok() {
        let _ = s.shutdown().await;
    }
}

/// Serves `listener` until stopped: each connection's TLS and its one request on a task of its own.
pub(crate) async fn serve(
    listener: tokio::net::TcpListener,
    tls: rustls::ServerConfig,
    host: Arc<Host>,
) {
    let acceptor = tokio_rustls::TlsAcceptor::from(Arc::new(tls));
    loop {
        let Ok((tcp, _)) = listener.accept().await else {
            // SEC-U6C-1: an accept error (EMFILE) waits 100 ms before the next accept, never a spin.
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            continue;
        };
        let (acceptor, host) = (acceptor.clone(), Arc::clone(&host));
        tokio::spawn(async move {
            if let Ok(t) = acceptor.accept(tcp).await {
                connection(t, &host).await;
            }
        });
    }
}

/// `npt-host serve <settings.json>`: exit 2 for bad settings or a start read that fails, 1 for a runtime or bind failure.
pub(crate) fn main(path: &str) -> std::process::ExitCode {
    let fail = |code: u8, why: &str| {
        eprintln!("npt-host: {why}");
        std::process::ExitCode::from(code)
    };
    let s: Settings = match std_capped(path, SETTINGS_CAP)
        .and_then(|t| serde_json::from_slice(&t).map_err(|e| e.to_string()))
    {
        Ok(s) => s,
        Err(e) => return fail(2, &format!("the settings: {e}")),
    };
    let (bind, tls, host) = match prepare(&s) {
        Ok(x) => x,
        Err(e) => return fail(2, &e),
    };
    let rt = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => return fail(1, &format!("the runtime: {e}")),
    };
    rt.block_on(async {
        match tokio::net::TcpListener::bind(bind).await {
            Ok(l) => {
                eprintln!("npt-host: serving {} files on {bind}", FILES.len());
                serve(l, tls, Arc::new(host)).await;
                std::process::ExitCode::SUCCESS
            }
            Err(e) => fail(1, &format!("bind {bind}: {e}")),
        }
    })
}

#[cfg(test)]
#[path = "serve_tests.rs"]
mod tests;
