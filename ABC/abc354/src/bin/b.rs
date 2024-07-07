use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        mut sc: [(String, i32); n]
    }

    sc.sort_by(|a, b| a.0.cmp(&b.0));

    let sum: i32 = sc.iter().map(|x| x.1).sum::<i32>();
    let sc_with_index = sc.iter().enumerate();

    sc_with_index.sorted_by(|a, b| a.1.cmp(b.1));

    let mod_num = sum % n;

    println!("{}", sc[mod_num as usize].0);
}
