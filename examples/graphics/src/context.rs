use crate::window_surface;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use spark::{vk, Builder, Device, DeviceExtensions, Globals, Instance, InstanceExtensions};
use std::ffi::CStr;
use std::os::raw::c_void;
use std::slice;
use winit::window::Window;

unsafe extern "system" fn debug_messenger(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut c_void,
) -> vk::Bool32 {
    if let Some(data) = p_callback_data.as_ref() {
        let message = CStr::from_ptr(data.p_message);
        println!("{}, {}: {:?}", message_severity, message_types, message);
    }
    vk::FALSE
}

pub trait InstanceExt {
    fn get_queue_family_index(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_flags: vk::QueueFlags,
        surface: vk::SurfaceKHR,
    ) -> Option<u32>;
}

impl InstanceExt for Instance {
    fn get_queue_family_index(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_flags: vk::QueueFlags,
        surface: vk::SurfaceKHR,
    ) -> Option<u32> {
        unsafe { self.get_physical_device_queue_family_properties_to_vec(physical_device) }
            .iter()
            .enumerate()
            .filter_map(|(index, &info)| {
                if info.queue_flags.contains(queue_flags)
                    && unsafe { self.get_physical_device_surface_support_khr(physical_device, index as u32, surface) }
                        .unwrap()
                {
                    Some(index as u32)
                } else {
                    None
                }
            })
            .next()
    }
}

pub struct Context {
    pub _globals: Globals,
    pub instance: Instance,
    pub debug_utils_messenger: vk::DebugUtilsMessengerEXT,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub physical_device_properties: vk::PhysicalDeviceProperties,
    pub physical_device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub queue_family_index: u32,
    pub device: Device,
    pub queue: vk::Queue,
}

impl Context {
    pub fn new(window: &Window, version: vk::Version, is_debug: bool) -> Self {
        let globals = Globals::new().unwrap();

        let display_handle = window.raw_display_handle();
        let instance = {
            let mut extensions = InstanceExtensions::new(version);
            window_surface::enable_extensions(&display_handle, &mut extensions);
            if is_debug {
                extensions.enable_ext_debug_utils();
            }

            #[cfg(target_os = "macos")]
            extensions.enable_khr_portability_enumeration();

            let extension_names = extensions.to_name_vec();

            let app_info = vk::ApplicationInfo::builder()
                .p_application_name(Some(c"graphics"))
                .api_version(version);

            let extension_name_ptrs: Vec<_> = extension_names.iter().map(|s| s.as_ptr()).collect();
            let instance_create_info = vk::InstanceCreateInfo::builder()
                .p_application_info(Some(&app_info))
                .pp_enabled_extension_names(&extension_name_ptrs);

            #[cfg(target_os = "macos")]
            let instance_create_info = instance_create_info.flags(vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR);

            unsafe { globals.create_instance_commands(&instance_create_info, None) }.unwrap()
        };

        let debug_utils_messenger = if is_debug {
            let create_info = vk::DebugUtilsMessengerCreateInfoEXT {
                message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
                message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                pfn_user_callback: Some(debug_messenger),
                ..Default::default()
            };
            unsafe { instance.create_debug_utils_messenger_ext(&create_info, None) }.unwrap()
        } else {
            vk::DebugUtilsMessengerEXT::null()
        };

        let window_handle = window.raw_window_handle();
        let surface = window_surface::create(&instance, &display_handle, &window_handle).unwrap();

        let physical_device = {
            let physical_devices = unsafe { instance.enumerate_physical_devices_to_vec() }.unwrap();
            physical_devices[0]
        };
        let physical_device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let physical_device_memory_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device) };

        let queue_family_index = instance
            .get_queue_family_index(physical_device, vk::QueueFlags::GRAPHICS, surface)
            .unwrap();

        let device = {
            let queue_priorities = [1.0];
            let device_queue_create_info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family_index)
                .p_queue_priorities(&queue_priorities);

            let mut extensions = DeviceExtensions::new(version);
            extensions.enable_khr_swapchain();
            let extension_names = extensions.to_name_vec();

            let extension_name_ptrs: Vec<_> = extension_names.iter().map(|s| s.as_ptr()).collect();
            let device_create_info = vk::DeviceCreateInfo::builder()
                .p_queue_create_infos(slice::from_ref(&device_queue_create_info))
                .pp_enabled_extension_names(&extension_name_ptrs);
            unsafe { instance.create_device_commands(&globals, physical_device, &device_create_info, None) }.unwrap()
        };

        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        Self {
            _globals: globals,
            instance,
            debug_utils_messenger,
            surface,
            physical_device,
            physical_device_properties,
            physical_device_memory_properties,
            queue_family_index,
            device,
            queue,
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_surface_khr(self.surface, None);
            if !self.debug_utils_messenger.is_null() {
                self.instance
                    .destroy_debug_utils_messenger_ext(self.debug_utils_messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
