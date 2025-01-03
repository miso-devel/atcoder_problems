use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut nums: [usize; 3] = [0, 0, 1];

    if n <= 3 {
        println!("{}", nums[n - 1]);
        return;
    }
    let mut last_num: usize = 1;
    for _ in 4..=n {
        last_num = (nums[0] + nums[1] + nums[2]) % 10007;
        nums[0] = nums[1];
        nums[1] = nums[2];
        nums[2] = last_num;
    }
    println!("{}", last_num);
}
