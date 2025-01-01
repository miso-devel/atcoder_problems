use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32
    }
    println!("{}", if (n % 3 == 0 || n == 3) { "YES" } else { "NO" });
}
