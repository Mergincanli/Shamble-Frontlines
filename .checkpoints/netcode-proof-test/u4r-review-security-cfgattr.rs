// security-auditor's U4r probe: does rustc 1.99.1 take a #[path] whose whole meta comes from a macro's `meta` fragment
// inside cfg_attr (a form check.mjs's carry 85 (c) rule, `#[$` / `#![$`, does not match)? The module's text is a .txt
// file, which crateFiles never lists. Built with rustc into .gate/u4r-review-security/ only.
#![deny(unsafe_code)]

macro_rules! m {
    ($a:meta) => {
        #[cfg_attr(all(), $a)]
        mod g;
    };
}

m!(path = "u4r-review-security-cfgattr-mod.txt");

fn main() {
    println!("module from the .txt file says {}", g::seven());
}
