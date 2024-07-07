use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        m: i32,
        a: [i32; m],
        xs: [[i32; m];n],
    }

    let columns = xs[0].len();

    let sums: Vec<i32> = (0..columns)
        .map(|i| xs.iter().fold(0, |sum, row| sum + row[i]))
        .collect();

    let mut result = "Yes";

    for index in 0..(a.len()) {
        if a[index] > sums[index] {
            result = "No";
            break;
        }
    }
    println!("{}", result)
}
