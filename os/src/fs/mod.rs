//! 文件系统
//!
//! 将读取第一个块设备作为根文件系统

use crate::drivers::{
    block::BlockDevice,
    driver::{DeviceType, DRIVERS},
};
use crate::kernel::Condvar;
use alloc::{sync::Arc, vec::Vec};
use core::any::Any;
use core::mem;
use lazy_static::lazy_static;
use rcore_fs_sfs::SimpleFileSystem;
use rcore_fs_lfs::LogFileSystem;
use spin::Mutex;

/// magic number for lfs
pub const MAGIC_LFS: u32 = 0x2f8dbe2c;
pub const MAGIC_SFS: u32 = 0x2f8dbe2b;

mod config;
mod inode_ext;
mod stdin;
mod stdout;

pub use rcore_fs::{dev::block_cache::BlockCache, vfs::*};
pub use config::*;
pub use inode_ext::INodeExt;
pub use stdin::STDIN;
pub use stdout::STDOUT;

lazy_static! {
    /// 根文件系统的根目录的 INode
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        // 选择第一个块设备
        for driver in DRIVERS.read().iter() {
            if driver.device_type() == DeviceType::Block {
                let device = BlockDevice(driver.clone());
                // read first 4B on the disk, for multi-FS support
                let mut magic_buf = [0u8; 512];
                let magic_word;
                device.0.read_block(0, &mut magic_buf);
                unsafe {
                    let mut magic_word_buf = [0u8; 4];
                    magic_word_buf.copy_from_slice(&magic_buf[0..4]);
                    magic_word = mem::transmute::<[u8; 4], u32>(magic_word_buf);
                }
                // println!("Disk magic is {}", magic_word);
                match magic_word {
                    MAGIC_LFS => {
                        println!("Loading a LFS disk");
                        // 动态分配一段内存空间作为设备 Cache
                        let device_with_cache = Arc::new(BlockCache::new(device, BLOCK_CACHE_CAPACITY));
                        return LogFileSystem::open(device_with_cache)
                        .expect("failed to open SFS")
                        .root_inode();
                    },
                    MAGIC_SFS => {
                        println!("Loading a SFS disk");
                        // 动态分配一段内存空间作为设备 Cache
                        let device_with_cache = Arc::new(BlockCache::new(device, BLOCK_CACHE_CAPACITY));
                        return SimpleFileSystem::open(device_with_cache)
                            .expect("failed to open SFS")
                            .root_inode();
                    },
                    _ => {
                        panic!("unknown FS magic number")
                    }
                }
            }
        }
        panic!("failed to load fs")
    };
}

/// 触发 [`static@ROOT_INODE`] 的初始化并打印根目录内容
pub fn init() {
    ROOT_INODE.ls();
    println!("");
    println!("mod fs initialized");
}
