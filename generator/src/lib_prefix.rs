#![allow(clippy::too_many_arguments, clippy::trivially_copy_pass_by_ref, clippy::missing_safety_doc, clippy::unnecessary_cast)]

pub mod vk;
pub mod builder;

use lazy_static::lazy_static;
use std::{
    ffi::CStr,
    os::raw::{c_void, c_int},
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
        match DynamicLibrary::open(Some(Path::new(&DL_PATH))) {
            Ok(lib) => match unsafe {
                lib.symbol("vkGetInstanceProcAddr")
                    .map(|f: *mut c_void| mem::transmute(f))
            } {
                Ok(fp_get_instance_proc_addr) => Ok(Self {
                    _lib: lib,
                    fp_get_instance_proc_addr,
                }),
                Err(s) => Err(LoaderError::MissingSymbol(s)),
            },
            Err(s) => Err(LoaderError::DynamicLibrary(s)),
        }
    }

    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Option<vk::Instance>,
        name: &CStr,
    ) -> Option<vk::FnVoidFunction> {
        (self.fp_get_instance_proc_addr)(instance, name.as_ptr())
    }
}

lazy_static! {
    static ref LIB: LoaderResult<Lib> = Lib::new();
}

struct VecMaybeUninit<T>(Vec<MaybeUninit<T>>);

impl<T> VecMaybeUninit<T> {
    fn with_len(n: usize) -> Self {
        let mut v = Vec::with_capacity(n);
        unsafe { v.set_len(n); }
        Self(v)
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self.0.as_mut_ptr() as *mut T
    }

    unsafe fn assume_init(self) -> Vec<T> {
        let s: Box<[T]> = mem::transmute(self.0.into_boxed_slice());
        s.into_vec()
    }
}

