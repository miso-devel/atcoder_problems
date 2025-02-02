use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:String,
        b:String,
        c:String
    }
    let strings = [a, b, c];
    let mut now = [0, 0, 0];
    let mut current = 0;

    while now[current] < strings[current].len() {
        let current_char = strings[current].chars().nth(now[current]).unwrap();
        now[current] += 1;
        current = match current_char {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => 0,
        };
    }
    let result = match current {
        0 => "A",
        1 => "B",
        2 => "C",
        _ => unreachable!(),
    };
    println!("{}", result);
}
