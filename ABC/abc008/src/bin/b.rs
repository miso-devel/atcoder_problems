use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n:i32,
        a:[String;n],
    }
    let mut map: HashMap<String, i32> = HashMap::new();
    for i in 0..n {
        if map.contains_key(&a[i as usize]) {
            let count: i32 = *map.get(&a[i as usize]).unwrap();
            map.insert(a[i as usize].clone(), count + 1);
        } else {
            map.insert(a[i as usize].clone(), 1);
        }
    }
    let max: (&String, &i32) = map.iter().max_by_key(|&(_, v)| v).unwrap();
    println!("{}", max.0);
}
