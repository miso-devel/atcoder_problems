use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      n: i32,
      h:  [String;n]
    }

    let mut count = 0;
    h.iter().for_each(|x| {
        if x == "Takahashi" {
            count += 1;
        }
    });

    println!("{}", count)
}
