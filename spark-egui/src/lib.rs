use egui::{
    ahash::HashMap,
    emath,
    epaint::{ClippedPrimitive, ImageDelta, Primitive, Vertex},
    ImageData, TextureFilter, TextureId, TextureOptions, TexturesDelta,
};
use memoffset::offset_of;
use spark::{vk, Builder, Device};
use std::{borrow::Cow, collections::VecDeque, ffi::CStr, mem, ops::Range, os::raw::c_void, slice};

type Index = u32;

fn load_shader_module(device: &Device, bytes: &[u8]) -> vk::ShaderModule {
    let shader_module_create_info = vk::ShaderModuleCreateInfo {
        code_size: bytes.len(),
        p_code: bytes.as_ptr() as *const u32,
        ..Default::default()
    };
    unsafe { device.create_shader_module(&shader_module_create_info, None) }.unwrap()
}

fn get_memory_type_index(
    physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
    type_filter: u32,
    property_flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    for i in 0..physical_device_memory_properties.memory_type_count {
        let mt = &physical_device_memory_properties.memory_types[i as usize];
        if (type_filter & (1 << i)) != 0 && mt.property_flags.contains(property_flags) {
            return Some(i);
        }
    }
    None
}

fn align_up(x: u32, alignment: u32) -> u32 {
    (x + alignment - 1) & !(alignment - 1)
}

trait TakePrefix {
    type Output;
    fn take_prefix(&mut self, len: usize) -> Option<Self::Output>;
}

impl TakePrefix for Range<usize> {
    type Output = Self;
    fn take_prefix(&mut self, len: usize) -> Option<Self::Output> {
        let next = self.start + len;
        if next <= self.end {
            let prefix = self.start..next;
            self.start = next;
            Some(prefix)
        } else {
            None
        }
    }
}

#[repr(C)]
struct BatchData {
    size_in_points_rcp: (f32, f32),
}

struct ClippedDraw {
    clip_rect: emath::Rect,
    texture_id: TextureId,
    first_index: u32,
    index_count: u32,
    vertex_offset: i32,
}

struct Layouts {
    descriptor_set_layout: vk::DescriptorSetLayout,
    pipeline_layout: vk::PipelineLayout,
    vertex_shader: vk::ShaderModule,
    fragment_shader: vk::ShaderModule,
}

struct Staging {
    size: usize,
    atom_size: u32,
    mem: vk::DeviceMemory,
    mapping: *mut c_void,
    buffer: vk::Buffer,
    next_subset_index: u8,
}

struct MeshBuffers {
    vertex_buffer_size: usize,
    index_buffer_size: usize,
    mem: vk::DeviceMemory,
    vertex_buffer: vk::Buffer,
    index_buffer: vk::Buffer,
    clipped_draws: Vec<ClippedDraw>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImageState {
    TransferDest,
    ShaderRead,
}

struct Texture {
    mem: vk::DeviceMemory,
    image: vk::Image,
    image_view: vk::ImageView,
    sampler: vk::Sampler,
    descriptor_pool: vk::DescriptorPool,
    descriptor_set: vk::DescriptorSet,
    image_state: ImageState,
}

#[derive(Default)]
struct TextureSet {
    active: HashMap<TextureId, Texture>,
    pending_updates: VecDeque<(TextureId, ImageDelta)>,
    pending_frees: Vec<TextureId>,
    pending_deletes: Vec<Texture>,
}

/*
   Notes:
       Single allocation for vertex and index buffer
       Single allocation per texture
       N buffer some staging memory, copy to VB/IB/tex as needed
*/
pub struct Renderer {
    layouts: Layouts,
    staging: Staging,
    mesh_buffers: MeshBuffers,
    texture_set: TextureSet,
}

impl Layouts {
    pub fn new(device: &Device) -> Self {
        let descriptor_set_layout = {
            let binding = vk::DescriptorSetLayoutBinding::builder()
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT);
            let descriptor_set_layout_create_info =
                vk::DescriptorSetLayoutCreateInfo::builder().p_bindings(slice::from_ref(&binding));
            unsafe { device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None) }.unwrap()
        };

        let pipeline_layout = {
            let push_constant_range = vk::PushConstantRange {
                stage_flags: vk::ShaderStageFlags::VERTEX,
                offset: 0,
                size: mem::size_of::<BatchData>() as u32,
            };
            let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::builder()
                .p_set_layouts(slice::from_ref(&descriptor_set_layout))
                .p_push_constant_ranges(slice::from_ref(&push_constant_range));
            unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None) }.unwrap()
        };

        let vertex_shader = load_shader_module(device, include_bytes!("egui.vert.spv"));
        let fragment_shader = load_shader_module(device, include_bytes!("egui.frag.spv"));

        Self {
            descriptor_set_layout,
            pipeline_layout,
            vertex_shader,
            fragment_shader,
        }
    }

    pub fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_shader_module(Some(self.fragment_shader), None);
            device.destroy_shader_module(Some(self.vertex_shader), None);
            device.destroy_pipeline_layout(Some(self.pipeline_layout), None);
            device.destroy_descriptor_set_layout(Some(self.descriptor_set_layout), None);
        }
    }

    pub fn create_pipeline(
        &self,
        device: &Device,
        render_pass: vk::RenderPass,
        samples: vk::SampleCountFlags,
    ) -> vk::Pipeline {
        let shader_entry_name = CStr::from_bytes_with_nul(b"main\0").unwrap();
        let shader_stage_create_info = [
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::VERTEX,
                module: Some(self.vertex_shader),
                p_name: shader_entry_name.as_ptr(),
                ..Default::default()
            },
            vk::PipelineShaderStageCreateInfo {
                stage: vk::ShaderStageFlags::FRAGMENT,
                module: Some(self.fragment_shader),
                p_name: shader_entry_name.as_ptr(),
                ..Default::default()
            },
        ];

        let vertex_input_binding = vk::VertexInputBindingDescription {
            binding: 0,
            stride: mem::size_of::<Vertex>() as u32,
            input_rate: vk::VertexInputRate::VERTEX,
        };
        let vertex_input_attributes = [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32_SFLOAT,
                offset: offset_of!(Vertex, uv) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 2,
                binding: 0,
                format: vk::Format::R8G8B8A8_UNORM,
                offset: offset_of!(Vertex, color) as u32,
            },
        ];

        let vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .p_vertex_binding_descriptions(slice::from_ref(&vertex_input_binding))
            .p_vertex_attribute_descriptions(&vertex_input_attributes);

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
            cull_mode: vk::CullModeFlags::NONE,
            front_face: vk::FrontFace::CLOCKWISE,
            line_width: 1.0,
            ..Default::default()
        };
        let multisample_state_create_info = vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: samples,
            ..Default::default()
        };

        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo { ..Default::default() };

        let color_blend_attachment_state = vk::PipelineColorBlendAttachmentState {
            blend_enable: vk::TRUE,
            src_color_blend_factor: vk::BlendFactor::ONE,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE_MINUS_DST_ALPHA,
            dst_alpha_blend_factor: vk::BlendFactor::ONE,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::all(),
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
            .p_rasterization_state(Some(&rasterization_state_create_info))
            .p_multisample_state(Some(&multisample_state_create_info))
            .p_depth_stencil_state(Some(&depth_stencil_state))
            .p_color_blend_state(Some(&color_blend_state_create_info))
            .p_dynamic_state(Some(&pipeline_dynamic_state_create_info))
            .layout(Some(self.pipeline_layout))
            .render_pass(Some(render_pass));

        unsafe { device.create_graphics_pipelines_single(None, &pipeline_create_info, None) }.unwrap()
    }
}

impl Staging {
    pub fn new(
        device: &Device,
        physical_device_properties: &vk::PhysicalDeviceProperties,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        size_per_subset: usize,
    ) -> Self {
        let atom_size = physical_device_properties.limits.non_coherent_atom_size as u32;

        let size = 2 * size_per_subset;

        let buffer = {
            let buffer_create_info = vk::BufferCreateInfo {
                size: size as vk::DeviceSize,
                usage: vk::BufferUsageFlags::TRANSFER_SRC,
                ..Default::default()
            };
            unsafe { device.create_buffer(&buffer_create_info, None) }.unwrap()
        };

        let mem = {
            let staging_mem_req = unsafe { device.get_buffer_memory_requirements(buffer) };
            let allocation_size = staging_mem_req.size;
            let memory_type_index = get_memory_type_index(
                physical_device_memory_properties,
                staging_mem_req.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE,
            )
            .unwrap();
            let memory_allocate_info = vk::MemoryAllocateInfo {
                allocation_size,
                memory_type_index,
                ..Default::default()
            };
            unsafe { device.allocate_memory(&memory_allocate_info, None) }.unwrap()
        };

        unsafe { device.bind_buffer_memory(buffer, mem, 0) }.unwrap();

        let mapping = unsafe { device.map_memory(mem, 0, vk::WHOLE_SIZE, Default::default()) }.unwrap();

        Self {
            size,
            atom_size,
            mem,
            mapping,
            buffer,
            next_subset_index: 0,
        }
    }

    pub fn destroy(&self, device: &Device) {
        unsafe {
            device.unmap_memory(self.mem);
            device.destroy_buffer(Some(self.buffer), None);
            device.free_memory(Some(self.mem), None);
        }
    }

    pub fn next_subset(&mut self) -> Range<usize> {
        let size = self.size / 2;
        let start = if (self.next_subset_index & 1) != 0 { size } else { 0 };
        self.next_subset_index ^= 1;
        start..(start + size)
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.mapping as *mut _, self.size) }
    }

    pub fn flush_mapped_range(&self, device: &Device, range: Range<usize>) {
        if !range.is_empty() {
            let mapped_range = vk::MappedMemoryRange {
                memory: Some(self.mem),
                offset: range.start as vk::DeviceSize,
                size: align_up(range.len() as u32, self.atom_size) as vk::DeviceSize,
                ..Default::default()
            };
            unsafe { device.flush_mapped_memory_ranges(slice::from_ref(&mapped_range)) }.unwrap();
        }
    }
}

impl MeshBuffers {
    pub fn new(
        device: &Device,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        vertex_buffer_size: usize,
        index_buffer_size: usize,
    ) -> Self {
        let vertex_buffer = {
            let buffer_create_info = vk::BufferCreateInfo {
                size: vertex_buffer_size as vk::DeviceSize,
                usage: vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
                ..Default::default()
            };
            unsafe { device.create_buffer(&buffer_create_info, None) }.unwrap()
        };
        let index_buffer = {
            let buffer_create_info = vk::BufferCreateInfo {
                size: index_buffer_size as vk::DeviceSize,
                usage: vk::BufferUsageFlags::INDEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
                ..Default::default()
            };
            unsafe { device.create_buffer(&buffer_create_info, None) }.unwrap()
        };

        let mem = {
            let vertex_mem_req = unsafe { device.get_buffer_memory_requirements(vertex_buffer) };
            let index_mem_req = unsafe { device.get_buffer_memory_requirements(index_buffer) };
            let allocation_size = vertex_mem_req.size + index_mem_req.size;
            let memory_type_index = get_memory_type_index(
                physical_device_memory_properties,
                vertex_mem_req.memory_type_bits & index_mem_req.memory_type_bits,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )
            .unwrap();
            let memory_allocate_info = vk::MemoryAllocateInfo {
                allocation_size,
                memory_type_index,
                ..Default::default()
            };
            let mem = unsafe { device.allocate_memory(&memory_allocate_info, None) }.unwrap();
            unsafe { device.bind_buffer_memory(vertex_buffer, mem, 0) }.unwrap();
            unsafe { device.bind_buffer_memory(index_buffer, mem, vertex_mem_req.size) }.unwrap();
            mem
        };

        Self {
            vertex_buffer_size,
            index_buffer_size,
            mem,
            vertex_buffer,
            index_buffer,
            clipped_draws: Vec::new(),
        }
    }

    pub fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_buffer(Some(self.index_buffer), None);
            device.destroy_buffer(Some(self.vertex_buffer), None);
            device.free_memory(Some(self.mem), None);
        }
    }

    pub fn update(
        &mut self,
        device: &Device,
        command_buffer: vk::CommandBuffer,
        clipped_primitives: Vec<ClippedPrimitive>,
        staging: &mut Staging,
        staging_remain: &mut Range<usize>,
    ) {
        let (vertex_count, index_count) = clipped_primitives
            .iter()
            .filter_map(|clipped_primitive| match &clipped_primitive.primitive {
                Primitive::Mesh(mesh) => Some(mesh),
                Primitive::Callback(_) => None,
            })
            .fold((0, 0), |acc, mesh| {
                (acc.0 + mesh.vertices.len(), acc.1 + mesh.indices.len())
            });
        let vertex_write_size = vertex_count * mem::size_of::<Vertex>();
        let index_write_size = index_count * mem::size_of::<Index>();

        self.clipped_draws.clear();
        if vertex_write_size > self.vertex_buffer_size {
            eprintln!("out of space trying to write {vertex_write_size} bytes of vertex data");
        } else if index_write_size > self.index_buffer_size {
            eprintln!("out of space trying to write {index_write_size} bytes of index data");
        } else if vertex_write_size > 0 && index_write_size > 0 {
            let mut vertex_range = staging_remain.take_prefix(vertex_write_size).unwrap();
            let mut index_range = staging_remain.take_prefix(index_write_size).unwrap();
            let vertex_start = vertex_range.start;
            let index_start = index_range.start;
            let mut vertex_offset = 0;
            let mut first_index = 0;
            let staging_bytes = staging.bytes_mut();
            for clipped_primitive in &clipped_primitives {
                match &clipped_primitive.primitive {
                    Primitive::Mesh(mesh) => {
                        let src_vertex_bytes: &[u8] = bytemuck::cast_slice(&mesh.vertices);
                        let dst_vertex_range = vertex_range.take_prefix(src_vertex_bytes.len()).unwrap();
                        staging_bytes[dst_vertex_range].copy_from_slice(src_vertex_bytes);

                        let src_index_bytes: &[u8] = bytemuck::cast_slice(&mesh.indices);
                        let dst_index_range = index_range.take_prefix(src_index_bytes.len()).unwrap();
                        staging_bytes[dst_index_range].copy_from_slice(src_index_bytes);

                        let index_count = mesh.indices.len() as u32;
                        self.clipped_draws.push(ClippedDraw {
                            clip_rect: clipped_primitive.clip_rect,
                            texture_id: mesh.texture_id,
                            first_index,
                            index_count,
                            vertex_offset,
                        });
                        vertex_offset += mesh.vertices.len() as i32;
                        first_index += index_count;
                    }
                    Primitive::Callback(_) => {}
                }
            }
            assert!(vertex_range.is_empty());
            assert!(index_range.is_empty());

            let vertex_buffer_copy = vk::BufferCopy {
                src_offset: vertex_start as vk::DeviceSize,
                dst_offset: 0,
                size: vertex_write_size as vk::DeviceSize,
            };
            let index_buffer_copy = vk::BufferCopy {
                src_offset: index_start as vk::DeviceSize,
                dst_offset: 0,
                size: index_write_size as vk::DeviceSize,
            };
            unsafe {
                device.cmd_copy_buffer(
                    command_buffer,
                    staging.buffer,
                    self.vertex_buffer,
                    slice::from_ref(&vertex_buffer_copy),
                );
                device.cmd_copy_buffer(
                    command_buffer,
                    staging.buffer,
                    self.index_buffer,
                    slice::from_ref(&index_buffer_copy),
                );
            }

            let buffer_memory_barriers = [
                vk::BufferMemoryBarrier {
                    src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                    dst_access_mask: vk::AccessFlags::VERTEX_ATTRIBUTE_READ,
                    src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    buffer: Some(self.vertex_buffer),
                    size: vertex_write_size as vk::DeviceSize,
                    ..Default::default()
                },
                vk::BufferMemoryBarrier {
                    src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                    dst_access_mask: vk::AccessFlags::INDEX_READ,
                    src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    buffer: Some(self.index_buffer),
                    size: index_write_size as vk::DeviceSize,
                    ..Default::default()
                },
            ];
            unsafe {
                device.cmd_pipeline_barrier(
                    command_buffer,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::VERTEX_INPUT,
                    vk::DependencyFlags::empty(),
                    &[],
                    &buffer_memory_barriers,
                    &[],
                )
            };
        }
    }
}

impl Texture {
    pub fn new(
        device: &Device,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        command_buffer: vk::CommandBuffer,
        width: u32,
        height: u32,
        options: TextureOptions,
        layouts: &Layouts,
    ) -> Self {
        let image = {
            let image_create_info = vk::ImageCreateInfo {
                image_type: vk::ImageType::N2D,
                format: vk::Format::R8G8B8A8_SRGB,
                extent: vk::Extent3D {
                    width,
                    height,
                    depth: 1,
                },
                mip_levels: 1,
                array_layers: 1,
                samples: vk::SampleCountFlags::N1,
                usage: vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
                ..Default::default()
            };
            unsafe { device.create_image(&image_create_info, None) }.unwrap()
        };

        let mem = {
            let mem_req = unsafe { device.get_image_memory_requirements(image) };

            let memory_type_index = get_memory_type_index(
                physical_device_memory_properties,
                mem_req.memory_type_bits,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )
            .unwrap();
            let memory_allocate_info = vk::MemoryAllocateInfo {
                allocation_size: mem_req.size,
                memory_type_index,
                ..Default::default()
            };
            let mem = unsafe { device.allocate_memory(&memory_allocate_info, None) }.unwrap();
            unsafe { device.bind_image_memory(image, mem, 0) }.unwrap();
            mem
        };

        let descriptor_pool = {
            let descriptor_pool_sizes = [vk::DescriptorPoolSize {
                ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: 1,
            }];
            let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo::builder()
                .max_sets(1)
                .p_pool_sizes(&descriptor_pool_sizes);
            unsafe { device.create_descriptor_pool(&descriptor_pool_create_info, None) }.unwrap()
        };

        let descriptor_set = {
            let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(descriptor_pool)
                .p_set_layouts(slice::from_ref(&layouts.descriptor_set_layout));
            unsafe { device.allocate_descriptor_sets_single(&descriptor_set_allocate_info) }.unwrap()
        };

        let image_view = {
            let image_view_create_info = vk::ImageViewCreateInfo {
                image: Some(image),
                view_type: vk::ImageViewType::N2D,
                format: vk::Format::R8G8B8A8_SRGB,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    level_count: 1,
                    layer_count: 1,
                    ..Default::default()
                },
                components: vk::ComponentMapping {
                    r: vk::ComponentSwizzle::R,
                    g: vk::ComponentSwizzle::G,
                    b: vk::ComponentSwizzle::B,
                    a: vk::ComponentSwizzle::A,
                },
                ..Default::default()
            };
            unsafe { device.create_image_view(&image_view_create_info, None) }.unwrap()
        };

        let sampler = {
            let map_filter = |filter: TextureFilter| match filter {
                TextureFilter::Nearest => vk::Filter::NEAREST,
                TextureFilter::Linear => vk::Filter::LINEAR,
            };
            let sampler_create_info = vk::SamplerCreateInfo {
                mag_filter: map_filter(options.magnification),
                min_filter: map_filter(options.minification),
                address_mode_u: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                address_mode_v: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                ..Default::default()
            };
            unsafe { device.create_sampler(&sampler_create_info, None) }.unwrap()
        };

        {
            let image_info = vk::DescriptorImageInfo {
                sampler: Some(sampler),
                image_view: Some(image_view),
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            };
            let write_descriptor_set = vk::WriteDescriptorSet::builder()
                .dst_set(descriptor_set)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .p_image_info(slice::from_ref(&image_info));
            unsafe { device.update_descriptor_sets(slice::from_ref(&write_descriptor_set), &[]) };
        }

        let image_memory_barrier = vk::ImageMemoryBarrier {
            dst_access_mask: vk::AccessFlags::TRANSFER_WRITE,
            new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            image: Some(image),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                level_count: 1,
                layer_count: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        unsafe {
            device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::HOST,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                slice::from_ref(&image_memory_barrier),
            )
        };

        Self {
            mem,
            image,
            image_view,
            sampler,
            descriptor_pool,
            descriptor_set,
            image_state: ImageState::TransferDest,
        }
    }

    pub fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_descriptor_pool(Some(self.descriptor_pool), None);
            device.destroy_sampler(Some(self.sampler), None);
            device.destroy_image_view(Some(self.image_view), None);
            device.destroy_image(Some(self.image), None);
            device.free_memory(Some(self.mem), None);
        }
    }

    pub fn make_transfer_dest(&mut self, device: &Device, command_buffer: vk::CommandBuffer) {
        if self.image_state != ImageState::TransferDest {
            let image_memory_barrier = vk::ImageMemoryBarrier {
                src_access_mask: vk::AccessFlags::SHADER_READ,
                dst_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                old_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                image: Some(self.image),
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    level_count: 1,
                    layer_count: 1,
                    ..Default::default()
                },
                ..Default::default()
            };
            unsafe {
                device.cmd_pipeline_barrier(
                    command_buffer,
                    vk::PipelineStageFlags::FRAGMENT_SHADER,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    slice::from_ref(&image_memory_barrier),
                )
            };
            self.image_state = ImageState::TransferDest;
        }
    }

    pub fn make_shader_read(&mut self, device: &Device, command_buffer: vk::CommandBuffer) {
        if self.image_state != ImageState::ShaderRead {
            let image_memory_barrier = vk::ImageMemoryBarrier {
                src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                dst_access_mask: vk::AccessFlags::SHADER_READ,
                old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                image: Some(self.image),
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    level_count: 1,
                    layer_count: 1,
                    ..Default::default()
                },
                ..Default::default()
            };
            unsafe {
                device.cmd_pipeline_barrier(
                    command_buffer,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::FRAGMENT_SHADER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    slice::from_ref(&image_memory_barrier),
                )
            };
            self.image_state = ImageState::ShaderRead;
        }
    }
}

impl TextureSet {
    pub fn destroy(&mut self, device: &Device) {
        for (_, texture) in self.active.drain() {
            texture.destroy(device);
        }
        self.pending_updates.clear();
        self.pending_frees.clear();
        for texture in self.pending_deletes.drain(..) {
            texture.destroy(device);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        &mut self,
        device: &Device,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        command_buffer: vk::CommandBuffer,
        mut textures_delta: TexturesDelta,
        layouts: &Layouts,
        staging: &mut Staging,
        staging_remain: &mut Range<usize>,
    ) {
        // peform pending deletes
        for texture in self.pending_deletes.drain(..) {
            texture.destroy(device);
        }

        // perform pending frees
        for id in self.pending_frees.drain(..) {
            self.pending_updates.retain(|(update_id, _)| *update_id != id);
            if let Some(texture) = self.active.remove(&id) {
                self.pending_deletes.push(texture);
            }
        }

        // copy in the new deltas
        self.pending_updates.append(&mut VecDeque::from(textures_delta.set));
        self.pending_frees.append(&mut textures_delta.free);

        // stage as many of the updates as we have staging memory for
        while let Some((id, image_delta)) = self.pending_updates.pop_front() {
            let width = image_delta.image.width();
            let height = image_delta.image.height();
            if let Some(dst_range) = staging_remain.take_prefix(width * height * 4) {
                let src_pixels = match &image_delta.image {
                    ImageData::Color(image) => Cow::Borrowed(&image.pixels),
                    ImageData::Font(image) => Cow::Owned(image.srgba_pixels(None).collect::<Vec<_>>()),
                };
                let staging_bytes = staging.bytes_mut();
                let buffer_offset = dst_range.start;
                staging_bytes[dst_range].copy_from_slice(bytemuck::cast_slice(src_pixels.as_slice()));

                if image_delta.pos.is_none() {
                    if let Some(texture) = self.active.remove(&id) {
                        self.pending_deletes.push(texture);
                    }
                    self.active.insert(
                        id,
                        Texture::new(
                            device,
                            physical_device_memory_properties,
                            command_buffer,
                            width as u32,
                            height as u32,
                            image_delta.options,
                            layouts,
                        ),
                    );
                }

                if let Some(texture) = self.active.get_mut(&id) {
                    texture.make_transfer_dest(device, command_buffer);

                    let offset = image_delta.pos.unwrap_or([0, 0]);
                    let buffer_image_copy = vk::BufferImageCopy {
                        buffer_offset: buffer_offset as vk::DeviceSize,
                        image_subresource: vk::ImageSubresourceLayers {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            layer_count: 1,
                            ..Default::default()
                        },
                        image_offset: vk::Offset3D {
                            x: offset[0] as i32,
                            y: offset[1] as i32,
                            z: 0,
                        },
                        image_extent: vk::Extent3D {
                            width: width as u32,
                            height: height as u32,
                            depth: 1,
                        },
                        ..Default::default()
                    };
                    unsafe {
                        device.cmd_copy_buffer_to_image(
                            command_buffer,
                            staging.buffer,
                            texture.image,
                            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                            slice::from_ref(&buffer_image_copy),
                        )
                    };
                }
            } else {
                // try again next frame
                self.pending_updates.push_front((id, image_delta));
                break;
            }
        }

        // ensure all images are ready to sample
        for texture in self.active.values_mut() {
            texture.make_shader_read(device, command_buffer);
        }
    }
}

impl Renderer {
    pub fn new(
        device: &Device,
        physical_device_properties: &vk::PhysicalDeviceProperties,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        max_vertex_count: u32,
        max_texture_side: u32,
    ) -> Self {
        let vertex_buffer_size = mem::size_of::<Vertex>() * (max_vertex_count as usize);
        let index_buffer_size = mem::size_of::<Index>() * (max_vertex_count as usize);
        let texture_upload_size = (4 * max_texture_side * max_texture_side) as usize;
        let staging_area_size = vertex_buffer_size + index_buffer_size + texture_upload_size;

        let layouts = Layouts::new(device);
        let staging = Staging::new(
            device,
            physical_device_properties,
            physical_device_memory_properties,
            staging_area_size,
        );
        let mesh_buffers = MeshBuffers::new(
            device,
            physical_device_memory_properties,
            vertex_buffer_size,
            index_buffer_size,
        );

        Self {
            layouts,
            staging,
            mesh_buffers,
            texture_set: Default::default(),
        }
    }

    pub fn destroy(&mut self, device: &Device) {
        self.texture_set.destroy(device);
        self.mesh_buffers.destroy(device);
        self.staging.destroy(device);
        self.layouts.destroy(device);
    }

    pub fn create_pipeline(
        &self,
        device: &Device,
        render_pass: vk::RenderPass,
        samples: vk::SampleCountFlags,
    ) -> vk::Pipeline {
        self.layouts.create_pipeline(device, render_pass, samples)
    }

    pub fn update(
        &mut self,
        device: &Device,
        physical_device_memory_properties: &vk::PhysicalDeviceMemoryProperties,
        command_buffer: vk::CommandBuffer,
        clipped_primitives: Vec<ClippedPrimitive>,
        textures_delta: TexturesDelta,
    ) {
        let mut staging_remain = self.staging.next_subset();
        let staging_start = staging_remain.start;

        self.mesh_buffers.update(
            device,
            command_buffer,
            clipped_primitives,
            &mut self.staging,
            &mut staging_remain,
        );

        self.texture_set.update(
            device,
            physical_device_memory_properties,
            command_buffer,
            textures_delta,
            &self.layouts,
            &mut self.staging,
            &mut staging_remain,
        );

        let written_range = staging_start..staging_remain.start;
        self.staging.flush_mapped_range(device, written_range);
    }

    pub fn render(
        &mut self,
        device: &Device,
        command_buffer: vk::CommandBuffer,
        pipeline: vk::Pipeline,
        width_in_pixels: u32,
        height_in_pixels: u32,
        pixels_per_point: f32,
    ) {
        unsafe {
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        }

        let batch_data = BatchData {
            size_in_points_rcp: (
                pixels_per_point / (width_in_pixels as f32),
                pixels_per_point / (height_in_pixels as f32),
            ),
        };
        unsafe {
            device.cmd_push_constants(
                command_buffer,
                self.layouts.pipeline_layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                slice::from_ref(&batch_data),
            )
        };

        unsafe {
            device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                slice::from_ref(&self.mesh_buffers.vertex_buffer),
                &[0],
            );
            device.cmd_bind_index_buffer(
                command_buffer,
                Some(self.mesh_buffers.index_buffer),
                0,
                vk::IndexType::UINT32,
            );
        }

        for clipped_draw in &self.mesh_buffers.clipped_draws {
            if let Some(texture) = self.texture_set.active.get(&clipped_draw.texture_id) {
                unsafe {
                    device.cmd_bind_descriptor_sets(
                        command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        self.layouts.pipeline_layout,
                        0,
                        slice::from_ref(&texture.descriptor_set),
                        &[],
                    );
                }

                let min_x = width_in_pixels.min((clipped_draw.clip_rect.min.x * pixels_per_point).round() as u32);
                let max_x = width_in_pixels.min((clipped_draw.clip_rect.max.x * pixels_per_point).round() as u32);

                let min_y = height_in_pixels.min((clipped_draw.clip_rect.min.y * pixels_per_point).round() as u32);
                let max_y = height_in_pixels.min((clipped_draw.clip_rect.max.y * pixels_per_point).round() as u32);

                let scissor = vk::Rect2D {
                    offset: vk::Offset2D {
                        x: min_x as i32,
                        y: min_y as i32,
                    },
                    extent: vk::Extent2D {
                        width: max_x - min_x,
                        height: max_y - min_y,
                    },
                };
                unsafe {
                    device.cmd_set_scissor(command_buffer, 0, slice::from_ref(&scissor));
                }

                unsafe {
                    device.cmd_draw_indexed(
                        command_buffer,
                        clipped_draw.index_count,
                        1,
                        clipped_draw.first_index,
                        clipped_draw.vertex_offset,
                        0,
                    );
                }
            }
        }
    }
}
