use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use spark::{vk, Instance, InstanceExtensions, Result};

pub fn enable_extensions(display_handle: &RawDisplayHandle, extensions: &mut InstanceExtensions) {
    match display_handle {
        #[cfg(target_os = "linux")]
        RawDisplayHandle::Xlib(_) => extensions.enable_khr_xlib_surface(),

        #[cfg(target_os = "linux")]
        RawDisplayHandle::Wayland(_) => extensions.enable_khr_wayland_surface(),

        #[cfg(target_os = "windows")]
        RawDisplayHandle::Windows(_) => extensions.enable_khr_win32_surface(),

        #[cfg(target_os = "android")]
        RawDisplayHandle::AndroidNdk(_) => extensions.enable_khr_android_surface(),

        #[cfg(target_os = "macos")]
        RawDisplayHandle::AppKit(_) => extensions.enable_ext_metal_surface(),

        _ => unimplemented!(),
    }
}

pub fn create(
    instance: &Instance,
    display_handle: &RawDisplayHandle,
    window_handle: &RawWindowHandle,
) -> Result<vk::SurfaceKHR> {
    match (display_handle, window_handle) {
        #[cfg(target_os = "linux")]
        (RawDisplayHandle::Xlib(display_handle), RawWindowHandle::Xlib(window_handle)) => {
            let create_info = vk::XlibSurfaceCreateInfoKHR {
                dpy: display_handle.display as _,
                window: window_handle.window,
                ..Default::default()
            };
            unsafe { instance.create_xlib_surface_khr(&create_info, None) }
        }

        #[cfg(target_os = "linux")]
        (RawDisplayHandle::Wayland(display_handle), RawWindowHandle::Wayland(window_handle)) => {
            let create_info = vk::WaylandSurfaceCreateInfoKHR {
                display: display_handle.display as _,
                surface: window_handle.surface as _,
                ..Default::default()
            };
            unsafe { instance.create_wayland_surface_khr(&create_info, None) }
        }

        #[cfg(target_os = "windows")]
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window_handle)) => {
            let create_info = vk::Win32SurfaceCreateInfoKHR {
                hwnd: window_handle.hwnd,
                ..Default::default()
            };
            unsafe { instance.create_win32_surface_khr(&create_info, None) }
        }

        #[cfg(target_os = "android")]
        (RawDisplayHandle::AndroidNdk(_), RawWindowHandle::AndroidNdk(window_handle)) => {
            let create_info = vk::AndroidSurfaceCreateInfoKHR {
                window: window_handle.a_native_window as _,
                ..Default::default()
            };
            unsafe { instance.create_android_surface_khr(&create_info, None) }
        }

        #[cfg(target_os = "macos")]
        (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window_handle)) => {
            use raw_window_metal::Layer;
            use spark::vk::CAMetalLayer;
            use std::ptr::NonNull;

            let layer = unsafe { Layer::from_ns_view(NonNull::new_unchecked(window_handle.ns_view)) };
            let p_layer: *mut CAMetalLayer = layer.into_raw().as_ptr().cast();

            let create_info = vk::MetalSurfaceCreateInfoEXT {
                p_layer,
                ..Default::default()
            };

            unsafe { instance.create_metal_surface_ext(&create_info, None) }
        }
        _ => unimplemented!(),
    }
}
