use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        h:[i32;n]
    }

    let highest = h[0];
    let mut result: i32 = -1;
    for (count, item) in h.into_iter().enumerate() {
        if highest < item {
            result = count as i32;
            break;
        }
    }

    println!("{}", if result == -1 { result } else { result + 1 });
}
