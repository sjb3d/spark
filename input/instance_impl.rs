    pub unsafe fn create_device_commands(
        &self,
        globals: &Globals,
        physical_device: vk::PhysicalDevice,
        p_create_info: &vk::DeviceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LoadResult<Device> {
        let device = self.create_device(physical_device, p_create_info, p_allocator)?;
        Device::load(globals, self, device, p_create_info)
    }