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

impl Lib {
    pub fn new() -> Self {
        let lib = DynamicLibrary::open(Some(&Path::new("libvulkan.so.1"))).expect("failed to load vulkan library");

        let get_instance_proc_addr: FnGetInstanceProcAddr = unsafe {
            lib.symbol("vkGetInstanceProcAddr")
                .map(|f: *mut c_void| mem::transmute(f))
                .expect("failed to load vkGetInstanceProcAddr")
        };

        Self {
            lib,
            fp: LibFn { get_instance_proc_addr },
        }
    }

    pub unsafe fn get_instance_proc_addr(&self, instance: Option<vk::Instance>, name: &CStr) -> Option<vk::FnVoidFunction> {
        (self.fp.get_instance_proc_addr)(instance, name.as_ptr())
    }
}

lazy_static! {
    static ref LIB: Lib = Lib::new();
}
