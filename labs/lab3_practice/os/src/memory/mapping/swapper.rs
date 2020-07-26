//! 页面置换算法

use super::*;
use crate::memory::{frame::FrameTracker, *};
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use bitflags::*;


/// 管理一个线程所映射的页面的置换操作
pub trait Swapper {
    /// 新建带有一个分配数量上限的置换器
    fn new(quota: usize) -> Self;

    /// 是否已达到上限
    fn full(&self) -> bool;

    /// 取出一组映射
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)>;

    /// 添加一组映射（不会在以达到分配上限时调用）
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry);

    /// 只保留符合某种条件的条目（用于移除一段虚拟地址）
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool);
}

pub type SwapperImpl = ClockSwapper;

/// 页面置换算法基础实现：FIFO
pub struct FIFOSwapper {
    /// 记录映射和添加的顺序
    queue: VecDeque<(VirtualPageNumber, FrameTracker)>,
    /// 映射数量上限
    quota: usize,
}

impl Swapper for FIFOSwapper {
    fn new(quota: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        self.queue.pop_front()
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, _entry: *mut PageTableEntry) {
        self.queue.push_back((vpn, frame));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _)| predicate(vpn));
    }
}


pub struct ClockSwapper {
    /// 记录映射和添加的顺序
    queue: Vec<(VirtualPageNumber, FrameTracker, usize)>,
    /// 映射数量上限
    quota: usize,
}


impl Swapper for ClockSwapper {

    fn new(quota: usize) -> Self {
        Self {
            queue: Vec::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        let len = self.queue.len()-1;
        let mut index:usize = 0;
        let mut isfound:bool = false;
        unsafe{
            loop{
                for i in 0..(len+1){
                    let pe = self.queue[i].2.clone() as *mut PageTableEntry;
                    //println!("{},{:?}",i,(*pe));
                    let mut flags = (*pe).flags().clone();
                    if flags.contains(Flags::ACCESSED) {
                        flags.set(Flags::ACCESSED,false);
                        //println!("{:?}",flags);
                        (*pe).set_flags(flags);
                    } else {
                        //println!("{:?}",flags);
                        index = i;
                        isfound = true;
                        break;
                    }
                }
                if isfound {
                    break;
                }
            }
        }
        //println!("{:?}",index);
        let f = self.queue.remove(index);
        Some((f.0,f.1))
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry) {
        self.queue.push((vpn, frame, entry as usize));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _, _)| predicate(vpn));
    }
}

