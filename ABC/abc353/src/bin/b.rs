use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        k:i32,
        a:[i32;n]
    }

    let mut attraction_count = 0;
    let mut current = 0;

    for (index, item) in a.iter().enumerate() {
        if a.len() == index + 1 {
            attraction_count += 1;
        } else if current + item >= k {
            current = 0;
            current += item;
            attraction_count += 1;
        } else if current + item + a[index + 1] > k {
            attraction_count += 1;
            current = 0;
        } else {
            current += item;
        }
    }
    println!("{}", attraction_count);
}
