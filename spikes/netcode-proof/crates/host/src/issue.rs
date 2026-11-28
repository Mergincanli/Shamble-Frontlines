//! The DNS-01 issue mode (u6c-brief.md Block 6, C1 (a); Q-U8-4; SEC-R1D-4; security-u6c-deps.md conditions 2-4;
//! INF-SB-6): `npt-host issue staging|production`, exactly two arguments, writing into the compiled-in /var/lib/npt/tls.
//!
//! One process across both phases, so no ACME account key or order state is ever on disk. Phase 1: a fresh account by
//! `Account::builder()` (the platform verifier; the fake directory's `builder_with_http` only under `cfg(test)`), its
//! key in memory only and its credentials dropped; the directory's profiles to stderr as one line
//! `PROFILES <name>,<name>…`; production orders only with `shortlived` listed (else exit 3, before any order) and asks
//! for it; staging takes `shortlived` when listed and its default otherwise; one order for the compiled-in name, its
//! one authorization's DNS-01 challenge; stdout `PROFILE <name>` (`default` for none), `TXT_NAME …`, `TXT_VALUE …`;
//! then one line `go` on stdin (read on a std thread under 16 bytes; anything else exit 2). Phase 2: `set_ready`,
//! `poll_ready` under a bounded `RetryPolicy` (Invalid or a timeout exit 4, never retried), a fresh ECDSA P-256 key and
//! its CSR by rcgen, `finalize_csr`, `poll_certificate`; the key (`<word>.key.pem`, PKCS#8, 0640) by `create_new`, its
//! `write_all` then `sync_all`; only then the chain as the server sent it, written to a temporary name (0644, synced)
//! and linked into place as `<word>.chain.pem` (a link refuses an existing name; the temporary one then removed), so a
//! watcher of the chain sees both files whole (INF-SB-6); stdout `SERIAL <hex>`, `SHA256 <hex of the leaf's DER>`,
//! `DONE`. Each ACME call under `tokio::time::timeout` (60 s), the whole under one 30-minute bound from the start. No
//! `install_default` or `set_default`. Exit 2 also for the directory (not owned by this user, or writable by others)
//! and for an output path that exists; 1 for the rest. Nothing but those six lines goes to stdout.

use std::io::{self, Read, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::Duration;

use instant_acme::{
    Account, ChallengeType, Identifier, LetsEncrypt, NewAccount, NewOrder, OrderStatus, RetryPolicy,
};
use sha2::{Digest, Sha256};
use tokio_rustls::rustls::pki_types::CertificateDer;
use tokio_rustls::rustls::pki_types::pem::PemObject;

/// The order's one name, compiled in (SEC-R1D-4: no free argument).
pub(crate) const NAME: &str = "rs.nt.shamblefrontlines.com";
const DIR: &str = "/var/lib/npt/tls";
const SHORTLIVED: &str = "shortlived";

/// One issue run's terms: the word and the output directory; the bounds (60 s a call, 30 minutes in all, the polls'
/// policy); in tests the fake directory's client.
pub(crate) struct Issue {
    pub(crate) word: String,
    pub(crate) dir: PathBuf,
    pub(crate) call: Duration,
    pub(crate) deadline: Duration,
    pub(crate) poll: RetryPolicy,
    #[cfg(test)]
    pub(crate) http: Option<Box<dyn instant_acme::HttpClient>>,
}

type Fail = (u8, String);

/// This process's effective uid, from /proc/self/status (std has no getuid; no libc here).
fn euid() -> Option<u32> {
    let s = std::fs::read_to_string("/proc/self/status").ok()?;
    s.lines()
        .find_map(|l| l.strip_prefix("Uid:"))?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}

/// The directory check: a directory owned by this user and writable by no one else.
pub(crate) fn check_dir(dir: &Path) -> Result<(), String> {
    let m = std::fs::metadata(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    if !m.is_dir() || Some(m.uid()) != euid() || m.mode() & 0o022 != 0 {
        return Err(format!(
            "{} is not a directory of this user writable by no one else",
            dir.display()
        ));
    }
    Ok(())
}

/// A DER element: (tag, content, what follows).
fn der(b: &[u8]) -> Option<(u8, &[u8], &[u8])> {
    let (&tag, &l0) = (b.first()?, b.get(1)?);
    let (len, at) = if l0 < 0x80 {
        (usize::from(l0), 2)
    } else {
        let n = usize::from(l0 & 0x7f);
        let len = b.get(2..2 + n).filter(|_| (1..=4).contains(&n))?;
        (
            len.iter().fold(0usize, |a, &x| a << 8 | usize::from(x)),
            2 + n,
        )
    };
    let end = at.checked_add(len)?;
    Some((tag, b.get(at..end)?, b.get(end..)?))
}

/// The leaf's serial number in lowercase hex (its sign byte left out), from its DER.
pub(crate) fn serial(leaf: &[u8]) -> Option<String> {
    let (_, cert, _) = der(leaf)?;
    let (_, tbs, _) = der(cert)?;
    let (tag, c, rest) = der(tbs)?;
    let (tag, int) = if tag == 0xa0 {
        der(rest).map(|(t, i, _)| (t, i))?
    } else {
        (tag, c)
    };
    let int = if int.len() > 1 && int[0] == 0 {
        &int[1..]
    } else {
        int
    };
    (tag == 0x02).then(|| int.iter().map(|b| format!("{b:02x}")).collect())
}

/// Writes `body` to a new file at `path` with `mode` exactly (set at creation, then on the open file whatever the umask),
/// then `sync_all`; an existing path is refused.
fn create(path: &Path, mode: u32, body: &[u8]) -> io::Result<()> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(mode)
        .open(path)?;
    f.set_permissions(std::fs::Permissions::from_mode(mode))?;
    f.write_all(body)?;
    f.sync_all()
}

/// One ACME call under its timeout: a timeout and an error are both code `code`'s.
async fn call<T, E: std::fmt::Display>(
    t: Duration,
    what: &str,
    code: u8,
    f: impl std::future::Future<Output = Result<T, E>>,
) -> Result<T, Fail> {
    match tokio::time::timeout(t, f).await {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(e)) => Err((code, format!("{what}: {e}"))),
        Err(_) => Err((code, format!("{what}: no answer within {} s", t.as_secs()))),
    }
}

/// One line `go` from `input`, read on a std thread under 16 bytes.
async fn go(mut input: Box<dyn Read + Send>) -> Result<(), Fail> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    std::thread::spawn(move || {
        let (mut buf, mut n) = ([0u8; 16], 0);
        while n < buf.len() && !buf[..n].contains(&b'\n') {
            match input.read(&mut buf[n..]) {
                Ok(0) | Err(_) => break,
                Ok(k) => n += k,
            }
        }
        let _ = tx.send(buf[..n].to_vec());
    });
    match rx.await {
        Ok(line) if line == b"go\n" => Ok(()),
        _ => Err((2, "stdin gave no `go` line".to_owned())),
    }
}

/// The run (the module text), its six lines to `out`, its one line to `err`.
pub(crate) async fn issue(
    i: Issue,
    input: Box<dyn Read + Send>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), Fail> {
    let url = match i.word.as_str() {
        "staging" => LetsEncrypt::Staging.url(),
        "production" => LetsEncrypt::Production.url(),
        w => return Err((2, format!("{w:?} is not staging or production"))),
    };
    check_dir(&i.dir).map_err(|e| (2, e))?;
    let key_path = i.dir.join(format!("{}.key.pem", i.word));
    let chain_path = i.dir.join(format!("{}.chain.pem", i.word));
    let tmp = i.dir.join(format!(".{}.chain.pem.new", i.word));
    if let Some(p) = [&key_path, &chain_path, &tmp]
        .into_iter()
        .find(|p| p.exists())
    {
        return Err((2, format!("{} exists", p.display())));
    }
    #[cfg(not(test))]
    let builder = Account::builder().map_err(|e| (1, format!("the ACME client: {e}")))?;
    #[cfg(test)]
    let builder = Account::builder_with_http(i.http.ok_or((1, "no fake directory".to_owned()))?);
    let new = NewAccount {
        contact: &[],
        terms_of_service_agreed: true,
        only_return_existing: false,
    };
    // The credentials (the account key) are dropped here: never written (Q-U8-4).
    let (account, _) = call(
        i.call,
        "the account",
        1,
        builder.create(&new, url.to_owned(), None),
    )
    .await?;
    let mut names: Vec<&str> = account.profiles().map(|p| p.name).collect();
    names.sort_unstable();
    let _ = writeln!(err, "PROFILES {}", names.join(","));
    let profile = names.contains(&SHORTLIVED).then_some(SHORTLIVED);
    if i.word == "production" && profile.is_none() {
        return Err((
            3,
            "production's directory lists no shortlived profile: no order".to_owned(),
        ));
    }
    let ids = [Identifier::Dns(NAME.to_owned())];
    let order = match profile {
        Some(p) => NewOrder::new(&ids).profile(p),
        None => NewOrder::new(&ids),
    };
    let mut order = call(i.call, "the order", 1, account.new_order(&order)).await?;
    {
        let mut auths = order.authorizations();
        let mut a = call(i.call, "the authorization", 1, async {
            auths
                .next()
                .await
                .unwrap_or(Err(instant_acme::Error::Str("no authorization")))
        })
        .await?;
        let mut ch = a
            .challenge(ChallengeType::Dns01)
            .ok_or((1, "no DNS-01 challenge".to_owned()))?;
        let value = ch.key_authorization().dns_value();
        let lines = format!(
            "PROFILE {}\nTXT_NAME _acme-challenge.{NAME}\nTXT_VALUE {value}\n",
            profile.unwrap_or("default")
        );
        out.write_all(lines.as_bytes())
            .and_then(|()| out.flush())
            .map_err(|e| (1, format!("stdout: {e}")))?;
        go(input).await?;
        call(i.call, "the challenge's ready", 1, ch.set_ready()).await?;
    }
    let poll = i.call + Duration::from_secs(300);
    match call(poll, "the order's poll", 4, order.poll_ready(&i.poll)).await? {
        OrderStatus::Ready => {}
        s => return Err((4, format!("the order is {s:?}, not ready: never retried"))),
    }
    let key = rcgen::KeyPair::generate().map_err(|e| (1, format!("the key: {e}")))?;
    let mut params = rcgen::CertificateParams::new(vec![NAME.to_owned()])
        .map_err(|e| (1, format!("the CSR: {e}")))?;
    params.distinguished_name = rcgen::DistinguishedName::new();
    let csr = params
        .serialize_request(&key)
        .map_err(|e| (1, format!("the CSR: {e}")))?;
    call(i.call, "the finalize", 1, order.finalize_csr(csr.der())).await?;
    let chain = call(
        poll,
        "the certificate's poll",
        4,
        order.poll_certificate(&i.poll),
    )
    .await?;
    let leaf = CertificateDer::pem_slice_iter(chain.as_bytes())
        .next()
        .and_then(Result::ok)
        .ok_or((1, "the chain holds no certificate".to_owned()))?;
    let serial = serial(leaf.as_ref()).ok_or((1, "the leaf's serial".to_owned()))?;
    let sha: String = Sha256::digest(leaf.as_ref())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let io = |e: io::Error| {
        (
            if e.kind() == io::ErrorKind::AlreadyExists {
                2
            } else {
                1
            },
            e.to_string(),
        )
    };
    create(&key_path, 0o640, key.serialize_pem().as_bytes()).map_err(io)?;
    create(&tmp, 0o644, chain.as_bytes()).map_err(io)?;
    std::fs::hard_link(&tmp, &chain_path).map_err(io)?;
    std::fs::remove_file(&tmp).map_err(io)?;
    let lines = format!("SERIAL {serial}\nSHA256 {sha}\nDONE\n");
    out.write_all(lines.as_bytes())
        .and_then(|()| out.flush())
        .map_err(|e| (1, format!("stdout: {e}")))
}

/// [`issue`] under its whole bound (30 minutes from the start, both phases).
pub(crate) async fn run(
    i: Issue,
    input: Box<dyn Read + Send>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Result<(), Fail> {
    let deadline = i.deadline;
    match tokio::time::timeout(deadline, issue(i, input, out, err)).await {
        Ok(r) => r,
        Err(_) => Err((1, format!("the {} s bound passed", deadline.as_secs()))),
    }
}

/// `npt-host issue staging|production` (the module text).
pub(crate) fn main(word: &str) -> std::process::ExitCode {
    let i = Issue {
        word: word.to_owned(),
        dir: PathBuf::from(DIR),
        call: Duration::from_secs(60),
        deadline: Duration::from_secs(30 * 60),
        poll: RetryPolicy::new()
            .initial_delay(Duration::from_secs(2))
            .backoff(1.5)
            .timeout(Duration::from_secs(300)),
        #[cfg(test)]
        http: None,
    };
    let rt = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => return fail((1, format!("the runtime: {e}"))),
    };
    let (mut out, mut err) = (io::stdout(), io::stderr());
    match rt.block_on(run(i, Box::new(io::stdin()), &mut out, &mut err)) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(f) => fail(f),
    }
}

fn fail((code, why): Fail) -> std::process::ExitCode {
    eprintln!("npt-host issue: {why}");
    std::process::ExitCode::from(code)
}

#[cfg(test)]
#[path = "issue_tests.rs"]
mod tests;
