use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let mut ans = vec![];
    for i in s {
        if i == 'B' {
            ans.pop();
        } else {
            ans.push(i);
        }
    }
    println!("{}", ans.iter().collect::<String>());
}
