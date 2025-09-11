#![allow(clippy::too_many_arguments, clippy::trivially_copy_pass_by_ref, clippy::missing_safety_doc, clippy::unnecessary_cast)]
#![allow(unsafe_op_in_unsafe_fn)]

pub mod vk;
pub mod builder;

use lazy_static::lazy_static;
use std::{
    ffi::CStr,
    os::raw::{c_int, c_void},
    mem::{self, MaybeUninit},
    path::Path,
    ptr,
    result,
    slice,
};
use shared_library::dynamic_library::DynamicLibrary;

#[doc(no_inline)]
pub use self::builder::*;

pub type Result<T> = result::Result<T, vk::Result>;

struct Lib {
    _lib: DynamicLibrary,
    fp_get_instance_proc_addr: vk::FnGetInstanceProcAddr,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    DynamicLibrary(String),
    MissingSymbol(&'static CStr),
    Vulkan(vk::Result),
}

impl From<vk::Result> for LoadError {
    fn from(err: vk::Result) -> Self {
        LoadError::Vulkan(err)
    }
}

pub type LoadResult<T> = result::Result<T, LoadError>;

#[cfg(target_os = "linux")]
const DL_PATH: &str = "libvulkan.so.1";

#[cfg(target_os = "windows")]
const DL_PATH: &str = "vulkan-1.dll";

#[cfg(target_os = "android")]
const DL_PATH: &str = "libvulkan.so";

impl Lib {
    pub fn new() -> LoadResult<Self> {
        match DynamicLibrary::open(Some(Path::new(&DL_PATH))) {
            Ok(lib) => match unsafe {
                lib.symbol("vkGetInstanceProcAddr")
                    .map(|f: *mut c_void| mem::transmute(f))
            } {
                Ok(fp_get_instance_proc_addr) => Ok(Self {
                    _lib: lib,
                    fp_get_instance_proc_addr,
                }),
                Err(s) => Err(LoadError::DynamicLibrary(s)),
            },
            Err(s) => Err(LoadError::DynamicLibrary(s)),
        }
    }
}

lazy_static! {
    static ref LIB: LoadResult<Lib> = Lib::new();
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum EnumerateResult {
    Success,
    Incomplete,
}

unsafe fn enumerate_generic_to_vec<T>(enumerator: impl Fn(&mut u32, *mut T) -> Result<EnumerateResult>) -> Result<Vec<T>> {
    let mut v = Vec::new();
    loop {
        let mut capacity = 0;
        match enumerator(&mut capacity, ptr::null_mut())? {
            EnumerateResult::Success => {},
            EnumerateResult::Incomplete => return Err(vk::Result::ERROR_UNKNOWN),
        };
        v.reserve(capacity as usize);

        let mut len = capacity;
        let values_res = enumerator(&mut len, v.as_mut_ptr())?;
        match values_res {
            EnumerateResult::Success => {
                v.set_len(len as usize);
                return Ok(v);
            },
            EnumerateResult::Incomplete => {},
        }
    }
}

unsafe fn enumerate_generic_unchecked_to_vec<T>(enumerator: impl Fn(&mut u32, *mut T)) -> Vec<T> {
    let mut len = 0;
    enumerator(&mut len, ptr::null_mut());

    let mut v = Vec::with_capacity(len as usize);
    enumerator(&mut len, v.as_mut_ptr());
    v.set_len(len as usize);
    v
}
