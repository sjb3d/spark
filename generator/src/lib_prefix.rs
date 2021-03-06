#![allow(clippy::too_many_arguments, clippy::trivially_copy_pass_by_ref, clippy::missing_safety_doc)]

pub mod vk;
pub mod builder;

use lazy_static::lazy_static;
use std::ffi::CStr;
use std::os::raw::{c_void, c_int};
use std::mem;
use std::mem::MaybeUninit;
use std::path::Path;
use std::ptr;
use std::result;
use std::slice;
use shared_library::dynamic_library::DynamicLibrary;

#[doc(no_inline)]
pub use self::builder::*;

pub type Result<T> = result::Result<T, vk::Result>;

struct Lib {
    pub lib: DynamicLibrary,
    pub fp_get_instance_proc_addr: vk::FnGetInstanceProcAddr,
}

#[derive(Debug, Clone)]
pub enum LoaderError {
    DynamicLibrary(String),
    MissingSymbol(String),
    Vulkan(vk::Result),
}

impl From<vk::Result> for LoaderError {
    fn from(err: vk::Result) -> Self {
        LoaderError::Vulkan(err)
    }
}

pub type LoaderResult<T> = result::Result<T, LoaderError>;

#[cfg(target_os = "linux")]
const DL_PATH: &str = "libvulkan.so.1";

#[cfg(target_os = "windows")]
const DL_PATH: &str = "vulkan-1.dll";

#[cfg(target_os = "android")]
const DL_PATH: &str = "libvulkan.so";

impl Lib {
    pub fn new() -> LoaderResult<Self> {
        match DynamicLibrary::open(Some(&Path::new(&DL_PATH))) {
            Ok(lib) => match unsafe {
                lib.symbol("vkGetInstanceProcAddr")
                    .map(|f: *mut c_void| mem::transmute(f))
            } {
                Ok(fp_get_instance_proc_addr) => Ok(Self {
                    lib,
                    fp_get_instance_proc_addr,
                }),
                Err(s) => Err(LoaderError::MissingSymbol(s)),
            },
            Err(s) => Err(LoaderError::DynamicLibrary(s)),
        }
    }

    pub unsafe fn get_instance_proc_addr(&self, name: &CStr) -> Option<vk::FnVoidFunction> {
        (self.fp_get_instance_proc_addr)(None, name.as_ptr())
    }
}

lazy_static! {
    static ref LIB: LoaderResult<Lib> = Lib::new();
}
