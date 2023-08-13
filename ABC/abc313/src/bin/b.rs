use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        xy:[(usize,usize);m],
    }

    let mut is_lose = vec![false; n];
    for now in xy {
        is_lose[now.1 - 1] = true;
    }

    let loser_count = is_lose.iter().filter(|&&x| !x).count();
    if loser_count != 1 {
        println!("-1");
        return;
    }

    let loser = is_lose
        .iter()
        .enumerate()
        .filter(|&x: &(usize, &bool)| !x.1)
        .collect::<Vec<_>>()[0]
        .0;

    println!("{:?}", loser + 1);
}
