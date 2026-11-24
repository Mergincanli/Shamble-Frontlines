//! The random stream replays exactly (PM-0004-27), natively: the simulation's generator against the vector file.
//! The same check runs in WebAssembly through the test-only module.

use devtools::rules::stream::{CARRY_WORDS, FILE_BYTES, STREAMS, WORDS, check};

fn file() -> Vec<u8> {
    // Read when the test runs, never built into the program.
    std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/vectors/chacha8-words.bin"
    ))
    .expect("the vector file")
}

#[test]
fn the_generator_gives_the_file_s_words() {
    assert_eq!(FILE_BYTES, 160_384);
    assert_eq!(check(&file()), Ok(()));
}

#[test]
fn one_changed_bit_anywhere_in_the_file_is_found_at_its_place() {
    let good = file();
    // The first and the last word of each long section, and of the carry window.
    let mut places = vec![(
        4 * STREAMS.len() * WORDS,
        "across the carry at block 2^32, word 0".to_owned(),
    )];
    places.push((
        FILE_BYTES - 1,
        format!("across the carry at block 2^32, word {}", CARRY_WORDS - 1),
    ));
    for (section, id) in STREAMS.iter().enumerate() {
        places.push((4 * section * WORDS, format!("stream {id}, word 0:")));
        places.push((
            4 * (section + 1) * WORDS - 1,
            format!("stream {id}, word {}:", WORDS - 1),
        ));
    }
    for (byte, must) in places {
        let mut bad = good.clone();
        bad[byte] ^= 0x10;
        let found = check(&bad).expect_err("a changed file must not pass");
        assert!(found.contains(&must), "byte {byte}: {found}");
    }
    assert!(
        check(&good[..FILE_BYTES - 4])
            .expect_err("a short file")
            .contains("160380 bytes")
    );
}
