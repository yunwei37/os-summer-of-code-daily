#[no_mangle]
pub fn main() -> usize {
    println!("Hello world from user mode program!");
    println!("tid: {}",sys_get_tid());
    println!("clone: {}",sys_clone());
    println!("tid: {}",sys_get_tid());
    0
}
