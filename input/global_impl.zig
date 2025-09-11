fn get_proc_addr(
    fp_get_instance_proc_addr: FpGetInstanceProcAddr,
    p_name: [*:0]const u8,
) MissingFunctionError!FpVoidFunction {
    return fp_get_instance_proc_addr(.null_handle, p_name) orelse return error.MissingFunction;
}

pub fn get_instance_proc_addr(
    self: GlobalCommands,
    instance: Instance,
    p_name: [*:0]const u8,
) MissingFunctionError!FpVoidFunction {
    return self.fp_get_instance_proc_addr(instance, p_name) orelse return error.MissingFunction;
}

pub const EnumerateInstanceVersionError = error{
    OutOfHostMemory,
    Unexpected,
};
pub fn enumerate_instance_version(
    self: GlobalCommands,
) EnumerateInstanceVersionError!Version {
    if (self.fp_enumerate_instance_version) |fp| {
        var version: Version = undefined;
        switch (fp(&version)) {
            .success => return version,
            .error_out_of_host_memory => return error.OutOfHostMemory,
            else => return error.Unexpected,
        }
    } else {
        return make_version(1, 0, 0);
    }
}

pub const CreateInstanceOrMissingFunctionError = CreateInstanceError || MissingFunctionError;
pub fn create_instance_to_commands(
    self: GlobalCommands,
    p_create_info: *const InstanceCreateInfo,
    p_allocator: ?*const AllocationCallbacks,
) CreateInstanceOrMissingFunctionError!InstanceCommands {
    const instance = try self.create_instance(p_create_info, p_allocator);
    return InstanceCommands.init(self, instance, p_create_info);
}
