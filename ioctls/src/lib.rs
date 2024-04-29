#[macro_use]
extern crate ioctl_sys;
pub use ioctl_sys::ioctl;

#[doc(hidden)]
pub extern crate libc;

#[cfg(any(target_os = "linux", target_os = "android"))]
#[path = "platform/linux.rs"]
#[macro_use]
mod platform;

pub use platform::*;

#[cfg(not(any(target_os = "linux", target_os = "android")))]
use platform_not_supported;
