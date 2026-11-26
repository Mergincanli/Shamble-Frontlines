//! The content folder's walk and the bundle tool (`devtools::content_files`), on scratch folders: what a clean tree
//! gives, and each thing the walk and the loader refuse, with the tree left as it was. The real `content/` joins
//! this file with the first content files.
#![cfg(unix)]

use std::path::{Path, PathBuf};

use devtools::content_files::{BUNDLE, bundle, run, walk};

/// A map the loader takes, in canonical form (made-up sizes: no value of the game rests on it).
fn map_text() -> String {
    let text = r#"{"bounds": {"max": [50, 20, 50], "min": [-50, 0, -50]}, "format": "shamble-map", "id": "test-yard", "regions": [], "shapes": [{"id": "step", "max": [1, 0.55, 1], "min": [0, 0, 0], "type": "box"}], "spawns": [{"at": [0, 0, -5], "facing_deg": 90, "id": "a"}], "version": 1}"#;
    content::write::canonical(&content::json::parse(text.as_bytes()).expect("a map in the grammar"))
}

/// A scratch `content/` with one file of each kind and a `source/` folder that holds what no loader would take.
fn tree(case: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("content-files-{case}"));
    let _ = std::fs::remove_dir_all(&root);
    let content = root.join("content");
    for folder in ["maps", "movement", "source"] {
        std::fs::create_dir_all(content.join(folder)).expect("a scratch folder");
    }
    std::fs::create_dir_all(root.join("out")).expect("a scratch folder");
    let write =
        |path: &str, text: &str| std::fs::write(content.join(path), text).expect("a scratch file");
    write("maps/test-yard.json", &map_text());
    write("movement/test-soldier.json", content::movement::TEST_TEXT);
    write(
        "source/export.json",
        "{\"anything\": true, \"at\": 1.0}\r\n",
    );
    root
}

/// Every entry under a folder with its type, its bytes and its time of change: what "left as it was" means.
fn snapshot(folder: &Path) -> Vec<String> {
    let mut found = Vec::new();
    let mut open = vec![folder.to_path_buf()];
    while let Some(next) = open.pop() {
        for entry in std::fs::read_dir(&next).expect("a readable folder") {
            let path = entry.expect("an entry").path();
            let meta = std::fs::symlink_metadata(&path).expect("an entry's own type");
            let bytes = if meta.is_file() {
                std::fs::read(&path).expect("a file")
            } else {
                Vec::new()
            };
            found.push(format!(
                "{} {:?} {:?} {:?}",
                path.display(),
                meta.file_type(),
                meta.modified().ok(),
                bytes
            ));
            if meta.is_dir() {
                open.push(path);
            }
        }
    }
    found.sort();
    found
}

/// Changes a clean tree, runs the tool in both modes, and gives what the check said. Both modes must agree, and
/// neither may leave a trace: the tree as the change left it, and no bundle written.
fn refused(case: &str, change: impl FnOnce(&Path)) -> String {
    let root = tree(case);
    change(&root.join("content"));
    let before = snapshot(&root);
    let check = run(&root.join("content"), None).expect_err("the check refuses");
    let write =
        run(&root.join("content"), Some(&root.join("out"))).expect_err("the writer refuses");
    assert_eq!(check, write, "{case}: the two modes say different things");
    assert_eq!(snapshot(&root), before, "{case}: the tool left a trace");
    check
}

#[test]
fn a_clean_tree_gives_its_files_in_path_order_and_one_bundle_the_loader_takes() {
    let root = tree("clean");
    let content = root.join("content");
    let files = walk(&content).expect("a clean tree");
    let paths: Vec<&str> = files.iter().map(|(path, _)| path.as_str()).collect();
    assert_eq!(paths, ["maps/test-yard.json", "movement/test-soldier.json"]);
    let text = bundle(&files).expect("the loader takes them");
    let set = content::load(text.as_bytes()).expect("the bundle loads");
    assert!(set.map("test-yard").is_some() && set.movement("test-soldier").is_some());
    // The check writes nothing; the writer puts exactly the bundle into the folder it is given.
    let before = snapshot(&root);
    let said = run(&content, None).expect("the check");
    assert!(
        said.starts_with("2 content files, a bundle of ") && said.ends_with("nothing written"),
        "{said}"
    );
    assert_eq!(snapshot(&root), before);
    run(&content, Some(&root.join("out"))).expect("the writer");
    assert_eq!(
        std::fs::read_to_string(root.join("out").join(BUNDLE)).expect("the bundle"),
        text
    );
    let left: Vec<_> = std::fs::read_dir(root.join("out"))
        .expect("the folder")
        .collect();
    assert_eq!(left.len(), 1, "the writer left more than the bundle");
    // Written again over the first: the same bytes.
    run(&content, Some(&root.join("out"))).expect("the writer, again");
    assert_eq!(
        std::fs::read_to_string(root.join("out").join(BUNDLE)).expect("the bundle"),
        text
    );
}

#[test]
fn a_file_that_is_not_in_canonical_form_is_refused_in_the_loaders_words() {
    let soldier = content::movement::TEST_TEXT;
    let rewrite = |to: String| {
        move |content: &Path| {
            std::fs::write(content.join("movement/test-soldier.json"), to).expect("a scratch file")
        }
    };
    for (case, text, must) in [
        (
            "crlf",
            soldier.replace('\n', "\r\n"),
            "movement/test-soldier.json",
        ),
        (
            "no-newline",
            soldier.trim_end().to_owned(),
            "movement/test-soldier.json",
        ),
        (
            "one-point-zero",
            soldier.replace("\"air_control_pct\": 100", "\"air_control_pct\": 100.0"),
            "movement/test-soldier.json",
        ),
        (
            "unsorted",
            {
                let (first, rest) = soldier.split_once(",\n").expect("two members");
                format!(
                    "{{\n{},\n{}\n}}\n",
                    rest.trim_end()
                        .trim_end_matches('}')
                        .trim_end()
                        .split_once(",\n")
                        .expect("three members")
                        .0,
                    first.trim_start_matches("{\n")
                )
            },
            "movement/test-soldier.json",
        ),
    ] {
        assert_ne!(text, soldier, "{case}: the plant changed nothing");
        let said = refused(case, rewrite(text));
        assert!(said.contains(must), "{case}: {said}");
    }
    // A value outside its range, and an id that is not the file's name: the kind's rules, through the same door.
    let range = refused(
        "range",
        rewrite(soldier.replace("\"air_control_pct\": 100", "\"air_control_pct\": 101")),
    );
    assert!(range.contains("air_control_pct"), "{range}");
    let id = refused("id", |content| {
        std::fs::rename(
            content.join("movement/test-soldier.json"),
            content.join("movement/other.json"),
        )
        .expect("a rename");
    });
    assert!(id.contains("movement/other.json"), "{id}");
}

#[test]
fn whatever_is_not_a_kinds_folder_with_plain_id_files_is_refused_before_it_is_read() {
    let secret = Path::new(env!("CARGO_TARGET_TMPDIR")).join("content-files-outside.json");
    std::fs::write(&secret, content::movement::TEST_TEXT).expect("a scratch file");
    let link = |to: &Path, at: PathBuf| std::os::unix::fs::symlink(to, at).expect("a link");
    type Change<'a> = Box<dyn FnOnce(&Path) + 'a>;
    let cases: Vec<(&str, Change, &str)> = vec![
        // A link in a file's place, to a file the loader would take: refused for what it is, never read.
        (
            "link-file",
            Box::new(|content| {
                std::fs::remove_file(content.join("movement/test-soldier.json"))
                    .expect("the file goes");
                link(&secret, content.join("movement/test-soldier.json"));
            }),
            "content/movement/test-soldier.json is not a plain file",
        ),
        (
            "link-folder",
            Box::new(|content| {
                std::fs::rename(content.join("movement"), content.join("../moved"))
                    .expect("a move");
                link(&content.join("../moved"), content.join("movement"));
            }),
            "content/movement is not a folder",
        ),
        (
            "link-source",
            Box::new(|content| {
                std::fs::remove_dir_all(content.join("source")).expect("the folder goes");
                link(&content.join("maps"), content.join("source"));
            }),
            "content/source is not a folder",
        ),
        (
            "pipe",
            Box::new(|content| {
                let made = std::process::Command::new("mkfifo")
                    .arg(content.join("maps/pipe.json"))
                    .status()
                    .expect("mkfifo runs");
                assert!(made.success(), "mkfifo made no pipe");
            }),
            "content/maps/pipe.json is not a plain file",
        ),
        (
            "nested-source",
            Box::new(|content| {
                std::fs::create_dir(content.join("maps/source")).expect("a folder");
            }),
            "content/maps/source is not a plain file",
        ),
        (
            "unknown-folder",
            Box::new(|content| {
                std::fs::create_dir(content.join("weapons")).expect("a folder");
            }),
            "content/weapons is no kind's folder",
        ),
        (
            "top-file",
            Box::new(|content| {
                std::fs::write(content.join("readme.json"), "{}\n").expect("a file");
            }),
            "content/readme.json is not a folder",
        ),
        (
            "capital",
            Box::new(|content| {
                std::fs::write(content.join("movement/Soldier.json"), "x").expect("a file");
            }),
            "content/movement/Soldier.json: a content file is named <id>.json",
        ),
        (
            "ending",
            Box::new(|content| {
                std::fs::write(content.join("movement/notes.txt"), "x").expect("a file");
            }),
            "content/movement/notes.txt: a content file is named <id>.json",
        ),
        (
            "device-name",
            Box::new(|content| {
                std::fs::write(content.join("maps/con.json"), "x").expect("a file");
            }),
            "content/maps/con.json: a content file is named <id>.json",
        ),
        (
            "too-large",
            Box::new(|content| {
                std::fs::write(content.join("maps/huge.json"), vec![b' '; 1024 * 1024 + 2])
                    .expect("a file");
            }),
            "content/maps: huge.json: more than the 1048576 bytes a read takes",
        ),
        (
            "not-text",
            Box::new(|content| {
                std::fs::write(content.join("maps/bytes.json"), [0xff, 0xfe]).expect("a file");
            }),
            "content/maps/bytes.json is not text",
        ),
    ];
    for (case, change, must) in cases {
        let said = refused(case, change);
        assert!(said.starts_with(must), "{case}: {said}");
    }
    assert_eq!(
        std::fs::read_to_string(&secret).expect("the outside file"),
        content::movement::TEST_TEXT
    );
    // The folder itself: missing, or a link to a real one.
    let root = tree("no-folder");
    let missing = run(&root.join("nothing"), None).expect_err("no folder");
    assert!(
        missing.ends_with("is not there as a plain folder"),
        "{missing}"
    );
    link(&root.join("content"), root.join("linked"));
    let linked = run(&root.join("linked"), None).expect_err("a link to the folder");
    assert!(
        linked.ends_with("is not there as a plain folder"),
        "{linked}"
    );
}

#[test]
fn the_bundle_is_written_outside_the_content_folder_and_never_through_a_link() {
    let root = tree("out");
    let content = root.join("content");
    let before = snapshot(&root);
    // Inside the content folder, by any spelling; a folder that is not there; a link to a folder.
    for inside in [
        content.clone(),
        content.join("maps"),
        content.join("source"),
        root.join("out/../content/maps"),
    ] {
        let said = run(&content, Some(&inside)).expect_err("an output inside content");
        assert!(
            said.ends_with(
                "lies inside the content folder: the bundle is build output, never content"
            ),
            "{said}"
        );
    }
    let nowhere = run(&content, Some(&root.join("nowhere"))).expect_err("no such folder");
    assert!(nowhere.ends_with("is not a plain folder"), "{nowhere}");
    assert_eq!(snapshot(&root), before);
    std::os::unix::fs::symlink(root.join("out"), root.join("linked-out")).expect("a link");
    let linked = run(&content, Some(&root.join("linked-out"))).expect_err("a link to a folder");
    assert!(linked.ends_with("is not a plain folder"), "{linked}");
    // A link where the bundle goes, and a leftover of an earlier write: refused, and what the link leads to is whole.
    let outside = root.join("outside.txt");
    std::fs::write(&outside, "not to be touched").expect("a scratch file");
    std::os::unix::fs::symlink(&outside, root.join("out").join(BUNDLE)).expect("a link");
    let through = run(&content, Some(&root.join("out"))).expect_err("a link in the bundle's place");
    assert!(
        through.ends_with("is a link or not a plain file"),
        "{through}"
    );
    std::fs::remove_file(root.join("out").join(BUNDLE)).expect("the link goes");
    std::os::unix::fs::symlink(&outside, root.join("out").join(".content.json.new"))
        .expect("a link");
    let leftover = run(&content, Some(&root.join("out"))).expect_err("a leftover");
    assert!(leftover.starts_with(".content.json.new: "), "{leftover}");
    assert_eq!(
        std::fs::read_to_string(&outside).expect("the outside file"),
        "not to be touched"
    );
}
