use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _:String,
        s:String,
    }
    println!("A{}C", s.chars().next().unwrap());
}
