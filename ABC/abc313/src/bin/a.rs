use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        p:[usize;n]
    }

    if n == 1 {
        println!("0");
        return;
    }

    let strength = p[0];
    let max = *p[1..].iter().max().unwrap();

    if strength > max {
        println!("0");
    } else {
        let result = max - strength + 1;
        println!("{}", result);
    }
}
