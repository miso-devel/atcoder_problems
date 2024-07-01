use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        a: [i32; n*2],
    }

    let separeted: Vec<&[i32]> = a.windows(3).collect();
    let mut result: i32 = 0;

    separeted.iter().for_each(|x| {
        if x[0] == x[2] && x[0] != x[1] {
            result += 1;
        }
    });

    println!("{}", result);
}
