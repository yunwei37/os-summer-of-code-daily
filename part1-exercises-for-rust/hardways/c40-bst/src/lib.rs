use std::rc::Rc;
use std::mem::swap;
use std::cell::RefCell;

struct TreeNode {
    key:i32,
    value:String,
    vaild:bool,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>
}

pub struct Bst {
    count:i32,
    root: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    fn new(key:i32,value:String) -> Rc<RefCell<TreeNode>>{
        Rc::new(RefCell::new(TreeNode {
            key,
            value,
            vaild:true,
            left: None,
            right: None,
        }))
    }
}



fn bst_search(node:&Rc<RefCell<TreeNode>>,key:i32) -> Option<String>{
    let mut result:Option<String> = None;
    //println!("search {} {}",key,node.borrow().key);
    if key == node.borrow().key {
        if node.borrow().vaild {
            return Some(node.borrow().value.clone());
        }else {
            return None;
        }
    }
    else if key < node.borrow().key{
        if let Some(ref n) = node.borrow().left {
            result = bst_search(n,key);
        }
    }
    else {
        if let Some(ref n) = node.borrow().right {
            result = bst_search(n,key);
        }
    }
    //println!("res {:?}",result);
    result
}

fn do_delete(node:&Rc<RefCell<TreeNode>>,key:i32){
    let mut node1 = node.borrow_mut();
    if key == node1.key {
        node1.vaild = false;
    }
    else if key < node1.key{
        if let Some(ref n) = node1.left {
            do_delete(n,key);
        }
    }
    else {
        if let Some(ref n) = node1.right {
            do_delete(n,key);
        }
    }
}


fn do_insert(node:&Rc<RefCell<TreeNode>>,key:i32,value:String) {
    let mut node1 = node.borrow_mut();
    //println!("doinsert {} {} {}",key,value,node1.key);
    if key < node1.key {
        match node1.left {
            None => {
                node1.left = Some(TreeNode::new(key,value));
            },
            Some(ref n) => {
                do_insert(n,key,value);
            }
        }
    }
    else if key > node1.key {
        match node1.right {
            None => {
                node1.right = Some(TreeNode::new(key,value));
            },
            Some(ref n) => {
                do_insert(n,key,value);
            }
        }
    }else{
        node1.vaild = true;
        node1.value = value;
    }
}

impl Bst {

    pub fn new() -> Bst {
        Bst {
            count:0,
            root:None
        }
    }

    pub fn insert(&mut self,key:i32,value:String){
        if let Some(_) = self.bst_get(key) {
            return;
        }
        match self.root {
            None => {
                self.root = Some(TreeNode::new(key,value));
            },
            Some(ref n) => {
                do_insert(&n,key,value);
            }
        }
        self.count = self.count+1;
    }

    pub fn bst_get(&self,key:i32) -> Option<String>{
        match self.root {
            None => None,
            Some(ref n) => bst_search(n,key)
        }
    }


    pub fn bst_delete(&self,key:i32){
        if let None = self.bst_get(key) {
            return;
        }
        match self.root {
            None => {},
            Some(ref n) => {
                do_delete(n,key)
            }
        };
    }

}

#[cfg(test)]
mod test {
    use super::*;

    fn insert(t:&mut Bst){
        t.insert(5,String::from("Hello5"));
        t.insert(3,String::from("Hello3"));
        t.insert(6,String::from("Hello6"));
        t.insert(4,String::from("Hello4"));
        t.insert(1,String::from("Hello1"));
    }

    #[test]
    fn test_search(){
        let mut t = Bst::new();
        insert(&mut t);
        assert_eq!(t.bst_get(6),Some(String::from("Hello6")));
        assert_eq!(t.bst_get(1),Some(String::from("Hello1")));
        assert_eq!(t.bst_get(5),Some(String::from("Hello5")));
        assert_eq!(t.bst_get(3),Some(String::from("Hello3")));
        assert_eq!(t.bst_get(7),None);
    }

    #[test]
    fn test_insert_delete(){
        let mut t = Bst::new();
        insert(&mut t);
        t.bst_delete(4);
        assert_eq!(t.bst_get(4),None);
        t.insert(4,String::from("Hellokkk"));
        assert_eq!(t.bst_get(4),Some(String::from("Hellokkk")));
    }

}