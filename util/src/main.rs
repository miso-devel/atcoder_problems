mod insertion_sort;
mod rotate;

fn main() {
    println!("Hello, world!");
    let mut a = [5, 2, 3];
    println!("{:?}", insertion_sort::insertion_sort(&mut a));
}
