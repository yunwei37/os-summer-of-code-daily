use crate::list::List;

#[derive(Debug)]
pub struct Stack {
    content:List
}

impl Stack {

    pub fn new() -> Stack {
        Stack {
            content: List::new(),
        }
    }

    pub fn stack_push(&mut self,value:i32){
        self.content.list_push(value);
    }

    pub fn stack_pop(&mut self) -> i32 {
        self.content.list_pop()
    }

    pub fn stack_top(&self) -> i32 {
        self.content.list_last()
    } 

    pub fn stack_count(&self) -> i32 {
        self.content.list_count()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push_pop(){
        let mut a = Stack::new();
        a.stack_push(1);
        a.stack_push(2);
        a.stack_push(3);
        assert_eq!(a.stack_top(),3);
        assert_eq!(a.stack_count(),3);
        assert_eq!(a.stack_pop(),3);
        assert_eq!(a.stack_pop(),2);
        assert_eq!(a.stack_top(),1);
        assert_eq!(a.stack_pop(),1);
        assert_eq!(a.stack_count(),0);
    }

    #[test]
    #[should_panic]
    fn test_pop_empty(){
        let mut a = Stack::new();
        a.stack_push(1);
        a.stack_pop();
        a.stack_pop();
    }

}