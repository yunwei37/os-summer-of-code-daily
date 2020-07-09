
use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;

#[derive(Debug)]
struct ListNode
{
    value :i32,
    next: Option<Rc<RefCell<ListNode>>>,
    prev: Option<Rc<RefCell<ListNode>>>
}

#[derive(Debug)]
pub struct List{
    count: i32,
    first: Option<Rc<RefCell<ListNode>>>,
    last: Option<Rc<RefCell<ListNode>>>
}

impl ListNode {
    fn new(value:i32) -> Rc<RefCell<ListNode>>{
        let pointer = Rc::new(RefCell::new(ListNode {
            value,
            next: None,
            prev: None,
        }));
        Rc::clone(&pointer)
    }
}

impl List {

    pub fn new() -> List {
        let first = ListNode::new(0);
        let last = ListNode::new(0);
        first.borrow_mut().next = Some(Rc::clone(&last));
        last.borrow_mut().prev = Some(Rc::clone(&first));
        List {
            count: 0,
            first: Some(first),
            last: Some(last),
        }
    }

    pub fn list_count(&self) -> i32 {
        self.count
    }

    pub fn list_push(&mut self,value:i32){
        let node = ListNode::new(value);
        if let Some(ref l) = self.last {
            let mut n = node.borrow_mut();
            n.next = Some(Rc::clone(&l));
            if let Some(ref p) = l.borrow().prev {
                n.prev = Some(Rc::clone(&p));
                p.borrow_mut().next = Some(Rc::clone(&node));
            };
            l.borrow_mut().prev = Some(Rc::clone(&node));
        };
        self.count = self.count+1;
    }

    pub fn list_unshift(&mut self, value:i32){
        let node = ListNode::new(value);
        if let Some(ref f) = self.first {
            let mut n = node.borrow_mut();
            n.prev = Some(Rc::clone(&f));
            if let Some(ref p) = f.borrow().next {
                n.next = Some(Rc::clone(&p));
                p.borrow_mut().prev = Some(Rc::clone(&node));
            };
            f.borrow_mut().next = Some(Rc::clone(&node));
        };
        self.count = self.count+1;
    }

    pub fn list_shift(&mut self) -> i32 {
        if self.count == 0 {
            panic!("No Items for pop!");
        }
        let mut value = 0;
        let mut pointer_pnext = None;
        if let Some(ref f) = self.first {
            if let Some(ref p) = f.borrow().next {
                if let Some(ref pnext) = p.borrow().next {
                    pointer_pnext = Some(Rc::clone(&pnext));
                    pnext.borrow_mut().prev = Some(Rc::clone(&f));
                }
                value = p.borrow().value;
            };
            f.borrow_mut().next = pointer_pnext;
        };
        self.count = self.count-1;
        value
    }

    pub fn list_pop(&mut self) -> i32 {
        if self.count == 0 {
            panic!("No Items for pop!");
        }
        let mut value = 0;
        let mut pointer_pnext = None;
        if let Some(ref l) = self.last {
            if let Some(ref p) = l.borrow().prev {
                if let Some(ref pnext) = p.borrow().prev {
                    pointer_pnext = Some(Rc::clone(&pnext));
                    pnext.borrow_mut().next = Some(Rc::clone(&l));
                }
                value = p.borrow().value;
            };
            l.borrow_mut().prev = pointer_pnext;
        };
        self.count = self.count-1;
        value
    }

    pub fn list_first(&self) -> i32 {
        if self.count == 0 {
            panic!("No Items!");
        }
        let mut value = 0;
        if let Some(ref f) = self.first {
            if let Some(ref n) = f.borrow().next {
                value = n.borrow().value;
            };
        }
        value
    }

    pub fn list_last(&self) -> i32 {
        if self.count == 0 {
            panic!("No Items!");
        }
        let mut value = 0;
        if let Some(ref l) = self.last {
            if let Some(ref p) = l.borrow().prev {
                value = p.borrow().value;
            };
        }
        value
    }

    pub fn list_clear(&mut self){
        while self.count > 0 {
            self.list_pop();
        }
    }

}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_push_pop(){
        let mut a = List::new();
        a.list_push(1);
        a.list_push(2);
        assert_eq!(a.list_pop(),2);
        assert_eq!(a.list_pop(),1);
    }

    #[test]
    fn test_shift(){
        let mut a = List::new();
        a.list_unshift(3);
        a.list_unshift(1);
        a.list_unshift(2);
        assert_eq!(a.list_shift(),2);
        assert_eq!(a.list_shift(),1);
    }

    #[test]
    fn test_shift_push(){
        let mut a = List::new();
        a.list_push(1);
        a.list_push(2);
        assert_eq!(a.list_shift(),1);
        assert_eq!(a.list_shift(),2);
    }

    #[test]
    fn test_clear(){
        let mut a = List::new();
        a.list_push(1);
        a.list_push(2);
        a.list_clear();
        assert_eq!(a.list_count(),0);
    }

    #[test]
    #[should_panic]
    fn test_pop_empty(){
        let mut a = List::new();
        a.list_push(1);
        a.list_pop();
        a.list_pop();
    }

    #[test]
    fn test_first_last(){
        let mut a = List::new();
        a.list_push(1);
        a.list_push(2);
        a.list_push(3);
        assert_eq!(a.list_first(),1);
        assert_eq!(a.list_last(),3);
    }
}