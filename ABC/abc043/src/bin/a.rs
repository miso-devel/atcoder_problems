use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n:i32}
    let mut ans = 0;
    for i in 1..=n {
        ans += i;
    }
    println!("{}", ans);
}
