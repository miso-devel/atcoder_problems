use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        a:String,
        b:String,
        c:String
    }
    let mut hash: HashMap<&str, Vec<char>> = HashMap::new();
    let mut current: &str = "a";
    hash.insert("a", a.chars().collect::<Vec<char>>());
    hash.insert("b", b.chars().collect::<Vec<char>>());
    hash.insert("c", c.chars().collect::<Vec<char>>());
    loop {
        let current_hash = hash[current].clone();
        let count = current_hash.len();
        if count == 0 {
            break;
        }

        let mut next = current_hash[0];
        if next == 'a' {
            current = "a";
        }
    }

    println!("{:?}", hash);
}
