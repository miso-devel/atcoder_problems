use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        k:i32,
        x:i32,
        y:i32,
    }

    let ans = if (n - k) > 0 {
        (x * k) + (y * (n - k))
    } else {
        x * n
    };
    println!("{}", ans);
}
