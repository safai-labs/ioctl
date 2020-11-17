`ioctl-sys` & `ioctls`
======================

[![ioctl-sys Crates.io](https://img.shields.io/crates/v/ioctl-sys.svg?style=flat-square)](https://crates.io/crates/ioctl-sys)
[![ioctls Crates.io](https://img.shields.io/crates/v/ioctls.svg?style=flat-square)](https://crates.io/crates/ioctls)
[ioctl-sys ![ioctl-sys Documentation](https://docs.rs/ioctl-sys/badge.svg)](https://docs.rs/ioctl-sys)
[ioctls ![ioctls Documentation](https://docs.rs/ioctls/badge.svg)](https://docs.rs/ioctls)

Helpers for binding `ioctl`s in Rust. Currently supports Linux on all architectures
except SPARC and Alpha. Other platforms welcome!

The `ioctl-sys` crate provides a basic interface to write your own ioctl wrappers.

The `ioctls` crate provides wrappers for a bunch of ioctls.

This library is pretty low-level and messy. `ioctl` is not fun.

What is an `ioctl`?
===================

The `ioctl` function is the grab-bag system call on POSIX systems. Don't want
to add a new syscall? Make it an `ioctl`! `ioctl` refers to both the syscall,
and the commands that can be send with it. `ioctl` stands for "IO control",
and the commands are always sent to a file descriptor.

What does this library support?
===============================

This library provides the `ioctl!` macro, for binding `ioctl`s. It also tries
to bind every `ioctl` supported by the system with said macro, but
many `ioctl`s require some amount of manual work to support (usually by
providing `struct`s or other types) that this library does not support yet.

Additionally, in `etc`, there are scripts for scraping system headers for
`ioctl` definitions, and generating calls to `ioctl!` corresponding to them.

How do I get the magic numbers?
===============================

Look at your system's headers. For example, `/usr/include/linux/input.h` has a
lot of lines defining macros which use `_IOR`, `_IOW`, `_IOC`, and `_IORW`.
These macros correspond to the `ior!`, `iow!`, `ioc!`, and `iorw!` macros
defined in this crate. Additionally, there is the `ioctl!` macro for
creating a wrapper around `ioctl` that is somewhat more type-safe.

Most `ioctl`s have no or little documentation. You'll need to scrounge through
the source to figure out what they do and how they should be used.

How do I figure out an ioctl's calling convention?
==================================================

For linux, you must look at the ioctl handlers in the kernel itself to determine how the value passed is being used. Look for the `copy_from_user()` and `get_user()` calls, these copy memory from userspace and may indicate that the ioctl's arg is a pointer. In othercases, the ioctl argument may simply be cast to an integer of some sort.

Example
=======

```rust
use ioctl_sys::ioctl;

ioctl!(bad kiocsound with 0x4B2F);
ioctl!(none drm_ioctl_set_master with b'd', 0x1e);
ioctl!(read ev_get_version with b'E', 0x01; u32);
ioctl!(write ev_set_repeat with b'E', 0x03; [u32; 2]);

fn main() {
    let mut x = 0;
    let ret = unsafe { ev_get_version(0, &mut x) };
    println!("returned {}, x = {}", ret, x);
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
