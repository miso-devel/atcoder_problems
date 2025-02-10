use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32
    }
    let sum = a + b + c;
    if [a, b, c].contains(&(sum / 2)) && sum % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
