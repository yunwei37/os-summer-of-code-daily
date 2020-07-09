mod lib;
use lib::merge_sort;

fn main() {
    let mut a = vec![5,2];
    let mut b = vec![5,2];
    merge_sort(&mut a);
    b.sort();
    println!("{:?}",a);
}
