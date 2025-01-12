use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        w:i32,
    }

    let mut hash = HashMap::new();
    for i in 1..=w {
        if hash.contains_key(&i) {
            hash.insert(i, hash[&i] + 1);
        } else {
            hash.insert(i, 1);
        }
    }

    let ans = hash.values().all(|&x| x % 2 == 0);

    println!("{}", if ans { "Yes" } else { "No" });
}
