use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        w: String,
    }
    println!(
        "{}",
        w.chars()
            .filter(|c| *c != 'a' && *c != 'i' && *c != 'u' && *c != 'e' && *c != 'o')
            .join("")
    );
}
