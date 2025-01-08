use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        _: i32,
        mut s: [String; n],
    }
    s.sort();
    println!("{}", s.join(""));
}
