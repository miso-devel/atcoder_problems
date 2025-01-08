use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [i32; 3],
    }

    abc.sort();
    if abc[0] == 5 && abc[1] == 5 && abc[2] == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
