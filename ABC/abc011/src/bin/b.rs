use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s:Chars
    }

    println!(
        "{}",
        s.iter()
            .enumerate()
            .map(|(i, c)| if i == 0 {
                c.to_ascii_uppercase()
            } else {
                c.to_ascii_lowercase()
            })
            .collect::<String>()
    );
}
