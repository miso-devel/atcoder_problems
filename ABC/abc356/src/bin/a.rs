use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        l: i32,
        r: i32,
    }
    let mut result: Vec<i32> = (1..=n).collect();
    let replace_range: std::ops::RangeInclusive<usize> = (l - 1) as usize..=(r - 1) as usize;
    let reverce_ary: Vec<i32> = (l..=r).rev().collect();
    result.splice(replace_range, reverce_ary);
    println!(
        "{}",
        result
            .iter()
            .map(|&num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
