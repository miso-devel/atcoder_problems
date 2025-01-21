use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32
    }
    let hours = format!("{:02}", n / 3600);
    let minutes = format!("{:02}", (n % 3600) / 60);
    let seconds = format!("{:02}", n % 60);
    println!("{}:{}:{}", hours, minutes, seconds);
}
