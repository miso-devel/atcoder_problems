use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        a:[i32;n],
    }
    let mut result = 0;
    for i in 0..n {
        let mut now = a[i as usize];
        while now % 2 == 0 || now % 5 == 0 {
            now -= 1;
            result += 1;
        }
    }
    println!("{}", result);
}
