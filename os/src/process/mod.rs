pub mod structs;
pub mod scheduler;
pub mod thread_pool;
pub mod processor;

use structs::Thread;
use processor::Processor;
use scheduler::RRScheduler;
use thread_pool::ThreadPool;
use alloc::boxed::Box;
use crate::fs::{
    ROOT_INODE,
    INodeExt
};

pub type Tid = usize;
pub type ExitCode = usize;


static CPU: Processor = Processor::new();


#[no_mangle]
pub extern "C" fn temp_thread(from_thread: &mut Thread, current_thread: &mut Thread) {
    println!("I'm leaving soon, but I still want to say: Hello world!");
    current_thread.switch_to(from_thread);
}

pub fn init() {
    let scheduler = RRScheduler::new(1);
    // let scheduler = RRScheduler::new(2);
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));
    let idle = Thread::new_kernel(Processor::idle_main as usize);
    idle.append_initial_arguments([&CPU as *const Processor as usize, 0, 0]);
    CPU.init(idle, Box::new(thread_pool));

    // for i in 0..5 {
    //     CPU.add_thread({
    //         let thread = Thread::new_kernel(hello_thread as usize);
    //         thread.append_initial_arguments([i, 0, 0]);
    //         thread
    //     });
    // }

	// let data = ROOT_INODE
    //     .lookup("rust/notebook")
    //     // .lookup("rust/hello_world")
    //     .unwrap()
    //     .read_as_vec()
    //     .unwrap();
    // let user_thread = unsafe { Thread::new_user(data.as_slice()) };
    // CPU.add_thread(user_thread);

	execute("rust/user_shell", None);

    println!("++++ setup process!   ++++");
}

#[no_mangle]
pub extern "C" fn hello_thread(arg: usize) -> ! {
    println!("begin of thread {}", arg);
    for i in 0..800 {
        print!("{}", arg);
	}
    println!("\nend  of thread {}", arg);
    CPU.exit(0);
    loop {}
}

pub fn execute(path: &str, host_tid: Option<Tid>) -> bool {
    let find_result = ROOT_INODE.lookup(path);
    match find_result {
        Ok(inode) => {
            let data = inode.read_as_vec().unwrap();
            // 这里创建用户线程时，传入 host_tid
            let user_thread = unsafe { Thread::new_user(data.as_slice(), host_tid) };
            CPU.add_thread(user_thread);
            true
        },
        Err(_) => {
            // 如果找不到路径字符串对应的用户程序
            println!("command not found!");
            false
        }
    }
}

pub fn tick() {
    CPU.tick();
}

pub fn run() {
    CPU.run();
}

pub fn exit(code: usize) {
    CPU.exit(code);
}

pub fn yield_now() {
    CPU.yield_now();
}

pub fn wake_up(tid: Tid) {
    CPU.wake_up(tid);
}

pub fn current_tid() -> usize {
    CPU.current_tid()
}

pub fn current_thread_mut() -> &'static mut Thread {
    CPU.current_thread_mut()
}