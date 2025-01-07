use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
    }
    println!("{}", if n % 2 == 0 { n / 2 } else { n / 2 + 1 });
}
