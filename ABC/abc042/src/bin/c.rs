use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        d: [char; k],
    }

    fn is_valid_num(num: usize, ary: &[char]) -> bool {
        return num.to_string().chars().all(|c| !ary.contains(&c));
    }

    for i in n..=100000 {
        if is_valid_num(i, &d) && i >= n {
            println!("{}", i);
            break;
        }
    }
}
