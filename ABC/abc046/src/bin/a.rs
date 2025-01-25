use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        abc: [i32; 3]
    }
    let mut abc = abc.to_vec();
    abc.sort();
    abc.dedup();
    println!("{}", abc.len());
}
