mod command_buffer;
mod context;
mod swapchain;
mod ui_state;
mod window_surface;

use command_buffer::*;
use context::*;
use imgui::im_str;
use std::collections::HashMap;
use std::env;
use std::ffi::CStr;
use std::mem;
use std::slice;
use std::sync::Arc;
use swapchain::*;
use ui_state::*;
use vkr::{vk, Builder};
use vkr_imgui;
use winit::{
    dpi::{LogicalSize, Size},
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Fullscreen, Window, WindowBuilder},
};

struct SwapTarget {
    context: Arc<Context>,
    image_view: vk::ImageView,
    framebuffer: vk::Framebuffer,
}

impl SwapTarget {
    fn new(context: &Arc<Context>, render_pass: vk::RenderPass, swapchain: &Swapchain, image_index: u32) -> Self {
        let image = swapchain.get_image(image_index);
        let format = Swapchain::FORMAT;
        let extent = swapchain.get_extent();

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
            context: Arc::clone(&context),
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
    window: Window,
    exit_requested: bool,

    context: Arc<Context>,
    ui_state: UiState,
    ui_renderer: vkr_imgui::Renderer,

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
}

enum AppEventResult {
    None,
    Redraw,
    Destroy,
}

#[repr(C)]
struct TestData {
    angle: f32,
    x_scale: f32,
}

impl App {
    const SWAPCHAIN_USAGE: vk::ImageUsageFlags = vk::ImageUsageFlags::COLOR_ATTACHMENT;

    fn new(window: Window, version: vk::Version, is_debug: bool) -> Self {
        let context = Arc::new(Context::new(&window, version, is_debug));

        println!(
            "physical device ({}): {:?}",
            context.physical_device_properties.device_type,
            unsafe { CStr::from_ptr(context.physical_device_properties.device_name.as_ptr()) }
        );

        let mut ui_state = UiState::new();
        let mut ui_renderer = vkr_imgui::Renderer::new(
            &context.device,
            &context.physical_device_properties,
            &context.physical_device_memory_properties,
            &mut ui_state.context,
        );

        let swapchain = Swapchain::new(&context, Self::SWAPCHAIN_USAGE);
        let command_buffer_pool = CommandBufferPool::new(&context);

        let render_pass = {
            let attachments = [vk::AttachmentDescription {
                flags: vk::AttachmentDescriptionFlags::empty(),
                format: Swapchain::FORMAT,
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
        ui_renderer.create_pipeline(&context.device, render_pass, vk::SampleCountFlags::N1);

        Self {
            window,
            exit_requested: false,

            context,
            ui_state,
            ui_renderer,

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
        }
    }

    fn process_event<T>(
        self: &mut Self,
        event: &Event<'_, T>,
        _target: &EventLoopWindowTarget<T>,
        control_flow: &mut ControlFlow,
    ) -> AppEventResult {
        let mut result = AppEventResult::None;
        match event {
            Event::WindowEvent { event, .. } => {
                self.ui_state.process_window_event(&event);
                match event {
                    WindowEvent::CloseRequested => {
                        self.exit_requested = true;
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                            self.exit_requested = true;
                        }
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                self.window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                result = AppEventResult::Redraw;
            }
            Event::LoopDestroyed => {
                result = AppEventResult::Destroy;
            }
            _ => {}
        }

        *control_flow = if self.exit_requested {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };

        result
    }

    fn render(&mut self) {
        // start creating UI for this frame
        let ui = {
            let extent = self.swapchain.get_extent();
            self.ui_state.start_ui(extent.width, extent.height)
        };
        {
            let frame_index = self.frame_index;
            let exit_requested = &mut self.exit_requested;
            imgui::Window::new(im_str!("Debug"))
                .size([300.0, 100.0], imgui::Condition::FirstUseEver)
                .build(&ui, || {
                    if ui.button(im_str!("Close Window"), [0.0, 0.0]) {
                        *exit_requested = true;
                    }
                    ui.text(format!("Frame: {}", frame_index));
                });
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
            let mut clear_value: vk::ClearValue = Default::default();
            clear_value.color.float32 = [0.1f32, 0.1f32, 0.1f32, 0f32];
            let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(self.render_pass)
                .framebuffer(target.framebuffer)
                .render_area(vk::Rect2D {
                    offset: Default::default(),
                    extent: self.swapchain.get_extent(),
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

            let extent = self.swapchain.get_extent();
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
                angle: (self.frame_index as f32) * 0.1f32,
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
        self.ui_renderer.render(ui.render(), &self.context.device, cmd);

        // end the render pass to the swapchain
        unsafe { self.context.device.cmd_end_render_pass(cmd) };

        // submit the command buffer and queue up the swapchain present
        let rendering_finished_semaphore = self.command_buffer_pool.submit();
        self.swapchain.present(swap_image_index, rendering_finished_semaphore);

        self.frame_index += 1;
    }
}

impl Drop for App {
    fn drop(&mut self) {
        let device = &self.context.device;
        unsafe {
            device.device_wait_idle().unwrap();

            self.ui_renderer.delete(device);

            device.destroy_pipeline(Some(self.pipeline), None);
            device.destroy_pipeline_layout(Some(self.pipeline_layout), None);
            device.destroy_shader_module(Some(self.fragment_shader), None);
            device.destroy_shader_module(Some(self.vertex_shader), None);
            device.destroy_render_pass(Some(self.render_pass), None);
        }
    }
}

fn main() {
    let version = Default::default();
    let mut is_debug = false;
    let mut is_fullscreen = false;
    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "-d" => is_debug = true,
            "-f" => is_fullscreen = true,
            _ => panic!("unknown argument {:?}", arg),
        }
    }

    let event_loop = EventLoop::new();

    let mut window_builder = WindowBuilder::new().with_title("graphics example");
    window_builder = if is_fullscreen {
        window_builder.with_fullscreen(Some(Fullscreen::Borderless(event_loop.primary_monitor())))
    } else {
        window_builder.with_inner_size(Size::Logical(LogicalSize::new(480.0, 360.0)))
    };
    let window = window_builder.build(&event_loop).unwrap();

    let mut app = Some(App::new(window, version, is_debug));
    event_loop.run(move |event, target, control_flow| {
        match app.as_mut().unwrap().process_event(&event, target, control_flow) {
            AppEventResult::None => {}
            AppEventResult::Redraw => {
                app.as_mut().unwrap().render();
            }
            AppEventResult::Destroy => {
                app.take();
            }
        }
    });
}
