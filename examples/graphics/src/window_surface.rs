use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::CStr;
use vkr::{vk, Instance, Result};
use winit::window::Window;

pub fn extension_names(window: &Window) -> Vec<&'static CStr> {
    match window.raw_window_handle() {
        #[cfg(target_os = "linux")]
        RawWindowHandle::Xlib(..) => vec![Instance::khr_surface_name(), Instance::khr_xlib_surface_name()],

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(..) => vec![Instance::khr_surface_name(), Instance::khr_win32_surface_name()],

        _ => unimplemented!(),
    }
}

pub fn create(instance: &Instance, window: &Window) -> Result<vk::SurfaceKHR> {
    match window.raw_window_handle() {
        #[cfg(target_os = "linux")]
        RawWindowHandle::Xlib(handle) => {
            let create_info = vk::XlibSurfaceCreateInfoKHR {
                dpy: handle.display as _,
                window: handle.window,
                ..Default::default()
            };
            unsafe { instance.create_xlib_surface_khr(&create_info, None) }
        }

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(handle) => {
            let create_info = vk::Win32SurfaceCreateInfoKHR {
                hwnd: handle.hwnd,
                ..Default::default()
            };
            unsafe { instance.create_win32_surface_khr(&create_info, None) }
        }

        _ => unimplemented!(),
    }
}
