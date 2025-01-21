use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        h:i32,
    }

    println!("{}", (a + b) * h / 2);
}
