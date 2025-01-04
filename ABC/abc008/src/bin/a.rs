use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s:i32,
        t:i32,
    }
    println!("{}", t - s + 1);
}
