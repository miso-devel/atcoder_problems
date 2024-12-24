use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let mut result: Vec<bool> = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        let s_char = s.chars().nth(i).unwrap();
        let t_char = t.chars().nth(i).unwrap();
        if s_char == t_char {
            result.push(true);
        } else {
            let ok_list: [char; 7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
            if s_char == '@' && ok_list.contains(&t_char)
                || t_char == '@' && ok_list.contains(&s_char)
            {
                result.push(true);
            } else {
                result.push(false);
            }
        }
    }
    println!(
        "{}",
        if result.iter().all(|&x| x) {
            "You can win"
        } else {
            "You will lose"
        }
    );
}
