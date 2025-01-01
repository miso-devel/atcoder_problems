use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
        y: u64,
    }
    println!("{:?}", y / x);
}
