AOS Project
===

This is a project in AOS course. Based on rCore tutorial, multi-FS support and a more complex BlockDevice struct have been added. As opposed to reusing SFS, we also implement a Log-Structured File System (LFS). A thorough evaluation and document can be found in `/document/`.

Usage
---

`git clone https://github.com/leepoly/aoslab rcore`

`git clone https://github.com/leepoly/rcore-fs # we use a modified rcore-fs-fuse tool in it`

`make run -C ./rcore # start full simulation with LFS`

This would generate a LFS image and load it to boot rcore kernel. More commands are supported in the inner Makefile (for example, see `rcore/user/Makefile` for generating different types of file systems)


Utils
---
Utilities are in `utils/`.
File `generator.py` would randomly creates files and directories.
`trace_sim.py` is a rough hard disk drive model for stats. (still working on...)
`aoslab-lfs.cfg` is a demo config file for SimpleSSD.
See the code for their usages.


