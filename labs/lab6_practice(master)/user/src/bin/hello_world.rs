#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::alloc::string::String;
use user_lib::syscall::*;

#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    println!("tid: {}", sys_get_tid());
    let fd = sys_open("hello_world.rs");
    println!("fd: {}", fd);
    let mut buffer = [0u8; 1024];
    let size = sys_read(fd as usize, &mut buffer);
    if let Ok(string) = String::from_utf8(buffer.iter().copied().take(size as usize).collect()) {
        print!("{}", string);
    }
    0
}
