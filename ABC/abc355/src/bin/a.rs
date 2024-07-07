use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32
    }
    if a == b {
        println!("{}", -1);
    } else {
        println!("{}", 6 - a - b);
    }
}
