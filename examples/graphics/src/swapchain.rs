use crate::context::*;
use spark::{vk, Builder};
use std::cmp;
use std::slice;
use std::sync::Arc;
use std::u64;

pub struct Swapchain {
    context: Arc<Context>,
    swapchain: vk::SwapchainKHR,
    surface_format: vk::SurfaceFormatKHR,
    extent: vk::Extent2D,
    images: Vec<vk::Image>,
}

pub enum SwapchainAcquireResult {
    RecreateNow,
    RecreateSoon(u32),
    Ok(u32),
}

impl Swapchain {
    const MIN_IMAGE_COUNT: u32 = 2;

    fn create(
        context: &Context,
        window_extent: vk::Extent2D,
        usage: vk::ImageUsageFlags,
        old_swapchain: vk::SwapchainKHR,
    ) -> (vk::SwapchainKHR, vk::SurfaceFormatKHR, vk::Extent2D) {
        let surface_capabilities = unsafe {
            context
                .instance
                .get_physical_device_surface_capabilities_khr(context.physical_device, context.surface)
        }
        .unwrap();
        let mut extent = surface_capabilities.current_extent;
        if extent.width == u32::MAX && extent.height == u32::MAX {
            extent = window_extent;
        }
        let surface_supported = unsafe {
            context.instance.get_physical_device_surface_support_khr(
                context.physical_device,
                context.queue_family_index,
                context.surface,
            )
        }
        .unwrap();
        if !surface_supported {
            panic!("swapchain surface not supported");
        }

        let surface_formats = unsafe {
            context
                .instance
                .get_physical_device_surface_formats_khr_to_vec(context.physical_device, context.surface)
        }
        .unwrap();

        let surface_format = surface_formats
            .iter()
            .find(|sf| match (sf.format, sf.color_space) {
                (vk::Format::R8G8B8A8_SRGB, vk::ColorSpaceKHR::SRGB_NONLINEAR) => true,
                (vk::Format::B8G8R8A8_SRGB, vk::ColorSpaceKHR::SRGB_NONLINEAR) => true,
                _ => false,
            })
            .copied()
            .expect("no supported swapchain format found");

        let min_image_count = cmp::max(Self::MIN_IMAGE_COUNT, surface_capabilities.min_image_count);

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(context.surface)
            .min_image_count(min_image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(usage)
            .p_queue_family_indices(slice::from_ref(&context.queue_family_index))
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO)
            .clipped(true)
            .old_swapchain(old_swapchain);
        let swapchain = unsafe { context.device.create_swapchain_khr(&swapchain_create_info, None) }.unwrap();

        (swapchain, surface_format, extent)
    }

    pub fn new(context: &Arc<Context>, window_extent: vk::Extent2D, usage: vk::ImageUsageFlags) -> Self {
        let (swapchain, surface_format, extent) =
            Swapchain::create(context, window_extent, usage, vk::SwapchainKHR::null());

        let images = unsafe { context.device.get_swapchain_images_khr_to_vec(swapchain) }.unwrap();

        Swapchain {
            context: Arc::clone(context),
            swapchain,
            surface_format,
            extent,
            images,
        }
    }

    pub fn recreate(&mut self, window_extent: vk::Extent2D, usage: vk::ImageUsageFlags) {
        let (swapchain, surface_format, extent) =
            Swapchain::create(&self.context, window_extent, usage, self.swapchain);
        unsafe { self.context.device.destroy_swapchain_khr(self.swapchain, None) };

        self.swapchain = swapchain;
        self.surface_format = surface_format;
        self.extent = extent;
        self.images = unsafe { self.context.device.get_swapchain_images_khr_to_vec(swapchain) }.unwrap();
    }

    pub fn acquire(
        &self,
        window_extent: vk::Extent2D,
        image_available_semaphore: vk::Semaphore,
    ) -> SwapchainAcquireResult {
        let mut image_index = 0;
        let res = unsafe {
            self.context.device.acquire_next_image_khr(
                self.swapchain,
                u64::MAX,
                image_available_semaphore,
                vk::Fence::null(),
                &mut image_index,
            )
        };
        match res {
            Ok(vk::Result::SUCCESS) => {
                if self.extent == window_extent {
                    SwapchainAcquireResult::Ok(image_index)
                } else {
                    SwapchainAcquireResult::RecreateSoon(image_index)
                }
            }
            Ok(vk::Result::SUBOPTIMAL_KHR) => SwapchainAcquireResult::RecreateSoon(image_index),
            Ok(err) => panic!("failed to acquire next image {}", err),
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => SwapchainAcquireResult::RecreateNow,
            Err(err) => panic!("failed to acquire next image {}", err),
        }
    }

    pub fn image(&self, image_index: u32) -> vk::Image {
        self.images[image_index as usize]
    }

    pub fn format(&self) -> vk::Format {
        self.surface_format.format
    }

    pub fn extent(&self) -> vk::Extent2D {
        self.extent
    }

    pub fn present(&self, image_index: u32, rendering_finished_semaphore: vk::Semaphore) {
        let present_info = vk::PresentInfoKHR::builder()
            .p_wait_semaphores(slice::from_ref(&rendering_finished_semaphore))
            .p_swapchains(slice::from_ref(&self.swapchain), slice::from_ref(&image_index), None);
        match unsafe { self.context.device.queue_present_khr(self.context.queue, &present_info) } {
            Ok(vk::Result::SUCCESS) | Ok(vk::Result::SUBOPTIMAL_KHR) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {}
            Ok(err) | Err(err) => panic!("failed to present {}", err),
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            self.context.device.destroy_swapchain_khr(self.swapchain, None);
        }
    }
}
