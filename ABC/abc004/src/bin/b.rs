use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        c: [[String; 4]; 4]
    }

    fn rotate(input: Vec<Vec<String>>, rotate_count: usize) -> Vec<Vec<String>> {
        let height = input.len();
        if height == 0 {
            return Vec::new();
        }

        let max_width = input
            .iter()
            .map(|row: &Vec<String>| row.len())
            .max()
            .unwrap_or(0);

        let mut result: Vec<Vec<String>> = input.clone();

        for _ in 0..rotate_count {
            let mut now: Vec<Vec<String>> = Vec::with_capacity(max_width);
            for x in 0..max_width {
                let mut new_row: Vec<String> = Vec::with_capacity(height);
                for y in (0..height).rev() {
                    if let Some(row) = result.get(y) {
                        if let Some(cell) = row.get(x) {
                            new_row.push(cell.clone());
                        } else {
                            new_row.push(" ".to_string());
                        }
                    } else {
                        new_row.push(" ".to_string());
                    }
                }
                now.push(new_row);
            }
            result = now;
        }

        return result;
    }

    for i in 0..4 {
        println!("{}", rotate(c.clone(), 2)[i].join(" "));
    }
}
