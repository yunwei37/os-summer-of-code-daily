use crate::list::List;

#[derive(Debug)]
pub struct Queue {
    content:List
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            content: List::new(),
        }
    }

    pub fn queue_push(&mut self,value:i32){
        self.content.list_push(value);
    }

    pub fn queue_pop(&mut self) -> i32 {
        self.content.list_shift()
    }

    pub fn queue_top(&self) -> i32 {
        self.content.list_first()
    } 

    pub fn queue_bottom(&self) -> i32 {
        self.content.list_last()
    } 

    pub fn queue_count(&self) -> i32 {
        self.content.list_count()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push_pop(){
        let mut a = Queue::new();
        a.queue_push(1);
        a.queue_push(2);
        a.queue_push(3);
        assert_eq!(a.queue_count(),3);
        assert_eq!(a.queue_bottom(),3);
        assert_eq!(a.queue_pop(),1);
        assert_eq!(a.queue_pop(),2);
        assert_eq!(a.queue_top(),3);
        assert_eq!(a.queue_pop(),3);
        assert_eq!(a.queue_count(),0);
    }

    #[test]
    #[should_panic]
    fn test_pop_empty(){
        let mut a = Queue::new();
        a.queue_push(1);
        a.queue_pop();
        a.queue_pop();
    }
    
}