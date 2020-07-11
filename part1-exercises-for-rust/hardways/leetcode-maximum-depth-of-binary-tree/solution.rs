use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn travel(root: &Option<Rc<RefCell<TreeNode>>>) -> i32{
            let mut d = 0;
            if let Some(ref n) = root {
                d = travel(&n.borrow().left);
                d = d.max( travel(&n.borrow().right));
                d = d+1;
            }else{
                d = 0;
            }
            d
        }
        travel(&root)
    }
}