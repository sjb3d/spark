#![allow(clippy::too_many_arguments, clippy::unreadable_literal)]

use std::default;
use std::os::raw::{c_void, c_char, c_int, c_ulong};
use std::ptr;
use std::ffi::CStr;
use std::mem;
use std::num;
use std::fmt;
use std::ops;

/// Wrapper around Vulkan API version number
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Version(u32);
impl Version {
    /// Forms a version number from major, minor and patch numbers
    ///
    /// ```
    /// # use vkr::vk;
    /// let v = vk::Version::from_raw_parts(1, 2, 0);
    /// assert_eq!(v.to_raw(), (1 << 22) | (2 << 12));
    /// ```
    pub const fn from_raw_parts(major: u32, minor: u32, patch: u32) -> Self {
        Self((major << 22) | ((minor & 0x3ff) << 12) | (patch & 0xfff))
    }

    pub const fn from_raw(version: u32) -> Self {
        Self(version)
    }
    pub fn to_raw(self) -> u32 {
        self.0
    }

    pub fn get_major(self) -> u32 {
        self.0 >> 22
    }
    pub fn get_minor(self) -> u32 {
        (self.0 >> 12) & 0x3ff
    }
    pub fn get_patch(self) -> u32 {
        self.0 & 0xfff
    }
}
impl default::Default for Version {
    fn default() -> Self {
        Self::from_raw_parts(1, 0, 0)
    }
}
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.get_major(), self.get_minor(), self.get_patch())
    }
}

// TODO: replace with ! type when stable
#[doc(hidden)]
pub enum Never {}

// X11
pub type Display = Never;
pub type VisualID = c_ulong;
pub type Window = c_ulong;
pub type RROutput = c_ulong;

// MIR
pub type MirConnection = Never;
pub type MirSurface = Never;

// wayland
#[allow(non_camel_case_types)]
pub type wl_display = Never;
#[allow(non_camel_case_types)]
pub type wl_surface = Never;

// windows
pub type HINSTANCE = *mut c_void;
pub type HWND = *mut c_void;
pub type HANDLE = *mut c_void;
pub type HMONITOR = *mut c_void;
#[allow(non_camel_case_types)]
pub type SECURITY_ATTRIBUTES = Never;
pub type DWORD = c_ulong;
pub type LPCWSTR = *const u16;

#[allow(non_camel_case_types)]
pub type xcb_connection_t = Never;
#[allow(non_camel_case_types)]
pub type xcb_window_t = u32;
#[allow(non_camel_case_types)]
pub type xcb_visualid_t = Never;

// Android
pub type ANativeWindow = Never;
pub type AHardwareBuffer = Never;

// Metal
pub type CAMetalLayer = Never;

// Zircon
#[allow(non_camel_case_types)]
pub type zx_handle_t = u32;

fn display_bitmask(bits: u32, bit_names: &[(u32, &str)], f: &mut fmt::Formatter) -> fmt::Result {
    let mut has_output = false;
    let mut remain = bits;
    for (bit, name) in bit_names.iter().cloned() {
        if (remain & bit) == bit {
            if has_output {
                f.write_str(" | ")?;
            }
            write!(f, "{}", name)?;
            has_output = true;
            remain &= !bit;
        }
    }
    if remain != 0 {
        if has_output {
            f.write_str(" | ")?;
        }
        write!(f, "{:#x}", remain)?;
        has_output = true;
    }
    if !has_output {
        f.write_str("0")?;
    }
    Ok(())
}
