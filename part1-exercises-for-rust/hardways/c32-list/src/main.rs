mod lib;

use lib::List;

fn main() {
    let mut a = List::new();
    a.list_push(1);
    a.list_push(2);
    a.list_unshift(3);
    a.list_unshift(1);
    a.list_unshift(2);
    a.list_clear();
}
