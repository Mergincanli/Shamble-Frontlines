//! Figures: the computations a mark needs that README §2.3's statistics cannot express (ratios across scale steps,
//! slopes, arithmetic against a formula). Every name `marks.json` uses must be listed here; one that is listed but
//! not built yet says which unit builds it, and its check comes out NOT-RUN rather than guessed.

use crate::eval::{Missing, Run};
use crate::ratio::Ratio;

/// A figure's computation: the run under judgement and every run of its set. An `Err` is a gap in the record.
pub type Compute = fn(run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, String>;

/// One listed figure.
pub struct Figure {
    pub name: &'static str,
    /// The computation, or `None` while it is not built.
    pub compute: Option<Compute>,
    /// The unit that builds it, for a figure not built yet.
    pub unit: &'static str,
    /// What it computes, in plain words.
    pub what: &'static str,
}

/// The list. Names join it with `marks.json` and are built with the code that first produces their inputs.
pub const FIGURES: &[Figure] = &[];

pub fn known(name: &str) -> bool {
    FIGURES.iter().any(|f| f.name == name)
}

/// Computes a figure: a gap if its inputs are missing, not-built if its code is not there yet.
pub(crate) fn compute(name: &str, run: &Run<'_>, set: &[&Run<'_>]) -> Result<Ratio, Missing> {
    match FIGURES.iter().find(|f| f.name == name) {
        Some(Figure {
            compute: Some(f), ..
        }) => f(run, set).map_err(Missing::Gap),
        Some(Figure { unit, .. }) => Err(Missing::NotBuilt(format!(
            "figure {name} is not built yet ({unit})"
        ))),
        None => Err(Missing::NotBuilt(format!(
            "figure {name} is not in the judge's list"
        ))),
    }
}
