//! Generated from vk.xml with `VK_HEADER_VERSION` 107
pub mod builder;
pub mod vk;

use lazy_static::lazy_static;
use shared_library::dynamic_library::DynamicLibrary;
use std::ffi::CStr;
use std::mem;
use std::os::raw::{c_int, c_void};
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

struct Lib {
    pub lib: DynamicLibrary,
    pub get_instance_proc_addr: vk::FnGetInstanceProcAddr,
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

pub type LoaderResult<T> = result::Result<T, LoaderError>;

#[cfg(unix)]
const DL_PATH: &'static str = "libvulkan.so.1";

#[cfg(windows)]
const DL_PATH: &'static str = "vulkan-1.dll";

impl Lib {
    pub fn new() -> LoaderResult<Self> {
        match DynamicLibrary::open(Some(&Path::new(&DL_PATH))) {
            Ok(lib) => match unsafe {
                lib.symbol("vkGetInstanceProcAddr")
                    .map(|f: *mut c_void| mem::transmute(f))
            } {
                Ok(get_instance_proc_addr) => Ok(Self {
                    lib,
                    get_instance_proc_addr,
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
        (self.get_instance_proc_addr)(instance, name.as_ptr())
    }
}

lazy_static! {
    static ref LIB: LoaderResult<Lib> = Lib::new();
}
pub struct Loader {
    pub fp_create_instance: Option<vk::FnCreateInstance>,
    pub fp_get_instance_proc_addr: Option<vk::FnGetInstanceProcAddr>,
    pub fp_enumerate_instance_version: Option<vk::FnEnumerateInstanceVersion>,
    pub fp_enumerate_instance_layer_properties: Option<vk::FnEnumerateInstanceLayerProperties>,
    pub fp_enumerate_instance_extension_properties: Option<vk::FnEnumerateInstanceExtensionProperties>,
}
impl Loader {
    pub fn new() -> LoaderResult<Self> {
        let lib = LIB.as_ref().map_err(|e| e.clone())?;
        unsafe {
            let f = |name: &CStr| lib.get_instance_proc_addr(None, name);
            Ok(Self {
                fp_create_instance: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateInstance\0"))
                    .map(|f| mem::transmute(f)),
                fp_get_instance_proc_addr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetInstanceProcAddr\0"))
                    .map(|f| mem::transmute(f)),
                fp_enumerate_instance_version: f(CStr::from_bytes_with_nul_unchecked(b"vkEnumerateInstanceVersion\0"))
                    .map(|f| mem::transmute(f)),
                fp_enumerate_instance_layer_properties: f(CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateInstanceLayerProperties\0",
                ))
                .map(|f| mem::transmute(f)),
                fp_enumerate_instance_extension_properties: f(CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateInstanceExtensionProperties\0",
                ))
                .map(|f| mem::transmute(f)),
            })
        }
    }
    pub unsafe fn create_instance(
        &self,
        p_create_info: &vk::InstanceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> result::Result<Instance, LoaderError> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_instance.unwrap())(p_create_info, p_allocator.map_or(ptr::null(), |r| r), &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res.map_err(|e| LoaderError::Vulkan(e)).and_then(|r| Instance::load(r))
    }
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Option<vk::Instance>,
        p_name: &CStr,
    ) -> Option<vk::FnVoidFunction> {
        let res = (self.fp_get_instance_proc_addr.unwrap())(instance, p_name.as_ptr());
        res
    }
    pub unsafe fn enumerate_instance_version(&self) -> Result<vk::Version> {
        let mut res = mem::uninitialized();
        let err = (self.fp_enumerate_instance_version.unwrap())(&mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn enumerate_instance_layer_properties_to_vec(&self) -> Result<Vec<vk::LayerProperties>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_enumerate_instance_layer_properties.unwrap())(&mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_enumerate_instance_layer_properties.unwrap())(&mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn enumerate_instance_extension_properties_to_vec(
        &self,
        p_layer_name: &CStr,
    ) -> Result<Vec<vk::ExtensionProperties>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_enumerate_instance_extension_properties.unwrap())(
            p_layer_name.as_ptr(),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err =
            (self.fp_enumerate_instance_extension_properties.unwrap())(p_layer_name.as_ptr(), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
}
pub struct Instance {
    pub handle: vk::Instance,
    pub fp_destroy_instance: Option<vk::FnDestroyInstance>,
    pub fp_enumerate_physical_devices: Option<vk::FnEnumeratePhysicalDevices>,
    pub fp_get_device_proc_addr: Option<vk::FnGetDeviceProcAddr>,
    pub fp_get_physical_device_properties: Option<vk::FnGetPhysicalDeviceProperties>,
    pub fp_get_physical_device_queue_family_properties: Option<vk::FnGetPhysicalDeviceQueueFamilyProperties>,
    pub fp_get_physical_device_memory_properties: Option<vk::FnGetPhysicalDeviceMemoryProperties>,
    pub fp_get_physical_device_features: Option<vk::FnGetPhysicalDeviceFeatures>,
    pub fp_get_physical_device_format_properties: Option<vk::FnGetPhysicalDeviceFormatProperties>,
    pub fp_get_physical_device_image_format_properties: Option<vk::FnGetPhysicalDeviceImageFormatProperties>,
    pub fp_create_device: Option<vk::FnCreateDevice>,
    pub fp_enumerate_device_layer_properties: Option<vk::FnEnumerateDeviceLayerProperties>,
    pub fp_enumerate_device_extension_properties: Option<vk::FnEnumerateDeviceExtensionProperties>,
    pub fp_get_physical_device_sparse_image_format_properties:
        Option<vk::FnGetPhysicalDeviceSparseImageFormatProperties>,
    pub fp_create_android_surface_khr: Option<vk::FnCreateAndroidSurfaceKHR>,
    pub fp_get_physical_device_display_properties_khr: Option<vk::FnGetPhysicalDeviceDisplayPropertiesKHR>,
    pub fp_get_physical_device_display_plane_properties_khr: Option<vk::FnGetPhysicalDeviceDisplayPlanePropertiesKHR>,
    pub fp_get_display_plane_supported_displays_khr: Option<vk::FnGetDisplayPlaneSupportedDisplaysKHR>,
    pub fp_get_display_mode_properties_khr: Option<vk::FnGetDisplayModePropertiesKHR>,
    pub fp_create_display_mode_khr: Option<vk::FnCreateDisplayModeKHR>,
    pub fp_get_display_plane_capabilities_khr: Option<vk::FnGetDisplayPlaneCapabilitiesKHR>,
    pub fp_create_display_plane_surface_khr: Option<vk::FnCreateDisplayPlaneSurfaceKHR>,
    pub fp_destroy_surface_khr: Option<vk::FnDestroySurfaceKHR>,
    pub fp_get_physical_device_surface_support_khr: Option<vk::FnGetPhysicalDeviceSurfaceSupportKHR>,
    pub fp_get_physical_device_surface_capabilities_khr: Option<vk::FnGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    pub fp_get_physical_device_surface_formats_khr: Option<vk::FnGetPhysicalDeviceSurfaceFormatsKHR>,
    pub fp_get_physical_device_surface_present_modes_khr: Option<vk::FnGetPhysicalDeviceSurfacePresentModesKHR>,
    pub fp_create_vi_surface_nn: Option<vk::FnCreateViSurfaceNN>,
    pub fp_create_wayland_surface_khr: Option<vk::FnCreateWaylandSurfaceKHR>,
    pub fp_get_physical_device_wayland_presentation_support_khr:
        Option<vk::FnGetPhysicalDeviceWaylandPresentationSupportKHR>,
    pub fp_create_win32_surface_khr: Option<vk::FnCreateWin32SurfaceKHR>,
    pub fp_get_physical_device_win32_presentation_support_khr:
        Option<vk::FnGetPhysicalDeviceWin32PresentationSupportKHR>,
    pub fp_create_xlib_surface_khr: Option<vk::FnCreateXlibSurfaceKHR>,
    pub fp_get_physical_device_xlib_presentation_support_khr: Option<vk::FnGetPhysicalDeviceXlibPresentationSupportKHR>,
    pub fp_create_xcb_surface_khr: Option<vk::FnCreateXcbSurfaceKHR>,
    pub fp_get_physical_device_xcb_presentation_support_khr: Option<vk::FnGetPhysicalDeviceXcbPresentationSupportKHR>,
    pub fp_create_image_pipe_surface_fuchsia: Option<vk::FnCreateImagePipeSurfaceFUCHSIA>,
    pub fp_create_debug_report_callback_ext: Option<vk::FnCreateDebugReportCallbackEXT>,
    pub fp_destroy_debug_report_callback_ext: Option<vk::FnDestroyDebugReportCallbackEXT>,
    pub fp_debug_report_message_ext: Option<vk::FnDebugReportMessageEXT>,
    pub fp_get_physical_device_external_image_format_properties_nv:
        Option<vk::FnGetPhysicalDeviceExternalImageFormatPropertiesNV>,
    pub fp_get_physical_device_features2: Option<vk::FnGetPhysicalDeviceFeatures2>,
    pub fp_get_physical_device_features2_khr: Option<vk::FnGetPhysicalDeviceFeatures2KHR>,
    pub fp_get_physical_device_properties2: Option<vk::FnGetPhysicalDeviceProperties2>,
    pub fp_get_physical_device_properties2_khr: Option<vk::FnGetPhysicalDeviceProperties2KHR>,
    pub fp_get_physical_device_format_properties2: Option<vk::FnGetPhysicalDeviceFormatProperties2>,
    pub fp_get_physical_device_format_properties2_khr: Option<vk::FnGetPhysicalDeviceFormatProperties2KHR>,
    pub fp_get_physical_device_image_format_properties2: Option<vk::FnGetPhysicalDeviceImageFormatProperties2>,
    pub fp_get_physical_device_image_format_properties2_khr: Option<vk::FnGetPhysicalDeviceImageFormatProperties2KHR>,
    pub fp_get_physical_device_queue_family_properties2: Option<vk::FnGetPhysicalDeviceQueueFamilyProperties2>,
    pub fp_get_physical_device_queue_family_properties2_khr: Option<vk::FnGetPhysicalDeviceQueueFamilyProperties2KHR>,
    pub fp_get_physical_device_memory_properties2: Option<vk::FnGetPhysicalDeviceMemoryProperties2>,
    pub fp_get_physical_device_memory_properties2_khr: Option<vk::FnGetPhysicalDeviceMemoryProperties2KHR>,
    pub fp_get_physical_device_sparse_image_format_properties2:
        Option<vk::FnGetPhysicalDeviceSparseImageFormatProperties2>,
    pub fp_get_physical_device_sparse_image_format_properties2_khr:
        Option<vk::FnGetPhysicalDeviceSparseImageFormatProperties2KHR>,
    pub fp_get_physical_device_external_buffer_properties: Option<vk::FnGetPhysicalDeviceExternalBufferProperties>,
    pub fp_get_physical_device_external_buffer_properties_khr:
        Option<vk::FnGetPhysicalDeviceExternalBufferPropertiesKHR>,
    pub fp_get_physical_device_external_semaphore_properties:
        Option<vk::FnGetPhysicalDeviceExternalSemaphoreProperties>,
    pub fp_get_physical_device_external_semaphore_properties_khr:
        Option<vk::FnGetPhysicalDeviceExternalSemaphorePropertiesKHR>,
    pub fp_get_physical_device_external_fence_properties: Option<vk::FnGetPhysicalDeviceExternalFenceProperties>,
    pub fp_get_physical_device_external_fence_properties_khr: Option<vk::FnGetPhysicalDeviceExternalFencePropertiesKHR>,
    pub fp_release_display_ext: Option<vk::FnReleaseDisplayEXT>,
    pub fp_acquire_xlib_display_ext: Option<vk::FnAcquireXlibDisplayEXT>,
    pub fp_get_rand_r_output_display_ext: Option<vk::FnGetRandROutputDisplayEXT>,
    pub fp_get_physical_device_surface_capabilities2_ext: Option<vk::FnGetPhysicalDeviceSurfaceCapabilities2EXT>,
    pub fp_enumerate_physical_device_groups: Option<vk::FnEnumeratePhysicalDeviceGroups>,
    pub fp_enumerate_physical_device_groups_khr: Option<vk::FnEnumeratePhysicalDeviceGroupsKHR>,
    pub fp_create_ios_surface_mvk: Option<vk::FnCreateIOSSurfaceMVK>,
    pub fp_create_mac_os_surface_mvk: Option<vk::FnCreateMacOSSurfaceMVK>,
    pub fp_create_metal_surface_ext: Option<vk::FnCreateMetalSurfaceEXT>,
    pub fp_get_physical_device_surface_capabilities2_khr: Option<vk::FnGetPhysicalDeviceSurfaceCapabilities2KHR>,
    pub fp_get_physical_device_surface_formats2_khr: Option<vk::FnGetPhysicalDeviceSurfaceFormats2KHR>,
    pub fp_get_physical_device_display_properties2_khr: Option<vk::FnGetPhysicalDeviceDisplayProperties2KHR>,
    pub fp_get_physical_device_display_plane_properties2_khr: Option<vk::FnGetPhysicalDeviceDisplayPlaneProperties2KHR>,
    pub fp_get_display_mode_properties2_khr: Option<vk::FnGetDisplayModeProperties2KHR>,
    pub fp_get_display_plane_capabilities2_khr: Option<vk::FnGetDisplayPlaneCapabilities2KHR>,
    pub fp_set_debug_utils_object_name_ext: Option<vk::FnSetDebugUtilsObjectNameEXT>,
    pub fp_set_debug_utils_object_tag_ext: Option<vk::FnSetDebugUtilsObjectTagEXT>,
    pub fp_queue_begin_debug_utils_label_ext: Option<vk::FnQueueBeginDebugUtilsLabelEXT>,
    pub fp_queue_end_debug_utils_label_ext: Option<vk::FnQueueEndDebugUtilsLabelEXT>,
    pub fp_queue_insert_debug_utils_label_ext: Option<vk::FnQueueInsertDebugUtilsLabelEXT>,
    pub fp_cmd_begin_debug_utils_label_ext: Option<vk::FnCmdBeginDebugUtilsLabelEXT>,
    pub fp_cmd_end_debug_utils_label_ext: Option<vk::FnCmdEndDebugUtilsLabelEXT>,
    pub fp_cmd_insert_debug_utils_label_ext: Option<vk::FnCmdInsertDebugUtilsLabelEXT>,
    pub fp_create_debug_utils_messenger_ext: Option<vk::FnCreateDebugUtilsMessengerEXT>,
    pub fp_destroy_debug_utils_messenger_ext: Option<vk::FnDestroyDebugUtilsMessengerEXT>,
    pub fp_submit_debug_utils_message_ext: Option<vk::FnSubmitDebugUtilsMessageEXT>,
    pub fp_create_headless_surface_ext: Option<vk::FnCreateHeadlessSurfaceEXT>,
}
impl Instance {
    unsafe fn load(instance: vk::Instance) -> LoaderResult<Self> {
        let lib = LIB.as_ref().map_err(|e| e.clone())?;
        let f = |name: &CStr| lib.get_instance_proc_addr(Some(instance), name);
        Ok(Self {
            handle: instance,
            fp_destroy_instance: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyInstance\0"))
                .map(|f| mem::transmute(f)),
            fp_enumerate_physical_devices: f(CStr::from_bytes_with_nul_unchecked(b"vkEnumeratePhysicalDevices\0"))
                .map(|f| mem::transmute(f)),
            fp_get_device_proc_addr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceProcAddr\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_queue_family_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceQueueFamilyProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_memory_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceMemoryProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_features: f(CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceFeatures\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_format_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceFormatProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_image_format_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceImageFormatProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_device: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDevice\0")).map(|f| mem::transmute(f)),
            fp_enumerate_device_layer_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkEnumerateDeviceLayerProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_enumerate_device_extension_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkEnumerateDeviceExtensionProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_sparse_image_format_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSparseImageFormatProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_android_surface_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateAndroidSurfaceKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_display_properties_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceDisplayPropertiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_display_plane_properties_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_display_plane_supported_displays_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDisplayPlaneSupportedDisplaysKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_display_mode_properties_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDisplayModePropertiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_display_mode_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDisplayModeKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_display_plane_capabilities_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDisplayPlaneCapabilitiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_display_plane_surface_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateDisplayPlaneSurfaceKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_destroy_surface_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroySurfaceKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_surface_support_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSurfaceSupportKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_surface_capabilities_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_surface_formats_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSurfaceFormatsKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_surface_present_modes_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSurfacePresentModesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_vi_surface_nn: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateViSurfaceNN\0"))
                .map(|f| mem::transmute(f)),
            fp_create_wayland_surface_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateWaylandSurfaceKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_wayland_presentation_support_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_win32_surface_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateWin32SurfaceKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_win32_presentation_support_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_xlib_surface_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateXlibSurfaceKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_xlib_presentation_support_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_xcb_surface_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateXcbSurfaceKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_xcb_presentation_support_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_image_pipe_surface_fuchsia: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateImagePipeSurfaceFUCHSIA\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_debug_report_callback_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateDebugReportCallbackEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_destroy_debug_report_callback_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDestroyDebugReportCallbackEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_debug_report_message_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkDebugReportMessageEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_external_image_format_properties_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_features2: f(CStr::from_bytes_with_nul_unchecked(b"vkGetPhysicalDeviceFeatures2\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_features2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceFeatures2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_properties2: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceProperties2\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_properties2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceProperties2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_format_properties2: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceFormatProperties2\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_format_properties2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceFormatProperties2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_image_format_properties2: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceImageFormatProperties2\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_image_format_properties2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceImageFormatProperties2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_queue_family_properties2: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceQueueFamilyProperties2\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_queue_family_properties2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceQueueFamilyProperties2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_memory_properties2: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceMemoryProperties2\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_memory_properties2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceMemoryProperties2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_sparse_image_format_properties2: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSparseImageFormatProperties2\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_sparse_image_format_properties2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_external_buffer_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceExternalBufferProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_external_buffer_properties_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceExternalBufferPropertiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_external_semaphore_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceExternalSemaphoreProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_external_semaphore_properties_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_external_fence_properties: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceExternalFenceProperties\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_external_fence_properties_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceExternalFencePropertiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_release_display_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkReleaseDisplayEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_acquire_xlib_display_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkAcquireXlibDisplayEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_rand_r_output_display_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkGetRandROutputDisplayEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_surface_capabilities2_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSurfaceCapabilities2EXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_enumerate_physical_device_groups: f(CStr::from_bytes_with_nul_unchecked(
                b"vkEnumeratePhysicalDeviceGroups\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_enumerate_physical_device_groups_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkEnumeratePhysicalDeviceGroupsKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_ios_surface_mvk: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateIOSSurfaceMVK\0"))
                .map(|f| mem::transmute(f)),
            fp_create_mac_os_surface_mvk: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateMacOSSurfaceMVK\0"))
                .map(|f| mem::transmute(f)),
            fp_create_metal_surface_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateMetalSurfaceEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_surface_capabilities2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSurfaceCapabilities2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_surface_formats2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSurfaceFormats2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_display_properties2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceDisplayProperties2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_physical_device_display_plane_properties2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceDisplayPlaneProperties2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_display_mode_properties2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDisplayModeProperties2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_display_plane_capabilities2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDisplayPlaneCapabilities2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_set_debug_utils_object_name_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkSetDebugUtilsObjectNameEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_set_debug_utils_object_tag_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkSetDebugUtilsObjectTagEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_queue_begin_debug_utils_label_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkQueueBeginDebugUtilsLabelEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_queue_end_debug_utils_label_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkQueueEndDebugUtilsLabelEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_queue_insert_debug_utils_label_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkQueueInsertDebugUtilsLabelEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_begin_debug_utils_label_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdBeginDebugUtilsLabelEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_end_debug_utils_label_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndDebugUtilsLabelEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_insert_debug_utils_label_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdInsertDebugUtilsLabelEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_debug_utils_messenger_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateDebugUtilsMessengerEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_destroy_debug_utils_messenger_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDestroyDebugUtilsMessengerEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_submit_debug_utils_message_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkSubmitDebugUtilsMessageEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_headless_surface_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateHeadlessSurfaceEXT\0"))
                .map(|f| mem::transmute(f)),
        })
    }
    pub unsafe fn destroy_instance(&self, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp_destroy_instance.unwrap())(Some(self.handle), p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn enumerate_physical_devices_to_vec(&self) -> Result<Vec<vk::PhysicalDevice>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_enumerate_physical_devices.unwrap())(Some(self.handle), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_enumerate_physical_devices.unwrap())(Some(self.handle), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn get_device_proc_addr(&self, device: vk::Device, p_name: &CStr) -> Option<vk::FnVoidFunction> {
        let res = (self.fp_get_device_proc_addr.unwrap())(Some(device), p_name.as_ptr());
        res
    }
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        let mut res = mem::uninitialized();
        (self.fp_get_physical_device_properties.unwrap())(Some(physical_device), &mut res);
        res
    }
    pub unsafe fn get_physical_device_queue_family_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties> {
        let mut len = mem::uninitialized();
        (self.fp_get_physical_device_queue_family_properties.unwrap())(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp_get_physical_device_queue_family_properties.unwrap())(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties {
        let mut res = mem::uninitialized();
        (self.fp_get_physical_device_memory_properties.unwrap())(Some(physical_device), &mut res);
        res
    }
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceFeatures {
        let mut res = mem::uninitialized();
        (self.fp_get_physical_device_features.unwrap())(Some(physical_device), &mut res);
        res
    }
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
    ) -> vk::FormatProperties {
        let mut res = mem::uninitialized();
        (self.fp_get_physical_device_format_properties.unwrap())(Some(physical_device), format, &mut res);
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
        let err = (self.fp_get_physical_device_image_format_properties.unwrap())(
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
    pub unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        p_create_info: &vk::DeviceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> result::Result<Device, LoaderError> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_device.unwrap())(
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
    pub unsafe fn enumerate_device_layer_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::LayerProperties>> {
        let mut len = mem::uninitialized();
        let len_err =
            (self.fp_enumerate_device_layer_properties.unwrap())(Some(physical_device), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err =
            (self.fp_enumerate_device_layer_properties.unwrap())(Some(physical_device), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn enumerate_device_extension_properties_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_layer_name: &CStr,
    ) -> Result<Vec<vk::ExtensionProperties>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_enumerate_device_extension_properties.unwrap())(
            Some(physical_device),
            p_layer_name.as_ptr(),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_enumerate_device_extension_properties.unwrap())(
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
        (self.fp_get_physical_device_sparse_image_format_properties.unwrap())(
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
        (self.fp_get_physical_device_sparse_image_format_properties.unwrap())(
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
    pub unsafe fn create_android_surface_khr(
        &self,
        p_create_info: &vk::AndroidSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_android_surface_khr.unwrap())(
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
    pub unsafe fn get_physical_device_display_properties_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayPropertiesKHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_get_physical_device_display_properties_khr.unwrap())(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_display_properties_khr.unwrap())(
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
    pub unsafe fn get_physical_device_display_plane_properties_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayPlanePropertiesKHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_get_physical_device_display_plane_properties_khr.unwrap())(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_display_plane_properties_khr.unwrap())(
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
        let len_err = (self.fp_get_display_plane_supported_displays_khr.unwrap())(
            Some(physical_device),
            plane_index,
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_display_plane_supported_displays_khr.unwrap())(
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
        let len_err = (self.fp_get_display_mode_properties_khr.unwrap())(
            Some(physical_device),
            Some(display),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_display_mode_properties_khr.unwrap())(
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
        let err = (self.fp_create_display_mode_khr.unwrap())(
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
        let err = (self.fp_get_display_plane_capabilities_khr.unwrap())(
            Some(physical_device),
            Some(mode),
            plane_index,
            &mut res,
        );
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
        let err = (self.fp_create_display_plane_surface_khr.unwrap())(
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
    pub unsafe fn destroy_surface_khr(
        &self,
        surface: Option<vk::SurfaceKHR>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp_destroy_surface_khr.unwrap())(Some(self.handle), surface, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        surface: vk::SurfaceKHR,
    ) -> Result<bool> {
        let mut res = mem::uninitialized();
        let err = (self.fp_get_physical_device_surface_support_khr.unwrap())(
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
        let err = (self.fp_get_physical_device_surface_capabilities_khr.unwrap())(
            Some(physical_device),
            Some(surface),
            &mut res,
        );
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
        let len_err = (self.fp_get_physical_device_surface_formats_khr.unwrap())(
            Some(physical_device),
            Some(surface),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_surface_formats_khr.unwrap())(
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
        let len_err = (self.fp_get_physical_device_surface_present_modes_khr.unwrap())(
            Some(physical_device),
            Some(surface),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_surface_present_modes_khr.unwrap())(
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
    pub unsafe fn create_vi_surface_nn(
        &self,
        p_create_info: &vk::ViSurfaceCreateInfoNN,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_vi_surface_nn.unwrap())(
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
    pub unsafe fn create_wayland_surface_khr(
        &self,
        p_create_info: &vk::WaylandSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_wayland_surface_khr.unwrap())(
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
        let res = (self.fp_get_physical_device_wayland_presentation_support_khr.unwrap())(
            Some(physical_device),
            queue_family_index,
            display,
        );
        res
    }
    pub unsafe fn create_win32_surface_khr(
        &self,
        p_create_info: &vk::Win32SurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_win32_surface_khr.unwrap())(
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
        let res = (self.fp_get_physical_device_win32_presentation_support_khr.unwrap())(
            Some(physical_device),
            queue_family_index,
        );
        res
    }
    pub unsafe fn create_xlib_surface_khr(
        &self,
        p_create_info: &vk::XlibSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_xlib_surface_khr.unwrap())(
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
        let res = (self.fp_get_physical_device_xlib_presentation_support_khr.unwrap())(
            Some(physical_device),
            queue_family_index,
            dpy,
            visual_id,
        );
        res
    }
    pub unsafe fn create_xcb_surface_khr(
        &self,
        p_create_info: &vk::XcbSurfaceCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_xcb_surface_khr.unwrap())(
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
        let res = (self.fp_get_physical_device_xcb_presentation_support_khr.unwrap())(
            Some(physical_device),
            queue_family_index,
            connection,
            visual_id,
        );
        res
    }
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        p_create_info: &vk::ImagePipeSurfaceCreateInfoFUCHSIA,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_image_pipe_surface_fuchsia.unwrap())(
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
    pub unsafe fn create_debug_report_callback_ext(
        &self,
        p_create_info: &vk::DebugReportCallbackCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DebugReportCallbackEXT> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_debug_report_callback_ext.unwrap())(
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
        (self.fp_destroy_debug_report_callback_ext.unwrap())(
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
        (self.fp_debug_report_message_ext.unwrap())(
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
        let err = (self.fp_get_physical_device_external_image_format_properties_nv.unwrap())(
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
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_features: &mut vk::PhysicalDeviceFeatures2,
    ) {
        (self.fp_get_physical_device_features2.unwrap())(Some(physical_device), p_features);
    }
    pub unsafe fn get_physical_device_features2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_features: &mut vk::PhysicalDeviceFeatures2,
    ) {
        (self.fp_get_physical_device_features2_khr.unwrap())(Some(physical_device), p_features);
    }
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_properties: &mut vk::PhysicalDeviceProperties2,
    ) {
        (self.fp_get_physical_device_properties2.unwrap())(Some(physical_device), p_properties);
    }
    pub unsafe fn get_physical_device_properties2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_properties: &mut vk::PhysicalDeviceProperties2,
    ) {
        (self.fp_get_physical_device_properties2_khr.unwrap())(Some(physical_device), p_properties);
    }
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        p_format_properties: &mut vk::FormatProperties2,
    ) {
        (self.fp_get_physical_device_format_properties2.unwrap())(Some(physical_device), format, p_format_properties);
    }
    pub unsafe fn get_physical_device_format_properties2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        p_format_properties: &mut vk::FormatProperties2,
    ) {
        (self.fp_get_physical_device_format_properties2_khr.unwrap())(
            Some(physical_device),
            format,
            p_format_properties,
        );
    }
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_image_format_info: &vk::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: &mut vk::ImageFormatProperties2,
    ) -> Result<()> {
        let err = (self.fp_get_physical_device_image_format_properties2.unwrap())(
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
    pub unsafe fn get_physical_device_image_format_properties2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_image_format_info: &vk::PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: &mut vk::ImageFormatProperties2,
    ) -> Result<()> {
        let err = (self.fp_get_physical_device_image_format_properties2_khr.unwrap())(
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
        (self.fp_get_physical_device_queue_family_properties2.unwrap())(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp_get_physical_device_queue_family_properties2.unwrap())(
            Some(physical_device),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn get_physical_device_queue_family_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties2> {
        let mut len = mem::uninitialized();
        (self.fp_get_physical_device_queue_family_properties2_khr.unwrap())(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp_get_physical_device_queue_family_properties2_khr.unwrap())(
            Some(physical_device),
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        p_memory_properties: &mut vk::PhysicalDeviceMemoryProperties2,
    ) {
        (self.fp_get_physical_device_memory_properties2.unwrap())(Some(physical_device), p_memory_properties);
    }
    pub unsafe fn get_physical_device_memory_properties2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_memory_properties: &mut vk::PhysicalDeviceMemoryProperties2,
    ) {
        (self.fp_get_physical_device_memory_properties2_khr.unwrap())(Some(physical_device), p_memory_properties);
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
    ) -> Vec<vk::SparseImageFormatProperties2> {
        let mut len = mem::uninitialized();
        (self.fp_get_physical_device_sparse_image_format_properties2.unwrap())(
            Some(physical_device),
            p_format_info,
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp_get_physical_device_sparse_image_format_properties2.unwrap())(
            Some(physical_device),
            p_format_info,
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
    ) -> Vec<vk::SparseImageFormatProperties2> {
        let mut len = mem::uninitialized();
        (self.fp_get_physical_device_sparse_image_format_properties2_khr.unwrap())(
            Some(physical_device),
            p_format_info,
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp_get_physical_device_sparse_image_format_properties2_khr.unwrap())(
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
        (self.fp_get_physical_device_external_buffer_properties.unwrap())(
            Some(physical_device),
            p_external_buffer_info,
            p_external_buffer_properties,
        );
    }
    pub unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_buffer_info: &vk::PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: &mut vk::ExternalBufferProperties,
    ) {
        (self.fp_get_physical_device_external_buffer_properties_khr.unwrap())(
            Some(physical_device),
            p_external_buffer_info,
            p_external_buffer_properties,
        );
    }
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: &mut vk::ExternalSemaphoreProperties,
    ) {
        (self.fp_get_physical_device_external_semaphore_properties.unwrap())(
            Some(physical_device),
            p_external_semaphore_info,
            p_external_semaphore_properties,
        );
    }
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: &mut vk::ExternalSemaphoreProperties,
    ) {
        (self.fp_get_physical_device_external_semaphore_properties_khr.unwrap())(
            Some(physical_device),
            p_external_semaphore_info,
            p_external_semaphore_properties,
        );
    }
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_fence_info: &vk::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: &mut vk::ExternalFenceProperties,
    ) {
        (self.fp_get_physical_device_external_fence_properties.unwrap())(
            Some(physical_device),
            p_external_fence_info,
            p_external_fence_properties,
        );
    }
    pub unsafe fn get_physical_device_external_fence_properties_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_external_fence_info: &vk::PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: &mut vk::ExternalFenceProperties,
    ) {
        (self.fp_get_physical_device_external_fence_properties_khr.unwrap())(
            Some(physical_device),
            p_external_fence_info,
            p_external_fence_properties,
        );
    }
    pub unsafe fn release_display_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> Result<()> {
        let err = (self.fp_release_display_ext.unwrap())(Some(physical_device), Some(display));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        dpy: &mut vk::Display,
        display: vk::DisplayKHR,
    ) -> Result<()> {
        let err = (self.fp_acquire_xlib_display_ext.unwrap())(Some(physical_device), dpy, Some(display));
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
        let err = (self.fp_get_rand_r_output_display_ext.unwrap())(Some(physical_device), dpy, rr_output, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_physical_device_surface_capabilities2_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        p_surface_capabilities: &mut vk::SurfaceCapabilities2EXT,
    ) -> Result<()> {
        let err = (self.fp_get_physical_device_surface_capabilities2_ext.unwrap())(
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
    pub unsafe fn enumerate_physical_device_groups_to_vec(&self) -> Result<Vec<vk::PhysicalDeviceGroupProperties>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_enumerate_physical_device_groups.unwrap())(Some(self.handle), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_enumerate_physical_device_groups.unwrap())(Some(self.handle), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn enumerate_physical_device_groups_khr_to_vec(&self) -> Result<Vec<vk::PhysicalDeviceGroupProperties>> {
        let mut len = mem::uninitialized();
        let len_err =
            (self.fp_enumerate_physical_device_groups_khr.unwrap())(Some(self.handle), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err =
            (self.fp_enumerate_physical_device_groups_khr.unwrap())(Some(self.handle), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn create_ios_surface_mvk(
        &self,
        p_create_info: &vk::IOSSurfaceCreateInfoMVK,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_ios_surface_mvk.unwrap())(
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
    pub unsafe fn create_mac_os_surface_mvk(
        &self,
        p_create_info: &vk::MacOSSurfaceCreateInfoMVK,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_mac_os_surface_mvk.unwrap())(
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
    pub unsafe fn create_metal_surface_ext(
        &self,
        p_create_info: &vk::MetalSurfaceCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_metal_surface_ext.unwrap())(
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
    pub unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
        p_surface_capabilities: &mut vk::SurfaceCapabilities2KHR,
    ) -> Result<()> {
        let err = (self.fp_get_physical_device_surface_capabilities2_khr.unwrap())(
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
        let len_err = (self.fp_get_physical_device_surface_formats2_khr.unwrap())(
            Some(physical_device),
            p_surface_info,
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_surface_formats2_khr.unwrap())(
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
    pub unsafe fn get_physical_device_display_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayProperties2KHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_get_physical_device_display_properties2_khr.unwrap())(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_display_properties2_khr.unwrap())(
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
    pub unsafe fn get_physical_device_display_plane_properties2_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::DisplayPlaneProperties2KHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_get_physical_device_display_plane_properties2_khr.unwrap())(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_display_plane_properties2_khr.unwrap())(
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
        let len_err = (self.fp_get_display_mode_properties2_khr.unwrap())(
            Some(physical_device),
            Some(display),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_display_mode_properties2_khr.unwrap())(
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
        let err = (self.fp_get_display_plane_capabilities2_khr.unwrap())(
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
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: vk::Device,
        p_name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) -> Result<()> {
        let err = (self.fp_set_debug_utils_object_name_ext.unwrap())(Some(device), p_name_info);
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
        let err = (self.fp_set_debug_utils_object_tag_ext.unwrap())(Some(device), p_tag_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn queue_begin_debug_utils_label_ext(&self, queue: vk::Queue, p_label_info: &vk::DebugUtilsLabelEXT) {
        (self.fp_queue_begin_debug_utils_label_ext.unwrap())(Some(queue), p_label_info);
    }
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: vk::Queue) {
        (self.fp_queue_end_debug_utils_label_ext.unwrap())(Some(queue));
    }
    pub unsafe fn queue_insert_debug_utils_label_ext(&self, queue: vk::Queue, p_label_info: &vk::DebugUtilsLabelEXT) {
        (self.fp_queue_insert_debug_utils_label_ext.unwrap())(Some(queue), p_label_info);
    }
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_label_info: &vk::DebugUtilsLabelEXT,
    ) {
        (self.fp_cmd_begin_debug_utils_label_ext.unwrap())(Some(command_buffer), p_label_info);
    }
    pub unsafe fn cmd_end_debug_utils_label_ext(&self, command_buffer: vk::CommandBuffer) {
        (self.fp_cmd_end_debug_utils_label_ext.unwrap())(Some(command_buffer));
    }
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_label_info: &vk::DebugUtilsLabelEXT,
    ) {
        (self.fp_cmd_insert_debug_utils_label_ext.unwrap())(Some(command_buffer), p_label_info);
    }
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        p_create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DebugUtilsMessengerEXT> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_debug_utils_messenger_ext.unwrap())(
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
        (self.fp_destroy_debug_utils_messenger_ext.unwrap())(
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
        (self.fp_submit_debug_utils_message_ext.unwrap())(
            Some(self.handle),
            message_severity,
            message_types,
            p_callback_data,
        );
    }
    pub unsafe fn create_headless_surface_ext(
        &self,
        p_create_info: &vk::HeadlessSurfaceCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SurfaceKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_headless_surface_ext.unwrap())(
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
pub struct Device {
    pub handle: vk::Device,
    pub fp_destroy_device: Option<vk::FnDestroyDevice>,
    pub fp_get_device_queue: Option<vk::FnGetDeviceQueue>,
    pub fp_queue_submit: Option<vk::FnQueueSubmit>,
    pub fp_queue_wait_idle: Option<vk::FnQueueWaitIdle>,
    pub fp_device_wait_idle: Option<vk::FnDeviceWaitIdle>,
    pub fp_allocate_memory: Option<vk::FnAllocateMemory>,
    pub fp_free_memory: Option<vk::FnFreeMemory>,
    pub fp_map_memory: Option<vk::FnMapMemory>,
    pub fp_unmap_memory: Option<vk::FnUnmapMemory>,
    pub fp_flush_mapped_memory_ranges: Option<vk::FnFlushMappedMemoryRanges>,
    pub fp_invalidate_mapped_memory_ranges: Option<vk::FnInvalidateMappedMemoryRanges>,
    pub fp_get_device_memory_commitment: Option<vk::FnGetDeviceMemoryCommitment>,
    pub fp_get_buffer_memory_requirements: Option<vk::FnGetBufferMemoryRequirements>,
    pub fp_bind_buffer_memory: Option<vk::FnBindBufferMemory>,
    pub fp_get_image_memory_requirements: Option<vk::FnGetImageMemoryRequirements>,
    pub fp_bind_image_memory: Option<vk::FnBindImageMemory>,
    pub fp_get_image_sparse_memory_requirements: Option<vk::FnGetImageSparseMemoryRequirements>,
    pub fp_queue_bind_sparse: Option<vk::FnQueueBindSparse>,
    pub fp_create_fence: Option<vk::FnCreateFence>,
    pub fp_destroy_fence: Option<vk::FnDestroyFence>,
    pub fp_reset_fences: Option<vk::FnResetFences>,
    pub fp_get_fence_status: Option<vk::FnGetFenceStatus>,
    pub fp_wait_for_fences: Option<vk::FnWaitForFences>,
    pub fp_create_semaphore: Option<vk::FnCreateSemaphore>,
    pub fp_destroy_semaphore: Option<vk::FnDestroySemaphore>,
    pub fp_create_event: Option<vk::FnCreateEvent>,
    pub fp_destroy_event: Option<vk::FnDestroyEvent>,
    pub fp_get_event_status: Option<vk::FnGetEventStatus>,
    pub fp_set_event: Option<vk::FnSetEvent>,
    pub fp_reset_event: Option<vk::FnResetEvent>,
    pub fp_create_query_pool: Option<vk::FnCreateQueryPool>,
    pub fp_destroy_query_pool: Option<vk::FnDestroyQueryPool>,
    pub fp_get_query_pool_results: Option<vk::FnGetQueryPoolResults>,
    pub fp_reset_query_pool_ext: Option<vk::FnResetQueryPoolEXT>,
    pub fp_create_buffer: Option<vk::FnCreateBuffer>,
    pub fp_destroy_buffer: Option<vk::FnDestroyBuffer>,
    pub fp_create_buffer_view: Option<vk::FnCreateBufferView>,
    pub fp_destroy_buffer_view: Option<vk::FnDestroyBufferView>,
    pub fp_create_image: Option<vk::FnCreateImage>,
    pub fp_destroy_image: Option<vk::FnDestroyImage>,
    pub fp_get_image_subresource_layout: Option<vk::FnGetImageSubresourceLayout>,
    pub fp_create_image_view: Option<vk::FnCreateImageView>,
    pub fp_destroy_image_view: Option<vk::FnDestroyImageView>,
    pub fp_create_shader_module: Option<vk::FnCreateShaderModule>,
    pub fp_destroy_shader_module: Option<vk::FnDestroyShaderModule>,
    pub fp_create_pipeline_cache: Option<vk::FnCreatePipelineCache>,
    pub fp_destroy_pipeline_cache: Option<vk::FnDestroyPipelineCache>,
    pub fp_get_pipeline_cache_data: Option<vk::FnGetPipelineCacheData>,
    pub fp_merge_pipeline_caches: Option<vk::FnMergePipelineCaches>,
    pub fp_create_graphics_pipelines: Option<vk::FnCreateGraphicsPipelines>,
    pub fp_create_compute_pipelines: Option<vk::FnCreateComputePipelines>,
    pub fp_destroy_pipeline: Option<vk::FnDestroyPipeline>,
    pub fp_create_pipeline_layout: Option<vk::FnCreatePipelineLayout>,
    pub fp_destroy_pipeline_layout: Option<vk::FnDestroyPipelineLayout>,
    pub fp_create_sampler: Option<vk::FnCreateSampler>,
    pub fp_destroy_sampler: Option<vk::FnDestroySampler>,
    pub fp_create_descriptor_set_layout: Option<vk::FnCreateDescriptorSetLayout>,
    pub fp_destroy_descriptor_set_layout: Option<vk::FnDestroyDescriptorSetLayout>,
    pub fp_create_descriptor_pool: Option<vk::FnCreateDescriptorPool>,
    pub fp_destroy_descriptor_pool: Option<vk::FnDestroyDescriptorPool>,
    pub fp_reset_descriptor_pool: Option<vk::FnResetDescriptorPool>,
    pub fp_allocate_descriptor_sets: Option<vk::FnAllocateDescriptorSets>,
    pub fp_free_descriptor_sets: Option<vk::FnFreeDescriptorSets>,
    pub fp_update_descriptor_sets: Option<vk::FnUpdateDescriptorSets>,
    pub fp_create_framebuffer: Option<vk::FnCreateFramebuffer>,
    pub fp_destroy_framebuffer: Option<vk::FnDestroyFramebuffer>,
    pub fp_create_render_pass: Option<vk::FnCreateRenderPass>,
    pub fp_destroy_render_pass: Option<vk::FnDestroyRenderPass>,
    pub fp_get_render_area_granularity: Option<vk::FnGetRenderAreaGranularity>,
    pub fp_create_command_pool: Option<vk::FnCreateCommandPool>,
    pub fp_destroy_command_pool: Option<vk::FnDestroyCommandPool>,
    pub fp_reset_command_pool: Option<vk::FnResetCommandPool>,
    pub fp_allocate_command_buffers: Option<vk::FnAllocateCommandBuffers>,
    pub fp_free_command_buffers: Option<vk::FnFreeCommandBuffers>,
    pub fp_begin_command_buffer: Option<vk::FnBeginCommandBuffer>,
    pub fp_end_command_buffer: Option<vk::FnEndCommandBuffer>,
    pub fp_reset_command_buffer: Option<vk::FnResetCommandBuffer>,
    pub fp_cmd_bind_pipeline: Option<vk::FnCmdBindPipeline>,
    pub fp_cmd_set_viewport: Option<vk::FnCmdSetViewport>,
    pub fp_cmd_set_scissor: Option<vk::FnCmdSetScissor>,
    pub fp_cmd_set_line_width: Option<vk::FnCmdSetLineWidth>,
    pub fp_cmd_set_depth_bias: Option<vk::FnCmdSetDepthBias>,
    pub fp_cmd_set_blend_constants: Option<vk::FnCmdSetBlendConstants>,
    pub fp_cmd_set_depth_bounds: Option<vk::FnCmdSetDepthBounds>,
    pub fp_cmd_set_stencil_compare_mask: Option<vk::FnCmdSetStencilCompareMask>,
    pub fp_cmd_set_stencil_write_mask: Option<vk::FnCmdSetStencilWriteMask>,
    pub fp_cmd_set_stencil_reference: Option<vk::FnCmdSetStencilReference>,
    pub fp_cmd_bind_descriptor_sets: Option<vk::FnCmdBindDescriptorSets>,
    pub fp_cmd_bind_index_buffer: Option<vk::FnCmdBindIndexBuffer>,
    pub fp_cmd_bind_vertex_buffers: Option<vk::FnCmdBindVertexBuffers>,
    pub fp_cmd_draw: Option<vk::FnCmdDraw>,
    pub fp_cmd_draw_indexed: Option<vk::FnCmdDrawIndexed>,
    pub fp_cmd_draw_indirect: Option<vk::FnCmdDrawIndirect>,
    pub fp_cmd_draw_indexed_indirect: Option<vk::FnCmdDrawIndexedIndirect>,
    pub fp_cmd_dispatch: Option<vk::FnCmdDispatch>,
    pub fp_cmd_dispatch_indirect: Option<vk::FnCmdDispatchIndirect>,
    pub fp_cmd_copy_buffer: Option<vk::FnCmdCopyBuffer>,
    pub fp_cmd_copy_image: Option<vk::FnCmdCopyImage>,
    pub fp_cmd_blit_image: Option<vk::FnCmdBlitImage>,
    pub fp_cmd_copy_buffer_to_image: Option<vk::FnCmdCopyBufferToImage>,
    pub fp_cmd_copy_image_to_buffer: Option<vk::FnCmdCopyImageToBuffer>,
    pub fp_cmd_update_buffer: Option<vk::FnCmdUpdateBuffer>,
    pub fp_cmd_fill_buffer: Option<vk::FnCmdFillBuffer>,
    pub fp_cmd_clear_color_image: Option<vk::FnCmdClearColorImage>,
    pub fp_cmd_clear_depth_stencil_image: Option<vk::FnCmdClearDepthStencilImage>,
    pub fp_cmd_clear_attachments: Option<vk::FnCmdClearAttachments>,
    pub fp_cmd_resolve_image: Option<vk::FnCmdResolveImage>,
    pub fp_cmd_set_event: Option<vk::FnCmdSetEvent>,
    pub fp_cmd_reset_event: Option<vk::FnCmdResetEvent>,
    pub fp_cmd_wait_events: Option<vk::FnCmdWaitEvents>,
    pub fp_cmd_pipeline_barrier: Option<vk::FnCmdPipelineBarrier>,
    pub fp_cmd_begin_query: Option<vk::FnCmdBeginQuery>,
    pub fp_cmd_end_query: Option<vk::FnCmdEndQuery>,
    pub fp_cmd_begin_conditional_rendering_ext: Option<vk::FnCmdBeginConditionalRenderingEXT>,
    pub fp_cmd_end_conditional_rendering_ext: Option<vk::FnCmdEndConditionalRenderingEXT>,
    pub fp_cmd_reset_query_pool: Option<vk::FnCmdResetQueryPool>,
    pub fp_cmd_write_timestamp: Option<vk::FnCmdWriteTimestamp>,
    pub fp_cmd_copy_query_pool_results: Option<vk::FnCmdCopyQueryPoolResults>,
    pub fp_cmd_push_constants: Option<vk::FnCmdPushConstants>,
    pub fp_cmd_begin_render_pass: Option<vk::FnCmdBeginRenderPass>,
    pub fp_cmd_next_subpass: Option<vk::FnCmdNextSubpass>,
    pub fp_cmd_end_render_pass: Option<vk::FnCmdEndRenderPass>,
    pub fp_cmd_execute_commands: Option<vk::FnCmdExecuteCommands>,
    pub fp_create_shared_swapchains_khr: Option<vk::FnCreateSharedSwapchainsKHR>,
    pub fp_create_swapchain_khr: Option<vk::FnCreateSwapchainKHR>,
    pub fp_destroy_swapchain_khr: Option<vk::FnDestroySwapchainKHR>,
    pub fp_get_swapchain_images_khr: Option<vk::FnGetSwapchainImagesKHR>,
    pub fp_acquire_next_image_khr: Option<vk::FnAcquireNextImageKHR>,
    pub fp_queue_present_khr: Option<vk::FnQueuePresentKHR>,
    pub fp_debug_marker_set_object_name_ext: Option<vk::FnDebugMarkerSetObjectNameEXT>,
    pub fp_debug_marker_set_object_tag_ext: Option<vk::FnDebugMarkerSetObjectTagEXT>,
    pub fp_cmd_debug_marker_begin_ext: Option<vk::FnCmdDebugMarkerBeginEXT>,
    pub fp_cmd_debug_marker_end_ext: Option<vk::FnCmdDebugMarkerEndEXT>,
    pub fp_cmd_debug_marker_insert_ext: Option<vk::FnCmdDebugMarkerInsertEXT>,
    pub fp_get_memory_win32_handle_nv: Option<vk::FnGetMemoryWin32HandleNV>,
    pub fp_cmd_process_commands_nvx: Option<vk::FnCmdProcessCommandsNVX>,
    pub fp_cmd_reserve_space_for_commands_nvx: Option<vk::FnCmdReserveSpaceForCommandsNVX>,
    pub fp_create_indirect_commands_layout_nvx: Option<vk::FnCreateIndirectCommandsLayoutNVX>,
    pub fp_destroy_indirect_commands_layout_nvx: Option<vk::FnDestroyIndirectCommandsLayoutNVX>,
    pub fp_create_object_table_nvx: Option<vk::FnCreateObjectTableNVX>,
    pub fp_destroy_object_table_nvx: Option<vk::FnDestroyObjectTableNVX>,
    pub fp_register_objects_nvx: Option<vk::FnRegisterObjectsNVX>,
    pub fp_unregister_objects_nvx: Option<vk::FnUnregisterObjectsNVX>,
    pub fp_get_physical_device_generated_commands_properties_nvx:
        Option<vk::FnGetPhysicalDeviceGeneratedCommandsPropertiesNVX>,
    pub fp_cmd_push_descriptor_set_khr: Option<vk::FnCmdPushDescriptorSetKHR>,
    pub fp_trim_command_pool: Option<vk::FnTrimCommandPool>,
    pub fp_trim_command_pool_khr: Option<vk::FnTrimCommandPoolKHR>,
    pub fp_get_memory_win32_handle_khr: Option<vk::FnGetMemoryWin32HandleKHR>,
    pub fp_get_memory_win32_handle_properties_khr: Option<vk::FnGetMemoryWin32HandlePropertiesKHR>,
    pub fp_get_memory_fd_khr: Option<vk::FnGetMemoryFdKHR>,
    pub fp_get_memory_fd_properties_khr: Option<vk::FnGetMemoryFdPropertiesKHR>,
    pub fp_get_semaphore_win32_handle_khr: Option<vk::FnGetSemaphoreWin32HandleKHR>,
    pub fp_import_semaphore_win32_handle_khr: Option<vk::FnImportSemaphoreWin32HandleKHR>,
    pub fp_get_semaphore_fd_khr: Option<vk::FnGetSemaphoreFdKHR>,
    pub fp_import_semaphore_fd_khr: Option<vk::FnImportSemaphoreFdKHR>,
    pub fp_get_fence_win32_handle_khr: Option<vk::FnGetFenceWin32HandleKHR>,
    pub fp_import_fence_win32_handle_khr: Option<vk::FnImportFenceWin32HandleKHR>,
    pub fp_get_fence_fd_khr: Option<vk::FnGetFenceFdKHR>,
    pub fp_import_fence_fd_khr: Option<vk::FnImportFenceFdKHR>,
    pub fp_display_power_control_ext: Option<vk::FnDisplayPowerControlEXT>,
    pub fp_register_device_event_ext: Option<vk::FnRegisterDeviceEventEXT>,
    pub fp_register_display_event_ext: Option<vk::FnRegisterDisplayEventEXT>,
    pub fp_get_swapchain_counter_ext: Option<vk::FnGetSwapchainCounterEXT>,
    pub fp_get_device_group_peer_memory_features: Option<vk::FnGetDeviceGroupPeerMemoryFeatures>,
    pub fp_get_device_group_peer_memory_features_khr: Option<vk::FnGetDeviceGroupPeerMemoryFeaturesKHR>,
    pub fp_bind_buffer_memory2: Option<vk::FnBindBufferMemory2>,
    pub fp_bind_buffer_memory2_khr: Option<vk::FnBindBufferMemory2KHR>,
    pub fp_bind_image_memory2: Option<vk::FnBindImageMemory2>,
    pub fp_bind_image_memory2_khr: Option<vk::FnBindImageMemory2KHR>,
    pub fp_cmd_set_device_mask: Option<vk::FnCmdSetDeviceMask>,
    pub fp_cmd_set_device_mask_khr: Option<vk::FnCmdSetDeviceMaskKHR>,
    pub fp_get_device_group_present_capabilities_khr: Option<vk::FnGetDeviceGroupPresentCapabilitiesKHR>,
    pub fp_get_device_group_surface_present_modes_khr: Option<vk::FnGetDeviceGroupSurfacePresentModesKHR>,
    pub fp_acquire_next_image2_khr: Option<vk::FnAcquireNextImage2KHR>,
    pub fp_cmd_dispatch_base: Option<vk::FnCmdDispatchBase>,
    pub fp_cmd_dispatch_base_khr: Option<vk::FnCmdDispatchBaseKHR>,
    pub fp_get_physical_device_present_rectangles_khr: Option<vk::FnGetPhysicalDevicePresentRectanglesKHR>,
    pub fp_create_descriptor_update_template: Option<vk::FnCreateDescriptorUpdateTemplate>,
    pub fp_create_descriptor_update_template_khr: Option<vk::FnCreateDescriptorUpdateTemplateKHR>,
    pub fp_destroy_descriptor_update_template: Option<vk::FnDestroyDescriptorUpdateTemplate>,
    pub fp_destroy_descriptor_update_template_khr: Option<vk::FnDestroyDescriptorUpdateTemplateKHR>,
    pub fp_update_descriptor_set_with_template: Option<vk::FnUpdateDescriptorSetWithTemplate>,
    pub fp_update_descriptor_set_with_template_khr: Option<vk::FnUpdateDescriptorSetWithTemplateKHR>,
    pub fp_cmd_push_descriptor_set_with_template_khr: Option<vk::FnCmdPushDescriptorSetWithTemplateKHR>,
    pub fp_set_hdr_metadata_ext: Option<vk::FnSetHdrMetadataEXT>,
    pub fp_get_swapchain_status_khr: Option<vk::FnGetSwapchainStatusKHR>,
    pub fp_get_refresh_cycle_duration_google: Option<vk::FnGetRefreshCycleDurationGOOGLE>,
    pub fp_get_past_presentation_timing_google: Option<vk::FnGetPastPresentationTimingGOOGLE>,
    pub fp_cmd_set_viewport_w_scaling_nv: Option<vk::FnCmdSetViewportWScalingNV>,
    pub fp_cmd_set_discard_rectangle_ext: Option<vk::FnCmdSetDiscardRectangleEXT>,
    pub fp_cmd_set_sample_locations_ext: Option<vk::FnCmdSetSampleLocationsEXT>,
    pub fp_get_physical_device_multisample_properties_ext: Option<vk::FnGetPhysicalDeviceMultisamplePropertiesEXT>,
    pub fp_get_buffer_memory_requirements2: Option<vk::FnGetBufferMemoryRequirements2>,
    pub fp_get_buffer_memory_requirements2_khr: Option<vk::FnGetBufferMemoryRequirements2KHR>,
    pub fp_get_image_memory_requirements2: Option<vk::FnGetImageMemoryRequirements2>,
    pub fp_get_image_memory_requirements2_khr: Option<vk::FnGetImageMemoryRequirements2KHR>,
    pub fp_get_image_sparse_memory_requirements2: Option<vk::FnGetImageSparseMemoryRequirements2>,
    pub fp_get_image_sparse_memory_requirements2_khr: Option<vk::FnGetImageSparseMemoryRequirements2KHR>,
    pub fp_create_sampler_ycbcr_conversion: Option<vk::FnCreateSamplerYcbcrConversion>,
    pub fp_create_sampler_ycbcr_conversion_khr: Option<vk::FnCreateSamplerYcbcrConversionKHR>,
    pub fp_destroy_sampler_ycbcr_conversion: Option<vk::FnDestroySamplerYcbcrConversion>,
    pub fp_destroy_sampler_ycbcr_conversion_khr: Option<vk::FnDestroySamplerYcbcrConversionKHR>,
    pub fp_get_device_queue2: Option<vk::FnGetDeviceQueue2>,
    pub fp_create_validation_cache_ext: Option<vk::FnCreateValidationCacheEXT>,
    pub fp_destroy_validation_cache_ext: Option<vk::FnDestroyValidationCacheEXT>,
    pub fp_get_validation_cache_data_ext: Option<vk::FnGetValidationCacheDataEXT>,
    pub fp_merge_validation_caches_ext: Option<vk::FnMergeValidationCachesEXT>,
    pub fp_get_descriptor_set_layout_support: Option<vk::FnGetDescriptorSetLayoutSupport>,
    pub fp_get_descriptor_set_layout_support_khr: Option<vk::FnGetDescriptorSetLayoutSupportKHR>,
    pub fp_get_shader_info_amd: Option<vk::FnGetShaderInfoAMD>,
    pub fp_set_local_dimming_amd: Option<vk::FnSetLocalDimmingAMD>,
    pub fp_get_physical_device_calibrateable_time_domains_ext:
        Option<vk::FnGetPhysicalDeviceCalibrateableTimeDomainsEXT>,
    pub fp_get_calibrated_timestamps_ext: Option<vk::FnGetCalibratedTimestampsEXT>,
    pub fp_get_memory_host_pointer_properties_ext: Option<vk::FnGetMemoryHostPointerPropertiesEXT>,
    pub fp_cmd_write_buffer_marker_amd: Option<vk::FnCmdWriteBufferMarkerAMD>,
    pub fp_create_render_pass2_khr: Option<vk::FnCreateRenderPass2KHR>,
    pub fp_cmd_begin_render_pass2_khr: Option<vk::FnCmdBeginRenderPass2KHR>,
    pub fp_cmd_next_subpass2_khr: Option<vk::FnCmdNextSubpass2KHR>,
    pub fp_cmd_end_render_pass2_khr: Option<vk::FnCmdEndRenderPass2KHR>,
    pub fp_get_android_hardware_buffer_properties_android: Option<vk::FnGetAndroidHardwareBufferPropertiesANDROID>,
    pub fp_get_memory_android_hardware_buffer_android: Option<vk::FnGetMemoryAndroidHardwareBufferANDROID>,
    pub fp_cmd_draw_indirect_count_khr: Option<vk::FnCmdDrawIndirectCountKHR>,
    pub fp_cmd_draw_indirect_count_amd: Option<vk::FnCmdDrawIndirectCountAMD>,
    pub fp_cmd_draw_indexed_indirect_count_khr: Option<vk::FnCmdDrawIndexedIndirectCountKHR>,
    pub fp_cmd_draw_indexed_indirect_count_amd: Option<vk::FnCmdDrawIndexedIndirectCountAMD>,
    pub fp_cmd_set_checkpoint_nv: Option<vk::FnCmdSetCheckpointNV>,
    pub fp_get_queue_checkpoint_data_nv: Option<vk::FnGetQueueCheckpointDataNV>,
    pub fp_cmd_bind_transform_feedback_buffers_ext: Option<vk::FnCmdBindTransformFeedbackBuffersEXT>,
    pub fp_cmd_begin_transform_feedback_ext: Option<vk::FnCmdBeginTransformFeedbackEXT>,
    pub fp_cmd_end_transform_feedback_ext: Option<vk::FnCmdEndTransformFeedbackEXT>,
    pub fp_cmd_begin_query_indexed_ext: Option<vk::FnCmdBeginQueryIndexedEXT>,
    pub fp_cmd_end_query_indexed_ext: Option<vk::FnCmdEndQueryIndexedEXT>,
    pub fp_cmd_draw_indirect_byte_count_ext: Option<vk::FnCmdDrawIndirectByteCountEXT>,
    pub fp_cmd_set_exclusive_scissor_nv: Option<vk::FnCmdSetExclusiveScissorNV>,
    pub fp_cmd_bind_shading_rate_image_nv: Option<vk::FnCmdBindShadingRateImageNV>,
    pub fp_cmd_set_viewport_shading_rate_palette_nv: Option<vk::FnCmdSetViewportShadingRatePaletteNV>,
    pub fp_cmd_set_coarse_sample_order_nv: Option<vk::FnCmdSetCoarseSampleOrderNV>,
    pub fp_cmd_draw_mesh_tasks_nv: Option<vk::FnCmdDrawMeshTasksNV>,
    pub fp_cmd_draw_mesh_tasks_indirect_nv: Option<vk::FnCmdDrawMeshTasksIndirectNV>,
    pub fp_cmd_draw_mesh_tasks_indirect_count_nv: Option<vk::FnCmdDrawMeshTasksIndirectCountNV>,
    pub fp_compile_deferred_nv: Option<vk::FnCompileDeferredNV>,
    pub fp_create_acceleration_structure_nv: Option<vk::FnCreateAccelerationStructureNV>,
    pub fp_destroy_acceleration_structure_nv: Option<vk::FnDestroyAccelerationStructureNV>,
    pub fp_get_acceleration_structure_memory_requirements_nv:
        Option<vk::FnGetAccelerationStructureMemoryRequirementsNV>,
    pub fp_bind_acceleration_structure_memory_nv: Option<vk::FnBindAccelerationStructureMemoryNV>,
    pub fp_cmd_copy_acceleration_structure_nv: Option<vk::FnCmdCopyAccelerationStructureNV>,
    pub fp_cmd_write_acceleration_structures_properties_nv: Option<vk::FnCmdWriteAccelerationStructuresPropertiesNV>,
    pub fp_cmd_build_acceleration_structure_nv: Option<vk::FnCmdBuildAccelerationStructureNV>,
    pub fp_cmd_trace_rays_nv: Option<vk::FnCmdTraceRaysNV>,
    pub fp_get_ray_tracing_shader_group_handles_nv: Option<vk::FnGetRayTracingShaderGroupHandlesNV>,
    pub fp_get_acceleration_structure_handle_nv: Option<vk::FnGetAccelerationStructureHandleNV>,
    pub fp_create_ray_tracing_pipelines_nv: Option<vk::FnCreateRayTracingPipelinesNV>,
    pub fp_get_image_drm_format_modifier_properties_ext: Option<vk::FnGetImageDrmFormatModifierPropertiesEXT>,
    pub fp_get_buffer_device_address_ext: Option<vk::FnGetBufferDeviceAddressEXT>,
    pub fp_get_physical_device_cooperative_matrix_properties_nv:
        Option<vk::FnGetPhysicalDeviceCooperativeMatrixPropertiesNV>,
    pub fp_get_image_view_handle_nvx: Option<vk::FnGetImageViewHandleNVX>,
    pub fp_get_physical_device_surface_present_modes2_ext: Option<vk::FnGetPhysicalDeviceSurfacePresentModes2EXT>,
    pub fp_get_device_group_surface_present_modes2_ext: Option<vk::FnGetDeviceGroupSurfacePresentModes2EXT>,
    pub fp_acquire_full_screen_exclusive_mode_ext: Option<vk::FnAcquireFullScreenExclusiveModeEXT>,
    pub fp_release_full_screen_exclusive_mode_ext: Option<vk::FnReleaseFullScreenExclusiveModeEXT>,
}
impl Device {
    unsafe fn load(instance: &Instance, device: vk::Device) -> LoaderResult<Self> {
        let f = |name: &CStr| instance.get_device_proc_addr(device, name);
        Ok(Self {
            handle: device,
            fp_destroy_device: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyDevice\0")).map(|f| mem::transmute(f)),
            fp_get_device_queue: f(CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceQueue\0"))
                .map(|f| mem::transmute(f)),
            fp_queue_submit: f(CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit\0")).map(|f| mem::transmute(f)),
            fp_queue_wait_idle: f(CStr::from_bytes_with_nul_unchecked(b"vkQueueWaitIdle\0")).map(|f| mem::transmute(f)),
            fp_device_wait_idle: f(CStr::from_bytes_with_nul_unchecked(b"vkDeviceWaitIdle\0"))
                .map(|f| mem::transmute(f)),
            fp_allocate_memory: f(CStr::from_bytes_with_nul_unchecked(b"vkAllocateMemory\0"))
                .map(|f| mem::transmute(f)),
            fp_free_memory: f(CStr::from_bytes_with_nul_unchecked(b"vkFreeMemory\0")).map(|f| mem::transmute(f)),
            fp_map_memory: f(CStr::from_bytes_with_nul_unchecked(b"vkMapMemory\0")).map(|f| mem::transmute(f)),
            fp_unmap_memory: f(CStr::from_bytes_with_nul_unchecked(b"vkUnmapMemory\0")).map(|f| mem::transmute(f)),
            fp_flush_mapped_memory_ranges: f(CStr::from_bytes_with_nul_unchecked(b"vkFlushMappedMemoryRanges\0"))
                .map(|f| mem::transmute(f)),
            fp_invalidate_mapped_memory_ranges: f(CStr::from_bytes_with_nul_unchecked(
                b"vkInvalidateMappedMemoryRanges\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_device_memory_commitment: f(CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceMemoryCommitment\0"))
                .map(|f| mem::transmute(f)),
            fp_get_buffer_memory_requirements: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetBufferMemoryRequirements\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_bind_buffer_memory: f(CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory\0"))
                .map(|f| mem::transmute(f)),
            fp_get_image_memory_requirements: f(CStr::from_bytes_with_nul_unchecked(b"vkGetImageMemoryRequirements\0"))
                .map(|f| mem::transmute(f)),
            fp_bind_image_memory: f(CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory\0"))
                .map(|f| mem::transmute(f)),
            fp_get_image_sparse_memory_requirements: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetImageSparseMemoryRequirements\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_queue_bind_sparse: f(CStr::from_bytes_with_nul_unchecked(b"vkQueueBindSparse\0"))
                .map(|f| mem::transmute(f)),
            fp_create_fence: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateFence\0")).map(|f| mem::transmute(f)),
            fp_destroy_fence: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyFence\0")).map(|f| mem::transmute(f)),
            fp_reset_fences: f(CStr::from_bytes_with_nul_unchecked(b"vkResetFences\0")).map(|f| mem::transmute(f)),
            fp_get_fence_status: f(CStr::from_bytes_with_nul_unchecked(b"vkGetFenceStatus\0"))
                .map(|f| mem::transmute(f)),
            fp_wait_for_fences: f(CStr::from_bytes_with_nul_unchecked(b"vkWaitForFences\0")).map(|f| mem::transmute(f)),
            fp_create_semaphore: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateSemaphore\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_semaphore: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroySemaphore\0"))
                .map(|f| mem::transmute(f)),
            fp_create_event: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateEvent\0")).map(|f| mem::transmute(f)),
            fp_destroy_event: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyEvent\0")).map(|f| mem::transmute(f)),
            fp_get_event_status: f(CStr::from_bytes_with_nul_unchecked(b"vkGetEventStatus\0"))
                .map(|f| mem::transmute(f)),
            fp_set_event: f(CStr::from_bytes_with_nul_unchecked(b"vkSetEvent\0")).map(|f| mem::transmute(f)),
            fp_reset_event: f(CStr::from_bytes_with_nul_unchecked(b"vkResetEvent\0")).map(|f| mem::transmute(f)),
            fp_create_query_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateQueryPool\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_query_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyQueryPool\0"))
                .map(|f| mem::transmute(f)),
            fp_get_query_pool_results: f(CStr::from_bytes_with_nul_unchecked(b"vkGetQueryPoolResults\0"))
                .map(|f| mem::transmute(f)),
            fp_reset_query_pool_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkResetQueryPoolEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_create_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateBuffer\0")).map(|f| mem::transmute(f)),
            fp_destroy_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyBuffer\0")).map(|f| mem::transmute(f)),
            fp_create_buffer_view: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateBufferView\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_buffer_view: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyBufferView\0"))
                .map(|f| mem::transmute(f)),
            fp_create_image: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateImage\0")).map(|f| mem::transmute(f)),
            fp_destroy_image: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyImage\0")).map(|f| mem::transmute(f)),
            fp_get_image_subresource_layout: f(CStr::from_bytes_with_nul_unchecked(b"vkGetImageSubresourceLayout\0"))
                .map(|f| mem::transmute(f)),
            fp_create_image_view: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateImageView\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_image_view: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyImageView\0"))
                .map(|f| mem::transmute(f)),
            fp_create_shader_module: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateShaderModule\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_shader_module: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyShaderModule\0"))
                .map(|f| mem::transmute(f)),
            fp_create_pipeline_cache: f(CStr::from_bytes_with_nul_unchecked(b"vkCreatePipelineCache\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_pipeline_cache: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipelineCache\0"))
                .map(|f| mem::transmute(f)),
            fp_get_pipeline_cache_data: f(CStr::from_bytes_with_nul_unchecked(b"vkGetPipelineCacheData\0"))
                .map(|f| mem::transmute(f)),
            fp_merge_pipeline_caches: f(CStr::from_bytes_with_nul_unchecked(b"vkMergePipelineCaches\0"))
                .map(|f| mem::transmute(f)),
            fp_create_graphics_pipelines: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateGraphicsPipelines\0"))
                .map(|f| mem::transmute(f)),
            fp_create_compute_pipelines: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateComputePipelines\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_pipeline: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipeline\0"))
                .map(|f| mem::transmute(f)),
            fp_create_pipeline_layout: f(CStr::from_bytes_with_nul_unchecked(b"vkCreatePipelineLayout\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_pipeline_layout: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipelineLayout\0"))
                .map(|f| mem::transmute(f)),
            fp_create_sampler: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateSampler\0")).map(|f| mem::transmute(f)),
            fp_destroy_sampler: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroySampler\0"))
                .map(|f| mem::transmute(f)),
            fp_create_descriptor_set_layout: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDescriptorSetLayout\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_descriptor_set_layout: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyDescriptorSetLayout\0"))
                .map(|f| mem::transmute(f)),
            fp_create_descriptor_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateDescriptorPool\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_descriptor_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyDescriptorPool\0"))
                .map(|f| mem::transmute(f)),
            fp_reset_descriptor_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkResetDescriptorPool\0"))
                .map(|f| mem::transmute(f)),
            fp_allocate_descriptor_sets: f(CStr::from_bytes_with_nul_unchecked(b"vkAllocateDescriptorSets\0"))
                .map(|f| mem::transmute(f)),
            fp_free_descriptor_sets: f(CStr::from_bytes_with_nul_unchecked(b"vkFreeDescriptorSets\0"))
                .map(|f| mem::transmute(f)),
            fp_update_descriptor_sets: f(CStr::from_bytes_with_nul_unchecked(b"vkUpdateDescriptorSets\0"))
                .map(|f| mem::transmute(f)),
            fp_create_framebuffer: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateFramebuffer\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_framebuffer: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyFramebuffer\0"))
                .map(|f| mem::transmute(f)),
            fp_create_render_pass: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_render_pass: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyRenderPass\0"))
                .map(|f| mem::transmute(f)),
            fp_get_render_area_granularity: f(CStr::from_bytes_with_nul_unchecked(b"vkGetRenderAreaGranularity\0"))
                .map(|f| mem::transmute(f)),
            fp_create_command_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateCommandPool\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_command_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyCommandPool\0"))
                .map(|f| mem::transmute(f)),
            fp_reset_command_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkResetCommandPool\0"))
                .map(|f| mem::transmute(f)),
            fp_allocate_command_buffers: f(CStr::from_bytes_with_nul_unchecked(b"vkAllocateCommandBuffers\0"))
                .map(|f| mem::transmute(f)),
            fp_free_command_buffers: f(CStr::from_bytes_with_nul_unchecked(b"vkFreeCommandBuffers\0"))
                .map(|f| mem::transmute(f)),
            fp_begin_command_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkBeginCommandBuffer\0"))
                .map(|f| mem::transmute(f)),
            fp_end_command_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkEndCommandBuffer\0"))
                .map(|f| mem::transmute(f)),
            fp_reset_command_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkResetCommandBuffer\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_bind_pipeline: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindPipeline\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_viewport: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewport\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_scissor: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissor\0")).map(|f| mem::transmute(f)),
            fp_cmd_set_line_width: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineWidth\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_depth_bias: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBias\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_blend_constants: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetBlendConstants\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_depth_bounds: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBounds\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_stencil_compare_mask: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilCompareMask\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_stencil_write_mask: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilWriteMask\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_stencil_reference: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilReference\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_bind_descriptor_sets: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindDescriptorSets\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_bind_index_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindIndexBuffer\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_bind_vertex_buffers: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_draw: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDraw\0")).map(|f| mem::transmute(f)),
            fp_cmd_draw_indexed: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexed\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_draw_indirect: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirect\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_draw_indexed_indirect: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexedIndirect\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_dispatch: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatch\0")).map(|f| mem::transmute(f)),
            fp_cmd_dispatch_indirect: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchIndirect\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_copy_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer\0")).map(|f| mem::transmute(f)),
            fp_cmd_copy_image: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage\0")).map(|f| mem::transmute(f)),
            fp_cmd_blit_image: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage\0")).map(|f| mem::transmute(f)),
            fp_cmd_copy_buffer_to_image: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBufferToImage\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_copy_image_to_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImageToBuffer\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_update_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdUpdateBuffer\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_fill_buffer: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdFillBuffer\0")).map(|f| mem::transmute(f)),
            fp_cmd_clear_color_image: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdClearColorImage\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_clear_depth_stencil_image: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdClearDepthStencilImage\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_clear_attachments: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdClearAttachments\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_resolve_image: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_event: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent\0")).map(|f| mem::transmute(f)),
            fp_cmd_reset_event: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent\0")).map(|f| mem::transmute(f)),
            fp_cmd_wait_events: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents\0")).map(|f| mem::transmute(f)),
            fp_cmd_pipeline_barrier: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_begin_query: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginQuery\0")).map(|f| mem::transmute(f)),
            fp_cmd_end_query: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndQuery\0")).map(|f| mem::transmute(f)),
            fp_cmd_begin_conditional_rendering_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdBeginConditionalRenderingEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_end_conditional_rendering_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdEndConditionalRenderingEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_reset_query_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdResetQueryPool\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_write_timestamp: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_copy_query_pool_results: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyQueryPoolResults\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_push_constants: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdPushConstants\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_begin_render_pass: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_next_subpass: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_end_render_pass: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_execute_commands: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdExecuteCommands\0"))
                .map(|f| mem::transmute(f)),
            fp_create_shared_swapchains_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateSharedSwapchainsKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_create_swapchain_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateSwapchainKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_swapchain_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroySwapchainKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_swapchain_images_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainImagesKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_acquire_next_image_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImageKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_queue_present_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkQueuePresentKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_debug_marker_set_object_name_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDebugMarkerSetObjectNameEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_debug_marker_set_object_tag_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDebugMarkerSetObjectTagEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_debug_marker_begin_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerBeginEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_debug_marker_end_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerEndEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_debug_marker_insert_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerInsertEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_memory_win32_handle_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleNV\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_process_commands_nvx: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdProcessCommandsNVX\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_reserve_space_for_commands_nvx: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdReserveSpaceForCommandsNVX\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_indirect_commands_layout_nvx: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateIndirectCommandsLayoutNVX\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_destroy_indirect_commands_layout_nvx: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDestroyIndirectCommandsLayoutNVX\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_object_table_nvx: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateObjectTableNVX\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_object_table_nvx: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyObjectTableNVX\0"))
                .map(|f| mem::transmute(f)),
            fp_register_objects_nvx: f(CStr::from_bytes_with_nul_unchecked(b"vkRegisterObjectsNVX\0"))
                .map(|f| mem::transmute(f)),
            fp_unregister_objects_nvx: f(CStr::from_bytes_with_nul_unchecked(b"vkUnregisterObjectsNVX\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_generated_commands_properties_nvx: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_push_descriptor_set_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdPushDescriptorSetKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_trim_command_pool: f(CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPool\0"))
                .map(|f| mem::transmute(f)),
            fp_trim_command_pool_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPoolKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_memory_win32_handle_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_memory_win32_handle_properties_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetMemoryWin32HandlePropertiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_memory_fd_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryFdKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_memory_fd_properties_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryFdPropertiesKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_semaphore_win32_handle_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetSemaphoreWin32HandleKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_import_semaphore_win32_handle_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkImportSemaphoreWin32HandleKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_semaphore_fd_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreFdKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_import_semaphore_fd_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkImportSemaphoreFdKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_fence_win32_handle_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetFenceWin32HandleKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_import_fence_win32_handle_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkImportFenceWin32HandleKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_fence_fd_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetFenceFdKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_import_fence_fd_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkImportFenceFdKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_display_power_control_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkDisplayPowerControlEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_register_device_event_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkRegisterDeviceEventEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_register_display_event_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkRegisterDisplayEventEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_swapchain_counter_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainCounterEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_device_group_peer_memory_features: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDeviceGroupPeerMemoryFeatures\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_device_group_peer_memory_features_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDeviceGroupPeerMemoryFeaturesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_bind_buffer_memory2: f(CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory2\0"))
                .map(|f| mem::transmute(f)),
            fp_bind_buffer_memory2_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory2KHR\0"))
                .map(|f| mem::transmute(f)),
            fp_bind_image_memory2: f(CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory2\0"))
                .map(|f| mem::transmute(f)),
            fp_bind_image_memory2_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory2KHR\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_device_mask: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDeviceMask\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_device_mask_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDeviceMaskKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_device_group_present_capabilities_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDeviceGroupPresentCapabilitiesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_device_group_surface_present_modes_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDeviceGroupSurfacePresentModesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_acquire_next_image2_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_dispatch_base: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchBase\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_dispatch_base_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchBaseKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_present_rectangles_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDevicePresentRectanglesKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_descriptor_update_template: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateDescriptorUpdateTemplate\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_descriptor_update_template_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateDescriptorUpdateTemplateKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_destroy_descriptor_update_template: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDestroyDescriptorUpdateTemplate\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_destroy_descriptor_update_template_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDestroyDescriptorUpdateTemplateKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_update_descriptor_set_with_template: f(CStr::from_bytes_with_nul_unchecked(
                b"vkUpdateDescriptorSetWithTemplate\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_update_descriptor_set_with_template_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkUpdateDescriptorSetWithTemplateKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_push_descriptor_set_with_template_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdPushDescriptorSetWithTemplateKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_set_hdr_metadata_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkSetHdrMetadataEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_swapchain_status_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainStatusKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_refresh_cycle_duration_google: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetRefreshCycleDurationGOOGLE\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_past_presentation_timing_google: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPastPresentationTimingGOOGLE\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_set_viewport_w_scaling_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportWScalingNV\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_discard_rectangle_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDiscardRectangleEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_sample_locations_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetSampleLocationsEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_multisample_properties_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceMultisamplePropertiesEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_buffer_memory_requirements2: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetBufferMemoryRequirements2\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_buffer_memory_requirements2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetBufferMemoryRequirements2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_image_memory_requirements2: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetImageMemoryRequirements2\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_image_memory_requirements2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetImageMemoryRequirements2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_image_sparse_memory_requirements2: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetImageSparseMemoryRequirements2\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_image_sparse_memory_requirements2_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetImageSparseMemoryRequirements2KHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_sampler_ycbcr_conversion: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateSamplerYcbcrConversion\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_sampler_ycbcr_conversion_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateSamplerYcbcrConversionKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_destroy_sampler_ycbcr_conversion: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDestroySamplerYcbcrConversion\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_destroy_sampler_ycbcr_conversion_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDestroySamplerYcbcrConversionKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_device_queue2: f(CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceQueue2\0"))
                .map(|f| mem::transmute(f)),
            fp_create_validation_cache_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateValidationCacheEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_destroy_validation_cache_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkDestroyValidationCacheEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_validation_cache_data_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkGetValidationCacheDataEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_merge_validation_caches_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkMergeValidationCachesEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_descriptor_set_layout_support: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDescriptorSetLayoutSupport\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_descriptor_set_layout_support_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDescriptorSetLayoutSupportKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_shader_info_amd: f(CStr::from_bytes_with_nul_unchecked(b"vkGetShaderInfoAMD\0"))
                .map(|f| mem::transmute(f)),
            fp_set_local_dimming_amd: f(CStr::from_bytes_with_nul_unchecked(b"vkSetLocalDimmingAMD\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_calibrateable_time_domains_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_calibrated_timestamps_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkGetCalibratedTimestampsEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_memory_host_pointer_properties_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetMemoryHostPointerPropertiesEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_write_buffer_marker_amd: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteBufferMarkerAMD\0"))
                .map(|f| mem::transmute(f)),
            fp_create_render_pass2_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass2KHR\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_begin_render_pass2_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass2KHR\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_next_subpass2_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass2KHR\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_end_render_pass2_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass2KHR\0"))
                .map(|f| mem::transmute(f)),
            fp_get_android_hardware_buffer_properties_android: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetAndroidHardwareBufferPropertiesANDROID\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_memory_android_hardware_buffer_android: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetMemoryAndroidHardwareBufferANDROID\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_draw_indirect_count_khr: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountKHR\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_draw_indirect_count_amd: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountAMD\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_draw_indexed_indirect_count_khr: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdDrawIndexedIndirectCountKHR\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_draw_indexed_indirect_count_amd: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdDrawIndexedIndirectCountAMD\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_set_checkpoint_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCheckpointNV\0"))
                .map(|f| mem::transmute(f)),
            fp_get_queue_checkpoint_data_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkGetQueueCheckpointDataNV\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_bind_transform_feedback_buffers_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdBindTransformFeedbackBuffersEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_begin_transform_feedback_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdBeginTransformFeedbackEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_end_transform_feedback_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdEndTransformFeedbackEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_begin_query_indexed_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginQueryIndexedEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_end_query_indexed_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdEndQueryIndexedEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_draw_indirect_byte_count_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdDrawIndirectByteCountEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_set_exclusive_scissor_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetExclusiveScissorNV\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_bind_shading_rate_image_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdBindShadingRateImageNV\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_set_viewport_shading_rate_palette_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdSetViewportShadingRatePaletteNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_set_coarse_sample_order_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCoarseSampleOrderNV\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_draw_mesh_tasks_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksNV\0"))
                .map(|f| mem::transmute(f)),
            fp_cmd_draw_mesh_tasks_indirect_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdDrawMeshTasksIndirectNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_draw_mesh_tasks_indirect_count_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdDrawMeshTasksIndirectCountNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_compile_deferred_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkCompileDeferredNV\0"))
                .map(|f| mem::transmute(f)),
            fp_create_acceleration_structure_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateAccelerationStructureNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_destroy_acceleration_structure_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkDestroyAccelerationStructureNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_acceleration_structure_memory_requirements_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetAccelerationStructureMemoryRequirementsNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_bind_acceleration_structure_memory_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkBindAccelerationStructureMemoryNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_copy_acceleration_structure_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdCopyAccelerationStructureNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_write_acceleration_structures_properties_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdWriteAccelerationStructuresPropertiesNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_build_acceleration_structure_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCmdBuildAccelerationStructureNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_cmd_trace_rays_nv: f(CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysNV\0"))
                .map(|f| mem::transmute(f)),
            fp_get_ray_tracing_shader_group_handles_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetRayTracingShaderGroupHandlesNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_acceleration_structure_handle_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetAccelerationStructureHandleNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_create_ray_tracing_pipelines_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkCreateRayTracingPipelinesNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_image_drm_format_modifier_properties_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetImageDrmFormatModifierPropertiesEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_buffer_device_address_ext: f(CStr::from_bytes_with_nul_unchecked(b"vkGetBufferDeviceAddressEXT\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_cooperative_matrix_properties_nv: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_image_view_handle_nvx: f(CStr::from_bytes_with_nul_unchecked(b"vkGetImageViewHandleNVX\0"))
                .map(|f| mem::transmute(f)),
            fp_get_physical_device_surface_present_modes2_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetPhysicalDeviceSurfacePresentModes2EXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_get_device_group_surface_present_modes2_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkGetDeviceGroupSurfacePresentModes2EXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_acquire_full_screen_exclusive_mode_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkAcquireFullScreenExclusiveModeEXT\0",
            ))
            .map(|f| mem::transmute(f)),
            fp_release_full_screen_exclusive_mode_ext: f(CStr::from_bytes_with_nul_unchecked(
                b"vkReleaseFullScreenExclusiveModeEXT\0",
            ))
            .map(|f| mem::transmute(f)),
        })
    }
    pub unsafe fn destroy_device(&self, p_allocator: Option<&vk::AllocationCallbacks>) {
        (self.fp_destroy_device.unwrap())(Some(self.handle), p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> vk::Queue {
        let mut res = mem::uninitialized();
        (self.fp_get_device_queue.unwrap())(Some(self.handle), queue_family_index, queue_index, &mut res);
        res
    }
    pub unsafe fn queue_submit(
        &self,
        queue: vk::Queue,
        p_submits: &[vk::SubmitInfo],
        fence: Option<vk::Fence>,
    ) -> Result<()> {
        let submit_count = p_submits.len() as u32;
        let err = (self.fp_queue_submit.unwrap())(Some(queue), submit_count, p_submits.as_ptr(), fence);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn queue_wait_idle(&self, queue: vk::Queue) -> Result<()> {
        let err = (self.fp_queue_wait_idle.unwrap())(Some(queue));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn device_wait_idle(&self) -> Result<()> {
        let err = (self.fp_device_wait_idle.unwrap())(Some(self.handle));
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
        let err = (self.fp_allocate_memory.unwrap())(
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
        (self.fp_free_memory.unwrap())(Some(self.handle), memory, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn map_memory(
        &self,
        memory: vk::DeviceMemory,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        flags: vk::MemoryMapFlags,
    ) -> Result<*mut c_void> {
        let mut res = mem::uninitialized();
        let err = (self.fp_map_memory.unwrap())(Some(self.handle), Some(memory), offset, size, flags, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn unmap_memory(&self, memory: vk::DeviceMemory) {
        (self.fp_unmap_memory.unwrap())(Some(self.handle), Some(memory));
    }
    pub unsafe fn flush_mapped_memory_ranges(&self, p_memory_ranges: &[vk::MappedMemoryRange]) -> Result<()> {
        let memory_range_count = p_memory_ranges.len() as u32;
        let err = (self.fp_flush_mapped_memory_ranges.unwrap())(
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
    pub unsafe fn invalidate_mapped_memory_ranges(&self, p_memory_ranges: &[vk::MappedMemoryRange]) -> Result<()> {
        let memory_range_count = p_memory_ranges.len() as u32;
        let err = (self.fp_invalidate_mapped_memory_ranges.unwrap())(
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
        (self.fp_get_device_memory_commitment.unwrap())(Some(self.handle), Some(memory), &mut res);
        res
    }
    pub unsafe fn get_buffer_memory_requirements(&self, buffer: vk::Buffer) -> vk::MemoryRequirements {
        let mut res = mem::uninitialized();
        (self.fp_get_buffer_memory_requirements.unwrap())(Some(self.handle), Some(buffer), &mut res);
        res
    }
    pub unsafe fn bind_buffer_memory(
        &self,
        buffer: vk::Buffer,
        memory: vk::DeviceMemory,
        memory_offset: vk::DeviceSize,
    ) -> Result<()> {
        let err = (self.fp_bind_buffer_memory.unwrap())(Some(self.handle), Some(buffer), Some(memory), memory_offset);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_image_memory_requirements(&self, image: vk::Image) -> vk::MemoryRequirements {
        let mut res = mem::uninitialized();
        (self.fp_get_image_memory_requirements.unwrap())(Some(self.handle), Some(image), &mut res);
        res
    }
    pub unsafe fn bind_image_memory(
        &self,
        image: vk::Image,
        memory: vk::DeviceMemory,
        memory_offset: vk::DeviceSize,
    ) -> Result<()> {
        let err = (self.fp_bind_image_memory.unwrap())(Some(self.handle), Some(image), Some(memory), memory_offset);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_image_sparse_memory_requirements_to_vec(
        &self,
        image: vk::Image,
    ) -> Vec<vk::SparseImageMemoryRequirements> {
        let mut len = mem::uninitialized();
        (self.fp_get_image_sparse_memory_requirements.unwrap())(
            Some(self.handle),
            Some(image),
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp_get_image_sparse_memory_requirements.unwrap())(
            Some(self.handle),
            Some(image),
            &mut len,
            v.as_mut_ptr(),
        );
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
        let err = (self.fp_queue_bind_sparse.unwrap())(Some(queue), bind_info_count, p_bind_info.as_ptr(), fence);
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
        let err = (self.fp_create_fence.unwrap())(
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
        (self.fp_destroy_fence.unwrap())(Some(self.handle), fence, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn reset_fences(&self, p_fences: &[vk::Fence]) -> Result<()> {
        let fence_count = p_fences.len() as u32;
        let err = (self.fp_reset_fences.unwrap())(Some(self.handle), fence_count, p_fences.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_fence_status(&self, fence: vk::Fence) -> Result<vk::Result> {
        let err = (self.fp_get_fence_status.unwrap())(Some(self.handle), Some(fence));
        let res = match err {
            vk::Result::SUCCESS | vk::Result::NOT_READY => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn wait_for_fences(&self, p_fences: &[vk::Fence], wait_all: bool, timeout: u64) -> Result<vk::Result> {
        let fence_count = p_fences.len() as u32;
        let err = (self.fp_wait_for_fences.unwrap())(
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
        let err = (self.fp_create_semaphore.unwrap())(
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
        (self.fp_destroy_semaphore.unwrap())(Some(self.handle), semaphore, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_event(
        &self,
        p_create_info: &vk::EventCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Event> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_event.unwrap())(
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
        (self.fp_destroy_event.unwrap())(Some(self.handle), event, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_event_status(&self, event: vk::Event) -> Result<vk::Result> {
        let err = (self.fp_get_event_status.unwrap())(Some(self.handle), Some(event));
        let res = match err {
            vk::Result::EVENT_SET | vk::Result::EVENT_RESET => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn set_event(&self, event: vk::Event) -> Result<()> {
        let err = (self.fp_set_event.unwrap())(Some(self.handle), Some(event));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn reset_event(&self, event: vk::Event) -> Result<()> {
        let err = (self.fp_reset_event.unwrap())(Some(self.handle), Some(event));
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
        let err = (self.fp_create_query_pool.unwrap())(
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
        (self.fp_destroy_query_pool.unwrap())(Some(self.handle), query_pool, p_allocator.map_or(ptr::null(), |r| r));
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
        let err = (self.fp_get_query_pool_results.unwrap())(
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
    pub unsafe fn reset_query_pool_ext(&self, query_pool: vk::QueryPool, first_query: u32, query_count: u32) {
        (self.fp_reset_query_pool_ext.unwrap())(Some(self.handle), Some(query_pool), first_query, query_count);
    }
    pub unsafe fn create_buffer(
        &self,
        p_create_info: &vk::BufferCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Buffer> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_buffer.unwrap())(
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
        (self.fp_destroy_buffer.unwrap())(Some(self.handle), buffer, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_buffer_view(
        &self,
        p_create_info: &vk::BufferViewCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::BufferView> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_buffer_view.unwrap())(
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
        (self.fp_destroy_buffer_view.unwrap())(Some(self.handle), buffer_view, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_image(
        &self,
        p_create_info: &vk::ImageCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Image> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_image.unwrap())(
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
        (self.fp_destroy_image.unwrap())(Some(self.handle), image, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_image_subresource_layout(
        &self,
        image: vk::Image,
        p_subresource: &vk::ImageSubresource,
    ) -> vk::SubresourceLayout {
        let mut res = mem::uninitialized();
        (self.fp_get_image_subresource_layout.unwrap())(Some(self.handle), Some(image), p_subresource, &mut res);
        res
    }
    pub unsafe fn create_image_view(
        &self,
        p_create_info: &vk::ImageViewCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ImageView> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_image_view.unwrap())(
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
        (self.fp_destroy_image_view.unwrap())(Some(self.handle), image_view, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_shader_module(
        &self,
        p_create_info: &vk::ShaderModuleCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ShaderModule> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_shader_module.unwrap())(
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
        (self.fp_destroy_shader_module.unwrap())(
            Some(self.handle),
            shader_module,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn create_pipeline_cache(
        &self,
        p_create_info: &vk::PipelineCacheCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::PipelineCache> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_pipeline_cache.unwrap())(
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
        (self.fp_destroy_pipeline_cache.unwrap())(
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
        let err =
            (self.fp_get_pipeline_cache_data.unwrap())(Some(self.handle), Some(pipeline_cache), p_data_size, p_data);
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
        let err = (self.fp_merge_pipeline_caches.unwrap())(
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
        let v_err = (self.fp_create_graphics_pipelines.unwrap())(
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
        let v_err = (self.fp_create_graphics_pipelines.unwrap())(
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
        let v_err = (self.fp_create_graphics_pipelines.unwrap())(
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
        let v_err = (self.fp_create_graphics_pipelines.unwrap())(
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
        let v_err = (self.fp_create_compute_pipelines.unwrap())(
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
        let v_err = (self.fp_create_compute_pipelines.unwrap())(
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
        let v_err = (self.fp_create_compute_pipelines.unwrap())(
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
        let v_err = (self.fp_create_compute_pipelines.unwrap())(
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
        (self.fp_destroy_pipeline.unwrap())(Some(self.handle), pipeline, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_pipeline_layout(
        &self,
        p_create_info: &vk::PipelineLayoutCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::PipelineLayout> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_pipeline_layout.unwrap())(
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
        (self.fp_destroy_pipeline_layout.unwrap())(
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
        let err = (self.fp_create_sampler.unwrap())(
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
        (self.fp_destroy_sampler.unwrap())(Some(self.handle), sampler, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_descriptor_set_layout(
        &self,
        p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorSetLayout> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_descriptor_set_layout.unwrap())(
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
        (self.fp_destroy_descriptor_set_layout.unwrap())(
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
        let err = (self.fp_create_descriptor_pool.unwrap())(
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
        (self.fp_destroy_descriptor_pool.unwrap())(
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
        let err = (self.fp_reset_descriptor_pool.unwrap())(Some(self.handle), Some(descriptor_pool), flags);
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
        let v_err = (self.fp_allocate_descriptor_sets.unwrap())(Some(self.handle), p_allocate_info, p_descriptor_sets);
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
        let v_err = (self.fp_allocate_descriptor_sets.unwrap())(Some(self.handle), p_allocate_info, v.as_mut_ptr());
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
        let v_err = (self.fp_allocate_descriptor_sets.unwrap())(Some(self.handle), p_allocate_info, v.as_mut_ptr());
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
        let v_err = (self.fp_allocate_descriptor_sets.unwrap())(Some(self.handle), p_allocate_info, &mut v);
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
        let err = (self.fp_free_descriptor_sets.unwrap())(
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
        (self.fp_update_descriptor_sets.unwrap())(
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
        let err = (self.fp_create_framebuffer.unwrap())(
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
        (self.fp_destroy_framebuffer.unwrap())(Some(self.handle), framebuffer, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn create_render_pass(
        &self,
        p_create_info: &vk::RenderPassCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::RenderPass> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_render_pass.unwrap())(
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
        (self.fp_destroy_render_pass.unwrap())(Some(self.handle), render_pass, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_render_area_granularity(&self, render_pass: vk::RenderPass) -> vk::Extent2D {
        let mut res = mem::uninitialized();
        (self.fp_get_render_area_granularity.unwrap())(Some(self.handle), Some(render_pass), &mut res);
        res
    }
    pub unsafe fn create_command_pool(
        &self,
        p_create_info: &vk::CommandPoolCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::CommandPool> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_command_pool.unwrap())(
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
        (self.fp_destroy_command_pool.unwrap())(
            Some(self.handle),
            command_pool,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn reset_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolResetFlags,
    ) -> Result<()> {
        let err = (self.fp_reset_command_pool.unwrap())(Some(self.handle), Some(command_pool), flags);
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
        let v_err = (self.fp_allocate_command_buffers.unwrap())(Some(self.handle), p_allocate_info, p_command_buffers);
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
        let v_err = (self.fp_allocate_command_buffers.unwrap())(Some(self.handle), p_allocate_info, v.as_mut_ptr());
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
        let v_err = (self.fp_allocate_command_buffers.unwrap())(Some(self.handle), p_allocate_info, v.as_mut_ptr());
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
        let v_err = (self.fp_allocate_command_buffers.unwrap())(Some(self.handle), p_allocate_info, &mut v);
        let res = match v_err {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(v_err),
        };
        res
    }
    pub unsafe fn free_command_buffers(&self, command_pool: vk::CommandPool, p_command_buffers: &[vk::CommandBuffer]) {
        let command_buffer_count = p_command_buffers.len() as u32;
        (self.fp_free_command_buffers.unwrap())(
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
        let err = (self.fp_begin_command_buffer.unwrap())(Some(command_buffer), p_begin_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> Result<()> {
        let err = (self.fp_end_command_buffer.unwrap())(Some(command_buffer));
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
        let err = (self.fp_reset_command_buffer.unwrap())(Some(command_buffer), flags);
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
        (self.fp_cmd_bind_pipeline.unwrap())(Some(command_buffer), pipeline_bind_point, Some(pipeline));
    }
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        p_viewports: &[vk::Viewport],
    ) {
        let viewport_count = p_viewports.len() as u32;
        (self.fp_cmd_set_viewport.unwrap())(
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
        (self.fp_cmd_set_scissor.unwrap())(Some(command_buffer), first_scissor, scissor_count, p_scissors.as_ptr());
    }
    pub unsafe fn cmd_set_line_width(&self, command_buffer: vk::CommandBuffer, line_width: f32) {
        (self.fp_cmd_set_line_width.unwrap())(Some(command_buffer), line_width);
    }
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: vk::CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        (self.fp_cmd_set_depth_bias.unwrap())(
            Some(command_buffer),
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
        );
    }
    pub unsafe fn cmd_set_blend_constants(&self, command_buffer: vk::CommandBuffer, blend_constants: [f32; 4]) {
        (self.fp_cmd_set_blend_constants.unwrap())(Some(command_buffer), blend_constants);
    }
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: vk::CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        (self.fp_cmd_set_depth_bounds.unwrap())(Some(command_buffer), min_depth_bounds, max_depth_bounds);
    }
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        compare_mask: u32,
    ) {
        (self.fp_cmd_set_stencil_compare_mask.unwrap())(Some(command_buffer), face_mask, compare_mask);
    }
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        write_mask: u32,
    ) {
        (self.fp_cmd_set_stencil_write_mask.unwrap())(Some(command_buffer), face_mask, write_mask);
    }
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: vk::CommandBuffer,
        face_mask: vk::StencilFaceFlags,
        reference: u32,
    ) {
        (self.fp_cmd_set_stencil_reference.unwrap())(Some(command_buffer), face_mask, reference);
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
        (self.fp_cmd_bind_descriptor_sets.unwrap())(
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
        (self.fp_cmd_bind_index_buffer.unwrap())(Some(command_buffer), Some(buffer), offset, index_type);
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
        (self.fp_cmd_bind_vertex_buffers.unwrap())(
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
        (self.fp_cmd_draw.unwrap())(
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
        (self.fp_cmd_draw_indexed.unwrap())(
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
        (self.fp_cmd_draw_indirect.unwrap())(Some(command_buffer), Some(buffer), offset, draw_count, stride);
    }
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self.fp_cmd_draw_indexed_indirect.unwrap())(Some(command_buffer), Some(buffer), offset, draw_count, stride);
    }
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: vk::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self.fp_cmd_dispatch.unwrap())(Some(command_buffer), group_count_x, group_count_y, group_count_z);
    }
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
    ) {
        (self.fp_cmd_dispatch_indirect.unwrap())(Some(command_buffer), Some(buffer), offset);
    }
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        src_buffer: vk::Buffer,
        dst_buffer: vk::Buffer,
        p_regions: &[vk::BufferCopy],
    ) {
        let region_count = p_regions.len() as u32;
        (self.fp_cmd_copy_buffer.unwrap())(
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
        (self.fp_cmd_copy_image.unwrap())(
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
        (self.fp_cmd_blit_image.unwrap())(
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
        (self.fp_cmd_copy_buffer_to_image.unwrap())(
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
        (self.fp_cmd_copy_image_to_buffer.unwrap())(
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
        (self.fp_cmd_update_buffer.unwrap())(Some(command_buffer), Some(dst_buffer), dst_offset, data_size, p_data);
    }
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: u32,
    ) {
        (self.fp_cmd_fill_buffer.unwrap())(Some(command_buffer), Some(dst_buffer), dst_offset, size, data);
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
        (self.fp_cmd_clear_color_image.unwrap())(
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
        (self.fp_cmd_clear_depth_stencil_image.unwrap())(
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
        (self.fp_cmd_clear_attachments.unwrap())(
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
        (self.fp_cmd_resolve_image.unwrap())(
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
        (self.fp_cmd_set_event.unwrap())(Some(command_buffer), Some(event), stage_mask);
    }
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags,
    ) {
        (self.fp_cmd_reset_event.unwrap())(Some(command_buffer), Some(event), stage_mask);
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
        (self.fp_cmd_wait_events.unwrap())(
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
        (self.fp_cmd_pipeline_barrier.unwrap())(
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
        (self.fp_cmd_begin_query.unwrap())(Some(command_buffer), Some(query_pool), query, flags);
    }
    pub unsafe fn cmd_end_query(&self, command_buffer: vk::CommandBuffer, query_pool: vk::QueryPool, query: u32) {
        (self.fp_cmd_end_query.unwrap())(Some(command_buffer), Some(query_pool), query);
    }
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_conditional_rendering_begin: &vk::ConditionalRenderingBeginInfoEXT,
    ) {
        (self.fp_cmd_begin_conditional_rendering_ext.unwrap())(Some(command_buffer), p_conditional_rendering_begin);
    }
    pub unsafe fn cmd_end_conditional_rendering_ext(&self, command_buffer: vk::CommandBuffer) {
        (self.fp_cmd_end_conditional_rendering_ext.unwrap())(Some(command_buffer));
    }
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self.fp_cmd_reset_query_pool.unwrap())(Some(command_buffer), Some(query_pool), first_query, query_count);
    }
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        (self.fp_cmd_write_timestamp.unwrap())(Some(command_buffer), pipeline_stage, Some(query_pool), query);
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
        (self.fp_cmd_copy_query_pool_results.unwrap())(
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
        (self.fp_cmd_push_constants.unwrap())(Some(command_buffer), Some(layout), stage_flags, offset, size, p_values);
    }
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: vk::CommandBuffer,
        p_render_pass_begin: &vk::RenderPassBeginInfo,
        contents: vk::SubpassContents,
    ) {
        (self.fp_cmd_begin_render_pass.unwrap())(Some(command_buffer), p_render_pass_begin, contents);
    }
    pub unsafe fn cmd_next_subpass(&self, command_buffer: vk::CommandBuffer, contents: vk::SubpassContents) {
        (self.fp_cmd_next_subpass.unwrap())(Some(command_buffer), contents);
    }
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: vk::CommandBuffer) {
        (self.fp_cmd_end_render_pass.unwrap())(Some(command_buffer));
    }
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: vk::CommandBuffer,
        p_command_buffers: &[vk::CommandBuffer],
    ) {
        let command_buffer_count = p_command_buffers.len() as u32;
        (self.fp_cmd_execute_commands.unwrap())(Some(command_buffer), command_buffer_count, p_command_buffers.as_ptr());
    }
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        p_create_infos: &[vk::SwapchainCreateInfoKHR],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_swapchains: *mut vk::SwapchainKHR,
    ) -> Result<()> {
        let swapchain_count = p_create_infos.len() as u32;
        let v_err = (self.fp_create_shared_swapchains_khr.unwrap())(
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
        let v_err = (self.fp_create_shared_swapchains_khr.unwrap())(
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
        let v_err = (self.fp_create_shared_swapchains_khr.unwrap())(
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
        let v_err = (self.fp_create_shared_swapchains_khr.unwrap())(
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
    pub unsafe fn create_swapchain_khr(
        &self,
        p_create_info: &vk::SwapchainCreateInfoKHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SwapchainKHR> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_swapchain_khr.unwrap())(
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
        (self.fp_destroy_swapchain_khr.unwrap())(Some(self.handle), swapchain, p_allocator.map_or(ptr::null(), |r| r));
    }
    pub unsafe fn get_swapchain_images_khr_to_vec(&self, swapchain: vk::SwapchainKHR) -> Result<Vec<vk::Image>> {
        let mut len = mem::uninitialized();
        let len_err =
            (self.fp_get_swapchain_images_khr.unwrap())(Some(self.handle), Some(swapchain), &mut len, ptr::null_mut());
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err =
            (self.fp_get_swapchain_images_khr.unwrap())(Some(self.handle), Some(swapchain), &mut len, v.as_mut_ptr());
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
        let err = (self.fp_acquire_next_image_khr.unwrap())(
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
        let err = (self.fp_queue_present_khr.unwrap())(Some(queue), p_present_info);
        let res = match err {
            vk::Result::SUCCESS | vk::Result::SUBOPTIMAL_KHR => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        p_name_info: &vk::DebugMarkerObjectNameInfoEXT,
    ) -> Result<()> {
        let err = (self.fp_debug_marker_set_object_name_ext.unwrap())(Some(self.handle), p_name_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn debug_marker_set_object_tag_ext(&self, p_tag_info: &vk::DebugMarkerObjectTagInfoEXT) -> Result<()> {
        let err = (self.fp_debug_marker_set_object_tag_ext.unwrap())(Some(self.handle), p_tag_info);
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
        (self.fp_cmd_debug_marker_begin_ext.unwrap())(Some(command_buffer), p_marker_info);
    }
    pub unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: vk::CommandBuffer) {
        (self.fp_cmd_debug_marker_end_ext.unwrap())(Some(command_buffer));
    }
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_marker_info: &vk::DebugMarkerMarkerInfoEXT,
    ) {
        (self.fp_cmd_debug_marker_insert_ext.unwrap())(Some(command_buffer), p_marker_info);
    }
    pub unsafe fn get_memory_win32_handle_nv(
        &self,
        memory: vk::DeviceMemory,
        handle_type: vk::ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<vk::HANDLE> {
        let mut res = mem::uninitialized();
        let err = (self.fp_get_memory_win32_handle_nv.unwrap())(Some(self.handle), Some(memory), handle_type, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn cmd_process_commands_nvx(
        &self,
        command_buffer: vk::CommandBuffer,
        p_process_commands_info: &vk::CmdProcessCommandsInfoNVX,
    ) {
        (self.fp_cmd_process_commands_nvx.unwrap())(Some(command_buffer), p_process_commands_info);
    }
    pub unsafe fn cmd_reserve_space_for_commands_nvx(
        &self,
        command_buffer: vk::CommandBuffer,
        p_reserve_space_info: &vk::CmdReserveSpaceForCommandsInfoNVX,
    ) {
        (self.fp_cmd_reserve_space_for_commands_nvx.unwrap())(Some(command_buffer), p_reserve_space_info);
    }
    pub unsafe fn create_indirect_commands_layout_nvx(
        &self,
        p_create_info: &vk::IndirectCommandsLayoutCreateInfoNVX,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::IndirectCommandsLayoutNVX> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_indirect_commands_layout_nvx.unwrap())(
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
        (self.fp_destroy_indirect_commands_layout_nvx.unwrap())(
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
        let err = (self.fp_create_object_table_nvx.unwrap())(
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
        (self.fp_destroy_object_table_nvx.unwrap())(
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
        let err = (self.fp_register_objects_nvx.unwrap())(
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
        let err = (self.fp_unregister_objects_nvx.unwrap())(
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
        (self.fp_get_physical_device_generated_commands_properties_nvx.unwrap())(
            Some(physical_device),
            p_features,
            p_limits,
        );
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
        (self.fp_cmd_push_descriptor_set_khr.unwrap())(
            Some(command_buffer),
            pipeline_bind_point,
            Some(layout),
            set,
            descriptor_write_count,
            p_descriptor_writes.as_ptr(),
        );
    }
    pub unsafe fn trim_command_pool(&self, command_pool: vk::CommandPool, flags: vk::CommandPoolTrimFlags) {
        (self.fp_trim_command_pool.unwrap())(Some(self.handle), Some(command_pool), flags);
    }
    pub unsafe fn trim_command_pool_khr(&self, command_pool: vk::CommandPool, flags: vk::CommandPoolTrimFlags) {
        (self.fp_trim_command_pool_khr.unwrap())(Some(self.handle), Some(command_pool), flags);
    }
    pub unsafe fn get_memory_win32_handle_khr(
        &self,
        p_get_win32_handle_info: &vk::MemoryGetWin32HandleInfoKHR,
    ) -> Result<vk::HANDLE> {
        let mut res = mem::uninitialized();
        let err = (self.fp_get_memory_win32_handle_khr.unwrap())(Some(self.handle), p_get_win32_handle_info, &mut res);
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
        let err = (self.fp_get_memory_win32_handle_properties_khr.unwrap())(
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
    pub unsafe fn get_memory_fd_khr(&self, p_get_fd_info: &vk::MemoryGetFdInfoKHR) -> Result<c_int> {
        let mut res = mem::uninitialized();
        let err = (self.fp_get_memory_fd_khr.unwrap())(Some(self.handle), p_get_fd_info, &mut res);
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
        let err =
            (self.fp_get_memory_fd_properties_khr.unwrap())(Some(self.handle), handle_type, fd, p_memory_fd_properties);
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
        let err =
            (self.fp_get_semaphore_win32_handle_khr.unwrap())(Some(self.handle), p_get_win32_handle_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn import_semaphore_win32_handle_khr(
        &self,
        p_import_semaphore_win32_handle_info: &vk::ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result<()> {
        let err = (self.fp_import_semaphore_win32_handle_khr.unwrap())(
            Some(self.handle),
            p_import_semaphore_win32_handle_info,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_semaphore_fd_khr(&self, p_get_fd_info: &vk::SemaphoreGetFdInfoKHR) -> Result<c_int> {
        let mut res = mem::uninitialized();
        let err = (self.fp_get_semaphore_fd_khr.unwrap())(Some(self.handle), p_get_fd_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        p_import_semaphore_fd_info: &vk::ImportSemaphoreFdInfoKHR,
    ) -> Result<()> {
        let err = (self.fp_import_semaphore_fd_khr.unwrap())(Some(self.handle), p_import_semaphore_fd_info);
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
        let err = (self.fp_get_fence_win32_handle_khr.unwrap())(Some(self.handle), p_get_win32_handle_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn import_fence_win32_handle_khr(
        &self,
        p_import_fence_win32_handle_info: &vk::ImportFenceWin32HandleInfoKHR,
    ) -> Result<()> {
        let err = (self.fp_import_fence_win32_handle_khr.unwrap())(Some(self.handle), p_import_fence_win32_handle_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_fence_fd_khr(&self, p_get_fd_info: &vk::FenceGetFdInfoKHR) -> Result<c_int> {
        let mut res = mem::uninitialized();
        let err = (self.fp_get_fence_fd_khr.unwrap())(Some(self.handle), p_get_fd_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn import_fence_fd_khr(&self, p_import_fence_fd_info: &vk::ImportFenceFdInfoKHR) -> Result<()> {
        let err = (self.fp_import_fence_fd_khr.unwrap())(Some(self.handle), p_import_fence_fd_info);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn display_power_control_ext(
        &self,
        display: vk::DisplayKHR,
        p_display_power_info: &vk::DisplayPowerInfoEXT,
    ) -> Result<()> {
        let err = (self.fp_display_power_control_ext.unwrap())(Some(self.handle), Some(display), p_display_power_info);
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
        let err = (self.fp_register_device_event_ext.unwrap())(
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
        let err = (self.fp_register_display_event_ext.unwrap())(
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
        let err = (self.fp_get_swapchain_counter_ext.unwrap())(Some(self.handle), Some(swapchain), counter, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
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
        (self.fp_get_device_group_peer_memory_features.unwrap())(
            Some(self.handle),
            heap_index,
            local_device_index,
            remote_device_index,
            &mut res,
        );
        res
    }
    pub unsafe fn get_device_group_peer_memory_features_khr(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> vk::PeerMemoryFeatureFlags {
        let mut res = mem::uninitialized();
        (self.fp_get_device_group_peer_memory_features_khr.unwrap())(
            Some(self.handle),
            heap_index,
            local_device_index,
            remote_device_index,
            &mut res,
        );
        res
    }
    pub unsafe fn bind_buffer_memory2(&self, p_bind_infos: &[vk::BindBufferMemoryInfo]) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp_bind_buffer_memory2.unwrap())(Some(self.handle), bind_info_count, p_bind_infos.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn bind_buffer_memory2_khr(&self, p_bind_infos: &[vk::BindBufferMemoryInfo]) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp_bind_buffer_memory2_khr.unwrap())(Some(self.handle), bind_info_count, p_bind_infos.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn bind_image_memory2(&self, p_bind_infos: &[vk::BindImageMemoryInfo]) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp_bind_image_memory2.unwrap())(Some(self.handle), bind_info_count, p_bind_infos.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn bind_image_memory2_khr(&self, p_bind_infos: &[vk::BindImageMemoryInfo]) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp_bind_image_memory2_khr.unwrap())(Some(self.handle), bind_info_count, p_bind_infos.as_ptr());
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: vk::CommandBuffer, device_mask: u32) {
        (self.fp_cmd_set_device_mask.unwrap())(Some(command_buffer), device_mask);
    }
    pub unsafe fn cmd_set_device_mask_khr(&self, command_buffer: vk::CommandBuffer, device_mask: u32) {
        (self.fp_cmd_set_device_mask_khr.unwrap())(Some(command_buffer), device_mask);
    }
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        p_device_group_present_capabilities: &mut vk::DeviceGroupPresentCapabilitiesKHR,
    ) -> Result<()> {
        let err = (self.fp_get_device_group_present_capabilities_khr.unwrap())(
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
        let err =
            (self.fp_get_device_group_surface_present_modes_khr.unwrap())(Some(self.handle), Some(surface), &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn acquire_next_image2_khr(
        &self,
        p_acquire_info: &vk::AcquireNextImageInfoKHR,
    ) -> Result<(vk::Result, u32)> {
        let mut res = mem::uninitialized();
        let err = (self.fp_acquire_next_image2_khr.unwrap())(Some(self.handle), p_acquire_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS | vk::Result::TIMEOUT | vk::Result::NOT_READY | vk::Result::SUBOPTIMAL_KHR => {
                Ok((err, res))
            }
            _ => Err(err),
        };
        res
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
        (self.fp_cmd_dispatch_base.unwrap())(
            Some(command_buffer),
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }
    pub unsafe fn cmd_dispatch_base_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self.fp_cmd_dispatch_base_khr.unwrap())(
            Some(command_buffer),
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }
    pub unsafe fn get_physical_device_present_rectangles_khr_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<Vec<vk::Rect2D>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_get_physical_device_present_rectangles_khr.unwrap())(
            Some(physical_device),
            Some(surface),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_present_rectangles_khr.unwrap())(
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
    pub unsafe fn create_descriptor_update_template(
        &self,
        p_create_info: &vk::DescriptorUpdateTemplateCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorUpdateTemplate> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_descriptor_update_template.unwrap())(
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
    pub unsafe fn create_descriptor_update_template_khr(
        &self,
        p_create_info: &vk::DescriptorUpdateTemplateCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DescriptorUpdateTemplate> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_descriptor_update_template_khr.unwrap())(
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
        (self.fp_destroy_descriptor_update_template.unwrap())(
            Some(self.handle),
            descriptor_update_template,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn destroy_descriptor_update_template_khr(
        &self,
        descriptor_update_template: Option<vk::DescriptorUpdateTemplate>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp_destroy_descriptor_update_template_khr.unwrap())(
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
        (self.fp_update_descriptor_set_with_template.unwrap())(
            Some(self.handle),
            Some(descriptor_set),
            Some(descriptor_update_template),
            p_data,
        );
    }
    pub unsafe fn update_descriptor_set_with_template_khr(
        &self,
        descriptor_set: vk::DescriptorSet,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        (self.fp_update_descriptor_set_with_template_khr.unwrap())(
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
        (self.fp_cmd_push_descriptor_set_with_template_khr.unwrap())(
            Some(command_buffer),
            Some(descriptor_update_template),
            Some(layout),
            set,
            p_data,
        );
    }
    pub unsafe fn set_hdr_metadata_ext(&self, p_swapchains: &[vk::SwapchainKHR], p_metadata: &[vk::HdrMetadataEXT]) {
        let swapchain_count = p_swapchains.len() as u32;
        assert_eq!(swapchain_count, p_metadata.len() as u32);
        (self.fp_set_hdr_metadata_ext.unwrap())(
            Some(self.handle),
            swapchain_count,
            p_swapchains.as_ptr(),
            p_metadata.as_ptr(),
        );
    }
    pub unsafe fn get_swapchain_status_khr(&self, swapchain: vk::SwapchainKHR) -> Result<vk::Result> {
        let err = (self.fp_get_swapchain_status_khr.unwrap())(Some(self.handle), Some(swapchain));
        let res = match err {
            vk::Result::SUCCESS | vk::Result::SUBOPTIMAL_KHR => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> Result<vk::RefreshCycleDurationGOOGLE> {
        let mut res = mem::uninitialized();
        let err = (self.fp_get_refresh_cycle_duration_google.unwrap())(Some(self.handle), Some(swapchain), &mut res);
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
        let len_err = (self.fp_get_past_presentation_timing_google.unwrap())(
            Some(self.handle),
            Some(swapchain),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_past_presentation_timing_google.unwrap())(
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
    pub unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        p_viewport_w_scalings: &[vk::ViewportWScalingNV],
    ) {
        let viewport_count = p_viewport_w_scalings.len() as u32;
        (self.fp_cmd_set_viewport_w_scaling_nv.unwrap())(
            Some(command_buffer),
            first_viewport,
            viewport_count,
            p_viewport_w_scalings.as_ptr(),
        );
    }
    pub unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_discard_rectangle: u32,
        p_discard_rectangles: &[vk::Rect2D],
    ) {
        let discard_rectangle_count = p_discard_rectangles.len() as u32;
        (self.fp_cmd_set_discard_rectangle_ext.unwrap())(
            Some(command_buffer),
            first_discard_rectangle,
            discard_rectangle_count,
            p_discard_rectangles.as_ptr(),
        );
    }
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        p_sample_locations_info: &vk::SampleLocationsInfoEXT,
    ) {
        (self.fp_cmd_set_sample_locations_ext.unwrap())(Some(command_buffer), p_sample_locations_info);
    }
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: vk::PhysicalDevice,
        samples: vk::SampleCountFlags,
        p_multisample_properties: &mut vk::MultisamplePropertiesEXT,
    ) {
        (self.fp_get_physical_device_multisample_properties_ext.unwrap())(
            Some(physical_device),
            samples,
            p_multisample_properties,
        );
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        p_info: &vk::BufferMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        (self.fp_get_buffer_memory_requirements2.unwrap())(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_buffer_memory_requirements2_khr(
        &self,
        p_info: &vk::BufferMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        (self.fp_get_buffer_memory_requirements2_khr.unwrap())(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_image_memory_requirements2(
        &self,
        p_info: &vk::ImageMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        (self.fp_get_image_memory_requirements2.unwrap())(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_image_memory_requirements2_khr(
        &self,
        p_info: &vk::ImageMemoryRequirementsInfo2,
        p_memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        (self.fp_get_image_memory_requirements2_khr.unwrap())(Some(self.handle), p_info, p_memory_requirements);
    }
    pub unsafe fn get_image_sparse_memory_requirements2_to_vec(
        &self,
        p_info: &vk::ImageSparseMemoryRequirementsInfo2,
    ) -> Vec<vk::SparseImageMemoryRequirements2> {
        let mut len = mem::uninitialized();
        (self.fp_get_image_sparse_memory_requirements2.unwrap())(Some(self.handle), p_info, &mut len, ptr::null_mut());
        let mut v = Vec::with_capacity(len as usize);
        (self.fp_get_image_sparse_memory_requirements2.unwrap())(Some(self.handle), p_info, &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn get_image_sparse_memory_requirements2_khr_to_vec(
        &self,
        p_info: &vk::ImageSparseMemoryRequirementsInfo2,
    ) -> Vec<vk::SparseImageMemoryRequirements2> {
        let mut len = mem::uninitialized();
        (self.fp_get_image_sparse_memory_requirements2_khr.unwrap())(
            Some(self.handle),
            p_info,
            &mut len,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(len as usize);
        (self.fp_get_image_sparse_memory_requirements2_khr.unwrap())(
            Some(self.handle),
            p_info,
            &mut len,
            v.as_mut_ptr(),
        );
        v.set_len(len as usize);
        let res = v;
        res
    }
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        p_create_info: &vk::SamplerYcbcrConversionCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SamplerYcbcrConversion> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_sampler_ycbcr_conversion.unwrap())(
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
    pub unsafe fn create_sampler_ycbcr_conversion_khr(
        &self,
        p_create_info: &vk::SamplerYcbcrConversionCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::SamplerYcbcrConversion> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_sampler_ycbcr_conversion_khr.unwrap())(
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
        (self.fp_destroy_sampler_ycbcr_conversion.unwrap())(
            Some(self.handle),
            ycbcr_conversion,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        ycbcr_conversion: Option<vk::SamplerYcbcrConversion>,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp_destroy_sampler_ycbcr_conversion_khr.unwrap())(
            Some(self.handle),
            ycbcr_conversion,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn get_device_queue2(&self, p_queue_info: &vk::DeviceQueueInfo2) -> vk::Queue {
        let mut res = mem::uninitialized();
        (self.fp_get_device_queue2.unwrap())(Some(self.handle), p_queue_info, &mut res);
        res
    }
    pub unsafe fn create_validation_cache_ext(
        &self,
        p_create_info: &vk::ValidationCacheCreateInfoEXT,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::ValidationCacheEXT> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_validation_cache_ext.unwrap())(
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
        (self.fp_destroy_validation_cache_ext.unwrap())(
            Some(self.handle),
            validation_cache,
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        validation_cache: vk::ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result<vk::Result> {
        let err = (self.fp_get_validation_cache_data_ext.unwrap())(
            Some(self.handle),
            Some(validation_cache),
            p_data_size,
            p_data,
        );
        let res = match err {
            vk::Result::SUCCESS | vk::Result::INCOMPLETE => Ok(err),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn merge_validation_caches_ext(
        &self,
        dst_cache: vk::ValidationCacheEXT,
        p_src_caches: &[vk::ValidationCacheEXT],
    ) -> Result<()> {
        let src_cache_count = p_src_caches.len() as u32;
        let err = (self.fp_merge_validation_caches_ext.unwrap())(
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
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        p_support: &mut vk::DescriptorSetLayoutSupport,
    ) {
        (self.fp_get_descriptor_set_layout_support.unwrap())(Some(self.handle), p_create_info, p_support);
    }
    pub unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        p_create_info: &vk::DescriptorSetLayoutCreateInfo,
        p_support: &mut vk::DescriptorSetLayoutSupport,
    ) {
        (self.fp_get_descriptor_set_layout_support_khr.unwrap())(Some(self.handle), p_create_info, p_support);
    }
    pub unsafe fn get_shader_info_amd(
        &self,
        pipeline: vk::Pipeline,
        shader_stage: vk::ShaderStageFlags,
        info_type: vk::ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> Result<vk::Result> {
        let err = (self.fp_get_shader_info_amd.unwrap())(
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
    pub unsafe fn set_local_dimming_amd(&self, swap_chain: vk::SwapchainKHR, local_dimming_enable: bool) {
        (self.fp_set_local_dimming_amd.unwrap())(
            Some(self.handle),
            Some(swap_chain),
            if local_dimming_enable { vk::TRUE } else { vk::FALSE },
        );
    }
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::TimeDomainEXT>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_get_physical_device_calibrateable_time_domains_ext.unwrap())(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_calibrateable_time_domains_ext.unwrap())(
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
        let v_err = (self.fp_get_calibrated_timestamps_ext.unwrap())(
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
        let v_err = (self.fp_get_calibrated_timestamps_ext.unwrap())(
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
        let v_err = (self.fp_get_calibrated_timestamps_ext.unwrap())(
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
        let v_err = (self.fp_get_calibrated_timestamps_ext.unwrap())(
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
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: &mut vk::MemoryHostPointerPropertiesEXT,
    ) -> Result<()> {
        let err = (self.fp_get_memory_host_pointer_properties_ext.unwrap())(
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
    pub unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        marker: u32,
    ) {
        (self.fp_cmd_write_buffer_marker_amd.unwrap())(
            Some(command_buffer),
            pipeline_stage,
            Some(dst_buffer),
            dst_offset,
            marker,
        );
    }
    pub unsafe fn create_render_pass2_khr(
        &self,
        p_create_info: &vk::RenderPassCreateInfo2KHR,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::RenderPass> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_render_pass2_khr.unwrap())(
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
        (self.fp_cmd_begin_render_pass2_khr.unwrap())(Some(command_buffer), p_render_pass_begin, p_subpass_begin_info);
    }
    pub unsafe fn cmd_next_subpass2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_subpass_begin_info: &vk::SubpassBeginInfoKHR,
        p_subpass_end_info: &vk::SubpassEndInfoKHR,
    ) {
        (self.fp_cmd_next_subpass2_khr.unwrap())(Some(command_buffer), p_subpass_begin_info, p_subpass_end_info);
    }
    pub unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: vk::CommandBuffer,
        p_subpass_end_info: &vk::SubpassEndInfoKHR,
    ) {
        (self.fp_cmd_end_render_pass2_khr.unwrap())(Some(command_buffer), p_subpass_end_info);
    }
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        buffer: &vk::AHardwareBuffer,
        p_properties: &mut vk::AndroidHardwareBufferPropertiesANDROID,
    ) -> Result<()> {
        let err =
            (self.fp_get_android_hardware_buffer_properties_android.unwrap())(Some(self.handle), buffer, p_properties);
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
        let err = (self.fp_get_memory_android_hardware_buffer_android.unwrap())(Some(self.handle), p_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
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
        (self.fp_cmd_draw_indirect_count_khr.unwrap())(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
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
        (self.fp_cmd_draw_indirect_count_amd.unwrap())(
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
        (self.fp_cmd_draw_indexed_indirect_count_khr.unwrap())(
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
        (self.fp_cmd_draw_indexed_indirect_count_amd.unwrap())(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn cmd_set_checkpoint_nv(&self, command_buffer: vk::CommandBuffer, p_checkpoint_marker: *const c_void) {
        (self.fp_cmd_set_checkpoint_nv.unwrap())(Some(command_buffer), p_checkpoint_marker);
    }
    pub unsafe fn get_queue_checkpoint_data_nv_to_vec(&self, queue: vk::Queue) -> Vec<vk::CheckpointDataNV> {
        let mut len = mem::uninitialized();
        (self.fp_get_queue_checkpoint_data_nv.unwrap())(Some(queue), &mut len, ptr::null_mut());
        let mut v = Vec::with_capacity(len as usize);
        (self.fp_get_queue_checkpoint_data_nv.unwrap())(Some(queue), &mut len, v.as_mut_ptr());
        v.set_len(len as usize);
        let res = v;
        res
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
        (self.fp_cmd_bind_transform_feedback_buffers_ext.unwrap())(
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
        p_counter_buffers: &[vk::Buffer],
        p_counter_buffer_offsets: Option<&[vk::DeviceSize]>,
    ) {
        let counter_buffer_count = p_counter_buffers.len() as u32;
        if let Some(s) = p_counter_buffer_offsets {
            assert_eq!(counter_buffer_count, s.len() as u32);
        }
        (self.fp_cmd_begin_transform_feedback_ext.unwrap())(
            Some(command_buffer),
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers.as_ptr(),
            p_counter_buffer_offsets.map_or(ptr::null(), |r| r.as_ptr()),
        );
    }
    pub unsafe fn cmd_end_transform_feedback_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        first_counter_buffer: u32,
        p_counter_buffers: &[vk::Buffer],
        p_counter_buffer_offsets: Option<&[vk::DeviceSize]>,
    ) {
        let counter_buffer_count = p_counter_buffers.len() as u32;
        if let Some(s) = p_counter_buffer_offsets {
            assert_eq!(counter_buffer_count, s.len() as u32);
        }
        (self.fp_cmd_end_transform_feedback_ext.unwrap())(
            Some(command_buffer),
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers.as_ptr(),
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
        (self.fp_cmd_begin_query_indexed_ext.unwrap())(Some(command_buffer), Some(query_pool), query, flags, index);
    }
    pub unsafe fn cmd_end_query_indexed_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        query_pool: vk::QueryPool,
        query: u32,
        index: u32,
    ) {
        (self.fp_cmd_end_query_indexed_ext.unwrap())(Some(command_buffer), Some(query_pool), query, index);
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
        (self.fp_cmd_draw_indirect_byte_count_ext.unwrap())(
            Some(command_buffer),
            instance_count,
            first_instance,
            Some(counter_buffer),
            counter_buffer_offset,
            counter_offset,
            vertex_stride,
        );
    }
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_exclusive_scissor: u32,
        p_exclusive_scissors: &[vk::Rect2D],
    ) {
        let exclusive_scissor_count = p_exclusive_scissors.len() as u32;
        (self.fp_cmd_set_exclusive_scissor_nv.unwrap())(
            Some(command_buffer),
            first_exclusive_scissor,
            exclusive_scissor_count,
            p_exclusive_scissors.as_ptr(),
        );
    }
    pub unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        image_view: vk::ImageView,
        image_layout: vk::ImageLayout,
    ) {
        (self.fp_cmd_bind_shading_rate_image_nv.unwrap())(Some(command_buffer), Some(image_view), image_layout);
    }
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        first_viewport: u32,
        p_shading_rate_palettes: &[vk::ShadingRatePaletteNV],
    ) {
        let viewport_count = p_shading_rate_palettes.len() as u32;
        (self.fp_cmd_set_viewport_shading_rate_palette_nv.unwrap())(
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
        (self.fp_cmd_set_coarse_sample_order_nv.unwrap())(
            Some(command_buffer),
            sample_order_type,
            custom_sample_order_count,
            p_custom_sample_orders.as_ptr(),
        );
    }
    pub unsafe fn cmd_draw_mesh_tasks_nv(&self, command_buffer: vk::CommandBuffer, task_count: u32, first_task: u32) {
        (self.fp_cmd_draw_mesh_tasks_nv.unwrap())(Some(command_buffer), task_count, first_task);
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        (self.fp_cmd_draw_mesh_tasks_indirect_nv.unwrap())(
            Some(command_buffer),
            Some(buffer),
            offset,
            draw_count,
            stride,
        );
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
        (self.fp_cmd_draw_mesh_tasks_indirect_count_nv.unwrap())(
            Some(command_buffer),
            Some(buffer),
            offset,
            Some(count_buffer),
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }
    pub unsafe fn compile_deferred_nv(&self, pipeline: vk::Pipeline, shader: u32) -> Result<()> {
        let err = (self.fp_compile_deferred_nv.unwrap())(Some(self.handle), Some(pipeline), shader);
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn create_acceleration_structure_nv(
        &self,
        p_create_info: &vk::AccelerationStructureCreateInfoNV,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::AccelerationStructureNV> {
        let mut res = mem::uninitialized();
        let err = (self.fp_create_acceleration_structure_nv.unwrap())(
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
    pub unsafe fn destroy_acceleration_structure_nv(
        &self,
        acceleration_structure: vk::AccelerationStructureNV,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp_destroy_acceleration_structure_nv.unwrap())(
            Some(self.handle),
            Some(acceleration_structure),
            p_allocator.map_or(ptr::null(), |r| r),
        );
    }
    pub unsafe fn get_acceleration_structure_memory_requirements_nv(
        &self,
        p_info: &vk::AccelerationStructureMemoryRequirementsInfoNV,
        p_memory_requirements: &mut vk::MemoryRequirements2KHR,
    ) {
        (self.fp_get_acceleration_structure_memory_requirements_nv.unwrap())(
            Some(self.handle),
            p_info,
            p_memory_requirements,
        );
    }
    pub unsafe fn bind_acceleration_structure_memory_nv(
        &self,
        p_bind_infos: &[vk::BindAccelerationStructureMemoryInfoNV],
    ) -> Result<()> {
        let bind_info_count = p_bind_infos.len() as u32;
        let err = (self.fp_bind_acceleration_structure_memory_nv.unwrap())(
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
    pub unsafe fn cmd_copy_acceleration_structure_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        mode: vk::CopyAccelerationStructureModeNV,
    ) {
        (self.fp_cmd_copy_acceleration_structure_nv.unwrap())(Some(command_buffer), Some(dst), Some(src), mode);
    }
    pub unsafe fn cmd_write_acceleration_structures_properties_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        p_acceleration_structures: &[vk::AccelerationStructureNV],
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        first_query: u32,
    ) {
        let acceleration_structure_count = p_acceleration_structures.len() as u32;
        (self.fp_cmd_write_acceleration_structures_properties_nv.unwrap())(
            Some(command_buffer),
            acceleration_structure_count,
            p_acceleration_structures.as_ptr(),
            query_type,
            Some(query_pool),
            first_query,
        );
    }
    pub unsafe fn cmd_build_acceleration_structure_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        p_info: &vk::AccelerationStructureInfoNV,
        instance_data: Option<vk::Buffer>,
        instance_offset: vk::DeviceSize,
        update: bool,
        dst: vk::AccelerationStructureNV,
        src: Option<vk::AccelerationStructureNV>,
        scratch: vk::Buffer,
        scratch_offset: vk::DeviceSize,
    ) {
        (self.fp_cmd_build_acceleration_structure_nv.unwrap())(
            Some(command_buffer),
            p_info,
            instance_data,
            instance_offset,
            if update { vk::TRUE } else { vk::FALSE },
            Some(dst),
            src,
            Some(scratch),
            scratch_offset,
        );
    }
    pub unsafe fn cmd_trace_rays_nv(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table_buffer: vk::Buffer,
        raygen_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_table_buffer: Option<vk::Buffer>,
        miss_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_stride: vk::DeviceSize,
        hit_shader_binding_table_buffer: Option<vk::Buffer>,
        hit_shader_binding_offset: vk::DeviceSize,
        hit_shader_binding_stride: vk::DeviceSize,
        callable_shader_binding_table_buffer: Option<vk::Buffer>,
        callable_shader_binding_offset: vk::DeviceSize,
        callable_shader_binding_stride: vk::DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        (self.fp_cmd_trace_rays_nv.unwrap())(
            Some(command_buffer),
            Some(raygen_shader_binding_table_buffer),
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        );
    }
    pub unsafe fn get_ray_tracing_shader_group_handles_nv(
        &self,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result<()> {
        let err = (self.fp_get_ray_tracing_shader_group_handles_nv.unwrap())(
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
    pub unsafe fn get_acceleration_structure_handle_nv(
        &self,
        acceleration_structure: vk::AccelerationStructureNV,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result<()> {
        let err = (self.fp_get_acceleration_structure_handle_nv.unwrap())(
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
    pub unsafe fn create_ray_tracing_pipelines_nv(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoNV],
        p_allocator: Option<&vk::AllocationCallbacks>,
        p_pipelines: *mut vk::Pipeline,
    ) -> Result<()> {
        let create_info_count = p_create_infos.len() as u32;
        let v_err = (self.fp_create_ray_tracing_pipelines_nv.unwrap())(
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
    pub unsafe fn create_ray_tracing_pipelines_nv_to_vec(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoNV],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::Pipeline>> {
        let create_info_count = p_create_infos.len() as u32;
        let mut v = Vec::with_capacity(create_info_count as usize);
        v.set_len(create_info_count as usize);
        let v_err = (self.fp_create_ray_tracing_pipelines_nv.unwrap())(
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
    pub unsafe fn create_ray_tracing_pipelines_nv_array<A: Array<Item = vk::Pipeline>>(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoNV],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<A> {
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, A::len() as u32);
        let mut v: A = mem::uninitialized();
        let v_err = (self.fp_create_ray_tracing_pipelines_nv.unwrap())(
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
    pub unsafe fn create_ray_tracing_pipelines_nv_single(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::RayTracingPipelineCreateInfoNV],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        let create_info_count = p_create_infos.len() as u32;
        assert_eq!(create_info_count, 1);
        let mut v = mem::uninitialized();
        let v_err = (self.fp_create_ray_tracing_pipelines_nv.unwrap())(
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
    pub unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        image: vk::Image,
        p_properties: &mut vk::ImageDrmFormatModifierPropertiesEXT,
    ) -> Result<()> {
        let err = (self.fp_get_image_drm_format_modifier_properties_ext.unwrap())(
            Some(self.handle),
            Some(image),
            p_properties,
        );
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn get_buffer_device_address_ext(&self, p_info: &vk::BufferDeviceAddressInfoEXT) -> vk::DeviceAddress {
        let res = (self.fp_get_buffer_device_address_ext.unwrap())(Some(self.handle), p_info);
        res
    }
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::CooperativeMatrixPropertiesNV>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_get_physical_device_cooperative_matrix_properties_nv.unwrap())(
            Some(physical_device),
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_cooperative_matrix_properties_nv.unwrap())(
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
    pub unsafe fn get_image_view_handle_nvx(&self, p_info: &vk::ImageViewHandleInfoNVX) -> u32 {
        let res = (self.fp_get_image_view_handle_nvx.unwrap())(Some(self.handle), p_info);
        res
    }
    pub unsafe fn get_physical_device_surface_present_modes2_ext_to_vec(
        &self,
        physical_device: vk::PhysicalDevice,
        p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<Vec<vk::PresentModeKHR>> {
        let mut len = mem::uninitialized();
        let len_err = (self.fp_get_physical_device_surface_present_modes2_ext.unwrap())(
            Some(physical_device),
            p_surface_info,
            &mut len,
            ptr::null_mut(),
        );
        if len_err != vk::Result::SUCCESS {
            return Err(len_err);
        }
        let mut v = Vec::with_capacity(len as usize);
        let v_err = (self.fp_get_physical_device_surface_present_modes2_ext.unwrap())(
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
    pub unsafe fn get_device_group_surface_present_modes2_ext(
        &self,
        p_surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut res = mem::uninitialized();
        let err =
            (self.fp_get_device_group_surface_present_modes2_ext.unwrap())(Some(self.handle), p_surface_info, &mut res);
        let res = match err {
            vk::Result::SUCCESS => Ok(res),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(&self, swapchain: vk::SwapchainKHR) -> Result<()> {
        let err = (self.fp_acquire_full_screen_exclusive_mode_ext.unwrap())(Some(self.handle), Some(swapchain));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err),
        };
        res
    }
    pub unsafe fn release_full_screen_exclusive_mode_ext(&self, swapchain: vk::SwapchainKHR) -> Result<()> {
        let err = (self.fp_release_full_screen_exclusive_mode_ext.unwrap())(Some(self.handle), Some(swapchain));
        let res = match err {
            vk::Result::SUCCESS => Ok(()),
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
