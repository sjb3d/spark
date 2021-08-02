mod command_buffer;
mod context;
mod swapchain;
mod window_surface;

use command_buffer::*;
use context::*;
use imgui::im_str;
use imgui::Key;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use spark::{vk, Builder};
use std::collections::HashMap;
use std::env;
use std::f32::consts::PI;
use std::ffi::CStr;
use std::mem;
use std::slice;
use std::sync::Arc;
use std::time::Instant;
use swapchain::*;
use winit::{
    dpi::{LogicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowBuilder},
};

struct SwapTarget {
    context: Arc<Context>,
    image_view: vk::ImageView,
    framebuffer: vk::Framebuffer,
}

impl SwapTarget {
    fn new(context: &Arc<Context>, render_pass: vk::RenderPass, swapchain: &Swapchain, image_index: u32) -> Self {
        let image = swapchain.image(image_index);
        let format = swapchain.format();
        let extent = swapchain.extent();

        let image_view_create_info = vk::ImageViewCreateInfo {
            image: Some(image),
            view_type: vk::ImageViewType::N2D,
            format,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };
        let image_view = unsafe { context.device.create_image_view(&image_view_create_info, None) }.unwrap();

        let attachments = [image_view];
        let framebuffer_create_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .p_attachments(&attachments)
            .width(extent.width)
            .height(extent.height)
            .layers(1);
        let framebuffer = unsafe { context.device.create_framebuffer(&framebuffer_create_info, None) }.unwrap();

        Self {
            context: Arc::clone(context),
            image_view,
            framebuffer,
        }
    }
}

impl Drop for SwapTarget {
    fn drop(&mut self) {
        let device = &self.context.device;
        unsafe {
            device.destroy_framebuffer(Some(self.framebuffer), None);
            device.destroy_image_view(Some(self.image_view), None);
        }
    }
}

struct App {
    context: Arc<Context>,
    ui_context: imgui::Context,
    ui_platform: WinitPlatform,
    ui_renderer: spark_imgui::Renderer,
    ui_pipeline: vk::Pipeline,
    last_instant: Instant,

    swapchain: Swapchain,
    recreate_swapchain: bool,
    command_buffer_pool: CommandBufferPool,

    render_pass: vk::RenderPass,
    vertex_shader: vk::ShaderModule,
    fragment_shader: vk::ShaderModule,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,

    swap_targets: HashMap<u32, SwapTarget>,
    old_swap_targets: Vec<SwapTarget>,

    frame_index: u32,
    angle: f32,
}

#[repr(C)]
struct TestData {
    angle: f32,
    x_scale: f32,
}

impl App {
    const SWAPCHAIN_USAGE: vk::ImageUsageFlags = vk::ImageUsageFlags::COLOR_ATTACHMENT;

    fn new(window: &Window, version: vk::Version, is_debug: bool) -> Self {
        let context = Arc::new(Context::new(window, version, is_debug));

        println!(
            "physical device ({}): {:?}",
            context.physical_device_properties.device_type,
            unsafe { CStr::from_ptr(context.physical_device_properties.device_name.as_ptr()) }
        );

        let mut ui_context = imgui::Context::create();
        ui_context.fonts().add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                size_pixels: 13.0,
                ..Default::default()
            }),
        }]);

        let mut ui_platform = WinitPlatform::init(&mut ui_context);
        ui_platform.attach_window(ui_context.io_mut(), window, HiDpiMode::Default);

        let ui_renderer = spark_imgui::Renderer::new(
            &context.device,
            &context.physical_device_properties,
            &context.physical_device_memory_properties,
            &mut ui_context,
        );

        let swapchain = Swapchain::new(&context, Self::SWAPCHAIN_USAGE);
        let command_buffer_pool = CommandBufferPool::new(&context);

        let render_pass = {
            let attachments = [vk::AttachmentDescription {
                flags: vk::AttachmentDescriptionFlags::empty(),
                format: swapchain.format(),
                samples: vk::SampleCountFlags::N1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
                stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
                initial_layout: vk::ImageLayout::UNDEFINED,
                final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            }];
            let subpass_color_attachment = vk::AttachmentReference {
                attachment: 0,
                layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            };
            let subpass_description = vk::SubpassDescription::builder()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .p_color_attachments(slice::from_ref(&subpass_color_attachment), None);
            let render_pass_create_info = vk::RenderPassCreateInfo::builder()
                .p_attachments(&attachments)
                .p_subpasses(slice::from_ref(&subpass_description));
            unsafe { context.device.create_render_pass(&render_pass_create_info, None) }.unwrap()
        };

        let vertex_shader = {
            let shader_bytes = include_bytes!("test.vert.spv");
            let shader_module_create_info = vk::ShaderModuleCreateInfo {
                code_size: shader_bytes.len(),
                p_code: shader_bytes.as_ptr() as _,
                ..Default::default()
            };
            unsafe { context.device.create_shader_module(&shader_module_create_info, None) }.unwrap()
        };
        let fragment_shader = {
            let shader_bytes = include_bytes!("test.frag.spv");
            let shader_module_create_info = vk::ShaderModuleCreateInfo {
                code_size: shader_bytes.len(),
                p_code: shader_bytes.as_ptr() as _,
                ..Default::default()
            };
            unsafe { context.device.create_shader_module(&shader_module_create_info, None) }.unwrap()
        };
        let pipeline_layout = {
            let push_constant_range = vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::VERTEX,
                offset: 0,
                size: mem::size_of::<TestData>() as u32,
            };
            let pipeline_layout_create_info =
                vk::PipelineLayoutCreateInfo::builder().p_push_constant_ranges(slice::from_ref(&push_constant_range));
            unsafe {
                context
                    .device
                    .create_pipeline_layout(&pipeline_layout_create_info, None)
            }
            .unwrap()
        };
        let pipeline = {
            let shader_entry_name = CStr::from_bytes_with_nul(b"main\0").unwrap();
            let shader_stage_create_info = [
                vk::PipelineShaderStageCreateInfo {
                    stage: vk::ShaderStageFlags::VERTEX,
                    module: Some(vertex_shader),
                    p_name: shader_entry_name.as_ptr(),
                    ..Default::default()
                },
                vk::PipelineShaderStageCreateInfo {
                    stage: vk::ShaderStageFlags::FRAGMENT,
                    module: Some(fragment_shader),
                    p_name: shader_entry_name.as_ptr(),
                    ..Default::default()
                },
            ];

            let vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo { ..Default::default() };
            let input_assembly_state_create_info = vk::PipelineInputAssemblyStateCreateInfo {
                topology: vk::PrimitiveTopology::TRIANGLE_LIST,
                ..Default::default()
            };

            let viewport_state_create_info = vk::PipelineViewportStateCreateInfo {
                viewport_count: 1,
                scissor_count: 1,
                ..Default::default()
            };

            let rasterization_state_create_info = vk::PipelineRasterizationStateCreateInfo {
                polygon_mode: vk::PolygonMode::FILL,
                cull_mode: vk::CullModeFlags::BACK,
                front_face: vk::FrontFace::CLOCKWISE,
                line_width: 1.0,
                ..Default::default()
            };
            let multisample_state_create_info = vk::PipelineMultisampleStateCreateInfo {
                rasterization_samples: vk::SampleCountFlags::N1,
                ..Default::default()
            };

            let color_blend_attachment_state = vk::PipelineColorBlendAttachmentState {
                color_write_mask: vk::ColorComponentFlags::all(),
                ..Default::default()
            };
            let color_blend_state_create_info = vk::PipelineColorBlendStateCreateInfo::builder()
                .p_attachments(slice::from_ref(&color_blend_attachment_state));

            let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
            let pipeline_dynamic_state_create_info =
                vk::PipelineDynamicStateCreateInfo::builder().p_dynamic_states(&dynamic_states);

            let pipeline_create_info = vk::GraphicsPipelineCreateInfo::builder()
                .p_stages(&shader_stage_create_info)
                .p_vertex_input_state(Some(&vertex_input_state_create_info))
                .p_input_assembly_state(Some(&input_assembly_state_create_info))
                .p_viewport_state(Some(&viewport_state_create_info))
                .p_rasterization_state(&rasterization_state_create_info)
                .p_multisample_state(Some(&multisample_state_create_info))
                .p_color_blend_state(Some(&color_blend_state_create_info))
                .p_dynamic_state(Some(&pipeline_dynamic_state_create_info))
                .layout(pipeline_layout)
                .render_pass(render_pass);

            unsafe {
                context
                    .device
                    .create_graphics_pipelines_single(None, &pipeline_create_info, None)
            }
            .unwrap()
        };

        // let the imgui renderer create its pipeline now
        let ui_pipeline = ui_renderer.create_pipeline(&context.device, render_pass, vk::SampleCountFlags::N1);

        Self {
            context,
            ui_context,
            ui_platform,
            ui_renderer,
            ui_pipeline,
            last_instant: Instant::now(),

            swapchain,
            recreate_swapchain: false,
            command_buffer_pool,

            render_pass,
            vertex_shader,
            fragment_shader,
            pipeline_layout,
            pipeline,

            swap_targets: HashMap::new(),
            old_swap_targets: Vec::new(),

            frame_index: 0,
            angle: 0.0,
        }
    }

    fn handle_event<T>(&mut self, event: &Event<'_, T>, window: &Window) {
        match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                self.ui_context.io_mut().update_delta_time(now - self.last_instant);
                self.last_instant = now;
            }
            _ => {}
        }
        self.ui_platform.handle_event(self.ui_context.io_mut(), window, event);
    }

    fn render(&mut self, window: &Window, exit_requested: &mut bool) {
        // start creating UI for this frame
        self.ui_platform
            .prepare_frame(self.ui_context.io_mut(), window)
            .expect("failed to prepare frame");
        let ui = self.ui_context.frame();

        {
            // show some test UI
            let frame_index = self.frame_index;
            imgui::Window::new(im_str!("Debug"))
                .size([300.0, 100.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    if ui.button(im_str!("Close Window"), [0.0, 0.0]) {
                        *exit_requested = true;
                    }
                    ui.text(format!("Frame: {}", frame_index));
                });

            // also exit if ESC is pressed
            if ui.is_key_pressed(Key::Escape) {
                *exit_requested = true;
            }
        }

        // acquire a command buffer from the pool, blocks on a fence
        /*
            We want to build commands for frame N.  We have 2 command buffers
            in the pool, we when the acquire returns we know that:

            * Frame N-2 has completed its command buffer (but maybe not presented yet)
            * Frame N-1 is likely still running its command buffer

            This is the intended location of CPU/GPU synchronisation: the
            CPU waits for the GPU to consume command buffers until at most
            1 is still running.

            The GPU handles the synchronisation with the swapchain itself
            via semaphores, which in turn can limit the rate at which command
            buffers are consumed (for example using VSYNC at 60Hz).  The CPU
            does not need to handle this explicitly, on the CPU side we only
            check command buffer consumption.
        */
        let (cmd, image_available_semaphore) = self.command_buffer_pool.acquire();

        // clean up targets that are 2+ frames old
        self.old_swap_targets.clear();

        // let the imgui renderer do per-frame work
        self.ui_renderer.begin_frame(&self.context.device, cmd);

        // we want to render to the swapchain, so acquire an image from it (this usually does not block)
        let swap_image_index = loop {
            if self.recreate_swapchain {
                for (_, target) in self.swap_targets.drain() {
                    self.old_swap_targets.push(target);
                }
                self.swapchain.recreate(Self::SWAPCHAIN_USAGE);
                self.recreate_swapchain = false;
            }
            match self.swapchain.acquire(image_available_semaphore) {
                SwapchainAcquireResult::Ok(image_index) => break image_index,
                SwapchainAcquireResult::RecreateSoon(image_index) => {
                    self.recreate_swapchain = true;
                    break image_index;
                }
                SwapchainAcquireResult::RecreateNow => self.recreate_swapchain = true,
            };
        };

        // get (and keep for later) a framebuffer for this swapchain image
        let target = {
            let context = &self.context;
            let render_pass = self.render_pass;
            let swapchain = &self.swapchain;
            self.swap_targets
                .entry(swap_image_index)
                .or_insert_with(|| SwapTarget::new(context, render_pass, swapchain, swap_image_index))
        };

        // start our render pass to the swapchain
        {
            let clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.1f32, 0.1f32, 0.1f32, 0f32],
                },
            };
            let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(self.render_pass)
                .framebuffer(target.framebuffer)
                .render_area(vk::Rect2D {
                    offset: Default::default(),
                    extent: self.swapchain.extent(),
                })
                .p_clear_values(slice::from_ref(&clear_value));
            unsafe {
                self.context
                    .device
                    .cmd_begin_render_pass(cmd, &render_pass_begin_info, vk::SubpassContents::INLINE)
            };
        }

        // draw a triangle
        {
            let device = &self.context.device;

            let extent = self.swapchain.extent();
            let viewport = vk::Viewport {
                width: extent.width as f32,
                height: extent.height as f32,
                max_depth: 1.0,
                ..Default::default()
            };
            let scissor = vk::Rect2D {
                extent,
                ..Default::default()
            };

            let test_data = TestData {
                angle: self.angle,
                x_scale: (extent.height as f32) / (extent.width as f32),
            };

            unsafe {
                device.cmd_set_viewport(cmd, 0, slice::from_ref(&viewport));
                device.cmd_set_scissor(cmd, 0, slice::from_ref(&scissor));

                device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
                device.cmd_push_constants(
                    cmd,
                    self.pipeline_layout,
                    vk::ShaderStageFlags::VERTEX,
                    0,
                    slice::from_ref(&test_data),
                );
                device.cmd_draw(cmd, 3, 1, 0, 0);
            }
        }

        // draw imgui in the same render pass
        self.ui_platform.prepare_render(&ui, window);
        self.ui_renderer
            .render(ui.render(), &self.context.device, cmd, self.ui_pipeline);

        // end the render pass to the swapchain
        unsafe { self.context.device.cmd_end_render_pass(cmd) };

        // submit the command buffer and queue up the swapchain present
        let rendering_finished_semaphore = self.command_buffer_pool.submit();
        self.swapchain.present(swap_image_index, rendering_finished_semaphore);

        self.frame_index += 1;
        self.angle += PI * self.ui_context.io().delta_time;
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let device = &self.context.device;
        unsafe {
            device.device_wait_idle().unwrap();

            device.destroy_pipeline(Some(self.ui_pipeline), None);
            self.ui_renderer.delete(device);

            device.destroy_pipeline(Some(self.pipeline), None);
            device.destroy_pipeline_layout(Some(self.pipeline_layout), None);
            device.destroy_shader_module(Some(self.fragment_shader), None);
            device.destroy_shader_module(Some(self.vertex_shader), None);
            device.destroy_render_pass(Some(self.render_pass), None);
        }
    }
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    let version = Default::default();
    let mut is_debug = false;
    let mut is_fullscreen = false;
    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-d" => is_debug = true,
            "-f" => is_fullscreen = true,
            _ => panic!("unknown argument {:?}", arg),
        }
    }

    let event_loop = EventLoop::new();

    let mut window_builder = WindowBuilder::new().with_title("graphics example");
    if cfg!(target_os = "android") {
        // use window_builder defaults
    } else {
        window_builder = if is_fullscreen {
            window_builder.with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
        } else {
            window_builder.with_inner_size(Size::Logical(LogicalSize::new(480.0, 360.0)))
        };
    }
    let window = window_builder.build(&event_loop).unwrap();

    let mut app = if cfg!(target_os = "android") {
        None
    } else {
        Some(App::new(&window, version, is_debug))
    };

    let mut exit_requested = false;
    event_loop.run(move |event, _target, control_flow| {
        match event {
            Event::Resumed => {
                if app.is_none() {
                    app = Some(App::new(&window, version, is_debug));
                }
            }
            Event::Suspended | Event::LoopDestroyed => {
                app.take();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                exit_requested = true;
            }
            Event::MainEventsCleared => {
                if let Some(app) = app.as_mut() {
                    app.render(&window, &mut exit_requested);
                }
            }
            event => {
                if let Some(app) = app.as_mut() {
                    app.handle_event(&event, &window);
                }
            }
        }

        *control_flow = if exit_requested {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
    });
}
