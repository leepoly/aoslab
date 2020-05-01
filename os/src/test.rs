
// fn write_readonly_test() {
//     extern "C" {
//         fn srodata();
//     }
//     unsafe {
//         let ptr = srodata as usize as *mut u8;
//         *ptr = 0xab;
//     }
// }

// fn execute_unexecutable_test() {
//     extern "C" {
//         fn sbss();
//     }
//     unsafe {
//         asm!("jr $0" :: "r"(sbss as usize) :: "volatile");
//     }
// }

// fn read_invalid_test() {
//     println!("{}", unsafe { *(0x12345678 as usize as *const u8) });
// }

// fn frame_allocating_test() {
//     println!("alloc {:x?}", alloc_frame());
//     let f = alloc_frame();
//     println!("alloc {:x?}", f);
//     println!("alloc {:x?}", alloc_frame());
//     println!("dealloc {:x?}", f);
//     dealloc_frame(f.unwrap());
//     println!("alloc {:x?}", alloc_frame());
//     println!("alloc {:x?}", alloc_frame());
// }

// fn dynamic_allocating_test() {
//     use alloc::vec::Vec;
//     use alloc::boxed::Box;

//     extern "C" {
//         fn sbss();
//         fn ebss();
//     }
//     let lbss = sbss as usize;
//     let rbss = ebss as usize;

//     let heap_value = Box::new(5);
//     assert!(*heap_value == 5);
//     println!("heap_value assertion successfully!");
//     println!("heap_value is at {:p}", heap_value);
//     let heap_value_addr = &*heap_value as *const _ as usize;
//     assert!(heap_value_addr >= lbss && heap_value_addr < rbss);
//     println!("heap_value is in section .bss!");

//     let mut vec = Vec::new();
//     for i in 0..500 {
//         vec.push(i);
//     }
//     for i in 0..500 {
//         assert!(vec[i] == i);
//     }
//     println!("vec assertion successfully!");
//     println!("vec is at {:p}", vec.as_slice());
//     let vec_addr = vec.as_ptr() as usize;
//     assert!(vec_addr >= lbss && vec_addr < rbss);
//     println!("vec is in section .bss!");
// }