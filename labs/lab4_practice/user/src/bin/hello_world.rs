#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main(num:i32) -> usize {
    //println!("Hello world from user mode program!");
    for i in 0..10000000{
        if i%1000000 == 0 {
            println!("Hello world from user mode {} program!{}",num,i);
        }
    }
    0
}
