//! The content as files: the folder `content/` at the repository's root, read as the loader reads a bundle, and the
//! one bundle the build ships (`content.json`). The `content-bundle` tool is this module's [`run`]; the plain test
//! run calls the same functions (`tests/content_files.rs`), so CI holds them with no line of its own.
//!
//! The folder's layout is the bundle's paths: one folder per kind (`content::bundle::FOLDERS`), each holding
//! `<id>.json` files and nothing else. Beside them `source/` holds what content is made from (Mustafa's Export
//! and its table): it is no content, the tool never opens it, and Mustafa's SOURCE-GO keeps changes there his.
//!
//! The repository's tree is not trusted (a branch can commit a link). So the walk asks every entry for its own
//! type without following it and refuses whatever is not what the layout says: a link in a file's or a folder's
//! place, a pipe, a folder inside a kind's folder (a `source` there too), a folder it does not know, a name outside
//! the grammar of an id. A file is read through the plain-file reader, at most the loader's own cap and one byte.
//! Whether a file is content is the loader's to say, and only the loader's: canonical form, the kind's fields and
//! ranges, the id against the file's name.

use std::path::Path;

use content::bundle::{FOLDERS, MAX_FILE_BYTES};

use crate::files::{is_plain_folder, read_plain_within, replace_plain};
use crate::sha256::{hex, sha256};

/// The folder beside the kinds' that holds no content.
pub const SOURCE: &str = "source";
/// The bundle's file name, in whatever folder the build names.
pub const BUNDLE: &str = "content.json";

/// The entries of one folder with their own types, sorted by name: a link is a link here, never what it leads to.
fn entries(folder: &Path, said: &str) -> Result<Vec<(String, std::fs::FileType)>, String> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(folder).map_err(|error| format!("{said}: {error}"))? {
        let entry = entry.map_err(|error| format!("{said}: {error}"))?;
        let kind = entry
            .file_type()
            .map_err(|error| format!("{said}: {error}"))?;
        let name = entry.file_name().into_string().map_err(|name| {
            format!(
                "{said}: {:?} is no name a content path can hold",
                name.display()
            )
        })?;
        found.push((name, kind));
    }
    found.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(found)
}

/// Every content file under `content`, as `(path, text)` in the order of the paths' bytes, read and not yet judged.
///
/// # Errors
/// A folder or a file that is not what the layout says, by its path; a file larger than the loader takes, or whose
/// bytes are no text.
pub fn walk(content: &Path) -> Result<Vec<(String, String)>, String> {
    if !is_plain_folder(content) {
        return Err(format!(
            "{:?} is not there as a plain folder",
            content.display()
        ));
    }
    let mut files = Vec::new();
    for (folder, kind) in entries(content, "content")? {
        if !kind.is_dir() {
            return Err(format!(
                "content/{folder} is not a folder (a file, a link or another type): content holds one folder per kind and `{SOURCE}`"
            ));
        }
        if folder == SOURCE {
            continue;
        }
        if !FOLDERS.contains(&folder.as_str()) {
            return Err(format!(
                "content/{folder} is no kind's folder: the kinds are {FOLDERS:?}"
            ));
        }
        for (name, kind) in entries(&content.join(&folder), &format!("content/{folder}"))? {
            let path = format!("{folder}/{name}");
            if !kind.is_file() {
                return Err(format!(
                    "content/{path} is not a plain file (a folder, a link or another type)"
                ));
            }
            if !name.strip_suffix(".json").is_some_and(content::json::is_id) {
                return Err(format!("content/{path}: a content file is named <id>.json"));
            }
            let cap = u64::try_from(MAX_FILE_BYTES).unwrap_or(u64::MAX);
            let bytes = read_plain_within(&content.join(&folder), &name, cap)
                .map_err(|why| format!("content/{folder}: {why}"))?;
            let text = String::from_utf8(bytes)
                .map_err(|_| format!("content/{path} is not text (UTF-8)"))?;
            files.push((path, text));
        }
    }
    files.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    Ok(files)
}

/// The bundle of these files, after the loader took each file and then the whole: what `content.json` holds.
///
/// # Errors
/// The loader's own refusal, in its own words, with the file and the place in it.
pub fn bundle(files: &[(String, String)]) -> Result<String, String> {
    // Each file on its own first: its refusal then names the file's line and column, not the bundle's.
    for (path, text) in files {
        content::load_file(path, text.as_bytes()).map_err(|error| error.message())?;
    }
    let pairs: Vec<(&str, &str)> = files
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();
    let text = content::bundle::write(&pairs);
    content::load(text.as_bytes()).map_err(|error| error.message())?;
    Ok(text)
}

/// The tool. With no `out`: reads `content`, asks the loader, writes nothing (`--check`). With `out`, a folder
/// that exists and lies outside `content`: also puts the bundle there as `content.json`.
///
/// # Errors
/// What [`walk`] and [`bundle`] give; an `out` inside the content folder; a bundle that could not be written.
pub fn run(content: &Path, out: Option<&Path>) -> Result<String, String> {
    let files = walk(content)?;
    let text = bundle(&files)?;
    let said = format!(
        "{} content files, a bundle of {} bytes, sha256 {}",
        files.len(),
        text.len(),
        hex(&sha256(text.as_bytes()))
    );
    let Some(out) = out else {
        return Ok(format!(
            "{said}: every file is what the loader takes; nothing written"
        ));
    };
    if !is_plain_folder(out) {
        return Err(format!("{:?} is not a plain folder", out.display()));
    }
    // Where the two folders really are, whatever the spelling: `..` and links in the way are resolved.
    let real = |path: &Path| {
        std::fs::canonicalize(path).map_err(|error| format!("{}: {error}", path.display()))
    };
    if real(out)?.starts_with(real(content)?) {
        return Err(format!(
            "{:?} lies inside the content folder: the bundle is build output, never content",
            out.display()
        ));
    }
    replace_plain(out, BUNDLE, text.as_bytes())?;
    Ok(format!("{said}: written to {}", out.join(BUNDLE).display()))
}
