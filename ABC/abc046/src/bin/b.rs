use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let mut ans = k;

    for _ in 1..n {
        ans *= k - 1;
    }

    println!("{}", ans);
}
