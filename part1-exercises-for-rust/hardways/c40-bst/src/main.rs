mod lib;
use lib::Bst;

fn main() {
    let mut t = Bst::new();
    t.insert(5,String::from("Hello5"));
    t.insert(3,String::from("Hello3"));
    t.insert(6,String::from("Hello6"));
    t.bst_get(4);
}
