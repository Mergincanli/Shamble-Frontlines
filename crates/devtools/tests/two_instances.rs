//! Nothing of a match lives outside its state (PM-0004-17): two matches in one process, stepped in turn, each give
//! exactly the hashes they give alone. The two differ in their seed and in their input stream. Until a system draws
//! a random number (change-set 8) the seed moves only the whole-state column; the input streams make the shared
//! columns differ too.

use devtools::compare::{Column, first_difference};
use devtools::recording::{Recording, parse_hashes};
use devtools::replay::{Replay, replay};
use devtools::scripts::{first_loop, looped};

#[test]
fn two_matches_stepped_in_turn_each_give_their_own_hashes() {
    let (first, second) = (first_loop(), looped("second-loop", 1_500, 5));
    let alone = [&first, &second].map(|recording| replay(recording).expect("it replays"));
    // The two are different matches: their hashes differ in both columns from the first step with a command.
    for column in [Column::Shared, Column::Whole] {
        assert!(first_difference(&alone[0], &alone[1], column).is_some());
    }
    let mut a = Replay::new(&first).expect("it replays");
    let mut b = Replay::new(&second).expect("it replays");
    let mut together = [Vec::new(), Vec::new()];
    // In turn, and unevenly: one step of the first, then up to two of the second.
    for round in 0..1_800u32 {
        together[0].extend(a.next());
        for _ in 0..=round % 2 {
            together[1].extend(b.next());
        }
    }
    assert_eq!((together[0].len(), together[1].len()), (1_800, 1_500));
    for (alone, together) in alone.iter().zip(&together) {
        for column in [Column::Shared, Column::Whole] {
            assert_eq!(first_difference(alone, together, column), None);
        }
    }
    // And the first of them is the committed recording: its hashes are the file's.
    let name = |suffix: &str| {
        format!(
            "{}/recordings/first-loop.{suffix}",
            env!("CARGO_MANIFEST_DIR")
        )
    };
    let bytes = std::fs::read(name("rec")).expect("the committed recording");
    assert_eq!(Recording::decode(&bytes).as_ref(), Ok(&first));
    let hashes = std::fs::read_to_string(name("hashes")).expect("its hashes file");
    let file = parse_hashes(&bytes, &hashes).expect("its hashes file reads");
    assert_eq!(file, together[0]);
}
