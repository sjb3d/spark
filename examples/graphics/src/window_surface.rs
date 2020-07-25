use std::ffi::CStr;
use vkr::{vk, Instance, Result};
use winit::window::Window;

#[cfg(unix)]
pub fn extension_names() -> Vec<&'static CStr> {
    vec![Instance::khr_surface_name(), Instance::khr_xlib_surface_name()]
}

#[cfg(windows)]
pub fn extension_names() -> Vec<&'static CStr> {
    vec![Instance::khr_surface_name(), Instance::khr_win32_surface_name()]
}

#[cfg(unix)]
pub fn create(instance: &Instance, window: &Window) -> Result<vk::SurfaceKHR> {
    use winit::platform::unix::WindowExtUnix;
    let x11_display = window.xlib_display().unwrap();
    let x11_window = window.xlib_window().unwrap();
    let create_info = vk::XlibSurfaceCreateInfoKHR {
        dpy: x11_display as _,
        window: x11_window,
        ..Default::default()
    };
    unsafe { instance.create_xlib_surface_khr(&create_info, None) }
}

#[cfg(windows)]
pub fn create(instance: &Instance, window: &Window) -> Result<vk::SurfaceKHR> {
    use winit::platform::windows::WindowExtWindows;
    let hwnd = window.hwnd();
    let create_info = vk::Win32SurfaceCreateInfoKHR {
        hwnd: hwnd as _,
        ..Default::default()
    };
    unsafe { instance.create_win32_surface_khr(&create_info, None) }
}
