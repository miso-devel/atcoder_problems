use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        a:i32,
        t:[i32;n]
    }
    let mut pre: i32 = 0;
    for x in t {
        let res = x.max(pre) + a;
        pre = res;
        println!("{}", res)
    }
}
