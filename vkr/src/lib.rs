//! Generated from vk.xml with `VK_HEADER_VERSION` 89

#[macro_use]
extern crate lazy_static;
extern crate shared_library;

pub mod builder;
pub mod vk;

use shared_library::dynamic_library::DynamicLibrary;
use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::path::Path;
use std::ptr;
use std::result;

#[doc(no_inline)]
pub use self::builder::*;

// For methods to be generic over array length (until there is language support)
pub trait Array {
    type Item;
    fn as_mut_ptr(&mut self) -> *mut Self::Item;
    fn len() -> usize;
}

macro_rules! array_impl {
    ($len:expr) => {
        impl<T> Array for [T; $len] {
            type Item = T;
            fn as_mut_ptr(&mut self) -> *mut T {
                self as *mut _ as *mut _
            }
            fn len() -> usize {
                $len
            }
        }
    };
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

type FnGetInstanceProcAddr =
    extern "system" fn(instance: Option<vk::Instance>, p_name: *const c_char) -> Option<vk::FnVoidFunction>;

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

    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Option<vk::Instance>,
        name: &CStr,
    ) -> Option<vk::FnVoidFunction> {
        (self.fp.get_instance_proc_addr)(instance, name.as_ptr())
    }
}

lazy_static! {
    static ref LIB: result::Result<Lib, LoaderError> = Lib::new();
}
/// Core library loader
pub struct Loader {
    pub version: vk::Version,
    pub fp1_0: vk::LoaderFn1_0,
    pub fp1_1: vk::LoaderFn1_1,
}
impl Loader {
    pub fn new() -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| unsafe { lib.get_instance_proc_addr(None, name).map(|p| mem::transmute(p)) };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::LoaderFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        let (fp1_1, ok1_1) = vk::LoaderFn1_1::load(f);
        ok = ok && ok1_1;
        if ok {
            version = vk::Version::from_raw_parts(1, 1, 0);
        }
        Ok(Self { version, fp1_0, fp1_1 })
    }
    pub unsafe fn create_instance(
        &self,
        p_create_info: &vk::InstanceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> result::Result<Instance, LoaderError> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_instance)(p_create_info, p_allocator.map_or(ptr::null(), |r| r), &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res.map_err(|e| LoaderError::Vulkan(e)).and_then(|r| Instance::load(r))
    }
    pub unsafe fn enumerate_instance_extension_properties_to_vec(
        &self,
        p_layer_name: &CStr,
    ) -> Result<Vec<vk::ExtensionProperties>> {
        let mut len = mem::uninitialized();
        let len_err =
            (self.fp1_0.enumerate_instance_extension_properties)(p_layer_name.as_ptr(), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err =
            (self.fp1_0.enumerate_instance_extension_properties)(p_layer_name.as_ptr(), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn enumerate_instance_layer_properties_to_vec(&self) -> Result<Vec<vk::LayerProperties>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.enumerate_instance_layer_properties)(&mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.enumerate_instance_layer_properties)(&mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn enumerate_instance_version(&self) -> Result<vk::Version> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_1.enumerate_instance_version)(&mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Core instance loader
pub struct Instance {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::InstanceFn1_0,
    pub fp1_1: vk::InstanceFn1_1,
}
impl Instance {
    unsafe fn load(instance: vk::Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::InstanceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        let (fp1_1, ok1_1) = vk::InstanceFn1_1::load(f);
        ok = ok && ok1_1;
        if ok {
            version = vk::Version::from_raw_parts(1, 1, 0);
        }
        Ok(Self {
            version,
            handle: instance,
            fp1_0,
            fp1_1,
        })
    }
    pub unsafe fn destroy_instance(&self, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp1_0.destroy_instance)(Some(self.handle), p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn enumerate_physical_devices_to_vec(&self) -> Result<Vec<vk::PhysicalDevice>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.enumerate_physical_devices)(Some(self.handle), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.enumerate_physical_devices)(Some(self.handle), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceFeatures {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_physical_device_features)(Some(physical_device), &mut res);
        res
    }
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
    ) -> vk::FormatProperties {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_physical_device_format_properties)(Some(physical_device), format, &mut res);
        res
    }
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        ty: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
    ) -> Result<vk::ImageFormatProperties> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_physical_device_image_format_properties)(
            Some(physical_device),
            format,
            ty,
            tiling,
            usage,
            flags,
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_physical_device_properties)(Some(physical_device), &mut res);
        res
    }
    pub unsafe fn get_physical_device_queue_family_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties> {
        let mut len = mem::uninitialized();
        (self.fp1_0.get_physical_device_queue_family_properties)(Some(physical_device), &mut len, ptr::null_mut());
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_0.get_physical_device_queue_family_properties)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_physical_device_memory_properties)(Some(physical_device), &mut res);
        res
    }
    pub unsafe fn get_device_proc_addr(&self, device: vk::Device, p_name: &CStr) -> Option<vk::FnVoidFunction> {
        let res = (self.fp1_0.get_device_proc_addr)(Some(device), p_name.as_ptr());
        res
    }
    pub unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        p_create_info: &vk::DeviceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> result::Result<Device, LoaderError> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_device)(
            Some(physical_device),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res.map_err(|e| LoaderError::Vulkan(e))
            .and_then(|r| Device::load(&self, r))
    }
    pub unsafe fn enumerate_device_extension_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_layer_name: &CStr,
    ) -> Result<Vec<vk::ExtensionProperties>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.enumerate_device_extension_properties)(
            Some(physical_device),
            p_layer_name.as_ptr(),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.enumerate_device_extension_properties)(
            Some(physical_device),
            p_layer_name.as_ptr(),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn enumerate_device_layer_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::LayerProperties>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.enumerate_device_layer_properties)(Some(physical_device), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.enumerate_device_layer_properties)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        ty: vk::ImageType,
        samples: vk::SampleCountFlags,
        usage: vk::ImageUsageFlags,
        tiling: vk::ImageTiling,
    ) -> Vec<vk::SparseImageFormatProperties> {
        let mut len = mem::uninitialized();
        (self.fp1_0.get_physical_device_sparse_image_format_properties)(
            Some(physical_device),
            format,
            ty,
            samples,
            usage,
            tiling,
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_0.get_physical_device_sparse_image_format_properties)(
            Some(physical_device),
            format,
            ty,
            samples,
            usage,
            tiling,
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn enumerate_physical_device_groups_to_vec(&self) -> Result<Vec<vk::PhysicalDeviceGroupProperties>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_1.enumerate_physical_device_groups)(Some(self.handle), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_1.enumerate_physical_device_groups)(Some(self.handle), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_features: &mut vk::PhysicalDeviceFeatures2,
    ) {
        (self.fp1_1.get_physical_device_features2)(Some(physical_device), p_features);
    }
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_properties: &mut vk::PhysicalDeviceProperties2,
    ) {
        (self.fp1_1.get_physical_device_properties2)(Some(physical_device), p_properties);
    }
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        p_format_properties: &mut vk::FormatProperties2,
    ) {
        (self.fp1_1.get_physical_device_format_properties2)(Some(physical_device), format, p_format_properties);
    }
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_image_format_info: &vk::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: &mut vk::ImageFormatProperties2,
    ) -> Result<()> {
        let err = (self.fp1_1.get_physical_device_image_format_properties2)(
            Some(physical_device),
            p_image_format_info,
            p_image_format_properties,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_queue_family_properties2_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties2> {
        let mut len = mem::uninitialized();
        (self.fp1_1.get_physical_device_queue_family_properties2)(Some(physical_device), &mut len, ptr::null_mut());
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_1.get_physical_device_queue_family_properties2)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_memory_properties: &mut vk::PhysicalDeviceMemoryProperties2,
    ) {
        (self.fp1_1.get_physical_device_memory_properties2)(Some(physical_device), p_memory_properties);
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
    ) -> Vec<vk::SparseImageFormatProperties2> {
        let mut len = mem::uninitialized();
        (self.fp1_1.get_physical_device_sparse_image_format_properties2)(
            Some(physical_device),
            p_format_info,
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_1.get_physical_device_sparse_image_format_properties2)(
            Some(physical_device),
            p_format_info,
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_buffer_info: &vk::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: &mut vk::ExternalBufferProperties,
    ) {
        (self.fp1_1.get_physical_device_external_buffer_properties)(
            Some(physical_device),
            p_external_buffer_info,
            p_external_buffer_properties,
        );
    }
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_fence_info: &vk::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: &mut vk::ExternalFenceProperties,
    ) {
        (self.fp1_1.get_physical_device_external_fence_properties)(
            Some(physical_device),
            p_external_fence_info,
            p_external_fence_properties,
        );
    }
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: &mut vk::ExternalSemaphoreProperties,
    ) {
        (self.fp1_1.get_physical_device_external_semaphore_properties)(
            Some(physical_device),
            p_external_semaphore_info,
            p_external_semaphore_properties,
        );
    }
}
/// Core device loader
pub struct Device {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::DeviceFn1_0,
    pub fp1_1: vk::DeviceFn1_1,
}
impl Device {
    unsafe fn load(instance: &Instance, device: vk::Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| instance.get_device_proc_addr(device, name).map(|p| mem::transmute(p));
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::DeviceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        let (fp1_1, ok1_1) = vk::DeviceFn1_1::load(f);
        ok = ok && ok1_1;
        if ok {
            version = vk::Version::from_raw_parts(1, 1, 0);
        }
        Ok(Self {
            version,
            handle: device,
            fp1_0,
            fp1_1,
        })
    }
    pub unsafe fn destroy_device(&self, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp1_0.destroy_device)(Some(self.handle), p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> vk::Queue {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_device_queue)(Some(self.handle), queue_family_index, queue_index, &mut res);
        res
    }
    pub unsafe fn queue_submit(
        &self,
        queue: vk::Queue,
        p_submits: &[vk::SubmitInfo],
        fence: Option<vk::Fence>,
    ) -> Result<()> {
        let submit_count = p_submits.len() as u32;
        let err = (self.fp1_0.queue_submit)(Some(queue), submit_count, p_submits.as_ptr(), fence);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn queue_wait_idle(&self, queue: vk::Queue) -> Result<()> {
        let err = (self.fp1_0.queue_wait_idle)(Some(queue));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn device_wait_idle(&self) -> Result<()> {
        let err = (self.fp1_0.device_wait_idle)(Some(self.handle));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn allocate_memory(
        &self,
        p_allocate_info: &vk::MemoryAllocateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DeviceMemory> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.allocate_memory)(
            Some(self.handle),
            p_allocate_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn free_memory(&self, memory: Option<vk::DeviceMemory>, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp1_0.free_memory)(Some(self.handle), memory, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn map_memory(
        &self,
        memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        flags: vk::MemoryMapFlags,
    ) -> Result<*mut c_void> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.map_memory)(Some(self.handle), Some(memory), offset, size, flags, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn unmap_memory(&self, memory: vk::DeviceMemory) {
        (self.fp1_0.unmap_memory)(Some(self.handle), Some(memory));
    }
    pub unsafe fn flush_mapped_memory_ranges(&self, p_memory_ranges: &[vk::MappedMemoryRange]) -> Result<()> {
        let memory_range_count = p_memory_ranges.len() as u32;
        let err =
            (self.fp1_0.flush_mapped_memory_ranges)(Some(self.handle), memory_range_count, p_memory_ranges.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn invalidate_mapped_memory_ranges(&self, p_memory_ranges: &[vk::MappedMemoryRange]) -> Result<()> {
        let memory_range_count = p_memory_ranges.len() as u32;
        let err = (self.fp1_0.invalidate_mapped_memory_ranges)(
            Some(self.handle),
            memory_range_count,
            p_memory_ranges.as_ptr(),
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_device_memory_commitment(&self, memory: vk::DeviceMemory) -> vk::DeviceSize {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_device_memory_commitment)(Some(self.handle), Some(memory), &mut res);
        res
    }
    pub unsafe fn bind_buffer_memory(
        &self,
        buffer: vk::Buffer,
        memory: vk::DeviceMemory,
        memory_offset: vk::DeviceSize,
    ) -> Result<()> {
        let err = (self.fp1_0.bind_buffer_memory)(Some(self.handle), Some(buffer), Some(memory), memory_offset);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn bind_image_memory(
        &self,
        image: vk::Image,
        memory: vk::DeviceMemory,
        memory_offset: vk::DeviceSize,
    ) -> Result<()> {
        let err = (self.fp1_0.bind_image_memory)(Some(self.handle), Some(image), Some(memory), memory_offset);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_buffer_memory_requirements(&self, buffer: vk::Buffer) -> vk::MemoryRequirements {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_buffer_memory_requirements)(Some(self.handle), Some(buffer), &mut res);
        res
    }
    pub unsafe fn get_image_memory_requirements(&self, image: vk::Image) -> vk::MemoryRequirements {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_image_memory_requirements)(Some(self.handle), Some(image), &mut res);
        res
    }
    pub unsafe fn get_image_sparse_memory_requirements_to_vec(
        &self,
        image: vk::Image,
    ) -> Vec<vk::SparseImageMemoryRequirements> {
        let mut len = mem::uninitialized();
        (self.fp1_0.get_image_sparse_memory_requirements)(Some(self.handle), Some(image), &mut len, ptr::null_mut());
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_0.get_image_sparse_memory_requirements)(Some(self.handle), Some(image), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: vk::Queue,
        p_bind_info: &[vk::BindSparseInfo],
        fence: Option<vk::Fence>,
    ) -> Result<()> {
        let bind_info_count = p_bind_info.len() as u32;
        let err = (self.fp1_0.queue_bind_sparse)(Some(queue), bind_info_count, p_bind_info.as_ptr(), fence);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn create_fence(
        &self,
        p_create_info: &vk::FenceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Fence> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_fence)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_fence(&self, fence: Option<vk::Fence>, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp1_0.destroy_fence)(Some(self.handle), fence, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn reset_fences(&self, p_fences: &[vk::Fence]) -> Result<()> {
        let fence_count = p_fences.len() as u32;
        let err = (self.fp1_0.reset_fences)(Some(self.handle), fence_count, p_fences.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_fence_status(&self, fence: vk::Fence) -> Result<vk::Result> {
        let err = (self.fp1_0.get_fence_status)(Some(self.handle), Some(fence));
        let res = match err {
            vk::Result::SUCCESS | vk::Result::NOT_READY => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn wait_for_fences(&self, p_fences: &[vk::Fence], wait_all: bool, timeout: u64) -> Result<vk::Result> {
        let fence_count = p_fences.len() as u32;
        let err = (self.fp1_0.wait_for_fences)(
            Some(self.handle),
            fence_count,
            p_fences.as_ptr(),
            if wait_all { vk::TRUE } else { vk::FALSE },
            timeout,
        );
        let res = match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn create_semaphore(
        &self,
        p_create_info: &vk::SemaphoreCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Semaphore> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_semaphore)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_semaphore(
        &self,
        semaphore: Option<vk::Semaphore>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_semaphore)(Some(self.handle), semaphore, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_event(
        &self,
        p_create_info: &vk::EventCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Event> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_event)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_event(&self, event: Option<vk::Event>, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp1_0.destroy_event)(Some(self.handle), event, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_event_status(&self, event: vk::Event) -> Result<vk::Result> {
        let err = (self.fp1_0.get_event_status)(Some(self.handle), Some(event));
        let res = match err {
            vk::Result::EVENT_SET | vk::Result::EVENT_RESET => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn set_event(&self, event: vk::Event) -> Result<()> {
        let err = (self.fp1_0.set_event)(Some(self.handle), Some(event));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn reset_event(&self, event: vk::Event) -> Result<()> {
        let err = (self.fp1_0.reset_event)(Some(self.handle), Some(event));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn create_query_pool(
        &self,
        p_create_info: &vk::QueryPoolCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::QueryPool> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_query_pool)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_query_pool(
        &self,
        query_pool: Option<vk::QueryPool>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_query_pool)(Some(self.handle), query_pool, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_query_pool_results(
        &self,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: vk::DeviceSize,
        flags: vk::QueryResultFlags,
    ) -> Result<vk::Result> {
        let err = (self.fp1_0.get_query_pool_results)(
            Some(self.handle),
            Some(query_pool),
            first_query,
            query_count,
            data_size,
            p_data,
            stride,
            flags,
        );
        let res = match err {
            vk::Result::SUCCESS | vk::Result::NOT_READY => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn create_buffer(
        &self,
        p_create_info: &vk::BufferCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Buffer> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_buffer)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_buffer(&self, buffer: Option<vk::Buffer>, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp1_0.destroy_buffer)(Some(self.handle), buffer, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_buffer_view(
        &self,
        p_create_info: &vk::BufferViewCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::BufferView> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_buffer_view)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_buffer_view(
        &self,
        buffer_view: Option<vk::BufferView>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_buffer_view)(Some(self.handle), buffer_view, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_image(
        &self,
        p_create_info: &vk::ImageCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Image> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_image)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_image(&self, image: Option<vk::Image>, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp1_0.destroy_image)(Some(self.handle), image, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_image_subresource_layout(
        &self,
        image: vk::Image,
        p_subresource: &vk::ImageSubresource,
    ) -> vk::SubresourceLayout {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_image_subresource_layout)(Some(self.handle), Some(image), p_subresource, &mut res);
        res
    }
    pub unsafe fn create_image_view(
        &self,
        p_create_info: &vk::ImageViewCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ImageView> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_image_view)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_image_view(
        &self,
        image_view: Option<vk::ImageView>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_image_view)(Some(self.handle), image_view, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_shader_module(
        &self,
        p_create_info: &vk::ShaderModuleCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ShaderModule> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_shader_module)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_shader_module(
        &self,
        shader_module: Option<vk::ShaderModule>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_shader_module)(Some(self.handle), shader_module, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_pipeline_cache(
        &self,
        p_create_info: &vk::PipelineCacheCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::PipelineCache> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_pipeline_cache)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_pipeline_cache(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_pipeline_cache)(
            Some(self.handle),
            pipeline_cache,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn get_pipeline_cache_data(
        &self,
        pipeline_cache: vk::PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result<vk::Result> {
        let err = (self.fp1_0.get_pipeline_cache_data)(Some(self.handle), Some(pipeline_cache), p_data_size, p_data);
        let res = match err {
            vk::Result::SUCCESS | vk::Result::INCOMPLETE => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn merge_pipeline_caches(
        &self,
        dst_cache: vk::PipelineCache,
        p_src_caches: &[vk::PipelineCache],
    ) -> Result<()> {
        let src_cache_count = p_src_caches.len() as u32;
        let err = (self.fp1_0.merge_pipeline_caches)(
            Some(self.handle),
            Some(dst_cache),
            src_cache_count,
            p_src_caches.as_ptr(),
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn create_graphics_pipelines(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::GraphicsPipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_pipelines: *mut vk::Pipeline,
    ) -> Result<()> {
        let create_info_count = p_create_infos.len() as u32;
        let v_err = (self.fp1_0.create_graphics_pipelines)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            p_pipelines,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_graphics_pipelines_to_vec(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::GraphicsPipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::Pipeline>> {
        let create_info_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(create_info_count as usize);
        v.set_len(create_info_count as usize);
        let v_err = (self.fp1_0.create_graphics_pipelines)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_graphics_pipelines_array<A: Array<Item = vk::Pipeline>>(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::GraphicsPipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<A> {
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, A::len() as u32);
        let mut v: A = mem::uninitialized();
        let v_err = (self.fp1_0.create_graphics_pipelines)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_graphics_pipelines_single(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::GraphicsPipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, 1);
        let mut v = mem::uninitialized();
        let v_err = (self.fp1_0.create_graphics_pipelines)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            &mut v,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_compute_pipelines(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_pipelines: *mut vk::Pipeline,
    ) -> Result<()> {
        let create_info_count = p_create_infos.len() as u32;
        let v_err = (self.fp1_0.create_compute_pipelines)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            p_pipelines,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_compute_pipelines_to_vec(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::Pipeline>> {
        let create_info_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(create_info_count as usize);
        v.set_len(create_info_count as usize);
        let v_err = (self.fp1_0.create_compute_pipelines)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_compute_pipelines_array<A: Array<Item = vk::Pipeline>>(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<A> {
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, A::len() as u32);
        let mut v: A = mem::uninitialized();
        let v_err = (self.fp1_0.create_compute_pipelines)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_compute_pipelines_single(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, 1);
        let mut v = mem::uninitialized();
        let v_err = (self.fp1_0.create_compute_pipelines)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            &mut v,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn destroy_pipeline(
        &self,
        pipeline: Option<vk::Pipeline>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_pipeline)(Some(self.handle), pipeline, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_pipeline_layout(
        &self,
        p_create_info: &vk::PipelineLayoutCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::PipelineLayout> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_pipeline_layout)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_pipeline_layout(
        &self,
        pipeline_layout: Option<vk::PipelineLayout>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_pipeline_layout)(
            Some(self.handle),
            pipeline_layout,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn create_sampler(
        &self,
        p_create_info: &vk::SamplerCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Sampler> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_sampler)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_sampler(&self, sampler: Option<vk::Sampler>, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp1_0.destroy_sampler)(Some(self.handle), sampler, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_descriptor_set_layout(
        &self,
        p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorSetLayout> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_descriptor_set_layout)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: Option<vk::DescriptorSetLayout>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_descriptor_set_layout)(
            Some(self.handle),
            descriptor_set_layout,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn create_descriptor_pool(
        &self,
        p_create_info: &vk::DescriptorPoolCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorPool> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_descriptor_pool)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_descriptor_pool(
        &self,
        descriptor_pool: Option<vk::DescriptorPool>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_descriptor_pool)(
            Some(self.handle),
            descriptor_pool,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn reset_descriptor_pool(
        &self,
        descriptor_pool: vk::DescriptorPool,
        flags: vk::DescriptorPoolResetFlags,
    ) -> Result<()> {
        let err = (self.fp1_0.reset_descriptor_pool)(Some(self.handle), Some(descriptor_pool), flags);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn allocate_descriptor_sets(
        &self,
        p_allocate_info: &vk::DescriptorSetAllocateInfo,
        p_descriptor_sets: *mut vk::DescriptorSet,
    ) -> Result<()> {
        let v_err = (self.fp1_0.allocate_descriptor_sets)(Some(self.handle), p_allocate_info, p_descriptor_sets);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn allocate_descriptor_sets_to_vec(
        &self,
        p_allocate_info: &vk::DescriptorSetAllocateInfo,
    ) -> Result<Vec<vk::DescriptorSet>> {
        let mut v = Vec::with_capacity(p_allocate_info.descriptor_set_count as usize);
        v.set_len(p_allocate_info.descriptor_set_count as usize);
        let v_err = (self.fp1_0.allocate_descriptor_sets)(Some(self.handle), p_allocate_info, v.as_mut_ptr());
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn allocate_descriptor_sets_array<A: Array<Item = vk::DescriptorSet>>(
        &self,
        p_allocate_info: &vk::DescriptorSetAllocateInfo,
    ) -> Result<A> {
        assert_eq!(p_allocate_info.descriptor_set_count, A::len() as u32);
        let mut v: A = mem::uninitialized();
        let v_err = (self.fp1_0.allocate_descriptor_sets)(Some(self.handle), p_allocate_info, v.as_mut_ptr());
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn allocate_descriptor_sets_single(
        &self,
        p_allocate_info: &vk::DescriptorSetAllocateInfo,
    ) -> Result<vk::DescriptorSet> {
        assert_eq!(p_allocate_info.descriptor_set_count, 1);
        let mut v = mem::uninitialized();
        let v_err = (self.fp1_0.allocate_descriptor_sets)(Some(self.handle), p_allocate_info, &mut v);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn free_descriptor_sets(
        &self,
        descriptor_pool: vk::DescriptorPool,
        p_descriptor_sets: &[vk::DescriptorSet],
    ) -> Result<()> {
        let descriptor_set_count = p_descriptor_sets.len() as u32;
        let err = (self.fp1_0.free_descriptor_sets)(
            Some(self.handle),
            Some(descriptor_pool),
            descriptor_set_count,
            p_descriptor_sets.as_ptr(),
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn update_descriptor_sets(
        &self,
        p_descriptor_writes: &[vk::WriteDescriptorSet],
        p_descriptor_copies: &[vk::CopyDescriptorSet],
    ) {
        let descriptor_write_count = p_descriptor_writes.len() as u32;
        let descriptor_copy_count = p_descriptor_copies.len() as u32;
        (self.fp1_0.update_descriptor_sets)(
            Some(self.handle),
            descriptor_write_count,
            p_descriptor_writes.as_ptr(),
            descriptor_copy_count,
            p_descriptor_copies.as_ptr(),
        );
    }
    pub unsafe fn create_framebuffer(
        &self,
        p_create_info: &vk::FramebufferCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Framebuffer> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_framebuffer)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_framebuffer(
        &self,
        framebuffer: Option<vk::Framebuffer>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_framebuffer)(Some(self.handle), framebuffer, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_render_pass(
        &self,
        p_create_info: &vk::RenderPassCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::RenderPass> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_render_pass)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_render_pass(
        &self,
        render_pass: Option<vk::RenderPass>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_render_pass)(Some(self.handle), render_pass, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_render_area_granularity(&self, render_pass: vk::RenderPass) -> vk::Extent2D {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_render_area_granularity)(Some(self.handle), Some(render_pass), &mut res);
        res
    }
    pub unsafe fn create_command_pool(
        &self,
        p_create_info: &vk::CommandPoolCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::CommandPool> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_command_pool)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_command_pool(
        &self,
        command_pool: Option<vk::CommandPool>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_command_pool)(Some(self.handle), command_pool, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn reset_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolResetFlags,
    ) -> Result<()> {
        let err = (self.fp1_0.reset_command_pool)(Some(self.handle), Some(command_pool), flags);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn allocate_command_buffers(
        &self,
        p_allocate_info: &vk::CommandBufferAllocateInfo,
        p_command_buffers: *mut vk::CommandBuffer,
    ) -> Result<()> {
        let v_err = (self.fp1_0.allocate_command_buffers)(Some(self.handle), p_allocate_info, p_command_buffers);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn allocate_command_buffers_to_vec(
        &self,
        p_allocate_info: &vk::CommandBufferAllocateInfo,
    ) -> Result<Vec<vk::CommandBuffer>> {
        let mut v = Vec::with_capacity(p_allocate_info.command_buffer_count as usize);
        v.set_len(p_allocate_info.command_buffer_count as usize);
        let v_err = (self.fp1_0.allocate_command_buffers)(Some(self.handle), p_allocate_info, v.as_mut_ptr());
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn allocate_command_buffers_array<A: Array<Item = vk::CommandBuffer>>(
        &self,
        p_allocate_info: &vk::CommandBufferAllocateInfo,
    ) -> Result<A> {
        assert_eq!(p_allocate_info.command_buffer_count, A::len() as u32);
        let mut v: A = mem::uninitialized();
        let v_err = (self.fp1_0.allocate_command_buffers)(Some(self.handle), p_allocate_info, v.as_mut_ptr());
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn allocate_command_buffers_single(
        &self,
        p_allocate_info: &vk::CommandBufferAllocateInfo,
    ) -> Result<vk::CommandBuffer> {
        assert_eq!(p_allocate_info.command_buffer_count, 1);
        let mut v = mem::uninitialized();
        let v_err = (self.fp1_0.allocate_command_buffers)(Some(self.handle), p_allocate_info, &mut v);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn free_command_buffers(&self, command_pool: vk::CommandPool, p_command_buffers: &[vk::CommandBuffer]) {
        let command_buffer_count = p_command_buffers.len() as u32;
        (self.fp1_0.free_command_buffers)(
            Some(self.handle),
            Some(command_pool),
            command_buffer_count,
            p_command_buffers.as_ptr(),
        );
    }
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        p_begin_info: &vk::CommandBufferBeginInfo,
    ) -> Result<()> {
        let err = (self.fp1_0.begin_command_buffer)(Some(command_buffer), p_begin_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> Result<()> {
        let err = (self.fp1_0.end_command_buffer)(Some(command_buffer));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        flags: vk::CommandBufferResetFlags,
    ) -> Result<()> {
        let err = (self.fp1_0.reset_command_buffer)(Some(command_buffer), flags);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) {
        (self.fp1_0.cmd_bind_pipeline)(Some(command_buffer), pipeline_bind_point, Some(pipeline));
    }
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        p_viewports: &[vk::Viewport],
    ) {
        let viewport_count = p_viewports.len() as u32;
        (self.fp1_0.cmd_set_viewport)(
            Some(command_buffer),
            first_viewport,
            viewport_count,
            p_viewports.as_ptr(),
        );
    }
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: vk::CommandBuffer,
        first_scissor: u32,
        p_scissors: &[vk::Rect2D],
    ) {
        let scissor_count = p_scissors.len() as u32;
        (self.fp1_0.cmd_set_scissor)(Some(command_buffer), first_scissor, scissor_count, p_scissors.as_ptr());
    }
    pub unsafe fn cmd_set_line_width(&self, command_buffer: vk::CommandBuffer, line_width: f32) {
        (self.fp1_0.cmd_set_line_width)(Some(command_buffer), line_width);
    }
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        (self.fp1_0.cmd_set_depth_bias)(
            Some(command_buffer),
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
        );
    }
    pub unsafe fn cmd_set_blend_constants(&self, command_buffer: vk::CommandBuffer, blend_constants: [f32; 4]) {
        (self.fp1_0.cmd_set_blend_constants)(Some(command_buffer), blend_constants);
    }
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: vk::CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        (self.fp1_0.cmd_set_depth_bounds)(Some(command_buffer), min_depth_bounds, max_depth_bounds);
    }
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        compare_mask: u32,
    ) {
        (self.fp1_0.cmd_set_stencil_compare_mask)(Some(command_buffer), face_mask, compare_mask);
    }
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        write_mask: u32,
    ) {
        (self.fp1_0.cmd_set_stencil_write_mask)(Some(command_buffer), face_mask, write_mask);
    }
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        reference: u32,
    ) {
        (self.fp1_0.cmd_set_stencil_reference)(Some(command_buffer), face_mask, reference);
    }
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        first_set: u32,
        p_descriptor_sets: &[vk::DescriptorSet],
        p_dynamic_offsets: &[u32],
    ) {
        let descriptor_set_count = p_descriptor_sets.len() as u32;
        let dynamic_offset_count = p_dynamic_offsets.len() as u32;
        (self.fp1_0.cmd_bind_descriptor_sets)(
            Some(command_buffer),
            pipeline_bind_point,
            Some(layout),
            first_set,
            descriptor_set_count,
            p_descriptor_sets.as_ptr(),
            dynamic_offset_count,
            p_dynamic_offsets.as_ptr(),
        );
    }
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        index_type: vk::IndexType,
    ) {
        (self.fp1_0.cmd_bind_index_buffer)(Some(command_buffer), Some(buffer), offset, index_type);
    }
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        p_buffers: &[vk::Buffer],
        p_offsets: &[vk::DeviceSize],
    ) {
        let binding_count = p_buffers.len() as u32;
        assert_eq!(binding_count, p_offsets.len() as u32);
        (self.fp1_0.cmd_bind_vertex_buffers)(
            Some(command_buffer),
            first_binding,
            binding_count,
            p_buffers.as_ptr(),
            p_offsets.as_ptr(),
        );
    }
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        (self.fp1_0.cmd_draw)(
            Some(command_buffer),
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        );
    }
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: vk::CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        (self.fp1_0.cmd_draw_indexed)(
            Some(command_buffer),
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        );
    }
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self.fp1_0.cmd_draw_indirect)(Some(command_buffer), Some(buffer), offset, draw_count, stride);
    }
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self.fp1_0.cmd_draw_indexed_indirect)(Some(command_buffer), Some(buffer), offset, draw_count, stride);
    }
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: vk::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self.fp1_0.cmd_dispatch)(Some(command_buffer), group_count_x, group_count_y, group_count_z);
    }
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
    ) {
        (self.fp1_0.cmd_dispatch_indirect)(Some(command_buffer), Some(buffer), offset);
    }
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_buffer: vk::Buffer,
        p_regions: &[vk::BufferCopy],
    ) {
        let region_count = p_regions.len() as u32;
        (self.fp1_0.cmd_copy_buffer)(
            Some(command_buffer),
            Some(src_buffer),
            Some(dst_buffer),
            region_count,
            p_regions.as_ptr(),
        );
    }
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        p_regions: &[vk::ImageCopy],
    ) {
        let region_count = p_regions.len() as u32;
        (self.fp1_0.cmd_copy_image)(
            Some(command_buffer),
            Some(src_image),
            src_image_layout,
            Some(dst_image),
            dst_image_layout,
            region_count,
            p_regions.as_ptr(),
        );
    }
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        p_regions: &[vk::ImageBlit],
        filter: vk::Filter,
    ) {
        let region_count = p_regions.len() as u32;
        (self.fp1_0.cmd_blit_image)(
            Some(command_buffer),
            Some(src_image),
            src_image_layout,
            Some(dst_image),
            dst_image_layout,
            region_count,
            p_regions.as_ptr(),
            filter,
        );
    }
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        p_regions: &[vk::BufferImageCopy],
    ) {
        let region_count = p_regions.len() as u32;
        (self.fp1_0.cmd_copy_buffer_to_image)(
            Some(command_buffer),
            Some(src_buffer),
            Some(dst_image),
            dst_image_layout,
            region_count,
            p_regions.as_ptr(),
        );
    }
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_buffer: vk::Buffer,
        p_regions: &[vk::BufferImageCopy],
    ) {
        let region_count = p_regions.len() as u32;
        (self.fp1_0.cmd_copy_image_to_buffer)(
            Some(command_buffer),
            Some(src_image),
            src_image_layout,
            Some(dst_buffer),
            region_count,
            p_regions.as_ptr(),
        );
    }
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        data_size: vk::DeviceSize,
        p_data: *const c_void,
    ) {
        (self.fp1_0.cmd_update_buffer)(Some(command_buffer), Some(dst_buffer), dst_offset, data_size, p_data);
    }
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: u32,
    ) {
        (self.fp1_0.cmd_fill_buffer)(Some(command_buffer), Some(dst_buffer), dst_offset, size, data);
    }
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: vk::ImageLayout,
        p_color: &vk::ClearColorValue,
        p_ranges: &[vk::ImageSubresourceRange],
    ) {
        let range_count = p_ranges.len() as u32;
        (self.fp1_0.cmd_clear_color_image)(
            Some(command_buffer),
            Some(image),
            image_layout,
            p_color,
            range_count,
            p_ranges.as_ptr(),
        );
    }
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: vk::CommandBuffer,
        image: vk::Image,
        image_layout: vk::ImageLayout,
        p_depth_stencil: &vk::ClearDepthStencilValue,
        p_ranges: &[vk::ImageSubresourceRange],
    ) {
        let range_count = p_ranges.len() as u32;
        (self.fp1_0.cmd_clear_depth_stencil_image)(
            Some(command_buffer),
            Some(image),
            image_layout,
            p_depth_stencil,
            range_count,
            p_ranges.as_ptr(),
        );
    }
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: vk::CommandBuffer,
        p_attachments: &[vk::ClearAttachment],
        p_rects: &[vk::ClearRect],
    ) {
        let attachment_count = p_attachments.len() as u32;
        let rect_count = p_rects.len() as u32;
        (self.fp1_0.cmd_clear_attachments)(
            Some(command_buffer),
            attachment_count,
            p_attachments.as_ptr(),
            rect_count,
            p_rects.as_ptr(),
        );
    }
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: vk::CommandBuffer,
        src_image: vk::Image,
        src_image_layout: vk::ImageLayout,
        dst_image: vk::Image,
        dst_image_layout: vk::ImageLayout,
        p_regions: &[vk::ImageResolve],
    ) {
        let region_count = p_regions.len() as u32;
        (self.fp1_0.cmd_resolve_image)(
            Some(command_buffer),
            Some(src_image),
            src_image_layout,
            Some(dst_image),
            dst_image_layout,
            region_count,
            p_regions.as_ptr(),
        );
    }
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags,
    ) {
        (self.fp1_0.cmd_set_event)(Some(command_buffer), Some(event), stage_mask);
    }
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags,
    ) {
        (self.fp1_0.cmd_reset_event)(Some(command_buffer), Some(event), stage_mask);
    }
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: vk::CommandBuffer,
        p_events: &[vk::Event],
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
        p_memory_barriers: &[vk::MemoryBarrier],
        p_buffer_memory_barriers: &[vk::BufferMemoryBarrier],
        p_image_memory_barriers: &[vk::ImageMemoryBarrier],
    ) {
        let event_count = p_events.len() as u32;
        let memory_barrier_count = p_memory_barriers.len() as u32;
        let buffer_memory_barrier_count = p_buffer_memory_barriers.len() as u32;
        let image_memory_barrier_count = p_image_memory_barriers.len() as u32;
        (self.fp1_0.cmd_wait_events)(
            Some(command_buffer),
            event_count,
            p_events.as_ptr(),
            src_stage_mask,
            dst_stage_mask,
            memory_barrier_count,
            p_memory_barriers.as_ptr(),
            buffer_memory_barrier_count,
            p_buffer_memory_barriers.as_ptr(),
            image_memory_barrier_count,
            p_image_memory_barriers.as_ptr(),
        );
    }
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: vk::CommandBuffer,
        src_stage_mask: vk::PipelineStageFlags,
        dst_stage_mask: vk::PipelineStageFlags,
        dependency_flags: vk::DependencyFlags,
        p_memory_barriers: &[vk::MemoryBarrier],
        p_buffer_memory_barriers: &[vk::BufferMemoryBarrier],
        p_image_memory_barriers: &[vk::ImageMemoryBarrier],
    ) {
        let memory_barrier_count = p_memory_barriers.len() as u32;
        let buffer_memory_barrier_count = p_buffer_memory_barriers.len() as u32;
        let image_memory_barrier_count = p_image_memory_barriers.len() as u32;
        (self.fp1_0.cmd_pipeline_barrier)(
            Some(command_buffer),
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barrier_count,
            p_memory_barriers.as_ptr(),
            buffer_memory_barrier_count,
            p_buffer_memory_barriers.as_ptr(),
            image_memory_barrier_count,
            p_image_memory_barriers.as_ptr(),
        );
    }
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        flags: vk::QueryControlFlags,
    ) {
        (self.fp1_0.cmd_begin_query)(Some(command_buffer), Some(query_pool), query, flags);
    }
    pub unsafe fn cmd_end_query(&self, command_buffer: vk::CommandBuffer, query_pool: vk::QueryPool, query: u32) {
        (self.fp1_0.cmd_end_query)(Some(command_buffer), Some(query_pool), query);
    }
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self.fp1_0.cmd_reset_query_pool)(Some(command_buffer), Some(query_pool), first_query, query_count);
    }
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        (self.fp1_0.cmd_write_timestamp)(Some(command_buffer), pipeline_stage, Some(query_pool), query);
    }
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        stride: vk::DeviceSize,
        flags: vk::QueryResultFlags,
    ) {
        (self.fp1_0.cmd_copy_query_pool_results)(
            Some(command_buffer),
            Some(query_pool),
            first_query,
            query_count,
            Some(dst_buffer),
            dst_offset,
            stride,
            flags,
        );
    }
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: vk::CommandBuffer,
        layout: vk::PipelineLayout,
        stage_flags: vk::ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const c_void,
    ) {
        (self.fp1_0.cmd_push_constants)(Some(command_buffer), Some(layout), stage_flags, offset, size, p_values);
    }
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: vk::CommandBuffer,
        p_render_pass_begin: &vk::RenderPassBeginInfo,
        contents: vk::SubpassContents,
    ) {
        (self.fp1_0.cmd_begin_render_pass)(Some(command_buffer), p_render_pass_begin, contents);
    }
    pub unsafe fn cmd_next_subpass(&self, command_buffer: vk::CommandBuffer, contents: vk::SubpassContents) {
        (self.fp1_0.cmd_next_subpass)(Some(command_buffer), contents);
    }
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: vk::CommandBuffer) {
        (self.fp1_0.cmd_end_render_pass)(Some(command_buffer));
    }
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: vk::CommandBuffer,
        p_command_buffers: &[vk::CommandBuffer],
    ) {
        let command_buffer_count = p_command_buffers.len() as u32;
        (self.fp1_0.cmd_execute_commands)(Some(command_buffer), command_buffer_count, p_command_buffers.as_ptr());
    }
    pub unsafe fn bind_buffer_memory2(&self, p_bind_infos: &[vk::BindBufferMemoryInfo]) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp1_1.bind_buffer_memory2)(Some(self.handle), bind_info_count, p_bind_infos.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn bind_image_memory2(&self, p_bind_infos: &[vk::BindImageMemoryInfo]) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp1_1.bind_image_memory2)(Some(self.handle), bind_info_count, p_bind_infos.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> vk::PeerMemoryFeatureFlags {
        let mut res = mem::uninitialized();
        (self.fp1_1.get_device_group_peer_memory_features)(
            Some(self.handle),
            heap_index,
            local_device_index,
            remote_device_index,
            &mut res,
        );
        res
    }
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: vk::CommandBuffer, device_mask: u32) {
        (self.fp1_1.cmd_set_device_mask)(Some(command_buffer), device_mask);
    }
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: vk::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self.fp1_1.cmd_dispatch_base)(
            Some(command_buffer),
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }
    pub unsafe fn get_image_memory_requirements2(
        &self,
        p_info: &vk::ImageMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        (self.fp1_1.get_image_memory_requirements2)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        p_info: &vk::BufferMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        (self.fp1_1.get_buffer_memory_requirements2)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_image_sparse_memory_requirements2_to_vec(
        &self,
        p_info: &vk::ImageSparseMemoryRequirementsInfo2,
    ) -> Vec<vk::SparseImageMemoryRequirements2> {
        let mut len = mem::uninitialized();
        (self.fp1_1.get_image_sparse_memory_requirements2)(Some(self.handle), p_info, &mut len, ptr::null_mut());
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_1.get_image_sparse_memory_requirements2)(Some(self.handle), p_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn trim_command_pool(&self, command_pool: vk::CommandPool, flags: vk::CommandPoolTrimFlags) {
        (self.fp1_1.trim_command_pool)(Some(self.handle), Some(command_pool), flags);
    }
    pub unsafe fn get_device_queue2(&self, p_queue_info: &vk::DeviceQueueInfo2) -> vk::Queue {
        let mut res = mem::uninitialized();
        (self.fp1_1.get_device_queue2)(Some(self.handle), p_queue_info, &mut res);
        res
    }
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        p_create_info: &vk::SamplerYcbcrConversionCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SamplerYcbcrConversion> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_1.create_sampler_ycbcr_conversion)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: Option<vk::SamplerYcbcrConversion>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_1.destroy_sampler_ycbcr_conversion)(
            Some(self.handle),
            ycbcr_conversion,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn create_descriptor_update_template(
        &self,
        p_create_info: &vk::DescriptorUpdateTemplateCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorUpdateTemplate> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_1.create_descriptor_update_template)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: Option<vk::DescriptorUpdateTemplate>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_1.destroy_descriptor_update_template)(
            Some(self.handle),
            descriptor_update_template,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        (self.fp1_1.update_descriptor_set_with_template)(
            Some(self.handle),
            Some(descriptor_set),
            Some(descriptor_update_template),
            p_data,
        );
    }
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        p_support: &mut vk::DescriptorSetLayoutSupport,
    ) {
        (self.fp1_1.get_descriptor_set_layout_support)(Some(self.handle), p_create_info, p_support);
    }
}
/// Loader for the `VK_KHR_surface` instance extension
pub struct KhrSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrSurfaceFn1_0,
}
impl KhrSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_surface\0").unwrap()
    }
    pub unsafe fn destroy_surface_khr(
        &self,
        surface: Option<vk::SurfaceKHR>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_surface_khr)(Some(self.handle), surface, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        surface: vk::SurfaceKHR,
    ) -> Result<bool> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_physical_device_surface_support_khr)(
            Some(physical_device),
            queue_family_index,
            Some(surface),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res.map(|r| r != vk::FALSE)
    }
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<vk::SurfaceCapabilitiesKHR> {
        let mut res = mem::uninitialized();
        let err =
            (self.fp1_0.get_physical_device_surface_capabilities_khr)(Some(physical_device), Some(surface), &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_surface_formats_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Vec<vk::SurfaceFormatKHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_physical_device_surface_formats_khr)(
            Some(physical_device),
            Some(surface),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_physical_device_surface_formats_khr)(
            Some(physical_device),
            Some(surface),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_physical_device_surface_present_modes_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Vec<vk::PresentModeKHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_physical_device_surface_present_modes_khr)(
            Some(physical_device),
            Some(surface),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_physical_device_surface_present_modes_khr)(
            Some(physical_device),
            Some(surface),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
}
/// Loader for the `VK_KHR_swapchain` device extension
pub struct KhrSwapchain {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrSwapchainFn1_0,
    pub fp1_1: vk::KhrSwapchainFn1_1,
}
impl KhrSwapchain {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrSwapchainFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        let (fp1_1, ok1_1) = vk::KhrSwapchainFn1_1::load(f);
        ok = ok && ok1_1;
        if ok {
            version = vk::Version::from_raw_parts(1, 1, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
            fp1_1,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_swapchain\0").unwrap()
    }
    pub unsafe fn create_swapchain_khr(
        &self,
        p_create_info: &vk::SwapchainCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SwapchainKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_swapchain_khr)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_swapchain_khr(
        &self,
        swapchain: Option<vk::SwapchainKHR>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_swapchain_khr)(Some(self.handle), swapchain, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_swapchain_images_khr_to_vec(&self, swapchain: vk::SwapchainKHR) -> Result<Vec<vk::Image>> {
        let mut len = mem::uninitialized();
        let len_err =
            (self.fp1_0.get_swapchain_images_khr)(Some(self.handle), Some(swapchain), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_swapchain_images_khr)(Some(self.handle), Some(swapchain), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn acquire_next_image_khr(
        &self,
        swapchain: vk::SwapchainKHR,
        timeout: u64,
        semaphore: Option<vk::Semaphore>,
        fence: Option<vk::Fence>,
    ) -> Result<(vk::Result, u32)> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.acquire_next_image_khr)(
            Some(self.handle),
            Some(swapchain),
            timeout,
            semaphore,
            fence,
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT | vk::Result::NOT_READY | vk::Result::SUBOPTIMAL_KHR => {
                Ok((err, res))
            }
            _ => Err(err),
        };
        res
    }
    pub unsafe fn queue_present_khr(
        &self,
        queue: vk::Queue,
        p_present_info: &vk::PresentInfoKHR,
    ) -> Result<vk::Result> {
        let err = (self.fp1_0.queue_present_khr)(Some(queue), p_present_info);
        let res = match err {
            vk::Result::SUCCESS | vk::Result::SUBOPTIMAL_KHR => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        p_device_group_present_capabilities: &mut vk::DeviceGroupPresentCapabilitiesKHR,
    ) -> Result<()> {
        let err = (self.fp1_1.get_device_group_present_capabilities_khr)(
            Some(self.handle),
            p_device_group_present_capabilities,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        surface: vk::SurfaceKHR,
    ) -> Result<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_1.get_device_group_surface_present_modes_khr)(Some(self.handle), Some(surface), &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_present_rectangles_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Vec<vk::Rect2D>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_1.get_physical_device_present_rectangles_khr)(
            Some(physical_device),
            Some(surface),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_1.get_physical_device_present_rectangles_khr)(
            Some(physical_device),
            Some(surface),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn acquire_next_image2_khr(
        &self,
        p_acquire_info: &vk::AcquireNextImageInfoKHR,
    ) -> Result<(vk::Result, u32)> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_1.acquire_next_image2_khr)(Some(self.handle), p_acquire_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT | vk::Result::NOT_READY | vk::Result::SUBOPTIMAL_KHR => {
                Ok((err, res))
            }
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_display` instance extension
pub struct KhrDisplay {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrDisplayFn1_0,
}
impl KhrDisplay {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrDisplayFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_display\0").unwrap()
    }
    pub unsafe fn get_physical_device_display_properties_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayPropertiesKHR>> {
        let mut len = mem::uninitialized();
        let len_err =
            (self.fp1_0.get_physical_device_display_properties_khr)(Some(physical_device), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err =
            (self.fp1_0.get_physical_device_display_properties_khr)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_physical_device_display_plane_properties_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayPlanePropertiesKHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_physical_device_display_plane_properties_khr)(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_physical_device_display_plane_properties_khr)(
            Some(physical_device),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_display_plane_supported_displays_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        plane_index: u32,
    ) -> Result<Vec<vk::DisplayKHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_display_plane_supported_displays_khr)(
            Some(physical_device),
            plane_index,
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_display_plane_supported_displays_khr)(
            Some(physical_device),
            plane_index,
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_display_mode_properties_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> Result<Vec<vk::DisplayModePropertiesKHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_display_mode_properties_khr)(
            Some(physical_device),
            Some(display),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_display_mode_properties_khr)(
            Some(physical_device),
            Some(display),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_display_mode_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        p_create_info: &vk::DisplayModeCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DisplayModeKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_display_mode_khr)(
            Some(physical_device),
            Some(display),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        mode: vk::DisplayModeKHR,
        plane_index: u32,
    ) -> Result<vk::DisplayPlaneCapabilitiesKHR> {
        let mut res = mem::uninitialized();
        let err =
            (self.fp1_0.get_display_plane_capabilities_khr)(Some(physical_device), Some(mode), plane_index, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn create_display_plane_surface_khr(
        &self,
        p_create_info: &vk::DisplaySurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_display_plane_surface_khr)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_display_swapchain` device extension
pub struct KhrDisplaySwapchain {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrDisplaySwapchainFn1_0,
}
impl KhrDisplaySwapchain {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrDisplaySwapchainFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_display_swapchain\0").unwrap()
    }
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        p_create_infos: &[vk::SwapchainCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_swapchains: *mut vk::SwapchainKHR,
    ) -> Result<()> {
        let swapchain_count = p_create_infos.len() as u32;
        let v_err = (self.fp1_0.create_shared_swapchains_khr)(
            Some(self.handle),
            swapchain_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            p_swapchains,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_shared_swapchains_khr_to_vec(
        &self,
        p_create_infos: &[vk::SwapchainCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::SwapchainKHR>> {
        let swapchain_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(swapchain_count as usize);
        v.set_len(swapchain_count as usize);
        let v_err = (self.fp1_0.create_shared_swapchains_khr)(
            Some(self.handle),
            swapchain_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_shared_swapchains_khr_array<A: Array<Item = vk::SwapchainKHR>>(
        &self,
        p_create_infos: &[vk::SwapchainCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<A> {
        let swapchain_count = p_create_infos.len() as u32;
        assert_eq!(swapchain_count, A::len() as u32);
        let mut v: A = mem::uninitialized();
        let v_err = (self.fp1_0.create_shared_swapchains_khr)(
            Some(self.handle),
            swapchain_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_shared_swapchains_khr_single(
        &self,
        p_create_infos: &[vk::SwapchainCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SwapchainKHR> {
        let swapchain_count = p_create_infos.len() as u32;
        assert_eq!(swapchain_count, 1);
        let mut v = mem::uninitialized();
        let v_err = (self.fp1_0.create_shared_swapchains_khr)(
            Some(self.handle),
            swapchain_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            &mut v,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
}
/// Loader for the `VK_KHR_xlib_surface` instance extension
pub struct KhrXlibSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrXlibSurfaceFn1_0,
}
impl KhrXlibSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrXlibSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_xlib_surface\0").unwrap()
    }
    pub unsafe fn create_xlib_surface_khr(
        &self,
        p_create_info: &vk::XlibSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_xlib_surface_khr)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        dpy: &mut vk::Display,
        visual_id: vk::VisualID,
    ) -> vk::Bool32 {
        let res = (self.fp1_0.get_physical_device_xlib_presentation_support_khr)(
            Some(physical_device),
            queue_family_index,
            dpy,
            visual_id,
        );
        res
    }
}
/// Loader for the `VK_KHR_xcb_surface` instance extension
pub struct KhrXcbSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrXcbSurfaceFn1_0,
}
impl KhrXcbSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrXcbSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_xcb_surface\0").unwrap()
    }
    pub unsafe fn create_xcb_surface_khr(
        &self,
        p_create_info: &vk::XcbSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_xcb_surface_khr)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        connection: &mut vk::xcb_connection_t,
        visual_id: vk::xcb_visualid_t,
    ) -> vk::Bool32 {
        let res = (self.fp1_0.get_physical_device_xcb_presentation_support_khr)(
            Some(physical_device),
            queue_family_index,
            connection,
            visual_id,
        );
        res
    }
}
/// Loader for the `VK_KHR_wayland_surface` instance extension
pub struct KhrWaylandSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrWaylandSurfaceFn1_0,
}
impl KhrWaylandSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrWaylandSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_wayland_surface\0").unwrap()
    }
    pub unsafe fn create_wayland_surface_khr(
        &self,
        p_create_info: &vk::WaylandSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_wayland_surface_khr)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        display: &mut vk::wl_display,
    ) -> vk::Bool32 {
        let res = (self.fp1_0.get_physical_device_wayland_presentation_support_khr)(
            Some(physical_device),
            queue_family_index,
            display,
        );
        res
    }
}
/// Loader for the `VK_KHR_mir_surface` instance extension
pub struct KhrMirSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrMirSurfaceFn1_0,
}
impl KhrMirSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrMirSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_mir_surface\0").unwrap()
    }
    pub unsafe fn create_mir_surface_khr(
        &self,
        p_create_info: &vk::MirSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_mir_surface_khr)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_mir_presentation_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        connection: &mut vk::MirConnection,
    ) -> vk::Bool32 {
        let res = (self.fp1_0.get_physical_device_mir_presentation_support_khr)(
            Some(physical_device),
            queue_family_index,
            connection,
        );
        res
    }
}
/// Loader for the `VK_KHR_android_surface` instance extension
pub struct KhrAndroidSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrAndroidSurfaceFn1_0,
}
impl KhrAndroidSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrAndroidSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_android_surface\0").unwrap()
    }
    pub unsafe fn create_android_surface_khr(
        &self,
        p_create_info: &vk::AndroidSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_android_surface_khr)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_win32_surface` instance extension
pub struct KhrWin32Surface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrWin32SurfaceFn1_0,
}
impl KhrWin32Surface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrWin32SurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_win32_surface\0").unwrap()
    }
    pub unsafe fn create_win32_surface_khr(
        &self,
        p_create_info: &vk::Win32SurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_win32_surface_khr)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> vk::Bool32 {
        let res =
            (self.fp1_0.get_physical_device_win32_presentation_support_khr)(Some(physical_device), queue_family_index);
        res
    }
}
/// Loader for the `VK_EXT_debug_report` instance extension
pub struct ExtDebugReport {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::ExtDebugReportFn1_0,
}
impl ExtDebugReport {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtDebugReportFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_debug_report\0").unwrap()
    }
    pub unsafe fn create_debug_report_callback_ext(
        &self,
        p_create_info: &vk::DebugReportCallbackCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DebugReportCallbackEXT> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_debug_report_callback_ext)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        callback: vk::DebugReportCallbackEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_debug_report_callback_ext)(
            Some(self.handle),
            Some(callback),
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn debug_report_message_ext(
        &self,
        flags: vk::DebugReportFlagsEXT,
        object_type: vk::DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: &CStr,
        p_message: &CStr,
    ) {
        (self.fp1_0.debug_report_message_ext)(
            Some(self.handle),
            flags,
            object_type,
            object,
            location,
            message_code,
            p_layer_prefix.as_ptr(),
            p_message.as_ptr(),
        );
    }
}
/// Loader for the `VK_EXT_debug_marker` device extension
pub struct ExtDebugMarker {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtDebugMarkerFn1_0,
}
impl ExtDebugMarker {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtDebugMarkerFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_debug_marker\0").unwrap()
    }
    pub unsafe fn debug_marker_set_object_tag_ext(&self, p_tag_info: &vk::DebugMarkerObjectTagInfoEXT) -> Result<()> {
        let err = (self.fp1_0.debug_marker_set_object_tag_ext)(Some(self.handle), p_tag_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        p_name_info: &vk::DebugMarkerObjectNameInfoEXT,
    ) -> Result<()> {
        let err = (self.fp1_0.debug_marker_set_object_name_ext)(Some(self.handle), p_name_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_marker_info: &vk::DebugMarkerMarkerInfoEXT,
    ) {
        (self.fp1_0.cmd_debug_marker_begin_ext)(Some(command_buffer), p_marker_info);
    }
    pub unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: vk::CommandBuffer) {
        (self.fp1_0.cmd_debug_marker_end_ext)(Some(command_buffer));
    }
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_marker_info: &vk::DebugMarkerMarkerInfoEXT,
    ) {
        (self.fp1_0.cmd_debug_marker_insert_ext)(Some(command_buffer), p_marker_info);
    }
}
/// Loader for the `VK_EXT_transform_feedback` device extension
pub struct ExtTransformFeedback {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtTransformFeedbackFn1_0,
}
impl ExtTransformFeedback {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtTransformFeedbackFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_transform_feedback\0").unwrap()
    }
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_binding: u32,
        p_buffers: &[vk::Buffer],
        p_offsets: &[vk::DeviceSize],
        p_sizes: Option<&[vk::DeviceSize]>,
    ) {
        let binding_count = p_buffers.len() as u32;
        assert_eq!(binding_count, p_offsets.len() as u32);
        if let Some(s) = p_sizes {
            assert_eq!(binding_count, s.len() as u32);
        }
        (self.fp1_0.cmd_bind_transform_feedback_buffers_ext)(
            Some(command_buffer),
            first_binding,
            binding_count,
            p_buffers.as_ptr(),
            p_offsets.as_ptr(),
            p_sizes.map_or(ptr::null(), |r| r.as_ptr()),
        );
    }
    pub unsafe fn cmd_begin_transform_feedback_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: Option<&[vk::Buffer]>,
        p_counter_buffer_offsets: Option<&[vk::DeviceSize]>,
    ) {
        if let Some(s) = p_counter_buffers {
            assert_eq!(counter_buffer_count, s.len() as u32);
        }
        if let Some(s) = p_counter_buffer_offsets {
            assert_eq!(counter_buffer_count, s.len() as u32);
        }
        (self.fp1_0.cmd_begin_transform_feedback_ext)(
            Some(command_buffer),
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers.map_or(ptr::null(), |r| r.as_ptr()),
            p_counter_buffer_offsets.map_or(ptr::null(), |r| r.as_ptr()),
        );
    }
    pub unsafe fn cmd_end_transform_feedback_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: Option<&[vk::Buffer]>,
        p_counter_buffer_offsets: Option<&[vk::DeviceSize]>,
    ) {
        if let Some(s) = p_counter_buffers {
            assert_eq!(counter_buffer_count, s.len() as u32);
        }
        if let Some(s) = p_counter_buffer_offsets {
            assert_eq!(counter_buffer_count, s.len() as u32);
        }
        (self.fp1_0.cmd_end_transform_feedback_ext)(
            Some(command_buffer),
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers.map_or(ptr::null(), |r| r.as_ptr()),
            p_counter_buffer_offsets.map_or(ptr::null(), |r| r.as_ptr()),
        );
    }
    pub unsafe fn cmd_begin_query_indexed_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        flags: vk::QueryControlFlags,
        index: u32,
    ) {
        (self.fp1_0.cmd_begin_query_indexed_ext)(Some(command_buffer), Some(query_pool), query, flags, index);
    }
    pub unsafe fn cmd_end_query_indexed_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        index: u32,
    ) {
        (self.fp1_0.cmd_end_query_indexed_ext)(Some(command_buffer), Some(query_pool), query, index);
    }
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: vk::Buffer,
        counter_buffer_offset: vk::DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        (self.fp1_0.cmd_draw_indirect_byte_count_ext)(
            Some(command_buffer),
            instance_count,
            first_instance,
            Some(counter_buffer),
            counter_buffer_offset,
            counter_offset,
            vertex_stride,
        );
    }
}
/// Loader for the `VK_AMD_draw_indirect_count` device extension
pub struct AmdDrawIndirectCount {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::AmdDrawIndirectCountFn1_0,
}
impl AmdDrawIndirectCount {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::AmdDrawIndirectCountFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_AMD_draw_indirect_count\0").unwrap()
    }
    pub unsafe fn cmd_draw_indirect_count_amd(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.fp1_0.cmd_draw_indirect_count_amd)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.fp1_0.cmd_draw_indexed_indirect_count_amd)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
}
/// Loader for the `VK_AMD_shader_info` device extension
pub struct AmdShaderInfo {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::AmdShaderInfoFn1_0,
}
impl AmdShaderInfo {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::AmdShaderInfoFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_AMD_shader_info\0").unwrap()
    }
    pub unsafe fn get_shader_info_amd(
        &self,
        pipeline: vk::Pipeline,
        shader_stage: vk::ShaderStageFlags,
        info_type: vk::ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> Result<vk::Result> {
        let err = (self.fp1_0.get_shader_info_amd)(
            Some(self.handle),
            Some(pipeline),
            shader_stage,
            info_type,
            p_info_size,
            p_info,
        );
        let res = match err {
            vk::Result::SUCCESS | vk::Result::INCOMPLETE => Ok(err),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_NV_external_memory_capabilities` instance extension
pub struct NvExternalMemoryCapabilities {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::NvExternalMemoryCapabilitiesFn1_0,
}
impl NvExternalMemoryCapabilities {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NvExternalMemoryCapabilitiesFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_external_memory_capabilities\0").unwrap()
    }
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        ty: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
        external_handle_type: vk::ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<vk::ExternalImageFormatPropertiesNV> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_physical_device_external_image_format_properties_nv)(
            Some(physical_device),
            format,
            ty,
            tiling,
            usage,
            flags,
            external_handle_type,
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_NV_external_memory_win32` device extension
pub struct NvExternalMemoryWin32 {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::NvExternalMemoryWin32Fn1_0,
}
impl NvExternalMemoryWin32 {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NvExternalMemoryWin32Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_external_memory_win32\0").unwrap()
    }
    pub unsafe fn get_memory_win32_handle_nv(
        &self,
        memory: vk::DeviceMemory,
        handle_type: vk::ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<vk::HANDLE> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_memory_win32_handle_nv)(Some(self.handle), Some(memory), handle_type, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_get_physical_device_properties2` instance extension
pub struct KhrGetPhysicalDeviceProperties2 {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrGetPhysicalDeviceProperties2Fn1_0,
}
impl KhrGetPhysicalDeviceProperties2 {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrGetPhysicalDeviceProperties2Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_get_physical_device_properties2\0").unwrap()
    }
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_features: &mut vk::PhysicalDeviceFeatures2,
    ) {
        (self.fp1_0.get_physical_device_features2)(Some(physical_device), p_features);
    }
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_properties: &mut vk::PhysicalDeviceProperties2,
    ) {
        (self.fp1_0.get_physical_device_properties2)(Some(physical_device), p_properties);
    }
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        p_format_properties: &mut vk::FormatProperties2,
    ) {
        (self.fp1_0.get_physical_device_format_properties2)(Some(physical_device), format, p_format_properties);
    }
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_image_format_info: &vk::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: &mut vk::ImageFormatProperties2,
    ) -> Result<()> {
        let err = (self.fp1_0.get_physical_device_image_format_properties2)(
            Some(physical_device),
            p_image_format_info,
            p_image_format_properties,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_queue_family_properties2_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties2> {
        let mut len = mem::uninitialized();
        (self.fp1_0.get_physical_device_queue_family_properties2)(Some(physical_device), &mut len, ptr::null_mut());
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_0.get_physical_device_queue_family_properties2)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_memory_properties: &mut vk::PhysicalDeviceMemoryProperties2,
    ) {
        (self.fp1_0.get_physical_device_memory_properties2)(Some(physical_device), p_memory_properties);
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
    ) -> Vec<vk::SparseImageFormatProperties2> {
        let mut len = mem::uninitialized();
        (self.fp1_0.get_physical_device_sparse_image_format_properties2)(
            Some(physical_device),
            p_format_info,
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_0.get_physical_device_sparse_image_format_properties2)(
            Some(physical_device),
            p_format_info,
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = v;
        res
    }
}
/// Loader for the `VK_KHR_device_group` device extension
pub struct KhrDeviceGroup {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrDeviceGroupFn1_0,
}
impl KhrDeviceGroup {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrDeviceGroupFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_device_group\0").unwrap()
    }
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> vk::PeerMemoryFeatureFlags {
        let mut res = mem::uninitialized();
        (self.fp1_0.get_device_group_peer_memory_features)(
            Some(self.handle),
            heap_index,
            local_device_index,
            remote_device_index,
            &mut res,
        );
        res
    }
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: vk::CommandBuffer, device_mask: u32) {
        (self.fp1_0.cmd_set_device_mask)(Some(command_buffer), device_mask);
    }
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: vk::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self.fp1_0.cmd_dispatch_base)(
            Some(command_buffer),
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        p_device_group_present_capabilities: &mut vk::DeviceGroupPresentCapabilitiesKHR,
    ) -> Result<()> {
        let err = (self.fp1_0.get_device_group_present_capabilities_khr)(
            Some(self.handle),
            p_device_group_present_capabilities,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        surface: vk::SurfaceKHR,
    ) -> Result<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_device_group_surface_present_modes_khr)(Some(self.handle), Some(surface), &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_present_rectangles_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Vec<vk::Rect2D>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_physical_device_present_rectangles_khr)(
            Some(physical_device),
            Some(surface),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_physical_device_present_rectangles_khr)(
            Some(physical_device),
            Some(surface),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn acquire_next_image2_khr(
        &self,
        p_acquire_info: &vk::AcquireNextImageInfoKHR,
    ) -> Result<(vk::Result, u32)> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.acquire_next_image2_khr)(Some(self.handle), p_acquire_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT | vk::Result::NOT_READY | vk::Result::SUBOPTIMAL_KHR => {
                Ok((err, res))
            }
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_NN_vi_surface` instance extension
pub struct NnViSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::NnViSurfaceFn1_0,
}
impl NnViSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NnViSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NN_vi_surface\0").unwrap()
    }
    pub unsafe fn create_vi_surface_nn(
        &self,
        p_create_info: &vk::ViSurfaceCreateInfoNN,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_vi_surface_nn)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_maintenance1` device extension
pub struct KhrMaintenance1 {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrMaintenance1Fn1_0,
}
impl KhrMaintenance1 {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrMaintenance1Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_maintenance1\0").unwrap()
    }
    pub unsafe fn trim_command_pool(&self, command_pool: vk::CommandPool, flags: vk::CommandPoolTrimFlags) {
        (self.fp1_0.trim_command_pool)(Some(self.handle), Some(command_pool), flags);
    }
}
/// Loader for the `VK_KHR_device_group_creation` instance extension
pub struct KhrDeviceGroupCreation {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrDeviceGroupCreationFn1_0,
}
impl KhrDeviceGroupCreation {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrDeviceGroupCreationFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_device_group_creation\0").unwrap()
    }
    pub unsafe fn enumerate_physical_device_groups_to_vec(&self) -> Result<Vec<vk::PhysicalDeviceGroupProperties>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.enumerate_physical_device_groups)(Some(self.handle), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.enumerate_physical_device_groups)(Some(self.handle), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
}
/// Loader for the `VK_KHR_external_memory_capabilities` instance extension
pub struct KhrExternalMemoryCapabilities {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrExternalMemoryCapabilitiesFn1_0,
}
impl KhrExternalMemoryCapabilities {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrExternalMemoryCapabilitiesFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_external_memory_capabilities\0").unwrap()
    }
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_buffer_info: &vk::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: &mut vk::ExternalBufferProperties,
    ) {
        (self.fp1_0.get_physical_device_external_buffer_properties)(
            Some(physical_device),
            p_external_buffer_info,
            p_external_buffer_properties,
        );
    }
}
/// Loader for the `VK_KHR_external_memory_win32` device extension
pub struct KhrExternalMemoryWin32 {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrExternalMemoryWin32Fn1_0,
}
impl KhrExternalMemoryWin32 {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrExternalMemoryWin32Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_external_memory_win32\0").unwrap()
    }
    pub unsafe fn get_memory_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &vk::MemoryGetWin32HandleInfoKHR,
    ) -> Result<vk::HANDLE> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_memory_win32_handle_khr)(Some(self.handle), p_get_win32_handle_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        handle: vk::HANDLE,
        p_memory_win32_handle_properties: &mut vk::MemoryWin32HandlePropertiesKHR,
    ) -> Result<()> {
        let err = (self.fp1_0.get_memory_win32_handle_properties_khr)(
            Some(self.handle),
            handle_type,
            handle,
            p_memory_win32_handle_properties,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_external_memory_fd` device extension
pub struct KhrExternalMemoryFd {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrExternalMemoryFdFn1_0,
}
impl KhrExternalMemoryFd {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrExternalMemoryFdFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_external_memory_fd\0").unwrap()
    }
    pub unsafe fn get_memory_fd_khr(&self, p_get_fd_info: &vk::MemoryGetFdInfoKHR) -> Result<c_int> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_memory_fd_khr)(Some(self.handle), p_get_fd_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        fd: c_int,
        p_memory_fd_properties: &mut vk::MemoryFdPropertiesKHR,
    ) -> Result<()> {
        let err = (self.fp1_0.get_memory_fd_properties_khr)(Some(self.handle), handle_type, fd, p_memory_fd_properties);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_external_semaphore_capabilities` instance extension
pub struct KhrExternalSemaphoreCapabilities {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrExternalSemaphoreCapabilitiesFn1_0,
}
impl KhrExternalSemaphoreCapabilities {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrExternalSemaphoreCapabilitiesFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_external_semaphore_capabilities\0").unwrap()
    }
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: &mut vk::ExternalSemaphoreProperties,
    ) {
        (self.fp1_0.get_physical_device_external_semaphore_properties)(
            Some(physical_device),
            p_external_semaphore_info,
            p_external_semaphore_properties,
        );
    }
}
/// Loader for the `VK_KHR_external_semaphore_win32` device extension
pub struct KhrExternalSemaphoreWin32 {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrExternalSemaphoreWin32Fn1_0,
}
impl KhrExternalSemaphoreWin32 {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrExternalSemaphoreWin32Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_external_semaphore_win32\0").unwrap()
    }
    pub unsafe fn import_semaphore_win32_handle_khr(
        &self,
        p_import_semaphore_win32_handle_info: &vk::ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result<()> {
        let err =
            (self.fp1_0.import_semaphore_win32_handle_khr)(Some(self.handle), p_import_semaphore_win32_handle_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_semaphore_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &vk::SemaphoreGetWin32HandleInfoKHR,
    ) -> Result<vk::HANDLE> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_semaphore_win32_handle_khr)(Some(self.handle), p_get_win32_handle_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_external_semaphore_fd` device extension
pub struct KhrExternalSemaphoreFd {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrExternalSemaphoreFdFn1_0,
}
impl KhrExternalSemaphoreFd {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrExternalSemaphoreFdFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_external_semaphore_fd\0").unwrap()
    }
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        p_import_semaphore_fd_info: &vk::ImportSemaphoreFdInfoKHR,
    ) -> Result<()> {
        let err = (self.fp1_0.import_semaphore_fd_khr)(Some(self.handle), p_import_semaphore_fd_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_semaphore_fd_khr(&self, p_get_fd_info: &vk::SemaphoreGetFdInfoKHR) -> Result<c_int> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_semaphore_fd_khr)(Some(self.handle), p_get_fd_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_push_descriptor` device extension
pub struct KhrPushDescriptor {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrPushDescriptorFn1_0,
    pub fp1_1: vk::KhrPushDescriptorFn1_1,
}
impl KhrPushDescriptor {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrPushDescriptorFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        let (fp1_1, ok1_1) = vk::KhrPushDescriptorFn1_1::load(f);
        ok = ok && ok1_1;
        if ok {
            version = vk::Version::from_raw_parts(1, 1, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
            fp1_1,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_push_descriptor\0").unwrap()
    }
    pub unsafe fn cmd_push_descriptor_set_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: u32,
        p_descriptor_writes: &[vk::WriteDescriptorSet],
    ) {
        let descriptor_write_count = p_descriptor_writes.len() as u32;
        (self.fp1_0.cmd_push_descriptor_set_khr)(
            Some(command_buffer),
            pipeline_bind_point,
            Some(layout),
            set,
            descriptor_write_count,
            p_descriptor_writes.as_ptr(),
        );
    }
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        layout: vk::PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ) {
        (self.fp1_1.cmd_push_descriptor_set_with_template_khr)(
            Some(command_buffer),
            Some(descriptor_update_template),
            Some(layout),
            set,
            p_data,
        );
    }
}
/// Loader for the `VK_EXT_conditional_rendering` device extension
pub struct ExtConditionalRendering {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtConditionalRenderingFn1_0,
}
impl ExtConditionalRendering {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtConditionalRenderingFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_conditional_rendering\0").unwrap()
    }
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_conditional_rendering_begin: &vk::ConditionalRenderingBeginInfoEXT,
    ) {
        (self.fp1_0.cmd_begin_conditional_rendering_ext)(Some(command_buffer), p_conditional_rendering_begin);
    }
    pub unsafe fn cmd_end_conditional_rendering_ext(&self, command_buffer: vk::CommandBuffer) {
        (self.fp1_0.cmd_end_conditional_rendering_ext)(Some(command_buffer));
    }
}
/// Loader for the `VK_KHR_descriptor_update_template` device extension
pub struct KhrDescriptorUpdateTemplate {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrDescriptorUpdateTemplateFn1_0,
}
impl KhrDescriptorUpdateTemplate {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrDescriptorUpdateTemplateFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_descriptor_update_template\0").unwrap()
    }
    pub unsafe fn create_descriptor_update_template(
        &self,
        p_create_info: &vk::DescriptorUpdateTemplateCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorUpdateTemplate> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_descriptor_update_template)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: Option<vk::DescriptorUpdateTemplate>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_descriptor_update_template)(
            Some(self.handle),
            descriptor_update_template,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        (self.fp1_0.update_descriptor_set_with_template)(
            Some(self.handle),
            Some(descriptor_set),
            Some(descriptor_update_template),
            p_data,
        );
    }
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        layout: vk::PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ) {
        (self.fp1_0.cmd_push_descriptor_set_with_template_khr)(
            Some(command_buffer),
            Some(descriptor_update_template),
            Some(layout),
            set,
            p_data,
        );
    }
}
/// Loader for the `VK_NVX_device_generated_commands` device extension
pub struct NvxDeviceGeneratedCommands {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::NvxDeviceGeneratedCommandsFn1_0,
}
impl NvxDeviceGeneratedCommands {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NvxDeviceGeneratedCommandsFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NVX_device_generated_commands\0").unwrap()
    }
    pub unsafe fn cmd_process_commands_nvx(
        &self,
        command_buffer: vk::CommandBuffer,
        p_process_commands_info: &vk::CmdProcessCommandsInfoNVX,
    ) {
        (self.fp1_0.cmd_process_commands_nvx)(Some(command_buffer), p_process_commands_info);
    }
    pub unsafe fn cmd_reserve_space_for_commands_nvx(
        &self,
        command_buffer: vk::CommandBuffer,
        p_reserve_space_info: &vk::CmdReserveSpaceForCommandsInfoNVX,
    ) {
        (self.fp1_0.cmd_reserve_space_for_commands_nvx)(Some(command_buffer), p_reserve_space_info);
    }
    pub unsafe fn create_indirect_commands_layout_nvx(
        &self,
        p_create_info: &vk::IndirectCommandsLayoutCreateInfoNVX,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::IndirectCommandsLayoutNVX> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_indirect_commands_layout_nvx)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_indirect_commands_layout_nvx(
        &self,
        indirect_commands_layout: vk::IndirectCommandsLayoutNVX,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_indirect_commands_layout_nvx)(
            Some(self.handle),
            Some(indirect_commands_layout),
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn create_object_table_nvx(
        &self,
        p_create_info: &vk::ObjectTableCreateInfoNVX,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ObjectTableNVX> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_object_table_nvx)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_object_table_nvx(
        &self,
        object_table: vk::ObjectTableNVX,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_object_table_nvx)(
            Some(self.handle),
            Some(object_table),
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn register_objects_nvx(
        &self,
        object_table: vk::ObjectTableNVX,
        pp_object_table_entries: *const *const vk::ObjectTableEntryNVX,
        p_object_indices: &[u32],
    ) -> Result<()> {
        let object_count = p_object_indices.len() as u32;
        let err = (self.fp1_0.register_objects_nvx)(
            Some(self.handle),
            Some(object_table),
            object_count,
            pp_object_table_entries,
            p_object_indices.as_ptr(),
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn unregister_objects_nvx(
        &self,
        object_table: vk::ObjectTableNVX,
        p_object_entry_types: &[vk::ObjectEntryTypeNVX],
        p_object_indices: &[u32],
    ) -> Result<()> {
        let object_count = p_object_entry_types.len() as u32;
        assert_eq!(object_count, p_object_indices.len() as u32);
        let err = (self.fp1_0.unregister_objects_nvx)(
            Some(self.handle),
            Some(object_table),
            object_count,
            p_object_entry_types.as_ptr(),
            p_object_indices.as_ptr(),
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_generated_commands_properties_nvx(
        &self,
        physical_device: vk::PhysicalDevice,
        p_features: &mut vk::DeviceGeneratedCommandsFeaturesNVX,
        p_limits: &mut vk::DeviceGeneratedCommandsLimitsNVX,
    ) {
        (self.fp1_0.get_physical_device_generated_commands_properties_nvx)(Some(physical_device), p_features, p_limits);
    }
}
/// Loader for the `VK_NV_clip_space_w_scaling` device extension
pub struct NvClipSpaceWScaling {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::NvClipSpaceWScalingFn1_0,
}
impl NvClipSpaceWScaling {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NvClipSpaceWScalingFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_clip_space_w_scaling\0").unwrap()
    }
    pub unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        p_viewport_w_scalings: &[vk::ViewportWScalingNV],
    ) {
        let viewport_count = p_viewport_w_scalings.len() as u32;
        (self.fp1_0.cmd_set_viewport_w_scaling_nv)(
            Some(command_buffer),
            first_viewport,
            viewport_count,
            p_viewport_w_scalings.as_ptr(),
        );
    }
}
/// Loader for the `VK_EXT_direct_mode_display` instance extension
pub struct ExtDirectModeDisplay {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::ExtDirectModeDisplayFn1_0,
}
impl ExtDirectModeDisplay {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtDirectModeDisplayFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_direct_mode_display\0").unwrap()
    }
    pub unsafe fn release_display_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> Result<()> {
        let err = (self.fp1_0.release_display_ext)(Some(physical_device), Some(display));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_EXT_acquire_xlib_display` instance extension
pub struct ExtAcquireXlibDisplay {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::ExtAcquireXlibDisplayFn1_0,
}
impl ExtAcquireXlibDisplay {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtAcquireXlibDisplayFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_acquire_xlib_display\0").unwrap()
    }
    pub unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        dpy: &mut vk::Display,
        display: vk::DisplayKHR,
    ) -> Result<()> {
        let err = (self.fp1_0.acquire_xlib_display_ext)(Some(physical_device), dpy, Some(display));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_rand_r_output_display_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        dpy: &mut vk::Display,
        rr_output: vk::RROutput,
    ) -> Result<vk::DisplayKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_rand_r_output_display_ext)(Some(physical_device), dpy, rr_output, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_EXT_display_surface_counter` instance extension
pub struct ExtDisplaySurfaceCounter {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::ExtDisplaySurfaceCounterFn1_0,
}
impl ExtDisplaySurfaceCounter {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtDisplaySurfaceCounterFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_display_surface_counter\0").unwrap()
    }
    pub unsafe fn get_physical_device_surface_capabilities2_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_surface_capabilities: &mut vk::SurfaceCapabilities2EXT,
    ) -> Result<()> {
        let err = (self.fp1_0.get_physical_device_surface_capabilities2_ext)(
            Some(physical_device),
            Some(surface),
            p_surface_capabilities,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_EXT_display_control` device extension
pub struct ExtDisplayControl {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtDisplayControlFn1_0,
}
impl ExtDisplayControl {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtDisplayControlFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_display_control\0").unwrap()
    }
    pub unsafe fn display_power_control_ext(
        &self,
        display: vk::DisplayKHR,
        p_display_power_info: &vk::DisplayPowerInfoEXT,
    ) -> Result<()> {
        let err = (self.fp1_0.display_power_control_ext)(Some(self.handle), Some(display), p_display_power_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn register_device_event_ext(
        &self,
        p_device_event_info: &vk::DeviceEventInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Fence> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.register_device_event_ext)(
            Some(self.handle),
            p_device_event_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn register_display_event_ext(
        &self,
        display: vk::DisplayKHR,
        p_display_event_info: &vk::DisplayEventInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Fence> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.register_display_event_ext)(
            Some(self.handle),
            Some(display),
            p_display_event_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_swapchain_counter_ext(
        &self,
        swapchain: vk::SwapchainKHR,
        counter: vk::SurfaceCounterFlagsEXT,
    ) -> Result<u64> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_swapchain_counter_ext)(Some(self.handle), Some(swapchain), counter, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_GOOGLE_display_timing` device extension
pub struct GoogleDisplayTiming {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::GoogleDisplayTimingFn1_0,
}
impl GoogleDisplayTiming {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::GoogleDisplayTimingFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_GOOGLE_display_timing\0").unwrap()
    }
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> Result<vk::RefreshCycleDurationGOOGLE> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_refresh_cycle_duration_google)(Some(self.handle), Some(swapchain), &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_past_presentation_timing_google_to_vec(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> Result<Vec<vk::PastPresentationTimingGOOGLE>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_past_presentation_timing_google)(
            Some(self.handle),
            Some(swapchain),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_past_presentation_timing_google)(
            Some(self.handle),
            Some(swapchain),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
}
/// Loader for the `VK_EXT_discard_rectangles` device extension
pub struct ExtDiscardRectangles {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtDiscardRectanglesFn1_0,
}
impl ExtDiscardRectangles {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtDiscardRectanglesFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_discard_rectangles\0").unwrap()
    }
    pub unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_discard_rectangle: u32,
        p_discard_rectangles: &[vk::Rect2D],
    ) {
        let discard_rectangle_count = p_discard_rectangles.len() as u32;
        (self.fp1_0.cmd_set_discard_rectangle_ext)(
            Some(command_buffer),
            first_discard_rectangle,
            discard_rectangle_count,
            p_discard_rectangles.as_ptr(),
        );
    }
}
/// Loader for the `VK_EXT_hdr_metadata` device extension
pub struct ExtHdrMetadata {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtHdrMetadataFn1_0,
}
impl ExtHdrMetadata {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtHdrMetadataFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_hdr_metadata\0").unwrap()
    }
    pub unsafe fn set_hdr_metadata_ext(&self, p_swapchains: &[vk::SwapchainKHR], p_metadata: &[vk::HdrMetadataEXT]) {
        let swapchain_count = p_swapchains.len() as u32;
        assert_eq!(swapchain_count, p_metadata.len() as u32);
        (self.fp1_0.set_hdr_metadata_ext)(
            Some(self.handle),
            swapchain_count,
            p_swapchains.as_ptr(),
            p_metadata.as_ptr(),
        );
    }
}
/// Loader for the `VK_KHR_create_renderpass2` device extension
pub struct KhrCreateRenderpass2 {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrCreateRenderpass2Fn1_0,
}
impl KhrCreateRenderpass2 {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrCreateRenderpass2Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_create_renderpass2\0").unwrap()
    }
    pub unsafe fn create_render_pass2_khr(
        &self,
        p_create_info: &vk::RenderPassCreateInfo2KHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::RenderPass> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_render_pass2_khr)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn cmd_begin_render_pass2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_render_pass_begin: &vk::RenderPassBeginInfo,
        p_subpass_begin_info: &vk::SubpassBeginInfoKHR,
    ) {
        (self.fp1_0.cmd_begin_render_pass2_khr)(Some(command_buffer), p_render_pass_begin, p_subpass_begin_info);
    }
    pub unsafe fn cmd_next_subpass2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_subpass_begin_info: &vk::SubpassBeginInfoKHR,
        p_subpass_end_info: &vk::SubpassEndInfoKHR,
    ) {
        (self.fp1_0.cmd_next_subpass2_khr)(Some(command_buffer), p_subpass_begin_info, p_subpass_end_info);
    }
    pub unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_subpass_end_info: &vk::SubpassEndInfoKHR,
    ) {
        (self.fp1_0.cmd_end_render_pass2_khr)(Some(command_buffer), p_subpass_end_info);
    }
}
/// Loader for the `VK_KHR_shared_presentable_image` device extension
pub struct KhrSharedPresentableImage {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrSharedPresentableImageFn1_0,
}
impl KhrSharedPresentableImage {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrSharedPresentableImageFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_shared_presentable_image\0").unwrap()
    }
    pub unsafe fn get_swapchain_status_khr(&self, swapchain: vk::SwapchainKHR) -> Result<vk::Result> {
        let err = (self.fp1_0.get_swapchain_status_khr)(Some(self.handle), Some(swapchain));
        let res = match err {
            vk::Result::SUCCESS | vk::Result::SUBOPTIMAL_KHR => Ok(err),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_external_fence_capabilities` instance extension
pub struct KhrExternalFenceCapabilities {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrExternalFenceCapabilitiesFn1_0,
}
impl KhrExternalFenceCapabilities {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrExternalFenceCapabilitiesFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_external_fence_capabilities\0").unwrap()
    }
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_fence_info: &vk::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: &mut vk::ExternalFenceProperties,
    ) {
        (self.fp1_0.get_physical_device_external_fence_properties)(
            Some(physical_device),
            p_external_fence_info,
            p_external_fence_properties,
        );
    }
}
/// Loader for the `VK_KHR_external_fence_win32` device extension
pub struct KhrExternalFenceWin32 {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrExternalFenceWin32Fn1_0,
}
impl KhrExternalFenceWin32 {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrExternalFenceWin32Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_external_fence_win32\0").unwrap()
    }
    pub unsafe fn import_fence_win32_handle_khr(
        &self,
        p_import_fence_win32_handle_info: &vk::ImportFenceWin32HandleInfoKHR,
    ) -> Result<()> {
        let err = (self.fp1_0.import_fence_win32_handle_khr)(Some(self.handle), p_import_fence_win32_handle_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_fence_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &vk::FenceGetWin32HandleInfoKHR,
    ) -> Result<vk::HANDLE> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_fence_win32_handle_khr)(Some(self.handle), p_get_win32_handle_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_external_fence_fd` device extension
pub struct KhrExternalFenceFd {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrExternalFenceFdFn1_0,
}
impl KhrExternalFenceFd {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrExternalFenceFdFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_external_fence_fd\0").unwrap()
    }
    pub unsafe fn import_fence_fd_khr(&self, p_import_fence_fd_info: &vk::ImportFenceFdInfoKHR) -> Result<()> {
        let err = (self.fp1_0.import_fence_fd_khr)(Some(self.handle), p_import_fence_fd_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_fence_fd_khr(&self, p_get_fd_info: &vk::FenceGetFdInfoKHR) -> Result<c_int> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_fence_fd_khr)(Some(self.handle), p_get_fd_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_get_surface_capabilities2` instance extension
pub struct KhrGetSurfaceCapabilities2 {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrGetSurfaceCapabilities2Fn1_0,
}
impl KhrGetSurfaceCapabilities2 {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrGetSurfaceCapabilities2Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_get_surface_capabilities2\0").unwrap()
    }
    pub unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
        p_surface_capabilities: &mut vk::SurfaceCapabilities2KHR,
    ) -> Result<()> {
        let err = (self.fp1_0.get_physical_device_surface_capabilities2_khr)(
            Some(physical_device),
            p_surface_info,
            p_surface_capabilities,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_surface_formats2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<Vec<vk::SurfaceFormat2KHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_physical_device_surface_formats2_khr)(
            Some(physical_device),
            p_surface_info,
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_physical_device_surface_formats2_khr)(
            Some(physical_device),
            p_surface_info,
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
}
/// Loader for the `VK_KHR_get_display_properties2` instance extension
pub struct KhrGetDisplayProperties2 {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::KhrGetDisplayProperties2Fn1_0,
}
impl KhrGetDisplayProperties2 {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrGetDisplayProperties2Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_get_display_properties2\0").unwrap()
    }
    pub unsafe fn get_physical_device_display_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayProperties2KHR>> {
        let mut len = mem::uninitialized();
        let len_err =
            (self.fp1_0.get_physical_device_display_properties2_khr)(Some(physical_device), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err =
            (self.fp1_0.get_physical_device_display_properties2_khr)(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_physical_device_display_plane_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayPlaneProperties2KHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_physical_device_display_plane_properties2_khr)(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_physical_device_display_plane_properties2_khr)(
            Some(physical_device),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_display_mode_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> Result<Vec<vk::DisplayModeProperties2KHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_display_mode_properties2_khr)(
            Some(physical_device),
            Some(display),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_display_mode_properties2_khr)(
            Some(physical_device),
            Some(display),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_display_plane_capabilities2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_display_plane_info: &vk::DisplayPlaneInfo2KHR,
        p_capabilities: &mut vk::DisplayPlaneCapabilities2KHR,
    ) -> Result<()> {
        let err = (self.fp1_0.get_display_plane_capabilities2_khr)(
            Some(physical_device),
            p_display_plane_info,
            p_capabilities,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_MVK_ios_surface` instance extension
pub struct MvkIosSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::MvkIosSurfaceFn1_0,
}
impl MvkIosSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::MvkIosSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_MVK_ios_surface\0").unwrap()
    }
    pub unsafe fn create_ios_surface_mvk(
        &self,
        p_create_info: &vk::IOSSurfaceCreateInfoMVK,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_ios_surface_mvk)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_MVK_macos_surface` instance extension
pub struct MvkMacosSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::MvkMacosSurfaceFn1_0,
}
impl MvkMacosSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::MvkMacosSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_MVK_macos_surface\0").unwrap()
    }
    pub unsafe fn create_mac_os_surface_mvk(
        &self,
        p_create_info: &vk::MacOSSurfaceCreateInfoMVK,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_mac_os_surface_mvk)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_EXT_debug_utils` instance extension
pub struct ExtDebugUtils {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::ExtDebugUtilsFn1_0,
}
impl ExtDebugUtils {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtDebugUtilsFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_debug_utils\0").unwrap()
    }
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: vk::Device,
        p_name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) -> Result<()> {
        let err = (self.fp1_0.set_debug_utils_object_name_ext)(Some(device), p_name_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        device: vk::Device,
        p_tag_info: &vk::DebugUtilsObjectTagInfoEXT,
    ) -> Result<()> {
        let err = (self.fp1_0.set_debug_utils_object_tag_ext)(Some(device), p_tag_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn queue_begin_debug_utils_label_ext(&self, queue: vk::Queue, p_label_info: &vk::DebugUtilsLabelEXT) {
        (self.fp1_0.queue_begin_debug_utils_label_ext)(Some(queue), p_label_info);
    }
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: vk::Queue) {
        (self.fp1_0.queue_end_debug_utils_label_ext)(Some(queue));
    }
    pub unsafe fn queue_insert_debug_utils_label_ext(&self, queue: vk::Queue, p_label_info: &vk::DebugUtilsLabelEXT) {
        (self.fp1_0.queue_insert_debug_utils_label_ext)(Some(queue), p_label_info);
    }
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_label_info: &vk::DebugUtilsLabelEXT,
    ) {
        (self.fp1_0.cmd_begin_debug_utils_label_ext)(Some(command_buffer), p_label_info);
    }
    pub unsafe fn cmd_end_debug_utils_label_ext(&self, command_buffer: vk::CommandBuffer) {
        (self.fp1_0.cmd_end_debug_utils_label_ext)(Some(command_buffer));
    }
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_label_info: &vk::DebugUtilsLabelEXT,
    ) {
        (self.fp1_0.cmd_insert_debug_utils_label_ext)(Some(command_buffer), p_label_info);
    }
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        p_create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DebugUtilsMessengerEXT> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_debug_utils_messenger_ext)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        messenger: vk::DebugUtilsMessengerEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_debug_utils_messenger_ext)(
            Some(self.handle),
            Some(messenger),
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_types: vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: &vk::DebugUtilsMessengerCallbackDataEXT,
    ) {
        (self.fp1_0.submit_debug_utils_message_ext)(
            Some(self.handle),
            message_severity,
            message_types,
            p_callback_data,
        );
    }
}
/// Loader for the `VK_ANDROID_external_memory_android_hardware_buffer` device extension
pub struct AndroidExternalMemoryAndroidHardwareBuffer {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::AndroidExternalMemoryAndroidHardwareBufferFn1_0,
}
impl AndroidExternalMemoryAndroidHardwareBuffer {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::AndroidExternalMemoryAndroidHardwareBufferFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_ANDROID_external_memory_android_hardware_buffer\0").unwrap()
    }
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        buffer: &vk::AHardwareBuffer,
        p_properties: &mut vk::AndroidHardwareBufferPropertiesANDROID,
    ) -> Result<()> {
        let err = (self.fp1_0.get_android_hardware_buffer_properties_android)(Some(self.handle), buffer, p_properties);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        p_info: &vk::MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> Result<*mut vk::AHardwareBuffer> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.get_memory_android_hardware_buffer_android)(Some(self.handle), p_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_EXT_sample_locations` device extension
pub struct ExtSampleLocations {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtSampleLocationsFn1_0,
}
impl ExtSampleLocations {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtSampleLocationsFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_sample_locations\0").unwrap()
    }
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_sample_locations_info: &vk::SampleLocationsInfoEXT,
    ) {
        (self.fp1_0.cmd_set_sample_locations_ext)(Some(command_buffer), p_sample_locations_info);
    }
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        samples: vk::SampleCountFlags,
        p_multisample_properties: &mut vk::MultisamplePropertiesEXT,
    ) {
        (self.fp1_0.get_physical_device_multisample_properties_ext)(
            Some(physical_device),
            samples,
            p_multisample_properties,
        );
    }
}
/// Loader for the `VK_KHR_get_memory_requirements2` device extension
pub struct KhrGetMemoryRequirements2 {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrGetMemoryRequirements2Fn1_0,
}
impl KhrGetMemoryRequirements2 {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrGetMemoryRequirements2Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_get_memory_requirements2\0").unwrap()
    }
    pub unsafe fn get_image_memory_requirements2(
        &self,
        p_info: &vk::ImageMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        (self.fp1_0.get_image_memory_requirements2)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        p_info: &vk::BufferMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        (self.fp1_0.get_buffer_memory_requirements2)(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_image_sparse_memory_requirements2_to_vec(
        &self,
        p_info: &vk::ImageSparseMemoryRequirementsInfo2,
    ) -> Vec<vk::SparseImageMemoryRequirements2> {
        let mut len = mem::uninitialized();
        (self.fp1_0.get_image_sparse_memory_requirements2)(Some(self.handle), p_info, &mut len, ptr::null_mut());
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_0.get_image_sparse_memory_requirements2)(Some(self.handle), p_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
    }
}
/// Loader for the `VK_KHR_sampler_ycbcr_conversion` device extension
pub struct KhrSamplerYcbcrConversion {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrSamplerYcbcrConversionFn1_0,
}
impl KhrSamplerYcbcrConversion {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrSamplerYcbcrConversionFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_sampler_ycbcr_conversion\0").unwrap()
    }
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        p_create_info: &vk::SamplerYcbcrConversionCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SamplerYcbcrConversion> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_sampler_ycbcr_conversion)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: Option<vk::SamplerYcbcrConversion>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_sampler_ycbcr_conversion)(
            Some(self.handle),
            ycbcr_conversion,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
}
/// Loader for the `VK_KHR_bind_memory2` device extension
pub struct KhrBindMemory2 {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrBindMemory2Fn1_0,
}
impl KhrBindMemory2 {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrBindMemory2Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_bind_memory2\0").unwrap()
    }
    pub unsafe fn bind_buffer_memory2(&self, p_bind_infos: &[vk::BindBufferMemoryInfo]) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp1_0.bind_buffer_memory2)(Some(self.handle), bind_info_count, p_bind_infos.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn bind_image_memory2(&self, p_bind_infos: &[vk::BindImageMemoryInfo]) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp1_0.bind_image_memory2)(Some(self.handle), bind_info_count, p_bind_infos.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_EXT_image_drm_format_modifier` device extension
pub struct ExtImageDrmFormatModifier {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtImageDrmFormatModifierFn1_0,
}
impl ExtImageDrmFormatModifier {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtImageDrmFormatModifierFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_image_drm_format_modifier\0").unwrap()
    }
    pub unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        image: vk::Image,
        p_properties: &mut vk::ImageDrmFormatModifierPropertiesEXT,
    ) -> Result<()> {
        let err =
            (self.fp1_0.get_image_drm_format_modifier_properties_ext)(Some(self.handle), Some(image), p_properties);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_EXT_validation_cache` device extension
pub struct ExtValidationCache {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtValidationCacheFn1_0,
}
impl ExtValidationCache {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtValidationCacheFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_validation_cache\0").unwrap()
    }
    pub unsafe fn create_validation_cache_ext(
        &self,
        p_create_info: &vk::ValidationCacheCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ValidationCacheEXT> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_validation_cache_ext)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        validation_cache: Option<vk::ValidationCacheEXT>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_validation_cache_ext)(
            Some(self.handle),
            validation_cache,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn merge_validation_caches_ext(
        &self,
        dst_cache: vk::ValidationCacheEXT,
        p_src_caches: &[vk::ValidationCacheEXT],
    ) -> Result<()> {
        let src_cache_count = p_src_caches.len() as u32;
        let err = (self.fp1_0.merge_validation_caches_ext)(
            Some(self.handle),
            Some(dst_cache),
            src_cache_count,
            p_src_caches.as_ptr(),
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        validation_cache: vk::ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result<vk::Result> {
        let err =
            (self.fp1_0.get_validation_cache_data_ext)(Some(self.handle), Some(validation_cache), p_data_size, p_data);
        let res = match err {
            vk::Result::SUCCESS | vk::Result::INCOMPLETE => Ok(err),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_NV_shading_rate_image` device extension
pub struct NvShadingRateImage {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::NvShadingRateImageFn1_0,
}
impl NvShadingRateImage {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NvShadingRateImageFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_shading_rate_image\0").unwrap()
    }
    pub unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        image_view: vk::ImageView,
        image_layout: vk::ImageLayout,
    ) {
        (self.fp1_0.cmd_bind_shading_rate_image_nv)(Some(command_buffer), Some(image_view), image_layout);
    }
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        p_shading_rate_palettes: &[vk::ShadingRatePaletteNV],
    ) {
        let viewport_count = p_shading_rate_palettes.len() as u32;
        (self.fp1_0.cmd_set_viewport_shading_rate_palette_nv)(
            Some(command_buffer),
            first_viewport,
            viewport_count,
            p_shading_rate_palettes.as_ptr(),
        );
    }
    pub unsafe fn cmd_set_coarse_sample_order_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        sample_order_type: vk::CoarseSampleOrderTypeNV,
        p_custom_sample_orders: &[vk::CoarseSampleOrderCustomNV],
    ) {
        let custom_sample_order_count = p_custom_sample_orders.len() as u32;
        (self.fp1_0.cmd_set_coarse_sample_order_nv)(
            Some(command_buffer),
            sample_order_type,
            custom_sample_order_count,
            p_custom_sample_orders.as_ptr(),
        );
    }
}
/// Loader for the `VK_NVX_raytracing` device extension
pub struct NvxRaytracing {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::NvxRaytracingFn1_0,
}
impl NvxRaytracing {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NvxRaytracingFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NVX_raytracing\0").unwrap()
    }
    pub unsafe fn create_acceleration_structure_nvx(
        &self,
        p_create_info: &vk::AccelerationStructureCreateInfoNVX,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::AccelerationStructureNVX> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_acceleration_structure_nvx)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn destroy_acceleration_structure_nvx(
        &self,
        acceleration_structure: vk::AccelerationStructureNVX,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp1_0.destroy_acceleration_structure_nvx)(
            Some(self.handle),
            Some(acceleration_structure),
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn get_acceleration_structure_memory_requirements_nvx(
        &self,
        p_info: &vk::AccelerationStructureMemoryRequirementsInfoNVX,
        p_memory_requirements: &mut vk::MemoryRequirements2KHR,
    ) {
        (self.fp1_0.get_acceleration_structure_memory_requirements_nvx)(
            Some(self.handle),
            p_info,
            p_memory_requirements,
        );
    }
    pub unsafe fn get_acceleration_structure_scratch_memory_requirements_nvx(
        &self,
        p_info: &vk::AccelerationStructureMemoryRequirementsInfoNVX,
        p_memory_requirements: &mut vk::MemoryRequirements2KHR,
    ) {
        (self.fp1_0.get_acceleration_structure_scratch_memory_requirements_nvx)(
            Some(self.handle),
            p_info,
            p_memory_requirements,
        );
    }
    pub unsafe fn bind_acceleration_structure_memory_nvx(
        &self,
        p_bind_infos: &[vk::BindAccelerationStructureMemoryInfoNVX],
    ) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp1_0.bind_acceleration_structure_memory_nvx)(
            Some(self.handle),
            bind_info_count,
            p_bind_infos.as_ptr(),
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn cmd_build_acceleration_structure_nvx(
        &self,
        command_buffer: vk::CommandBuffer,
        ty: vk::AccelerationStructureTypeNVX,
        instance_count: u32,
        instance_data: Option<vk::Buffer>,
        instance_offset: vk::DeviceSize,
        p_geometries: &[vk::GeometryNVX],
        flags: vk::BuildAccelerationStructureFlagsNVX,
        update: bool,
        dst: vk::AccelerationStructureNVX,
        src: Option<vk::AccelerationStructureNVX>,
        scratch: vk::Buffer,
        scratch_offset: vk::DeviceSize,
    ) {
        let geometry_count = p_geometries.len() as u32;
        (self.fp1_0.cmd_build_acceleration_structure_nvx)(
            Some(command_buffer),
            ty,
            instance_count,
            instance_data,
            instance_offset,
            geometry_count,
            p_geometries.as_ptr(),
            flags,
            if update { vk::TRUE } else { vk::FALSE },
            Some(dst),
            src,
            Some(scratch),
            scratch_offset,
        );
    }
    pub unsafe fn cmd_copy_acceleration_structure_nvx(
        &self,
        command_buffer: vk::CommandBuffer,
        dst: vk::AccelerationStructureNVX,
        src: vk::AccelerationStructureNVX,
        mode: vk::CopyAccelerationStructureModeNVX,
    ) {
        (self.fp1_0.cmd_copy_acceleration_structure_nvx)(Some(command_buffer), Some(dst), Some(src), mode);
    }
    pub unsafe fn cmd_trace_rays_nvx(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table_buffer: vk::Buffer,
        raygen_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_table_buffer: vk::Buffer,
        miss_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_stride: vk::DeviceSize,
        hit_shader_binding_table_buffer: vk::Buffer,
        hit_shader_binding_offset: vk::DeviceSize,
        hit_shader_binding_stride: vk::DeviceSize,
        width: u32,
        height: u32,
    ) {
        (self.fp1_0.cmd_trace_rays_nvx)(
            Some(command_buffer),
            Some(raygen_shader_binding_table_buffer),
            raygen_shader_binding_offset,
            Some(miss_shader_binding_table_buffer),
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            Some(hit_shader_binding_table_buffer),
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            width,
            height,
        );
    }
    pub unsafe fn create_raytracing_pipelines_nvx(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RaytracingPipelineCreateInfoNVX],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_pipelines: *mut vk::Pipeline,
    ) -> Result<()> {
        let create_info_count = p_create_infos.len() as u32;
        let v_err = (self.fp1_0.create_raytracing_pipelines_nvx)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            p_pipelines,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_raytracing_pipelines_nvx_to_vec(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RaytracingPipelineCreateInfoNVX],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::Pipeline>> {
        let create_info_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(create_info_count as usize);
        v.set_len(create_info_count as usize);
        let v_err = (self.fp1_0.create_raytracing_pipelines_nvx)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_raytracing_pipelines_nvx_array<A: Array<Item = vk::Pipeline>>(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RaytracingPipelineCreateInfoNVX],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<A> {
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, A::len() as u32);
        let mut v: A = mem::uninitialized();
        let v_err = (self.fp1_0.create_raytracing_pipelines_nvx)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            v.as_mut_ptr(),
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_raytracing_pipelines_nvx_single(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RaytracingPipelineCreateInfoNVX],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, 1);
        let mut v = mem::uninitialized();
        let v_err = (self.fp1_0.create_raytracing_pipelines_nvx)(
            Some(self.handle),
            pipeline_cache,
            create_info_count,
            p_create_infos.as_ptr(),
            p_allocator.map_or(ptr::null(), |r| r),
            &mut v,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_raytracing_shader_handles_nvx(
        &self,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result<()> {
        let err = (self.fp1_0.get_raytracing_shader_handles_nvx)(
            Some(self.handle),
            Some(pipeline),
            first_group,
            group_count,
            data_size,
            p_data,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_acceleration_structure_handle_nvx(
        &self,
        acceleration_structure: vk::AccelerationStructureNVX,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result<()> {
        let err = (self.fp1_0.get_acceleration_structure_handle_nvx)(
            Some(self.handle),
            Some(acceleration_structure),
            data_size,
            p_data,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn cmd_write_acceleration_structure_properties_nvx(
        &self,
        command_buffer: vk::CommandBuffer,
        acceleration_structure: vk::AccelerationStructureNVX,
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        (self.fp1_0.cmd_write_acceleration_structure_properties_nvx)(
            Some(command_buffer),
            Some(acceleration_structure),
            query_type,
            Some(query_pool),
            query,
        );
    }
    pub unsafe fn compile_deferred_nvx(&self, pipeline: vk::Pipeline, shader: u32) -> Result<()> {
        let err = (self.fp1_0.compile_deferred_nvx)(Some(self.handle), Some(pipeline), shader);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_KHR_maintenance3` device extension
pub struct KhrMaintenance3 {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrMaintenance3Fn1_0,
}
impl KhrMaintenance3 {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrMaintenance3Fn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_maintenance3\0").unwrap()
    }
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        p_support: &mut vk::DescriptorSetLayoutSupport,
    ) {
        (self.fp1_0.get_descriptor_set_layout_support)(Some(self.handle), p_create_info, p_support);
    }
}
/// Loader for the `VK_KHR_draw_indirect_count` device extension
pub struct KhrDrawIndirectCount {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::KhrDrawIndirectCountFn1_0,
}
impl KhrDrawIndirectCount {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::KhrDrawIndirectCountFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_draw_indirect_count\0").unwrap()
    }
    pub unsafe fn cmd_draw_indirect_count_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.fp1_0.cmd_draw_indirect_count_khr)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.fp1_0.cmd_draw_indexed_indirect_count_khr)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
}
/// Loader for the `VK_EXT_external_memory_host` device extension
pub struct ExtExternalMemoryHost {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtExternalMemoryHostFn1_0,
}
impl ExtExternalMemoryHost {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtExternalMemoryHostFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_external_memory_host\0").unwrap()
    }
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: &mut vk::MemoryHostPointerPropertiesEXT,
    ) -> Result<()> {
        let err = (self.fp1_0.get_memory_host_pointer_properties_ext)(
            Some(self.handle),
            handle_type,
            p_host_pointer,
            p_memory_host_pointer_properties,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
}
/// Loader for the `VK_AMD_buffer_marker` device extension
pub struct AmdBufferMarker {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::AmdBufferMarkerFn1_0,
}
impl AmdBufferMarker {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::AmdBufferMarkerFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_AMD_buffer_marker\0").unwrap()
    }
    pub unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        marker: u32,
    ) {
        (self.fp1_0.cmd_write_buffer_marker_amd)(
            Some(command_buffer),
            pipeline_stage,
            Some(dst_buffer),
            dst_offset,
            marker,
        );
    }
}
/// Loader for the `VK_EXT_calibrated_timestamps` device extension
pub struct ExtCalibratedTimestamps {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::ExtCalibratedTimestampsFn1_0,
}
impl ExtCalibratedTimestamps {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::ExtCalibratedTimestampsFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_calibrated_timestamps\0").unwrap()
    }
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::TimeDomainEXT>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp1_0.get_physical_device_calibrateable_time_domains_ext)(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp1_0.get_physical_device_calibrateable_time_domains_ext)(
            Some(physical_device),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_calibrated_timestamps_ext(
        &self,
        p_timestamp_infos: &[vk::CalibratedTimestampInfoEXT],
        p_timestamps: *mut u64,
        p_max_deviation: &mut u64,
    ) -> Result<()> {
        let timestamp_count = p_timestamp_infos.len() as u32;
        let v_err = (self.fp1_0.get_calibrated_timestamps_ext)(
            Some(self.handle),
            timestamp_count,
            p_timestamp_infos.as_ptr(),
            p_timestamps,
            p_max_deviation,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_calibrated_timestamps_ext_to_vec(
        &self,
        p_timestamp_infos: &[vk::CalibratedTimestampInfoEXT],
        p_max_deviation: &mut u64,
    ) -> Result<Vec<u64>> {
        let timestamp_count = p_timestamp_infos.len() as u32;
        let mut v = Vec::with_capacity(timestamp_count as usize);
        v.set_len(timestamp_count as usize);
        let v_err = (self.fp1_0.get_calibrated_timestamps_ext)(
            Some(self.handle),
            timestamp_count,
            p_timestamp_infos.as_ptr(),
            v.as_mut_ptr(),
            p_max_deviation,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_calibrated_timestamps_ext_array<A: Array<Item = u64>>(
        &self,
        p_timestamp_infos: &[vk::CalibratedTimestampInfoEXT],
        p_max_deviation: &mut u64,
    ) -> Result<A> {
        let timestamp_count = p_timestamp_infos.len() as u32;
        assert_eq!(timestamp_count, A::len() as u32);
        let mut v: A = mem::uninitialized();
        let v_err = (self.fp1_0.get_calibrated_timestamps_ext)(
            Some(self.handle),
            timestamp_count,
            p_timestamp_infos.as_ptr(),
            v.as_mut_ptr(),
            p_max_deviation,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_calibrated_timestamps_ext_single(
        &self,
        p_timestamp_infos: &[vk::CalibratedTimestampInfoEXT],
        p_max_deviation: &mut u64,
    ) -> Result<u64> {
        let timestamp_count = p_timestamp_infos.len() as u32;
        assert_eq!(timestamp_count, 1);
        let mut v = mem::uninitialized();
        let v_err = (self.fp1_0.get_calibrated_timestamps_ext)(
            Some(self.handle),
            timestamp_count,
            p_timestamp_infos.as_ptr(),
            &mut v,
            p_max_deviation,
        );
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
}
/// Loader for the `VK_NV_mesh_shader` device extension
pub struct NvMeshShader {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::NvMeshShaderFn1_0,
}
impl NvMeshShader {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NvMeshShaderFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_mesh_shader\0").unwrap()
    }
    pub unsafe fn cmd_draw_mesh_tasks_nv(&self, command_buffer: vk::CommandBuffer, task_count: u32, first_task: u32) {
        (self.fp1_0.cmd_draw_mesh_tasks_nv)(Some(command_buffer), task_count, first_task);
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self.fp1_0.cmd_draw_mesh_tasks_indirect_nv)(Some(command_buffer), Some(buffer), offset, draw_count, stride);
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.fp1_0.cmd_draw_mesh_tasks_indirect_count_nv)(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
}
/// Loader for the `VK_NV_scissor_exclusive` device extension
pub struct NvScissorExclusive {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::NvScissorExclusiveFn1_0,
}
impl NvScissorExclusive {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NvScissorExclusiveFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_scissor_exclusive\0").unwrap()
    }
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_exclusive_scissor: u32,
        p_exclusive_scissors: &[vk::Rect2D],
    ) {
        let exclusive_scissor_count = p_exclusive_scissors.len() as u32;
        (self.fp1_0.cmd_set_exclusive_scissor_nv)(
            Some(command_buffer),
            first_exclusive_scissor,
            exclusive_scissor_count,
            p_exclusive_scissors.as_ptr(),
        );
    }
}
/// Loader for the `VK_NV_device_diagnostic_checkpoints` device extension
pub struct NvDeviceDiagnosticCheckpoints {
    pub version: vk::Version,
    pub handle: vk::Device,
    pub fp1_0: vk::NvDeviceDiagnosticCheckpointsFn1_0,
}
impl NvDeviceDiagnosticCheckpoints {
    pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {
        let f = |name: &CStr| {
            instance
                .get_device_proc_addr(device.handle, name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::NvDeviceDiagnosticCheckpointsFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: device.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_device_diagnostic_checkpoints\0").unwrap()
    }
    pub unsafe fn cmd_set_checkpoint_nv(&self, command_buffer: vk::CommandBuffer, p_checkpoint_marker: *const c_void) {
        (self.fp1_0.cmd_set_checkpoint_nv)(Some(command_buffer), p_checkpoint_marker);
    }
    pub unsafe fn get_queue_checkpoint_data_nv_to_vec(&self, queue: vk::Queue) -> Vec<vk::CheckpointDataNV> {
        let mut len = mem::uninitialized();
        (self.fp1_0.get_queue_checkpoint_data_nv)(Some(queue), &mut len, ptr::null_mut());
        let mut v = Vec::with_capacity(len as usize);
        (self.fp1_0.get_queue_checkpoint_data_nv)(Some(queue), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
    }
}
/// Loader for the `VK_FUCHSIA_imagepipe_surface` instance extension
pub struct FuchsiaImagepipeSurface {
    pub version: vk::Version,
    pub handle: vk::Instance,
    pub fp1_0: vk::FuchsiaImagepipeSurfaceFn1_0,
}
impl FuchsiaImagepipeSurface {
    pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {
        let lib = LIB.as_ref().map_err(|e| (*e).clone())?;
        let f = |name: &CStr| {
            lib.get_instance_proc_addr(Some(instance.handle), name)
                .map(|p| mem::transmute(p))
        };
        let mut version = vk::Version::from_raw(0);
        let mut ok = true;
        let (fp1_0, ok1_0) = vk::FuchsiaImagepipeSurfaceFn1_0::load(f);
        ok = ok && ok1_0;
        if ok {
            version = vk::Version::from_raw_parts(1, 0, 0);
        }
        Ok(Self {
            version,
            handle: instance.handle,
            fp1_0,
        })
    }
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_FUCHSIA_imagepipe_surface\0").unwrap()
    }
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        p_create_info: &vk::ImagePipeSurfaceCreateInfoFUCHSIA,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp1_0.create_image_pipe_surface_fuchsia)(
            Some(self.handle),
            p_create_info,
            p_allocator.map_or(ptr::null(), |r| r),
            &mut res,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerate_instance_version() {
        let loader = Loader::new().unwrap();
        let v = unsafe { loader.enumerate_instance_version() };
        assert!(v.is_ok());
    }
}
