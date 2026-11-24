//! The room's WebTransport certificate (u6b-brief.md Block 6, S8b; U6-Q17; replan-units.md:119-121; conditions 4 and
//! 6): made at start, rcgen ECDSA P-256, X.509 v3, valid 13 days (`serverCertificateHashes` takes at most 14); its key
//! in memory only, never written; its SHA-256 written to the settings' path as the hash file U6c's host serves.
//!
//! **The validity dates (condition 6):** `not_before` is today's civil date at 00:00 UTC from one reviewed
//! `SystemTime::now` read (`today`, the one allow: calendar validity), by a days-to-civil routine; `not_after` the civil
//! date 13 days on, both through `rcgen::date_time_ymd` (so `time` is never named). A clock before 1970 or a date past
//! 9999 is a settings-class refusal (exit 2), never a panic. The read feeds no timing series.
//!
//! **The hash file (condition 4):** serde_json of exactly `algorithm`, `value_hex` (64 lowercase hex digits) and
//! `validity_ns`, made by `create_new` with mode 0644 at creation, one `write_all` then `sync_all`, after the
//! certificate is made and before OPEN; an existing path is refused (exit 2), so U8's run plan removes the previous
//! room's file before any restart.

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;

use serde::Serialize;
use sha2::{Digest, Sha256};

/// The validity in days (`serverCertificateHashes`: at most 14).
pub(crate) const VALIDITY_DAYS: i64 = 13;
/// The validity in ns (`header.params.cert_validity_ns`).
pub(crate) const VALIDITY_NS: u64 = VALIDITY_DAYS as u64 * 86_400 * 1_000_000_000;

/// The room's certificate: its DER, its PKCS#8 key (memory only) and its SHA-256.
pub(crate) struct DevCert {
    pub(crate) der: Vec<u8>,
    pub(crate) key: Vec<u8>,
    pub(crate) hash: [u8; 32],
}

/// Days since 1970-01-01 of the civil date `(y, m, d)` (proleptic Gregorian; H. Hinnant's `days_from_civil`): the
/// tests' inverse of `civil_from_days`.
#[cfg(test)]
pub(crate) fn days_from_civil(y: i64, m: u8, d: u8) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let m = i64::from(m);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(d) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// The civil date of `days` since 1970-01-01 (`civil_from_days`).
pub(crate) fn civil_from_days(days: i64) -> (i64, u8, u8) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
    let y = yoe + era * 400 + i64::from(m <= 2);
    (y, m, d)
}

/// Today: whole days since 1970-01-01 (UTC), from the one wall-clock read.
///
/// # Errors
/// A clock before 1970.
#[allow(
    clippy::disallowed_methods,
    clippy::disallowed_types,
    reason = "U6b's one wall-clock read (u6b-brief.md condition 6): the certificate's calendar validity, never a timing \
              series; every other read in npt-server is clock.rs's monotonic clock"
)]
pub(crate) fn today() -> Result<i64, String> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| (d.as_secs() / 86_400) as i64)
        .map_err(|_| "the clock reads before 1970".to_owned())
}

/// The validity dates from `today` (days since 1970): today 00:00 UTC and 13 days on.
///
/// # Errors
/// A date before 1970 or past 9999.
pub(crate) fn validity(today: i64) -> Result<[(i32, u8, u8); 2], String> {
    let mut out = [(0, 0, 0); 2];
    for (o, days) in out.iter_mut().zip([today, today + VALIDITY_DAYS]) {
        let (y, m, d) = civil_from_days(days);
        if !(1970..=9999).contains(&y) {
            return Err(format!(
                "the certificate's date {y:04}-{m:02}-{d:02} is outside 1970 to 9999"
            ));
        }
        *o = (y as i32, m, d);
    }
    Ok(out)
}

/// The room's certificate for `today` (days since 1970).
///
/// # Errors
/// A date out of range, or rcgen refused.
pub(crate) fn make(today: i64) -> Result<DevCert, String> {
    let [(y0, m0, d0), (y1, m1, d1)] = validity(today)?;
    let key = rcgen::KeyPair::generate().map_err(|e| format!("the certificate's key: {e}"))?;
    let mut params = rcgen::CertificateParams::new(vec!["npt-room".to_owned()])
        .map_err(|e| format!("the certificate's names: {e}"))?;
    params.not_before = rcgen::date_time_ymd(y0, m0, d0);
    params.not_after = rcgen::date_time_ymd(y1, m1, d1);
    let mut dn = rcgen::DistinguishedName::new();
    dn.push(rcgen::DnType::CommonName, "npt-room");
    params.distinguished_name = dn;
    let cert = params
        .self_signed(&key)
        .map_err(|e| format!("the certificate: {e}"))?;
    let der = cert.der().to_vec();
    let hash = Sha256::digest(&der).into();
    Ok(DevCert {
        der,
        key: key.serialize_der(),
        hash,
    })
}

/// The hash file's form (condition 4): exactly these three fields.
#[derive(Serialize)]
struct HashFile {
    algorithm: &'static str,
    value_hex: String,
    validity_ns: u64,
}

/// Writes the hash file at `path` (the module text): `AlreadyExists` for an existing path.
pub(crate) fn write_hash(path: &str, hash: &[u8; 32]) -> io::Result<()> {
    let body = HashFile {
        algorithm: "sha-256",
        value_hex: hash.iter().map(|b| format!("{b:02x}")).collect(),
        validity_ns: VALIDITY_NS,
    };
    let text = serde_json::to_vec(&body).map_err(io::Error::other)?;
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o644)
        .open(path)?;
    f.write_all(&text)?;
    f.sync_all()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Condition 6: the days-to-civil routine on fixed dates, both ways: the epoch, a leap day, a century's 1 March
    /// after a non-leap February, a year's end and 13 days on, and the brief's date.
    #[test]
    fn u6b_days_to_civil_on_fixed_dates() {
        for (days, ymd) in [
            (0, (1970, 1, 1)),
            (10_957, (2000, 1, 1)),
            (11_016, (2000, 2, 29)),
            (20_779, (2026, 11, 22)),
        ] {
            assert_eq!(civil_from_days(days), ymd);
            assert_eq!(days_from_civil(ymd.0, ymd.1, ymd.2), days);
        }
        assert_eq!(
            civil_from_days(days_from_civil(2100, 2, 28) + 1),
            (2100, 3, 1),
            "2100 is no leap year"
        );
        let end = days_from_civil(2025, 12, 31);
        assert_eq!(civil_from_days(end + VALIDITY_DAYS), (2026, 1, 13));
        assert_eq!(validity(end), Ok([(2025, 12, 31), (2026, 1, 13)]));
        assert!(validity(-1).is_err(), "before 1970");
        assert!(
            validity(days_from_civil(9999, 12, 25)).is_err(),
            "past 9999"
        );
        assert_eq!(VALIDITY_NS, 1_123_200_000_000_000);
    }

    /// Where `needle` first occurs in `hay`.
    fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
        hay.windows(needle.len()).position(|w| w == needle)
    }

    /// Block 6 (a test-local DER read, no new dev-dependency): the certificate is X.509 v3 (`[0] INTEGER 2`), signed
    /// ECDSA with SHA-256 on a P-256 key, valid from the day's 00:00 UTC for 13 days exactly; its hash is its SHA-256;
    /// the key is PKCS#8.
    #[test]
    fn u6b_the_dev_certificate_is_ecdsa_p256_v3_and_valid_13_days() {
        let today = days_from_civil(2026, 11, 22);
        let c = make(today).expect("made");
        assert!(
            find(&c.der, &[0xa0, 0x03, 0x02, 0x01, 0x02]).is_some(),
            "v3"
        );
        let ecdsa_sha256 = [0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02];
        let p256 = [0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07];
        assert!(find(&c.der, &ecdsa_sha256).is_some() && find(&c.der, &p256).is_some());
        let at = find(&c.der, b"261122000000Z").expect("not before: the day's 00:00 UTC");
        let until = find(&c.der, b"261205000000Z").expect("not after: 13 days on");
        assert!(at < until);
        assert_eq!(c.hash, <[u8; 32]>::from(Sha256::digest(&c.der)));
        assert!(c.key.len() > 100 && c.key[0] == 0x30, "a PKCS#8 key");
    }

    /// Condition 4: the hash file holds exactly the three fields, 64 lowercase hex digits; it is made 0644; an existing
    /// path is refused and left as it was; no key bytes are in it.
    #[test]
    fn u6b_the_hash_file_is_made_new_with_its_three_fields_and_no_key() {
        use std::os::unix::fs::PermissionsExt;
        let c = make(days_from_civil(2026, 11, 22)).expect("made");
        let dir = std::env::temp_dir().join(format!("npt-u6b-hash-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a folder");
        let path = dir.join("hash.json");
        let p = path.to_str().expect("utf-8");
        write_hash(p, &c.hash).expect("written");
        let files = std::fs::read_dir(&dir).expect("listed").count();
        assert_eq!(files, 1, "the hash file alone: no key beside it");
        let text = std::fs::read(&path).expect("read");
        let v: serde_json::Value = serde_json::from_slice(&text).expect("JSON");
        let o = v.as_object().expect("an object");
        assert_eq!(o.len(), 3);
        assert_eq!(o["algorithm"], "sha-256");
        assert_eq!(o["validity_ns"], VALIDITY_NS);
        let hex = o["value_hex"].as_str().expect("a string");
        assert!(
            hex.len() == 64
                && hex
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        );
        let mode = std::fs::metadata(&path)
            .expect("there")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o644);
        assert!(
            find(&text, &c.key[c.key.len() - 32..]).is_none(),
            "no key bytes"
        );
        let again = write_hash(p, &c.hash).expect_err("an existing path is refused");
        assert_eq!(again.kind(), io::ErrorKind::AlreadyExists);
        assert_eq!(
            std::fs::read(&path).expect("read"),
            text,
            "and left as it was"
        );
        std::fs::remove_dir_all(&dir).expect("removed");
    }
}
