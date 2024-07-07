use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:i32
    }
    let mut height: i32 = 0;
    let mut day: u32 = 0;
    while h >= height {
        height += 2_i32.pow(day);
        day += 1;
    }
    println!("{}", day);
}
