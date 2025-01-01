use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let ans = if a > b { a } else { b };
    println!("{}", ans);
}
