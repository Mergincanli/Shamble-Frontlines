//! Block 5's tests (u6c-brief.md section 3, C1 (e)): the host on loopback over TLS, a test certificate from rcgen.

use super::*;
use std::path::PathBuf;

struct Fixture {
    dir: PathBuf,
    addr: SocketAddr,
    rt: tokio::runtime::Runtime,
    client: Arc<rustls::ClientConfig>,
    root: CertificateDer<'static>,
}

fn write(path: &std::path::Path, body: &[u8]) {
    std::fs::write(path, body).expect("written");
}

/// A host on 127.0.0.1 over a temporary dist folder (each file holding its own name), rooms 00 and 05's hash files
/// and the host-key file, with a self-signed certificate for `localhost` that the client trusts.
fn fixture(name: &str) -> Fixture {
    let dir = std::env::temp_dir().join(format!("npt-u6c-host-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("dist")).expect("a folder");
    for (_, file, _) in FILES {
        write(
            &dir.join("dist").join(file),
            format!("the {file}").as_bytes(),
        );
    }
    for nn in ["00", "05"] {
        write(
            &dir.join(format!("room-{nn}.json")),
            format!("{{\"room\":\"{nn}\"}}").as_bytes(),
        );
    }
    write(&dir.join("keys.txt"), b"rs.nt ssh-ed25519 AAAA\n");
    let key = rcgen::KeyPair::generate().expect("a key");
    let cert = rcgen::CertificateParams::new(vec!["localhost".into()])
        .and_then(|p| p.self_signed(&key))
        .expect("a certificate");
    write(&dir.join("chain.pem"), cert.pem().as_bytes());
    write(&dir.join("key.pem"), key.serialize_pem().as_bytes());
    let p = |n: &str| dir.join(n).to_str().expect("utf-8").to_owned();
    let s = Settings {
        bind: "127.0.0.1:0".into(),
        cert_chain: p("chain.pem"),
        cert_key: p("key.pem"),
        dist: p("dist"),
        hash_template: p("room-NN.json"),
        host_keys: p("keys.txt"),
    };
    let (bind, tls, host) = prepare(&s).expect("prepared");
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("rt");
    let listener = rt
        .block_on(tokio::net::TcpListener::bind(bind))
        .expect("bound");
    let addr = listener.local_addr().expect("an address");
    rt.spawn(serve(listener, tls, Arc::new(host)));
    let root = cert.der().clone();
    let client = Arc::new(client(&root, &rustls::version::TLS13));
    Fixture {
        dir,
        addr,
        rt,
        client,
        root,
    }
}

fn client(
    root: &CertificateDer<'static>,
    v: &'static rustls::SupportedProtocolVersion,
) -> rustls::ClientConfig {
    let mut roots = rustls::RootCertStore::empty();
    roots.add(root.clone()).expect("a root");
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut c = rustls::ClientConfig::builder_with_provider(provider)
        .with_protocol_versions(&[v])
        .expect("a version")
        .with_root_certificates(roots)
        .with_no_client_auth();
    c.alpn_protocols = vec![b"http/1.1".to_vec()];
    c
}

/// The raw answer to `req` (empty when the host closed without one), and the ALPN agreed.
fn raw(
    f: &Fixture,
    cfg: Arc<rustls::ClientConfig>,
    req: &[u8],
) -> io::Result<(Vec<u8>, Option<Vec<u8>>)> {
    f.rt.block_on(async {
        let tcp = tokio::net::TcpStream::connect(f.addr).await?;
        let name = rustls::pki_types::ServerName::try_from("localhost").expect("a name");
        let mut t = tokio_rustls::TlsConnector::from(cfg)
            .connect(name, tcp)
            .await?;
        let alpn = t.get_ref().1.alpn_protocol().map(<[u8]>::to_vec);
        t.write_all(req).await?;
        let mut out = Vec::new();
        let _ = t.read_to_end(&mut out).await;
        Ok((out, alpn))
    })
}

/// (status line, headers, body) of `method target`.
fn get(f: &Fixture, method: &str, target: &str) -> (String, Vec<(String, String)>, Vec<u8>) {
    let req = format!("{method} {target} HTTP/1.1\r\nHost: localhost\r\nUser-Agent: t\r\n\r\n");
    let (out, _) = raw(f, Arc::clone(&f.client), req.as_bytes()).expect("an answer");
    let at = out
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .expect("a head");
    let head = String::from_utf8(out[..at].to_vec()).expect("text");
    let mut lines = head.split("\r\n");
    let status = lines.next().unwrap_or_default().to_owned();
    let headers = lines
        .filter_map(|l| l.split_once(": "))
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();
    (status, headers, out[at + 4..].to_vec())
}

fn header<'a>(h: &'a [(String, String)], k: &str) -> Option<&'a str> {
    h.iter().find(|x| x.0 == k).map(|x| x.1.as_str())
}

/// The headers every answer carries (the CSP word for word, nosniff, no-referrer, close) and its length.
fn common(h: &[(String, String)], len: usize) {
    assert_eq!(header(h, "Content-Security-Policy"), Some(CSP));
    assert_eq!(header(h, "X-Content-Type-Options"), Some("nosniff"));
    assert_eq!(header(h, "Referrer-Policy"), Some("no-referrer"));
    assert_eq!(header(h, "Connection"), Some("close"));
    assert_eq!(header(h, "Content-Length"), Some(len.to_string().as_str()));
}

/// The settings refused (no NN, NN twice, a bad bind, a control character, an unknown field) and a missing dist
/// file refused at the start (exit 2's).
#[test]
fn u6c_host_settings_are_refused_and_a_missing_dist_file_stops_the_start() {
    let f = fixture("settings");
    let p = |n: &str| f.dir.join(n).to_str().expect("utf-8").to_owned();
    let good = || Settings {
        bind: "127.0.0.1:0".into(),
        cert_chain: p("chain.pem"),
        cert_key: p("key.pem"),
        dist: p("dist"),
        hash_template: p("room-NN.json"),
        host_keys: p("keys.txt"),
    };
    assert!(prepare(&good()).is_ok());
    let edits: [fn(&mut Settings); 4] = [
        |s| s.hash_template = s.hash_template.replace("NN", "00"),
        |s| s.hash_template.push_str("NN"),
        |s| s.bind = "localhost".into(),
        |s| s.dist.push('\u{7}'),
    ];
    for e in edits {
        let mut s = good();
        e(&mut s);
        assert!(prepare(&s).is_err());
    }
    std::fs::remove_file(f.dir.join("dist/hooks.js")).expect("removed");
    assert!(
        prepare(&good())
            .err()
            .is_some_and(|e| e.contains("hooks.js")),
        "a missing file"
    );
    let bad = br#"{"bind":"x","cert_chain":"","cert_key":"","dist":"","hash_template":"","host_keys":"","x":1}"#;
    assert!(
        serde_json::from_slice::<Settings>(bad).is_err(),
        "an unknown field"
    );
}

/// Every allowed path over TLS: its exact bytes, its type (`application/wasm` for the Wasm) and the common headers.
#[test]
fn u6c_every_allowed_path_has_its_bytes_type_and_headers() {
    let f = fixture("paths");
    for (path, file, kind) in FILES {
        let (status, h, body) = get(&f, "GET", path);
        assert_eq!(status, "HTTP/1.1 200 OK", "{path}");
        assert_eq!(body, format!("the {file}").as_bytes());
        assert_eq!(header(&h, "Content-Type"), Some(kind));
        assert_eq!(header(&h, "Cache-Control"), None);
        common(&h, body.len());
    }
}

/// HEAD: the same headers and length, no body.
#[test]
fn u6c_head_gives_the_headers_and_no_body() {
    let f = fixture("head");
    let (status, h, body) = get(&f, "HEAD", "/npt_client_wasm_bg.wasm");
    assert_eq!((status.as_str(), body.len()), ("HTTP/1.1 200 OK", 0));
    assert_eq!(header(&h, "Content-Type"), Some("application/wasm"));
    common(&h, "the npt_client_wasm_bg.wasm".len());
}

/// P14's test: every target outside the list is 404 (no prefix match, no traversal, no plain hash name, no room 32, no
/// sign in NN: SEC-U6C-3, with a `room-+5.json` there to be read if the check let `+5` through).
#[test]
fn u6c_targets_outside_the_list_are_404() {
    let f = fixture("404");
    write(&f.dir.join("room-+5.json"), b"{}");
    for t in [
        "/npt/cert-hash-+5.json",
        "/../Cargo.toml",
        "/%2e%2e/Cargo.toml",
        "/npt/",
        "/index.html",
        "/main.js/",
        "/main.jsx",
        "/npt/cert-hash.json",
        "/npt/cert-hash-32.json",
        "/npt/cert-hash-5.json",
        "/x?a=b",
    ] {
        let (status, h, body) = get(&f, "GET", t);
        assert_eq!(status, "HTTP/1.1 404 Not Found", "{t}");
        assert!(body.is_empty());
        common(&h, 0);
    }
}

/// C1 (e), P20's test: the path up to the first `?` is matched and the query ignored: `/?a=b` serves the index.
#[test]
fn u6c_a_query_is_ignored() {
    let f = fixture("query");
    let (status, _, body) = get(
        &f,
        "GET",
        "/?smoke=1&suite=smoke&room=05&transport=wt&attempt=1",
    );
    assert_eq!(
        (status.as_str(), body.as_slice()),
        ("HTTP/1.1 200 OK", &b"the index.html"[..])
    );
    assert_eq!(get(&f, "GET", "/main.js?v=2").2, b"the main.js");
}

/// GET and HEAD only: anything else 405 with `Allow`.
#[test]
fn u6c_other_methods_are_405() {
    let f = fixture("405");
    for m in ["POST", "PUT", "DELETE", "OPTIONS"] {
        let (status, h, _) = get(&f, m, "/");
        assert_eq!(status, "HTTP/1.1 405 Method Not Allowed", "{m}");
        assert_eq!(header(&h, "Allow"), Some("GET, HEAD"));
        common(&h, 0);
    }
}

/// P13's test: each room's hash file verbatim with `Cache-Control: no-store`; an absent one 404, one above 1 KiB 500.
#[test]
fn u6c_the_hash_files_are_served_verbatim_with_no_store_per_room() {
    let f = fixture("hash");
    for nn in ["00", "05"] {
        let (status, h, body) = get(&f, "GET", &format!("/npt/cert-hash-{nn}.json"));
        assert_eq!(status, "HTTP/1.1 200 OK");
        assert_eq!(
            body,
            std::fs::read(f.dir.join(format!("room-{nn}.json"))).expect("read")
        );
        assert_eq!(header(&h, "Cache-Control"), Some("no-store"));
        assert_eq!(header(&h, "Content-Type"), Some("application/json"));
    }
    assert_eq!(
        get(&f, "GET", "/npt/cert-hash-07.json").0,
        "HTTP/1.1 404 Not Found",
        "absent"
    );
    write(&f.dir.join("room-09.json"), &[b' '; 1_025]);
    assert_eq!(
        get(&f, "GET", "/npt/cert-hash-09.json").0,
        "HTTP/1.1 500 Internal Server Error"
    );
    write(&f.dir.join("room-09.json"), &[b' '; 1_024]);
    assert_eq!(
        get(&f, "GET", "/npt/cert-hash-09.json").2.len(),
        1_024,
        "at the cap: whole"
    );
}

/// P12's test: a replaced hash file is served anew (read per request, nothing cached).
#[test]
fn u6c_a_replaced_hash_file_is_served_anew() {
    let f = fixture("replaced");
    assert_eq!(
        get(&f, "GET", "/npt/cert-hash-05.json").2,
        br#"{"room":"05"}"#
    );
    write(&f.dir.join("room-05.json"), br#"{"room":"05","next":1}"#);
    assert_eq!(
        get(&f, "GET", "/npt/cert-hash-05.json").2,
        br#"{"room":"05","next":1}"#
    );
}

/// The host-key file verbatim with `no-store` (Q-U6C-6), as plain text.
#[test]
fn u6c_the_host_key_file_is_served_with_no_store() {
    let f = fixture("keys");
    let (status, h, body) = get(&f, "GET", "/npt/ssh-host-keys.txt");
    assert_eq!(
        (status.as_str(), body.as_slice()),
        ("HTTP/1.1 200 OK", &b"rs.nt ssh-ed25519 AAAA\n"[..])
    );
    assert_eq!(header(&h, "Cache-Control"), Some("no-store"));
    assert_eq!(
        header(&h, "Content-Type"),
        Some("text/plain; charset=utf-8")
    );
}

/// Q-U6C-4 (a): a request past the 8 KiB buffer, or with more than 32 headers, is closed with no answer.
#[test]
fn u6c_a_request_past_the_buffer_or_its_header_slots_is_closed() {
    let f = fixture("big");
    let long = format!("GET / HTTP/1.1\r\nX: {}\r\n\r\n", "a".repeat(9_000));
    let many = format!("GET / HTTP/1.1\r\n{}\r\n", "X: 1\r\n".repeat(33));
    for req in [long, many] {
        let (out, _) = raw(&f, Arc::clone(&f.client), req.as_bytes()).expect("connects");
        assert!(out.is_empty(), "no answer");
    }
    assert_eq!(get(&f, "GET", "/").0, "HTTP/1.1 200 OK", "the host goes on");
}

/// TLS 1.3 only (a TLS 1.2-only client is refused: security's condition 3) and the one ALPN, `http/1.1`.
#[test]
fn u6c_tls_1_3_only_and_the_one_alpn() {
    let f = fixture("tls");
    let (_, alpn) = raw(&f, Arc::clone(&f.client), b"GET / HTTP/1.1\r\n\r\n").expect("TLS 1.3");
    assert_eq!(alpn.as_deref(), Some(&b"http/1.1"[..]));
    let tls12 = Arc::new(client(&f.root, &rustls::version::TLS12));
    assert!(
        raw(&f, tls12, b"GET / HTTP/1.1\r\n\r\n").is_err(),
        "a TLS 1.2-only client"
    );
}
