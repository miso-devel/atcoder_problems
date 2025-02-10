use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        c:String
    }
    let vowels = ['a', 'i', 'u', 'e', 'o'];
    if vowels.contains(&c.chars().next().unwrap()) {
        println!("vowel");
    } else {
        println!("consonant");
    }
}
