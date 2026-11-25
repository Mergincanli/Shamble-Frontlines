//! Block 6's tests (u6c-brief.md section 3, C1 (a)): the issue mode against an in-process fake ACME directory, a test
//! `HttpClient` given to `Account::builder_with_http` (U6-Q18's term; no network), on short test bounds.

use super::*;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use instant_acme::{BodyWrapper, BytesResponse, HttpClient};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio_rustls::rustls::pki_types::PrivateKeyDer;

const BASE: &str = "https://fake.test";

/// The fake directory's state: what it lists and what it saw.
#[derive(Default)]
struct Dir {
    profiles: Vec<&'static str>,
    /// The order's `profile` as sent (None: no order yet).
    ordered: Option<Option<String>>,
    invalid: bool,
    hang: Option<&'static str>,
    finalized: bool,
    refreshes: u32,
    requests: u32,
    pem: String,
}

#[derive(Clone)]
struct Fake(Arc<Mutex<Dir>>);

/// base64url without padding, decoded (the JWS payload).
fn unb64(t: &[u8]) -> Vec<u8> {
    let v = |c: u8| match c {
        b'A'..=b'Z' => c - b'A',
        b'a'..=b'z' => c - b'a' + 26,
        b'0'..=b'9' => c - b'0' + 52,
        b'-' => 62,
        _ => 63,
    };
    let (mut acc, mut bits, mut out) = (0u32, 0, Vec::new());
    for &c in t {
        acc = (acc << 6 | u32::from(v(c))) & 0xffff;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    out
}

fn order(status: &str, cert: bool) -> String {
    let mut o = serde_json::json!({"status": status, "authorizations": [format!("{BASE}/authz/1")],
        "finalize": format!("{BASE}/finalize/1")});
    if cert {
        o["certificate"] = format!("{BASE}/cert/1").into();
    }
    o.to_string()
}

/// (status, Location, body) for one request: RFC 8555's flow, one order, one authorization, one DNS-01 challenge.
fn route(d: &mut Dir, path: &str, body: &[u8]) -> (u16, Option<String>, String) {
    d.requests += 1;
    match path {
        "/directory" => {
            let p: serde_json::Map<_, _> = d
                .profiles
                .iter()
                .map(|n| (n.to_string(), "a profile".into()))
                .collect();
            let dir = serde_json::json!({"newNonce": format!("{BASE}/nonce"), "newAccount": format!("{BASE}/acct"),
                "newOrder": format!("{BASE}/order"), "meta": {"profiles": p}});
            (200, None, dir.to_string())
        }
        "/nonce" => (200, None, String::new()),
        "/acct" => (201, Some(format!("{BASE}/acct/1")), "{}".into()),
        "/order" => {
            let jws: serde_json::Value = serde_json::from_slice(body).expect("a JWS");
            let payload: serde_json::Value = serde_json::from_slice(&unb64(
                jws["payload"].as_str().expect("a payload").as_bytes(),
            ))
            .expect("json");
            d.ordered = Some(payload["profile"].as_str().map(str::to_owned));
            (
                201,
                Some(format!("{BASE}/order/1")),
                order("pending", false),
            )
        }
        "/authz/1" => {
            let ch = serde_json::json!({"type": "dns-01", "url": format!("{BASE}/chall/1"), "token": "tok",
                "status": "pending"});
            let a = serde_json::json!({"status": "pending", "identifier": {"type": "dns", "value": NAME},
                "challenges": [ch]});
            (200, None, a.to_string())
        }
        "/chall/1" => {
            let ch = serde_json::json!({"type": "dns-01", "url": format!("{BASE}/chall/1"), "token": "tok",
                "status": "processing"});
            (200, None, ch.to_string())
        }
        "/order/1" => {
            d.refreshes += 1;
            let s = match (d.invalid, d.finalized) {
                (true, _) => "invalid",
                (false, true) => "valid",
                (false, false) => "ready",
            };
            (200, None, order(s, d.finalized))
        }
        "/finalize/1" => {
            d.finalized = true;
            (200, None, order("processing", false))
        }
        "/cert/1" => (200, None, d.pem.clone()),
        _ => (
            404,
            None,
            r#"{"type":"urn:ietf:params:acme:error:malformed"}"#.into(),
        ),
    }
}

impl HttpClient for Fake {
    fn request(
        &self,
        req: http::Request<BodyWrapper<Bytes>>,
    ) -> Pin<Box<dyn Future<Output = Result<BytesResponse, instant_acme::Error>> + Send>> {
        let d = Arc::clone(&self.0);
        Box::pin(async move {
            let (parts, body) = req.into_parts();
            let body = body
                .collect()
                .await
                .map(|b| b.to_bytes())
                .unwrap_or_default();
            let path = parts.uri.path().to_owned();
            let hang = d.lock().unwrap().hang.is_some_and(|h| h == path);
            if hang {
                std::future::pending::<()>().await;
            }
            let (status, location, json) = route(&mut d.lock().unwrap(), &path, &body);
            let mut r = http::Response::builder()
                .status(status)
                .header("Replay-Nonce", "nonce");
            if let Some(l) = location {
                r = r.header("Location", l);
            }
            Ok(BytesResponse::from(
                r.body(Full::new(Bytes::from(json))).expect("a response"),
            ))
        })
    }
}

/// A fake listing `profiles`, its leaf with serial 01 23 45.
fn fake(profiles: &[&'static str]) -> (Fake, Arc<Mutex<Dir>>, Vec<u8>) {
    let key = rcgen::KeyPair::generate().expect("a key");
    let mut p = rcgen::CertificateParams::new(vec![NAME.to_owned()]).expect("params");
    p.serial_number = Some(rcgen::SerialNumber::from(vec![0x01, 0x23, 0x45]));
    let cert = p.self_signed(&key).expect("a leaf");
    let d = Arc::new(Mutex::new(Dir {
        profiles: profiles.to_vec(),
        pem: cert.pem(),
        ..Dir::default()
    }));
    (Fake(Arc::clone(&d)), d, cert.der().to_vec())
}

/// A new 0700 folder.
fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("npt-u6c-issue-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a folder");
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).expect("0700");
    dir
}

/// One run of `word` into `dir` against `fake`, stdin `input`, under `call` a call and 3 s in all: the result, stdout
/// and stderr.
fn run_with(
    word: &str,
    dir: &Path,
    f: Fake,
    input: Box<dyn Read + Send>,
    call: Duration,
) -> (Result<(), Fail>, String, String) {
    let i = Issue {
        word: word.to_owned(),
        dir: dir.to_owned(),
        call,
        deadline: Duration::from_secs(3),
        poll: RetryPolicy::new()
            .initial_delay(Duration::from_millis(5))
            .backoff(1.0)
            .timeout(Duration::from_secs(1)),
        http: Some(Box::new(f)),
    };
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("rt");
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let r = rt.block_on(super::run(i, input, &mut out, &mut err));
    (
        r,
        String::from_utf8(out).expect("utf-8"),
        String::from_utf8(err).expect("utf-8"),
    )
}

fn go_line() -> Box<dyn Read + Send> {
    Box::new(io::Cursor::new(b"go\n".to_vec()))
}

fn files(dir: &Path) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .expect("listed")
        .map(|e| e.expect("an entry").file_name().to_string_lossy().into())
        .collect();
    v.sort();
    v
}

fn mode(p: &Path) -> u32 {
    std::fs::metadata(p).expect("there").permissions().mode() & 0o777
}

/// C1 (a), P17's and P21's test: a staging order then a production one in one directory both succeed; stdout holds
/// exactly the six lines (the profile, the TXT name and value; after `go` the serial, the leaf's SHA-256, DONE) and
/// stderr the one profile list; the four files, each key 0640 (PKCS#8) and each chain 0644 as the server sent it, no
/// temporary name left.
#[test]
fn u6c_staging_then_production_in_one_directory_print_the_six_lines_and_write_the_four_files() {
    let dir = folder("both");
    for word in ["staging", "production"] {
        let (f, d, leaf) = fake(&["tlsserver", "shortlived", "classic"]);
        let (r, out, err) = run_with(word, &dir, f, go_line(), Duration::from_secs(1));
        r.expect(word);
        let lines: Vec<&str> = out.lines().collect();
        let sha: String = Sha256::digest(&leaf)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(lines.len(), 6, "{out}");
        assert_eq!(
            lines[..2],
            [
                "PROFILE shortlived",
                "TXT_NAME _acme-challenge.rs.nt.shamblefrontlines.com"
            ]
        );
        let v = lines[2].strip_prefix("TXT_VALUE ").expect("the value");
        assert!(
            v.len() == 43
                && v.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        );
        assert_eq!(
            lines[3..],
            ["SERIAL 012345", &format!("SHA256 {sha}"), "DONE"]
        );
        assert_eq!(err, "PROFILES classic,shortlived,tlsserver\n");
        let key = std::fs::read(dir.join(format!("{word}.key.pem"))).expect("the key");
        let parsed = PrivateKeyDer::from_pem_slice(&key);
        assert!(
            matches!(parsed, Ok(PrivateKeyDer::Pkcs8(_))),
            "a PKCS#8 key"
        );
        assert_eq!(mode(&dir.join(format!("{word}.key.pem"))), 0o640);
        assert_eq!(mode(&dir.join(format!("{word}.chain.pem"))), 0o644);
        let chain =
            std::fs::read_to_string(dir.join(format!("{word}.chain.pem"))).expect("the chain");
        assert_eq!(chain, d.lock().unwrap().pem, "verbatim");
    }
    let want = [
        "production.chain.pem",
        "production.key.pem",
        "staging.chain.pem",
        "staging.key.pem",
    ];
    assert_eq!(files(&dir), want);
}

/// P15's test: the production order asks for `shortlived` (the fake reads the order's payload).
#[test]
fn u6c_the_production_order_asks_for_shortlived() {
    let dir = folder("profile");
    let (f, d, _) = fake(&["classic", "shortlived"]);
    run_with("production", &dir, f, go_line(), Duration::from_secs(1))
        .0
        .expect("issued");
    assert_eq!(
        d.lock().unwrap().ordered,
        Some(Some("shortlived".to_owned()))
    );
}

/// Production with no `shortlived` listed exits 3 before any order and writes nothing (fails closed).
#[test]
fn u6c_production_without_shortlived_exits_3_before_any_order() {
    let dir = folder("closed");
    let (f, d, _) = fake(&["classic"]);
    let (r, out, _) = run_with("production", &dir, f, go_line(), Duration::from_secs(1));
    assert_eq!(r.map_err(|e| e.0), Err(3));
    assert!(out.is_empty() && d.lock().unwrap().ordered.is_none() && files(&dir).is_empty());
}

/// Staging with no `shortlived` listed orders its default and says so (`PROFILE default`).
#[test]
fn u6c_staging_without_shortlived_orders_its_default_and_says_so() {
    let dir = folder("default");
    let (f, d, _) = fake(&["classic"]);
    let (r, out, _) = run_with("staging", &dir, f, go_line(), Duration::from_secs(1));
    r.expect("staging");
    assert!(out.starts_with("PROFILE default\n"));
    assert_eq!(d.lock().unwrap().ordered, Some(None), "no profile asked");
}

/// P16's test: an Invalid order exits 4 after one poll sequence (one refresh: never retried), nothing written.
#[test]
fn u6c_an_invalid_order_exits_4_after_one_poll_and_writes_nothing() {
    let dir = folder("invalid");
    let (f, d, _) = fake(&["shortlived"]);
    d.lock().unwrap().invalid = true;
    let (r, out, _) = run_with("staging", &dir, f, go_line(), Duration::from_secs(1));
    assert_eq!(r.map_err(|e| e.0), Err(4));
    assert_eq!((d.lock().unwrap().refreshes, out.lines().count()), (1, 3));
    assert!(files(&dir).is_empty());
}

/// stdin that never yields, until the test ends.
struct Never(std::sync::mpsc::Receiver<()>);
impl Read for Never {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        let _ = self.0.recv();
        Ok(0)
    }
}

/// No `go` before the whole bound (3 s here) exits non-zero, nothing written.
#[test]
fn u6c_no_go_within_the_bound_exits_non_zero_and_writes_nothing() {
    let dir = folder("no-go");
    let (_hold, rx) = std::sync::mpsc::channel::<()>();
    let (f, _, _) = fake(&["shortlived"]);
    let (r, _, _) = run_with(
        "staging",
        &dir,
        f,
        Box::new(Never(rx)),
        Duration::from_secs(1),
    );
    assert_eq!(r.map_err(|e| e.0), Err(1));
    assert!(files(&dir).is_empty());
}

/// Any line but `go` exits 2, nothing written.
#[test]
fn u6c_another_line_than_go_exits_2() {
    let dir = folder("yes");
    let (f, _, _) = fake(&["shortlived"]);
    let yes = Box::new(io::Cursor::new(b"yes\n".to_vec()));
    assert_eq!(
        run_with("staging", &dir, f, yes, Duration::from_secs(1))
            .0
            .map_err(|e| e.0),
        Err(2)
    );
    assert!(files(&dir).is_empty());
}

/// SEC-R1D-4: a call that never answers ends by its own timeout (200 ms here), nothing written.
#[test]
fn u6c_a_call_that_never_answers_ends_by_its_timeout() {
    let dir = folder("hang");
    let (f, d, _) = fake(&["shortlived"]);
    d.lock().unwrap().hang = Some("/order");
    let (r, out, _) = run_with("staging", &dir, f, go_line(), Duration::from_millis(200));
    let (code, why) = r.expect_err("timed out");
    assert!(code == 1 && why.contains("no answer"), "{why}");
    assert!(out.is_empty() && files(&dir).is_empty());
}

/// `word` into `dir` exits 2 before any call.
fn refused_before_any_call(word: &str, dir: &Path) {
    let (f, d, _) = fake(&["shortlived"]);
    let (r, out, _) = run_with(word, dir, f, go_line(), Duration::from_secs(1));
    assert_eq!(r.map_err(|e| e.0), Err(2), "{word}");
    assert!(
        out.is_empty() && d.lock().unwrap().requests == 0,
        "{word}: no call"
    );
}

/// An existing output path exits 2 before any call and is left as it was.
#[test]
fn u6c_an_existing_output_path_exits_2_before_any_call() {
    let dir = folder("exists");
    std::fs::write(dir.join("staging.chain.pem"), b"there").expect("written");
    refused_before_any_call("staging", &dir);
    assert_eq!(
        std::fs::read(dir.join("staging.chain.pem")).expect("read"),
        b"there"
    );
}

/// A word outside the list exits 2 before any call (the directory URL is a constant per word: SEC-R1D-4).
#[test]
fn u6c_a_word_outside_the_list_exits_2() {
    refused_before_any_call("prod", &folder("word"));
}

/// C1 (a)'s directory check: one others may write to exits 2 before any call.
#[test]
fn u6c_a_directory_others_may_write_exits_2() {
    let dir = folder("open");
    std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o777)).expect("0777");
    refused_before_any_call("production", &dir);
}

/// The leaf's serial from its DER, its sign byte left out.
#[test]
fn u6c_the_leafs_serial_reads_from_its_der() {
    let key = rcgen::KeyPair::generate().expect("a key");
    let mut p = rcgen::CertificateParams::new(vec![NAME.to_owned()]).expect("params");
    p.serial_number = Some(rcgen::SerialNumber::from(vec![0x00, 0x9a, 0x01]));
    let cert = p.self_signed(&key).expect("a leaf");
    assert_eq!(serial(cert.der()).as_deref(), Some("9a01"));
    assert_eq!(serial(b"\x30\x03\x30\x01"), None, "cut short");
}
