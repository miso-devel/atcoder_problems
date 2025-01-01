use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut sum: u64 = 0;
    for i in 0..=n {
        sum += i * 10000;
    }
    println!("{}", sum / n);
}
