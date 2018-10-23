
#[macro_use]
extern crate lazy_static;
extern crate shared_library;

pub mod vk;
pub mod builder;

use std::ffi::CStr;
use std::os::raw::{c_void, c_char, c_int};
use std::mem;
use std::path::Path;
use std::ptr;
use std::result;
use shared_library::dynamic_library::DynamicLibrary;

#[doc(no_inline)]
pub use self::builder::*;

// For methods to be generic over array length (until there is language support)
pub trait Array {
    type Item;
    fn as_mut_ptr(&mut self) -> *mut Self::Item;
    fn len() -> usize;
}

macro_rules! array_impl {
    ($len:expr) => (
        impl<T> Array for [T; $len] {
            type Item = T;
            fn as_mut_ptr(&mut self) -> *mut T { self as *mut _ as *mut _ }
            fn len() -> usize { $len }
        }
    )
}

array_impl!(1);
array_impl!(2);
array_impl!(3);
array_impl!(4);
array_impl!(5);
array_impl!(6);
array_impl!(7);
array_impl!(8);

pub type Result<T> = result::Result<T, vk::Result>;

type FnGetInstanceProcAddr = extern "system" fn(instance: Option<vk::Instance>, p_name: *const c_char,) -> Option<vk::FnVoidFunction>;

struct LibFn {
    pub get_instance_proc_addr: FnGetInstanceProcAddr,
}

struct Lib {
    pub lib: DynamicLibrary,
    pub fp: LibFn,
}

#[derive(Debug, Clone)]
pub enum LoaderError {
    DynamicLibrary(String),
    MissingSymbol(String),
    Vulkan(vk::Result),
}

impl From<vk::Result> for LoaderError {
    fn from(err: vk::Result) -> LoaderError {
        LoaderError::Vulkan(err)
    }
}

#[cfg(unix)]
const DL_PATH: &'static str = "libvulkan.so.1";

#[cfg(windows)]
const DL_PATH: &'static str = "vulkan-1.dll";

impl Lib {
    pub fn new() -> result::Result<Self, LoaderError> {
        match DynamicLibrary::open(Some(&Path::new(&DL_PATH))) {
            Ok(lib) => match unsafe {
                lib.symbol("vkGetInstanceProcAddr")
                    .map(|f: *mut c_void| mem::transmute(f))
            } {
                Ok(get_instance_proc_addr) => Ok(Self {
                    lib,
                    fp: LibFn { get_instance_proc_addr },
                }),
                Err(s) => Err(LoaderError::MissingSymbol(s)),
            },
            Err(s) => Err(LoaderError::DynamicLibrary(s)),
        }
    }

    pub unsafe fn get_instance_proc_addr(&self, instance: Option<vk::Instance>, name: &CStr) -> Option<vk::FnVoidFunction> {
        (self.fp.get_instance_proc_addr)(instance, name.as_ptr())
    }
}

lazy_static! {
    static ref LIB: result::Result<Lib, LoaderError> = Lib::new();
}
