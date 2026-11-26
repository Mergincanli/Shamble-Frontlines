//! The content reader's proofs, natively (content items 1 and 2): the number corpus and the tree corpus against a
//! browser engine's own answers, and every class of broken file against the verdict its class says. The same checks
//! run in WebAssembly through the test-only module (`packages/sim-worker/boundary/content.test.mjs` under Node,
//! `packages/e2e/tests/rules/content.spec.ts` in the browsers).

use devtools::content_proofs::{CLASSES, corpus_check, trees_check, verdicts, verdicts_check};

/// The committed verdicts' seed and count per class (the classes that make megabytes run a tenth of it).
const SEED: u64 = 20_261_126;
const PER_CLASS: u32 = 500;

fn path_of(name: &str) -> String {
    format!("{}/vectors/{name}", env!("CARGO_MANIFEST_DIR"))
}

fn vector(name: &str) -> String {
    // Read when the test runs, never built into the program.
    std::fs::read_to_string(path_of(name)).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn the_reader_and_the_writer_agree_with_a_browser_engine_on_every_number_of_the_corpus() {
    let corpus = vector("content-corpus.txt");
    assert_eq!(corpus.lines().count(), 1 + 2_038 + 10_000 + 1_000 + 29);
    assert_eq!(corpus_check(&corpus), "");
    // The corpus starts with the old tuning file's numbers, each as that file writes it.
    let legacy = vector("legacy-numbers.txt");
    let texts = corpus.lines().skip(1).map(|line| line.split(' ').nth(1));
    let same = legacy
        .lines()
        .skip(1)
        .zip(texts)
        .filter(|(old, text)| Some(*old) == *text);
    assert_eq!(same.count(), 2_038);
}

#[test]
fn the_corpus_check_names_a_reader_or_a_writer_that_differs_from_the_engine() {
    // 0.1 with one bit more, 0.1 written with 17 digits, a text the loader must refuse given as canonical, a line
    // short of a field, and a first line that miscounts: each is found, by its own name.
    let wrong = |line: &str| corpus_check(&format!("shamble-content-corpus 1 1\n{line}\n"));
    assert_eq!(wrong("3fb999999999999a 0.1 0.1"), "");
    assert!(wrong("3fb999999999999b 0.1 0.1").contains("the reader gives 3fb999999999999a"));
    assert!(wrong("3fb999999999999a 0.1 0.10000000000000001").contains("the writer writes 0.1"));
    assert!(wrong("3ff0000000000000 1.0 1.0").contains("the writer writes 1"));
    assert!(wrong("3fb999999999999a 0.1").contains("not three fields"));
    assert!(wrong("7ff0000000000000 1e400 1e400").contains("1e400 is refused: NotFinite"));
    let miscounted = corpus_check("shamble-content-corpus 1 2\n3fb999999999999a 0.1 0.1\n");
    assert!(miscounted.contains("and the file holds 1"), "{miscounted}");
}

#[test]
fn the_canonical_writer_writes_what_a_browser_engine_writes_for_every_tree_of_the_corpus() {
    let trees = vector("content-trees.txt");
    assert_eq!(trees.lines().next(), Some("shamble-content-trees 1 400"));
    assert_eq!(trees_check(&trees), "");
    // An empty array written on two lines, as a writer with a wrong layout would: found.
    let wrong = trees_check("shamble-content-trees 1 1\n{\"a\":[]}\n{\n  \"a\": [\n  ]\n}\n");
    assert!(
        wrong.contains("the writer writes \"{\\n  \\\"a\\\": []\\n}\\n\""),
        "{wrong}"
    );
}

#[test]
fn every_class_of_broken_file_gets_the_refusal_its_class_says_and_the_committed_digests() {
    // SHAMBLE_WRITE_VERDICTS=1 writes the file again (after a change that is meant to change a verdict or a
    // message); without it the file is only read.
    if std::env::var_os("SHAMBLE_WRITE_VERDICTS").is_some() {
        let made = verdicts(SEED, PER_CLASS).unwrap_or_else(|why| panic!("{why}"));
        std::fs::write(path_of("content-verdicts.txt"), made).expect("the verdicts file");
    }
    let committed = vector("content-verdicts.txt");
    let first = format!("shamble-content-verdicts 1 {SEED} {PER_CLASS}");
    assert_eq!(committed.lines().next(), Some(first.as_str()));
    assert_eq!(committed.lines().count(), 2 + CLASSES.len());
    // Every case of every class is judged as its class says, and each class's digest is the committed one.
    assert_eq!(verdicts_check(&committed), "");
    // The compare bites: one digit of one digest changed is found, with its class's line.
    assert!(verdicts_check("shamble-content-verdicts 2 1 1\n").starts_with("not a verdicts file"));
    let small = verdicts(SEED, 10).unwrap_or_else(|why| panic!("{why}"));
    let found = verdicts_check(&small.replacen("a comment|10|", "a comment|10|0", 1));
    assert!(
        found.starts_with("this build gives\n  a comment|10|"),
        "{found}"
    );
    let renamed = verdicts_check(&small.replacen("a comment|", "a remark|", 1));
    assert!(
        renamed.contains("and the file has\n  a remark|10|"),
        "{renamed}"
    );
    assert!(verdicts_check(&format!("{small}more\n")).starts_with("the file has other lines"));
}

#[test]
fn the_classes_hold_on_seeds_that_were_never_committed() {
    for seed in [1, 2, 3] {
        let made = verdicts(seed, 40).unwrap_or_else(|why| panic!("{why}"));
        assert_eq!(made.lines().count(), 2 + CLASSES.len());
    }
}
