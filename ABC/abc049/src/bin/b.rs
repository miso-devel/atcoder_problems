use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:i32,
        _:i32,
        c:[String;h]
    }
    let mut result = String::new();
    for i in 0..h {
        result.push_str(&c[i as usize]);
        result.push('\n');
        result.push_str(&c[i as usize]);
        if i != h - 1 {
            result.push('\n');
        }
    }
    println!("{}", result);
}
