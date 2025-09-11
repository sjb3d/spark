pub fn get_device_proc_addr(
    self: InstanceCommands,
    device: Device,
    p_name: [*:0]const u8,
) MissingFunctionError!FpVoidFunction {
    return self.fp_get_device_proc_addr(device, p_name) orelse return error.MissingFunction;
}

pub const CreateDeviceOrMissingFunctionError = CreateDeviceError || MissingFunctionError;
pub fn create_device_to_commands(
    self: InstanceCommands,
    globals: GlobalCommands,
    physical_device: PhysicalDevice,
    p_create_info: *const DeviceCreateInfo,
    p_allocator: ?*const AllocationCallbacks,
) CreateDeviceOrMissingFunctionError!DeviceCommands {
    const device = try self.create_device(physical_device, p_create_info, p_allocator);
    return DeviceCommands.init(globals, self, device, p_create_info);
}
