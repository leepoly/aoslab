#![no_std]
#![no_main]

extern crate alloc;

#[macro_use]

extern crate user;
use user::console::*;
use user::syscall::{
    sys_open,
    sys_close,
    sys_read,
    sys_write,
};

const BUFFER_SIZE: usize = 20;
const FILE: &'static str = "temp123\0";
const TEXT: &'static str = "Hello world!\0";

#[no_mangle]
pub fn main() -> usize {
    println!("\nTask3: drop inode and reopen file 1");
    let read_fd = sys_open(FILE.as_ptr(), O_RDONLY);
    println!("ready to read file {}", FILE);
    let mut read = [0u8; BUFFER_SIZE];
    let len = sys_read(read_fd as usize, &mut read, BUFFER_SIZE); // Liyiwei: can &mut be used?
    assert!(len != -1);
    print!("content = ");
    for i in 0usize..len as usize {
        // assert!(read[i] == TEXT.as_bytes()[i]);
        putchar(read[i] as char);
    }
    putchar('\n');
    println!("read from file 'temp123' successfully...");
    sys_close(read_fd as i32);
    0
}
