#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::syscall::*;

#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    println!("tid: {}",sys_get_tid());
    println!("clone: {}",sys_clone());
    println!("tid: {}",sys_get_tid());
    0
}
