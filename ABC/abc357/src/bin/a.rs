use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        m: i32,
        h: [i32; n],
    }
    let mut result = 0;
    let mut now = 0;

    for item in h {
        if (now + item) <= m {
            result += 1;
            now += item;
        } else {
            break;
        }
    }

    println!("{}", result);
}
