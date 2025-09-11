    pub unsafe fn enumerate_instance_version(&self) -> Result<vk::Version> {
        if let Some(fp) = self.fp_enumerate_instance_version {
            let mut p_api_version = MaybeUninit::<_>::uninit();
            let err = (fp)(p_api_version.as_mut_ptr());
            match err {
                vk::Result::SUCCESS => Ok(p_api_version.assume_init()),
                _ => Err(err),
            }
        } else {
            Ok(vk::Version::default())
        }
    }
    pub unsafe fn create_instance_commands(
        &self,
        p_create_info: &vk::InstanceCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> LoadResult<Instance> {
        let instance = self.create_instance(p_create_info, p_allocator)?;
        Instance::load(self, instance, p_create_info)
    }
