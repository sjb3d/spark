use std::ffi::CStr;
use std::mem;
use std::slice;
use vkr::{vk, Builder, Loader};

fn get_memory_type_index(
    memory_properties: &vk::PhysicalDeviceMemoryProperties,
    memory_type_bits: u32,
    property_flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    for i in 0..memory_properties.memory_type_count {
        let mt = &memory_properties.memory_types[i as usize];
        if (memory_type_bits & (1 << i)) != 0 && mt.property_flags.contains(property_flags) {
            return Some(i);
        }
    }
    None
}

fn main() -> Result<(), vkr::LoaderError> {
    // load the Vulkan lib
    let instance = {
        let loader = Loader::new()?;
        let layer_names_raw =
            [unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_LUNARG_standard_validation\0") }.as_ptr()];
        let instance_create_info = vk::InstanceCreateInfo::builder().pp_enabled_layer_names(&layer_names_raw);
        unsafe { loader.create_instance(&instance_create_info, None) }?
    };

    // find the first physical device
    let physical_device = {
        let physical_devices = unsafe { instance.enumerate_physical_devices_to_vec() }?;
        for physical_device in &physical_devices {
            let props = unsafe { instance.get_physical_device_properties(*physical_device) };
            println!("physical device ({}): {:?}", props.device_type, unsafe {
                CStr::from_ptr(props.device_name.as_ptr())
            });
        }
        physical_devices.first().cloned().expect("no physical device found")
    };

    // find the first queue family that supports compute
    let queue_family_properties =
        unsafe { instance.get_physical_device_queue_family_properties_to_vec(physical_device) };
    let queue_family_index = queue_family_properties
        .iter()
        .enumerate()
        .filter_map(|(index, &info)| {
            if info.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                Some(index as u32)
            } else {
                None
            }
        })
        .next()
        .expect("no queue family supports compute");

    // create a device for this queue family
    let device = {
        let queue_priority = 1.0;
        let device_queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(queue_family_index)
            .p_queue_priorities(slice::from_ref(&queue_priority));
        let device_create_info =
            vk::DeviceCreateInfo::builder().p_queue_create_infos(slice::from_ref(&device_queue_create_info));
        unsafe { instance.create_device(physical_device, &device_create_info, None, vk::Version::default()) }?
    };

    // load the compute shader
    let shader_module = {
        let shader_bytes = include_bytes!("compute_fill.comp.spv");
        let shader_module_create_info = vk::ShaderModuleCreateInfo {
            code_size: shader_bytes.len(),
            p_code: shader_bytes.as_ptr() as _,
            ..Default::default()
        };
        unsafe { device.create_shader_module(&shader_module_create_info, None) }?
    };

    // create a buffer for outputs
    let dispatch_size = 256;
    let buffer_size = dispatch_size * mem::size_of::<f32>();
    let buffer = {
        let buffer_create_info = vk::BufferCreateInfo {
            size: buffer_size as vk::DeviceSize,
            usage: vk::BufferUsageFlags::STORAGE_BUFFER,
            ..Default::default()
        };
        unsafe { device.create_buffer(&buffer_create_info, None) }?
    };
    let mem_req = unsafe { device.get_buffer_memory_requirements(buffer) };

    // allocate memory for the buffer
    let mem = {
        let memory_properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };
        let memory_type_index = get_memory_type_index(
            &memory_properties,
            mem_req.memory_type_bits,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        )
        .expect("no suitable memory type found");
        let memory_allocate_info = vk::MemoryAllocateInfo {
            allocation_size: mem_req.size,
            memory_type_index,
            ..Default::default()
        };
        unsafe { device.allocate_memory(&memory_allocate_info, None) }?
    };
    unsafe { device.bind_buffer_memory(buffer, mem, 0) }?;

    // make the pipeline layout
    let descriptor_set_layout = {
        let descriptor_set_layout_bindings = [vk::DescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlags::COMPUTE,
            ..Default::default()
        }];
        let descriptor_set_layout_create_info =
            vk::DescriptorSetLayoutCreateInfo::builder().p_bindings(&descriptor_set_layout_bindings);
        unsafe { device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None) }?
    };
    let pipeline_layout = {
        let pipeline_layout_create_info =
            vk::PipelineLayoutCreateInfo::builder().p_set_layouts(slice::from_ref(&descriptor_set_layout));
        unsafe { device.create_pipeline_layout(&pipeline_layout_create_info, None) }?
    };

    // create the pipeline
    let pipeline_create_info = vk::ComputePipelineCreateInfo {
        stage: vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::COMPUTE,
            module: Some(shader_module),
            p_name: unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") }.as_ptr(),
            ..Default::default()
        },
        layout: Some(pipeline_layout),
        ..Default::default()
    };
    let pipeline =
        unsafe { device.create_compute_pipelines_single(None, slice::from_ref(&pipeline_create_info), None) }?;

    // create a pool for the descriptor we need
    let descriptor_pool = {
        let descriptor_pool_sizes = [vk::DescriptorPoolSize {
            ty: vk::DescriptorType::STORAGE_BUFFER,
            descriptor_count: 1,
        }];
        let descriptor_pool_create_info = vk::DescriptorPoolCreateInfo::builder()
            .max_sets(1)
            .p_pool_sizes(&descriptor_pool_sizes);
        unsafe { device.create_descriptor_pool(&descriptor_pool_create_info, None) }?
    };

    // allocate and write the descriptor set
    let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(descriptor_pool)
        .p_set_layouts(slice::from_ref(&descriptor_set_layout));
    let descriptor_set = unsafe { device.allocate_descriptor_sets_single(&descriptor_set_allocate_info) }?;

    let descriptor_buffer_info = [vk::DescriptorBufferInfo {
        buffer: Some(buffer),
        offset: 0,
        range: vk::WHOLE_SIZE,
    }];
    let write_descriptor_set = vk::WriteDescriptorSet::builder()
        .dst_set(descriptor_set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .p_buffer_info(&descriptor_buffer_info);
    unsafe { device.update_descriptor_sets(slice::from_ref(&write_descriptor_set), &[]) };

    // run a command buffer to run the shader
    let command_pool_create_info = vk::CommandPoolCreateInfo {
        queue_family_index,
        ..Default::default()
    };
    let command_pool = unsafe { device.create_command_pool(&command_pool_create_info, None) }?;
    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
        command_pool: Some(command_pool),
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: 1,
        ..Default::default()
    };
    let command_buffer = unsafe { device.allocate_command_buffers_single(&command_buffer_allocate_info) }?;

    // make a command buffer that runs the ceompute shader
    let command_buffer_begin_info = vk::CommandBufferBeginInfo {
        flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        ..Default::default()
    };
    unsafe { device.begin_command_buffer(command_buffer, &command_buffer_begin_info) }?;
    unsafe { device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline) };
    unsafe {
        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::COMPUTE,
            pipeline_layout,
            0,
            slice::from_ref(&descriptor_set),
            &[],
        )
    };
    unsafe { device.cmd_dispatch(command_buffer, (dispatch_size as u32) / 16, 1, 1) };
    unsafe { device.end_command_buffer(command_buffer) }?;

    // run it and wait until it is completed
    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };
    let submit_info = vk::SubmitInfo::builder().p_command_buffers(slice::from_ref(&command_buffer));
    unsafe { device.queue_submit(queue, slice::from_ref(&submit_info), None) }?;
    unsafe { device.device_wait_idle() }?;

    // check results
    let mapping = unsafe { device.map_memory(mem, 0, vk::WHOLE_SIZE, Default::default()) }?;
    let check = unsafe { slice::from_raw_parts(mapping as *const f32, dispatch_size) };
    for (i, v) in check.iter().cloned().enumerate() {
        assert_eq!(i as f32, v);
    }
    println!("compute shader run successfully!");

    // let the driver clean up
    Ok(())
}
