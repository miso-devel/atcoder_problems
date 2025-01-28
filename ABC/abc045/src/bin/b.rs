use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        a:String,
        b:String,
        c:String
    }
    let mut hash: HashMap<String, Vec<char>> = HashMap::new();

    let mut current: String = "a".to_string();
    hash.insert("a".to_string(), a.chars().collect::<Vec<char>>());
    hash.insert("b".to_string(), b.chars().collect::<Vec<char>>());
    hash.insert("c".to_string(), c.chars().collect::<Vec<char>>());

    for _ in 0..=100 {
        if hash[&current].is_empty() {
            break;
        }
        let mut current_hand: Vec<char> = hash[&current].clone();
        let popped: char = current_hand.pop().unwrap();
        hash.insert(current, current_hand);
        let next_str: String = popped.to_string();
        current = next_str;
    }

    if current == "a" {
        println!("A");
    } else if current == "b" {
        println!("B");
    } else if current == "c" {
        println!("C");
    }
}
