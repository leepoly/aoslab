#![no_std]
#![no_main]

#[macro_use]
extern crate user;

use user::console::*;

#[no_mangle]
pub fn main() -> ! {
    println!("\x1b[2J<notebook>");
    loop {
        let string = getchars();
        print!("{}", string);
    }
}