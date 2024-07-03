use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut lower = 0;
    let mut upper = 0;

    for char in s.chars() {
        if char.is_lowercase() {
            lower += 1;
        } else {
            upper += 1;
        }
    }

    if lower > upper {
        println!("{}", s.to_lowercase())
    } else {
        println!("{}", s.to_uppercase())
    }
}
