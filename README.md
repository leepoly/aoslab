AOS Project
===

This is a project in AOS course. Based on rCore tutorial, multi-FS support and a more complex BlockDevice struct have been added. As opposed to reusing SFS, we also implement a Log-Structured File System (LFS). A thorough comparison and document can be found in `/document/`.

Usage
---

In aoslab, `make run` would generate a LFS image and load it to boot rcore kernel.

See https://github.com/leepoly/rcore-fs for image generating (fuse tool).

Utils
---
Utilities are in `utils/`.
File `generator.py` would randomly creates files and directories.
`trace_sim.py` is a rough hard disk drive model for stats. (still working on...)
`aoslab-lfs.cfg` is config file for SimpleSSD.SSD.
See code for their usages.
