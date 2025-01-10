use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32
    }
    println!("{}", if n == 12 { 1 } else { n + 1 });
}
