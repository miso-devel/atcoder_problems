use proconio::{fastout, input};

// #[fastout]
fn main() {
    input! {m: i32}
    let km: f64 = m as f64 / 1000.0;
    match km {
        0.1..=5.0 => {
            if (km * 10.0).to_string().len() == 1 {
                println!("0{}", km * 10.0)
            } else {
                println!("{}", km * 10.0)
            }
        }
        6.0..=30.0 => println!("{}", km + 50.0),
        35.0..=70.0 => println!("{}", ((km - 30.0) / 5.0) + 80.0),
        km if km > 70.0 => println!("89"),
        _ => println!("00"),
    }
}
