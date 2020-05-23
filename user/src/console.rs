//! 在系统调用基础上实现 `print!` `println!`
//!
//! 代码与 `os` crate 中的 `console.rs` 基本相同

use crate::syscall::*;
use core::fmt::{self, Write};
use alloc::string::String;

/// 实现 [`core::fmt::Write`] trait 来进行格式化输出
struct Stdout;

impl Write for Stdout {
    /// 打印一个字符串
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(STDOUT, s.as_bytes(), s.chars().count());
        Ok(())
    }
}

/// 打印由 [`core::format_args!`] 格式化后的数据
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 实现类似于标准库中的 `print!` 宏
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// 实现类似于标准库中的 `println!` 宏
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

/// 从控制台读取一个字符（阻塞）
pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    sys_read(STDIN, &mut c, 1);
    c[0]
}

/// 从控制台读取一个或多个字符（阻塞）
pub fn getchars() -> String {
    let mut buffer = [0u8; 64];
    loop {
        let size = sys_read(STDIN, &mut buffer, 64);
        if let Ok(string) = String::from_utf8(buffer.iter().copied().take(size as usize).collect()) {
            return string;
        }
    }
}

pub fn putchar(ch: char) {
    let mut ch_u8 = [0u8; 1];
    ch.encode_utf8(&mut ch_u8); // liyiwei: change char to &[u8]
    sys_write(STDOUT, &ch_u8, 1);
}

pub fn puts(s: &str) {
    for ch in s.chars() {
        putchar(ch);
    }
}

pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_CREAT: u32 = 64;
pub const O_APPEND: u32 = 1024;