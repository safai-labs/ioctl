use std::os::raw::{c_int, c_ulong};

#[cfg(target_os = "linux")]
#[macro_use]
mod platform;

pub use platform::*;

extern "C" {
    #[doc(hidden)]
    pub fn ioctl(fd: c_int, req: c_ulong, ...) -> c_int;
}

#[cfg(not(target_os = "linux"))]
use platform_not_supported;
