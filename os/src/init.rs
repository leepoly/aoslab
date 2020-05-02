global_asm!(include_str!("boot/entry64.asm"));
global_asm!(include_str!("link_user.S"));

use crate::consts::*;
use crate::memory::{
    alloc_frame,
    dealloc_frame
};

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn end();
    }
    println!("kernel end vaddr = {:#x}", end as usize);
    println!(
        "free physical memory ppn = [{:#x}, {:#x})",
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );
    crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );
    // init memory before interuption, because some int depend on memory mapping
    crate::interrupt::init();
    // dynamic_allocating_test();
    // write_readonly_test();
    // execute_unexecutable_test();
    // read_invalid_test();

    // unsafe {
    //     asm!("ebreak"::::"volatile");
    // }
    // panic!("end of rust_main");
    crate::fs::init();
    crate::process::init();
    crate::timer::init();
    crate::process::run();
    loop {}
}
