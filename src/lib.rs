#[doc(hidden)]
pub extern crate libc;
#[macro_use]
extern crate bitflags;

#[cfg(target_os = "linux")]
#[path = "platform/linux.rs"]
#[macro_use]
mod platform;

pub use platform::*;

extern "C" {
    #[doc(hidden)]
    pub fn ioctl(fd: libc::c_int, req: libc::c_ulong, ...) -> libc::c_int;
}

#[cfg(not(target_os = "linux"))]
use platform_not_supported;
