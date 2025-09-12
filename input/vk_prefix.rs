#![allow(clippy::too_many_arguments, clippy::unreadable_literal)]

use std::{
    default::Default,
    os::raw::{c_void, c_char, c_int, c_ulong},
    ptr,
    mem,
    fmt,
    ops,
};

/// Wrapper around Vulkan API version number
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Version(u32);
impl Version {
    /// Forms a version number from major, minor and patch numbers
    ///
    /// ```
    /// # use spark::vk;
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
impl Default for Version {
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

fn display_bitmask(bits: u64, bit_names: &[(u64, &str)], f: &mut fmt::Formatter) -> fmt::Result {
    let mut has_output = false;
    let mut remain = bits;
    for (bit, name) in bit_names.iter().copied() {
        if (remain & bit) == bit {
            if has_output {
                f.write_str(" | ")?;
            }
            f.write_fmt(format_args!("{name}"))?;
            has_output = true;
            remain &= !bit;
        }
    }
    if remain != 0 {
        if has_output {
            f.write_str(" | ")?;
        }
        f.write_fmt(format_args!("{remain:#x}"))?;
        has_output = true;
    }
    if !has_output {
        f.write_str("0")?;
    }
    Ok(())
}

macro_rules! impl_bitmask {
    ($name:ident) => {
        impl $name {
            pub fn empty() -> Self {
                Self(0)
            }
            pub fn is_empty(self) -> bool {
                self.0 == 0
            }
            pub fn intersects(self, other: Self) -> bool {
                (self.0 & other.0) != 0
            }
            pub fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }
        }
        impl ops::BitOr for $name {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self {
                Self(self.0 | rhs.0)
            }
        }
        impl ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
        impl ops::BitAnd for $name {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self {
                Self(self.0 & rhs.0)
            }
        }
        impl ops::BitAndAssign for $name {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0; 
            }
        }
        impl ops::BitXor for $name {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self {
                Self(self.0 ^ rhs.0)
            }
        }
        impl ops::BitXorAssign for $name {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
    }
}

macro_rules! impl_handle {
    ($name:ident) => {
        impl $name {
            pub fn null() -> Self {
                Self(0)
            }
            pub fn is_null(self) -> bool {
                self.0 == 0
            }
        }
    }
}
