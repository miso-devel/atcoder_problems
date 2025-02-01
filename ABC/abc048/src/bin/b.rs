use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:usize,
        b:usize,
        x:usize,
    }
    if a == 0 {
        println!("{}", b / x + 1);
    } else {
        println!("{}", b / x - (a - 1) / x);
    }
}
