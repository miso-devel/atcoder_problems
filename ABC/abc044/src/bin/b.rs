use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        w:String,
    }

    let mut hash = HashMap::new();
    for i in w.chars() {
        if hash.contains_key(&i) {
            hash.insert(i, hash.get(&i).unwrap() + 1);
        } else {
            hash.insert(i, 1);
        }
    }

    let ans = hash.values().all(|&x| x % 2 == 0);

    println!("{}", if ans { "Yes" } else { "No" });
}
