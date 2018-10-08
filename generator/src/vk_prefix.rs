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
    pub fn from_raw_parts(major: u32, minor: u32, patch: u32) -> Self {
        Version((major << 22) | ((minor & 0x3ff) << 12) | (patch & 0xfff))
    }

    pub fn from_raw(version: u32) -> Self {
        Version(version)
    }
    pub fn to_raw(&self) -> u32 {
        self.0
    }

    pub fn get_major(&self) -> u32 {
        self.0 >> 22
    }
    pub fn get_minor(&self) -> u32 {
        (self.0 >> 12) & 0x3ff
    }
    pub fn get_patch(&self) -> u32 {
        self.0 & 0xfff
    }
}
impl default::Default for Version {
    fn default() -> Self {
        Version(0)
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

