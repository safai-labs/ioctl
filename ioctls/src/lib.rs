#[macro_use]
extern crate ioctl_sys;
pub use ioctl_sys::ioctl;

#[doc(hidden)]
pub extern crate libc;

#[cfg(target_os = "linux")]
#[path = "platform/linux.rs"]
#[macro_use]
mod platform;

pub use platform::*;

#[cfg(not(target_os = "linux"))]
use platform_not_supported;
