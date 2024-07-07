use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        m: i32,
        a: [i32;n],
        b: [i32;m]
    }

    fn windowed_array(ary: &[i32], window_size: usize) -> Vec<Vec<i32>> {
        ary.windows(window_size)
            .map(|window| window.to_vec())
            .collect()
    }

    let extend_ary: Vec<i32> = [a.clone(), b.clone()]
        .concat()
        .iter()
        .sorted()
        .cloned()
        .collect();

    let window_ary2 = windowed_array(&extend_ary, 2);

    let mut result = "No";
    for window in window_ary2 {
        let find1 = a.contains(&window[0]);
        let find2 = a.contains(&window[1]);
        if find1 && find2 {
            result = "Yes";
        }
    }
    println!("{}", result)
}
