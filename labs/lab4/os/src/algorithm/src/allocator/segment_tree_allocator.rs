use super::Allocator;
use alloc::{vec, vec::Vec};

pub struct SegmentTreeAllocator {
    tree: Vec<bool>,
}

impl Allocator for SegmentTreeAllocator {
    fn new(capacity: usize) -> Self {
        assert!(capacity >= 8, "capacity is too low");
        let leaf_count = capacity.next_power_of_two();
        let mut tree = vec![false; 2 * leaf_count];
        for i in capacity..leaf_count {
            tree[leaf_count + i] = true;
        }
        for i in (1..leaf_count).rev() {
            let v = tree[i * 2] && tree[i * 2 + 1];
            tree[i] = v;
        }
        Self { tree }
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.tree[1] == true {
            // tree is full
            None
        } else {
            let mut node = 1;
            // search for a false leaf
            while node < self.tree.len() / 2 {
                if self.tree[node * 2] == false {
                    node *= 2;
                } else if self.tree[node * 2 + 1] == false {
                    node = node * 2 + 1;
                } else {
                    panic!("tree is full or damaged");
                }
            }
            assert!(!self.tree[node], "tree is damaged");
            self.update_node(node, true);
            Some(node - self.tree.len() / 2)
        }
    }

    fn dealloc(&mut self, index: usize) {
        let node = index + self.tree.len() / 2;
        assert!(self.tree[node]);
        self.update_node(node, false);
    }
}

impl SegmentTreeAllocator {
    fn update_node(&mut self, mut index: usize, value: bool) {
        self.tree[index] = value;
        while index > 1 {
            index /= 2;
            let v = self.tree[index * 2] && self.tree[index * 2 + 1];
            self.tree[index] = v;
        }
    }
}
