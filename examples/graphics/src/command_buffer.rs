use crate::context::*;
use spark::{vk, Builder};
use std::slice;
use std::sync::Arc;

struct CommandBufferSet {
    pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    fence: vk::Fence,
    image_available_semaphore: vk::Semaphore,
    rendering_finished_semaphore: vk::Semaphore,
}

impl CommandBufferSet {
    fn new(context: &Context) -> Self {
        let device = &context.device;

        let pool = {
            let command_pool_create_info = vk::CommandPoolCreateInfo {
                flags: vk::CommandPoolCreateFlags::empty(),
                queue_family_index: context.queue_family_index,
                ..Default::default()
            };
            unsafe { device.create_command_pool(&command_pool_create_info, None) }.unwrap()
        };

        let command_buffer = {
            let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
                command_pool: Some(pool),
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: 1,
                ..Default::default()
            };

            unsafe { device.allocate_command_buffers_single(&command_buffer_allocate_info) }.unwrap()
        };

        let fence = {
            let fence_create_info = vk::FenceCreateInfo {
                flags: vk::FenceCreateFlags::SIGNALED,
                ..Default::default()
            };
            unsafe { device.create_fence(&fence_create_info, None) }.unwrap()
        };

        let image_available_semaphore = unsafe { device.create_semaphore(&Default::default(), None) }.unwrap();
        let rendering_finished_semaphore = unsafe { device.create_semaphore(&Default::default(), None) }.unwrap();

        Self {
            pool,
            command_buffer,
            fence,
            image_available_semaphore,
            rendering_finished_semaphore,
        }
    }
}

pub struct CommandBufferPool {
    context: Arc<Context>,
    sets: [CommandBufferSet; Self::COUNT],
    index: usize,
}

impl CommandBufferPool {
    pub const COUNT: usize = 2;

    pub fn new(context: &Arc<Context>) -> Self {
        Self {
            context: Arc::clone(context),
            sets: [CommandBufferSet::new(context), CommandBufferSet::new(context)],
            index: 0,
        }
    }

    pub fn acquire(&mut self) -> (vk::CommandBuffer, vk::Semaphore) {
        self.index = (self.index + 1) % Self::COUNT;

        let set = &self.sets[self.index];

        let timeout_ns = 1000 * 1000 * 1000;
        loop {
            let res = unsafe {
                self.context
                    .device
                    .wait_for_fences(slice::from_ref(&set.fence), true, timeout_ns)
            };
            match res {
                Ok(_) => break,
                Err(vk::Result::TIMEOUT) => {}
                Err(err_code) => panic!("failed to wait for fence {}", err_code),
            }
        }

        unsafe { self.context.device.reset_fences(slice::from_ref(&set.fence)) }.unwrap();

        unsafe {
            self.context
                .device
                .reset_command_pool(set.pool, vk::CommandPoolResetFlags::empty())
        }
        .unwrap();

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };
        unsafe {
            self.context
                .device
                .begin_command_buffer(set.command_buffer, &command_buffer_begin_info)
        }
        .unwrap();

        (set.command_buffer, set.image_available_semaphore)
    }

    pub fn submit(&self) -> vk::Semaphore {
        let set = &self.sets[self.index];

        unsafe { self.context.device.end_command_buffer(set.command_buffer) }.unwrap();

        let submit_info = vk::SubmitInfo::builder()
            .p_wait_semaphores(
                slice::from_ref(&set.image_available_semaphore),
                slice::from_ref(&vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT),
            )
            .p_command_buffers(slice::from_ref(&set.command_buffer))
            .p_signal_semaphores(slice::from_ref(&set.rendering_finished_semaphore));

        unsafe {
            self.context
                .device
                .queue_submit(self.context.queue, slice::from_ref(&submit_info), Some(set.fence))
        }
        .unwrap();

        set.rendering_finished_semaphore
    }
}

impl Drop for CommandBufferPool {
    fn drop(&mut self) {
        let device = &self.context.device;
        for set in self.sets.iter() {
            unsafe {
                device.destroy_semaphore(Some(set.rendering_finished_semaphore), None);
                device.destroy_semaphore(Some(set.image_available_semaphore), None);
                device.destroy_fence(Some(set.fence), None);
                device.free_command_buffers(set.pool, slice::from_ref(&set.command_buffer));
                device.destroy_command_pool(Some(set.pool), None);
            }
        }
    }
}
