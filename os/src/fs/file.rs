use alloc::sync::Arc;
use rcore_fs::vfs::{FileType, FsError, INode};
use rcore_fs_sfs::INodeImpl;
use crate::fs::ROOT_INODE;
use bitflags::bitflags;

#[derive(Copy,Clone,Debug)]
pub enum FileDescriptorType {
    FD_NONE,
    FD_INODE,
    FD_DEVICE,
}

#[derive(Clone)]
pub struct File {
    fdtype: FileDescriptorType,
    readable: bool,
    writable: bool,
    pub inode: Option<Arc<dyn INode>>,
    offset: usize,
}

/// Split a `path` str to `(base_path, file_name)`
fn split_path(path: &str) -> (&str, &str) {
    let mut split = path.trim_end_matches('/').rsplitn(2, '/');
    let file_name = split.next().unwrap();
    let mut dir_path = split.next().unwrap_or(".");
    if dir_path == "" {
        dir_path = "/";
    }
    (dir_path, file_name)
}

bitflags! {
    struct OpenFlags: usize {
        /// read only
        const RDONLY = 0;
        /// write only
        const WRONLY = 1;
        /// read write
        const RDWR = 2;
        /// create file if it does not exist
        const CREATE = 1 << 6;
        /// error if CREATE and the file exists
        const EXCLUSIVE = 1 << 7;
        /// truncate file upon open
        const TRUNCATE = 1 << 9;
        /// append on each write
        const APPEND = 1 << 10;
    }
}

impl File {
    pub fn default() -> Self {
        File {
            fdtype: FileDescriptorType::FD_NONE,
            readable: false,
            writable: false,
            inode: None,
            offset: 0,
        }
    }
    pub fn set_readable(&mut self, v: bool) { self.readable = v; }
    pub fn set_writable(&mut self, v: bool) { self.writable = v; }
    pub fn get_readable(&self) -> bool { self.readable }
    pub fn get_writable(&self) -> bool { self.writable }
    pub fn set_fdtype(&mut self, t: FileDescriptorType) { self.fdtype = t; }
    pub fn get_fdtype(&self) -> FileDescriptorType { self.fdtype }
    pub fn set_offset(&mut self, o: usize) { self.offset = o; }
    pub fn get_offset(&self) -> usize { self.offset }

    pub fn open_file(&mut self, path: &'static str, flags: i32) {
        self.set_fdtype(FileDescriptorType::FD_INODE);
        self.set_readable(true);
        if (flags & 1) > 0 {
            self.set_readable(false);
        }
        if (flags & 3) > 0 {
            self.set_writable(true);
        }
        println!("access mode {:?}", flags as u32);
        self.inode = if (flags & 64) > 0 {
            let (dir_path, file_name) = split_path(&path);
            // relative to cwd
            let dir_inode = ROOT_INODE.lookup(dir_path).unwrap();
            match dir_inode.find(file_name) {
                Ok(file_inode) => {
                    Some(file_inode)
                }
                Err(FsError::EntryNotFound) => {
                    Some(dir_inode.create(file_name, FileType::File, flags as u32).unwrap().clone())
                }
                Err(e) => None,
            }
        } else {
            Some(ROOT_INODE.lookup(path).unwrap().clone())
        };
        // self.inode = Some(ROOT_INODE.lookup(path).unwrap().clone());

        self.set_offset(0);
    }
}
