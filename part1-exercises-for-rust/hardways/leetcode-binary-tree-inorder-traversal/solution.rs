use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder(root: &Option<Rc<RefCell<TreeNode>>>,mut array: Vec<i32>) -> Vec<i32> {
        if let Some(ref n) = root {
            array = Solution::inorder(&n.borrow().left,array);
            if let Some(ref v) = root{
                array.push(v.borrow().val);
            }
            array = Solution::inorder(&n.borrow().right,array);
        }
        return array;
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut a = Vec::<i32>::new();
        Solution::inorder(&root,a)
    }
}