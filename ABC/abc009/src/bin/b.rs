use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        mut a:[i32;n],
    }
    a.sort();
    a.dedup();
    a.reverse();
    println!("{}", a[1]);
}
