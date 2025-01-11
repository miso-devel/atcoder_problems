use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i32; n]
    }
    let sum = a.iter().sum::<i32>();
    let avg = sum / n;
    let rem = sum % n;
    let new_avg = if rem == 0 {
        avg
    } else if n / 2 < rem {
        avg + 1
    } else {
        avg
    };
    let mut ans = 0;
    for i in a {
        ans += (i - new_avg).pow(2);
    }
    println!("{}", ans);
}
